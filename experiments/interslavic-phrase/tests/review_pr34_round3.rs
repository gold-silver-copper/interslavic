//! Regression suite for the third PR #34 review round: reject duplicate
//! relative arguments and irrelevant options, traverse relative clauses
//! during discourse planning, compose `li` questions with topics, reject
//! tense combinations that the realizer cannot express, and preserve the
//! semantic plurality of plural-only entities.

use interslavic::{Gender, Number, Person};
use interslavic_phrase::discourse::{Connective, DiscourseSentence, narrate};
use interslavic_phrase::*;

fn sentence(tree: &Clause) -> Result<String, PhraseError> {
    realize(tree, RealizeOpts::sentence())
}

#[test]
fn typed_relative_gaps_reject_duplicate_arguments() {
    let object_gap_with_object = clause(
        np("kniga").relative(RelClause::object_gap(
            np("krålj"),
            vp("kupiti").object(np("moneta")),
        )),
        vp("ležati"),
    );
    assert!(matches!(
        sentence(&object_gap_with_object),
        Err(PhraseError::IncoherentClause(
            "object-gap relative clause also supplies an object"
        ))
    ));

    let subject_gap_with_subject = clause(
        np("mųž").relative(RelClause {
            gap: GapRole::Subject,
            subject: Some(np("žena").into()),
            vp: vp("spati"),
            tense: TenseSpec::Present,
            polarity: Polarity::Affirmative,
            relativizer: Relativizer::Ktory,
        }),
        vp("spati"),
    );
    assert!(matches!(
        sentence(&subject_gap_with_subject),
        Err(PhraseError::IncoherentClause(
            "subject-gap relative clause also supplies a subject"
        ))
    ));
}

#[test]
fn sexpr_relative_gaps_reject_duplicate_arguments() {
    let subject_gap =
        "(clause (np (n mųž) (rel :gap subj (np (n žena)) (vp (v spati)))) (vp (v spati)))";
    assert!(clause_from_str(subject_gap).is_err());

    let object_gap = "\
        (clause
          (np (n kniga)
              (rel :gap obj (np (n krålj))
                   (vp (v kupiti) (np (n moneta)))))
          (vp (v ležati)))";
    assert!(clause_from_str(object_gap).is_err());

    let valid_subject_gap = "(clause (np (n mųž) (rel :gap subj (vp (v spati)))) (vp (v spati)))";
    let valid_object_gap = "\
        (clause
          (np (n kniga)
              (rel :gap obj (np (n krålj)) (vp (v kupiti))))
          (vp (v ležati)))";
    assert!(clause_from_str(valid_subject_gap).is_ok());
    assert!(clause_from_str(valid_object_gap).is_ok());
}

#[test]
fn discourse_tracks_first_mentions_inside_relative_clauses() {
    let story = vec![
        DiscourseSentence::new(
            clause(
                np("krålj").entity("k"),
                vp("viděti").object(
                    np("kniga")
                        .relative(RelClause::object_gap(np("žena").entity("w"), vp("kupiti"))),
                ),
            )
            .past(),
        ),
        DiscourseSentence::new(clause(np("žena").entity("w"), vp("spati")).past())
            .connective(Connective::Potom),
    ];

    assert_eq!(
        narrate(story, RealizeOpts::sentence()).unwrap(),
        "Krålj viděl knigų, ktorų žena kupi. Potom ona spala."
    );
}

#[test]
fn repeated_relative_bearing_nps_remain_overt_and_track_mentions() {
    let story = vec![
        DiscourseSentence::new(clause(np("krålj").entity("k"), vp("spati"))),
        DiscourseSentence::new(clause(
            np("krålj")
                .entity("k")
                .relative(RelClause::object_gap(np("žena").entity("w"), vp("viděti"))),
            vp("spati"),
        ))
        .connective(Connective::Potom),
        DiscourseSentence::new(clause(np("žena").entity("w"), vp("spati")).past())
            .connective(Connective::Potom),
    ];

    assert_eq!(
        narrate(story, RealizeOpts::sentence()).unwrap(),
        "Krålj spi. Potom krålj, ktorogo žena vidi, spi. Potom ona spala."
    );
}

