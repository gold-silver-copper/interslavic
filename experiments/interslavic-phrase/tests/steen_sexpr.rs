//! Source conformance through the data-driven public API.
//!
//! Every fixture starts as a literal S-expression, not a Rust builder.
//! `source_text` preserves Steen's standard spelling; `expected` uses the
//! repository's etymological/flavored spelling and the facade's preferred
//! surface variants. The maintained source mirror states that these pages
//! are republished with the permission of their original author, Jan van
//! Steenbergen.

use interslavic_phrase::{
    RealizeOpts, clause_from_str, print_validated, realize_validated_checked, validate,
};

struct SteenCase {
    id: &'static str,
    source: &'static str,
    source_text: &'static str,
    normalization: &'static str,
    sexpr: &'static str,
    expected: &'static str,
}

const CASES: &[SteenCase] = &[
    SteenCase {
        id: "syntax-question-intonation",
        source: "https://steen.free.fr/interslavic/syntax.html#questions",
        source_text: "Otec kupil knigu?",
        normalization: "otec→otėc; knigu→knigų",
        sexpr: "(clause (np (n otėc)) (vp (v kupiti) (object (np (n kniga)))) \
                :tense past :force intonation)",
        expected: "Otėc kupil knigų?",
    },
    SteenCase {
        id: "syntax-question-ci",
        source: "https://steen.free.fr/interslavic/syntax.html#questions",
        source_text: "Či otec kupil knigu?",
        normalization: "otec→otėc; knigu→knigų",
        sexpr: "(clause (np (n otėc)) (vp (v kupiti) (object (np (n kniga)))) \
                :tense past :force či)",
        expected: "Či otėc kupil knigų?",
    },
    SteenCase {
        id: "syntax-question-li",
        source: "https://steen.free.fr/interslavic/syntax.html#questions",
        source_text: "Kupil li otec knigu?",
        normalization: "otec→otėc; knigu→knigų",
        sexpr: "(clause (np (n otėc)) (vp (v kupiti) (object (np (n kniga)))) \
                :tense past :force li)",
        expected: "Kupil li otėc knigų?",
    },
    SteenCase {
        id: "pronouns-reflexive-clitic",
        source: "https://steen.free.fr/interslavic/pronouns.html#personal_pronouns",
        source_text: "Ja myju se",
        normalization: "myju→myjų; se→sę; sentence punctuation added",
        sexpr: "(clause (pron :1 :sg :m) (vp (v myti sę)))",
        expected: "Ja myjų sę.",
    },
    SteenCase {
        id: "pronouns-reflexive-possessive",
        source: "https://steen.free.fr/interslavic/pronouns.html#possessive_pronouns",
        source_text: "Ja myju svoje avto",
        normalization: "myju→myjų; sentence punctuation added",
        sexpr: "(clause (pron :1 :sg :m) \
                (vp (v myti) (object (np (det svoj) (n avto)))))",
        expected: "Ja myjų svoje avto.",
    },
    SteenCase {
        id: "pronouns-ditransitive-reflexive-possessive",
        source: "https://steen.free.fr/interslavic/pronouns.html#possessive_pronouns",
        source_text: "Pjotr dal Ivanu svoju knigu",
        normalization: "svoju→svojų; knigu→knigų; sentence punctuation added",
        sexpr: "(clause (name Pjotr :m) \
                (vp (v dati) \
                    (recipient (name Ivan :m)) \
                    (object (np (det svoj) (n kniga)))) \
                :tense past)",
        expected: "Pjotr dal Ivanu svojų knigų.",
    },
    SteenCase {
        id: "pronouns-ditransitive-third-person-possessive",
        source: "https://steen.free.fr/interslavic/pronouns.html#possessive_pronouns",
        source_text: "Pjotr dal Ivanu jegovu knigu",
        normalization: "jegovu→jegovų; knigu→knigų; sentence punctuation added",
        sexpr: "(clause (name Pjotr :m) \
                (vp (v dati) \
                    (recipient (name Ivan :m)) \
                    (object (np (det jegov) (n kniga)))) \
                :tense past)",
        expected: "Pjotr dal Ivanu jegovų knigų.",
    },
    SteenCase {
        id: "prepositions-stable-location",
        source: "https://steen.free.fr/interslavic/prepositions.html#with_the_accusative_and_the_instrumental",
        source_text: "Kot spi pod stolom",
        normalization: "sentence punctuation added",
        sexpr: "(clause (np (n kot)) \
                (vp (v spati) (pp (prep pod) :case ins (np (n stol)))))",
        expected: "Kot spi pod stolom.",
    },
    SteenCase {
        id: "prepositions-motion-toward",
        source: "https://steen.free.fr/interslavic/prepositions.html#with_the_accusative_and_the_instrumental",
        source_text: "Kot poběgl pod stol",
        normalization: "sentence punctuation added",
        sexpr: "(clause (np (n kot)) \
                (vp (v poběgti) (pp (prep pod) :case acc (np (n stol)))) \
                :tense past)",
        expected: "Kot poběgl pod stol.",
    },
    SteenCase {
        id: "syntax-participial-passive",
        source: "https://steen.free.fr/interslavic/syntax.html#passive_voice",
        source_text: "Pica je dělana",
        normalization: "preferred full copula jest; sentence punctuation added",
        sexpr: "(clause (np (n pica)) (pred (part dělati)))",
        expected: "Pica jest dělana.",
    },
    SteenCase {
        id: "syntax-impersonal-active",
        source: "https://steen.free.fr/interslavic/syntax.html#passive_voice",
        source_text: "Dělajut picu",
        normalization: "dělajut→dělajųt; picu→picų; sentence punctuation added",
        sexpr: "(clause (pron :3 :pl :m) \
                (vp (v dělati) (object (np (n pica)))) :prodrop)",
        expected: "Dělajųt picų.",
    },
    SteenCase {
        id: "syntax-reflexive-passive",
        source: "https://steen.free.fr/interslavic/syntax.html#passive_voice",
        source_text: "Pica dělaje se",
        normalization: "se→sę; sentence punctuation added",
        sexpr: "(clause (np (n pica)) (vp (v dělati sę)))",
        expected: "Pica dělaje sę.",
    },
];

