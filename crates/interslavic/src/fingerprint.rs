//! Whole-dictionary paradigm fingerprint — the mechanical enforcement of
//! the byform/output compatibility contract.
//!
//! [`dump_string`] streams every generated paradigm cell for the entire
//! embedded dictionary (nouns, adjectives, verbs — raw cells, the clean
//! view is derived — plus the closed-class pronoun grid and the numeral
//! rows) as stable `lemma\tcell\tform` lines; [`fnv1a`] hashes the
//! stream. The unit test below pins the hash: any output change anywhere
//! in the dictionary fails it, and `cargo xtask dump-paradigms` /
//! `cargo xtask diff-fingerprint <old> <new>` enumerate exactly which
//! cells moved, so the changelog entry is mechanical. Update the pinned
//! constant only deliberately, in a commit whose message lists the delta.
//!
//! Infrastructure, not API: hidden from docs, exposed only so the
//! `dump_paradigms` example and the test share one corpus generator. The
//! caller supplies the dictionary TSV text (the crate does not read
//! files at run time).

use crate::{Animacy, CASE_ORDER, Case, Gender, Number, Person, PronounStyle};
use std::collections::BTreeSet;
use std::fmt::Write;

/// FNV-1a, 64-bit — inline so the workspace stays dependency-free.
pub fn fnv1a(text: &str) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in text.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

const GENDERS: [(&str, Gender); 3] = [
    ("m", Gender::Masculine),
    ("f", Gender::Feminine),
    ("n", Gender::Neuter),
];
const ANIMACIES: [(&str, Animacy); 2] = [("anim", Animacy::Animate), ("inan", Animacy::Inanimate)];
const NUMBERS: [(&str, Number); 2] = [("sg", Number::Singular), ("pl", Number::Plural)];
const STYLES: [(&str, PronounStyle); 3] = [
    ("full", PronounStyle::Full),
    ("clitic", PronounStyle::Clitic),
    ("nprep", PronounStyle::AfterPreposition),
];

fn case_key(case: Case) -> &'static str {
    match case {
        Case::Nom => "nom",
        Case::Acc => "acc",
        Case::Gen => "gen",
        Case::Loc => "loc",
        Case::Dat => "dat",
        Case::Ins => "ins",
    }
}