#[test]
fn aggregation_does_not_discard_later_subject_content() {
    let story = vec![
        DiscourseSentence::new(clause(np("krålj").entity("k"), vp("čitati"))),
        DiscourseSentence::new(clause(
            np("krålj")
                .entity("k")
                .relative(RelClause::object_gap(np("žena"), vp("viděti"))),
            vp("spati"),
        )),
    ];

    assert_eq!(
        narrate(story, RealizeOpts::sentence()).unwrap(),
        "Krålj čitaje. Krålj, ktorogo žena vidi, spi."
    );
}

#[test]
fn discourse_tracks_relatives_retained_on_copular_predicates() {
    let repeated_predicate = Predicate::Nominal(
        np("učitelj")
            .entity("teacher")
            .relative(RelClause::object_gap(np("žena").entity("w"), vp("viděti"))),
    );
    let story = vec![
        DiscourseSentence::new(copular(
            np("mųž"),
            Predicate::Nominal(np("učitelj").entity("teacher")),
        )),
        // A repeated nominal copular predicate is intentionally retained
        // rather than replaced by a pronoun; its relative is still overt.
        DiscourseSentence::new(copular(np("otėc"), repeated_predicate))
            .connective(Connective::Potom),
        DiscourseSentence::new(clause(np("žena").entity("w"), vp("spati")).past())
            .connective(Connective::Potom),
    ];

    assert_eq!(
        narrate(story, RealizeOpts::sentence()).unwrap(),
        "Mųž jest učitelj. Potom otėc jest učitelj, ktorogo žena vidi. Potom ona spala."
    );
}

#[test]
fn li_questions_compose_with_topics() {
    let question = clause(np("krålj"), vp("kupiti").object(np("kniga")))
        .past()
        .force(Force::LiQuestion);
    assert_eq!(sentence(&question).unwrap(), "Kupil li krålj knigų?");
    assert_eq!(
        sentence(&question.clone().topic(SlotRef::Object)).unwrap(),
        "Knigų kupil li krålj?"
    );

    let from_sexpr = clause_from_str(
        "(clause (np (n krålj)) (vp (v kupiti) (np (n kniga))) \
         :tense past :force li :topic obj)",
    )
    .unwrap();
    assert_eq!(sentence(&from_sexpr).unwrap(), "Knigų kupil li krålj?");
}

#[test]
fn li_precedes_second_position_clitics_when_a_topic_is_fronted() {
    let question = clause(np("krålj"), vp("myti sę"))
        .force(Force::LiQuestion)
        .topic(SlotRef::Subject);
    assert_eq!(
        realize(
            &question,
            RealizeOpts::sentence().clitics(CliticStyle::SecondPosition)
        )
        .unwrap(),
        "Krålj myje li sę?"
    );

    // A lexical constituent spelled "li" is not the structural particle.
    let collision = clause_from_str(
        "(clause (name li :m :indecl) (vp (v myti sę)) \
         :force li :topic subj)",
    )
    .unwrap();
    assert_eq!(
        realize(
            &collision,
            RealizeOpts::sentence().clitics(CliticStyle::SecondPosition)
        )
        .unwrap(),
        "Li myje li sę?"
    );
}

#[test]
fn sexpr_pred_case_is_restricted_to_nominal_predicates() {
    for invalid in [
        "(clause (np (n krålj)) (vp (v spati)) :pred-case ins)",
        "(clause (np (n krålj)) (vp (v spati)) :pred-case nom)",
        "(clause (np (n krålj)) (pred (adj dobry)) :pred-case ins)",
        "(clause (np (n krålj)) (pred (adj dobry)) :pred-case nom)",
    ] {
        assert!(clause_from_str(invalid).is_err(), "accepted `{invalid}`");
    }

    for valid in [
        "(clause (np (n otėc)) (pred (np (n krålj))) :pred-case ins)",
        "(clause (np (n otėc)) (pred (np (n krålj))) :pred-case nom)",
    ] {
        assert!(clause_from_str(valid).is_ok(), "rejected `{valid}`");
    }
}

#[test]
fn sexpr_rejects_other_context_only_options() {
    for invalid in [
        "(clause (np (n krålj)) (vp (v spati)) :force li :addressee 1pl)",
        "(clause (np (n krålj)) (pred (adj dobry)) :conj ili)",
        "(clause (np (n mųž) (rel :gap subj :case nom (vp (v spati)))) (vp (v spati)))",
        "(clause (np (n kniga) (rel :gap obj (prep v) (np (n žena)) \
         (vp (v čitati)))) (vp (v ležati)))",
    ] {
        assert!(clause_from_str(invalid).is_err(), "accepted `{invalid}`");
    }
}

