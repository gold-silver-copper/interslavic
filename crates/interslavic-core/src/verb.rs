//! Verb conjugation, paradigms, and verb-specific stem helpers.

use crate::{Conjugation, Gender, Number, Person, Tense, VerbParadigm, utils};

pub fn get_present_tense_stem(infinitive: &str) -> (String, Conjugation) {
    let ps = sonic_present_tense_stem(infinitive, "");
    let conjugation = if ps.ends_with('i') {
        Conjugation::Second
    } else {
        Conjugation::First
    };
    (ps, conjugation)
}

pub fn get_infinitive_stem(word: &str) -> String {
    sonic_infinitive_stem(&sonic_prefix(word), word)
        .unwrap_or_else(|| utils::string_without_last_n(word, 2))
}

pub fn conjugate_verb(
    word: &str,
    person: Person,
    number: Number,
    gender: Gender,
    tense: Tense,
) -> String {
    conjugate_verb_with_options(word, "", person, number, gender, tense, true, true)
}

pub fn conjugate_verb_with_present_hint(
    word: &str,
    present_hint: &str,
    person: Person,
    number: Number,
    gender: Gender,
    tense: Tense,
) -> String {
    conjugate_verb_with_options(
        word,
        present_hint,
        person,
        number,
        gender,
        tense,
        true,
        true,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn conjugate_verb_with_options(
    word: &str,
    present_hint: &str,
    person: Person,
    number: Number,
    gender: Gender,
    tense: Tense,
    transitive: bool,
    imperfective: bool,
) -> String {
    let paradigm = verb_paradigm_with_options(word, present_hint, transitive, imperfective);
    verb_slot(&paradigm, person, number, gender, tense)
}

/// Pick one finite form out of an already-built paradigm — the shared slot
/// selector for the total and checked conjugation entry points.
fn verb_slot(
    paradigm: &VerbParadigm,
    person: Person,
    number: Number,
    gender: Gender,
    tense: Tense,
) -> String {
    match tense {
        Tense::Present => finite_slot(&paradigm.present, person, number),
        Tense::Imperfect => finite_slot(&paradigm.imperfect, person, number),
        Tense::Future => finite_slot(&paradigm.future, person, number),
        Tense::Perfect => gendered_compound_slot(&paradigm.perfect, person, number, gender),
        Tense::Pluperfect => gendered_compound_slot(&paradigm.pluperfect, person, number, gender),
        Tense::Conditional => gendered_compound_slot(&paradigm.conditional, person, number, gender),
    }
}

/// One finite verb form, or `None` when an infinitive stem cannot be
/// derived from `word` (roughly, it does not look like an infinitive) —
/// the checked counterpart of [`conjugate_verb_with_options`].
#[allow(clippy::too_many_arguments)]
pub fn conjugate_verb_checked(
    word: &str,
    present_hint: &str,
    person: Person,
    number: Number,
    gender: Gender,
    tense: Tense,
    transitive: bool,
    imperfective: bool,
) -> Option<String> {
    let paradigm = verb_paradigm_checked(word, present_hint, transitive, imperfective)?;
    Some(verb_slot(&paradigm, person, number, gender, tense))
}

/// The full verb paradigm. Total: it never panics. A word whose infinitive
/// stem cannot be derived falls back to the last-two-characters-stripped
/// stem (the same best-effort [`get_infinitive_stem`] uses); call
/// [`verb_paradigm_checked`] to detect that case instead of
/// silently degrading.
pub fn verb_paradigm_with_options(
    word: &str,
    present_hint: &str,
    transitive: bool,
    imperfective: bool,
) -> VerbParadigm {
    verb_paradigm_inner(word, present_hint, transitive, imperfective, false)
        .unwrap_or_else(|| empty_phrase_verb_paradigm(word.trim()))
}

/// The full verb paradigm, or `None` when an infinitive stem cannot be
/// derived from `word` — the checked counterpart of
/// [`verb_paradigm_with_options`], which falls back to a
/// best-effort stem instead of signalling. Use it to reject clearly
/// non-verb input without a `catch_unwind` guard.
///
/// The check is mechanical, not lexical: a stem is derivable from anything
/// shaped like an infinitive (ending in `-ti`/`-t`/`-ť`), so a `-t`/`-ť`
/// noun such as `kosť` still yields `Some`. It returns `None` for input
/// that cannot be a verb stem at all (empty, `voda`, `xyz`).
pub fn verb_paradigm_checked(
    word: &str,
    present_hint: &str,
    transitive: bool,
    imperfective: bool,
) -> Option<VerbParadigm> {
    verb_paradigm_inner(word, present_hint, transitive, imperfective, true)
}

/// The stem derivation shared by every emitter: finite-form normalization
/// (`je`/`jest`/`sųt` → `byti`), prefix split, hint cleaning, and the
/// hint-aware infinitive stem — the context [`verb_paradigm_with_options`],
/// [`l_participle_with_hint`], and [`perfect_parts_with_hint`] all derive
/// from, so their stems can never disagree. `None` only in strict mode
/// when no infinitive stem is derivable; otherwise the stem degrades to
/// the same best-effort form [`get_infinitive_stem`] uses.
fn stem_context(word: &str, present_hint: &str, strict: bool) -> Option<(String, String, String)> {
    let normalized = match word {
        "sųt" | "je" | "jest" => "byti",
        other => other,
    };
    let prefix = sonic_prefix(normalized);
    let clean_hint = clean_present_hint(present_hint);
    let infinitive_stem = match sonic_infinitive_stem_with_hint(&prefix, normalized, &clean_hint) {
        Some(stem) => stem,
        None if strict => return None,
        None => utils::string_without_last_n(normalized, 2),
    };
    Some((prefix, clean_hint, infinitive_stem))
}

fn verb_paradigm_inner(
    word: &str,
    present_hint: &str,
    transitive: bool,
    imperfective: bool,
    strict: bool,
) -> Option<VerbParadigm> {
    let word = word.trim();
    if word.split_whitespace().nth(1).is_some() {
        return Some(empty_phrase_verb_paradigm(word));
    }

    let (pref, clean_hint, infinitive_stem) = stem_context(word, present_hint, strict)?;
    let present_stem = sonic_present_tense_stem_from_parts(&pref, &clean_hint, &infinitive_stem);
    let secondary_stem = secondary_present_tense_stem(&present_stem);
    let lpa = sonic_l_participle(&pref, &infinitive_stem);
    let infinitive = build_sonic_infinitive(&pref, &infinitive_stem);
    let present = build_present_vec(&pref, &present_stem, &secondary_stem);
    let imperfect = build_imperfect_vec(&pref, &infinitive_stem);
    let perfect = build_perfect_vec(&lpa);
    let pluperfect = build_pluperfect_vec(&lpa);
    let future = build_future_vec(&infinitive, &present_stem);
    let conditional = build_conditional_vec(&lpa);
    let imperative = build_imperative_vec(&pref, &secondary_stem);
    let prap = imperfective.then(|| build_prap(&pref, &present_stem));
    let prpp =
        (imperfective && transitive).then(|| build_prpp(&pref, &present_stem, &secondary_stem));
    let pfap = build_pfap(&lpa);
    let computed_pfpp = build_pfpp(&pref, &infinitive_stem, &secondary_stem);
    let pfpp = transitive.then(|| computed_pfpp.clone());
    let gerund = build_gerund(&computed_pfpp);

    Some(VerbParadigm {
        infinitive,
        present,
        imperfect,
        perfect,
        pluperfect,
        future,
        conditional,
        imperative,
        prap,
        prpp,
        pfap,
        pfpp,
        gerund,
    })
}

fn empty_phrase_verb_paradigm(word: &str) -> VerbParadigm {
    VerbParadigm {
        infinitive: word.to_string(),
        present: vec![word.to_string(); 6],
        imperfect: vec![word.to_string(); 6],
        perfect: vec![word.to_string(); 8],
        pluperfect: vec![word.to_string(); 8],
        future: vec![word.to_string(); 6],
        conditional: vec![word.to_string(); 8],
        imperative: vec![word.to_string(); 3],
        prap: None,
        prpp: None,
        pfap: word.to_string(),
        pfpp: None,
        gerund: word.to_string(),
    }
}

fn finite_slot(forms: &[String], person: Person, number: Number) -> String {
    let slot = match (person, number) {
        (Person::First, Number::Singular) => 0,
        (Person::Second, Number::Singular) => 1,
        (Person::Third, Number::Singular) => 2,
        (Person::First, Number::Plural) => 3,
        (Person::Second, Number::Plural) => 4,
        (Person::Third, Number::Plural) => 5,
    };
    forms.get(slot).cloned().unwrap_or_default()
}

fn gendered_compound_slot(
    forms: &[String],
    person: Person,
    number: Number,
    gender: Gender,
) -> String {
    let slot = match (person, number, gender) {
        (Person::First, Number::Singular, _) => 0,
        (Person::Second, Number::Singular, _) => 1,
        (Person::Third, Number::Singular, Gender::Masculine) => 2,
        (Person::Third, Number::Singular, Gender::Feminine) => 3,
        (Person::Third, Number::Singular, Gender::Neuter) => 4,
        (Person::First, Number::Plural, _) => 5,
        (Person::Second, Number::Plural, _) => 6,
        (Person::Third, Number::Plural, _) => 7,
    };
    forms.get(slot).cloned().unwrap_or_default()
}

fn sonic_prefix(infinitive: &str) -> String {
    const PREFIXES: &[&str] = &[
        "prědpo", "razpro", "råzpro", "izpo", "odpo", "nad", "pod", "pre", "pri", "pro", "prě",
        "raz", "råz", "voz", "vȯz", "do", "iz", "na", "ne", "ob", "od", "po", "sȯ", "vo", "vy",
        "vȯ", "za", "o", "s", "u", "v",
    ];
    const NON_REGULAR: &[&str] = &[
        "vedeti", "věděti", "jesti", "jěsti", "dati", "dųti", "byti", "žegti",
    ];
    for verb in NON_REGULAR {
        if let Some(maybe_prefix) = infinitive
            .strip_suffix(verb)
            .filter(|maybe_prefix| PREFIXES.contains(maybe_prefix))
        {
            return maybe_prefix.to_string();
        }
    }
    if let Some(index) = infinitive.find('-') {
        return infinitive[..=index].to_string();
    }
    if infinitive.starts_with("ne ") {
        return "ne ".to_string();
    }
    String::new()
}

fn sonic_infinitive_stem(prefix: &str, infinitive: &str) -> Option<String> {
    sonic_infinitive_stem_with_hint(prefix, infinitive, "")
}

fn sonic_infinitive_stem_with_hint(
    prefix: &str,
    infinitive: &str,
    present_hint: &str,
) -> Option<String> {
    let trunc = infinitive.strip_prefix(prefix).unwrap_or(infinitive);
    if trunc.is_empty() {
        return None;
    }
    let mut result = None;
    for ending in ["ti", "tì", "t", "ť"] {
        if let Some(stem) = trunc.strip_suffix(ending) {
            result = Some(stem.to_string());
            break;
        }
    }
    let mut result = result?;
    if result.ends_with('s') {
        if result == "jes" {
            result = "jed".to_string();
        } else if !present_hint.is_empty() {
            result = utils::string_without_last_n(present_hint, 1);
        }
    }
    Some(result)
}

fn clean_present_hint(raw: &str) -> String {
    let mut pts = raw.trim().replace(") (", ")(").replace(['(', ')'], "");
    if let Some((head, _)) = pts.split_once([';', ',', '/']) {
        pts = head.to_string();
    }
    if let Some(index) = pts.find('+').filter(|index| {
        pts[*index + 1..]
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_digit())
    }) {
        pts.replace_range(index..=index + 1, "");
    }
    pts.trim().to_string()
}

fn sonic_present_tense_stem(infinitive: &str, present_hint: &str) -> String {
    let pref = sonic_prefix(infinitive);
    let stem = sonic_infinitive_stem(&pref, infinitive)
        .unwrap_or_else(|| utils::string_without_last_n(infinitive, 2));
    sonic_present_tense_stem_from_parts(&pref, &clean_present_hint(present_hint), &stem)
}

fn sonic_present_tense_stem_from_parts(
    prefix: &str,
    present_hint: &str,
    infinitive_stem: &str,
) -> String {
    let mut result = if present_hint.is_empty() {
        derive_sonic_present_tense_stem(infinitive_stem)
    } else {
        let mut pts = present_hint.to_string();
        if pts.ends_with(" se") || pts.ends_with(" sę") {
            pts.truncate(pts.len() - 3);
        } else if pts.starts_with("se ") || pts.starts_with("sę ") {
            pts = pts[3..].to_string();
        }
        if !prefix.is_empty() {
            // Strip the prefix off the present hint, char-boundary safe.
            // `strip_prefix` replaces the old `contains` + byte-index slice
            // that could panic when the prefix contained a multibyte char.
            if let Some(rest) = pts.strip_prefix(prefix) {
                pts = rest.to_string();
            } else {
                let mut short = prefix.to_string();
                short.pop(); // prefix minus its last char, on a boundary
                if let Some(rest) = pts.strip_prefix(short.as_str()) {
                    pts = rest.to_string();
                }
            }
        }
        if pts.ends_with(['m', 'e', 'u', 'ų', '-']) {
            pts[..pts.len() - pts.chars().last().unwrap().len_utf8()].to_string()
        } else {
            pts
        }
    };
    result = process_present_tense_stem_exceptions(prefix, infinitive_stem, &result);
    result
}

fn derive_sonic_present_tense_stem(infinitive_stem: &str) -> String {
    let result = infinitive_stem;
    if result == "vzę" {
        "vȯzm".to_string()
    } else if result == "umě" {
        "uměĵ".to_string()
    } else if result == "hova" {
        "hovaĵ".to_string()
    } else if result.ends_with("eva") || result.ends_with("ova") {
        format!("{}uj", &result[..result.len() - 3])
    } else if result.chars().count() >= 3 && (result.ends_with("nu") || result.ends_with("nų")) {
        result[..result.len() - result.chars().last().unwrap().len_utf8()].to_string()
    } else if is_short_oueě_stem(result) {
        format!("{result}j")
    } else if ["bję", "dję", "sję", "zję"]
        .iter()
        .any(|suffix| result.ends_with(suffix))
    {
        format!("{}ȯjm", &result[..result.len() - "ję".len()])
    } else if result.ends_with("ję") {
        format!("{}m", &result[..result.len() - "ę".len()])
    } else if result.ends_with('ę') {
        format!("{}n", &result[..result.len() - 'ę'.len_utf8()])
    } else if result.ends_with('ų') {
        result[..result.len() - 'ų'.len_utf8()].to_string()
    } else if result.ends_with('y') {
        format!("{result}j")
    } else if result.ends_with(['a', 'e', 'ě']) {
        format!("{result}ĵ")
    } else {
        result.to_string()
    }
}

fn is_short_oueě_stem(stem: &str) -> bool {
    let chars: Vec<char> = stem.chars().collect();
    (2..=3).contains(&chars.len())
        && chars
            .last()
            .is_some_and(|c| matches!(c, 'e' | 'ě' | 'o' | 'u'))
}

fn process_present_tense_stem_exceptions(
    prefix: &str,
    infinitive_stem: &str,
    present_stem: &str,
) -> String {
    let mut result = present_stem.to_string();
    if (infinitive_stem == "by" && prefix.is_empty())
        || ((result == "je" || result == "j") && infinitive_stem == "bi")
    {
        result = "jes".to_string();
    } else if result == "věděĵ" {
        result = "vě".to_string();
    } else if result == "jed" || (result == "j" && infinitive_stem == "jed") {
        result = "je".to_string();
    } else if result == "jěd" || (result == "j" && infinitive_stem == "jěd") {
        result = "jě".to_string();
    } else if result == "jad" || (result == "j" && infinitive_stem == "jad") {
        result = "ja".to_string();
    } else if result == "daĵ" {
        result = "da".to_string();
    } else if result == "žeg" || result == "žž" {
        result = "žg".to_string();
    } else if result.ends_with("maj") {
        result = format!("{}ĵ", &result[..result.len() - 1]);
    }
    if result == "jěhaĵ" || (result == "jě" && infinitive_stem == "jěha") {
        result = "jěd".to_string();
    }
    if result == "jehaĵ" || (result == "je" && infinitive_stem == "jeha") {
        result = "jed".to_string();
    }
    if result == "jahaĵ" || (result == "ja" && infinitive_stem == "jaha") {
        result = "jad".to_string();
    }
    result
}

fn secondary_present_tense_stem(present_stem: &str) -> String {
    if let Some(stem) = present_stem.strip_suffix('g') {
        format!("{stem}ž")
    } else if let Some(stem) = present_stem.strip_suffix('k') {
        format!("{stem}č")
    } else {
        present_stem.to_string()
    }
}

fn sonic_l_participle(prefix: &str, infinitive_stem: &str) -> String {
    if infinitive_stem == "vojd" || infinitive_stem == "vȯjd" {
        "všėl".to_string()
    } else if infinitive_stem == "id" || infinitive_stem == "jd" {
        format!("{prefix}šėl")
    } else if infinitive_stem.ends_with("id") || infinitive_stem.ends_with("jd") {
        format!(
            "{}{}šėl",
            prefix,
            &infinitive_stem[..infinitive_stem.len() - 2]
        )
    } else {
        format!("{prefix}{infinitive_stem}l")
    }
}

fn build_sonic_infinitive(prefix: &str, infinitive_stem: &str) -> String {
    let mut stem = infinitive_stem.to_string();
    if stem.ends_with("st") {
        stem.truncate(stem.len() - 1);
    } else if stem.ends_with('t')
        || (stem.ends_with('d') && !stem.ends_with("id") && !stem.ends_with("jd"))
    {
        stem.truncate(stem.len() - 1);
        stem.push('s');
    }
    transliterate_sonic_back(&format!("{prefix}{stem}tì"))
}

fn build_present_vec(prefix: &str, ps: &str, psi: &str) -> Vec<String> {
    (0..6)
        .map(|slot| sonic_present_form_by_slot(prefix, ps, psi, slot))
        .collect()
}

fn sonic_present_form_by_slot(prefix: &str, ps: &str, psi: &str, slot: usize) -> String {
    let raw = match ps {
        "jes" => ["jesm", "jesi", "jest", "jesmo", "jeste", "sųt"][slot].to_string(),
        "da" => format!(
            "{}{}",
            prefix,
            ["dam", "daš", "da", "damo", "date", "dadųt"][slot]
        ),
        "vě" => format!(
            "{}{}",
            prefix,
            ["věm", "věš", "vě", "věmo", "věte", "vědųt"][slot]
        ),
        "jě" => format!(
            "{}{}",
            prefix,
            ["jěm", "jěš", "jě", "jěmo", "jěte", "jědųt"][slot]
        ),
        "je" => format!(
            "{}{}",
            prefix,
            ["jem", "ješ", "je", "jemo", "jete", "jedųt"][slot]
        ),
        "ja" => format!(
            "{}{}",
            prefix,
            ["jam", "jaš", "ja", "jamo", "jate", "jadųt"][slot]
        ),
        _ if ps.ends_with('ĵ') => {
            let cut = &ps[..ps.len() - 'ĵ'.len_utf8()];
            let pps = format!("{cut}j");
            match slot {
                0 => format!("{prefix}{pps}ų"),
                1 => format!("{prefix}{pps}eš"),
                2 => format!("{prefix}{pps}e"),
                3 => format!("{prefix}{pps}emo"),
                4 => format!("{prefix}{pps}ete"),
                _ => format!("{prefix}{pps}ųt"),
            }
        }
        _ if ps.ends_with('i') => {
            let cut = &ps[..ps.len() - 'i'.len_utf8()];
            match slot {
                0 => format!("{prefix}{cut}xų"),
                1 => format!("{prefix}{ps}š"),
                2 => format!("{prefix}{ps}"),
                3 => format!("{prefix}{ps}mo"),
                4 => format!("{prefix}{ps}te"),
                _ => format!("{prefix}{cut}ęt"),
            }
        }
        _ => match slot {
            0 => format!("{prefix}{ps}ų"),
            1 => format!("{prefix}{psi}eš"),
            2 => format!("{prefix}{psi}e"),
            3 => format!("{prefix}{psi}emo"),
            4 => format!("{prefix}{psi}ete"),
            _ => format!("{prefix}{ps}ųt"),
        },
    };
    transliterate_sonic_back(&raw)
}

fn build_imperfect_vec(prefix: &str, infinitive_stem: &str) -> Vec<String> {
    let last = infinitive_stem.chars().last().unwrap_or('\0');
    let impst = if !matches!(
        last,
        'a' | 'e' | 'i' | 'o' | 'u' | 'y' | 'ę' | 'ų' | 'å' | 'ě' | 'ė' | 'ȯ' | ')'
    ) {
        if last == 'k' {
            format!("{}če", utils::string_without_last_n(infinitive_stem, 1))
        } else if infinitive_stem.ends_with("žeg") {
            "žže".to_string()
        } else if last == 'g' {
            format!("{}že", utils::string_without_last_n(infinitive_stem, 1))
        } else {
            format!("{infinitive_stem}e")
        }
    } else if infinitive_stem == "by" && prefix.is_empty() {
        "bě".to_string()
    } else {
        infinitive_stem.to_string()
    };
    ["h", "še", "še", "hmo", "ste", "hų"]
        .iter()
        .map(|ending| transliterate_sonic_back(&format!("{prefix}{impst}{ending}")))
        .collect()
}

fn build_future_vec(infinitive: &str, ps: &str) -> Vec<String> {
    let verb = if ((infinitive == "biti" || infinitive == "бити")
        && (ps == "j" || ps == "je" || ps == "jes"))
        || infinitive == "byti"
        || infinitive == "bytì"
        || infinitive == "быти"
    {
        "".to_string()
    } else {
        transliterate_sonic_back(infinitive)
    };
    ["bųdų", "bųdeš", "bųde", "bųdemo", "bųdete", "bųdųt"]
        .iter()
        .map(|aux| format!("{aux} {verb}"))
        .collect()
}

fn build_perfect_vec(lpa: &str) -> Vec<String> {
    [
        format!("jesm {lpa}(a)"),
        format!("jesi {lpa}(a)"),
        format!("(je) {lpa}"),
        format!("(je) {lpa}a"),
        format!("(je) {lpa}o"),
        format!("jesmo {lpa}i"),
        format!("jeste {lpa}i"),
        format!("(sųt) {lpa}i"),
    ]
    .into_iter()
    .map(|line| postprocess_lpa_line(&line))
    .collect()
}

fn build_pluperfect_vec(lpa: &str) -> Vec<String> {
    [
        format!("běh {lpa}(a)"),
        format!("běše {lpa}(a)"),
        format!("běše {lpa}"),
        format!("běše {lpa}a"),
        format!("běše {lpa}o"),
        format!("běhmo {lpa}i"),
        format!("běste {lpa}i"),
        format!("běhų {lpa}i"),
    ]
    .into_iter()
    .map(|line| postprocess_lpa_line(&line))
    .collect()
}

/// The person-marked conditional auxiliary — the single source both the
/// paradigm row and [`conditional_parts_with_hint`] draw from.
fn conditional_auxiliary(person: Person, number: Number) -> &'static str {
    match (person, number) {
        (Person::First, Number::Singular) => "byh",
        (Person::Second, Number::Singular) => "bys",
        (Person::Third, Number::Singular) => "by",
        (Person::First, Number::Plural) => "byhmo",
        (Person::Second, Number::Plural) => "byste",
        (Person::Third, Number::Plural) => "by",
    }
}

