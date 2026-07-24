//! Regression suite for the second PR #34 review round
//! (review/pr34-code-review.txt). Every reproduction from the review is
//! pinned here with its corrected behavior: reflexive-construction
//! government, scope-safe aggregation, semantic discourse number,
//! rejected-not-dropped information structure, the `či` clitic domain,
//! member-derived coordination animacy, nominal-only predicate case,
//! validated PP gaps, and the accusative-gated syncretism warning.

use interslavic::{Case, Gender, Number, Person};
use interslavic_phrase::discourse::{Connective, DiscourseSentence, narrate};
use interslavic_phrase::*;

fn s(tree: &Clause) -> String {
    realize(tree, RealizeOpts::sentence()).unwrap()
}

// --- Finding 1: verb government reaches the actual construction ----------

#[test]
fn reflexive_construction_reads_its_own_government() {
    // "ostrěgati sę (+2)" is its own dictionary row: the reflexive VP
    // takes a genitive object, while the bare transitive row keeps its
    // accusative default.
    let tree = clause(np("krålj"), vp("ostrěgati sę").object(np("voda")));
    assert_eq!(s(&tree), "Krålj ostrěgaje sę vody.");
    let tree = clause(np("krålj"), vp("ostrěgati").object(np("voda")));
    assert_eq!(s(&tree), "Krålj ostrěgaje vodų.");
}

#[test]
fn reflexive_government_applies_inside_relative_gaps() {
    // The relativizer's case comes from the reflexive construction too.
    let tree = clause(
        np("voda").relative(RelClause::object_gap(np("krålj"), vp("ostrěgati sę"))),
        vp("tekti"),
    );
    assert_eq!(s(&tree), "Voda, ktoroj krålj ostrěgaje sę, teče.");
}

#[test]
fn government_conventions_hold_for_annotated_rows() {
    // Hint + (+2) on one bare row.
    let tree = clause(np("krålj"), vp("izběgti").object(np("voda"))).past();
    assert_eq!(s(&tree), "Krålj izběgl vody.");
    // Duplicate-row lemmas follow the documented first-entry
    // convention: izbaviti's plain v.tr. row precedes its (+2) row, so
    // the default stays accusative — an explicit override reaches the
    // genitive sense.
    let tree = clause(np("krålj"), vp("izbaviti").object(np("voda"))).past();
    assert_eq!(s(&tree), "Krålj izbavil vodų.");
    let tree = clause(
        np("krålj"),
        vp("izbaviti").object(np("voda").case(Case::Gen)),
    )
    .past();
    assert_eq!(s(&tree), "Krålj izbavil vody.");
}

// --- Finding 2: aggregation cannot change conjunction scope --------------

#[test]
fn non_i_coordination_is_never_flattened() {
    // "čitaje ili piše" + independently asserted "spi": flattening would
    // demote the assertion to an alternative, so the sentences stay
    // separate (and the second still pronominalizes).
    let story = vec![
        DiscourseSentence::new(
            clause(np("krålj").entity("k"), vp("čitati"))
                .and_vp(vp("pisati"))
                .conj(Conj::Ili),
        ),
        DiscourseSentence::new(clause(np("krålj").entity("k"), vp("spati"))),
    ];
    assert_eq!(
        narrate(story, RealizeOpts::sentence()).unwrap(),
        "Krålj čitaje ili piše. On spi."
    );
}

#[test]
fn dormant_singleton_conjunction_canonicalizes_to_i() {
    // A one-item coordination "realizes as itself (no conjunction)", so
    // its conjunction field is meaningless; aggregation makes the merged
    // list explicitly `i` instead of letting a dormant `ili` surface.
    let story = vec![
        DiscourseSentence::new(clause(np("krålj").entity("k"), vp("čitati")).conj(Conj::Ili)),
        DiscourseSentence::new(clause(np("krålj").entity("k"), vp("spati"))),
    ];
    assert_eq!(
        narrate(story, RealizeOpts::sentence()).unwrap(),
        "Krålj čitaje i spi."
    );
}

#[test]
fn i_lists_still_aggregate_and_prodrop_must_match() {
    // Plain `i` semantics on both sides: the merge is safe and happens.
    let story = vec![
        DiscourseSentence::new(clause(np("krålj").entity("k"), vp("čitati")).and_vp(vp("pisati"))),
        DiscourseSentence::new(clause(np("krålj").entity("k"), vp("spati"))),
    ];
    assert_eq!(
        narrate(story, RealizeOpts::sentence()).unwrap(),
        "Krålj čitaje, piše i spi."
    );
    // A differing prodrop choice keeps the clauses separate.
    let story = vec![
        DiscourseSentence::new(clause(np("krålj").entity("k"), vp("čitati"))),
        DiscourseSentence::new(clause(np("krålj").entity("k"), vp("spati")).prodrop()),
    ];
    assert_eq!(
        narrate(story, RealizeOpts::sentence()).unwrap(),
        "Krålj čitaje. Spi."
    );
}

