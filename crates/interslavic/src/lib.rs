//! Interslavic inflection with embedded dictionary metadata.
//!
//! `interslavic-core` contains the dependency-free morphology rules. This crate
//! adds generated dictionary metadata for noun and verb lookup and keeps the public
//! API focused on single-form inflection.
//!
//! Integrating into a text pipeline? See `INTEGRATION.md` in the repository
//! root for the downstream guide: citation-form conventions, the byform
//! order contract, clean vs `_raw` paradigms, pronoun styles, counting,
//! and a worked runtime-assembly example.
//!
//! # Examples
//!
//! ```
//! use interslavic::*;
//!
//! assert_eq!(interslavic::noun("adept", Case::Acc, Number::Singular), "adepta");
//! assert_eq!(interslavic::noun("oko", Case::Nom, Number::Plural), "oči / očesa");
//! assert_eq!(
//!     interslavic::adj(
//!         "dobry",
//!         Case::Gen,
//!         Number::Singular,
//!         Gender::Masculine,
//!         Animacy::Animate,
//!     ),
//!     "dobrogo"
//! );
//! assert_eq!(
//!     interslavic::verb(
//!         "učiti",
//!         Person::First,
//!         Number::Singular,
//!         Gender::Feminine,
//!         Tense::Present,
//!     ),
//!     "učų"
//! );
//! ```

pub use interslavic_core::{
    AdjParadigm, Animacy, CASE_ORDER, Case, Gender, NounParadigm, Number, PerfectParts, Person,
    PronounStyle, Tense, VerbParadigm, adjective, cells, derivation, noun, orthography, paradigm,
    phono, prepositions, pronoun, types, utils, verb,
};
// The dependency-free rule engine is also re-exported, so consumers can reach
// the lower-level dictionary-less API (and the shared morphophonemics helpers)
// through this crate alone, without a separate `interslavic-core` dependency.
pub use interslavic_core::{ComplexNoun, Conjugation, HARD_CONSONANTS, J_MERGE_CHARS, VOWELS};

mod dictionary;
use dictionary::*;

#[doc(hidden)]
pub mod fingerprint;

/// Return one dictionary-backed noun form. Unknown words fall back to the
/// core rule engine's gender and animacy inference.
pub fn noun(lemma: &str, case: Case, number: Number) -> String {
    let entries = lookup_nouns_by_lemma(lemma);
    if let Some(entry) = entries.first() {
        return noun_from_entry(entry, case, number, None, None);
    }

    noun::decline_noun(lemma, case, number)
}

/// Return one noun form with explicit gender and animacy.
///
/// If the noun exists in the generated dictionary, dictionary-only metadata
/// such as fleeting-vowel additions, number restrictions, and indeclinability
/// is still used; `gender` and `animacy` override the dictionary row.
pub fn noun_with(
    lemma: &str,
    case: Case,
    number: Number,
    gender: Gender,
    animacy: Animacy,
) -> String {
    let entries = lookup_nouns_by_lemma(lemma);
    if let Some(entry) =
        select_noun_entry(lemma, entries, case, number, Some(gender), Some(animacy))
    {
        return noun_from_entry(entry, case, number, Some(gender), Some(animacy));
    }

    noun::decline_noun_explicit(
        lemma, case, number, gender, animacy, false, false, false, None,
    )
}

/// One adjective form. Adjective phrases are not declined as a unit; callers
/// should model particles/complements separately and pass only the adjective.
pub fn adj(word: &str, case: Case, number: Number, gender: Gender, animacy: Animacy) -> String {
    adjective::decline_adj(word, case, number, gender, animacy)
}

/// The whole noun paradigm — every case in both numbers — with gender and
/// animacy inferred from the dictionary (falling back to the rule engine's
/// guess for out-of-lexicon words), the counterpart of [`verb_forms()`].
/// Cells equal the corresponding [`noun()`] calls; index them with
/// [`NounParadigm::get`].
///
/// ```
/// use interslavic::{Case, Number};
/// let p = interslavic::noun_forms("žena");
/// assert_eq!(p.get(Case::Gen, Number::Singular), "ženy");
/// assert_eq!(p.get(Case::Ins, Number::Singular), "ženojų");
/// ```
pub fn noun_forms(lemma: &str) -> NounParadigm {
    let trimmed = lemma.trim();
    let (gender, animacy) = match lookup_nouns_by_lemma(trimmed).first() {
        Some(entry) => (
            dictionary_gender_to_api(entry.gender),
            if entry.animate {
                Animacy::Animate
            } else {
                Animacy::Inanimate
            },
        ),
        None => (
            noun::guess_gender(trimmed),
            if noun::noun_is_animate(trimmed) {
                Animacy::Animate
            } else {
                Animacy::Inanimate
            },
        ),
    };
    NounParadigm {
        lemma: trimmed.to_string(),
        gender,
        animacy,
        singular: CASE_ORDER
            .iter()
            .map(|&c| noun(trimmed, c, Number::Singular))
            .collect(),
        plural: CASE_ORDER
            .iter()
            .map(|&c| noun(trimmed, c, Number::Plural))
            .collect(),
    }
}

/// The whole noun paradigm with caller-supplied gender and animacy — the
/// paradigm counterpart of [`noun_with()`]. Dictionary metadata such as
/// fleeting-vowel additions and number restrictions still applies;
/// `gender`/`animacy` override the dictionary row.
pub fn noun_forms_with(lemma: &str, gender: Gender, animacy: Animacy) -> NounParadigm {
    let trimmed = lemma.trim();
    NounParadigm {
        lemma: trimmed.to_string(),
        gender,
        animacy,
        singular: CASE_ORDER
            .iter()
            .map(|&c| noun_with(trimmed, c, Number::Singular, gender, animacy))
            .collect(),
        plural: CASE_ORDER
            .iter()
            .map(|&c| noun_with(trimmed, c, Number::Plural, gender, animacy))
            .collect(),
    }
}

/// The whole adjective paradigm — every case × number × gender/animacy
/// column. Purely rule-driven (no dictionary), like [`adj()`]; cells
/// equal the corresponding `interslavic::adj` calls. Index with [`AdjParadigm::get`].
///
/// ```
/// use interslavic::{Animacy, Case, Gender, Number};
/// let p = interslavic::adj_forms("dobry");
/// assert_eq!(p.get(Case::Gen, Number::Singular, Gender::Masculine, Animacy::Animate), "dobrogo");
/// assert_eq!(p.get(Case::Nom, Number::Singular, Gender::Feminine, Animacy::Inanimate), "dobra");
/// ```
pub fn adj_forms(word: &str) -> AdjParadigm {
    let w = word.trim();
    let column = |gender: Gender, animacy: Animacy| -> [Vec<String>; 2] {
        [Number::Singular, Number::Plural].map(|number| {
            CASE_ORDER
                .iter()
                .map(|&c| adj(w, c, number, gender, animacy))
                .collect()
        })
    };
    AdjParadigm {
        lemma: w.to_string(),
        masculine_animate: column(Gender::Masculine, Animacy::Animate),
        masculine_inanimate: column(Gender::Masculine, Animacy::Inanimate),
        feminine: column(Gender::Feminine, Animacy::Inanimate),
        neuter: column(Gender::Neuter, Animacy::Inanimate),
    }
}

