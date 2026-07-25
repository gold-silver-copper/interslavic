//! Noun declension and noun-specific inference and stem helpers.

use crate::known_nouns::*;
use crate::{Animacy, Case, Gender, Number, adjective, utils};

#[allow(clippy::too_many_arguments)]
pub fn decline_noun_explicit(
    lemma: &str,
    case: Case,
    number: Number,
    gender: Gender,
    animacy: Animacy,
    plural_only: bool,
    singular_only: bool,
    indeclinable: bool,
    addition: Option<&str>,
) -> String {
    decline_noun_steen(
        lemma,
        addition.unwrap_or(""),
        gender,
        animacy == Animacy::Animate,
        plural_only,
        singular_only,
        indeclinable,
        case,
        number,
    )
}

pub fn decline_noun_simple(
    lemma: &str,
    case: Case,
    number: Number,
    gender: Gender,
    animacy: Animacy,
) -> String {
    decline_noun_explicit(
        lemma, case, number, gender, animacy, false, false, false, None,
    )
}

pub fn decline_noun(word: &str, case: Case, number: Number) -> String {
    let word = word.trim();
    let gender = guess_gender(word);
    let word_is_animate = noun_is_animate(word);
    decline_noun_steen(
        word,
        "",
        gender,
        word_is_animate,
        false,
        false,
        false,
        case,
        number,
    )
}

