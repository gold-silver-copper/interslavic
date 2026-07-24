//! Microplanning: from a sequence of clauses to connected prose.
//!
//! The step above realization in the classical NLG pipeline (Reiter &
//! Dale 2000). Three components, each usable alone, all TREE-TO-TREE —
//! the string is produced exactly once, by [`crate::realize`], so the
//! never-post-process contract extends upward:
//!
//! - **Referring expressions** (frame: Krahmer & van Deemter 2012):
//!   entity-tagged NPs render in full on first mention and pronominalize
//!   afterwards; the full NP returns after an interfering same-featured
//!   entity. The salience model is deliberately just recency +
//!   interference — no scoring.
//! - **Aggregation**: adjacent clauses sharing subject entity, tense,
//!   polarity, mood, voice, prodrop, and declarative force merge into
//!   one clause with `i` VP coordination ("Krålj kupil knigų i pročital
//!   jų"-shaped). Only plain-conjunction cores merge: a clause already
//!   coordinated with `ili`/`a`/`ale` keeps its scope and stays its own
//!   sentence.
//! - **Connectives**: a small curated, dictionary-attested table
//!   (`potom` "then", `ale` "but", `zato` "therefore", `tomu` "hence",
//!   `i` "and"), rendered sentence-initially.

use crate::ast::*;
use crate::realize::{PhraseError, RealizeOpts, realize_with_lead_in};
use interslavic::{Gender, Number, Person, noun_info};
use std::collections::HashMap;

/// Sentence-initial discourse connectives — a closed table, each entry
/// a dictionary row (preposition-table discipline): `potom` adv. (row
/// 3150), `ale` conj. (row 134), `zato` adv. (row 3056), `tomu` adv.
/// (rows 4031/13020 — attested as an adverb, not only the dative of
/// `to`), `i` conj. (row 718).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Connective {
    /// "then, afterwards"
    Potom,
    /// "but"
    Ale,
    /// "therefore, for that"
    Zato,
    /// "hence, so"
    Tomu,
    /// "and"
    I,
}

impl Connective {
    pub fn word(self) -> &'static str {
        match self {
            Connective::Potom => "potom",
            Connective::Ale => "ale",
            Connective::Zato => "zato",
            Connective::Tomu => "tomu",
            Connective::I => "i",
        }
    }
}

/// One sentence of a narrative: an optional connective plus a clause.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscourseSentence {
    pub connective: Option<Connective>,
    pub clause: Clause,
}

impl DiscourseSentence {
    pub fn new(clause: Clause) -> Self {
        Self {
            connective: None,
            clause,
        }
    }
    pub fn connective(mut self, connective: Connective) -> Self {
        self.connective = Some(connective);
        self
    }
}

#[derive(Clone, Copy)]
struct EntityFeatures {
    gender: Gender,
    number: Number,
}

/// State of the referring-expression pass.
#[derive(Default)]
struct Mentions {
    /// entity id → (features, sequence number of the last mention).
    seen: HashMap<String, (EntityFeatures, usize)>,
    counter: usize,
}

impl Mentions {
    /// Decide whether this mention pronominalizes, and record it.
    fn mention(&mut self, id: &str, features: EntityFeatures) -> bool {
        self.counter += 1;
        let now = self.counter;
        let pronominalize = match self.seen.get(id) {
            None => false,
            Some((_, last)) => {
                // Interference: some OTHER entity with the same
                // gender+number mentioned since our last mention forces
                // the full NP again.
                let interfered = self.seen.iter().any(|(other, (other_features, at))| {
                    other != id
                        && other_features.gender == features.gender
                        && other_features.number == features.number
                        && at > last
                });
                !interfered
            }
        };
        self.seen.insert(id.to_string(), (features, now));
        pronominalize
    }
}

