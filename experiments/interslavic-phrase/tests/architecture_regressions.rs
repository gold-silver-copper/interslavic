//! Architectural regression and bounded-generative tests.
//!
//! These tests target ownership boundaries rather than implementation
//! details: grammatical-role edges own case, nested clauses own their
//! clitics, validation is shared, discourse changes referential choice
//! without replacing syntax, and every valid serialized leaf roundtrips.

use interslavic::{Case, Gender, Number, Person};
use interslavic_phrase::discourse::{Connective, DiscourseSentence, narrate, narrate_checked};
use interslavic_phrase::*;
use std::panic::{AssertUnwindSafe, catch_unwind};

fn sentence(tree: &Clause) -> String {
    realize(tree, RealizeOpts::sentence()).unwrap()
}

#[test]
fn nested_relative_clitics_never_migrate_to_the_parent_domain() {
    let reflexive = clause(
        np("žena"),
        vp("viděti").object(np("mųž").relative(RelClause::subject_gap(vp("myti sę")))),
    );
    assert_eq!(sentence(&reflexive), "Žena vidi mųža, ktory myje sę.");

    let object_clitic = clause(
        np("žena"),
        vp("viděti").object(
            np("mųž").relative(RelClause::subject_gap(vp("viděti").object(pron_clitic(
                Person::Third,
                Number::Singular,
                Gender::Masculine,
            )))),
        ),
    );
    assert_eq!(sentence(&object_clitic), "Žena vidi mųža, ktory vidi go.");

    // The parent uses second-position placement, but the relative remains
    // an opaque domain and places its own reflexive internally.
    assert_eq!(
        realize(
            &reflexive,
            RealizeOpts::sentence().clitics(CliticStyle::SecondPosition),
        )
        .unwrap(),
        "Žena vidi mųža, ktory myje sę."
    );
}

#[test]
fn coincident_relative_and_coordination_commas_are_emitted_once() {
    let coordinated_subject = clause(
        coordinate(
            Conj::I,
            vec![
                np("mųž")
                    .relative(RelClause::subject_gap(vp("spati")))
                    .into(),
                np("žena").into(),
                np("otėc").into(),
            ],
        ),
        vp("stojati"),
    );
    assert_eq!(
        sentence(&coordinated_subject),
        "Mųž, ktory spi, žena i otėc stojęt."
    );

    let coordinated_vps = clause(
        np("žena"),
        vp("viděti").object(np("mųž").relative(RelClause::subject_gap(vp("spati")))),
    )
    .and_vp(vp("čitati"))
    .unwrap()
    .and_vp(vp("pisati"))
    .unwrap();
    assert_eq!(
        sentence(&coordinated_vps),
        "Žena vidi mųža, ktory spi, čitaje i piše."
    );
}

#[test]
fn one_complement_edge_assigns_one_case_to_the_whole_coordination() {
    let tree = clause(
        np("krålj"),
        vp("vladati").object_case(
            Case::Acc,
            coordinate(Conj::I, vec![np("zemja").into(), np("država").into()]),
        ),
    );
    let realized = realize_checked(&tree, RealizeOpts::sentence()).unwrap();
    assert_eq!(realized.text, "Krålj vladaje zemjų i državų.");
    assert_eq!(
        realized
            .warnings
            .iter()
            .filter(|warning| matches!(warning, PhraseWarning::GovernsConflict { .. }))
            .count(),
        1
    );
    assert!(matches!(
        realized.warnings.as_slice(),
        [PhraseWarning::GovernsConflict {
            path,
            dictionary: Case::Ins,
            used: Case::Acc,
            ..
        }] if path == "clause.core.vp[0].object"
    ));
}

#[test]
fn information_object_is_the_first_object_that_exists() {
    let tree = clause(np("žena"), vp("spati"))
        .and_vp(vp("viděti").object(np("mųž")))
        .unwrap()
        .topic(SlotRef::Object);
    assert_eq!(sentence(&tree), "Mųža žena spi i vidi.");

    // Default li ordering must leave a later object's ownership intact:
    // only an explicitly selected topic/focus may detach it from VP1.
    let question = clause(np("žena"), vp("spati"))
        .and_vp(vp("viděti").object(np("mųž")))
        .unwrap()
        .force(Force::LiQuestion);
    assert_eq!(sentence(&question), "Spi li žena i vidi mųža?");
}