fn build_conditional_vec(lpa: &str) -> Vec<String> {
    let aux = conditional_auxiliary;
    [
        format!("{} {lpa}(a)", aux(Person::First, Number::Singular)),
        format!("{} {lpa}(a)", aux(Person::Second, Number::Singular)),
        format!("{} {lpa}", aux(Person::Third, Number::Singular)),
        format!("{} {lpa}a", aux(Person::Third, Number::Singular)),
        format!("{} {lpa}o", aux(Person::Third, Number::Singular)),
        format!("{} {lpa}i", aux(Person::First, Number::Plural)),
        format!("{} {lpa}i", aux(Person::Second, Number::Plural)),
        format!("{} {lpa}i", aux(Person::Third, Number::Plural)),
    ]
    .into_iter()
    .map(|line| postprocess_lpa_line(&line))
    .collect()
}

fn postprocess_lpa_line(line: &str) -> String {
    let mut res = if line.contains("šėl") {
        idti(line)
    } else {
        line.to_string()
    };
    if res.contains("žegl") {
        res = zegti(&res);
    }
    transliterate_sonic_back(&res)
}

fn build_imperative_vec(prefix: &str, ps: &str) -> Vec<String> {
    let chars: Vec<char> = ps.chars().collect();
    let last = chars.last().copied().unwrap_or('\0');
    let penultimate = chars.iter().rev().nth(1).copied().unwrap_or('\0');
    let mut p2s = if ps == "jes" {
        "bųď".to_string()
    } else if ps == "da" {
        format!("{prefix}{ps}j")
    } else if is_irregular_stem(ps) {
        format!("{prefix}{ps}ď")
    } else if (last == 'ĵ' || last == 'j') && !(penultimate == 'l' || penultimate == 'n') {
        format!("{prefix}{ps}")
    } else if last == 'a' || last == 'e' || last == 'ě' {
        format!("{prefix}{ps}j")
    } else if last == 'i' {
        format!("{prefix}{ps}")
    } else {
        format!("{prefix}{ps}i")
    };
    p2s = p2s.replace("jij", "j").replace("ĵij", "ĵ");
    [p2s.clone(), format!("{p2s}mo"), format!("{p2s}te")]
        .into_iter()
        .map(|form| transliterate_sonic_back(&form))
        .collect()
}

