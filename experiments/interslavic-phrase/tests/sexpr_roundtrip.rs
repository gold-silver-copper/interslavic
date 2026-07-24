//! The S-expression contract, exercised over the FULL node vocabulary.
//!
//! Coverage checklist against the format spec in `sexpr.rs` — every
//! entry below appears in at least one tree:
//!   clause keys: :tense past, :tense future, :neg, :prodrop,
//!                :mood cond, :voice passive, :force li|či|intonation|imp,
//!                :addressee 1pl|2pl, :conj, :topic, :focus, :pred-case ins
//!   cores:       (vp …)+ (coordinated), (pred (np …)|(adj …)|(part …))
//!   np:          :case, :entity, (det …), (num …), (adj …)+, (rel …)
//!   rel:         :gap subj|obj|pp (+ (prep …) :case), :tense, :neg,
//!                :relativizer iže
//!   pron:        person/number/gender flags, :clitic
//!   name:        gender flags, :indecl
//!   coord:       every conjunction (i, ili, a, ale)
//!   vp:          (v … sę), (adv …), object, (pp … :case …)
//!
//! Contract (see `print`): parse ∘ print = id for canonical trees;
//! print(parse(print(t))) == print(t) for all trees (print
//! canonicalizes — e.g. single-item coordinations print as their item).

use interslavic::{Case, Gender, Number, Person};
use interslavic_phrase::*;

fn trees() -> Vec<Clause> {
    vec![
        // 0.1.0 shapes.
        clause(np("otėc"), vp("kupiti").object(np("kniga"))).past(),
        clause(
            np("krålj").det("toj").adj("dobry"),
            vp("ukrasti").object(np("moneta").count(5).adj("zlåty")),
        )
        .past(),
        clause(
            pron(Person::First, Number::Singular, Gender::Masculine),
            vp("myti sę"),
        ),
        clause(
            np("kot"),
            vp("spati").pp(pp("pod", np("stol")).case(Case::Ins)),
        ),
        clause(np("otėc"), vp("kupiti").object(np("kniga")))
            .past()
            .negated()
            .force(Force::LiQuestion),
        clause(
            pron(Person::Third, Number::Plural, Gender::Feminine),
            vp("spati"),
        )
        .future()
        .prodrop(),
        clause(
            np("otėc"),
            vp("kupiti").object(np("kniga").case(Case::Gen)).pp(pp(
                "za",
                pron(Person::Third, Number::Singular, Gender::Neuter),
            )
            .case(Case::Acc)),
        )
        .force(Force::CiQuestion),
        // Copular cores: nominal, adjectival, participial; pred-case.
        copular(
            np("avto").det("tutoj"),
            Predicate::Adjectival("dragy".into()),
        )
        .negated(),
        copular(np("komnata"), Predicate::Participial("osvětliti".into())),
        Clause::with_core(
            pron(Person::Third, Number::Singular, Gender::Masculine),
            ClauseCore::Copular {
                predicate: Predicate::Nominal(np("krålj")),
                pred_case: PredCase::Instrumental,
            },
        )
        .past(),
        // Voice, moods, forces.
        clause(np("kniga"), vp("kupiti").pp(pp("od", np("otėc"))))
            .passive()
            .past(),
        clause(
            pron(Person::Second, Number::Singular, Gender::Masculine),
            vp("kupiti").object(np("kniga")),
        )
        .force(Force::Imperative(Addressee::You)),
        clause(
            pron(Person::First, Number::Plural, Gender::Masculine),
            vp("spati"),
        )
        .force(Force::Imperative(Addressee::We)),
        clause(
            pron(Person::Second, Number::Plural, Gender::Masculine),
            vp("spati"),
        )
        .force(Force::Imperative(Addressee::YouAll))
        .negated(),
        clause(np("otėc"), vp("kupiti").object(np("kniga"))).conditional(),
        clause(np("otėc"), vp("kupiti").object(np("kniga"))).force(Force::IntonationQuestion),
        // Relatives: all three gap roles, tense, negation, iže.
        copular(
            np("moneta").relative(RelClause::object_gap(np("krålj"), vp("ukrasti")).past()),
            Predicate::Adjectival("zlåty".into()),
        ),
        copular(
            np("krålj").relative(RelClause::subject_gap(vp("spati").adv("dobro"))),
            Predicate::Adjectival("dobry".into()),
        ),
        copular(
            np("dom").relative(RelClause {
                gap: GapRole::PpObject {
                    preposition: "v".into(),
                    case: Case::Loc,
                },
                subject: Some(np("krålj").into()),
                vp: vp("spati"),
                tense: TenseSpec::Present,
                polarity: Polarity::Negative,
                relativizer: Relativizer::Ktory,
            }),
            Predicate::Adjectival("veliky".into()),
        ),
        copular(
            np("moneta").relative(
                RelClause::object_gap(np("krålj"), vp("ukrasti"))
                    .past()
                    .relativizer(Relativizer::Iže),
            ),
            Predicate::Adjectival("zlåty".into()),
        ),
        // Coordination: every conjunction; VP coordination with :conj.
        clause(
            coordinate(Conj::I, vec![np("otėc").into(), np("žena").into()]),
            vp("kupiti").object(np("kniga")),
        )
        .past(),
        clause(
            coordinate(
                Conj::Ili,
                vec![
                    name("Anna", Gender::Feminine),
                    pron(Person::Second, Number::Singular, Gender::Masculine),
                ],
            ),
            vp("spati"),
        ),
        clause(
            coordinate(Conj::A, vec![np("kot").into(), np("otėc").into()]),
            vp("spati"),
        ),
        clause(np("krålj"), vp("kupiti"))
            .and_vp(vp("pročitati").object(np("kniga")))
            .conj(Conj::Ale)
            .past(),
        // Pronoun clitics, names, information structure, entities.
        clause(
            np("krålj"),
            vp("viděti").object(pron_clitic(
                Person::Third,
                Number::Singular,
                Gender::Masculine,
            )),
        ),
        clause(name("Anna", Gender::Feminine), vp("spati").adv("dobro")),
        clause(
            Nominal::Name {
                text: "Interslavic".into(),
                gender: Gender::Neuter,
                indeclinable: true,
            },
            vp("spati"),
        ),
        clause(
            np("otėc").entity("o"),
            vp("kupiti").object(np("kniga").entity("b")),
        )
        .past()
        .topic(SlotRef::Object)
        .focus(SlotRef::Subject),
    ]
}

