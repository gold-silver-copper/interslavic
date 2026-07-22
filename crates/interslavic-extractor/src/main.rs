use std::collections::BTreeMap;
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Gender {
    Masculine,
    Feminine,
    Neuter,
    MasculineFeminine,
}

#[derive(Debug, Clone)]
struct NounEntry {
    lemma: String,
    addition: String,
    gender: Gender,
    animate: bool,
    plural_only: bool,
    singular_only: bool,
    indeclinable: bool,
}

#[derive(Debug, Clone)]
struct VerbEntry {
    lemma: String,
    addition: String,
    transitive: bool,
    imperfective: bool,
    perfective: bool,
    reflexive: bool,
    intransitive: bool,
    governs: Option<u8>,
}

#[derive(Debug)]
struct ParsedPartOfSpeech {
    is_noun: bool,
    is_verb: bool,
    gender: Option<Gender>,
    animate: bool,
    plural_only: bool,
    singular_only: bool,
    indeclinable: bool,
    transitive: bool,
    imperfective: bool,
    perfective: bool,
    reflexive: bool,
    intransitive: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = PathBuf::from("crates/interslavic/data/dictionary_metadata.tsv");
    let mut output_dir = PathBuf::from("crates/interslavic/generated");
    let mut check_only = false;

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input" => input = PathBuf::from(require_value(&mut args, "--input")?),
            "--output-dir" => output_dir = PathBuf::from(require_value(&mut args, "--output-dir")?),
            "--check-only" => check_only = true,
            "-h" | "--help" => {
                print_usage();
                return Ok(());
            }
            other => return Err(format!("unknown argument: {other}").into()),
        }
    }

    let generated = generate(&input)?;
    let noun_output_path = output_dir.join("noun_metadata_phf.rs");
    let verb_output_path = output_dir.join("verb_metadata_phf.rs");

    if check_only {
        check_generated_file(&noun_output_path, &generated.nouns)?;
        check_generated_file(&verb_output_path, &generated.verbs)?;
        println!(
            "check-registry: OK — generated dictionary metadata is deterministic and current."
        );
        return Ok(());
    }

    fs::create_dir_all(&output_dir)?;
    write_generated_file(&noun_output_path, &generated.nouns)?;
    write_generated_file(&verb_output_path, &generated.verbs)?;
    Ok(())
}

fn require_value(
    args: &mut impl Iterator<Item = String>,
    flag: &str,
) -> Result<String, Box<dyn Error>> {
    args.next()
        .ok_or_else(|| format!("expected a value after {flag}").into())
}

fn print_usage() {
    eprintln!("Usage: interslavic-extractor [--input PATH] [--output-dir DIR] [--check-only]");
}

fn check_generated_file(path: &Path, generated: &str) -> Result<(), Box<dyn Error>> {
    let committed = fs::read_to_string(path)?;
    if committed != generated {
        return Err(format!(
            "generated metadata is stale: run `cargo xtask refresh-data` and commit {}",
            path.display()
        )
        .into());
    }
    Ok(())
}

fn write_generated_file(path: &Path, generated: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;
    file.write_all(generated.as_bytes())?;
    println!("wrote {}", path.display());
    Ok(())
}

struct GeneratedFiles {
    nouns: String,
    verbs: String,
}