fn build_prap(prefix: &str, ps: &str) -> String {
    let last = ps.chars().last().unwrap_or('\0');
    let base = if ps == "jes" {
        format!("{prefix}sų")
    } else if is_irregular_stem(ps) {
        format!("{prefix}{ps}dų")
    } else if last == 'a' || last == 'e' || last == 'ě' {
        format!("{prefix}{ps}jų")
    } else if last == 'i' {
        format!("{}{}ę", prefix, utils::string_without_last_n(ps, 1))
    } else {
        format!("{prefix}{ps}ų")
    };
    transliterate_sonic_back(&format!("{base}ćí ({base}ćá, {base}ćé)"))
}

fn build_prpp(prefix: &str, ps: &str, psi: &str) -> String {
    let mut result = String::new();
    let mut ps = ps.to_string();
    if ps == "jes" {
        result = "—".to_string();
    } else if is_irregular_stem(&ps) {
        ps = format!("{ps}do");
    }
    let last = ps.chars().last().unwrap_or('\0');
    if last == 'ĵ' {
        let cut = utils::string_without_last_n(&ps, 1);
        let psj = format!("{cut}j");
        result = format!("{prefix}{psj}emý (—á, —œ), {prefix}{cut}mý (—á, —œ)");
    } else if last == 'j' {
        result = format!("{prefix}{psi}emý ({prefix}{psi}emá, {prefix}{psi}emœ)");
    } else if matches!(last, 's' | 'z' | 't' | 'd' | 'l') {
        result = format!("{prefix}{ps}omý ({prefix}{ps}omá, {prefix}{ps}omœ)");
    } else if last == 'i' || last == 'o' {
        result = format!("{prefix}{ps}mý ({prefix}{ps}má, {prefix}{ps}mœ)");
    } else if result != "—" {
        result = format!("{prefix}{psi}emý ({prefix}{psi}emá, {prefix}{psi}emœ)");
    }
    transliterate_sonic_back(&result)
}

