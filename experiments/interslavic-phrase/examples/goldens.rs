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
        clause(np("krålj"), vp("vladati").object(np("zemja"))),
        clause(
            coordinate(Conj::I, vec![np("otėc").into(), np("žena").into()]),
            vp("kupiti").object(np("kniga")),
        )
        .past(),
        clause(np("krålj"), vp("kupiti"))
            .and_vp(vp("pročitati").object(np("kniga")))
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
}
