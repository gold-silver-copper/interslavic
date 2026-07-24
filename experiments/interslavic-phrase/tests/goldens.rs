//! Golden sentences: steen's own examples (syntax and pronoun pages),
//! re-spelled to the flavored orthography this workspace uses and pinned
//! against actual facade output. Where steen's legacy plain orthography
//! and the facade differ (knigu → knigų, se → sę, otec → otėc), the
//! facade wins — the parity-standard rule — and the divergence is noted
//! inline. These strings are the linearization contract; `cargo xtask
//! phrase-check` additionally runs every golden through slovowiki's
//! independent agreement checker.

use interslavic::{Case, Gender, Number, Person};
use interslavic_phrase::*;

fn sentence(tree: &Clause) -> String {
    realize(tree, RealizeOpts::sentence()).unwrap()
}

#[test]
fn steen_declaratives() {
    // steen: "Otec kupil knigu." (flavored: otėc, knigų)
    let tree = clause(np("otėc"), vp("kupiti").object(np("kniga"))).past();
    assert_eq!(sentence(&tree), "Otėc kupil knigų.");

    // steen (pronouns page): "Ja myju se" — flavored myjų, sę; the
    // reflexive comes from the lemma spelling "myti sę".
    let tree = clause(
        pron(Person::First, Number::Singular, Gender::Masculine),
        vp("myti sę"),
    );
    assert_eq!(sentence(&tree), "Ja myjų sę.");
}

#[test]
fn steen_questions() {
    let declarative = || clause(np("otėc"), vp("kupiti").object(np("kniga"))).past();
    assert_eq!(
        sentence(&declarative().force(Force::IntonationQuestion)),
        "Otėc kupil knigų?"
    );
    assert_eq!(
        sentence(&declarative().force(Force::CiQuestion)),
        "Či otėc kupil knigų?"
    );
    assert_eq!(
        sentence(&declarative().force(Force::LiQuestion)),
        "Kupil li otėc knigų?"
    );
}

#[test]
fn steen_stable_vs_motion_pp() {
    // steen: "Kot spi pod stolom" / "Kot poběgl pod stol" — the Acc/Ins
    // pair that motivates explicit sense selection on `pod`.
    let stable = clause(
        np("kot"),
        vp("spati").pp(pp("pod", np("stol")).case(Case::Ins)),
    );
    assert_eq!(sentence(&stable), "Kot spi pod stolom.");

    let motion = clause(
        np("kot"),
        vp("poběgti").pp(pp("pod", np("stol")).case(Case::Acc)),
    )
    .past();
    assert_eq!(sentence(&motion), "Kot poběgl pod stol.");
}

#[test]
fn flagship_quantified_agreement() {
    let tree = clause(
        np("krålj").det("toj").adj("dobry"),
        vp("ukrasti").object(np("moneta").count(5).adj("zlåty")),
    )
    .past();
    assert_eq!(sentence(&tree), "Toj dobry krålj ukradl 5 zlåtyh monet.");
}

#[test]
fn negation_feminine_past_and_pronoun_pp() {
    // Negation before the finite complex.
    let tree = clause(np("otėc"), vp("kupiti").object(np("kniga")))
        .past()
        .negated();
    assert_eq!(sentence(&tree), "Otėc ne kupil knigų.");

    // Feminine subject gender flows into perfect_parts.
    let tree = clause(np("žena"), vp("kupiti").object(np("kniga"))).past();
    assert_eq!(sentence(&tree), "Žena kupila knigų.");

    // A pronoun inside a PP takes the prepositional n- form automatically.
    let tree = clause(
        np("otėc"),
        vp("kupiti").object(np("kniga")).pp(pp(
            "za",
            pron(Person::Third, Number::Singular, Gender::Masculine),
        )
        .case(Case::Acc)),
    )
    .past();
    assert_eq!(sentence(&tree), "Otėc kupil knigų za njego.");
}