fn build_pfap(lpa: &str) -> String {
    let before_l = lpa.chars().rev().nth(1).unwrap_or('\0');
    let mut result = if matches!(
        before_l,
        'a' | 'e' | 'i' | 'o' | 'u' | 'y' | 'ę' | 'ų' | 'å' | 'ě' | 'ė' | 'ȯ' | ')'
    ) {
        format!("{}vši", utils::string_without_last_n(lpa, 1))
    } else {
        format!("{}ši", utils::string_without_last_n(lpa, 1))
    };
    if result.contains("šėv") {
        result = idti(&result);
    }
    let fem_neut_stem = utils::string_without_last_n(&result, 1);
    result = format!("{result} ({fem_neut_stem}á, {fem_neut_stem}é)");
    transliterate_sonic_back(&result)
}

fn build_pfpp(prefix: &str, infinitive_stem: &str, psi: &str) -> String {
    let is = infinitive_stem;
    let last = is.chars().last().unwrap_or('\0');
    let ppps = if (matches!(last, 'i' | 'y' | 'u' | 'ě')
        && psi.ends_with(['j', 'v', 'n'])
        && psi != "imaj")
        || matches!(last, 'ę' | 'u' | 'ų' | 'å')
        || is == "by"
        || (is.ends_with("lě") && psi.ends_with("lj"))
    {
        format!("{prefix}{is}t")
    } else if last == 'a' || last == 'e' || last == 'ě' {
        format!("{prefix}{is}n")
    } else if last == 'i' {
        let mut ppps = format!("{prefix}{is}Xen");
        for (from, to) in [
            ("stiX", "šćX"),
            ("zdiX", "žđX"),
            ("siX", "šX"),
            ("ziX", "žX"),
            ("tiX", "ćX"),
            ("diX", "đX"),
            ("jiX", "jX"),
            ("šiX", "šX"),
            ("žiX", "žX"),
            ("čiX", "čX"),
            ("iX", "jX"),
            ("X", ""),
        ] {
            ppps = ppps.replace(from, to);
        }
        ppps
    } else if last == 'k' || last == 'g' {
        if psi.ends_with('i') {
            format!("{}{}en", prefix, utils::string_without_last_n(psi, 1))
        } else {
            format!("{prefix}{psi}en")
        }
    } else {
        format!("{prefix}{is}en")
    };
    transliterate_sonic_back(&format!("{ppps}ý ({ppps}á, {ppps}ó)"))
}