#[test]
fn object_relative_gaps_share_explicit_case_resolution() {
    let tree = clause(
        np("zemja").relative(RelClause::object_gap_case(
            Case::Gen,
            np("krålj"),
            vp("vladati"),
        )),
        vp("ležati"),
    );
    let realized = realize_checked(&tree, RealizeOpts::sentence()).unwrap();
    assert!(matches!(
        realized.warnings.as_slice(),
        [PhraseWarning::GovernsConflict {
            path,
            dictionary: Case::Ins,
            used: Case::Gen,
            ..
        }] if path == "clause.subject.relative.gap"
    ));

    let printed = print(&tree);
    assert!(printed.contains(":gap obj :case gen"));
    assert_eq!(clause_from_str(&printed).unwrap(), tree);
}

#[test]
fn sexpr_cannot_hide_case_overrides_inside_governed_nominals() {
    let nested_pp_override = "\
        (clause (np (n kot))
          (vp (v spati)
            (pp (prep pod) :case ins (np :case dat (n stol)))))";
    let error = clause_from_str(nested_pp_override).unwrap_err();
    assert!(error.msg.contains("unknown np key `:case`"));

    let mixed_coordinated_object = "\
        (clause (np (n krålj))
          (vp (v vladati)
            (object :case gen
              (coord i (np (n zemja)) (np :case acc (n država))))))";
    let error = clause_from_str(mixed_coordinated_object).unwrap_err();
    assert!(error.msg.contains("unknown np key `:case`"));
}

#[test]
fn discourse_pronominalization_preserves_the_complement_case_edge() {
    let story = vec![
        DiscourseSentence::new(clause(
            np("krålj").entity("king"),
            vp("čitati").object_case(Case::Gen, np("kniga").entity("book")),
        )),
        DiscourseSentence::new(clause(
            np("krålj").entity("king"),
            vp("čitati").object_case(Case::Gen, np("kniga").entity("book")),
        ))
        .connective(Connective::Potom),
    ];
    assert_eq!(
        narrate(story, RealizeOpts::sentence()).unwrap(),
        "Krålj čitaje knigy. Potom on čitaje jej."
    );

    let explicit_clitic = vec![
        DiscourseSentence::new(clause(
            np("žena"),
            vp("viděti").object(np("mųž").entity("man").referential(ReferentialForm::Clitic)),
        )),
        DiscourseSentence::new(clause(
            np("žena"),
            vp("viděti").object(np("mųž").entity("man").referential(ReferentialForm::Clitic)),
        ))
        .connective(Connective::Potom),
    ];
    assert_eq!(
        narrate(explicit_clitic, RealizeOpts::sentence()).unwrap(),
        "Žena vidi go. Potom žena vidi go."
    );
}

#[test]
fn discourse_aggregation_requires_explicit_coreference() {
    let untagged = vec![
        DiscourseSentence::new(clause(np("krålj"), vp("čitati"))),
        DiscourseSentence::new(clause(np("krålj"), vp("pisati"))),
    ];
    assert_eq!(
        narrate(untagged, RealizeOpts::sentence()).unwrap(),
        "Krålj čitaje. Krålj piše."
    );

    let tagged = vec![
        DiscourseSentence::new(clause(np("krålj").entity("king"), vp("čitati"))),
        DiscourseSentence::new(clause(np("krålj").entity("king"), vp("pisati"))),
    ];
    assert_eq!(
        narrate(tagged, RealizeOpts::sentence()).unwrap(),
        "Krålj čitaje i piše."
    );
}

#[test]
fn discourse_salience_follows_surface_information_order() {
    let story = vec![
        DiscourseSentence::new(
            clause(
                np("krålj").entity("a"),
                vp("viděti").object(np("otėc").entity("b")),
            )
            .topic(SlotRef::Object),
        ),
        DiscourseSentence::new(clause(np("krålj").entity("a"), vp("spati"))),
    ];
    assert_eq!(
        narrate(story, RealizeOpts::sentence()).unwrap(),
        "Otca krålj vidi. On spi."
    );
}

#[test]
fn plural_only_counted_subject_and_reference_share_one_profile_policy() {
    let story = vec![
        DiscourseSentence::new(clause(np("noviny").count(1).entity("news"), vp("ležati"))),
        DiscourseSentence::new(clause(np("noviny").count(1).entity("news"), vp("ležati")))
            .connective(Connective::Potom),
    ];
    assert_eq!(
        narrate(story, RealizeOpts::sentence()).unwrap(),
        "1 noviny ležęt. Potom one ležęt."
    );
}