fn generate(input: &Path) -> Result<GeneratedFiles, Box<dyn Error>> {
    let text = fs::read_to_string(input)?;
    let mut noun_map: BTreeMap<String, Vec<NounEntry>> = BTreeMap::new();
    let mut verb_map: BTreeMap<String, Vec<VerbEntry>> = BTreeMap::new();

    for line in text.lines().skip(1) {
        let columns: Vec<&str> = line.split('\t').collect();
        if columns.len() < 4 {
            continue;
        }

        let isv = columns[1];
        let addition = columns[2].to_string();
        let metadata = parse_part_of_speech(columns[3]);
        if metadata.is_noun {
            let Some(gender) = metadata.gender.clone() else {
                continue;
            };
            for lemma in isv
                .split(',')
                .map(normalize_lemma)
                .filter(|lemma| !lemma.is_empty())
            {
                let entry = NounEntry {
                    lemma: lemma.clone(),
                    addition: addition.clone(),
                    gender: gender.clone(),
                    animate: metadata.animate,
                    plural_only: metadata.plural_only,
                    singular_only: metadata.singular_only,
                    indeclinable: metadata.indeclinable,
                };
                insert_noun_entry(&mut noun_map, lemma.clone(), entry.clone());
                let lower = lemma.to_lowercase();
                if lower != lemma {
                    insert_noun_entry(&mut noun_map, lower, entry);
                }
            }
        } else if metadata.is_verb {
            for lemma in isv
                .split(',')
                .map(normalize_lemma)
                .filter(|lemma| is_core_verb_lemma(lemma))
            {
                let entry = VerbEntry {
                    lemma: lemma.clone(),
                    addition: addition.clone(),
                    transitive: metadata.transitive,
                    imperfective: metadata.imperfective,
                    perfective: metadata.perfective,
                    reflexive: metadata.reflexive,
                    intransitive: metadata.intransitive,
                    governs: parse_governs(&addition),
                };
                insert_verb_entry(&mut verb_map, lemma.clone(), entry.clone());
                let lower = lemma.to_lowercase();
                if lower != lemma {
                    insert_verb_entry(&mut verb_map, lower, entry);
                }
            }
        }
    }

    Ok(GeneratedFiles {
        nouns: write_noun_phf(&noun_map),
        verbs: write_verb_phf(&verb_map),
    })
}

fn insert_noun_entry(map: &mut BTreeMap<String, Vec<NounEntry>>, key: String, entry: NounEntry) {
    let entries = map.entry(key).or_default();
    if !entries
        .iter()
        .any(|existing| same_noun_entry(existing, &entry))
    {
        entries.push(entry);
    }
}

fn same_noun_entry(a: &NounEntry, b: &NounEntry) -> bool {
    a.lemma == b.lemma
        && a.addition == b.addition
        && a.gender == b.gender
        && a.animate == b.animate
        && a.plural_only == b.plural_only
        && a.singular_only == b.singular_only
        && a.indeclinable == b.indeclinable
}

fn insert_verb_entry(map: &mut BTreeMap<String, Vec<VerbEntry>>, key: String, entry: VerbEntry) {
    let entries = map.entry(key).or_default();
    if !entries.iter().any(|existing| {
        existing.lemma == entry.lemma
            && existing.addition == entry.addition
            && existing.transitive == entry.transitive
            && existing.imperfective == entry.imperfective
            && existing.perfective == entry.perfective
            && existing.reflexive == entry.reflexive
            && existing.intransitive == entry.intransitive
            && existing.governs == entry.governs
    }) {
        entries.push(entry);
    }
}

fn write_verb_phf(map: &BTreeMap<String, Vec<VerbEntry>>) -> String {
    let mut out = String::new();
    out.push_str("use phf::phf_map;\n");
    out.push_str("use super::VerbDictionaryEntry;\n\n");
    out.push_str("pub(crate) static VERB_METADATA: phf::Map<&'static str, &'static [VerbDictionaryEntry]> = phf_map! {\n");
    for (key, entries) in map {
        out.push_str(&format!("    {:?} => &[\n", key));
        for entry in entries {
            out.push_str("        VerbDictionaryEntry { ");
            out.push_str(&format!("lemma: {:?}, ", entry.lemma));
            out.push_str(&format!("addition: {:?}, ", entry.addition));
            out.push_str(&format!("transitive: {}, ", entry.transitive));
            out.push_str(&format!("imperfective: {}, ", entry.imperfective));
            out.push_str(&format!("perfective: {}, ", entry.perfective));
            out.push_str(&format!("reflexive: {}, ", entry.reflexive));
            out.push_str(&format!("intransitive: {}, ", entry.intransitive));
            out.push_str(&format!("governs: {:?} ", entry.governs));
            out.push_str("},\n");
        }
        out.push_str("    ],\n");
    }
    out.push_str("};\n\n");
    out.push_str(
        "pub(crate) fn get_verbs(word: &str) -> Option<&'static [VerbDictionaryEntry]> {\n",
    );
    out.push_str("    VERB_METADATA.get(word).copied()\n");
    out.push_str("}\n");
    out
}