#[test]
fn print_parse_roundtrip_is_identity_on_canonical_trees() {
    for tree in trees() {
        let printed = print(&tree);
        let reparsed = clause_from_str(&printed)
            .unwrap_or_else(|e| panic!("reparse of `{printed}` failed: {e}"));
        assert_eq!(
            reparsed, tree,
            "round-trip changed the tree for `{printed}`"
        );
        assert_eq!(
            print(&reparsed),
            printed,
            "print not canonical for `{printed}`"
        );
    }
}

#[test]
fn print_canonicalizes_single_item_coordination() {
    // A single-item coordination is surface-indistinguishable from its
    // item; print normalizes it, and re-parsing yields the canonical
    // tree. print(parse(print(t))) == print(t) still holds.
    let tree = clause(
        Nominal::Coord(Coordination {
            conjunction: Conj::I,
            items: vec![np("otėc").into()],
        }),
        vp("spati"),
    );
    let printed = print(&tree);
    assert_eq!(printed, "(clause (np (n otėc)) (vp (v spati)))");
    let reparsed = clause_from_str(&printed).unwrap();
    assert_eq!(reparsed, clause(np("otėc"), vp("spati")));
    assert_eq!(print(&reparsed), printed);
}

#[test]
fn reader_and_typed_builders_agree() {
    let from_text = clause_from_str(
        "(clause (np (det toj) (adj dobry) (n krålj)) \
                 (vp (v ukrasti) (np (num 5) (adj zlåty) (n moneta))) \
                 :tense past)",
    )
    .unwrap();
    let from_code = clause(
        np("krålj").det("toj").adj("dobry"),
        vp("ukrasti").object(np("moneta").count(5).adj("zlåty")),
    )
    .past();
    assert_eq!(from_text, from_code);
}

#[test]
fn garbage_input_never_panics() {
    for garbage in [
        "",
        "(",
        ")",
        "()",
        "(clause)",
        "(clause (np) (vp))",
        "(clause (np (n dom)) (vp (v spati)) :tense yesterday)",
        "(clause (np (n dom)) (vp (v spati)) :frobnicate)",
        "((()))",
        "(clause (np (n dom)) (vp (v spati))) extra",
        "(pron :sg)",
        "(clause (pron :9 :sg) (vp (v spati)))",
        ":::",
        "(clause (np (n dom)) (vp (v spati)) :tense",
        "(clause (np (n dom) (num x)) (vp (v spati)))",
        "(clause (np (n dom)) (vp (v spati)) (pred (adj dobry)))",
        "(coord i (np (n a)))",
        "(clause (np (n dom) (rel (vp (v spati)))) (vp (v spati)))",
        "(name Anna)",
        "žčš đćę (ų)",
    ] {
        let _ = clause_from_str(garbage);
    }
}

#[test]
fn spans_point_into_the_input() {
    let input = "(clause (np (n dom)) (vp (v spati)) :tense yesterday)";
    let err = clause_from_str(input).unwrap_err();
    assert!(err.at < input.len());
    assert!(err.msg.contains("yesterday"));
}