#[test]
fn diagnostics_are_values() {
    // Ambiguous preposition without a case: the error lists the senses.
    let tree = clause(np("kot"), vp("spati").pp(pp("pod", np("stol"))));
    match realize(&tree, RealizeOpts::sentence()) {
        Err(PhraseError::AmbiguousPreposition {
            preposition,
            senses,
        }) => {
            assert_eq!(preposition, "pod");
            assert_eq!(senses.len(), 2);
        }
        other => panic!("expected AmbiguousPreposition, got {other:?}"),
    }

    // Object under a dictionary-intransitive verb.
    let tree = clause(np("otėc"), vp("spati").object(np("kniga")));
    assert!(matches!(
        realize(&tree, RealizeOpts::sentence()),
        Err(PhraseError::ObjectOfIntransitive { .. })
    ));

    // Strict mode rejects guessed heads; default mode trusts the guess.
    let tree = clause(np("glorbina"), vp("spati"));
    assert!(matches!(
        realize(&tree, RealizeOpts::sentence().strict()),
        Err(PhraseError::GuessedHead { .. })
    ));
    assert!(realize(&tree, RealizeOpts::sentence()).is_ok());
}

// ---------------------------------------------------------------------------
// 0.2.0: stages 1–4 (clause core, deferred syntax, information structure,
// discourse). Every pinned form verified against facade output first.
// ---------------------------------------------------------------------------

#[test]
fn copular_predicates() {
    // Nominal predicate (nominative default — policy; steen silent).
    let tree = copular(
        pron(Person::Third, Number::Singular, Gender::Masculine),
        Predicate::Nominal(np("krålj")),
    );
    assert_eq!(sentence(&tree), "On jest krålj.");

    // steen's own negated adjectival predicate: "Tuto avto ne jest drago."
    let tree = copular(
        np("avto").det("tutoj"),
        Predicate::Adjectival("dragy".into()),
    )
    .negated();
    assert_eq!(sentence(&tree), "Tuto avto ne jest drago.");

    // Adjective agreement with a feminine subject.
    let tree = copular(np("komnata"), Predicate::Adjectival("veliky".into()));
    assert_eq!(sentence(&tree), "Komnata jest velika.");

    // Participial predicate — the passive-participle construction.
    let tree = copular(np("komnata"), Predicate::Participial("osvětliti".into()));
    assert_eq!(sentence(&tree), "Komnata jest osvětljena.");

    // Past copula, feminine agreement.
    let tree = copular(np("žena"), Predicate::Adjectival("dobry".into())).past();
    assert_eq!(sentence(&tree), "Žena byla dobra.");

    // Instrumental predicate as the exposed option.
    let tree = Clause::with_core(
        pron(Person::Third, Number::Singular, Gender::Masculine),
        ClauseCore::Copular {
            predicate: Predicate::Nominal(np("krålj")),
            pred_case: PredCase::Instrumental,
        },
    )
    .past();
    assert_eq!(sentence(&tree), "On byl kråljem.");
}

#[test]
fn passive_voice() {
    // steen: the agent is "either in the instrumental case or preceded
    // by the preposition od with the genitive".
    let tree = clause(np("kniga"), vp("kupiti").pp(pp("od", np("otėc"))))
        .passive()
        .past();
    assert_eq!(sentence(&tree), "Kniga byla kupjena od otca.");

    let err = realize(
        &clause(np("kniga"), vp("kupiti").object(np("moneta"))).passive(),
        RealizeOpts::sentence(),
    )
    .unwrap_err();
    assert!(matches!(err, PhraseError::ObjectInPassive { .. }));
}

#[test]
fn moods() {
    // Imperative: subject omitted, ! punctuation.
    let tree = clause(
        pron(Person::Second, Number::Singular, Gender::Masculine),
        vp("kupiti").object(np("kniga")),
    )
    .force(Force::Imperative(Addressee::You));
    assert_eq!(sentence(&tree), "Kupi knigų!");

    let tree = clause(
        pron(Person::Second, Number::Plural, Gender::Masculine),
        vp("kupiti").object(np("kniga")),
    )
    .force(Force::Imperative(Addressee::YouAll))
    .negated();
    assert_eq!(sentence(&tree), "Ne kupite knigų!");

    // Conditional: person-marked auxiliary from the paradigm row.
    let tree = clause(np("otėc"), vp("kupiti").object(np("kniga"))).conditional();
    assert_eq!(sentence(&tree), "Otėc by kupil knigų.");
    let tree = clause(
        pron(Person::First, Number::Singular, Gender::Feminine),
        vp("kupiti").object(np("kniga")),
    )
    .conditional();
    assert_eq!(sentence(&tree), "Ja byh kupila knigų.");
}