/// The regular derivational family of a lemma — its abstract noun, adverb,
/// verbal noun, agentive, denominal adjectives, diminutive, negation, and
/// so on, seam morphophonemics applied. Backed by the [`derivation`]
/// module; the caller filters against attestation for real words only.
///
/// ```
/// use interslavic::derivation::Pos;
/// let fam = interslavic::derive("kniga", Pos::Noun);
/// assert!(fam.iter().any(|d| d.form == "knižny")); // denominal adjective
/// assert!(fam.iter().any(|d| d.form == "knižka")); // diminutive
/// ```
pub fn derive(base: &str, pos: derivation::Pos) -> Vec<derivation::Derived> {
    derivation::derive(base, pos)
}

/// The synthetic comparative of an adjective, as `(comparative adjective,
/// comparative adverb)`. `None` for adjectives that do not gradate
/// synthetically (relational `-sky`/`-cky`, already-comparative `-ši`/`-ći`,
/// soft `-ji` possessives) — use the analytic comparative (`vyše`/`bolje`
/// followed by the positive) there. The comparative is a soft adjective,
/// so its paradigm is `interslavic::adj(comparative, …)`. Expects a positive-degree
/// qualitative adjective in flavored orthography; other input (verbs,
/// determiners) is unspecified.
///
/// ```
/// assert_eq!(interslavic::comparative("novy"), Some(("novějši".into(), "nověje".into())));
/// assert_eq!(interslavic::comparative("dobry"), Some(("lěpši".into(), "lěpje".into())));
/// assert_eq!(interslavic::comparative("russky"), None);
/// // The regular -ějši/-ěje pattern and the -ky truncation class.
/// assert_eq!(interslavic::comparative("bystry"), Some(("bystrějši".into(), "bystrěje".into())));
/// assert_eq!(interslavic::comparative("silny"), Some(("silnějši".into(), "silněje".into())));
/// assert_eq!(interslavic::comparative("slaby"), Some(("slabějši".into(), "slaběje".into())));
/// assert_eq!(interslavic::comparative("blizky"), Some(("blizši".into(), "bliže".into())));
/// ```
pub fn comparative(adj: &str) -> Option<(String, String)> {
    adjective::comparative(adj.trim())
}

/// The synthetic superlative of an adjective, as `(superlative adjective,
/// superlative adverb)` — the comparative with the `naj-` prefix. `None`
/// when the adjective does not gradate synthetically.
///
/// ```
/// assert_eq!(interslavic::superlative("novy"), Some(("najnovějši".into(), "najnověje".into())));
/// ```
pub fn superlative(adj: &str) -> Option<(String, String)> {
    adjective::superlative(adj.trim())
}

/// One pronoun form, or `None` if the lemma is not a recognized pronoun.
/// Covers the personal pronouns and reflexive `sebe` (full forms; the
/// clitic and prepositional n- series are explicit in
/// [`personal_pronoun()`]/[`reflexive_pronoun()`]), the `toj`-class
/// demonstratives, the `moj`-class possessives and interrogatives (incl.
/// `naš`/`vaš`/`čij`), `kto`/`čto` and derivatives, the `-koli`
/// indefinites, `veś`, and the adjectivally-declined determiners
/// (`ktory`, `kaky`, `samy`, …).
///
/// A personal lemma fixes the person and (3rd person) gender, overriding
/// the `gender`/`animacy` arguments; `number` still selects the column.
///
/// ```
/// use interslavic::*;
/// assert_eq!(interslavic::pronoun("toj", Case::Gen, Number::Singular, Gender::Masculine, Animacy::Inanimate), Some("togo".into()));
/// assert_eq!(interslavic::pronoun("moj", Case::Dat, Number::Singular, Gender::Neuter, Animacy::Inanimate), Some("mojemu".into()));
/// assert_eq!(interslavic::pronoun("kto", Case::Gen, Number::Singular, Gender::Masculine, Animacy::Animate), Some("kogo".into()));
/// // svoj declines via the moj-class (soft pronominal) path.
/// assert_eq!(interslavic::pronoun("svoj", Case::Gen, Number::Singular, Gender::Masculine, Animacy::Animate), Some("svojego".into()));
/// assert_eq!(interslavic::pronoun("svoj", Case::Loc, Number::Singular, Gender::Feminine, Animacy::Inanimate), Some("svojej".into()));
/// assert_eq!(interslavic::pronoun("ty", Case::Gen, Number::Singular, Gender::Masculine, Animacy::Animate), Some("tebe".into()));
/// assert_eq!(interslavic::pronoun("on", Case::Dat, Number::Singular, Gender::Masculine, Animacy::Animate), Some("jemu".into()));
/// assert_eq!(interslavic::pronoun("my", Case::Gen, Number::Plural, Gender::Masculine, Animacy::Animate), Some("nas".into()));
/// assert_eq!(interslavic::pronoun("sebe", Case::Gen, Number::Singular, Gender::Masculine, Animacy::Animate), Some("sebe".into()));
/// assert_eq!(interslavic::pronoun("stol", Case::Gen, Number::Singular, Gender::Masculine, Animacy::Inanimate), None);
/// ```
pub fn pronoun(
    lemma: &str,
    case: Case,
    number: Number,
    gender: Gender,
    animacy: Animacy,
) -> Option<String> {
    adjective::decline_pronoun(lemma.trim(), case, number, gender, animacy)
}

/// One personal-pronoun form (`ja`/`ty`/`on`/`ona`/`ono`/`my`/`vy`/`oni`/
/// `one`), or `None` when the requested cell does not exist — only for
/// [`PronounStyle::Clitic`] where no clitic is attested. `gender`
/// distinguishes forms in the third person only.
///
/// The three form series the standard distinguishes are selected by
/// `style`: full forms (`mene`, `jego`), clitics (`mę`, `go`), and the
/// prepositional n- forms of the third person (`od njego`, `s njim`;
/// non-3rd-person cells have no n- variant, so `AfterPreposition` returns
/// the full form there). Backed by the explicit tables in [`pronoun`]
/// (`interslavic_core::pronoun`), which follow the `@interslavic/utils`
/// parity reference; see that module's docs for sourcing.
///
/// ```
/// use interslavic::*;
/// use PronounStyle::*;
/// let m = Gender::Masculine;
/// assert_eq!(personal_pronoun(Person::Second, Number::Singular, m, Case::Gen, Full), Some("tebe".into()));
/// assert_eq!(personal_pronoun(Person::Second, Number::Singular, m, Case::Acc, Clitic), Some("tę".into()));
/// assert_eq!(personal_pronoun(Person::Second, Number::Singular, m, Case::Ins, Full), Some("tobojų".into()));
/// assert_eq!(personal_pronoun(Person::Third, Number::Singular, m, Case::Gen, AfterPreposition), Some("njego".into()));
/// assert_eq!(personal_pronoun(Person::Third, Number::Singular, m, Case::Nom, Clitic), None);
/// ```
pub fn personal_pronoun(
    person: Person,
    number: Number,
    gender: Gender,
    case: Case,
    style: PronounStyle,
) -> Option<String> {
    pronoun::personal_pronoun(person, number, gender, case, style)
}

