//! Regression suite for the PR #34 review findings. Every probe sentence
//! the review used to confirm a defect is pinned here with its corrected
//! output — and each fix is structural (see realize.rs): the mechanisms
//! that produced these bugs (string-search clitic placement, the
//! clause-global cluster, the droppable clitic side channel, the
//! imperative short-circuit, the hand-maintained order flag, discourse
//! string assembly) no longer exist.

use interslavic::{Gender, Number, Person};
use interslavic_phrase::discourse::{Connective, DiscourseSentence, narrate};
use interslavic_phrase::*;

fn s(tree: &Clause) -> String {
    realize(tree, RealizeOpts::sentence()).unwrap()
}

#[test]
fn clitics_stay_in_their_domain() {
    // Identical verb in relative and main clause: each sę attaches to
    // its own verb (the old subsequence search doubled the relative's).
    let tree = clause(
        np("mųž").relative(RelClause::subject_gap(vp("myti sę"))),
        vp("myti sę"),
    );
    assert_eq!(s(&tree), "Mųž, ktory myje sę, myje sę.");
}

#[test]
fn coordinated_vp_clitics_attach_to_their_own_verb() {
    // The old clause-global cluster migrated this to the FIRST verb.
    let tree = clause(np("krålj"), vp("kupiti").object(np("kniga")))
        .and_vp(vp("viděti").object(pron_clitic(
            Person::Third,
            Number::Singular,
            Gender::Masculine,
        )))
        .past();
    assert_eq!(s(&tree), "Krålj kupil knigų i viděl go.");

    // Two same-case clitics across conjuncts: the old Option<String>
    // overwrote the first — both now surface with their verbs.
    let tree = clause(
        pron(Person::First, Number::Singular, Gender::Masculine),
        vp("viděti").object(pron_clitic(
            Person::Second,
            Number::Singular,
            Gender::Masculine,
        )),
    )
    .and_vp(vp("slyšati").object(pron_clitic(
        Person::Third,
        Number::Singular,
        Gender::Masculine,
    )));
    assert_eq!(s(&tree), "Ja viđų tę i slyšų go.");
}

#[test]
fn clitics_are_never_dropped() {
    // In a relative clause (the old Rendered.clitic channel lost it).
    let tree = copular(
        np("krålj").relative(RelClause::subject_gap(vp("viděti").object(pron_clitic(
            Person::Third,
            Number::Singular,
            Gender::Masculine,
        )))),
        Predicate::Adjectival("dobry".into()),
    );
    assert_eq!(s(&tree), "Krålj, ktory vidi go, jest dobry.");

    // In a coordination conjunct: the full form is FORCED (a clitic
    // cannot carry conjunct stress — steen: clitics are "always
    // unstressed"), so nothing dangles.
    let tree = clause(
        np("krålj"),
        vp("viděti").object(coordinate(
            Conj::I,
            vec![
                name("Anna", Gender::Feminine),
                pron_clitic(Person::Third, Number::Singular, Gender::Masculine),
            ],
        )),
    );
    assert_eq!(s(&tree), "Krålj vidi Annų i jego.");
}

#[test]
fn focused_clitic_forces_full_form() {
    // Focus = stress, so the clitic contradiction resolves to the full
    // pronoun and li lands after a real word (the old code emitted
    // sentence-initial "Li").
    let tree = clause(
        np("krålj"),
        vp("viděti").object(pron_clitic(
            Person::Third,
            Number::Singular,
            Gender::Masculine,
        )),
    )
    .force(Force::LiQuestion)
    .focus(SlotRef::Object);
    assert_eq!(s(&tree), "Jego li vidi krålj?");
}

#[test]
fn second_position_respects_constituents() {
    // The old word-counting split "toj dobry krålj".
    let tree = clause(
        np("krålj").det("toj").adj("dobry"),
        vp("viděti").object(pron_clitic(
            Person::Third,
            Number::Singular,
            Gender::Masculine,
        )),
    );
    assert_eq!(
        realize(
            &tree,
            RealizeOpts::sentence().clitics(CliticStyle::SecondPosition)
        )
        .unwrap(),
        "Toj dobry krålj go vidi."
    );
    // li stays first in the cluster in a li-question.
    assert_eq!(
        realize(
            &tree.force(Force::LiQuestion),
            RealizeOpts::sentence().clitics(CliticStyle::SecondPosition)
        )
        .unwrap(),
        "Vidi li go toj dobry krålj?"
    );
}

