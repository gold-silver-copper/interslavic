//! Dependency-free Interslavic morphology rules.
//!
//! This crate contains the algorithmic core used by the table-backed
//! `interslavic` facade. It does not read dictionary TSV data and has no
//! third-party dependencies.
//!
//! ```
//! use interslavic_core::{noun, verb, Case, Gender, Number, Person, Tense};
//!
//! assert_eq!(noun::decline_noun("suma", Case::Acc, Number::Singular), "sumų");
//! assert_eq!(
//!     verb::conjugate_verb(
//!         "učiti",
//!         Person::First,
//!         Number::Singular,
//!         Gender::Feminine,
//!         Tense::Present,
//!     ),
//!     "učų"
//! );
//! ```

mod case_endings;
mod known_nouns;

pub mod adjective;
pub mod cells;
pub mod derivation;
pub mod noun;
pub mod orthography;
pub mod paradigm;
pub mod phono;
pub mod prepositions;
pub mod pronoun;
pub mod types;
pub mod utils;
pub mod verb;

pub use paradigm::{AdjParadigm, CASE_ORDER, NounParadigm, VerbParadigm};
pub use types::{
    Animacy, Case, ComplexNoun, Conjugation, Gender, Number, Person, PronounStyle, Tense,
};
pub use verb::{ConditionalParts, PerfectParts};

pub const VOWELS: &[char] = &[
    'a', 'e', 'i', 'í', 'ó', 'o', 'u', 'å', 'ą', 'ę', 'ė', 'é', 'ȯ', 'ų', 'ů', 'ú', 'ý', 'y', 'ě',
    'A', 'E', 'I', 'O', 'U', 'á',
];

pub const HARD_CONSONANTS: &[char] = &[
    'p', 'b', 'f', 'v', 'm', 's', 'z', 't', 'd', 'r', 'n', 'l', 'k', 'g', 'h',
];

pub const J_MERGE_CHARS: &[&str] = &[
    "st", "zd", "sk", "zg", "s", "z", "t", "d", "k", "g", "h", "č", "š", "ž", "ć", "đ", "j",
];