/// One form of the reflexive pronoun `sebe`/`sę`/`si`, or `None` for a
/// nonexistent cell (the nominative, and `Clitic` outside the accusative
/// and dative). `AfterPreposition` returns the full form — the reflexive
/// has no n- variant (`za sebe`, `o sobě`).
///
/// ```
/// use interslavic::*;
/// use PronounStyle::*;
/// assert_eq!(reflexive_pronoun(Case::Acc, Clitic), Some("sę".into()));
/// assert_eq!(reflexive_pronoun(Case::Gen, Full), Some("sebe".into()));
/// assert_eq!(reflexive_pronoun(Case::Dat, Clitic), Some("si".into()));
/// assert_eq!(reflexive_pronoun(Case::Nom, Full), None);
/// ```
pub fn reflexive_pronoun(case: Case, style: PronounStyle) -> Option<String> {
    pronoun::reflexive_pronoun(case, style)
}

/// One numeral form, or `None` if the lemma is not a recognized numeral.
/// Covers `jedin`, the dual-remnant `dva`/`oba`/`obydva` and `tri`/`četyri`,
/// the i-stem numerals `pęť`…`desęť`, and the adjectivally-declined ordinals
/// (`pŕvy`, `drugy`, …). Cardinals return their citation form for the
/// nominative and accusative.
///
/// ```
/// use interslavic::*;
/// assert_eq!(interslavic::numeral("pęť", Case::Gen, Number::Plural, Gender::Masculine, Animacy::Inanimate), Some("pęti".into()));
/// assert_eq!(interslavic::numeral("tri", Case::Gen, Number::Plural, Gender::Masculine, Animacy::Inanimate), Some("trěh".into()));
/// assert_eq!(interslavic::numeral("pŕvy", Case::Gen, Number::Singular, Gender::Masculine, Animacy::Inanimate), Some("pŕvogo".into()));
/// // The masculine animate accusative is the genitive form; feminine
/// // and neuter animates keep the plain form, agreeing with dvě/tri.
/// assert_eq!(interslavic::numeral("dva", Case::Acc, Number::Plural, Gender::Masculine, Animacy::Animate), Some("dvoh".into()));
/// assert_eq!(interslavic::numeral("dva", Case::Acc, Number::Plural, Gender::Feminine, Animacy::Animate), Some("dvě".into()));
/// assert_eq!(interslavic::numeral("tri", Case::Acc, Number::Plural, Gender::Masculine, Animacy::Animate), Some("trěh".into()));
/// assert_eq!(interslavic::numeral("tri", Case::Acc, Number::Plural, Gender::Feminine, Animacy::Animate), Some("tri".into()));
/// // Oblique forms of the low cardinals exist across the paradigm.
/// assert_eq!(interslavic::numeral("dva", Case::Gen, Number::Plural, Gender::Masculine, Animacy::Inanimate), Some("dvoh".into()));
/// assert_eq!(interslavic::numeral("dva", Case::Dat, Number::Plural, Gender::Masculine, Animacy::Inanimate), Some("dvom".into()));
/// assert_eq!(interslavic::numeral("dva", Case::Ins, Number::Plural, Gender::Masculine, Animacy::Inanimate), Some("dvoma".into()));
/// assert_eq!(interslavic::numeral("tri", Case::Dat, Number::Plural, Gender::Masculine, Animacy::Inanimate), Some("trěm".into()));
/// assert_eq!(interslavic::numeral("pęť", Case::Ins, Number::Plural, Gender::Masculine, Animacy::Inanimate), Some("pęťjų".into()));
/// ```
pub fn numeral(
    lemma: &str,
    case: Case,
    number: Number,
    gender: Gender,
    animacy: Animacy,
) -> Option<String> {
    adjective::decline_numeral(lemma.trim(), case, number, gender, animacy)
}

/// Verb aspect as the dictionary marks it: `ipf.`, `pf.`, or the
/// biaspectual `ipf./pf.` (120 dictionary rows). Follows the reference
/// parser (`@interslavic/utils` `parsePos`), where a biaspectual row is
/// imperfective AND perfective.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Aspect {
    Ipf,
    Pf,
    Biaspectual,
}

/// Dictionary metadata for a verb lemma — the row's aspect, transitivity,
/// and reflexivity, which the conjugation API consumes internally but
/// never exposed. `transitive` is `Some(true)` for `v.tr.` rows,
/// `Some(false)` for `v.intr.`, and `None` where the row carries no
/// transitivity marker (`v.ipf.`/`v.pf.`, `v.aux.`, and plain `v.refl.`
/// rows — reflexivity is its own flag).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerbInfo {
    pub aspect: Option<Aspect>,
    pub transitive: Option<bool>,
    pub reflexive: bool,
    /// The object case the dictionary's `(+N)` government annotation
    /// marks (`dękovati (+3)` → dative), `None` for unmarked rows —
    /// callers default to the accusative. Sourced from the same
    /// annotation convention the preposition table was curated from.
    pub governs: Option<Case>,
}

/// Dictionary metadata for a verb lemma, or `None` if the lemma is not in
/// the embedded dictionary. Multi-entry lemmas follow the same
/// first-entry convention the inflection lookups use.
///
/// ```
/// use interslavic::*;
/// // ukrasti: v.tr. pf.
/// let info = verb_info("ukrasti").unwrap();
/// assert_eq!(info.aspect, Some(Aspect::Pf));
/// assert_eq!(info.transitive, Some(true));
/// assert!(!info.reflexive);
/// // hybiti: v.intr. ipf. — the dictionary answers the downstream
/// // valence-audit question: intransitive.
/// let info = verb_info("hybiti").unwrap();
/// assert_eq!(info.aspect, Some(Aspect::Ipf));
/// assert_eq!(info.transitive, Some(false));
/// // abstrahovati: v.tr. ipf./pf. — biaspectual.
/// assert_eq!(verb_info("abstrahovati").unwrap().aspect, Some(Aspect::Biaspectual));
/// // Object government from the dictionary's (+N) annotation.
/// assert_eq!(verb_info("dękovati").unwrap().governs, Some(Case::Dat));
/// assert_eq!(verb_info("izběgti").unwrap().governs, Some(Case::Gen)); // hint + (+2) coexist
/// assert_eq!(verb_info("vladati").unwrap().governs, Some(Case::Ins));
/// // Multi-entry lemmas follow the first-entry convention: izbaviti has
/// // a plain v.tr. row before its (+2) row, so its governs is None.
/// assert_eq!(verb_info("izbaviti").unwrap().governs, None);
/// assert_eq!(verb_info("ukrasti").unwrap().governs, None);
/// // Not a dictionary verb.
/// assert_eq!(verb_info("xyzzy"), None);
/// ```
pub fn verb_info(infinitive: &str) -> Option<VerbInfo> {
    let entry = lookup_verbs_by_lemma(infinitive.trim()).first()?;
    let aspect = match (entry.imperfective, entry.perfective) {
        (true, true) => Some(Aspect::Biaspectual),
        (true, false) => Some(Aspect::Ipf),
        (false, true) => Some(Aspect::Pf),
        (false, false) => None,
    };
    let transitive = if entry.transitive {
        Some(true)
    } else if entry.intransitive {
        Some(false)
    } else {
        None
    };
    let governs = match entry.governs {
        Some(2) => Some(Case::Gen),
        Some(3) => Some(Case::Dat),
        Some(4) => Some(Case::Acc),
        Some(5) => Some(Case::Ins),
        Some(7) => Some(Case::Loc),
        _ => None,
    };
    Some(VerbInfo {
        aspect,
        transitive,
        reflexive: entry.reflexive,
        governs,
    })
}