#[allow(clippy::too_many_arguments)]
fn decline_noun_steen(
    word: &str,
    add: &str,
    origin_gender: Gender,
    animate: bool,
    plural_only: bool,
    singular_only: bool,
    indeclinable: bool,
    case: Case,
    number: Number,
) -> String {
    let noun = remove_bracketed_text(word.trim(), '[', ']');
    if indeclinable {
        return noun;
    }
    if plural_only {
        return decline_plural_only_noun(&noun, add, origin_gender, case, number).unwrap_or(noun);
    }
    if singular_only && number == Number::Plural {
        return noun;
    }
    if let Some(form) =
        decline_substantivized_adjective(&noun, add, origin_gender, animate, case, number)
    {
        return form;
    }

    let noun = mark_or_infer_fluent_vowel(&noun, add);
    let marked_noun =
        mark_final_soft_noun_consonants(&noun.replace("(e)", "ė").replace("(o)", "ȯ"));
    let noun_without_fluent =
        mark_final_soft_noun_consonants(&noun.replace("(e)", "").replace("(o)", ""));
    let raw_gender = prepare_noun_gender(origin_gender, animate);
    let gender = establish_noun_gender(&marked_noun, &raw_gender);
    let root = establish_noun_root(&noun_without_fluent, &gender);
    let plural_root = establish_plural_noun_root(&root);
    let plural_gender = establish_plural_noun_gender(&root, &plural_root, &gender, &raw_gender);

    match number {
        Number::Singular => match case {
            Case::Nom => noun_nominative_sg(&marked_noun, &root, &gender),
            Case::Acc => {
                let nominative_sg = noun_nominative_sg(&marked_noun, &root, &gender);
                noun_accusative_sg(&nominative_sg, &root, &gender)
            }
            Case::Gen => noun_genitive_sg(&root, &gender),
            Case::Loc => noun_locative_sg(&root, &gender),
            Case::Dat => noun_dative_sg(&root, &gender),
            Case::Ins => noun_instrumental_sg(&root, &gender),
        },
        Number::Plural => match case {
            Case::Nom => noun_nominative_pl(&plural_root, &plural_gender),
            Case::Acc => {
                let nom = noun_nominative_pl(&plural_root, &plural_gender);
                let r#gen = noun_genitive_pl(&plural_root, &plural_gender);
                if plural_gender == "m1" { r#gen } else { nom }
            }
            Case::Gen => noun_genitive_pl(&plural_root, &plural_gender),
            Case::Loc => noun_locative_pl(&plural_root, &gender),
            Case::Dat => noun_dative_pl(&plural_root, &gender),
            Case::Ins => noun_instrumental_pl(&plural_root, &gender),
        },
    }
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

fn mark_or_infer_fluent_vowel(word: &str, add: &str) -> String {
    let add = add.trim().trim_matches(['(', ')']);
    if !add.is_empty() && word != add {
        mark_fluent_vowel(word, add)
    } else {
        infer_fluent_vowel(word)
    }
}

fn mark_fluent_vowel(word: &str, add: &str) -> String {
    let word_chars: Vec<char> = word.chars().collect();
    let add_chars: Vec<char> = add.chars().collect();
    let limit = word_chars.len().saturating_sub(1).min(add_chars.len());
    let mut idx = 0usize;
    while idx < limit && word_chars[idx] == add_chars[idx] {
        idx += 1;
    }
    if idx + 1 < word_chars.len() && idx < add_chars.len() && word_chars[idx + 1] == add_chars[idx]
    {
        replace_fluent_vowel(word, idx)
    } else {
        word.to_string()
    }
}

pub(crate) fn infer_fluent_vowel(word: &str) -> String {
    if word.ends_with("lv") || word.ends_with("ėj") {
        return word.to_string();
    }
    let chars: Vec<char> = word.chars().collect();
    let mut result = word.to_string();
    let mut replaced = false;
    let mut end = chars.len();

    for idx in (0..chars.len()).rev() {
        let c = chars[idx];
        if !is_dictionary_letter(c) {
            end = idx;
            replaced = false;
        }
        if !replaced
            && matches!(c, 'è' | 'ė' | 'ȯ' | 'ò')
            && is_last_fluent_syllable(&chars, idx, end)
        {
            result = replace_fluent_vowel(&result, idx);
            replaced = true;
        }
    }
    result
}

fn replace_fluent_vowel(word: &str, index: usize) -> String {
    let mut result = String::new();
    for (idx, c) in word.chars().enumerate() {
        if idx == index {
            let plain = match c {
                'ė' | 'è' => 'e',
                'ȯ' | 'ò' => 'o',
                _ => c,
            };
            result.push('(');
            result.push(plain);
            result.push(')');
        } else {
            result.push(c);
        }
    }
    result
}

fn is_last_fluent_syllable(chars: &[char], idx: usize, end: usize) -> bool {
    if idx + 2 == end {
        chars.get(idx + 1).is_some_and(utils::is_consonant)
    } else if idx + 3 == end {
        chars.get(idx + 1) == Some(&'n') && chars.get(idx + 2) == Some(&'j')
    } else {
        false
    }
}

fn is_dictionary_letter(c: char) -> bool {
    c.is_alphabetic()
        || matches!(
            c,
            'å' | 'ę'
                | 'ė'
                | 'ȯ'
                | 'ų'
                | 'ě'
                | 'ć'
                | 'č'
                | 'ď'
                | 'đ'
                | 'ĺ'
                | 'ľ'
                | 'ń'
                | 'ŕ'
                | 'ś'
                | 'š'
                | 'ť'
                | 'ź'
                | 'ž'
        )
}

fn decline_plural_only_noun(
    word: &str,
    add: &str,
    gender: Gender,
    case: Case,
    number: Number,
) -> Option<String> {
    if number == Number::Singular {
        return None;
    }

    let word_without_last = utils::string_without_last_n(word, 1);
    let add = add.trim().trim_matches(['(', ')']);
    if add.ends_with("yh") || add.ends_with("ih") {
        let i_or_y = if add.ends_with("yh") { "y" } else { "i" };
        return Some(match case {
            Case::Nom | Case::Acc => word.to_string(),
            Case::Gen | Case::Loc => format!("{}{}h", word_without_last, i_or_y),
            Case::Dat => format!("{}{}m", word_without_last, i_or_y),
            Case::Ins => format!("{}{}mi", word_without_last, i_or_y),
        });
    }
    if !add.is_empty() {
        return None;
    }

    match gender {
        Gender::Masculine if word.ends_with(['i', 'y', 'e']) => Some(match case {
            Case::Nom | Case::Acc => word.to_string(),
            Case::Gen => format!("{}ov", word_without_last),
            Case::Loc => format!("{}ah", word_without_last),
            Case::Dat => format!("{}am", word_without_last),
            Case::Ins => format!("{}ami", word_without_last),
        }),
        Gender::Feminine if word.ends_with(['y', 'e']) => Some(match case {
            Case::Nom | Case::Acc => word.to_string(),
            Case::Gen if word.ends_with("je") => word_without_last.clone(),
            Case::Gen => noun_rules(&plural_gen_ending(&(word_without_last.clone() + "%"), true)),
            Case::Loc => format!("{}ah", word_without_last),
            Case::Dat => format!("{}am", word_without_last),
            Case::Ins => format!("{}ami", word_without_last),
        }),
        Gender::Neuter if word.ends_with('a') => Some(match case {
            Case::Nom | Case::Acc => word.to_string(),
            Case::Gen => noun_rules(&plural_gen_ending(&(word_without_last.clone() + "%"), true)),
            Case::Loc => format!("{}ah", word_without_last),
            Case::Dat => format!("{}am", word_without_last),
            Case::Ins => format!("{}ami", word_without_last),
        }),
        Gender::Feminine if word.ends_with('i') => Some(match case {
            Case::Nom | Case::Acc => word.to_string(),
            Case::Gen => format!("{}ij", word_without_last),
            Case::Loc => format!("{}ah", word_without_last),
            Case::Dat => format!("{}am", word_without_last),
            Case::Ins => format!("{}ami", word_without_last),
        }),
        _ => None,
    }
}

fn decline_substantivized_adjective(
    word: &str,
    add: &str,
    gender: Gender,
    animate: bool,
    case: Case,
    number: Number,
) -> Option<String> {
    let add = add.trim().trim_matches(['(', ')']);
    let stem = utils::string_without_last_n(word, 1);
    let suffix = if ["-ogo", "-ego", "-oj", "-ej"].contains(&add) {
        Some(add.to_string())
    } else if !add.is_empty() && add.starts_with(&stem) {
        Some(add.replacen(&stem, "-", 1))
    } else {
        None
    }?;

    if !["-ogo", "-ego", "-oj", "-ej"].contains(&suffix.as_str()) {
        return None;
    }

    let adjective = match gender {
        Gender::Masculine | Gender::Neuter => stem + if suffix == "-ogo" { "y" } else { "i" },
        Gender::Feminine => stem + if suffix == "-oj" { "y" } else { "i" },
    };

    Some(adjective::decline_adj(
        &adjective,
        case,
        number,
        gender,
        if animate {
            Animacy::Animate
        } else {
            Animacy::Inanimate
        },
    ))
}

fn prepare_noun_gender(gender: Gender, animate: bool) -> String {
    match gender {
        Gender::Feminine => "f".into(),
        Gender::Neuter => "n".into(),
        Gender::Masculine => {
            if animate {
                "m1".into()
            } else {
                "m2".into()
            }
        }
    }
}

fn mark_final_soft_noun_consonants(word: &str) -> String {
    let mut normalized = word.to_string();
    for (from, to) in [("ń", "nj"), ("ň", "nj"), ("ľ", "lj"), ("ĺ", "lj")] {
        if normalized.ends_with(from) {
            normalized = utils::replace_last_occurence(&normalized, from, to);
        }
    }

    let chars: Vec<char> = normalized.chars().collect();
    let mark_from = chars.len().saturating_sub(2);
    let mut result = String::new();
    for (idx, c) in chars.iter().enumerate() {
        result.push(*c);
        if idx >= mark_from && "cšžčćńľŕťďśźđj".contains(*c) {
            // A LEMMA ending in soft consonant + `o` is a foreign citation
            // form (adadžo, bandžo): native soft neuters already end in -e
            // (morje). Marking the softness there would O⇒E-convert the
            // nominative itself (adadžo → adadže) — the citation form must
            // echo. Oblique cells are unaffected (the yer is stripped).
            let before_final_o = idx + 2 == chars.len() && chars.last() == Some(&'o');
            if !before_final_o {
                result.push('ь');
            }
        }
    }
    result
}

fn establish_noun_gender(noun: &str, raw_gender: &str) -> String {
    let last_char = utils::last_char(noun);
    let before_last_char = nth_char_from_end(noun, 2);
    let last_two = utils::last_n_chars(noun, 2);

    if noun == "den" || noun == "dėn" || noun == "denjь" || noun == "dėnjь" {
        return "m3".into();
    }
    if raw_gender.starts_with('m')
        && (last_two == "en" || noun.ends_with("enjь"))
        && (noun.starts_with("kamen")
            || noun.starts_with("jelen")
            || noun.starts_with("jęčmen")
            || noun.starts_with("ječmen")
            || noun.starts_with("koren")
            || noun.starts_with("kremen")
            || noun.starts_with("plåmen")
            || noun.starts_with("plamen")
            || noun.starts_with("pŕsten")
            || noun.starts_with("prsten")
            || noun.starts_with("strumen")
            || noun.starts_with("greben")
            || noun.starts_with("stępen")
            || noun.starts_with("stepen")
            || noun.starts_with("stųpen")
            || noun.starts_with("stupen")
            || noun.starts_with("šršen")
            || noun.starts_with("šŕšen")
            || noun.starts_with("sršen")
            || noun.starts_with("sŕšen")
            || noun.starts_with("šeršen"))
    {
        return "m3".into();
    }
    if raw_gender.starts_with('n')
        && [
            "čudo", "dělo", "divo", "drěvo", "igo", "kolo", "licьe", "nebo", "ojьe", "oko",
            "slovo", "tělo", "uho",
        ]
        .contains(&noun)
    {
        return "n3".into();
    }
    if raw_gender == "f" && last_char == Some('v') {
        return "f3".into();
    }
    if noun == "mati" || noun == "dočьi" || noun == "doćьi" || noun == "doči" {
        return "f3".into();
    }
    if last_char == Some('a') || last_char == Some('i') {
        return "f1".into();
    }
    if last_char == Some('ę') {
        return "n2".into();
    }
    if before_last_char != Some('ь') && last_char == Some('e') {
        return "n2".into();
    }
    if last_char == Some('o') || last_char == Some('e') {
        return "n1".into();
    }
    if raw_gender == "m1" {
        return "m1".into();
    }
    if raw_gender == "f" {
        return "f2".into();
    }
    "m2".into()
}

fn establish_noun_root(noun: &str, gender: &str) -> String {
    let has_vowel_ending = matches!(utils::last_char(noun), Some('a' | 'e' | 'ę' | 'o'));
    let mut result = if noun == "lėv" || noun == "lev" {
        "ljv".into()
    } else if noun == "Lėv" || noun == "Lev" {
        "Ljv".into()
    } else if gender.starts_with('m')
        && (noun.ends_with("ecь") || noun.ends_with("ėcь") || noun.ends_with("ècь"))
        && masculine_ec_keeps_c(noun)
    {
        utils::string_without_last_n(noun, 3) + "cь"
    } else if gender == "m3" {
        noun.trim_end_matches("jь").to_string()
    } else if noun == "mati" || noun == "dočьi" || noun == "doćьi" || noun == "doči" {
        utils::string_without_last_n(noun, 1) + "er"
    } else if gender == "f3" && noun.ends_with("ov") {
        utils::string_without_last_n(noun, 2) + "v"
    } else if gender == "f3" {
        noun.into()
    } else if gender == "n2" && nth_char_from_end(noun, 2) == Some('m') {
        utils::string_without_last_n(noun, 1) + "en"
    } else if gender == "n2" {
        utils::string_without_last_n(noun, 1) + "ęt"
    } else if gender == "f1" && (noun == "pani" || noun.ends_with("yni")) {
        utils::string_without_last_n(noun, 1) + "jь"
    } else if noun.ends_with('i') {
        utils::string_without_last_n(noun, 1) + "ь"
    } else if has_vowel_ending {
        utils::string_without_last_n(noun, 1)
    } else {
        noun.into()
    };

    let fluent_vowel_index = if has_vowel_ending {
        None
    } else {
        last_fluent_vowel_index(noun)
    };
    if let Some(index) = fluent_vowel_index {
        let result_len = result.chars().count();
        if index > result_len.saturating_sub(3) {
            result = remove_char_at(&result, index);
        }
    }
    result
}

fn masculine_ec_keeps_c(noun: &str) -> bool {
    let chars: Vec<char> = noun.chars().collect();
    if chars.len() < 5 {
        return false;
    }
    let before = chars[chars.len() - 5];
    let near = chars[chars.len() - 4];
    "aeiouyęųåėěȯrŕ".contains(before) || "jdtc".contains(near)
}

fn establish_plural_noun_root(root: &str) -> String {
    match root {
        "dětęt" | "detet" | "dětet" | "detęt" => "dětь".into(),
        "človek" | "člověk" => "ljudь".into(),
        "ok" => "očь".into(),
        "uh" => "ušь".into(),
        _ if root.ends_with("anin") => utils::string_without_last_n(root, 2),
        _ => root.into(),
    }
}

fn establish_plural_noun_gender(
    root: &str,
    plural_root: &str,
    gender: &str,
    raw_gender: &str,
) -> String {
    if root != plural_root && !plural_root.contains('n') {
        return "f2".into();
    }
    if gender == "f1" && raw_gender == "m1" {
        return "m1".into();
    }
    gender.into()
}

fn noun_nominative_sg(noun: &str, root: &str, gender: &str) -> String {
    let result = if gender == "f2" && (noun.contains('ȯ') || noun.contains('ė')) {
        noun.into()
    } else if gender == "f2" {
        root.into()
    } else if gender == "f3" {
        noun.into()
    } else if gender == "m3" && root == "dn" {
        "den / denj".into()
    } else if gender == "m3" {
        format!("{} / {}j", root, root)
    } else {
        noun.into()
    };
    noun_rules(&result)
}

fn noun_accusative_sg(noun: &str, root: &str, gender: &str) -> String {
    let result = if gender == "m1" {
        format!("{}a", root)
    } else if gender == "f1" {
        format!("{}ų", root)
    } else {
        noun.into()
    };
    noun_rules(&result)
}

fn noun_genitive_sg(root: &str, gender: &str) -> String {
    let result = match gender {
        "m1" | "m2" | "n1" => format!("{}a", root),
        "f1" => format!("{}y", root),
        "f2" => format!("{}i", root),
        "f3" => format!("{}e / {}i", root, root),
        "m3" => format!("{}e / {}ja", root, root),
        "n2" => format!("{}e / {}a", root, root),
        "n3" => format!("{}a / {}ese", root, palatalization_ending(root)),
        _ => root.into(),
    };
    noun_rules(&result)
}

fn noun_dative_sg(root: &str, gender: &str) -> String {
    let result = match gender {
        "m1" | "m2" | "n1" => format!("{}u", root),
        "f1" => format!("{}ě", root),
        "f2" | "f3" => format!("{}i", root),
        "m3" => format!("{}i / {}ju", root, root),
        "n2" => format!("{}i / {}u", root, root),
        "n3" => format!("{}u / {}esi", root, palatalization_ending(root)),
        _ => root.into(),
    };
    noun_rules(&result)
}

fn noun_locative_sg(root: &str, gender: &str) -> String {
    noun_dative_sg(root, gender)
}

fn noun_instrumental_sg(root: &str, gender: &str) -> String {
    let result = match gender {
        "m1" | "m2" | "n1" => format!("{}om", root),
        "f1" => format!("{}ojų", root),
        "f3" if is_feminine_v_stem_with_restored_o(root) => {
            format!("{}ȯvjų", utils::string_without_last_n(root, 1))
        }
        "f2" | "f3" => format!("{}jų", root),
        "m3" => format!("{}em / {}jem", root, root),
        "n2" => format!("{}em / {}om", root, root),
        "n3" => format!("{}om / {}esem", root, palatalization_ending(root)),
        _ => root.into(),
    };
    noun_rules(&result)
}

fn is_feminine_v_stem_with_restored_o(root: &str) -> bool {
    let chars: Vec<char> = root.chars().collect();
    if chars.len() < 2 || chars.last() != Some(&'v') {
        return false;
    }
    let before_v = chars[chars.len() - 2];
    let vowels = "aåeęěėioȯuųy";
    if before_v == 'l' && chars.len() >= 3 {
        return chars[chars.len() - 3] != 'o' && chars[chars.len() - 3] != 'ȯ';
    }
    !vowels.contains(before_v)
}

fn noun_nominative_pl(root: &str, gender: &str) -> String {
    let result = if gender == "n3" {
        format!("{}a / {}esa", root, palatalization_ending(root))
    } else if root == "očь" || root == "ušь" {
        format!("{}i / {}esa", root, root)
    } else if gender.starts_with('n') {
        format!("{}a", root)
    } else if gender == "m1" {
        format!("{}i", root)
    } else if gender == "f1" || gender == "m2" {
        format!("{}y", root)
    } else if gender == "m3" {
        format!("{}i / {}je", root, root)
    } else {
        format!("{}i", root)
    };
    noun_rules(&result)
}

fn noun_genitive_pl(root: &str, gender: &str) -> String {
    let (result, use_ej_for_j_percent) = if gender == "f1" || root == "morjь" || root == "poljь" {
        (root.replacen('ь', "%", 1) + "%", true)
    } else if root == "st" {
        ("sȯt".into(), true)
    } else if gender.starts_with('n') {
        let mut result = root.replacen('ь', "%", 1) + "%";
        if gender == "n3" {
            result = format!("{} / {}es", result, palatalization_ending(root));
        }
        (result, false)
    } else if gender == "m3" {
        (format!("{}ev / {}jev", root, root), true)
    } else if gender.starts_with('m') {
        (format!("{}ov", root), true)
    } else if root == "očь" || root == "ušь" {
        (format!("{}ij / {}es", root, root), true)
    } else {
        (format!("{}ij", root), true)
    };
    noun_rules(&plural_gen_ending(&result, use_ej_for_j_percent))
}

fn noun_dative_pl(root: &str, gender: &str) -> String {
    let result = if gender == "m3" {
        format!("{}am / {}jam", root, root)
    } else if gender == "n3" {
        format!("{}am / {}esam", root, palatalization_ending(root))
    } else {
        format!("{}am", root)
    };
    noun_rules(&result)
}

fn noun_instrumental_pl(root: &str, gender: &str) -> String {
    let result = if gender == "m3" {
        format!("{}ami / {}jami", root, root)
    } else if gender == "n3" {
        format!("{}ami / {}esami", root, palatalization_ending(root))
    } else {
        format!("{}ami", root)
    };
    noun_rules(&result)
}

fn noun_locative_pl(root: &str, gender: &str) -> String {
    let result = if gender == "m3" {
        format!("{}ah / {}jah", root, root)
    } else if gender == "n3" {
        format!("{}ah / {}esah", root, palatalization_ending(root))
    } else {
        format!("{}ah", root)
    };
    noun_rules(&result)
}

fn noun_rules(word: &str) -> String {
    word.replace("ьo", "ьe")
        .replace("ьy", "ьe")
        .replace("ьě", "i")
        .replace('#', "")
        .replace("tь", "ť")
        .replace("dь", "ď")
        .replace("sь", "ś")
        .replace("zь", "ź")
        .replace('ь', "")
        .replace("ťi", "ti")
        .replace("ďi", "di")
        .replace("śi", "si")
        .replace("źi", "zi")
        .replace("ľi", "li")
        .replace("ńi", "ni")
        .replace("ŕi", "ri")
        .replace("jy", "ji")
        .replace("cy", "ci")
        .replace("ljj", "ľj")
        .replace("njj", "ńj")
}

fn plural_gen_ending(word: &str, use_ej_for_j_percent: bool) -> String {
    let mut word = word
        .replace("jsk%", "jsk")
        .replace("mš%", "meš")
        .replace("zl%", "zȯl")
        .replace("tl%", "tȯl")
        .replace("mgl%", "mgȯl")
        .replace("ńj%", "nij");

    if use_ej_for_j_percent {
        word = replace_j_percent(&word, "pbvfmlnr", "ej");
    }
    word = replace_j_percent(&word, "pbvfmlnrszńľŕťďśźščžđ", "ij");
    word = replace_final_pair_percent(&word, "jśźďťľŕńčšžćđc", 'k', "e");
    word = replace_final_pair_percent(&word, "pbfvmlnrtdszkgh", 'k', "ȯ");
    word = replace_final_pair_percent(&word, "vmpzšžt", 'n', "e");
    word = replace_first_second_percent(&word, 'k', "nl", "ȯ");
    word = replace_first_second_percent(&word, 's', "nl", "e");

    if word.starts_with("dn%") {
        word = word.replacen("dn%", "dȯn", 1);
    }
    if word.contains("pismo%") {
        word = word.replace("pismo%", "pisem");
    }
    if word.starts_with("ťm%") {
        word = word.replacen("ťm%", "tem", 1);
    }
    if word.starts_with("sto%") {
        word = word.replacen("sto%", "sȯt", 1);
    }
    word.replace('%', "")
}

fn replace_j_percent(word: &str, preceding: &str, replacement: &str) -> String {
    let chars: Vec<char> = word.chars().collect();
    for idx in 2..chars.len() {
        if chars[idx - 1] == 'j' && chars[idx] == '%' && preceding.contains(chars[idx - 2]) {
            let mut result: String = chars[..idx - 1].iter().collect();
            result.push_str(replacement);
            result.extend(chars[idx + 1..].iter());
            return result;
        }
    }
    word.into()
}

fn replace_final_pair_percent(word: &str, first_set: &str, second: char, insert: &str) -> String {
    let chars: Vec<char> = word.chars().collect();
    for idx in 2..chars.len() {
        if chars[idx] == '%' && chars[idx - 1] == second && first_set.contains(chars[idx - 2]) {
            let mut result: String = chars[..idx - 1].iter().collect();
            result.push_str(insert);
            result.push(second);
            result.extend(chars[idx + 1..].iter());
            return result;
        }
    }
    word.into()
}

fn replace_first_second_percent(word: &str, first: char, second_set: &str, insert: &str) -> String {
    let chars: Vec<char> = word.chars().collect();
    for idx in 2..chars.len() {
        if chars[idx] == '%' && second_set.contains(chars[idx - 1]) && chars[idx - 2] == first {
            let mut result: String = chars[..idx - 1].iter().collect();
            result.push_str(insert);
            result.push(chars[idx - 1]);
            result.extend(chars[idx + 1..].iter());
            return result;
        }
    }
    word.into()
}

fn palatalization_ending(root: &str) -> String {
    if root.ends_with('g') {
        utils::string_without_last_n(root, 1) + "žь"
    } else if root.ends_with('h') {
        utils::string_without_last_n(root, 1) + "šь"
    } else if root.ends_with('k') {
        utils::string_without_last_n(root, 1) + "čь"
    } else if root.ends_with("cь") {
        utils::string_without_last_n(root, 2) + "čь"
    } else {
        root.into()
    }
}

fn nth_char_from_end(s: &str, n: usize) -> Option<char> {
    s.chars().rev().nth(n - 1)
}

fn last_fluent_vowel_index(s: &str) -> Option<usize> {
    s.chars()
        .enumerate()
        .filter(|(_, c)| *c == 'ė' || *c == 'ȯ')
        .map(|(idx, _)| idx)
        .last()
}

fn remove_char_at(s: &str, index: usize) -> String {
    s.chars()
        .enumerate()
        .filter_map(|(idx, c)| if idx == index { None } else { Some(c) })
        .collect()
}
pub fn noun_is_animate(word: &str) -> bool {
    ANIMATE_MASCULINE_NOUNS.contains(&word)
}

pub fn guess_gender(word: &str) -> Gender {
    if ANIMATE_MASCULINE_NOUNS.contains(&word) || INANIMATE_MASCULINE_NOUNS.contains(&word) {
        Gender::Masculine
    } else if FEMININE_NOUNS.contains(&word) {
        Gender::Feminine
    } else if NEUTER_NOUNS.contains(&word) {
        Gender::Neuter
    } else {
        let last_one = utils::last_n_chars(word, 1);

        if is_ost_class(word) || (last_one == "a") || (last_one == "i") {
            Gender::Feminine
        } else if (last_one == "o") || (last_one == "e") {
            Gender::Neuter
        } else {
            Gender::Masculine
        }
    }
}

pub fn is_ost_class(word: &str) -> bool {
    word.ends_with("ost")
            || word.ends_with("ost́")
            // The precomposed soft-t spelling (U+0165), the common way the
            // abstract feminine suffix is written (novosť, točnosť). The
            // length guard leaves the two 4-char lexical words (kosť f,
            // gosť m!) to the dictionary lookup instead of the heuristic.
            || (word.ends_with("osť") && word.chars().count() >= 5)
            || word.ends_with("ěć")
            || word.ends_with("ěč")
            || word.ends_with("eć")
            || word.ends_with("at́")
}

pub fn get_noun_stem(word: &str, number: Number) -> String {
    if word.ends_with("anin") && number == Number::Plural {
        return utils::string_without_last_n(word, 2);
    }
    if word.ends_with("anina") && number == Number::Plural {
        return utils::string_without_last_n(word, 3);
    }

    if utils::last_char(word).is_some_and(|c| utils::is_vowel(&c)) {
        utils::string_without_last_n(word, 1)
    } else {
        String::from(word)
    }
}
pub fn stem_of_noun_is_soft(word: &str) -> bool {
    utils::ends_with_soft_consonant(&get_noun_stem(word, Number::Singular))
}

/// The vocative, built from the nominative citation form.
///
/// This is deliberately NOT a [`Case`] variant. The source is explicit
/// that the vocative "is not a real case, and it behaves significantly
/// different from other cases: it does not have a plural, it never
/// affects neuter nouns, adjectives or pronouns, and it has nothing to do
/// with the syntactic structure of the sentence" (nouns page). Modelling
/// it as a case would give every consumer six real cells plus one that
/// silently means something else, and would force adjectives and pronouns
/// to answer a question the language does not ask of them.
///
/// `None` is returned exactly where the source says the form is to be
/// avoided — feminine consonant stems and neuters — because "the
/// nominative can always be used instead of the vocative". `None`
/// therefore means "address this with the nominative", not "unknown".
///
/// The recommended forms, all from the same page:
///
/// ```
/// use interslavic_core::{Gender, noun::vocative};
///
/// // Hard masculine stems take -e, with k/g/h → č/ž/š before it.
/// assert_eq!(vocative("Ivan", Gender::Masculine).as_deref(), Some("Ivane"));
/// assert_eq!(vocative("doktor", Gender::Masculine).as_deref(), Some("doktore"));
/// assert_eq!(vocative("člověk", Gender::Masculine).as_deref(), Some("člověče"));
/// assert_eq!(vocative("Bog", Gender::Masculine).as_deref(), Some("Bože"));
///
/// // Soft masculine stems take -u; the soft sign unpacks before it.
/// assert_eq!(vocative("muž", Gender::Masculine).as_deref(), Some("mužu"));
/// assert_eq!(vocative("prijateľ", Gender::Masculine).as_deref(), Some("prijatelju"));
/// assert_eq!(vocative("koń", Gender::Masculine).as_deref(), Some("konju"));
///
/// // Masculine words on -ec take -če, not the expected -cu.
/// assert_eq!(vocative("hlåpec", Gender::Masculine).as_deref(), Some("hlåpče"));
/// assert_eq!(vocative("otec", Gender::Masculine).as_deref(), Some("otče"));
///
/// // Masculine and feminine words on -a change the ending to -o.
/// assert_eq!(vocative("sluga", Gender::Masculine).as_deref(), Some("slugo"));
/// assert_eq!(vocative("žena", Gender::Feminine).as_deref(), Some("ženo"));
/// assert_eq!(vocative("zemja", Gender::Feminine).as_deref(), Some("zemjo"));
///
/// // Elsewhere the source says to avoid the vocative.
/// assert_eq!(vocative("noč", Gender::Feminine), None);
/// assert_eq!(vocative("slovo", Gender::Neuter), None);
/// ```
pub fn vocative(word: &str, gender: Gender) -> Option<String> {
    let word = word.trim();
    if word.is_empty() {
        return None;
    }

    // "masculine and feminine words on -a change their ending to -o".
    // This is tested before gender, because it covers both.
    if let Some(stem) = word.strip_suffix('a') {
        return Some(format!("{stem}o"));
    }

    match gender {
        // "it never affects neuter nouns"; "feminine nouns on a
        // consonant ... the vocative is to be avoided".
        Gender::Neuter | Gender::Feminine => None,
        Gender::Masculine => Some(masculine_vocative(word)),
    }
}

fn masculine_vocative(word: &str) -> String {
    // "Words on -ec have the vocative ending -če instead of the expected
    // -cu": the -e- is the fleeting vowel, so the stem loses it.
    if let Some(stem) = word.strip_suffix("ec") {
        return format!("{stem}če");
    }
    // "soft masculine stems take -u". The soft sign unpacks to its
    // digraph before a vowel, the same convention the declension uses
    // (`mark_final_soft_noun_consonants`): prijateľ → prijatelju.
    if utils::ends_with_soft_consonant(word) {
        let mut stem = word.to_string();
        for (from, to) in [("ń", "nj"), ("ň", "nj"), ("ľ", "lj"), ("ĺ", "lj")] {
            if stem.ends_with(from) {
                stem = utils::replace_last_occurence(&stem, from, to);
            }
        }
        return format!("{stem}u");
    }
    // "hard masculine stems take -e", and "in the vocative, k, g and h
    // become č, ž and š before e".
    format!("{}e", crate::phono::palatalize_final(word))
}
