use interslavic_core::{
    Animacy, CASE_ORDER, Case, Gender, Number, cells,
    noun::{decline_noun_explicit, decline_noun_simple},
    orthography,
};

#[test]
fn citation_spelling_is_not_rewritten_as_generated_morphology() {
    for lemma in ["pancyŕ", "pancyrovoz"] {
        assert_eq!(
            decline_noun_simple(
                lemma,
                Case::Nom,
                Number::Singular,
                Gender::Masculine,
                Animacy::Inanimate,
            ),
            lemma
        );
        assert_eq!(
            decline_noun_simple(
                lemma,
                Case::Acc,
                Number::Singular,
                Gender::Masculine,
                Animacy::Inanimate,
            ),
            lemma
        );
    }

    assert_eq!(
        decline_noun_simple(
            "pancyŕ",
            Case::Gen,
            Number::Singular,
            Gender::Masculine,
            Animacy::Inanimate,
        ),
        "panciŕa"
    );
    assert_eq!(
        decline_noun_simple(
            "pancyrovoz",
            Case::Gen,
            Number::Singular,
            Gender::Masculine,
            Animacy::Inanimate,
        ),
        "pancirovoza"
    );
}

#[test]
fn indeclinable_noun_keeps_its_citation_in_every_cell() {
    for number in [Number::Singular, Number::Plural] {
        for case in CASE_ORDER {
            assert_eq!(
                decline_noun_explicit(
                    "Jangcy",
                    case,
                    number,
                    Gender::Feminine,
                    Animacy::Inanimate,
                    false,
                    false,
                    true,
                    None,
                ),
                "Jangcy"
            );
        }
    }
}

#[test]
fn m3_nominative_variants_stay_stable_and_include_the_citation() {
    let actual = decline_noun_explicit(
        "dėnj",
        Case::Nom,
        Number::Singular,
        Gender::Masculine,
        Animacy::Inanimate,
        false,
        false,
        false,
        Some("dnja"),
    );
    assert_eq!(actual, "den / denj");

    let actual_variants: Vec<_> = actual.split('/').flat_map(cells::variants).collect();
    let citation = orthography::to_standard("dėnj");
    assert!(
        actual_variants
            .iter()
            .any(|variant| orthography::to_standard(variant.trim()) == citation),
        "{actual:?} must include the citation variant for dėnj"
    );
}