/// Whether a [`NounInfo`] came from a dictionary row or from the same
/// ending heuristics [`noun()`] applies to out-of-lexicon words.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provenance {
    Dictionary,
    Guessed,
}

/// Gender, animacy, and restriction metadata for a noun lemma. Always
/// answers: a lemma without a dictionary row reports the rule engine's
/// guess (`Provenance::Guessed`, restrictions all `false`), which is
/// exactly what [`noun()`] inflects with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NounInfo {
    pub gender: Gender,
    pub animacy: Animacy,
    pub provenance: Provenance,
    pub plural_only: bool,
    pub singular_only: bool,
    pub indeclinable: bool,
}

/// Metadata for a noun lemma — what [`noun()`] knows or guesses about
/// it. Multi-entry lemmas follow the same first-entry convention the
/// inflection lookups use.
///
/// ```
/// use interslavic::*;
/// // A dictionary row.
/// let info = noun_info("sliva");
/// assert_eq!(info.gender, Gender::Feminine);
/// assert_eq!(info.animacy, Animacy::Inanimate);
/// assert_eq!(info.provenance, Provenance::Dictionary);
/// // A plural-only row.
/// assert!(noun_info("noviny").plural_only);
/// // Out-of-lexicon: the rule engine's guess, marked as such.
/// let info = noun_info("glorbina");
/// assert_eq!(info.provenance, Provenance::Guessed);
/// assert_eq!(info.gender, Gender::Feminine); // -a heuristic
/// ```
pub fn noun_info(lemma: &str) -> NounInfo {
    let trimmed = lemma.trim();
    if let Some(entry) = lookup_nouns_by_lemma(trimmed).first() {
        return NounInfo {
            gender: dictionary_gender_to_api(entry.gender),
            animacy: if entry.animate {
                Animacy::Animate
            } else {
                Animacy::Inanimate
            },
            provenance: Provenance::Dictionary,
            plural_only: entry.plural_only,
            singular_only: entry.singular_only,
            indeclinable: entry.indeclinable,
        };
    }
    NounInfo {
        gender: noun::guess_gender(trimmed),
        animacy: if noun::noun_is_animate(trimmed) {
            Animacy::Animate
        } else {
            Animacy::Inanimate
        },
        provenance: Provenance::Guessed,
        plural_only: false,
        singular_only: false,
        indeclinable: false,
    }
}

/// The correctly-governed noun form for a counted phrase in a given
/// syntactic slot; `n` is rendered as digits by the caller ("5 zlåtnikov"
/// — spelled-out numerals are out of scope, and steen itself recommends
/// digits). Gender/animacy inflect the noun (they override the
/// dictionary, like [`noun_with()`]); plural-only lemmas are detected
/// from the dictionary — for out-of-dictionary pluralia tantum use
/// [`quantified_with_info()`].
///
/// Government rules (steen numerals page; where the sources are silent
/// the rule is documented POLICY, not a citation):
///
/// - sourced: 1 → nominative singular, 2–4 → nominative plural, 5+ →
///   genitive plural ("jedin dom, dva domy, četyri domy, pet domov");
/// - policy (sources silent on compounds): the rule is value-based —
///   steen's "all numbers higher than 4" covers 11–14, 21, and every
///   compound; the East-Slavic last-digit rule (21 + singular) is
///   deliberately not applied because no ISV source attests it. 0 takes
///   the genitive plural like 5+;
/// - policy (implied by the declension tables and steen's "Dom s tri
///   etažami"): the gen-pl override applies in Nom and Acc only; in an
///   oblique slot the noun takes the phrase case, plural (singular for
///   1) — "s pęťjų zlåtnikami", not *"s pęť zlåtnikov";
/// - policy (pan-Slavic pattern, unstated in ISV sources): the MASCULINE
///   animate accusative of 2–4 takes the genitive plural, agreeing with
///   the numeral's genitive-shaped accusative ("vidžų dvoh mųžev").
///   Feminine/neuter animates keep the plain accusative ("vidžų dvě
///   ženy") because the numeral's feminine column has no animate variant
///   (`numeral("dva", Acc, …, Feminine, Animate)` is `dvě`, per the JS
///   reference) — the noun and the numeral must compose into one
///   coherent phrase;
/// - sourced: plural-only nouns count with the collective numerals,
///   which always take the genitive plural ("dvoje dverij"), and with
///   plural jedin for 1 ("jedne dveri"); oblique collective phrases are
///   unsourced, so the plain oblique rule applies there.
///
/// ```
/// use interslavic::*;
/// let q = |n, case| quantified(n, "dom", case, Gender::Masculine, Animacy::Inanimate);
/// // Nominative: 1 → sg, 2–4 → nom pl, 0/5+/compounds → gen pl.
/// assert_eq!(q(1, Case::Nom), "dom");
/// assert_eq!(q(2, Case::Nom), "domy");
/// assert_eq!(q(4, Case::Nom), "domy");
/// assert_eq!(q(5, Case::Nom), "domov");
/// assert_eq!(q(11, Case::Nom), "domov");
/// assert_eq!(q(21, Case::Nom), "domov");
/// assert_eq!(q(100, Case::Nom), "domov");
/// // Accusative, inanimate: same pattern.
/// assert_eq!(q(2, Case::Acc), "domy");
/// assert_eq!(q(5, Case::Acc), "domov");
/// // Accusative, masculine animate: 2–4 go genitive with the numeral
/// // ("vidžų dvoh mųžev").
/// let qa = |n, case| quantified(n, "mųž", case, Gender::Masculine, Animacy::Animate);
/// assert_eq!(qa(1, Case::Acc), "mųža");
/// assert_eq!(qa(2, Case::Acc), "mųžev");
/// assert_eq!(qa(4, Case::Acc), "mųžev");
/// assert_eq!(qa(5, Case::Acc), "mųžev");
/// // Feminine animate keeps the plain accusative, agreeing with dvě
/// // ("vidžų dvě ženy") — the genitive override is masculine-only.
/// assert_eq!(quantified(2, "žena", Case::Acc, Gender::Feminine, Animacy::Animate), "ženy");
/// assert_eq!(quantified(5, "žena", Case::Acc, Gender::Feminine, Animacy::Animate), "žen");
/// // Instrumental: the gen-pl override dissolves — phrase case throughout.
/// assert_eq!(q(1, Case::Ins), "domom");
/// assert_eq!(q(2, Case::Ins), "domami");
/// assert_eq!(q(5, Case::Ins), "domami");
/// assert_eq!(q(21, Case::Ins), "domami");
/// assert_eq!(q(100, Case::Ins), "domami");
/// // Dictionary pluralia tantum: collective government, never *"dvě noviny".
/// assert_eq!(quantified(2, "noviny", Case::Nom, Gender::Feminine, Animacy::Inanimate), "novin");
/// assert_eq!(quantified(1, "noviny", Case::Nom, Gender::Feminine, Animacy::Inanimate), "noviny");
/// ```
pub fn quantified(n: u64, lemma: &str, case: Case, gender: Gender, animacy: Animacy) -> String {
    quantified_parts(n, lemma, case, gender, animacy).noun
}