/// The dictionary's object-government annotation in the addition column:
/// `(+N)` with the community dictionary's case numbering (2=Gen, 3=Dat,
/// 4=Acc, 5=Ins, 7=Loc) — the same convention the preposition table was
/// curated from. May coexist with a present-stem hint
/// ("izběgti (izběži) (+2)"); the hint parser strips the marker on its
/// side, so extracting it here clobbers nothing.
fn parse_governs(addition: &str) -> Option<u8> {
    let bytes = addition.as_bytes();
    bytes
        .windows(2)
        .find(|pair| pair[0] == b'+' && pair[1].is_ascii_digit())
        .map(|pair| pair[1] - b'0')
}

fn is_core_verb_lemma(lemma: &str) -> bool {
    !lemma.is_empty()
        && !lemma.chars().any(char::is_whitespace)
        && !lemma.contains('(')
        && !lemma.contains(')')
        && !lemma.contains('/')
}

fn write_noun_phf(map: &BTreeMap<String, Vec<NounEntry>>) -> String {
    let mut out = String::new();
    out.push_str("use phf::phf_map;\n");
    out.push_str("use super::{DictionaryEntry, DictionaryGender};\n\n");
    out.push_str("pub(crate) static NOUN_METADATA: phf::Map<&'static str, &'static [DictionaryEntry]> = phf_map! {\n");
    for (key, entries) in map {
        out.push_str(&format!("    {:?} => &[\n", key));
        for entry in entries {
            out.push_str("        DictionaryEntry { ");
            out.push_str(&format!("lemma: {:?}, ", entry.lemma));
            out.push_str(&format!("addition: {:?}, ", entry.addition));
            out.push_str(&format!(
                "gender: DictionaryGender::{}, ",
                gender_name(&entry.gender)
            ));
            out.push_str(&format!("animate: {}, ", entry.animate));
            out.push_str(&format!("plural_only: {}, ", entry.plural_only));
            out.push_str(&format!("singular_only: {}, ", entry.singular_only));
            out.push_str(&format!("indeclinable: {} ", entry.indeclinable));
            out.push_str("},\n");
        }
        out.push_str("    ],\n");
    }
    out.push_str("};\n\n");
    out.push_str("pub(crate) fn get_nouns(word: &str) -> Option<&'static [DictionaryEntry]> {\n");
    out.push_str("    NOUN_METADATA.get(word).copied()\n");
    out.push_str("}\n");
    out
}

fn gender_name(gender: &Gender) -> &'static str {
    match gender {
        Gender::Masculine => "Masculine",
        Gender::Feminine => "Feminine",
        Gender::Neuter => "Neuter",
        Gender::MasculineFeminine => "MasculineFeminine",
    }
}