#[test]
fn sexpr_rejects_duplicate_singular_fields() {
    for invalid in [
        "(clause (np (n krålj)) (pred (adj dobry)) (pred (adj stary)))",
        "(clause (np (det toj) (det tutoj) (n krålj)) (vp (v spati)))",
        "(clause (np (n krålj) (n mųž)) (vp (v spati)))",
        "(clause (np (num 1) (num 2) (n kniga)) (vp (v ležati)))",
        "(clause (np (n mųž) (rel :gap subj (vp (v spati))) \
         (rel :gap subj (vp (v ležati)))) (vp (v spati)))",
        "(clause (np (n mųž) (rel :gap subj (vp (v spati)) \
         (vp (v ležati)))) (vp (v spati)))",
        "(clause (np (n kniga) (rel :gap obj (np (n žena)) (np (n mųž)) \
         (vp (v čitati)))) (vp (v ležati)))",
        "(clause (np (n krålj)) (vp (v spati) (v ležati)))",
        "(clause (np (n krålj)) (vp (v spati) \
         (pp (prep pod) (prep nad) (np (n stol)) :case ins)))",
        "(clause (np (n krålj)) (vp (v spati) \
         (pp (prep pod) (np (n stol)) (np (n dom)) :case ins)))",
        "(clause (pron :1 :2 :sg :m) (vp (v spati)))",
        "(clause (name Anna :f :m) (vp (v spati)))",
        "(clause (np (n krålj)) (vp (v spati)) :force li :force či)",
        "(clause (np (n krålj) :entity k :entity other) (vp (v spati)))",
    ] {
        assert!(clause_from_str(invalid).is_err(), "accepted `{invalid}`");
    }

    // Repeated fields that are structurally plural remain valid.
    assert!(
        clause_from_str(
            "(clause (np (adj dobry) (adj stary) (n krålj)) \
             (vp (v spati) (adv dobro) (adv tiho) \
             (pp (prep pod) (np (n stol)) :case ins) \
             (pp (prep v) (np (n dom)) :case loc)) \
             (vp (v ležati)))"
        )
        .is_ok()
    );
}

#[test]
fn imperative_and_conditional_reject_independent_past_or_future_tense() {
    let imperative = clause(
        pron(Person::Second, Number::Singular, Gender::Masculine),
        vp("kupiti").object(np("kniga")),
    )
    .force(Force::Imperative(Addressee::You));
    assert_eq!(sentence(&imperative).unwrap(), "Kupi knigų!");
    for invalid in [imperative.clone().past(), imperative.future()] {
        assert!(matches!(
            sentence(&invalid),
            Err(PhraseError::IncoherentClause(
                "imperative with past or future tense"
            ))
        ));
    }

    let conditional = clause(np("krålj"), vp("kupiti").object(np("kniga"))).conditional();
    assert_eq!(sentence(&conditional).unwrap(), "Krålj by kupil knigų.");
    for invalid in [conditional.clone().past(), conditional.future()] {
        assert!(matches!(
            sentence(&invalid),
            Err(PhraseError::IncoherentClause(
                "conditional mood with an independently specified past or future tense"
            ))
        ));
    }

    for invalid in [
        "(clause (pron :2 :sg :m) (vp (v kupiti) (np (n kniga))) \
         :force imp :tense future)",
        "(clause (np (n krålj)) (vp (v kupiti) (np (n kniga))) \
         :mood cond :tense past)",
    ] {
        let tree = clause_from_str(invalid).unwrap();
        assert!(matches!(
            sentence(&tree),
            Err(PhraseError::IncoherentClause(_))
        ));
    }
}

#[test]
fn counted_plural_only_entities_stay_plural() {
    let story = vec![
        DiscourseSentence::new(
            clause(
                np("krålj").entity("k"),
                vp("kupiti").object(np("noviny").count(1).entity("n")),
            )
            .past(),
        ),
        DiscourseSentence::new(
            clause(
                np("krålj").entity("k"),
                vp("viděti").object(np("noviny").count(1).entity("n")),
            )
            .past(),
        )
        .connective(Connective::Potom),
    ];

    assert_eq!(
        narrate(story, RealizeOpts::sentence()).unwrap(),
        "Krålj kupil 1 noviny. Potom on viděl je."
    );
}