/// [`quantified()`] with caller-supplied noun metadata, for lemmas the
/// dictionary does not carry. Steen's canonical pluralia-tantum example
/// `dveri` is itself not a dictionary row, so automatic detection cannot
/// see it — pass `plural_only: true` explicitly:
///
/// ```
/// use interslavic::*;
/// // A dictionary pluralia tantum through the explicit path ("dvoje ust").
/// assert_eq!(
///     quantified_with_info(2, "usta", Case::Nom, Gender::Neuter, Animacy::Inanimate, true),
///     "ust"
/// );
/// // 0 takes the genitive plural here too, like 5+.
/// assert_eq!(
///     quantified_with_info(0, "usta", Case::Nom, Gender::Neuter, Animacy::Inanimate, true),
///     "ust"
/// );
/// ```
pub fn quantified_with_info(
    n: u64,
    lemma: &str,
    case: Case,
    gender: Gender,
    animacy: Animacy,
    plural_only: bool,
) -> String {
    quantified_parts_with_info(n, lemma, case, gender, animacy, plural_only).noun
}

/// The government decision behind [`quantified()`], as data: the case and
/// number the counted slot resolves to — which is what an AGREEING
/// adjective or determiner in the phrase must be declined with — plus the
/// governed noun itself. Exists so no consumer ever re-derives the
/// government policy locally to decline a modifier; there is exactly one
/// implementation of the rules ([`quantified()`] is a thin wrapper over
/// this).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantifiedParts {
    /// The slot case after count government: `Gen` where the genitive
    /// override applies (0/5+/compounds in Nom/Acc slots; the masculine
    /// animate accusative of 2–4; the collective construction for
    /// plural-only lemmas, likewise in Nom/Acc slots only), the phrase
    /// case everywhere else. This is the
    /// SURFACE-driving case: for "vidžų dvoh velikyh mųžev" it reports
    /// `Gen`, because the agreeing adjective needs the genitive too —
    /// the syntactic slot is still an accusative, but nothing in the
    /// phrase declines with `Acc` there.
    pub case: Case,
    /// `Singular` for n == 1 (except plural-only lemmas, which are
    /// always `Plural`), `Plural` otherwise.
    pub number: Number,
    /// The governed noun form — byte-identical to what [`quantified()`]
    /// returns for the same arguments.
    pub noun: String,
}

/// The counted-phrase government decision for `n` — see
/// [`QuantifiedParts`]. Same rules and policy notes as [`quantified()`];
/// plural-only detection is automatic for dictionary lemmas
/// ([`quantified_parts_with_info()`] takes the explicit override).
///
/// ```
/// use interslavic::*;
/// let p = |n, case| quantified_parts(n, "dom", case, Gender::Masculine, Animacy::Inanimate);
/// // Nominative slot: the number and the genitive override both show.
/// assert_eq!(p(1, Case::Nom), QuantifiedParts { case: Case::Nom, number: Number::Singular, noun: "dom".into() });
/// assert_eq!(p(3, Case::Nom), QuantifiedParts { case: Case::Nom, number: Number::Plural, noun: "domy".into() });
/// assert_eq!(p(5, Case::Nom), QuantifiedParts { case: Case::Gen, number: Number::Plural, noun: "domov".into() });
/// assert_eq!(p(21, Case::Nom), QuantifiedParts { case: Case::Gen, number: Number::Plural, noun: "domov".into() });
/// // Instrumental slot: the override dissolves — adjectives agree in Ins.
/// assert_eq!(p(1, Case::Ins), QuantifiedParts { case: Case::Ins, number: Number::Singular, noun: "domom".into() });
/// assert_eq!(p(3, Case::Ins), QuantifiedParts { case: Case::Ins, number: Number::Plural, noun: "domami".into() });
/// assert_eq!(p(5, Case::Ins), QuantifiedParts { case: Case::Ins, number: Number::Plural, noun: "domami".into() });
/// assert_eq!(p(21, Case::Ins), QuantifiedParts { case: Case::Ins, number: Number::Plural, noun: "domami".into() });
/// // Masculine animate accusative of 2–4: reports Gen — "vidžų dvoh
/// // velikyh mųžev" declines its adjective in the genitive too.
/// let p = quantified_parts(2, "mųž", Case::Acc, Gender::Masculine, Animacy::Animate);
/// assert_eq!((p.case, p.number, p.noun.as_str()), (Case::Gen, Number::Plural, "mųžev"));
/// // The adjective for the phrase, driven entirely by the parts:
/// assert_eq!(adj("veliky", p.case, p.number, Gender::Masculine, Animacy::Animate), "velikyh");
/// ```
pub fn quantified_parts(
    n: u64,
    lemma: &str,
    case: Case,
    gender: Gender,
    animacy: Animacy,
) -> QuantifiedParts {
    let trimmed = lemma.trim();
    let plural_only = lookup_nouns_by_lemma(trimmed)
        .first()
        .is_some_and(|entry| entry.plural_only);
    quantified_parts_with_info(n, trimmed, case, gender, animacy, plural_only)
}

/// [`quantified_parts()`] with the explicit `plural_only` override, for
/// out-of-dictionary pluralia tantum — the parts counterpart of
/// [`quantified_with_info()`], and the single implementation of the
/// government rules that every `quantified*` entry point resolves to.
///
/// In the collective construction (plural-only lemma, direct slot,
/// n ≠ 1) the parts report `Gen`/`Plural`. Steen states the noun after a
/// collective is genitive plural and a collective subject takes a
/// 3sg-neuter verb, but is SILENT on attributive adjective agreement in
/// that construction; reporting `Gen`/`Plural` — the adjective follows
/// the noun, "dvoje velikyh dverij" — is documented policy, consistent
/// with how this crate labels every decision the sources don't make.
///
/// ```
/// use interslavic::*;
/// let p = quantified_parts_with_info(2, "usta", Case::Nom, Gender::Neuter, Animacy::Inanimate, true);
/// assert_eq!((p.case, p.number, p.noun.as_str()), (Case::Gen, Number::Plural, "ust"));
/// // n = 1 keeps the phrase case, plural ("jedne dveri" nominative).
/// let p = quantified_parts_with_info(1, "usta", Case::Nom, Gender::Neuter, Animacy::Inanimate, true);
/// assert_eq!((p.case, p.number, p.noun.as_str()), (Case::Nom, Number::Plural, "usta"));
/// ```
pub fn quantified_parts_with_info(
    n: u64,
    lemma: &str,
    case: Case,
    gender: Gender,
    animacy: Animacy,
    plural_only: bool,
) -> QuantifiedParts {
    let lemma = lemma.trim();
    let direct = matches!(case, Case::Nom | Case::Acc);
    let (slot_case, number) = if plural_only {
        // Collective government: gen pl after the collective in a direct
        // slot ("dvoje dverij", and likewise for 0); plural jedin for 1
        // ("jedne dveri"); oblique slots follow the plain oblique rule.
        if direct && n != 1 {
            (Case::Gen, Number::Plural)
        } else {
            (case, Number::Plural)
        }
    } else {
        match n {
            1 => (case, Number::Singular),
            2..=4 => {
                if case == Case::Acc && animacy == Animacy::Animate && gender == Gender::Masculine {
                    (Case::Gen, Number::Plural)
                } else {
                    (case, Number::Plural)
                }
            }
            // 0 and 5+ (value-based; compounds included — see the
            // policy notes on `quantified`).
            _ => {
                if direct {
                    (Case::Gen, Number::Plural)
                } else {
                    (case, Number::Plural)
                }
            }
        }
    };
    QuantifiedParts {
        case: slot_case,
        number,
        noun: noun_with(lemma, slot_case, number, gender, animacy),
    }
}