#[test]
fn pronoun_ambiguity_uses_the_actual_facade_forms() {
    // The facade's 3sg-neuter personal paradigm is `ono` (Nom) versus
    // `jego` (Acc), so it is not syncretic and must not produce a false
    // warning when the otherwise-syncretic object is fronted.
    let tree = clause(
        pron(Person::Third, Number::Singular, Gender::Neuter),
        vp("viděti").object(np("avto")),
    )
    .topic(SlotRef::Object);
    let realized = realize_checked(&tree, RealizeOpts::sentence()).unwrap();
    assert_eq!(realized.text, "Avto ono vidi.");
    assert!(
        !realized
            .warnings
            .iter()
            .any(|warning| matches!(warning, PhraseWarning::AmbiguousOrder { .. }))
    );

    // Planned NP reference uses the same actual pronoun comparison; it
    // must not fall back to comparing the hidden lexical noun.
    let planned = clause(
        np("avto").referential(ReferentialForm::Pronoun),
        vp("viděti").object(np("nebo")),
    )
    .topic(SlotRef::Object);
    let realized = realize_checked(&planned, RealizeOpts::sentence()).unwrap();
    assert_eq!(realized.text, "Nebo ono vidi.");
    assert!(
        !realized
            .warnings
            .iter()
            .any(|warning| matches!(warning, PhraseWarning::AmbiguousOrder { .. }))
    );

    // A genuinely syncretic noun pair still receives one pathful warning.
    let realized = realize_checked(
        &clause(np("avto"), vp("viděti").object(np("nebo"))).topic(SlotRef::Object),
        RealizeOpts::sentence(),
    )
    .unwrap();
    assert_eq!(
        realized.warnings,
        vec![PhraseWarning::AmbiguousOrder {
            path: "clause.order".into(),
        }]
    );
}

#[test]
fn typed_and_sexpr_inputs_share_structured_validation() {
    let typed = clause(np("kniga"), vp("kupiti").object(np("moneta"))).passive();
    let errors = validate(&typed).unwrap_err();
    assert!(matches!(
        errors.0.as_slice(),
        [ValidationError {
            path: AstPath(path),
            kind: ValidationErrorKind::IncoherentClause(
                "a passive clause promotes the patient; it cannot retain an object"
            ),
        }] if path == "clause.core.vp[0].object"
    ));

    let sexpr = "\
        (clause (np (n kniga))
          (vp (v kupiti) (object (np (n moneta))))
          :voice passive)";
    let error = clause_from_str(sexpr).unwrap_err();
    assert_eq!(error.at, sexpr.find("(object").unwrap());
    assert!(error.msg.contains("clause.core.vp[0].object"));
    assert!(error.msg.contains("passive clause promotes the patient"));
}

#[test]
fn invalid_raw_trees_are_rejected_before_resolution_or_planning() {
    let raw: RawClause = clause(coordinate(Conj::I, vec![]), vp("spati"));
    assert!(matches!(
        validate(&raw),
        Err(ValidationErrors(errors))
            if errors.iter().any(|error| matches!(
                error.kind,
                ValidationErrorKind::EmptyCoordination
            ))
    ));
    assert!(matches!(
        realize(&raw, RealizeOpts::sentence()),
        Err(PhraseError::Validation(_))
    ));

    let lossy_reference = clause(
        np("mųž")
            .relative(RelClause::subject_gap(vp("spati")))
            .referential(ReferentialForm::Pronoun),
        vp("spati"),
    );
    assert!(matches!(
        validate(&lossy_reference),
        Err(ValidationErrors(errors))
            if errors.iter().any(|error| matches!(
                error.kind,
                ValidationErrorKind::InvalidReferentialForm(_)
            ))
    ));

    let copular = copular(np("dom"), Predicate::Adjectival("veliky".into()));
    assert!(matches!(
        copular.clone().and_vp(vp("stojati")),
        Err(BuildError::VerbalCoreRequired("and_vp"))
    ));
    assert!(matches!(
        copular.clone().conj(Conj::Ili),
        Err(BuildError::VerbalCoreRequired("conj"))
    ));
    assert!(matches!(
        validate(&copular.passive()),
        Err(ValidationErrors(errors))
            if errors.iter().any(|error| matches!(
                error.kind,
                ValidationErrorKind::IncoherentClause(
                    "passive voice requires a verbal clause core"
                )
            ))
    ));
}