/// The full corpus as `lemma\tcell\tform` lines in a stable order:
/// single-word dictionary lemmas, deduplicated and sorted per kind, cells
/// in fixed iteration order. `tsv` is the embedded dictionary metadata
/// TSV (`id\tisv\taddition\tpartOfSpeech`).
pub fn dump_string(tsv: &str) -> String {
    let mut nouns = BTreeSet::new();
    let mut adjectives = BTreeSet::new();
    let mut verbs = BTreeSet::new();
    let mut numerals = BTreeSet::new();
    for line in tsv.lines().skip(1) {
        let mut columns = line.split('\t');
        let (_, Some(isv), _, Some(pos)) = (
            columns.next(),
            columns.next(),
            columns.next(),
            columns.next(),
        ) else {
            continue;
        };
        for lemma in isv.split(',').map(str::trim) {
            if lemma.is_empty() || lemma.contains(' ') {
                continue;
            }
            if pos.starts_with("m.") || pos.starts_with("f.") || pos.starts_with("n.") {
                nouns.insert(lemma);
            } else if pos.starts_with("adj.") {
                adjectives.insert(lemma);
            } else if pos.starts_with("v.") || pos.starts_with("#v.") {
                // A prefix match, NOT contains("v."): "adv." contains
                // "v." and adverbs must not be conjugated into the
                // corpus.
                verbs.insert(lemma);
            } else if pos.starts_with("num.") {
                numerals.insert(lemma);
            }
        }
    }

    let mut out = String::new();
    let mut cell = |lemma: &str, key: &str, form: &str| {
        let _ = writeln!(out, "{lemma}\t{key}\t{form}");
    };

    for lemma in &nouns {
        let paradigm = crate::noun_forms(lemma);
        for (n_key, number) in NUMBERS {
            for &case in &CASE_ORDER {
                cell(
                    lemma,
                    &format!("noun.{}.{}", case_key(case), n_key),
                    paradigm.get(case, number),
                );
            }
        }
    }
    for lemma in &adjectives {
        let paradigm = crate::adj_forms(lemma);
        for (g_key, gender) in GENDERS {
            for (a_key, animacy) in ANIMACIES {
                for (n_key, number) in NUMBERS {
                    for &case in &CASE_ORDER {
                        cell(
                            lemma,
                            &format!("adj.{}.{}.{}.{}", case_key(case), n_key, g_key, a_key),
                            paradigm.get(case, number, gender, animacy),
                        );
                    }
                }
            }
        }
    }
    for lemma in &verbs {
        let p = crate::verb_forms_raw(lemma);
        cell(lemma, "verb.infinitive", &p.infinitive);
        for (name, cells) in [
            ("present", &p.present),
            ("imperfect", &p.imperfect),
            ("perfect", &p.perfect),
            ("pluperfect", &p.pluperfect),
            ("future", &p.future),
            ("conditional", &p.conditional),
            ("imperative", &p.imperative),
        ] {
            for (index, form) in cells.iter().enumerate() {
                cell(lemma, &format!("verb.{name}.{index}"), form);
            }
        }
        cell(lemma, "verb.prap", p.prap.as_deref().unwrap_or("∅"));
        cell(lemma, "verb.prpp", p.prpp.as_deref().unwrap_or("∅"));
        cell(lemma, "verb.pfap", &p.pfap);
        cell(lemma, "verb.pfpp", p.pfpp.as_deref().unwrap_or("∅"));
        cell(lemma, "verb.gerund", &p.gerund);
    }
    for lemma in &numerals {
        for &case in &CASE_ORDER {
            for (n_key, number) in NUMBERS {
                for (g_key, gender) in GENDERS {
                    for (a_key, animacy) in ANIMACIES {
                        let form = crate::numeral(lemma, case, number, gender, animacy)
                            .unwrap_or_else(|| "∅".to_string());
                        cell(
                            lemma,
                            &format!("num.{}.{}.{}.{}", case_key(case), n_key, g_key, a_key),
                            &form,
                        );
                    }
                }
            }
        }
    }
    // Closed-class personal/reflexive pronoun grid, all three styles.
    for (p_key, person) in [
        ("1", Person::First),
        ("2", Person::Second),
        ("3", Person::Third),
    ] {
        for (n_key, number) in NUMBERS {
            for (g_key, gender) in GENDERS {
                for &case in &CASE_ORDER {
                    for (s_key, style) in STYLES {
                        let form = crate::personal_pronoun(person, number, gender, case, style)
                            .unwrap_or_else(|| "∅".to_string());
                        cell(
                            "⟨personal⟩",
                            &format!("pron.{p_key}.{n_key}.{g_key}.{}.{s_key}", case_key(case)),
                            &form,
                        );
                    }
                }
            }
        }
    }
    for &case in &CASE_ORDER {
        for (s_key, style) in STYLES {
            let form = crate::reflexive_pronoun(case, style).unwrap_or_else(|| "∅".to_string());
            cell(
                "⟨reflexive⟩",
                &format!("pron.refl.{}.{s_key}", case_key(case)),
                &form,
            );
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The pinned whole-dictionary fingerprint. If this test fails, some
    /// paradigm cell changed somewhere in the embedded dictionary:
    /// regenerate dumps with `cargo xtask dump-paradigms` on the old and
    /// new trees, run `cargo xtask diff-fingerprint <old> <new>` to
    /// enumerate the changed cells, put that enumeration in the
    /// changelog, and only then update this constant — in that commit,
    /// deliberately. Byform order and cell content are compatibility
    /// contracts (see CHANGELOG.md policy).
    const EXPECTED_FINGERPRINT: u64 = 0xed64_3269_cca3_fb22; // 533,286 cells, 0.14.1

    #[test]
    fn whole_dictionary_paradigm_fingerprint_is_stable() {
        let tsv = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/data/dictionary_metadata.tsv"
        ));
        let dump = dump_string(tsv);
        let hash = fnv1a(&dump);
        assert_eq!(
            hash,
            EXPECTED_FINGERPRINT,
            "paradigm output changed (fingerprint {hash:#018x}, {} cells). \
             Enumerate the delta with `cargo xtask diff-fingerprint` and \
             update EXPECTED_FINGERPRINT deliberately with a changelog entry.",
            dump.lines().count()
        );
    }
}