/// The case(s) a preposition governs, or `None` if `prep` is not a
/// recognized single-word preposition. `prep` is the flavored citation
/// form; a preposition may govern several cases (the case selects the
/// meaning). Backed by the curated [`prepositions`] table.
///
/// ```
/// use interslavic::Case;
/// assert_eq!(interslavic::preposition_cases("bez"), Some(&[Case::Gen][..]));
/// assert_eq!(interslavic::preposition_cases("na"), Some(&[Case::Acc, Case::Loc][..]));
/// assert_eq!(interslavic::preposition_cases("žaba"), None);
/// ```
pub fn preposition_cases(prep: &str) -> Option<&'static [Case]> {
    prepositions::preposition_cases(prep.trim())
}

/// The per-case senses of a preposition — each governed case paired with
/// its English gloss, because the case selects the meaning (s+Gen "off,
/// down from" is a different word than s+Ins "with"). `None` if `prep`
/// is not a recognized single-word preposition. Agrees with
/// [`preposition_cases()`] case-for-case (same curated table; glosses
/// sourced from the steen prepositions page, dictionary translations
/// where steen has no entry). A government lint can drive "multiple
/// senses exist → warn on ambiguous pairings" from this instead of a
/// hand-copied suspicious-pair set.
///
/// ```
/// use interslavic::*;
/// assert_eq!(
///     preposition_senses("s"),
///     Some(&[
///         (Case::Gen, "off, down from"),
///         (Case::Ins, "with, together with; by means of, using"),
///     ][..])
/// );
/// assert_eq!(preposition_senses("na").unwrap().len(), 2); // on(to) vs on/at
/// assert_eq!(preposition_senses("žaba"), None);
/// ```
pub fn preposition_senses(prep: &str) -> Option<&'static [(Case, &'static str)]> {
    prepositions::preposition_senses(prep.trim())
}

/// One finite verb form. Present, imperfect, future, perfect, pluperfect,
/// and conditional are supported; imperative and participial/gerund forms
/// are available through `verb_forms`.
pub fn verb(word: &str, person: Person, number: Number, gender: Gender, tense: Tense) -> String {
    let trimmed = word.trim();
    let entries = lookup_verbs_by_lemma(trimmed);
    if let Some(entry) = entries.first() {
        return verb::conjugate_verb_with_options(
            entry.lemma,
            entry.addition,
            person,
            number,
            gender,
            tense,
            entry.transitive,
            entry.imperfective,
        );
    }

    verb::conjugate_verb(trimmed, person, number, gender, tense)
}

/// One finite verb form, or `None` when an infinitive stem cannot be
/// derived from `word` (roughly, it does not end in `-ti`/`-t`/`-ť`). The
/// checked counterpart of [`verb()`]: it reports clearly non-verb input
/// as `None` instead of degrading to a best-effort string, so callers need
/// no `catch_unwind` guard. The check is mechanical, not lexical — a `-t`
/// noun shaped like an infinitive still yields `Some`.
///
/// ```
/// use interslavic::*;
/// assert!(interslavic::try_verb("pisati", Person::First, Number::Singular, Gender::Masculine, Tense::Present).is_some());
/// assert_eq!(interslavic::try_verb("xyz", Person::First, Number::Singular, Gender::Masculine, Tense::Present), None);
/// ```
pub fn try_verb(
    word: &str,
    person: Person,
    number: Number,
    gender: Gender,
    tense: Tense,
) -> Option<String> {
    let trimmed = word.trim();
    let entries = lookup_verbs_by_lemma(trimmed);
    if let Some(entry) = entries.first() {
        return verb::conjugate_verb_checked(
            entry.lemma,
            entry.addition,
            person,
            number,
            gender,
            tense,
            entry.transitive,
            entry.imperfective,
        );
    }
    verb::conjugate_verb_checked(trimmed, "", person, number, gender, tense, true, true)
}

/// One finite verb form with an explicit dictionary present-stem hint.
///
/// This is intended for typed dictionary rows that have multiple entries for
/// the same lemma. It does not parse arbitrary phrase strings.
pub fn verb_with_present_hint(
    word: &str,
    present_hint: &str,
    person: Person,
    number: Number,
    gender: Gender,
    tense: Tense,
) -> String {
    verb::conjugate_verb_with_present_hint(word.trim(), present_hint, person, number, gender, tense)
}

/// The l-participle of an infinitive — the gender/number-marked past
/// active form the compound tenses are built on, useful on its own for
/// fixed-gender past subjects ("strěla tę ubila"). Suppletion is handled
/// (`idti` → `šėl`/`šla`/…), and the form always agrees with the
/// participle inside [`verb_forms()`]'s perfect/pluperfect/conditional.
///
/// ```
/// use interslavic::*;
/// // A regular verb.
/// assert_eq!(l_participle("pisati", Gender::Masculine, Number::Singular), "pisal");
/// assert_eq!(l_participle("pisati", Gender::Feminine, Number::Singular), "pisala");
/// assert_eq!(l_participle("pisati", Gender::Neuter, Number::Singular), "pisalo");
/// assert_eq!(l_participle("pisati", Gender::Masculine, Number::Plural), "pisali");
/// // The idti suppletion.
/// assert_eq!(l_participle("idti", Gender::Masculine, Number::Singular), "šėl");
/// assert_eq!(l_participle("idti", Gender::Feminine, Number::Singular), "šla");
/// assert_eq!(l_participle("idti", Gender::Neuter, Number::Singular), "šlo");
/// assert_eq!(l_participle("idti", Gender::Masculine, Number::Plural), "šli");
/// // A prefixed perfective.
/// assert_eq!(l_participle("ubiti", Gender::Masculine, Number::Singular), "ubil");
/// assert_eq!(l_participle("ubiti", Gender::Feminine, Number::Singular), "ubila");
/// // A d/t-stem -sti verb: the dictionary present-hint recovers the
/// // dental stem, in agreement with the compound tenses.
/// assert_eq!(l_participle("ukrasti", Gender::Feminine, Number::Singular), "ukradla");
/// assert_eq!(l_participle("vesti", Gender::Masculine, Number::Plural), "vedli");
/// ```
pub fn l_participle(infinitive: &str, gender: Gender, number: Number) -> String {
    let trimmed = infinitive.trim();
    let entries = lookup_verbs_by_lemma(trimmed);
    if let Some(entry) = entries.first() {
        return verb::l_participle_with_hint(entry.lemma, entry.addition, gender, number);
    }
    verb::l_participle(trimmed, gender, number)
}