fn np_features(np: &NounPhrase) -> EntityFeatures {
    let info = noun_info(&np.head);
    // Discourse number is the SEMANTIC number of the referent.
    // Plural-only lexemes stay plural under every count; otherwise an
    // explicit count of 1 is singular and every other count denotes a
    // plurality (0 included — POLICY). Distinct from the 3sg-neuter
    // finite-verb agreement policy for gen-pl quantified subjects.
    let number = if info.plural_only {
        Number::Plural
    } else {
        match np.count {
            Some(1) | None => Number::Singular,
            Some(_) => Number::Plural,
        }
    };
    EntityFeatures {
        gender: info.gender,
        number,
    }
}

/// Replace later mentions of entity-tagged NPs with pronouns, in place.
fn pronominalize_nominal(nominal: &mut Nominal, mentions: &mut Mentions) {
    match nominal {
        Nominal::Np(np) => {
            if let Some(id) = np.entity.clone() {
                let features = np_features(np);
                // A relative contributes a proposition of its own. Only
                // replace a repeated NP when doing so cannot discard that
                // content or its nested mentions.
                if mentions.mention(&id, features) && np.relative.is_none() {
                    *nominal = Nominal::Pron {
                        person: Person::Third,
                        number: features.number,
                        gender: features.gender,
                        clitic: false,
                    };
                    return;
                }
            }
            if let Some(relative) = &mut np.relative {
                pronominalize_relative(relative, mentions);
            }
        }
        Nominal::Coord(coordination) => {
            for item in &mut coordination.items {
                pronominalize_nominal(item, mentions);
            }
        }
        _ => {}
    }
}

/// Traverse the overt arguments nested inside a relative clause in their
/// surface order. The gapped role has no nominal node to register.
fn pronominalize_relative(relative: &mut RelClause, mentions: &mut Mentions) {
    if let Some(subject) = &mut relative.subject {
        pronominalize_nominal(subject, mentions);
    }
    if let Some(object) = &mut relative.vp.object {
        pronominalize_nominal(object, mentions);
    }
    for pp in &mut relative.vp.pps {
        pronominalize_nominal(&mut pp.object, mentions);
    }
}

fn pronominalize_clause(clause: &mut Clause, mentions: &mut Mentions) {
    pronominalize_nominal(&mut clause.subject, mentions);
    match &mut clause.core {
        ClauseCore::Verbal(coordination) => {
            for verb_phrase in &mut coordination.items {
                if let Some(object) = &mut verb_phrase.object {
                    pronominalize_nominal(object, mentions);
                }
                for pp in &mut verb_phrase.pps {
                    pronominalize_nominal(&mut pp.object, mentions);
                }
            }
        }
        ClauseCore::Copular { predicate, .. } => {
            if let Predicate::Nominal(np) = predicate {
                let mut wrapped = Nominal::Np(np.clone());
                pronominalize_nominal(&mut wrapped, mentions);
                if let Nominal::Np(new_np) = wrapped {
                    *np = new_np;
                }
                // A pronominalized copular predicate stays a full NP:
                // "on jest on" is not a sentence. Mention tracking still
                // recorded it above.
            }
        }
    }
}

/// Can the first subject safely stand for the second during aggregation?
/// Exact trees are safe. For same-entity NPs, the first surface must
/// contain every modifier/proposition present on the second; a later bare
/// reference may be elided, but later-added content must keep its sentence.
fn first_subject_covers_second(first: &Nominal, second: &Nominal) -> bool {
    if first == second {
        return true;
    }
    match (first, second) {
        (Nominal::Np(a), Nominal::Np(b)) => {
            a.entity.is_some()
                && a.entity == b.entity
                && a.head == b.head
                && a.count == b.count
                && a.case_override == b.case_override
                && b.determiner
                    .as_ref()
                    .is_none_or(|det| a.determiner.as_ref() == Some(det))
                && (b.adjectives.is_empty() || a.adjectives == b.adjectives)
                && b.relative
                    .as_ref()
                    .is_none_or(|relative| a.relative.as_ref() == Some(relative))
        }
        _ => false,
    }
}