#[test]
fn literal_steen_sexpressions_realize_exactly_and_roundtrip() {
    assert_eq!(CASES.len(), 12);
    assert_eq!(
        CASES
            .iter()
            .map(|case| case.expected.split_whitespace().count())
            .sum::<usize>(),
        44
    );

    for case in CASES {
        let context = || {
            format!(
                "{}\nsource: {}\nsource text: {}\nnormalization: {}\nsexpr: {}",
                case.id, case.source, case.source_text, case.normalization, case.sexpr
            )
        };
        let raw = clause_from_str(case.sexpr)
            .unwrap_or_else(|error| panic!("{}\nparse failed: {error}", context()));
        let validated = validate(&raw)
            .unwrap_or_else(|error| panic!("{}\nvalidation failed: {error}", context()));
        let realized = realize_validated_checked(&validated, RealizeOpts::sentence())
            .unwrap_or_else(|error| panic!("{}\nrealization failed: {error}", context()));
        assert_eq!(realized.text, case.expected, "{}", context());

        let canonical = print_validated(&validated);
        let reparsed = clause_from_str(&canonical).unwrap_or_else(|error| {
            panic!("{}\ncanonical `{canonical}` failed: {error}", context())
        });
        assert_eq!(reparsed, raw, "{}\ncanonical: {canonical}", context());
        let revalidated = validate(&reparsed)
            .unwrap_or_else(|error| panic!("{}\nrevalidation failed: {error}", context()));
        let rerealized = realize_validated_checked(&revalidated, RealizeOpts::sentence())
            .unwrap_or_else(|error| panic!("{}\nrerealization failed: {error}", context()));
        assert_eq!(rerealized.text, case.expected, "{}", context());
    }
}
