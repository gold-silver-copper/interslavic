//! The second Steen slice: every unique complete sentence made
//! expressible by the grammar extensions. Each fixture keeps its source
//! page and section, original spelling, and exact normalization beside
//! the literal S-expression and byte-exact flavored output.

use interslavic::Case;
use interslavic_phrase::{
    PhraseError, RealizeOpts, ValidationErrorKind, clause, clause_from_str, np,
    participial_adjunct, pp, print_validated, realize, realize_validated_checked, validate, vp,
};

const ADJECTIVES: &str = "https://interslavic.fun/learn/grammar/adjectives/";
const PRONOUNS: &str = "https://interslavic.fun/learn/grammar/pronouns/";
const SYNTAX: &str = "https://interslavic.fun/learn/grammar/syntax/";
const VERBS: &str = "https://interslavic.fun/learn/grammar/verbs/";

struct SteenCase {
    id: &'static str,
    source: &'static str,
    section: &'static str,
    source_text: &'static str,
    normalization: &'static str,
    sexpr: &'static str,
    expected: &'static str,
}

macro_rules! steen_case {
    (
        $id:literal,
        $source:expr,
        $section:literal,
        $source_text:literal,
        $normalization:literal,
        $sexpr:literal,
        $expected:literal
    ) => {
        SteenCase {
            id: $id,
            source: $source,
            section: $section,
            source_text: $source_text,
            normalization: $normalization,
            sexpr: $sexpr,
            expected: $expected,
        }
    };
}