fn build_gerund(pfpp: &str) -> String {
    let stem = pfpp.split('ý').next().unwrap_or(pfpp).trim_end();
    transliterate_sonic_back(&format!("{stem}ıje")).replace("ne ", "ne")
}

fn is_irregular_stem(ps: &str) -> bool {
    matches!(ps, "da" | "je" | "jě" | "ja" | "vě")
}

fn idti(input: &str) -> String {
    input
        .replace("šėl(a)", "šėl/šla")
        .replace("šėl(a)", "šėl/šla")
        .replace("všėl/šla", "všėl/vȯšla")
        .replace("všėl/šla", "všėl/vȯšla")
        .replace("šėla", "šla")
        .replace("šėlo", "šlo")
        .replace("šėli", "šli")
        .replace("všl", "vȯšl")
        .replace("izoš", "izš")
        .replace("izȯš", "izš")
        .replace("obȯš", "obš")
        .replace("oboš", "obš")
        .replace("odȯš", "odš")
        .replace("odoš", "odš")
        .replace("podȯš", "podš")
        .replace("podoš", "podš")
        .replace("nadȯš", "nadš")
        .replace("nadoš", "nadš")
}

fn zegti(input: &str) -> String {
    if input.ends_with("žegl(a)") {
        input.replace("žegl(a)", "žegl/žgla")
    } else {
        input
            .replace("žegla", "žgla")
            .replace("žeglo", "žglo")
            .replace("žegli", "žgli")
    }
}