#[test]
fn verb_government_from_dictionary() {
    // dękovati (+3): dative object with no annotation in the tree.
    let tree = clause(
        pron(Person::First, Number::Singular, Gender::Masculine),
        vp("dękovati").object(pron(Person::Second, Number::Singular, Gender::Masculine)),
    );
    assert_eq!(sentence(&tree), "Ja dękujų tobě.");

    // vladati (+5): instrumental object.
    let tree = clause(np("krålj"), vp("vladati").object(np("zemja")));
    assert_eq!(sentence(&tree), "Krålj vladaje zemjejų.");

    // Conflicting explicit case is honored but warned about.
    let tree = clause(
        np("krålj"),
        vp("vladati").object(np("zemja").case(Case::Acc)),
    );
    let realized = realize_checked(&tree, RealizeOpts::sentence()).unwrap();
    assert_eq!(realized.text, "Krålj vladaje zemjų.");
    assert!(matches!(
        realized.warnings[..],
        [PhraseWarning::GovernsConflict { .. }]
    ));
}

#[test]
fn aspect_warning_and_adverb_and_name() {
    // Perfective present warns (reads as future/completed).
    let realized = realize_checked(
        &clause(np("otėc"), vp("kupiti").object(np("kniga"))),
        RealizeOpts::sentence(),
    )
    .unwrap();
    assert!(matches!(
        realized.warnings[..],
        [PhraseWarning::PerfectivePresent { .. }]
    ));

    // Adverb before the verb (policy), declined name subject.
    let tree = clause(name("Anna", Gender::Feminine), vp("spati").adv("dobro"));
    assert_eq!(sentence(&tree), "Anna dobro spi.");
    let tree = clause(
        np("otėc"),
        vp("kupiti")
            .object(np("kniga"))
            .pp(pp("za", name("Anna", Gender::Feminine)).case(Case::Acc)),
    )
    .past();
    assert_eq!(sentence(&tree), "Otėc kupil knigų za Annų.");
}

#[test]
fn relative_clauses() {
    // Subject gap: relativizer Nom, agrees with the head.
    let tree = copular(
        np("krålj").relative(
            RelClause::subject_gap(vp("ukrasti").object(np("moneta").count(5).adj("zlåty"))).past(),
        ),
        Predicate::Adjectival("dobry".into()),
    )
    .negated();
    assert_eq!(
        sentence(&tree),
        "Krålj, ktory ukradl 5 zlåtyh monet, ne jest dobry."
    );

    // Object gap: relativizer takes the gap's accusative, feminine from
    // the head; trailing comma survives mid-sentence.
    let tree = copular(
        np("moneta").relative(RelClause::object_gap(np("krålj"), vp("ukrasti")).past()),
        Predicate::Adjectival("zlåty".into()),
    );
    assert_eq!(sentence(&tree), "Moneta, ktorų krålj ukradl, jest zlåta.");

    // iže is a declared gap, not silent output.
    let tree = copular(
        np("moneta").relative(
            RelClause::object_gap(np("krålj"), vp("ukrasti"))
                .past()
                .relativizer(Relativizer::Iže),
        ),
        Predicate::Adjectival("zlåty".into()),
    );
    assert!(matches!(
        realize(&tree, RealizeOpts::sentence()),
        Err(PhraseError::Unsupported(_))
    ));
}

#[test]
fn coordination() {
    // Coordinated subject: plural agreement, masculine for mixed gender.
    let tree = clause(
        coordinate(Conj::I, vec![np("otėc").into(), np("žena").into()]),
        vp("kupiti").object(np("kniga")),
    )
    .past();
    assert_eq!(sentence(&tree), "Otėc i žena kupili knigų.");

    // VP coordination with per-VP objects.
    let tree = clause(np("krålj"), vp("kupiti"))
        .and_vp(vp("pročitati").object(np("kniga")))
        .past();
    assert_eq!(sentence(&tree), "Krålj kupil i pročital knigų.");
}

