//! The vocative, against the rules and examples on Steen's nouns page.
//!
//! Source: <https://steen.free.fr/interslavic/nouns.html> ("The vocative").
//! The page states the vocative "is not a real case", has no plural, and
//! never affects neuter nouns, adjectives or pronouns — which is why the
//! facade exposes it as a function rather than a seventh `Case`.

use interslavic::{Gender, vocative, vocative_with};

/// Every worked example the source itself gives, in source order.
#[test]
fn source_examples() {
    // "hard masculine stems take -e"
    assert_eq!(
        vocative_with("Ivan", Gender::Masculine).as_deref(),
        Some("Ivane")
    );
    assert_eq!(
        vocative_with("doktor", Gender::Masculine).as_deref(),
        Some("doktore")
    );
    assert_eq!(
        vocative_with("člověk", Gender::Masculine).as_deref(),
        Some("člověče")
    );

    // "soft masculine stems take -u"
    assert_eq!(
        vocative_with("prijateľ", Gender::Masculine).as_deref(),
        Some("prijatelju")
    );
    assert_eq!(
        vocative_with("muž", Gender::Masculine).as_deref(),
        Some("mužu")
    );
    assert_eq!(
        vocative_with("koń", Gender::Masculine).as_deref(),
        Some("konju")
    );

    // "masculine words on -ec take -če"
    assert_eq!(
        vocative_with("hlåpec", Gender::Masculine).as_deref(),
        Some("hlåpče")
    );

    // "masculine and feminine words on -a change their ending to -o"
    assert_eq!(
        vocative_with("sluga", Gender::Masculine).as_deref(),
        Some("slugo")
    );
    assert_eq!(
        vocative_with("žena", Gender::Feminine).as_deref(),
        Some("ženo")
    );
    assert_eq!(
        vocative_with("zemja", Gender::Feminine).as_deref(),
        Some("zemjo")
    );
}

/// "In the vocative, k, g and h become č, ž and š before e."
#[test]
fn first_palatalization_before_the_hard_ending() {
    assert_eq!(
        vocative_with("člověk", Gender::Masculine).as_deref(),
        Some("člověče")
    );
    assert_eq!(
        vocative_with("Bog", Gender::Masculine).as_deref(),
        Some("Bože")
    );
    assert_eq!(
        vocative_with("duh", Gender::Masculine).as_deref(),
        Some("duše")
    );
}

/// "Words on -ec have the vocative ending -če instead of the expected -cu:
/// otec > otče." The fleeting vowel goes with the ending.
#[test]
fn ec_stems_take_ce_not_the_expected_cu() {
    assert_eq!(
        vocative_with("otec", Gender::Masculine).as_deref(),
        Some("otče")
    );
    assert_ne!(
        vocative_with("otec", Gender::Masculine).as_deref(),
        Some("otcu")
    );
}

/// "the vocative is to be avoided" for feminine consonant stems and
/// neuters, and "the nominative can always be used instead". `None` is
/// that recommendation, not a gap in the tables.
#[test]
fn absent_where_the_source_says_to_use_the_nominative() {
    assert_eq!(vocative_with("noč", Gender::Feminine), None);
    assert_eq!(vocative_with("kosť", Gender::Feminine), None);
    assert_eq!(vocative_with("slovo", Gender::Neuter), None);
    assert_eq!(vocative_with("morje", Gender::Neuter), None);
}

/// The vocatives the Steen sample texts actually use as address forms.
#[test]
fn forms_used_in_the_sample_corpus() {
    // volk_i_pes: «Drågy prijatelju» and «Drågy brate»
    assert_eq!(
        vocative_with("prijatelj", Gender::Masculine).as_deref(),
        Some("prijatelju")
    );
    assert_eq!(
        vocative_with("brat", Gender::Masculine).as_deref(),
        Some("brate")
    );
    // selo: «ljubezny čitatelju»
    assert_eq!(
        vocative_with("čitatelj", Gender::Masculine).as_deref(),
        Some("čitatelju")
    );
    // jokes: «Pane doktore»
    assert_eq!(
        vocative_with("pan", Gender::Masculine).as_deref(),
        Some("pane")
    );
    // schleicher: «Slušaj, ovco»
    assert_eq!(
        vocative_with("ovca", Gender::Feminine).as_deref(),
        Some("ovco")
    );
    // jokes: «Mamo, začto...»
    assert_eq!(
        vocative_with("mama", Gender::Feminine).as_deref(),
        Some("mamo")
    );
}

/// Capitalization is the caller's; the vocative builder must not launder
/// it through a dictionary lookup the way case declension does.
#[test]
fn preserves_the_caller_s_capitalization() {
    assert_eq!(
        vocative_with("Bog", Gender::Masculine).as_deref(),
        Some("Bože")
    );
    assert_eq!(
        vocative_with("Ivan", Gender::Masculine).as_deref(),
        Some("Ivane")
    );
    assert_eq!(
        vocative_with("Petr", Gender::Masculine).as_deref(),
        Some("Petre")
    );
}

/// The gender-inferring entry point agrees with the explicit one.
#[test]
fn dictionary_gender_inference_matches_explicit_gender() {
    for lemma in ["brat", "muž", "žena", "zemja", "otec", "slovo"] {
        let gender = interslavic::noun_info(lemma).gender;
        assert_eq!(
            vocative(lemma),
            vocative_with(lemma, gender),
            "`{lemma}` disagrees between the inferring and explicit entry points"
        );
    }
}

#[test]
fn empty_and_whitespace_input_is_not_a_form() {
    assert_eq!(vocative_with("", Gender::Masculine), None);
    assert_eq!(vocative_with("   ", Gender::Masculine), None);
}
