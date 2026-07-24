//! Experiment: typed Interslavic syntax trees, realized into sentences
//! through the [`interslavic`] inflection crate.
//!
//! See `PROPOSAL_SYNTAX.md` in the repository root for the design
//! rationale (GF-style abstract/concrete split; dependency-shaped trees
//! because Interslavic's hard problems are agreement, government, and
//! clitics — not word order, which is "basically free" per the steen
//! syntax page).
//!
//! Two authoring surfaces over one AST:
//!
//! - typed builders ([`clause`], [`np`], [`vp`], [`pp`], [`pron`]),
//! - the S-expression reader ([`clause_from_str`]) for data-driven
//!   templates, with a canonical [`print`]er (`parse ∘ print = id`).
//!
//! ```
//! use interslavic_phrase::*;
//!
//! let tree = clause_from_str(
//!     "(clause (np (det toj) (adj dobry) (n krålj)) \
//!              (vp (v ukrasti) (np (num 5) (adj zlåty) (n moneta))) \
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
mod realize;
mod sexpr;

pub use ast::{
    Addressee, Clause, ClauseCore, Conj, Coordination, Force, GapRole, Mood, Nominal, NounPhrase,
    Polarity, PredCase, Predicate, PrepPhrase, RelClause, Relativizer, SlotRef, TenseSpec,
    VerbPhrase, Voice, clause, coordinate, copular, name, np, pp, pron, pron_clitic, vp,
};
pub use realize::{
    CliticStyle, PhraseError, PhraseWarning, RealizeOpts, Realized, realize, realize_checked,
    realize_with_lead_in,
};
pub use sexpr::{SexprError, Value, clause_from_str, compile_clause, parse, print};