#[test]
fn reflexive_coordination_is_rule_uniform() {
    // Each conjunct is its own clitic domain (Franks & King): postverbal
    // everywhere by default; in 2P mode the first domain is the clause,
    // later domains are verb-initial conjuncts.
    let tree = clause(
        pron(Person::First, Number::Singular, Gender::Masculine),
        vp("myti sę"),
    )
    .and_vp(vp("myti sę"));
    assert_eq!(s(&tree), "Ja myjų sę i myjų sę.");
    assert_eq!(
        realize(
            &tree,
            RealizeOpts::sentence().clitics(CliticStyle::SecondPosition)
        )
        .unwrap(),
        "Ja sę myjų i myjų sę."
    );
    // The reflexive li-question keeps the cited li-first cluster order.
    let tree = clause(
        pron(Person::Third, Number::Singular, Gender::Feminine),
        vp("myti sę"),
    )
    .force(Force::LiQuestion);
    assert_eq!(s(&tree), "Myje li sę ona?");
}

#[test]
fn incoherent_clauses_are_decided_not_silent() {
    // The old imperative flag short-circuited voice and mood.
    let tree = clause(np("kniga"), vp("kupiti"))
        .passive()
        .force(Force::Imperative(Addressee::You));
    assert!(matches!(
        realize(&tree, RealizeOpts::sentence()),
        Err(PhraseError::Unsupported(_))
    ));
    let tree = clause(np("otėc"), vp("kupiti").object(np("kniga")))
        .conditional()
        .force(Force::Imperative(Addressee::You));
    assert!(matches!(
        realize(&tree, RealizeOpts::sentence()),
        Err(PhraseError::IncoherentClause(_))
    ));
}

#[test]
fn relatives_share_the_vp_machinery() {
    // Valence applies inside relatives (the old copy skipped it)…
    let tree = copular(
        np("kot").relative(RelClause::subject_gap(vp("spati").object(np("kniga")))),
        Predicate::Adjectival("dobry".into()),
    );
    assert!(matches!(
        realize(&tree, RealizeOpts::sentence()),
        Err(PhraseError::ObjectOfIntransitive { .. })
    ));
    // …and adverbs render (the old copy dropped them).
    let tree = copular(
        np("kot").relative(RelClause::subject_gap(vp("spati").adv("dobro"))),
        Predicate::Adjectival("dobry".into()),
    );
    assert_eq!(s(&tree), "Kot, ktory dobro spi, jest dobry.");
}

#[test]
fn order_markedness_is_derived_and_warnings_fire_once() {
    // focus(Object) leaves S-V-O unchanged → no warning (the old
    // hand-maintained boolean had dead terms and warned anyway).
    let realized = realize_checked(
        &clause(np("avto"), vp("viděti").object(np("nebo"))).focus(SlotRef::Object),
        RealizeOpts::sentence(),
    )
    .unwrap();
    assert_eq!(realized.text, "Avto vidi nebo.");
    assert!(realized.warnings.is_empty());

    // A genuine inversion of a syncretic pair warns exactly once.
    let realized = realize_checked(
        &clause(np("avto"), vp("viděti").object(np("nebo"))).topic(SlotRef::Object),
        RealizeOpts::sentence(),
    )
    .unwrap();
    assert_eq!(realized.text, "Nebo avto vidi.");
    assert_eq!(realized.warnings, vec![PhraseWarning::AmbiguousOrder]);

    // Warnings from inside relatives are emitted once, not tripled by
    // syncretism probes (the probe is pure now).
    let realized = realize_checked(
        &clause(
            np("žena"),
            vp("viděti")
                .object(np("kniga").relative(RelClause::object_gap(np("otėc"), vp("kupiti")))),
        ),
        RealizeOpts::sentence(),
    )
    .unwrap();
    assert_eq!(
        realized
            .warnings
            .iter()
            .filter(|w| matches!(w, PhraseWarning::PerfectivePresent { .. }))
            .count(),
        1
    );
}

#[test]
fn narrate_honors_caller_options() {
    // strict_guessed applies to connective-bearing sentences too (the
    // old path reset the options).
    let story = vec![
        DiscourseSentence::new(clause(np("krålj").entity("k"), vp("spati"))),
        DiscourseSentence::new(clause(np("glorbina").entity("g"), vp("spati")))
            .connective(Connective::Potom),
    ];
    assert!(matches!(
        narrate(story, RealizeOpts::sentence().strict()),
        Err(PhraseError::GuessedHead { .. })
    ));

    // Clitic style is uniform across connective and connective-free
    // sentences.
    let story = vec![
        DiscourseSentence::new(clause(
            np("krålj"),
            vp("viděti").object(pron_clitic(
                Person::Third,
                Number::Singular,
                Gender::Masculine,
            )),
        )),
        DiscourseSentence::new(clause(
            np("žena"),
            vp("viděti").object(pron_clitic(
                Person::Third,
                Number::Singular,
                Gender::Masculine,
            )),
        ))
        .connective(Connective::Potom),
    ];
    assert_eq!(
        narrate(
            story,
            RealizeOpts::sentence().clitics(CliticStyle::SecondPosition)
        )
        .unwrap(),
        "Krålj go vidi. Potom žena go vidi."
    );
}