const CASES: &[SteenCase] = &[
    steen_case!(
        "adjective-long-petr",
        ADJECTIVES,
        "Predicative forms",
        "Petr jest ščestlivy",
        "Sentence punctuation added.",
        "(clause (name Petr :m) (pred (adj ščestlivy)))",
        "Petr jest ščestlivy."
    ),
    steen_case!(
        "adjective-long-dom",
        ADJECTIVES,
        "Predicative forms",
        "dom jest veliky",
        "Sentence-initial capitalization and punctuation added.",
        "(clause (np (n dom)) (pred (adj veliky)))",
        "Dom jest veliky."
    ),
    steen_case!(
        "adjective-short-petr",
        ADJECTIVES,
        "Predicative forms",
        "Petr jest ščestliv",
        "Sentence punctuation added.",
        "(clause (name Petr :m) (pred (short-adj ščestlivy)))",
        "Petr jest ščestliv."
    ),
    steen_case!(
        "adjective-short-dom",
        ADJECTIVES,
        "Predicative forms",
        "dom jest velik",
        "Sentence-initial capitalization and punctuation added.",
        "(clause (np (n dom)) (pred (short-adj veliky)))",
        "Dom jest velik."
    ),
    steen_case!(
        "pronoun-overt",
        PRONOUNS,
        "Personal pronouns and subject omission",
        "ja čitaju",
        "`ja` capitalized, `čitaju` respelled `čitajų`, and punctuation added.",
        "(clause (pron :1 :sg :m) (vp (v čitati)))",
        "Ja čitajų."
    ),
    steen_case!(
        "pronoun-prodrop",
        PRONOUNS,
        "Personal pronouns and subject omission",
        "čitaju",
        "`čitaju` respelled `čitajų`, sentence-initial capitalization, and punctuation added.",
        "(clause (pron :1 :sg :m) (vp (v čitati)) :prodrop)",
        "Čitajų."
    ),
    steen_case!(
        "reciprocal-reflexive",
        PRONOUNS,
        "Reciprocal constructions",
        "Oni bijut se",
        "`bijut se` respelled `bijųt sę`; punctuation added.",
        "(clause (pron :3 :pl :m) (vp (v biti sę)))",
        "Oni bijųt sę."
    ),
    steen_case!(
        "aspect-imperfective-past",
        VERBS,
        "Aspect",
        "ja čital jesm knigu",
        "The facade's auxiliary-first perfect order is used; `knigu` is respelled `knigų`; capitalization and punctuation added.",
        "(clause (pron :1 :sg :m) (vp (v čitati) (object (np (n kniga)))) :tense past)",
        "Ja jesm čital knigų."
    ),
    steen_case!(
        "aspect-perfective-past",
        VERBS,
        "Aspect",
        "ja pročital jesm knigu",
        "The facade's auxiliary-first perfect order is used; `knigu` is respelled `knigų`; capitalization and punctuation added.",
        "(clause (pron :1 :sg :m) (vp (v pročitati) (object (np (n kniga)))) :tense past)",
        "Ja jesm pročital knigų."
    ),
    steen_case!(
        "motion-nondirectional",
        VERBS,
        "Verbs of motion",
        "Igor jezdil po Moskvě",
        "Sentence punctuation added.",
        "(clause (name Igor :m) (vp (v jezditi) (pp (prep po) :case loc (name Moskva :f))) :tense past)",
        "Igor jezdil po Moskvě."
    ),
    steen_case!(
        "motion-directional",
        VERBS,
        "Verbs of motion",
        "Igor jehal do Moskvy",
        "Sentence punctuation added.",
        "(clause (name Igor :m) (vp (v jehati) (pp (prep do) (name Moskva :f))) :tense past)",
        "Igor jehal do Moskvy."
    ),
    steen_case!(
        "motion-perfective",
        VERBS,
        "Verbs of motion",
        "Igor pojehal do Moskvy",
        "Sentence punctuation added.",
        "(clause (name Igor :m) (vp (v pojehati) (pp (prep do) (name Moskva :f))) :tense past)",
        "Igor pojehal do Moskvy."
    ),
    steen_case!(
        "motion-habit",
        VERBS,
        "Verbs of motion",
        "Igor jezdil do Moskvy",
        "Sentence punctuation added.",
        "(clause (name Igor :m) (vp (v jezditi) (pp (prep do) (name Moskva :f))) :tense past)",
        "Igor jezdil do Moskvy."
    ),
    steen_case!(
        "past-imperfective",
        VERBS,
        "Past tense",
        "ja jesm dělal",
        "Sentence-initial capitalization and punctuation added.",
        "(clause (pron :1 :sg :m) (vp (v dělati)) :tense past)",
        "Ja jesm dělal."
    ),
    steen_case!(
        "past-perfective",
        VERBS,
        "Past tense",
        "ja jesm sdělal",
        "Sentence-initial capitalization and punctuation added.",
        "(clause (pron :1 :sg :m) (vp (v sdělati)) :tense past)",
        "Ja jesm sdělal."
    ),
    steen_case!(
        "pluperfect",
        VERBS,
        "Pluperfect",
        "ja běh dělal",
        "Sentence-initial capitalization and punctuation added.",
        "(clause (pron :1 :sg :m) (vp (v dělati)) :tense pluperfect)",
        "Ja běh dělal."
    ),
    steen_case!(
        "compound-pluperfect",
        VERBS,
        "Pluperfect",
        "ja byl jesm dělal",
        "Sentence-initial capitalization and punctuation added.",
        "(clause (pron :1 :sg :m) (vp (v dělati)) :tense compound-pluperfect)",
        "Ja byl jesm dělal."
    ),
    steen_case!(
        "conditional-perfect-m",
        VERBS,
        "Past conditional",
        "ja byl byh dělal",
        "Sentence-initial capitalization and punctuation added.",
        "(clause (pron :1 :sg :m) (vp (v dělati)) :mood cond-perfect)",
        "Ja byl byh dělal."
    ),
    steen_case!(
        "conditional-perfect-f",
        VERBS,
        "Past conditional",
        "ja byla byh dělala",
        "Sentence-initial capitalization and punctuation added.",
        "(clause (pron :1 :sg :f) (vp (v dělati)) :mood cond-perfect)",
        "Ja byla byh dělala."
    ),
    steen_case!(
        "optative-die",
        VERBS,
        "Optative",
        "Nehaj umre!",
        "No normalization.",
        "(clause (pron :3 :sg :m) (vp (v umrěti)) :force optative :prodrop)",
        "Nehaj umre!"
    ),
    steen_case!(
        "optative-live",
        VERBS,
        "Optative",
        "Nehaj žive dolgo!",
        "No normalization.",
        "(clause (pron :3 :sg :m) (vp (v žiti) (adv dolgo)) :force optative :prodrop)",
        "Nehaj žive dolgo!"
    ),
    steen_case!(
        "adverbial-participle",
        VERBS,
        "Present active adverbial participle",
        "Iduči do raboty, ona vsegda dymi cigaretoju",
        "`Iduči` and `cigaretoju` are respelled `Idųći` and `cigaretojų`; punctuation added.",
        "(clause (pron :3 :sg :f) \
            (vp (v dymiti) (adv vsegda) \
                (oblique :case ins (np (n cigareta)))) \
            (initial-participle (v idti) \
                (pp (prep do) (np (n rabota)))))",
        "Idųći do raboty, ona vsegda dymi cigaretojų."
    ),
    steen_case!(
        "passive-present-past-participle",
        VERBS,
        "Passive voice",
        "ja jesm neseny",
        "Sentence-initial capitalization and punctuation added.",
        "(clause (pron :1 :sg :m) (vp (v nesti)) :voice passive)",
        "Ja jesm neseny."
    ),
    steen_case!(
        "passive-present-present-participle",
        VERBS,
        "Passive voice",
        "ja jesm nesomy",
        "Sentence-initial capitalization and punctuation added.",
        "(clause (pron :1 :sg :m) (vp (v nesti)) :voice passive-present)",
        "Ja jesm nesomy."
    ),
    steen_case!(
        "passive-past-past-participle",
        VERBS,
        "Passive voice",
        "ja byl neseny",
        "The facade makes first-person perfect `byti` explicit as `jesm byl`; capitalization and punctuation added.",
        "(clause (pron :1 :sg :m) (vp (v nesti)) :voice passive :tense past)",
        "Ja jesm byl neseny."
    ),
    steen_case!(
        "passive-past-present-participle",
        VERBS,
        "Passive voice",
        "ja byl nesomy",
        "The facade makes first-person perfect `byti` explicit as `jesm byl`; capitalization and punctuation added.",
        "(clause (pron :1 :sg :m) (vp (v nesti)) :voice passive-present :tense past)",
        "Ja jesm byl nesomy."
    ),
    steen_case!(
        "passive-future",
        VERBS,
        "Passive voice",
        "ja budu neseny",
        "`budu` is respelled `bųdų`; capitalization and punctuation added.",
        "(clause (pron :1 :sg :m) (vp (v nesti)) :voice passive :tense future)",
        "Ja bųdų neseny."
    ),
    steen_case!(
        "passive-conditional",
        VERBS,
        "Passive voice",
        "ja byh neseny",
        "Sentence-initial capitalization and punctuation added.",
        "(clause (pron :1 :sg :m) (vp (v nesti)) :voice passive :mood cond)",
        "Ja byh neseny."
    ),
    steen_case!(
        "passive-conditional-perfect",
        VERBS,
        "Passive voice",
        "ja byl byh neseny",
        "Sentence-initial capitalization and punctuation added.",
        "(clause (pron :1 :sg :m) (vp (v nesti)) :voice passive :mood cond-perfect)",
        "Ja byl byh neseny."
    ),
    steen_case!(
        "wh-object",
        SYNTAX,
        "Constituent questions",
        "Koju knigu kupil otec?",
        "`Koju knigu` and `otec` are respelled `Kojų knigų` and `otėc`.",
        "(clause (np (n otėc)) \
            (vp (v kupiti) (object (np (det koj) (n kniga)))) \
            :tense past :force wh :wh obj)",
        "Kojų knigų kupil otėc?"
    ),
    steen_case!(
        "wh-adverb",
        SYNTAX,
        "Constituent questions",
        "Kde otec kupil tu knigu?",
        "`otec` and `tu knigu` are respelled `otėc` and `tų knigų`.",
        "(clause (np (n otėc)) \
            (vp (v kupiti) (object (np (det toj) (n kniga)))) \
            :tense past :force wh :wh-adv kde)",
        "Kde otėc kupil tų knigų?"
    ),
    steen_case!(
        "wh-subject",
        SYNTAX,
        "Constituent questions",
        "Koja žena ljubi togo muža?",
        "`muža` is respelled `mųža`.",
        "(clause (np (det koj) (n žena)) \
            (vp (v ljubiti) (object (np (det toj) (n mųž)))) \
            :force wh :wh subj)",
        "Koja žena ljubi togo mųža?"
    ),
    steen_case!(
        "wh-object-case",
        SYNTAX,
        "Constituent questions",
        "Koju ženu ljubi toj muž?",
        "`Koju ženu` and `muž` are respelled `Kojų ženų` and `mųž`.",
        "(clause (np (det toj) (n mųž)) \
            (vp (v ljubiti) (object (np (det koj) (n žena)))) \
            :force wh :wh obj)",
        "Kojų ženų ljubi toj mųž?"
    ),
    steen_case!(
        "wh-passive-subject",
        SYNTAX,
        "Constituent questions",
        "Koja žena jest ljubjena od togo muža?",
        "`muža` is respelled `mųža`.",
        "(clause (np (det koj) (n žena)) \
            (vp (v ljubiti) \
                (pp (prep od) (np (det toj) (n mųž)))) \
            :voice passive :force wh :wh subj)",
        "Koja žena jest ljubjena od togo mųža?"
    ),
    steen_case!(
        "syntax-present-passive",
        SYNTAX,
        "Passive voice",
        "Pica je dělajema",
        "The facade's preferred full copula `jest` replaces source `je`; punctuation added.",
        "(clause (np (n pica)) (vp (v dělati)) :voice passive-present)",
        "Pica jest dělajema."
    ),
];

