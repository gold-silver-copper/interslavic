//! Print every golden sentence, one per line — the input for
//! `cargo xtask phrase-check`, which runs them through slovowiki's
//! independent agreement checker.

use interslavic::{Case, Gender, Number, Person};
use interslavic_phrase::*;

fn main() {
    let trees: Vec<Clause> = vec![
        clause(np("otėc"), vp("kupiti").object(np("kniga"))).past(),
        clause(
            pron(Person::First, Number::Singular, Gender::Masculine),
            vp("myti sę"),
        ),
        clause(np("otėc"), vp("kupiti").object(np("kniga")))
            .past()
            .force(Force::CiQuestion),
        clause(np("otėc"), vp("kupiti").object(np("kniga")))
            .past()
            .force(Force::LiQuestion),
        clause(
            np("kot"),
            vp("spati").pp(pp("pod", np("stol")).case(Case::Ins)),
        ),
        clause(
            np("kot"),
            vp("poběgti").pp(pp("pod", np("stol")).case(Case::Acc)),
        )
        .past(),
        clause(
            np("krålj").det("toj").adj("dobry"),
            vp("ukrasti").object(np("moneta").count(5).adj("zlåty")),
        )
        .past(),
        clause(np("otėc"), vp("kupiti").object(np("kniga")))
            .past()
            .negated(),
        clause(np("žena"), vp("kupiti").object(np("kniga"))).past(),
        clause(
            np("otėc"),
            vp("kupiti").object(np("kniga")).pp(pp(
                "za",
                pron(Person::Third, Number::Singular, Gender::Masculine),
            )
            .case(Case::Acc)),
        )
        .past(),
    ];
    for tree in &trees {
        println!("{}", realize(tree, RealizeOpts::sentence()).unwrap());
    }

    // 0.2.0 constructions. Proper-name sentences are excluded from this
    // corpus: names are knowingly absent from slovowiki's index
    // (report, don't force).
    let more: Vec<Clause> = vec![
        copular(
            np("avto").det("tutoj"),
            Predicate::Adjectival("dragy".into()),
        )
        .negated(),
        copular(np("komnata"), Predicate::Adjectival("veliky".into())),
        copular(np("komnata"), Predicate::Participial("osvětliti".into())),
        copular(np("žena"), Predicate::Adjectival("dobry".into())).past(),
        clause(np("kniga"), vp("kupiti").pp(pp("od", np("otėc"))))
            .passive()
            .past(),
        clause(
            pron(Person::Second, Number::Singular, Gender::Masculine),
            vp("kupiti").object(np("kniga")),
        )
        .force(Force::Imperative(Addressee::You)),
        clause(np("otėc"), vp("kupiti").object(np("kniga"))).conditional(),
        clause(
            pron(Person::First, Number::Singular, Gender::Masculine),
            vp("dękovati").object(pron(Person::Second, Number::Singular, Gender::Masculine)),
        ),
        // Dictionary-backed leaves for the Steen ditransitive frame.
        // Proper names in the source fixture stay out of this external
        // corpus because slovowiki intentionally lacks names.
        clause(
            np("otėc"),
            vp("dati")
                .recipient(np("žena"))
                .object(np("kniga").det("svoj")),
        )
        .past(),
        clause(np("krålj"), vp("vladati").object(np("zemja"))),
        clause(
            coordinate(Conj::I, vec![np("otėc").into(), np("žena").into()]),
            vp("kupiti").object(np("kniga")),
        )
        .past(),
        clause(np("krålj"), vp("kupiti"))
            .and_vp(vp("pročitati").object(np("kniga")))
            .unwrap()
            .past(),
        clause(
            np("krålj"),
            vp("viděti").object(pron_clitic(
                Person::Third,
                Number::Singular,
                Gender::Masculine,
            )),
        ),
        clause(np("otėc"), vp("kupiti").object(np("kniga")))
            .past()
            .topic(SlotRef::Object),
        clause(np("otėc"), vp("kupiti").object(np("kniga")))
            .past()
            .force(Force::LiQuestion)
            .focus(SlotRef::Object),
        copular(
            np("moneta").relative(RelClause::object_gap(np("krålj"), vp("ukrasti")).past()),
            Predicate::Adjectival("zlåty".into()),
        ),
        copular(
            np("krålj").relative(
                RelClause::subject_gap(vp("ukrasti").object(np("moneta").count(5).adj("zlåty")))
                    .past(),
            ),
            Predicate::Adjectival("dobry".into()),
        )
        .negated(),
    ];
    for tree in &more {
        println!("{}", realize(tree, RealizeOpts::sentence()).unwrap());
    }

    // The discourse narrative.
    use interslavic_phrase::discourse::*;
    let story = vec![
        DiscourseSentence::new(
            clause(
                np("krålj").entity("k").det("toj"),
                vp("kupiti").object(np("kniga").entity("b")),
            )
            .past(),
        ),
        DiscourseSentence::new(
            clause(
                np("krålj").entity("k"),
                vp("pročitati").object(np("kniga").entity("b")),
            )
            .past(),
        )
        .connective(Connective::Potom),
    ];
    println!("{}", narrate(story, RealizeOpts::sentence()).unwrap());

    // The Steen sample-corpus fixtures, so slovowiki's independent
    // agreement checker sees the source-backed sentences too and not only
    // the hand-built goldens. Kept in step with `tests/steen_samples.rs`
    // by `sample_corpus_sentences_are_all_checked_by_slovowiki` there.
    for (lead_in, clitics, sexpr) in SAMPLE_CORPUS {
        let tree = clause_from_str(sexpr).expect("sample fixture parses");
        let opts = RealizeOpts::sentence().clitics(*clitics);
        let realized =
            realize_with_lead_in(&tree, *lead_in, opts).expect("sample fixture realizes");
        println!("{}", realized.text);
    }
}