// --- Finding 3: discourse number is the referent's semantic number -------

#[test]
fn counted_entities_pronominalize_by_semantic_number() {
    let counted = |n: u64| {
        let story = vec![
            DiscourseSentence::new(
                clause(
                    np("krålj").entity("k"),
                    vp("kupiti").object(np("kniga").count(n).entity("b")),
                )
                .past(),
            ),
            DiscourseSentence::new(
                clause(
                    np("krålj").entity("k"),
                    vp("pročitati").object(np("kniga").count(n).entity("b")),
                )
                .past(),
            )
            .connective(Connective::Potom),
        ];
        narrate(story, RealizeOpts::sentence()).unwrap()
    };
    // Count 1 is singular; every other count (0 included — POLICY)
    // refers to a plurality — "je" is the facade's feminine accusative
    // PLURAL, where the defect produced the singular "jų".
    assert_eq!(counted(1), "Krålj kupil 1 knigų. Potom on pročital jų.");
    assert_eq!(counted(2), "Krålj kupil 2 knigy. Potom on pročital je.");
    assert_eq!(counted(5), "Krålj kupil 5 knig. Potom on pročital je.");
    assert_eq!(counted(0), "Krålj kupil 0 knig. Potom on pročital je.");
}

#[test]
fn plural_only_entities_stay_plural() {
    let story = vec![
        DiscourseSentence::new(
            clause(
                np("krålj").entity("k"),
                vp("kupiti").object(np("noviny").entity("n")),
            )
            .past(),
        ),
        DiscourseSentence::new(
            clause(
                np("krålj").entity("k"),
                vp("viděti").object(np("noviny").entity("n")),
            )
            .past(),
        )
        .connective(Connective::Potom),
    ];
    assert_eq!(
        narrate(story, RealizeOpts::sentence()).unwrap(),
        "Krålj kupil noviny. Potom on viděl je."
    );
}

// --- Finding 4: information structure is validated, never dropped --------

#[test]
fn li_question_focus_must_reference_an_existing_slot() {
    // A focused object the clause does not have.
    let tree = clause(np("otėc"), vp("spati"))
        .force(Force::LiQuestion)
        .focus(SlotRef::Object);
    assert!(matches!(
        realize(&tree, RealizeOpts::sentence()),
        Err(PhraseError::IncoherentClause(_))
    ));
    // A focused subject the clause drops.
    let tree = clause(
        pron(Person::First, Number::Singular, Gender::Masculine),
        vp("spati"),
    )
    .force(Force::LiQuestion)
    .focus(SlotRef::Subject)
    .prodrop();
    assert!(matches!(
        realize(&tree, RealizeOpts::sentence()),
        Err(PhraseError::IncoherentClause(_))
    ));
}

#[test]
fn declarative_topic_and_focus_are_validated_too() {
    let tree = clause(np("otėc"), vp("spati")).topic(SlotRef::Object);
    assert!(matches!(
        realize(&tree, RealizeOpts::sentence()),
        Err(PhraseError::IncoherentClause(_))
    ));
    let tree = clause(
        pron(Person::First, Number::Singular, Gender::Masculine),
        vp("spati"),
    )
    .focus(SlotRef::Subject)
    .prodrop();
    assert!(matches!(
        realize(&tree, RealizeOpts::sentence()),
        Err(PhraseError::IncoherentClause(_))
    ));
}

// --- Finding 5: či is part of the clitic domain --------------------------

#[test]
fn second_position_clitics_follow_ci() {
    let tree = clause(np("krålj"), vp("myti sę"))
        .force(Force::CiQuestion)
        .past();
    assert_eq!(
        realize(
            &tree,
            RealizeOpts::sentence().clitics(CliticStyle::SecondPosition)
        )
        .unwrap(),
        "Či sę krålj myl?"
    );
    // Postverbal placement is unaffected by the fronted particle.
    assert_eq!(s(&tree), "Či krålj myl sę?");
}

// --- Finding 6: coordination animacy comes from the members --------------