#[test]
fn object_gap_valence_is_resolved_like_an_overt_direct_object() {
    let tree = clause(
        np("kniga").relative(RelClause::object_gap(np("otėc"), vp("spati"))),
        vp("ležati"),
    );
    assert!(matches!(
        realize(&tree, RealizeOpts::sentence()),
        Err(PhraseError::Resolution(ResolutionErrors(errors)))
            if errors.iter().any(|error| {
                error.path
                    == "clause.subject.relative.gap"
                    && matches!(
                        error.kind,
                        ResolutionErrorKind::ObjectOfIntransitive { .. }
                    )
            })
    ));
}

#[test]
fn all_force_mood_voice_tense_combinations_are_decided_and_panic_free() {
    let forces = [
        Force::Declarative,
        Force::IntonationQuestion,
        Force::CiQuestion,
        Force::LiQuestion,
        Force::Imperative(Addressee::You),
        Force::Imperative(Addressee::We),
        Force::Imperative(Addressee::YouAll),
    ];
    let moods = [Mood::Indicative, Mood::Conditional];
    let voices = [Voice::Active, Voice::Passive];
    let tenses = [TenseSpec::Present, TenseSpec::Past, TenseSpec::Future];

    let mut combinations = 0;
    for force in forces {
        for mood in moods {
            for voice in voices {
                for tense in tenses {
                    combinations += 1;
                    let mut raw = clause(np("kniga"), vp("kupiti")).force(force).tense(tense);
                    if mood == Mood::Conditional {
                        raw = raw.conditional();
                    }
                    if voice == Voice::Passive {
                        raw = raw.passive();
                    }

                    let imperative = matches!(force, Force::Imperative(_));
                    let expected_valid = !(imperative
                        && (mood == Mood::Conditional
                            || voice == Voice::Passive
                            || tense != TenseSpec::Present))
                        && !(mood == Mood::Conditional && tense != TenseSpec::Present);
                    let validated = validate(&raw);
                    assert_eq!(
                        validated.is_ok(),
                        expected_valid,
                        "unexpected decision for {force:?}/{mood:?}/{voice:?}/{tense:?}"
                    );
                    if let Ok(validated) = validated {
                        let result = catch_unwind(AssertUnwindSafe(|| {
                            realize_validated_checked(&validated, RealizeOpts::sentence())
                        }));
                        assert!(
                            result.is_ok(),
                            "validated realization panicked for {force:?}/{mood:?}/{voice:?}/{tense:?}"
                        );
                    }
                }
            }
        }
    }
    assert_eq!(combinations, 84);
}

#[test]
fn information_structure_references_are_validated_structurally() {
    let missing = clause(np("otėc"), vp("spati")).topic(SlotRef::Object);
    assert!(matches!(
        validate(&missing),
        Err(ValidationErrors(errors))
            if errors.iter().any(|error| matches!(
                error.kind,
                ValidationErrorKind::MissingInformationSlot(SlotRef::Object)
            ))
    ));

    let duplicate = clause(np("otėc"), vp("kupiti").object(np("kniga")))
        .topic(SlotRef::Object)
        .focus(SlotRef::Object);
    assert!(matches!(
        validate(&duplicate),
        Err(ValidationErrors(errors))
            if errors.iter().any(|error| matches!(
                error.kind,
                ValidationErrorKind::DuplicateInformationSlot(SlotRef::Object)
            ))
    ));
}

#[test]
fn checked_discourse_retains_nested_diagnostic_paths() {
    let story = vec![DiscourseSentence::new(clause(
        np("žena"),
        vp("viděti")
            .object(np("mųž").relative(RelClause::subject_gap(vp("kupiti").object(np("kniga"))))),
    ))];
    let narrated = narrate_checked(story, RealizeOpts::sentence()).unwrap();
    assert!(narrated.warnings.iter().any(|warning| matches!(
        warning,
        PhraseWarning::PerfectivePresent { path, verb }
            if path == "sentence[0].clause.core.vp[0].object.nominal.relative.vp"
                && verb == "kupiti"
    )));
}

#[test]
fn discourse_tracks_but_does_not_pronominalize_nominal_predicates() {
    let story = vec![
        DiscourseSentence::new(copular(
            np("krålj"),
            Predicate::Nominal(np("učitelj").entity("role")),
        )),
        DiscourseSentence::new(copular(
            np("žena"),
            Predicate::Nominal(np("učitelj").entity("role")),
        )),
    ];
    assert_eq!(
        narrate(story, RealizeOpts::sentence()).unwrap(),
        "Krålj jest učitelj. Žena jest učitelj."
    );
}