fn context(case: &SteenCase) -> String {
    format!(
        "{}\nsource: {} — {}\nsource text: {}\nnormalization: {}\nsexpr: {}\nexpected output: {}",
        case.id,
        case.source,
        case.section,
        case.source_text,
        case.normalization,
        case.sexpr,
        case.expected
    )
}

#[test]
fn newly_supported_steen_sentences_realize_exactly_and_roundtrip() {
    assert_eq!(CASES.len(), 35);
    assert_eq!(
        CASES
            .iter()
            .map(|case| case.expected.split_whitespace().count())
            .sum::<usize>(),
        128
    );

    for case in CASES {
        let raw = clause_from_str(case.sexpr)
            .unwrap_or_else(|error| panic!("{}\nparse failed: {error}", context(case)));
        let validated = validate(&raw)
            .unwrap_or_else(|error| panic!("{}\nvalidation failed: {error}", context(case)));
        let realized = realize_validated_checked(&validated, RealizeOpts::sentence())
            .unwrap_or_else(|error| panic!("{}\nrealization failed: {error}", context(case)));
        assert_eq!(
            realized.text,
            case.expected,
            "{}\nactual output: {}",
            context(case),
            realized.text
        );

        let canonical = print_validated(&validated);
        let reparsed = clause_from_str(&canonical).unwrap_or_else(|error| {
            panic!("{}\ncanonical `{canonical}` failed: {error}", context(case))
        });
        assert_eq!(reparsed, raw, "{}\ncanonical: {canonical}", context(case));
        let revalidated = validate(&reparsed)
            .unwrap_or_else(|error| panic!("{}\nrevalidation failed: {error}", context(case)));
        let rerealized = realize_validated_checked(&revalidated, RealizeOpts::sentence())
            .unwrap_or_else(|error| panic!("{}\nrerealization failed: {error}", context(case)));
        assert_eq!(
            rerealized.text,
            case.expected,
            "{}\nactual output after roundtrip: {}",
            context(case),
            rerealized.text
        );
    }
}