/// Can `second` be aggregated into `first` as VP coordination? Both
/// cores must carry plain-conjunction semantics — a single VP (whose
/// dormant conjunction is meaningless: "a single item realizes as
/// itself") or an `i` list. Any other conjunction has scope a flat
/// merge would corrupt: appending an asserted clause to "čitaje ili
/// piše" would demote it to an alternative. Every surface-significant
/// clause feature, `prodrop` included, must also match.
fn can_aggregate(first: &Clause, second: &DiscourseSentence) -> bool {
    let plain_and = |core: &ClauseCore| match core {
        ClauseCore::Verbal(coordination) => {
            coordination.items.len() == 1 || coordination.conjunction == Conj::I
        }
        ClauseCore::Copular { .. } => false,
    };
    second.connective.is_none()
        && first_subject_covers_second(&first.subject, &second.clause.subject)
        && first.tense == second.clause.tense
        && first.polarity == second.clause.polarity
        && first.mood == second.clause.mood
        && first.voice == second.clause.voice
        && first.prodrop == second.clause.prodrop
        && first.force == Force::Declarative
        && second.clause.force == Force::Declarative
        && first.topic.is_none()
        && first.focus.is_none()
        && second.clause.topic.is_none()
        && second.clause.focus.is_none()
        && plain_and(&first.core)
        && plain_and(&second.clause.core)
}

/// Realize a narrative: aggregation first (tree-to-tree), then the
/// referring-expression pass (tree-to-tree), then one realization per
/// sentence — every sentence, connective-bearing or not, goes through
/// the ONE realization pipeline (`realize_with_lead_in`), so the
/// caller's options (strictness, clitic style, sentence mode) apply
/// uniformly and no string is assembled here.
///
/// ```
/// use interslavic_phrase::*;
/// use interslavic_phrase::discourse::*;
///
/// let story = vec![
///     DiscourseSentence::new(
///         clause(
///             np("krålj").entity("k").det("toj"),
///             vp("kupiti").object(np("kniga").entity("b")),
///         )
///         .past(),
///     ),
///     DiscourseSentence::new(
///         clause(
///             np("krålj").entity("k"),
///             vp("pročitati").object(np("kniga").entity("b")),
///         )
///         .past(),
///     )
///     .connective(Connective::Potom),
/// ];
/// assert_eq!(
///     narrate(story, RealizeOpts::sentence()).unwrap(),
///     "Toj krålj kupil knigų. Potom on pročital jų."
/// );
/// ```
pub fn narrate(
    sentences: Vec<DiscourseSentence>,
    opts: RealizeOpts,
) -> Result<String, PhraseError> {
    // Aggregation pass (tree-to-tree).
    let mut aggregated: Vec<DiscourseSentence> = Vec::new();
    for sentence in sentences {
        if let Some(previous) = aggregated.last_mut() {
            if can_aggregate(&previous.clause, &sentence) {
                let (ClauseCore::Verbal(target), ClauseCore::Verbal(source)) =
                    (&mut previous.clause.core, sentence.clause.core)
                else {
                    unreachable!("can_aggregate checked verbal cores");
                };
                target.items.extend(source.items);
                // Both sides passed `plain_and`, so the merged list is
                // an `i` list — set it explicitly, canonicalizing away
                // a singleton's dormant conjunction.
                target.conjunction = Conj::I;
                continue;
            }
        }
        aggregated.push(sentence);
    }

    // Referring-expression pass (tree-to-tree).
    let mut mentions = Mentions::default();
    for sentence in &mut aggregated {
        pronominalize_clause(&mut sentence.clause, &mut mentions);
    }

    // Realization, one string per sentence, all through the single
    // pipeline with the caller's options intact.
    let mut out = String::new();
    for sentence in &aggregated {
        if !out.is_empty() {
            out.push(' ');
        }
        let realized = realize_with_lead_in(
            &sentence.clause,
            sentence.connective.map(Connective::word),
            opts,
        )?;
        out.push_str(&realized.text);
    }
    Ok(out)
}