fn generated_atoms() -> Vec<String> {
    let mut atoms = vec![
        "(".to_string(),
        ")".to_string(),
        "two words".to_string(),
        ":leading-key-shape".to_string(),
        "(parentheses)".to_string(),
        "a \"quote\"".to_string(),
        r"a\\backslash".to_string(),
        "line\nbreak".to_string(),
        "tab\tbreak".to_string(),
        "žųłva 世界 🙂".to_string(),
        "12345".to_string(),
    ];
    let alphabet = [
        'a', 'Z', '0', ' ', ':', '(', ')', '"', '\\', '\n', '\r', '\t', 'ž', 'ų', '界', '🙂',
    ];
    let mut state = 0x9e37_79b9_7f4a_7c15_u64;
    for sample in 0..512 {
        state ^= state << 7;
        state ^= state >> 9;
        state ^= state << 8;
        let len = 1 + (state as usize % 24);
        let mut atom = format!("g{sample:x}");
        for _ in 0..len {
            state ^= state << 7;
            state ^= state >> 9;
            state ^= state << 8;
            atom.push(alphabet[state as usize % alphabet.len()]);
        }
        atoms.push(atom);
    }
    atoms
}

fn assert_roundtrip(tree: Clause) {
    validate(&tree).unwrap();
    let printed = print(&tree);
    let reparsed = clause_from_str(&printed).unwrap_or_else(|error| {
        panic!("failed to parse generated canonical tree `{printed}`: {error}")
    });
    assert_eq!(reparsed, tree, "roundtrip changed `{printed}`");
}

#[test]
fn generated_escaped_atoms_roundtrip_in_every_free_text_position() {
    for (index, atom) in generated_atoms().into_iter().enumerate() {
        let referential = match index % 3 {
            0 => ReferentialForm::Full,
            1 => ReferentialForm::Pronoun,
            _ => ReferentialForm::Clitic,
        };
        assert_roundtrip(clause(
            np(&atom)
                .det(&atom)
                .adj(&atom)
                .entity(&atom)
                .referential(referential),
            vp(&atom)
                .adv(&atom)
                .object(name(&atom, Gender::Feminine))
                .pp(pp("od", name(&atom, Gender::Masculine))),
        ));
        assert_roundtrip(copular(np("dom"), Predicate::Adjectival(atom.clone())));
        assert_roundtrip(copular(np("komnata"), Predicate::Participial(atom.clone())));
        assert_roundtrip(clause(
            np("dom").relative(RelClause::pp_gap(
                "v",
                Case::Loc,
                np(&atom),
                vp(&atom).adv(&atom),
            )),
            vp("stojati"),
        ));
    }
}

#[test]
fn generated_malformed_sexprs_never_panic() {
    let alphabet = ['(', ')', ':', '"', '\\', ' ', '\n', 'a', '0', 'ž', '🙂'];
    let mut state = 0xd1b5_4a32_d192_ed03_u64;
    for _ in 0..2_048 {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let len = state as usize % 96;
        let mut input = String::new();
        for _ in 0..len {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            input.push(alphabet[state as usize % alphabet.len()]);
        }
        assert!(
            catch_unwind(AssertUnwindSafe(|| clause_from_str(&input))).is_ok(),
            "parser panicked for generated input {input:?}"
        );
    }

    let deeply_nested = format!("{}x{}", "(".repeat(100_000), ")".repeat(100_000));
    let result = catch_unwind(AssertUnwindSafe(|| parse(&deeply_nested)));
    assert!(matches!(
        result,
        Ok(Err(error)) if error.msg.contains("maximum nesting depth")
    ));

    let mut value = Value::Sym("x".into(), 0);
    for at in 0..600 {
        value = Value::List(vec![value], at);
    }
    let result = catch_unwind(AssertUnwindSafe(|| compile_clause(&value)));
    assert!(matches!(
        result,
        Ok(Err(error)) if error.msg.contains("maximum nesting depth")
    ));
}

#[test]
fn typed_and_serialized_depth_share_one_validation_limit() {
    let mut nominal: Nominal = np("dom").into();
    for _ in 0..=MAX_STRUCTURE_DEPTH {
        nominal = coordinate(Conj::I, vec![nominal]);
    }
    assert!(matches!(
        validate(&clause(nominal, vp("stojati"))),
        Err(ValidationErrors(errors))
            if matches!(
                errors.as_slice(),
                [ValidationError {
                    kind: ValidationErrorKind::MaximumDepth {
                        limit: MAX_STRUCTURE_DEPTH
                    },
                    ..
                }]
            )
    ));
}
