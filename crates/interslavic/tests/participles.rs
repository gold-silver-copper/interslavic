//! Round-trip checks for participle declension: the declined participle
//! wrappers must reproduce the gender forms the verb paradigm itself lists
//! inside its `pfpp`/`prap` cells (`"osvětljený (osvětljená, osvětljenó)"`),
//! across the `-ny`, `-ty`, and iotated `-jeny` participle shapes.

use interslavic::*;

/// Parse a paradigm participle cell `"M (F, N)"` into the three
/// nominative-singular forms, with the acute-marked long endings
/// flattened to the plain vowels the declension engine emits.
fn cell_genders(cell: &str) -> (String, String, String) {
    let plain = |s: &str| -> String {
        s.trim()
            .chars()
            .map(|c| match c {
                'ý' => 'y',
                'á' => 'a',
                'ó' => 'o',
                'í' => 'i',
                'é' => 'e',
                c => c,
            })
            .collect()
    };
    let (masc, rest) = cell.split_once(" (").expect("cell has gender forms");
    let (fem, neut) = rest
        .trim_end_matches(')')
        .split_once(", ")
        .expect("cell has fem and neut");
    (plain(masc), plain(fem), plain(neut))
}

#[test]
fn passive_participle_agrees_with_paradigm_across_shapes() {
    // -ny (opoznati, pisati, dělati, kovati), -ty (ubiti, vzęti), iotated
    // -jeny (osvětliti, prinesti, učiti), and the žegti alternation.
    for verb in [
        "opoznati",
        "pisati",
        "dělati",
        "kovati",
        "kupovati",
        "ubiti",
        "vzęti",
        "osvětliti",
        "prinesti",
        "učiti",
        "žegti",
    ] {
        let pfpp = verb_forms(verb).pfpp.expect("transitive verb has pfpp");
        let (masc, fem, neut) = cell_genders(&pfpp);
        let form = |g| passive_participle(verb, Case::Nom, Number::Singular, g, Animacy::Inanimate);
        assert_eq!(
            form(Gender::Masculine).as_deref(),
            Some(masc.as_str()),
            "{verb}"
        );
        assert_eq!(
            form(Gender::Feminine).as_deref(),
            Some(fem.as_str()),
            "{verb}"
        );
        assert_eq!(
            form(Gender::Neuter).as_deref(),
            Some(neut.as_str()),
            "{verb}"
        );
    }
}

#[test]
fn active_participle_agrees_with_paradigm() {
    for verb in ["pisati", "učiti", "dělati", "kovati", "idti"] {
        let prap = verb_forms(verb).prap.expect("imperfective verb has prap");
        let (masc, fem, neut) = cell_genders(&prap);
        let form = |g| active_participle(verb, Case::Nom, Number::Singular, g, Animacy::Inanimate);
        assert_eq!(
            form(Gender::Masculine).as_deref(),
            Some(masc.as_str()),
            "{verb}"
        );
        assert_eq!(
            form(Gender::Feminine).as_deref(),
            Some(fem.as_str()),
            "{verb}"
        );
        assert_eq!(
            form(Gender::Neuter).as_deref(),
            Some(neut.as_str()),
            "{verb}"
        );
    }
}

#[test]
fn present_passive_and_adverbial_active_helpers_expose_their_distinct_roles() {
    assert_eq!(
        present_passive_participle(
            "dělati",
            Case::Nom,
            Number::Singular,
            Gender::Feminine,
            Animacy::Inanimate,
        ),
        Some("dělajema".into()),
    );
    assert_eq!(
        present_passive_participle(
            "nesti",
            Case::Nom,
            Number::Singular,
            Gender::Masculine,
            Animacy::Animate,
        ),
        Some("nesomy".into()),
    );
    assert_eq!(
        present_passive_participle(
            "spati",
            Case::Nom,
            Number::Singular,
            Gender::Masculine,
            Animacy::Animate,
        ),
        None,
    );
    assert_eq!(
        present_passive_participle(
            "kupiti",
            Case::Nom,
            Number::Singular,
            Gender::Masculine,
            Animacy::Animate,
        ),
        None,
    );
    assert_eq!(active_adverbial_participle("idti"), Some("idųći".into()));
    assert_eq!(active_adverbial_participle("ubiti"), None);
}

#[test]
fn declined_participles_take_oblique_and_plural_endings() {
    // Oblique and plural agreement forms downstream templates need.
    let f = |c, n, g, a| passive_participle("osvětliti", c, n, g, a);
    assert_eq!(
        f(
            Case::Loc,
            Number::Singular,
            Gender::Feminine,
            Animacy::Inanimate
        )
        .as_deref(),
        Some("osvětljenoj")
    );
    assert_eq!(
        f(
            Case::Nom,
            Number::Plural,
            Gender::Feminine,
            Animacy::Inanimate
        )
        .as_deref(),
        Some("osvětljene")
    );
    assert_eq!(
        passive_participle(
            "ubiti",
            Case::Ins,
            Number::Singular,
            Gender::Masculine,
            Animacy::Animate
        )
        .as_deref(),
        Some("ubitym")
    );
}
