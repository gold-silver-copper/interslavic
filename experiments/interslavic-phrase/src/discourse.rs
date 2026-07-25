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
//!   interference — no scoring. Recency follows typed constituent order
//!   after topic/focus movement and before clitic-cluster placement; token
//!   displacement inside a clitic domain does not redefine discourse
//!   salience.
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
use crate::profile::nominal_profile;
use crate::realize::{PhraseError, PhraseWarning, RealizeOpts, realize_validated_with_lead_in};
use crate::resolve::ResolutionErrors;
use crate::validate::{AstPath, ValidationErrors, validate};
use interslavic::{Gender, Number};
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
    let profile = nominal_profile(&Nominal::Np(np.clone()));
    EntityFeatures {
        gender: profile.gender,
        number: profile.referent_number,
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
                if mentions.mention(&id, features)
                    && np.relative.is_none()
                    && np.referential == ReferentialForm::Full
                {
                    // Change only the requested referential form. The
                    // lexical NP, entity identity, and the grammatical
                    // role edge that owns case remain intact. An explicit
                    // caller request for Pronoun/Clitic remains stronger
                    // than the planner's default choice.
                    np.referential = ReferentialForm::Pronoun;
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
    if let Some(recipient) = &mut relative.vp.recipient {
        pronominalize_nominal(&mut recipient.nominal, mentions);
    }
    if let Some(object) = &mut relative.vp.object {
        pronominalize_nominal(&mut object.nominal, mentions);
    }
    for pp in &mut relative.vp.pps {
        pronominalize_nominal(&mut pp.object, mentions);
    }
    for oblique in &mut relative.vp.obliques {
        pronominalize_nominal(&mut oblique.nominal, mentions);
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MentionSlot {
    InitialPpObject { adjunct: usize, pp: usize },
    Subject,
    Recipient(usize),
    Object(usize),
    PpObject { vp: usize, pp: usize },
    Oblique { vp: usize, oblique: usize },
    Predicate,
}

fn pronominalize_clause(clause: &mut Clause, mentions: &mut Mentions) {
    // Build the nominal traversal from typed slots in the same order as
    // realization. Salience follows what the reader encounters, not
    // storage order in the AST.
    let mut slots = Vec::new();
    for (adjunct, participle) in clause.initial_participles.iter().enumerate() {
        for pp in 0..participle.pps.len() {
            slots.push(MentionSlot::InitialPpObject { adjunct, pp });
        }
    }
    if !clause.prodrop && !matches!(clause.force, Force::Imperative(_)) {
        slots.push(MentionSlot::Subject);
    }
    match &clause.core {
        ClauseCore::Verbal(coordination) => {
            for (vp, verb_phrase) in coordination.items.iter().enumerate() {
                if verb_phrase.recipient.is_some() {
                    slots.push(MentionSlot::Recipient(vp));
                }
                if verb_phrase.object.is_some() {
                    slots.push(MentionSlot::Object(vp));
                }
                for pp in 0..verb_phrase.pps.len() {
                    slots.push(MentionSlot::PpObject { vp, pp });
                }
                for oblique in 0..verb_phrase.obliques.len() {
                    slots.push(MentionSlot::Oblique { vp, oblique });
                }
            }
        }
        ClauseCore::Copular { predicate, .. } => {
            if matches!(predicate, Predicate::Nominal(_)) {
                slots.push(MentionSlot::Predicate);
            }
        }
    }

    let object_slot = match &clause.core {
        ClauseCore::Verbal(_) => information_object_index(&clause.core).map(MentionSlot::Object),
        ClauseCore::Copular { .. } => Some(MentionSlot::Predicate),
    };
    let recipient_slot = match &clause.core {
        ClauseCore::Verbal(_) => {
            information_recipient_index(&clause.core).map(MentionSlot::Recipient)
        }
        ClauseCore::Copular { .. } => None,
    };
    let slot_of = |slot| match slot {
        SlotRef::Subject => Some(MentionSlot::Subject),
        SlotRef::Recipient => recipient_slot,
        SlotRef::Object => object_slot,
    };
    let take = |slots: &mut Vec<MentionSlot>, slot: MentionSlot| {
        slots
            .iter()
            .position(|candidate| *candidate == slot)
            .map(|index| slots.remove(index))
    };
    let mut initial = Vec::new();
    let mut index = 0;
    while index < slots.len() {
        if matches!(slots[index], MentionSlot::InitialPpObject { .. }) {
            initial.push(slots.remove(index));
        } else {
            index += 1;
        }
    }

    if let Some(WhFront::Slot(front)) = clause.wh.as_ref()
        && let Some(slot) = slot_of(*front).and_then(|slot| take(&mut slots, slot))
    {
        slots.insert(0, slot);
    } else if clause.force == Force::LiQuestion {
        let mut ordered = Vec::new();
        if let Some(topic) = clause.topic.and_then(slot_of) {
            ordered.extend(take(&mut slots, topic));
        }
        if let Some(focus) = clause.focus.and_then(slot_of) {
            ordered.extend(take(&mut slots, focus));
        }
        ordered.extend(take(&mut slots, MentionSlot::Subject));
        ordered.extend(take(&mut slots, MentionSlot::Recipient(0)));
        ordered.extend(take(&mut slots, MentionSlot::Object(0)));
        ordered.append(&mut slots);
        slots = ordered;
    } else {
        if let Some(topic) = clause.topic.and_then(slot_of)
            && let Some(slot) = take(&mut slots, topic)
        {
            slots.insert(0, slot);
        }
        if let Some(focus) = clause.focus.and_then(slot_of)
            && let Some(slot) = take(&mut slots, focus)
        {
            slots.push(slot);
        }
    }
    initial.append(&mut slots);
    slots = initial;

    for slot in slots {
        match slot {
            MentionSlot::InitialPpObject { adjunct, pp } => {
                pronominalize_nominal(
                    &mut clause.initial_participles[adjunct].pps[pp].object,
                    mentions,
                );
            }
            MentionSlot::Subject => pronominalize_nominal(&mut clause.subject, mentions),
            MentionSlot::Recipient(vp) => {
                let ClauseCore::Verbal(coordination) = &mut clause.core else {
                    unreachable!("recipient mention slots belong to verbal cores");
                };
                let recipient = coordination.items[vp]
                    .recipient
                    .as_mut()
                    .expect("slot was derived from an existing recipient");
                pronominalize_nominal(&mut recipient.nominal, mentions);
            }
            MentionSlot::Object(vp) => {
                let ClauseCore::Verbal(coordination) = &mut clause.core else {
                    unreachable!("object mention slots belong to verbal cores");
                };
                let object = coordination.items[vp]
                    .object
                    .as_mut()
                    .expect("slot was derived from an existing object");
                pronominalize_nominal(&mut object.nominal, mentions);
            }
            MentionSlot::PpObject { vp, pp } => {
                let ClauseCore::Verbal(coordination) = &mut clause.core else {
                    unreachable!("PP mention slots belong to verbal cores");
                };
                pronominalize_nominal(&mut coordination.items[vp].pps[pp].object, mentions);
            }
            MentionSlot::Oblique { vp, oblique } => {
                let ClauseCore::Verbal(coordination) = &mut clause.core else {
                    unreachable!("oblique mention slots belong to verbal cores");
                };
                pronominalize_nominal(
                    &mut coordination.items[vp].obliques[oblique].nominal,
                    mentions,
                );
            }
            MentionSlot::Predicate => {
                let ClauseCore::Copular {
                    predicate: Predicate::Nominal(np),
                    ..
                } = &mut clause.core
                else {
                    unreachable!("predicate mention slot was derived from a nominal predicate");
                };
                let requested = np.referential;
                let mut wrapped = Nominal::Np(np.clone());
                pronominalize_nominal(&mut wrapped, mentions);
                let Nominal::Np(mut new_np) = wrapped else {
                    unreachable!("the wrapper remains a noun phrase");
                };
                // A nominal predicate is not an argumental referring
                // expression: changing it to a pronoun produces
                // "on jest on"-shaped output. Track its mentions and
                // nested propositions, but preserve the caller's
                // original referential request.
                new_np.referential = requested;
                *np = new_np;
            }
        }
    }
}

/// Can the first subject safely stand for the second during aggregation?
/// Both subjects must be NPs with the same explicit entity. The first
/// surface must contain every modifier/proposition present on the second;
/// a later bare reference may be elided, but later-added content must keep
/// its sentence. Lexical identity alone is not evidence of coreference.
fn first_subject_covers_second(first: &Nominal, second: &Nominal) -> bool {
    match (first, second) {
        (Nominal::Np(a), Nominal::Np(b)) => {
            a.entity.is_some()
                && a.entity == b.entity
                && a.head == b.head
                && a.count == b.count
                && a.referential == b.referential
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
        && first.wh.is_none()
        && first.initial_participles.is_empty()
        && second.clause.topic.is_none()
        && second.clause.focus.is_none()
        && second.clause.wh.is_none()
        && second.clause.initial_participles.is_empty()
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Narrated {
    pub text: String,
    pub warnings: Vec<PhraseWarning>,
}

pub fn narrate_checked(
    sentences: Vec<DiscourseSentence>,
    opts: RealizeOpts,
) -> Result<Narrated, PhraseError> {
    // The planner never receives unchecked authoring trees. It keeps
    // owned raw clones so it can perform tree-to-tree transformations,
    // then validates each transformed result again before resolution.
    for (index, sentence) in sentences.iter().enumerate() {
        validate(&sentence.clause)
            .map_err(|errors| prefix_error(PhraseError::Validation(errors), index))?;
    }

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
    let mut warnings = Vec::new();
    for (index, sentence) in aggregated.iter().enumerate() {
        if !out.is_empty() {
            out.push(' ');
        }
        let validated = validate(&sentence.clause)
            .map_err(|errors| prefix_error(PhraseError::Validation(errors), index))?;
        let realized = realize_validated_with_lead_in(
            &validated,
            sentence.connective.map(Connective::word),
            opts,
        )
        .map_err(|error| prefix_error(error, index))?;
        out.push_str(&realized.text);
        warnings.extend(
            realized
                .warnings
                .into_iter()
                .map(|warning| prefix_warning(warning, index)),
        );
    }
    Ok(Narrated {
        text: out,
        warnings,
    })
}

fn prefix_error(error: PhraseError, sentence: usize) -> PhraseError {
    let prefix = |path: String| format!("sentence[{sentence}].{path}");
    match error {
        PhraseError::Validation(ValidationErrors(errors)) => {
            PhraseError::Validation(ValidationErrors(
                errors
                    .into_iter()
                    .map(|mut error| {
                        error.path = AstPath(prefix(error.path.0));
                        error
                    })
                    .collect(),
            ))
        }
        PhraseError::Resolution(ResolutionErrors(errors)) => {
            PhraseError::Resolution(ResolutionErrors(
                errors
                    .into_iter()
                    .map(|mut error| {
                        error.path = prefix(error.path);
                        error
                    })
                    .collect(),
            ))
        }
        PhraseError::GuessedHead { path, lemma } => PhraseError::GuessedHead {
            path: prefix(path),
            lemma,
        },
        PhraseError::Unsupported { path, feature } => PhraseError::Unsupported {
            path: prefix(path),
            feature,
        },
    }
}

/// Warning-discarding discourse convenience. Use [`narrate_checked`] when
/// diagnostics must be retained.
pub fn narrate(
    sentences: Vec<DiscourseSentence>,
    opts: RealizeOpts,
) -> Result<String, PhraseError> {
    narrate_checked(sentences, opts).map(|narrated| narrated.text)
}

fn prefix_warning(warning: PhraseWarning, sentence: usize) -> PhraseWarning {
    let prefix = |path: String| format!("sentence[{sentence}].{path}");
    match warning {
        PhraseWarning::PerfectivePresent { path, verb } => PhraseWarning::PerfectivePresent {
            path: prefix(path),
            verb,
        },
        PhraseWarning::GovernsConflict {
            path,
            verb,
            dictionary,
            used,
        } => PhraseWarning::GovernsConflict {
            path: prefix(path),
            verb,
            dictionary,
            used,
        },
        PhraseWarning::AmbiguousOrder { path } => {
            PhraseWarning::AmbiguousOrder { path: prefix(path) }
        }
    }
}