fn transliterate_sonic_back(input: &str) -> String {
    input
        .replace("stx", "šć")
        .replace("zdx", "žđ")
        .replace("sx", "š")
        .replace("šx", "š")
        .replace("zx", "ž")
        .replace("žx", "ž")
        .replace("cx", "č")
        .replace("čx", "č")
        .replace("tx", "ć")
        .replace("dx", "đ")
        .replace("jx", "j")
        .replace('x', "j")
        .replace('-', "")
        .replace('—', "-")
        .replace("lı", "ľ")
        .replace("nı", "ń")
        .replace("rı", "ŕ")
        .replace("tı", "ť")
        .replace("dı", "ď")
        .replace("sı", "ś")
        .replace("zı", "ź")
        .replace('ı', "")
}

/// The l-participle of an infinitive — the gender/number-marked form the
/// perfect, pluperfect, and conditional are built on (`pisal`/`pisala`/
/// `pisalo`, plural `pisali`). Built on the same stem derivation and
/// suppletion handling as [`verb_paradigm_with_options`], so the result
/// always matches the participle inside the paradigm's compound tenses:
/// the `idti` family is suppletive (`šėl`/`šla`/`šlo`/`šli`, `došėl`/
/// `došla`, …) and `žegti` alternates (`žegl`/`žgla`). Total: a word
/// without an infinitive shape degrades to the same best-effort stem
/// [`get_infinitive_stem`] uses.
pub fn l_participle(word: &str, gender: Gender, number: Number) -> String {
    l_participle_with_hint(word, "", gender, number)
}

