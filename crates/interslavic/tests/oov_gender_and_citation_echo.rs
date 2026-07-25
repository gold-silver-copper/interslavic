//! Regression tests for gold-silver-copper/interslavic#5:
//! out-of-lexicon gender guessing and citation-form echo.

use interslavic::{Animacy, CASE_ORDER, Case, Gender, Number, cells};

/// An out-of-lexicon abstract noun in -osť must decline as a feminine i-stem
/// (like kosť), not be guessed masculine (točnosťa/točnosťem was the bug).
#[test]
fn oov_ost_nouns_are_feminine_i_stems() {
    // A nonsense -osť word guaranteed to be outside the dictionary.
    for w in ["zzzukavosť", "točnosť"] {
        let r#gen = interslavic::noun(w, Case::Gen, Number::Singular);
        assert!(
            r#gen.ends_with("osti"),
            "{w}: gen.sg must be the i-stem -osti, got {gen}"
        );
        let nom = interslavic::noun(w, Case::Nom, Number::Singular);
        assert_eq!(nom, w, "{w}: nom.sg must echo the citation form");
        let nom_pl = interslavic::noun(w, Case::Nom, Number::Plural);
        assert!(
            nom_pl.ends_with("osti"),
            "{w}: nom.pl must be the i-stem -osti, got {nom_pl}"
        );
    }
}

/// The 4-char lexical words stay with their dictionary genders: kosť is
/// feminine, gosť is MASCULINE — the length guard keeps the heuristic away.
#[test]
fn short_ost_words_keep_dictionary_gender() {
    assert!(
        interslavic::noun("kosť", Case::Gen, Number::Singular).contains("kosti"),
        "kosť stays feminine"
    );
    let gost_gen = interslavic::noun("gosť", Case::Gen, Number::Singular);
    assert!(
        !gost_gen.contains("gosti"),
        "gosť must NOT flip to a feminine i-stem: got {gost_gen}"
    );
}

/// Loan lemmas in soft consonant + -o must echo their citation form in the
/// nominative (adadžo → adadže was the bug); native soft neuters in -e are
/// untouched.
#[test]
fn soft_o_loans_echo_citation_form() {
    for w in ["adadžo", "bandžo"] {
        assert_eq!(interslavic::noun(w, Case::Nom, Number::Singular), w);
        // Accusative of an inanimate neuter echoes the nominative.
        assert_eq!(interslavic::noun(w, Case::Acc, Number::Singular), w);
    }
    // Native soft neuter unchanged by the fix.
    assert_eq!(
        interslavic::noun("morje", Case::Nom, Number::Singular),
        "morje"
    );
    assert_eq!(
        interslavic::noun("polje", Case::Nom, Number::Singular),
        "polje"
    );
}

#[test]
fn dictionary_citations_are_preserved_but_generated_forms_are_not() {
    for lemma in ["pancyŕ", "pancyrovoz"] {
        for case in [Case::Nom, Case::Acc] {
            assert_eq!(
                interslavic::noun(lemma, case, Number::Singular),
                lemma,
                "{lemma} {case:?}"
            );
            assert_eq!(
                interslavic::noun_with(
                    lemma,
                    case,
                    Number::Singular,
                    Gender::Masculine,
                    Animacy::Inanimate,
                ),
                lemma,
                "{lemma} {case:?} through noun_with"
            );
        }
    }

    assert_eq!(
        interslavic::noun("pancyŕ", Case::Gen, Number::Singular),
        "panciŕa"
    );
    assert_eq!(
        interslavic::noun("pancyrovoz", Case::Gen, Number::Singular),
        "pancirovoza"
    );
}

#[test]
fn indeclinable_dictionary_citation_remains_unchanged() {
    for number in [Number::Singular, Number::Plural] {
        for case in CASE_ORDER {
            assert_eq!(
                interslavic::noun("Jangcy", case, number),
                "Jangcy",
                "Jangcy {case:?} {number:?}"
            );
        }
    }
}

#[test]
fn m3_nominative_variants_stay_stable_and_include_the_citation() {
    let actual = interslavic::noun("dėnj", Case::Nom, Number::Singular);
    assert_eq!(actual, "den / denj");

    let actual_variants: Vec<_> = actual.split('/').flat_map(cells::variants).collect();
    let citation = interslavic::orthography::to_standard("dėnj");
    assert!(
        actual_variants
            .iter()
            .any(|variant| interslavic::orthography::to_standard(variant.trim()) == citation),
        "{actual:?} must include the citation variant for dėnj"
    );
}

#[test]
fn official_single_word_nouns_include_their_citation_in_nominative_singular() {
    let mut checked = 0usize;

    for line in include_str!("../data/dictionary_metadata.tsv")
        .lines()
        .skip(1)
    {
        let mut fields = line.split('\t');
        let _id = fields.next();
        let Some(citation) = fields.next() else {
            continue;
        };
        let _addition = fields.next();
        let Some(part_of_speech) = fields.next() else {
            continue;
        };

        let is_noun = ["m.", "f.", "n."]
            .iter()
            .any(|prefix| part_of_speech.starts_with(prefix));
        if !is_noun
            || part_of_speech.contains("pl.")
            || citation.contains(['#', '!'])
            || citation.chars().any(char::is_whitespace)
        {
            continue;
        }

        let gender = if part_of_speech.starts_with("f.") {
            Gender::Feminine
        } else if part_of_speech.starts_with("n.") {
            Gender::Neuter
        } else {
            Gender::Masculine
        };
        let animacy = if part_of_speech.contains("anim.") {
            Animacy::Animate
        } else {
            Animacy::Inanimate
        };
        let actual = interslavic::noun_with(citation, Case::Nom, Number::Singular, gender, animacy);
        let variants: Vec<_> = actual.split('/').flat_map(cells::variants).collect();
        let citation_variants = cells::variants(citation);
        assert!(
            citation_variants.iter().any(|citation_variant| {
                let citation_variant =
                    interslavic::orthography::to_standard(citation_variant.trim());
                variants.iter().any(|actual_variant| {
                    interslavic::orthography::to_standard(actual_variant.trim()) == citation_variant
                })
            }),
            "{citation:?} ({part_of_speech}) must occur in nominative singular {actual:?}"
        );
        checked += 1;
    }

    assert!(
        checked > 8_000,
        "the corpus invariant unexpectedly checked only {checked} nouns"
    );
}