/// The perfect tense as structured parts: the auxiliary where the
/// standard requires one (`None` in the third person, which normally
/// drops it — "ona ukradla", not "ona jest ukradla"), and the correctly
/// gendered l-participle with no bracket conventions. Built on the same
/// stem context as [`l_participle()`] and [`verb_forms()`]'s compound
/// cells, so the three can never disagree. A caller that wants the
/// emphatic third-person auxiliary adds `je`/`jest` or `sųt` itself.
///
/// ```
/// use interslavic::*;
/// let p = perfect_parts("pisati", Person::First, Number::Singular, Gender::Masculine);
/// assert_eq!(p.auxiliary.as_deref(), Some("jesm"));
/// assert_eq!(p.participle, "pisal");
/// let p = perfect_parts("pisati", Person::First, Number::Singular, Gender::Feminine);
/// assert_eq!(p.participle, "pisala");
///
/// // 3rd person: no auxiliary, all genders.
/// let p = perfect_parts("pisati", Person::Third, Number::Singular, Gender::Feminine);
/// assert_eq!(p.auxiliary, None);
/// assert_eq!(p.participle, "pisala");
/// assert_eq!(perfect_parts("pisati", Person::Third, Number::Singular, Gender::Neuter).participle, "pisalo");
/// let p = perfect_parts("pisati", Person::Third, Number::Plural, Gender::Masculine);
/// assert_eq!((p.auxiliary, p.participle), (None, "pisali".into()));
///
/// // The d/t-stem -sti class resolves through the dictionary hint.
/// let p = perfect_parts("ukrasti", Person::Third, Number::Singular, Gender::Feminine);
/// assert_eq!((p.auxiliary, p.participle), (None, "ukradla".into()));
/// ```
pub fn perfect_parts(
    infinitive: &str,
    person: Person,
    number: Number,
    gender: Gender,
) -> PerfectParts {
    let trimmed = infinitive.trim();
    let entries = lookup_verbs_by_lemma(trimmed);
    if let Some(entry) = entries.first() {
        return verb::perfect_parts_with_hint(entry.lemma, entry.addition, person, number, gender);
    }
    verb::perfect_parts_with_hint(trimmed, "", person, number, gender)
}

/// Flatten the builders' internal intervocalic-j marker `ĵ` to its
/// surface spelling `j` in every cell of a paradigm. Only the imperative
/// and present-active-participle cells ever carry the marker; the other
/// fields pass through untouched. Variant structure, gender-form
/// conventions, and citation accents are all preserved — they are
/// documented cell shape, and `cells::variants` remains the flattening
/// step for them.
fn surface_paradigm(mut p: VerbParadigm) -> VerbParadigm {
    fn clean(s: &mut String) {
        if s.contains('ĵ') {
            *s = s.replace('ĵ', "j");
        }
    }
    clean(&mut p.infinitive);
    for cells in [
        &mut p.present,
        &mut p.imperfect,
        &mut p.perfect,
        &mut p.pluperfect,
        &mut p.future,
        &mut p.conditional,
        &mut p.imperative,
    ] {
        cells.iter_mut().for_each(clean);
    }
    for cell in [&mut p.prap, &mut p.prpp, &mut p.pfpp] {
        if let Some(s) = cell.as_mut() {
            clean(s);
        }
    }
    clean(&mut p.pfap);
    clean(&mut p.gerund);
    p
}

/// Full verb paradigm with dictionary metadata when available.
///
/// Cells are surface-ready: the internal `ĵ` marker the builders use is
/// flattened to `j` (`imperative[0]` of *počivati* is `"počivaj"`, its
/// `prap` head `"počivajųćí"`). [`verb_forms_raw()`] returns the
/// marker-carrying cells for integrations that need them (the parity
/// harness compares those, because the JS reference emits the marker
/// too). Parenthesized variant conventions are still present — flatten
/// them with [`cells::variants`].
///
/// The imperative and gerund cells are populated across the conjugation
/// classes:
///
/// ```
/// use interslavic::*;
/// // -iti class.
/// let p = verb_forms("učiti");
/// assert_eq!(p.imperative, vec!["uči", "učimo", "učite"]);
/// assert_eq!(p.gerund, "učeńje");
/// // -ovati class.
/// let p = verb_forms("kupovati");
/// assert_eq!(p.imperative[0], "kupuj");
/// assert_eq!(p.gerund, "kupovańje");
/// // -ati class (with the consonant mutation of the present stem).
/// let p = verb_forms("pisati");
/// assert_eq!(p.imperative[0], "piši");
/// assert_eq!(p.gerund, "pisańje");
/// // -ati class without mutation: surface j, not the internal ĵ.
/// let p = verb_forms("počivati");
/// assert_eq!(p.imperative[0], "počivaj");
/// assert_eq!(p.prap.as_deref(), Some("počivajųćí (počivajųćá, počivajųćé)"));
/// ```
pub fn verb_forms(word: &str) -> VerbParadigm {
    surface_paradigm(verb_forms_raw(word))
}

/// [`verb_forms()`] without the surface cleaning: cells carry the
/// builders' internal `ĵ` marker exactly as the JS parity reference
/// emits it (`"počivaĵ"`). This is the escape hatch for integrations
/// that normalize cells themselves (via [`cells::variants`], which
/// flattens the marker along with the other conventions) or that compare
/// against `@interslavic/utils` byte-for-byte.
///
/// ```
/// use interslavic::*;
/// assert_eq!(verb_forms_raw("počivati").imperative[0], "počivaĵ");
/// assert_eq!(cells::variants(&verb_forms_raw("počivati").imperative[0]), ["počivaj"]);
/// ```
pub fn verb_forms_raw(word: &str) -> VerbParadigm {
    let trimmed = word.trim();
    let entries = lookup_verbs_by_lemma(trimmed);
    if let Some(entry) = entries.first() {
        return verb::verb_paradigm_with_options(
            entry.lemma,
            entry.addition,
            entry.transitive,
            entry.imperfective,
        );
    }
    verb::verb_paradigm_with_options(trimmed, "", true, true)
}

/// The full verb paradigm, or `None` when an infinitive stem cannot be
/// derived from `word` — the checked counterpart of [`verb_forms()`].
/// The check is mechanical (infinitive shape), not a lexical verb lookup.
/// Cells are surface-ready like [`verb_forms()`]'s; the raw counterpart
/// is [`try_verb_forms_raw()`].
///
/// ```
/// use interslavic::*;
/// assert!(interslavic::try_verb_forms("pisati").is_some());
/// assert_eq!(interslavic::try_verb_forms("xyz"), None);
/// ```
pub fn try_verb_forms(word: &str) -> Option<VerbParadigm> {
    try_verb_forms_raw(word).map(surface_paradigm)
}

/// [`try_verb_forms()`] without the surface cleaning — see
/// [`verb_forms_raw()`].
pub fn try_verb_forms_raw(word: &str) -> Option<VerbParadigm> {
    let trimmed = word.trim();
    let entries = lookup_verbs_by_lemma(trimmed);
    if let Some(entry) = entries.first() {
        return verb::verb_paradigm_checked(
            entry.lemma,
            entry.addition,
            entry.transitive,
            entry.imperfective,
        );
    }
    verb::verb_paradigm_checked(trimmed, "", true, true)
}

/// The declinable adjective lemma inside a paradigm participle cell: the
/// masculine head of `"osvětljený (osvětljená, osvětljenó)"` with the
/// acute-marked long ending flattened to the plain `-y`/`-i` the
/// adjective engine declines.
fn participle_lemma(participle: &str) -> String {
    let head = participle.split(" (").next().unwrap_or(participle).trim();
    head.chars()
        .map(|c| match c {
            'ý' => 'y',
            'í' => 'i',
            c => c,
        })
        .collect()
}