#[test]
fn new_clause_forms_reject_incoherent_feature_combinations() {
    let missing_wh =
        clause_from_str("(clause (np (n žena)) (vp (v ljubiti)) :force wh)").unwrap_err();
    assert!(
        missing_wh
            .msg
            .contains("constituent question requires a fronted slot or adverb")
    );

    let marked_wh = clause_from_str(
        "(clause (np (n žena)) (vp (v ljubiti)) \
         :force wh :wh subj :topic subj)",
    )
    .unwrap_err();
    assert!(
        marked_wh
            .msg
            .contains("constituent questions cannot also set topic or focus")
    );

    let first_person_optative =
        clause_from_str("(clause (pron :1 :sg :m) (vp (v žiti)) :force optative)").unwrap_err();
    assert!(
        first_person_optative
            .msg
            .contains("optative requires a third-person subject")
    );

    let passive_with_object = clause_from_str(
        "(clause (np (n pica)) \
         (vp (v dělati) (object (np (n pica)))) \
         :voice passive-present)",
    )
    .unwrap_err();
    assert!(
        passive_with_object
            .msg
            .contains("a passive clause promotes the patient")
    );
}

#[test]
fn new_adjunct_edges_report_structured_paths() {
    let invalid = clause(np("žena"), vp("dymiti").oblique(Case::Ins, np("")))
        .initial_participle(participial_adjunct("idti").pp(pp("not-a-preposition", np("rabota"))));
    let errors = validate(&invalid).unwrap_err();
    assert!(errors.0.iter().any(|error| {
        error.path.0 == "clause.core.vp[0].oblique[0].nominal.head"
            && matches!(error.kind, ValidationErrorKind::EmptyLeaf("noun"))
    }));
    assert!(errors.0.iter().any(|error| {
        error.path.0 == "clause.initial_participle[0].pp[0].preposition"
            && matches!(
                &error.kind,
                ValidationErrorKind::UnknownPreposition(preposition)
                    if preposition == "not-a-preposition"
            )
    }));

    let unsupported =
        clause(np("žena"), vp("dymiti")).initial_participle(participial_adjunct("kupiti"));
    assert!(matches!(
        realize(&unsupported, RealizeOpts::sentence()),
        Err(PhraseError::Unsupported { path, .. })
            if path == "clause.initial_participle[0].verb"
    ));
}

#[test]
fn simple_past_rule_has_an_exact_serialized_contract() {
    let raw =
        clause_from_str("(clause (pron :1 :sg :m) (vp (v dělati)) :tense imperfect)").unwrap();
    let validated = validate(&raw).unwrap();
    assert_eq!(
        realize_validated_checked(&validated, RealizeOpts::sentence())
            .unwrap()
            .text,
        "Ja dělah."
    );
    assert_eq!(
        print_validated(&validated),
        "(clause (pron :1 :sg :m) (vp (v dělati)) :tense imperfect)"
    );
}