#[test]
fn clitic_styles() {
    // Postverbal default (steen's examples).
    let tree = clause(
        np("krålj"),
        vp("viděti").object(pron_clitic(
            Person::Third,
            Number::Singular,
            Gender::Masculine,
        )),
    );
    assert_eq!(sentence(&tree), "Krålj vidi go.");

    // Second position (Wackernagel).
    assert_eq!(
        realize(
            &tree,
            RealizeOpts::sentence().clitics(CliticStyle::SecondPosition)
        )
        .unwrap(),
        "Krålj go vidi."
    );

    // li stays first in the cluster (Franks & King order).
    let question = tree.clone().force(Force::LiQuestion);
    assert_eq!(
        realize(
            &question,
            RealizeOpts::sentence().clitics(CliticStyle::SecondPosition)
        )
        .unwrap(),
        "Vidi li go krålj?"
    );
}

#[test]
fn information_structure() {
    // Topicalized object (theme-first): the topic fronts, the rest
    // keeps its order — O S V.
    let tree = clause(np("otėc"), vp("kupiti").object(np("kniga")))
        .past()
        .topic(SlotRef::Object);
    assert_eq!(sentence(&tree), "Knigų otėc kupil.");

    // Topic + subject focus gives the classic emphatic OVS ("it was
    // father who bought the book").
    let tree = clause(np("otėc"), vp("kupiti").object(np("kniga")))
        .past()
        .topic(SlotRef::Object)
        .focus(SlotRef::Subject);
    assert_eq!(sentence(&tree), "Knigų kupil otėc.");

    // Focused-object li question: li right after the fronted focus.
    let tree = clause(np("otėc"), vp("kupiti").object(np("kniga")))
        .past()
        .force(Force::LiQuestion)
        .focus(SlotRef::Object);
    assert_eq!(sentence(&tree), "Knigų li kupil otėc?");

    // Syncretism guard: neuter subject and object are Nom/Acc-identical,
    // so a marked order draws steen's clarity warning.
    let tree = clause(np("avto"), vp("viděti").object(np("nebo"))).topic(SlotRef::Object);
    let realized = realize_checked(&tree, RealizeOpts::sentence()).unwrap();
    assert!(realized.warnings.contains(&PhraseWarning::AmbiguousOrder));
}

#[test]
fn discourse_narrative() {
    use interslavic_phrase::discourse::*;
    // Aggregation: same subject entity, tense, force — merges to VP
    // coordination; then the referring-expression pass pronominalizes.
    let story = vec![
        DiscourseSentence::new(
            clause(
                np("krålj").entity("k").det("toj"),
                vp("kupiti").object(np("kniga").entity("b")),
            )
            .past(),
        ),
        DiscourseSentence::new(clause(np("krålj").entity("k"), vp("spati")).past()),
        DiscourseSentence::new(
            clause(
                np("krålj").entity("k"),
                vp("pročitati").object(np("kniga").entity("b")),
            )
            .past(),
        )
        .connective(Connective::Potom),
    ];
    // Sentences 1+2 aggregate ("kupil knigų i spal"); sentence 3 keeps
    // its connective and pronominalizes both entities.
    assert_eq!(
        narrate(story, RealizeOpts::sentence()).unwrap(),
        "Toj krålj kupil knigų i spal. Potom on pročital jų."
    );

    // Interference: a second same-gender entity forces the full NP back.
    let story = vec![
        DiscourseSentence::new(clause(np("krålj").entity("k"), vp("spati"))),
        DiscourseSentence::new(clause(np("otėc").entity("o"), vp("spati"))),
        DiscourseSentence::new(clause(np("krålj").entity("k"), vp("spati"))),
    ];
    assert_eq!(
        narrate(story, RealizeOpts::sentence()).unwrap(),
        "Krålj spi. Otėc spi. Krålj spi."
    );
}