/// One declined form of the past passive participle — the participle from
/// [`verb_forms()`]'s `pfpp`, declined as an adjective ("komnata jest
/// osvětljena", "to jest opoznano"). `None` when the verb has no passive
/// participle (intransitives). Covers the `-ny`, `-ty`, and iotated
/// `-jeny` shapes; equivalent to feeding the `pfpp` lemma through
/// [`adj()`], with the `Option` handling in one place.
///
/// ```
/// use interslavic::*;
/// // Iotated -jeny stem: osvětliti -> osvětljeny.
/// assert_eq!(passive_participle("osvětliti", Case::Nom, Number::Singular, Gender::Feminine, Animacy::Inanimate), Some("osvětljena".into()));
/// // -ny shape: opoznati -> opoznany.
/// assert_eq!(passive_participle("opoznati", Case::Nom, Number::Singular, Gender::Neuter, Animacy::Inanimate), Some("opoznano".into()));
/// // -ty shape, declined obliquely: ubiti -> ubity -> ubitogo.
/// assert_eq!(passive_participle("ubiti", Case::Gen, Number::Singular, Gender::Masculine, Animacy::Animate), Some("ubitogo".into()));
/// // Intransitive verbs have no passive participle.
/// assert_eq!(passive_participle("idti", Case::Nom, Number::Singular, Gender::Masculine, Animacy::Animate), None);
/// ```
pub fn passive_participle(
    infinitive: &str,
    case: Case,
    number: Number,
    gender: Gender,
    animacy: Animacy,
) -> Option<String> {
    let pfpp = verb_forms(infinitive).pfpp?;
    Some(adj(&participle_lemma(&pfpp), case, number, gender, animacy))
}

/// One declined form of the present active participle — the `prap` from
/// [`verb_forms()`] declined as a (soft) adjective ("pišųća žena").
/// `None` when the verb has no present active participle (perfectives).
///
/// ```
/// use interslavic::*;
/// assert_eq!(active_participle("pisati", Case::Nom, Number::Singular, Gender::Feminine, Animacy::Inanimate), Some("pišųća".into()));
/// assert_eq!(active_participle("pisati", Case::Gen, Number::Singular, Gender::Masculine, Animacy::Animate), Some("pišųćego".into()));
/// // -aj stems come out with the surface j, not the internal ĵ marker.
/// assert_eq!(active_participle("dělati", Case::Nom, Number::Singular, Gender::Feminine, Animacy::Inanimate), Some("dělajųća".into()));
/// // Perfective verbs have no present active participle.
/// assert_eq!(active_participle("ubiti", Case::Nom, Number::Singular, Gender::Masculine, Animacy::Animate), None);
/// ```
pub fn active_participle(
    infinitive: &str,
    case: Case,
    number: Number,
    gender: Gender,
    animacy: Animacy,
) -> Option<String> {
    let prap = verb_forms(infinitive).prap?;
    Some(adj(&participle_lemma(&prap), case, number, gender, animacy))
}

/// Full verb paradigm with explicit dictionary metadata. Cells are RAW
/// (marker-carrying) like [`verb_forms_raw()`]'s: this is the typed
/// dictionary-integration entry point, and the parity harness compares
/// its output byte-for-byte against `@interslavic/utils`, which emits
/// the internal `ĵ` marker too. Flatten with [`cells::variants`] for
/// display.
pub fn verb_forms_with_metadata(
    word: &str,
    present_hint: &str,
    transitive: bool,
    imperfective: bool,
) -> VerbParadigm {
    verb::verb_paradigm_with_options(word.trim(), present_hint, transitive, imperfective)
}

fn select_noun_entry<'a>(
    lemma: &str,
    entries: &'a [DictionaryEntry],
    case: Case,
    number: Number,
    gender: Option<Gender>,
    animacy: Option<Animacy>,
) -> Option<&'a DictionaryEntry> {
    entries.iter().max_by_key(|entry| {
        let mut score = 0;
        if entry.lemma == lemma {
            score += 16;
        }
        if gender.is_some_and(|wanted| dictionary_gender_to_api(entry.gender) == wanted) {
            score += 8;
        }
        if animacy.is_some_and(|wanted| {
            let entry_animacy = if entry.animate {
                Animacy::Animate
            } else {
                Animacy::Inanimate
            };
            entry_animacy == wanted
        }) {
            score += 4;
        }
        if number == Number::Singular && !entry.plural_only {
            score += 2;
        }
        if number == Number::Plural && !entry.singular_only {
            score += 2;
        }
        if case == Case::Nom {
            score += 1;
        }
        if !entry.indeclinable {
            score += 1;
        }
        score
    })
}

fn noun_from_entry(
    entry: &DictionaryEntry,
    case: Case,
    number: Number,
    gender_override: Option<Gender>,
    animacy_override: Option<Animacy>,
) -> String {
    let gender = gender_override.unwrap_or_else(|| dictionary_gender_to_api(entry.gender));
    let animacy = animacy_override.unwrap_or(if entry.animate {
        Animacy::Animate
    } else {
        Animacy::Inanimate
    });

    noun::decline_noun_explicit(
        entry.lemma,
        case,
        number,
        gender,
        animacy,
        entry.plural_only,
        entry.singular_only,
        entry.indeclinable,
        (!entry.addition.is_empty()).then_some(entry.addition),
    )
}

fn dictionary_gender_to_api(gender: DictionaryGender) -> Gender {
    match gender {
        DictionaryGender::Masculine | DictionaryGender::MasculineFeminine => Gender::Masculine,
        DictionaryGender::Feminine => Gender::Feminine,
        DictionaryGender::Neuter => Gender::Neuter,
    }
}

#[cfg(test)]
mod l_participle_consistency {
    use super::*;

    /// The invariant: for EVERY verb lemma in the embedded dictionary,
    /// the standalone [`l_participle`] agrees with the l-participle
    /// inside [`verb_forms`]'s compound cells in all six gender/number
    /// slots. The paradigm path is the parity-verified one, so any
    /// divergence is a bug by definition. Phrase lemmas are skipped:
    /// their paradigm cells echo the raw phrase (phrases are not part of
    /// the typed lemma API).
    #[test]
    fn every_dictionary_lemma_agrees_with_the_paradigm() {
        let mut compared = 0usize;
        let mut divergent: Vec<String> = Vec::new();
        for lemma in dictionary::all_verb_lemmas() {
            if lemma.split_whitespace().nth(1).is_some() {
                continue;
            }
            let paradigm = verb_forms(lemma);
            // perfect[2..5] are the 3sg m/f/n cells "(je) <participle>",
            // perfect[7] the 3pl "(sųt) <participle>i"; variants()[0] is
            // the auxiliary-less form.
            let expectations = [
                (Gender::Masculine, Number::Singular, &paradigm.perfect[2]),
                (Gender::Feminine, Number::Singular, &paradigm.perfect[3]),
                (Gender::Neuter, Number::Singular, &paradigm.perfect[4]),
                (Gender::Masculine, Number::Plural, &paradigm.perfect[7]),
                (Gender::Feminine, Number::Plural, &paradigm.perfect[7]),
                (Gender::Neuter, Number::Plural, &paradigm.perfect[7]),
            ];
            for (gender, number, cell) in expectations {
                let expected = cells::variants(cell)
                    .into_iter()
                    .next()
                    .expect("variants is never empty");
                let actual = l_participle(lemma, gender, number);
                compared += 1;
                if actual != expected {
                    divergent.push(format!(
                        "{lemma} {gender:?}/{number:?}: l_participle={actual} paradigm={expected}"
                    ));
                }
            }
        }
        assert!(compared > 10_000, "sweep unexpectedly small: {compared}");
        assert!(
            divergent.is_empty(),
            "{} of {} cells diverge from the paradigm:\n{}",
            divergent.len(),
            compared,
            divergent.join("\n")
        );
    }
}