/// The perfect tense decomposed into data instead of a convention-string:
/// the auxiliary (where the standard requires one) and the correctly
/// gendered l-participle, with no bracket conventions to parse.
///
/// `auxiliary` is `Some("jesm"/"jesi"/"jesmo"/"jeste")` in the first and
/// second persons and `None` in the third: per the grammar (steen,
/// "Verbs": the auxiliary in the 3rd person is normally omitted), `on
/// ukradl`, not `on jest ukradl`. A caller that wants the emphatic
/// third-person auxiliary adds `je`/`jest` (singular) or `sųt` (plural)
/// itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerfectParts {
    pub auxiliary: Option<String>,
    pub participle: String,
}

/// The perfect tense of a verb as structured [`PerfectParts`], with an
/// explicit dictionary present-stem hint. The participle is
/// [`l_participle_with_hint`]'s output — the same shared stem context the
/// paradigm's compound tenses use, so this accessor and
/// [`verb_paradigm_with_options`]'s `perfect` cells can never disagree.
pub fn perfect_parts_with_hint(
    word: &str,
    present_hint: &str,
    person: Person,
    number: Number,
    gender: Gender,
) -> PerfectParts {
    let auxiliary = match (person, number) {
        (Person::First, Number::Singular) => Some("jesm"),
        (Person::Second, Number::Singular) => Some("jesi"),
        (Person::First, Number::Plural) => Some("jesmo"),
        (Person::Second, Number::Plural) => Some("jeste"),
        (Person::Third, _) => None,
    };
    PerfectParts {
        auxiliary: auxiliary.map(str::to_string),
        participle: l_participle_with_hint(word, present_hint, gender, number),
    }
}