fn parse_part_of_speech(details: &str) -> ParsedPartOfSpeech {
    let normalized = details.replace("./", "/").replace(' ', "");
    let parts: Vec<&str> = normalized
        .split('.')
        .filter(|part| !part.is_empty())
        .collect();

    let has = |needle: &str| parts.contains(&needle);
    let is_noun = has("f") || has("n") || has("m") || has("m/f");

    let gender = if has("m/f") {
        Some(Gender::MasculineFeminine)
    } else if has("m") {
        Some(Gender::Masculine)
    } else if has("f") {
        Some(Gender::Feminine)
    } else if has("n") {
        Some(Gender::Neuter)
    } else {
        None
    };

    ParsedPartOfSpeech {
        is_noun,
        is_verb: has("v"),
        gender,
        animate: has("anim"),
        plural_only: has("pl"),
        singular_only: has("sg"),
        indeclinable: has("indecl"),
        transitive: has("tr"),
        // The conjugation booleans above keep their historical semantics
        // exactly (paradigm output must not change); the aspect/reflexivity
        // metadata below mirrors the reference parser (@interslavic/utils
        // src/partOfSpeech/partOfSpeech.ts): a biaspectual "ipf./pf." row is
        // imperfective AND perfective, reflexivity is its own flag.
        imperfective: normalized.contains("ipf"),
        perfective: parts
            .iter()
            .any(|part| part.split('/').any(|token| token == "pf")),
        reflexive: has("refl"),
        intransitive: has("intr"),
    }
}

fn normalize_lemma(lemma: &str) -> String {
    let cleaned = lemma.trim().replace('!', "");
    remove_bracketed_text(cleaned.trim_matches('"').trim(), '[', ']')
}

fn remove_bracketed_text(input: &str, open: char, close: char) -> String {
    let mut result = String::new();
    let mut depth = 0usize;
    for c in input.chars() {
        if c == open {
            depth += 1;
        } else if c == close && depth > 0 {
            depth -= 1;
        } else if depth == 0 {
            result.push(c);
        }
    }
    result.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_part_of_speech_extracts_noun_metadata() {
        let parsed = parse_part_of_speech("m.anim.pl");
        assert!(parsed.is_noun);
        assert!(!parsed.is_verb);
        assert_eq!(parsed.gender, Some(Gender::Masculine));
        assert!(parsed.animate);
        assert!(parsed.plural_only);
        assert!(!parsed.singular_only);
        assert!(!parsed.indeclinable);
    }

    #[test]
    fn parse_part_of_speech_extracts_verb_metadata() {
        let parsed = parse_part_of_speech("v.tr.ipf");
        assert!(parsed.is_verb);
        assert!(parsed.transitive);
        assert!(parsed.imperfective);
        assert!(!parsed.perfective);
        assert!(!parsed.reflexive);
        assert!(!parsed.intransitive);
    }

    #[test]
    fn parse_part_of_speech_extracts_aspect_and_reflexivity() {
        // Biaspectual: imperfective AND perfective (reference parser
        // semantics), with the conjugation bool staying imperfective.
        let parsed = parse_part_of_speech("v.tr. ipf./pf.");
        assert!(parsed.imperfective);
        assert!(parsed.perfective);
        // Plain perfective: not imperfective ("pf" must not match "ipf").
        let parsed = parse_part_of_speech("v.tr. pf.");
        assert!(!parsed.imperfective);
        assert!(parsed.perfective);
        // Reflexive is its own flag; carries no transitivity marker.
        let parsed = parse_part_of_speech("v.refl. ipf.");
        assert!(parsed.reflexive);
        assert!(!parsed.transitive);
        assert!(!parsed.intransitive);
        // Intransitive marker must not read as transitive ("tr" is
        // token-exact) and must set its own flag.
        let parsed = parse_part_of_speech("v.intr. ipf.");
        assert!(!parsed.transitive);
        assert!(parsed.intransitive);
    }

    #[test]
    fn normalize_lemma_removes_dictionary_annotations() {
        assert_eq!(normalize_lemma(" \"hektar [unit]\"! "), "hektar");
        assert_eq!(normalize_lemma("uško [dim. [nested]]"), "uško");
    }

    #[test]
    fn core_verb_filter_rejects_phrases_and_alternatives() {
        assert!(is_core_verb_lemma("pisati"));
        assert!(!is_core_verb_lemma("bazovati na"));
        assert!(!is_core_verb_lemma("pisati/pisati"));
        assert!(!is_core_verb_lemma("(piše)"));
    }
}
