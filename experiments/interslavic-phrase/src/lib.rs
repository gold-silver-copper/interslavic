//! Experiment: typed Interslavic syntax trees, realized into sentences
//! through the [`interslavic`] inflection crate.
//!
//! Two authoring surfaces produce the same raw AST:
//!
//! - typed builders ([`clause`], [`np`], [`vp`], [`pp`],
//!   [`participial_adjunct`], [`pron`]),
//! - the S-expression reader ([`clause_from_str`]) for data-driven
//!   templates, with a canonical [`print()`]er.
//!
//! Both enter one staged pipeline:
//!
//! `RawClause` → [`validate`] → [`ValidatedClause`] → grammar
//! resolution → hierarchical surface plans → one final stringification.
//!
//! Case belongs to grammatical-role edges such as [`Recipient`],
//! [`Complement`], [`PrepPhrase`], and [`Oblique`], never to
//! [`NounPhrase`]. Nested
//! relative clauses and coordinated verb phrases remain distinct clitic
//! domains until they place their own clusters. See `ARCHITECTURE.md`
//! beside the crate README for the full ownership and serialization
//! contracts.
//!
//! ```
//! use interslavic_phrase::*;
//!
//! let tree = clause_from_str(
//!     "(clause (np (det toj) (adj dobry) (n krålj)) \
//!              (vp (v ukrasti) \
//!                  (object (np (num 5) (adj zlåty) (n moneta)))) \
//!              :tense past)",
//! )
//! .unwrap();
//! assert_eq!(
//!     realize(&tree, RealizeOpts::sentence()).unwrap(),
//!     "Toj dobry krålj ukradl 5 zlåtyh monet."
//! );
//! ```
//!
//! Deferred by design (do not look for them here): genitive of
//! negation, spelled-out numerals, the `iže` relativizer, the passive
//! imperative, preposition-phrasal verb government ("bazovati na").

mod ast;
pub mod discourse;
mod plan;
mod profile;
mod realize;
mod resolve;
mod sexpr;
mod validate;

pub use ast::{
    Addressee, BuildError, Clause, ClauseCore, Complement, Conj, Coordination, Force, GapRole,
    Mood, Nominal, NounPhrase, Oblique, ParticipialAdjunct, Polarity, PredCase, Predicate,
    PrepPhrase, Recipient, ReferentialForm, RelClause, Relativizer, SlotRef, TenseSpec, VerbPhrase,
    Voice, WhFront, clause, coordinate, copular, name, np, participial_adjunct, pp, pron,
    pron_clitic, vp,
};
/// The unvalidated authoring tree produced by builders and the
/// S-expression compiler.
pub type RawClause = Clause;
pub use realize::{
    CliticStyle, PhraseError, PhraseWarning, RealizeOpts, Realized, realize, realize_checked,
    realize_validated_checked, realize_with_lead_in,
};
pub use resolve::{ResolutionError, ResolutionErrorKind, ResolutionErrors};
pub use sexpr::{
    SexprError, Value, clause_from_str, compile_clause, parse, print, print_validated,
};
pub use validate::{
    AstPath, MAX_STRUCTURE_DEPTH, ValidatedClause, ValidationError, ValidationErrorKind,
    ValidationErrors, validate,
};
