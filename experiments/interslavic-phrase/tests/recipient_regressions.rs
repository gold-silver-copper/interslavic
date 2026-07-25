use interslavic::{Gender, Number, Person};
use interslavic_phrase::discourse::{DiscourseSentence, narrate};
use interslavic_phrase::*;

fn sentence(clause: &Clause) -> String {
    realize(clause, RealizeOpts::sentence()).unwrap()
}

#[test]
fn recipient_is_dative_and_precedes_the_theme() {
    let tree = clause(
        name("Pjotr", Gender::Masculine),
        vp("dati")
            .recipient(name("Ivan", Gender::Masculine))
            .object(np("kniga").det("svoj")),
    )
    .past();
    assert_eq!(sentence(&tree), "Pjotr dal Ivanu svojų knigų.");
}

#[test]
fn recipient_and_object_clitics_share_one_ordered_domain() {
    let tree = clause(
        name("Pjotr", Gender::Masculine),
        vp("dati")
            .recipient(pron_clitic(
                Person::Third,
                Number::Singular,
                Gender::Masculine,
            ))
            .object(pron_clitic(
                Person::Third,
                Number::Singular,
                Gender::Masculine,
            )),
    )
    .past();
    assert_eq!(sentence(&tree), "Pjotr dal mu go.");
    assert_eq!(
        realize(
            &tree,
            RealizeOpts::sentence().clitics(CliticStyle::SecondPosition)
        )
        .unwrap(),
        "Pjotr mu go dal."
    );

    let reflexive = clause(
        name("Pjotr", Gender::Masculine),
        vp("dati sę")
            .recipient(pron_clitic(
                Person::Third,
                Number::Singular,
                Gender::Masculine,
            ))
            .object(pron_clitic(
                Person::Third,
                Number::Singular,
                Gender::Masculine,
            )),
    )
    .past();
    assert_eq!(sentence(&reflexive), "Pjotr dal mu go sę.");
}

#[test]
fn marked_recipient_forces_a_full_form_and_has_its_own_slot() {
    let tree = clause(
        name("Pjotr", Gender::Masculine),
        vp("dati")
            .recipient(pron_clitic(
                Person::Third,
                Number::Singular,
                Gender::Masculine,
            ))
            .object(np("kniga")),
    )
    .past()
    .topic(SlotRef::Recipient);
    assert_eq!(sentence(&tree), "Jemu Pjotr dal knigų.");

    let printed = print(&tree).unwrap();
    assert!(printed.contains(":topic recipient"));
    assert_eq!(clause_from_str(&printed).unwrap(), tree);
}

#[test]
fn focused_recipient_hosts_li_and_remains_a_full_form() {
    let tree = clause(
        name("Pjotr", Gender::Masculine),
        vp("dati")
            .recipient(pron_clitic(
                Person::Third,
                Number::Singular,
                Gender::Masculine,
            ))
            .object(np("kniga")),
    )
    .past()
    .force(Force::LiQuestion)
    .focus(SlotRef::Recipient);
    assert_eq!(sentence(&tree), "Jemu li dal Pjotr knigų?");
}

#[test]
fn coordinated_vps_keep_recipient_clitics_in_their_own_domains() {
    let tree = clause(
        name("Pjotr", Gender::Masculine),
        vp("dati")
            .recipient(pron_clitic(
                Person::Third,
                Number::Singular,
                Gender::Masculine,
            ))
            .object(np("kniga")),
    )
    .and_vp(
        vp("dati")
            .recipient(pron_clitic(
                Person::Third,
                Number::Singular,
                Gender::Feminine,
            ))
            .object(np("kniga")),
    )
    .unwrap()
    .past();
    assert_eq!(sentence(&tree), "Pjotr dal mu knigų i dal jej knigų.");
}

#[test]
fn passive_promotes_the_theme_but_retains_the_recipient() {
    let tree = clause(
        np("kniga"),
        vp("dati").recipient(name("Ivan", Gender::Masculine)),
    )
    .past()
    .passive();
    assert_eq!(sentence(&tree), "Kniga byla dana Ivanu.");
}

#[test]
fn discourse_tracks_the_recipient_in_surface_order_without_losing_its_role() {
    let story = vec![
        DiscourseSentence::new(clause(np("žena").entity("r"), vp("spati"))),
        DiscourseSentence::new(
            clause(
                np("otėc"),
                vp("dati")
                    .recipient(np("žena").entity("r"))
                    .object(np("kniga")),
            )
            .past(),
        ),
    ];
    assert_eq!(
        narrate(story, RealizeOpts::sentence()).unwrap(),
        "Žena spi. Otėc dal jej knigų."
    );
}

#[test]
fn a_relative_clause_keeps_its_recipient_clitic_in_the_nested_vp_domain() {
    let tree = clause(
        np("žena"),
        vp("viděti").object(
            np("mųž").relative(RelClause::subject_gap(
                vp("dati")
                    .recipient(pron_clitic(
                        Person::Third,
                        Number::Singular,
                        Gender::Masculine,
                    ))
                    .object(np("kniga")),
            )),
        ),
    );
    assert_eq!(sentence(&tree), "Žena vidi mųža, ktory da mu knigų.");
}

#[test]
fn duplicate_or_malformed_recipient_forms_are_rejected_at_the_source() {
    let duplicate = "(clause (np (n otėc))
        (vp (v dati)
            (recipient (np (n žena)))
            (recipient (np (n student)))
            (object (np (n kniga))))
        :tense past)";
    let second_at = duplicate.rfind("(recipient").unwrap();
    let error = clause_from_str(duplicate).unwrap_err();
    assert_eq!(error.at, second_at);
    assert!(error.msg.contains("at most one recipient"));

    let missing = "(clause (np (n otėc)) (vp (v dati) (recipient)))";
    let recipient_at = missing.find("(recipient").unwrap();
    let error = clause_from_str(missing).unwrap_err();
    assert_eq!(error.at, recipient_at);
    assert!(error.msg.contains("needs a nominal"));
}

#[test]
fn recipient_validation_errors_keep_role_specific_paths_and_spans() {
    let missing_slot = clause(np("otėc"), vp("spati")).topic(SlotRef::Recipient);
    let error = realize(&missing_slot, RealizeOpts::sentence()).unwrap_err();
    assert!(matches!(
        error,
        PhraseError::Validation(ValidationErrors(errors))
            if errors == vec![ValidationError {
                path: AstPath("clause.topic".into()),
                kind: ValidationErrorKind::MissingInformationSlot(SlotRef::Recipient),
            }]
    ));

    let empty = "(clause (np (n otėc))
        (vp (v dati) (recipient (np (n \"\"))) (object (np (n kniga))))
        :tense past)";
    let head_at = empty.find("(n \"\")").unwrap();
    let error = clause_from_str(empty).unwrap_err();
    assert_eq!(error.at, head_at);
    assert!(error.msg.contains("recipient.nominal.head"));
    assert!(error.msg.contains("EmptyLeaf(\"noun\")"));
}