/// The `tests/steen_samples.rs` fixtures, in the same order, minus the
/// one noted below.
pub const SAMPLE_CORPUS: &[(Option<&str>, CliticStyle, &str)] = &[
    (
        None,
        CliticStyle::Postverbal,
        "(clause (name Strižik :m) (vp (v iměti) (object (np (det svoj) (n gnězdo))) \
         (pp (prep v) :case loc (np (n garaž)))) :tense past)",
    ),
    (
        None,
        CliticStyle::SecondPosition,
        "(clause (pron :1 :sg :m) (vp (v shvatiti) (object (pron :3 :sg :n :clitic))))",
    ),
    (
        Some("Potom"),
        CliticStyle::Postverbal,
        "(clause (pron :3 :sg :m) (vp (v odletěti) \
         (pp (prep za) :case ins (np (n čudovišče)))) :tense past)",
    ),
    (
        Some("Ale"),
        CliticStyle::Postverbal,
        "(clause (name strižik :m) (vp (v iměti) (object (np (n strah)))) :tense past :neg)",
    ),
    (
        None,
        CliticStyle::Postverbal,
        "(clause (pron :2 :sg :m) (vp (v dostavati) (object (np (det svoj) (n jeda)))) \
         :force wh :wh-adv kde :prodrop)",
    ),
    (
        None,
        CliticStyle::Postverbal,
        "(clause (pron :2 :sg :m) (vp (v žiti)) :force wh :wh-adv kde :prodrop)",
    ),
    (
        None,
        CliticStyle::Postverbal,
        "(clause (pron :2 :sg :m) (vp (v jesti)) :force wh :wh-adv kde :prodrop)",
    ),
    (
        None,
        CliticStyle::Postverbal,
        "(clause (coord i (np (det moj) (n žena)) (np :pl (det moj) (n dětę))) \
         (vp (v umirati) (pp (prep od) (np (n glåd)))))",
    ),
    // `volk_i_pes-017` (`Imajųt li vsi psi šije bez vlåsov?`) is
    // deliberately NOT fed to slovowiki. Its nominative plural `psi` is
    // the form Steen's own text uses and this repo's dictionary produces
    // from the lemma `pės`, but slovowiki's lexicon does not carry it, so
    // the checker reports it as an unknown token and fails the run. The
    // fixture still asserts that sentence byte-for-byte against Steen's
    // published etymological text, which is the stronger check; excluding
    // one sentence with a stated reason is preferable to relaxing the
    // zero-unknowns gate for every sentence.
    (
        None,
        CliticStyle::Postverbal,
        "(clause (np (n pųť)) (pred (adj dȯlgy)) :tense future :neg)",
    ),
    (
        None,
        CliticStyle::Postverbal,
        "(clause (pron :3 :pl :m) (vp (v razvoditi) (adv takože) (object (np :pl (n kura)))))",
    ),
    (
        None,
        CliticStyle::Postverbal,
        "(clause (pron :3 :pl :m) (vp (v kupovati) \
         (object (np :pl (adj gotovy) (n prědmet))) \
         (pp (prep od) (np :pl (n trgovec)))))",
    ),
    (
        Some("Tako"),
        CliticStyle::Postverbal,
        "(clause (np (adj maly) (n princ)) (vp (v odomašniti) (object (np (n lisica)))) \
         :tense past)",
    ),
    (
        Some("I"),
        CliticStyle::Postverbal,
        "(clause (pron :3 :sg :m) (vp (v vratiti sę) (pp (prep k) (np (n lisica)))) :tense past)",
    ),
];