#[test]
fn coordination_animacy_is_member_derived() {
    // All-inanimate: the predicate declines inanimate ("nove", not
    // "novi").
    let tree = copular(
        coordinate(Conj::I, vec![np("dom").into(), np("stol").into()]),
        Predicate::Adjectival("novy".into()),
    );
    assert_eq!(s(&tree), "Dom i stol sųt nove.");
    // All-animate stays animate.
    let tree = copular(
        coordinate(Conj::I, vec![np("krålj").into(), np("otėc").into()]),
        Predicate::Adjectival("dobry".into()),
    );
    assert_eq!(s(&tree), "Krålj i otėc sųt dobri.");
    // Mixed animacy resolves animate (POLICY, the masculine-personal
    // pattern — mirroring the mixed-gender → masculine rule).
    let tree = copular(
        coordinate(Conj::I, vec![np("krålj").into(), np("stol").into()]),
        Predicate::Adjectival("dobry".into()),
    );
    assert_eq!(s(&tree), "Krålj i stol sųt dobri.");
}

// --- Finding 7: PredCase is nominal-only ---------------------------------

#[test]
fn instrumental_pred_case_is_nominal_only() {
    // The nominal instrumental predicate still works.
    let tree = Clause::with_core(
        np("otėc"),
        ClauseCore::Copular {
            predicate: Predicate::Nominal(np("krålj")),
            pred_case: PredCase::Instrumental,
        },
    )
    .past();
    assert_eq!(s(&tree), "Otėc byl kråljem.");
    // On adjectival and participial predicates it is rejected.
    for predicate in [
        Predicate::Adjectival("dobry".into()),
        Predicate::Participial("kupiti".into()),
    ] {
        let tree = Clause::with_core(
            np("krålj"),
            ClauseCore::Copular {
                predicate,
                pred_case: PredCase::Instrumental,
            },
        );
        assert!(matches!(
            realize(&tree, RealizeOpts::sentence()),
            Err(PhraseError::IncoherentClause(_))
        ));
    }
    // The S-expression reader rejects the same combination at parse
    // time; the nominal form still compiles.
    assert!(clause_from_str("(clause (np (n krålj)) (pred (adj dobry)) :pred-case ins)").is_err());
    assert!(clause_from_str("(clause (np (n otėc)) (pred (np (n krålj))) :pred-case ins)").is_ok());
}

// --- Finding 8: PP gaps pass the ordinary government check ---------------

#[test]
fn pp_relative_gaps_validate_their_preposition() {
    let with_gap = |preposition: &str, case: Case| {
        clause(
            np("stol").relative(RelClause {
                gap: GapRole::PpObject {
                    preposition: preposition.into(),
                    case,
                },
                subject: Some(np("kot").into()),
                vp: vp("spati"),
                tense: TenseSpec::Present,
                polarity: Polarity::Affirmative,
                relativizer: Relativizer::Ktory,
            }),
            vp("stojati"),
        )
    };
    // A licensed pairing realizes.
    assert_eq!(
        s(&with_gap("pod", Case::Ins)),
        "Stol, pod ktorym kot spi, stoji."
    );
    // The same errors as an ordinary PP: wrong case, unknown
    // preposition.
    assert!(matches!(
        realize(&with_gap("pod", Case::Dat), RealizeOpts::sentence()),
        Err(PhraseError::InvalidPrepositionCase { .. })
    ));
    assert!(matches!(
        realize(&with_gap("blorp", Case::Dat), RealizeOpts::sentence()),
        Err(PhraseError::UnknownPreposition(_))
    ));
}

// --- Finding 9: the syncretism warning is accusative-gated ---------------

#[test]
fn oblique_objects_do_not_trigger_ambiguous_order() {
    // Dictionary-governed dative: "oknu" already disambiguates.
    let realized = realize_checked(
        &clause(np("stol"), vp("dękovati").object(np("okno"))).topic(SlotRef::Object),
        RealizeOpts::sentence(),
    )
    .unwrap();
    assert_eq!(realized.text, "Oknu stol dękuje.");
    assert!(realized.warnings.is_empty());

    // An explicit oblique override disambiguates the same way.
    let realized = realize_checked(
        &clause(np("avto"), vp("vladati").object(np("okno").case(Case::Ins)))
            .topic(SlotRef::Object),
        RealizeOpts::sentence(),
    )
    .unwrap();
    assert_eq!(realized.text, "Oknom avto vladaje.");
    assert!(realized.warnings.is_empty());

    // The genuinely syncretic accusative inversion still warns.
    let realized = realize_checked(
        &clause(np("avto"), vp("viděti").object(np("nebo"))).topic(SlotRef::Object),
        RealizeOpts::sentence(),
    )
    .unwrap();
    assert_eq!(realized.text, "Nebo avto vidi.");
    assert_eq!(realized.warnings, vec![PhraseWarning::AmbiguousOrder]);
}