/// The conditional mood decomposed into data: the person-marked
/// auxiliary and the correctly gendered l-participle, with no bracket
/// conventions to parse — the conditional counterpart of
/// [`PerfectParts`]. The auxiliary comes from the same table the
/// paradigm's conditional row is built from, and the participle from the
/// shared stem context, so this accessor and
/// [`verb_paradigm_with_options`]'s `conditional` cells can never
/// disagree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionalParts {
    pub auxiliary: String,
    pub participle: String,
}

/// The conditional of a verb as structured [`ConditionalParts`], with an
/// explicit dictionary present-stem hint.
pub fn conditional_parts_with_hint(
    word: &str,
    present_hint: &str,
    person: Person,
    number: Number,
    gender: Gender,
) -> ConditionalParts {
    ConditionalParts {
        auxiliary: conditional_auxiliary(person, number).to_string(),
        participle: l_participle_with_hint(word, present_hint, gender, number),
    }
}

/// The l-participle with an explicit dictionary present-stem hint — the
/// hint-aware counterpart of [`l_participle`]. The hint is what recovers
/// the dental stem of the d/t-stem `-sti` verbs (`ukrasti` + `(ukrade)` →
/// `ukradla`, not `*ukrasla`), exactly as it does inside
/// [`verb_paradigm_with_options`]: both derive their stem from the same
/// context, so the standalone participle and the one inside the compound
/// tenses can never disagree.
pub fn l_participle_with_hint(
    word: &str,
    present_hint: &str,
    gender: Gender,
    number: Number,
) -> String {
    let word = word.trim();
    let (prefix, _, infinitive_stem) =
        stem_context(word, present_hint, false).expect("non-strict stem derivation is total");
    let lpa = sonic_l_participle(&prefix, &infinitive_stem);
    let form = match (number, gender) {
        (Number::Plural, _) => format!("{lpa}i"),
        (Number::Singular, Gender::Masculine) => lpa,
        (Number::Singular, Gender::Feminine) => format!("{lpa}a"),
        (Number::Singular, Gender::Neuter) => format!("{lpa}o"),
    };
    postprocess_lpa_line(&form)
}
