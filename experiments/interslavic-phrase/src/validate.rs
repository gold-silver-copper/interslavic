//! Shared raw-AST validation.
//!
//! Typed builders and the S-expression compiler both produce the same
//! raw [`Clause`](crate::Clause). Every public realization boundary
//! passes that tree through this validator before grammar resolution.

use crate::ast::*;
use crate::profile::nominal_profile;
use interslavic::Person;
use interslavic::preposition_cases;
use std::fmt;

/// Maximum recursive syntax depth accepted by the shared validator.
///
/// This bounds every recursive consumer of a validated tree. The
/// S-expression reader uses a larger derived list-depth bound because
/// wrapper forms such as `(object …)` do not correspond one-for-one to
/// AST nodes.
pub const MAX_STRUCTURE_DEPTH: usize = 128;

/// Maximum depth of *clause* embedding specifically.
///
/// Clause recursion is bounded far below [`MAX_STRUCTURE_DEPTH`] because
/// each embedded clause costs a recursive-descent frame in the
/// S-expression compiler and another in clause planning, and those frames
/// are large — a clause carries a subject, a core, and a dozen feature
/// fields. The generic structure bound was set when nothing recursed this
/// expensively; at 128 it permits nesting deep enough to exhaust a test
/// thread's stack before any diagnostic is produced.
///
/// 32 is far above anything attested: the deepest sentence in the Steen
/// sample corpus embeds three clauses. This bound is checked iteratively,
/// before the recursive compiler runs, so exceeding it is a spanned
/// diagnostic rather than a crash.
pub const MAX_CLAUSE_DEPTH: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstPath(pub String);

impl fmt::Display for AstPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationErrorKind {
    EmptyCoordination,
    EmptyLeaf(&'static str),
    MaximumDepth {
        limit: usize,
    },
    IncoherentClause(&'static str),
    InvalidRelative(&'static str),
    UnknownPreposition(String),
    AmbiguousPreposition {
        preposition: String,
        allowed: Vec<interslavic::Case>,
    },
    InvalidPrepositionCase {
        preposition: String,
        allowed: Vec<interslavic::Case>,
        used: interslavic::Case,
    },
    MissingInformationSlot(SlotRef),
    DuplicateInformationSlot(SlotRef),
    InvalidReferentialForm(&'static str),
    InvalidSubordinate(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub path: AstPath,
    pub kind: ValidationErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationErrors(pub Vec<ValidationError>);

impl fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, error) in self.0.iter().enumerate() {
            if index > 0 {
                f.write_str("; ")?;
            }
            write!(f, "{}: {:?}", error.path, error.kind)?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationErrors {}

/// A clause whose structural invariants have been checked. Its fields
/// are private; grammar resolution is the next allowed consumer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedClause {
    pub(crate) clause: Clause,
}

impl ValidatedClause {
    pub fn as_raw(&self) -> &Clause {
        &self.clause
    }
}

pub fn validate(clause: &Clause) -> Result<ValidatedClause, ValidationErrors> {
    if let Some(error) = validate_structure_depth(clause) {
        return Err(ValidationErrors(vec![error]));
    }
    let mut errors = Vec::new();
    validate_clause(clause, "clause", &mut errors);
    if errors.is_empty() {
        Ok(ValidatedClause {
            clause: clause.clone(),
        })
    } else {
        Err(ValidationErrors(errors))
    }
}

enum StructureNode<'a> {
    Clause(&'a Clause),
    Nominal(&'a Nominal),
    Np(&'a NounPhrase),
    Relative(&'a RelClause),
    Vp(&'a VerbPhrase),
    Pp(&'a PrepPhrase),
    Participial(&'a ParticipialAdjunct),
}

/// Iterative preflight: callers may construct arbitrarily deep public
/// typed trees, so check depth before entering the recursive semantic
/// validator. This also makes the parser/printer round-trip contract a
/// property of `validate`, rather than a parser-only implementation cap.
fn validate_structure_depth(clause: &Clause) -> Option<ValidationError> {
    let mut stack = vec![(StructureNode::Clause(clause), 0, "clause".to_string())];
    while let Some((node, depth, path)) = stack.pop() {
        if depth > MAX_STRUCTURE_DEPTH {
            return Some(ValidationError {
                path: AstPath(path),
                kind: ValidationErrorKind::MaximumDepth {
                    limit: MAX_STRUCTURE_DEPTH,
                },
            });
        }
        let next = depth + 1;
        match node {
            StructureNode::Clause(clause) => {
                for (index, coordinate) in clause.coordinate_clauses.iter().enumerate() {
                    stack.push((
                        StructureNode::Clause(&coordinate.clause),
                        next,
                        format!("{path}.coordinate_clause[{index}]"),
                    ));
                }
                for (index, adjunct) in clause.adverbial_clauses.iter().enumerate() {
                    stack.push((
                        StructureNode::Clause(&adjunct.clause),
                        next,
                        format!("{path}.adverbial_clause[{index}]"),
                    ));
                }
                for (index, adjunct) in clause.initial_participles.iter().enumerate() {
                    stack.push((
                        StructureNode::Participial(adjunct),
                        next,
                        format!("{path}.initial_participle[{index}]"),
                    ));
                }
                stack.push((
                    StructureNode::Nominal(&clause.subject),
                    next,
                    format!("{path}.subject"),
                ));
                match &clause.core {
                    ClauseCore::Verbal(coordination) => {
                        for (index, vp) in coordination.items.iter().enumerate() {
                            stack.push((
                                StructureNode::Vp(vp),
                                next,
                                format!("{path}.core.vp[{index}]"),
                            ));
                        }
                    }
                    ClauseCore::Copular { predicates, .. } => {
                        for (index, predicate) in predicates.items().iter().enumerate() {
                            match predicate {
                                Predicate::Nominal(np) => stack.push((
                                    StructureNode::Np(np),
                                    next,
                                    format!("{path}.core.predicate[{index}]"),
                                )),
                                Predicate::Prepositional(pp) => stack.push((
                                    StructureNode::Pp(pp),
                                    next,
                                    format!("{path}.core.predicate[{index}]"),
                                )),
                                _ => {}
                            }
                        }
                    }
                }
            }
            StructureNode::Nominal(nominal) => match nominal {
                Nominal::Np(np) => {
                    stack.push((StructureNode::Np(np), next, path));
                }
                Nominal::Coord(coordination) => {
                    for (index, item) in coordination.items.iter().enumerate() {
                        stack.push((
                            StructureNode::Nominal(item),
                            next,
                            format!("{path}.item[{index}]"),
                        ));
                    }
                }
                Nominal::Pron { .. } | Nominal::Name { .. } => {}
            },
            StructureNode::Np(np) => {
                if let Some(relative) = &np.relative {
                    stack.push((
                        StructureNode::Relative(relative),
                        next,
                        format!("{path}.relative"),
                    ));
                }
            }
            StructureNode::Relative(relative) => {
                if let Some(subject) = &relative.subject {
                    stack.push((
                        StructureNode::Nominal(subject),
                        next,
                        format!("{path}.subject"),
                    ));
                }
                stack.push((StructureNode::Vp(&relative.vp), next, format!("{path}.vp")));
            }
            StructureNode::Vp(vp) => {
                if let Some(infinitive) = &vp.infinitive {
                    stack.push((
                        StructureNode::Vp(infinitive),
                        next,
                        format!("{path}.infinitive"),
                    ));
                }
                if let Some(sub) = &vp.complement_clause {
                    stack.push((
                        StructureNode::Clause(&sub.clause),
                        next,
                        format!("{path}.complement_clause"),
                    ));
                }
                if let Some(object) = &vp.object {
                    stack.push((
                        StructureNode::Nominal(&object.nominal),
                        next,
                        format!("{path}.object.nominal"),
                    ));
                }
                if let Some(recipient) = &vp.recipient {
                    stack.push((
                        StructureNode::Nominal(&recipient.nominal),
                        next,
                        format!("{path}.recipient.nominal"),
                    ));
                }
                for (index, pp) in vp.pps.iter().enumerate() {
                    stack.push((StructureNode::Pp(pp), next, format!("{path}.pp[{index}]")));
                }
                for (index, oblique) in vp.obliques.iter().enumerate() {
                    stack.push((
                        StructureNode::Nominal(&oblique.nominal),
                        next,
                        format!("{path}.oblique[{index}].nominal"),
                    ));
                }
            }
            StructureNode::Pp(pp) => {
                stack.push((
                    StructureNode::Nominal(&pp.object),
                    next,
                    format!("{path}.object"),
                ));
            }
            StructureNode::Participial(adjunct) => {
                for (index, pp) in adjunct.pps.iter().enumerate() {
                    stack.push((StructureNode::Pp(pp), next, format!("{path}.pp[{index}]")));
                }
            }
        }
    }
    None
}

fn push(errors: &mut Vec<ValidationError>, path: impl Into<String>, kind: ValidationErrorKind) {
    errors.push(ValidationError {
        path: AstPath(path.into()),
        kind,
    });
}

/// A subordinate clause is a full clause, so it goes through the same
/// validator. What it may NOT do is carry independent sentence force: an
/// embedded clause is not asserted, questioned, or commanded on its own,
/// and the terminal punctuation and capitalization belong to the matrix
/// sentence. Information structure is likewise a matrix-level decision.
fn validate_subordinate(sub: &SubClause, path: &str, errors: &mut Vec<ValidationError>) {
    if sub.clause.force != Force::Declarative {
        push(
            errors,
            format!("{path}.force"),
            ValidationErrorKind::InvalidSubordinate(
                "a subordinate clause cannot carry independent sentence force",
            ),
        );
    }
    // Every `da` in the sources is a purpose clause with the irrealis
    // auxiliary (`da by uviděl`, `da byhmo ne råzprostrånili sę`). The
    // complementizer does not supply `by`; the embedded clause's own
    // conditional mood does, which is what person-marks it.
    if sub.complementizer == Complementizer::Da
        && !matches!(
            sub.clause.mood,
            Mood::Conditional | Mood::ConditionalPerfect
        )
    {
        push(
            errors,
            format!("{path}.mood"),
            ValidationErrorKind::InvalidSubordinate(
                "`da` introduces a purpose clause and needs conditional mood for its `by`",
            ),
        );
    }
    validate_clause(&sub.clause, path, errors);
}

fn validate_clause(clause: &Clause, path: &str, errors: &mut Vec<ValidationError>) {
    validate_nominal(&clause.subject, &format!("{path}.subject"), errors);
    for (index, coordinate) in clause.coordinate_clauses.iter().enumerate() {
        let coordinate_path = format!("{path}.coordinate_clause[{index}]");
        // A coordinate clause is asserted alongside the matrix clause, so
        // sentence force belongs to the sentence, not to each conjunct.
        if coordinate.clause.force != Force::Declarative {
            push(
                errors,
                format!("{coordinate_path}.force"),
                ValidationErrorKind::InvalidSubordinate(
                    "a coordinated clause cannot carry its own sentence force",
                ),
            );
        }
        validate_clause(&coordinate.clause, &coordinate_path, errors);
    }
    for (index, adjunct) in clause.adverbial_clauses.iter().enumerate() {
        validate_subordinate(
            adjunct,
            &format!("{path}.adverbial_clause[{index}]"),
            errors,
        );
    }
    for (index, adjunct) in clause.initial_participles.iter().enumerate() {
        let adjunct_path = format!("{path}.initial_participle[{index}]");
        validate_leaf(
            &adjunct.verb,
            "participle lemma",
            &format!("{adjunct_path}.verb"),
            errors,
        );
        for (pp_index, pp) in adjunct.pps.iter().enumerate() {
            validate_pp(pp, &format!("{adjunct_path}.pp[{pp_index}]"), errors);
        }
    }

    let imperative = matches!(clause.force, Force::Imperative(_));
    if imperative && clause.mood != Mood::Indicative {
        push(
            errors,
            format!("{path}.mood"),
            ValidationErrorKind::IncoherentClause("conditional imperative"),
        );
    }
    if imperative && clause.voice != Voice::Active {
        push(
            errors,
            format!("{path}.voice"),
            ValidationErrorKind::IncoherentClause("passive imperative is unsupported"),
        );
    }
    if imperative && clause.tense != TenseSpec::Present {
        push(
            errors,
            format!("{path}.tense"),
            ValidationErrorKind::IncoherentClause("imperative cannot carry past/future tense"),
        );
    }
    if clause.mood != Mood::Indicative && clause.tense != TenseSpec::Present {
        push(
            errors,
            format!("{path}.tense"),
            ValidationErrorKind::IncoherentClause(
                "conditional cannot carry independent past/future tense",
            ),
        );
    }
    if clause.force == Force::Optative {
        if clause.mood != Mood::Indicative {
            push(
                errors,
                format!("{path}.mood"),
                ValidationErrorKind::IncoherentClause(
                    "optative force cannot carry independent conditional mood",
                ),
            );
        }
        if clause.voice != Voice::Active {
            push(
                errors,
                format!("{path}.voice"),
                ValidationErrorKind::IncoherentClause("passive optative is unsupported"),
            );
        }
        if clause.tense != TenseSpec::Present {
            push(
                errors,
                format!("{path}.tense"),
                ValidationErrorKind::IncoherentClause("optative must use present morphology"),
            );
        }
        if nominal_profile(&clause.subject).person != Person::Third {
            push(
                errors,
                format!("{path}.subject"),
                ValidationErrorKind::IncoherentClause("optative requires a third-person subject"),
            );
        }
    }
    match (&clause.force, &clause.wh) {
        (Force::WhQuestion, Some(WhFront::Adverb(adverb))) => {
            validate_leaf(
                adverb,
                "interrogative adverb",
                &format!("{path}.wh"),
                errors,
            );
        }
        (Force::WhQuestion, Some(WhFront::Slot(_))) => {}
        (Force::WhQuestion, None) => push(
            errors,
            format!("{path}.wh"),
            ValidationErrorKind::IncoherentClause(
                "constituent question requires a fronted slot or adverb",
            ),
        ),
        (_, Some(_)) => push(
            errors,
            format!("{path}.wh"),
            ValidationErrorKind::IncoherentClause(
                "a fronted interrogative requires constituent-question force",
            ),
        ),
        (_, None) => {}
    }
    if clause.wh.is_some() && (clause.topic.is_some() || clause.focus.is_some()) {
        push(
            errors,
            path,
            ValidationErrorKind::IncoherentClause(
                "constituent questions cannot also set topic or focus",
            ),
        );
    }

    let (has_recipient_slot, has_object_slot) = match &clause.core {
        ClauseCore::Verbal(coordination) => {
            if coordination.items.is_empty() {
                push(
                    errors,
                    format!("{path}.core.verbal"),
                    ValidationErrorKind::EmptyCoordination,
                );
            }
            for (index, vp) in coordination.items.iter().enumerate() {
                validate_vp(
                    vp,
                    &format!("{path}.core.vp[{index}]"),
                    clause.voice,
                    errors,
                );
            }
            (
                information_recipient_index(&clause.core).is_some(),
                information_object_index(&clause.core).is_some(),
            )
        }
        ClauseCore::Copular {
            predicates,
            pred_case,
        } => {
            if clause.voice != Voice::Active {
                push(
                    errors,
                    format!("{path}.voice"),
                    ValidationErrorKind::IncoherentClause(
                        "passive voice requires a verbal clause core",
                    ),
                );
            }
            if predicates.items().is_empty() {
                push(
                    errors,
                    format!("{path}.core.predicate"),
                    ValidationErrorKind::EmptyCoordination,
                );
            }
            for (index, predicate) in predicates.items().iter().enumerate() {
                let predicate_path = format!("{path}.core.predicate[{index}]");
                // Only a nominal predicate can take the instrumental.
                // Everything else agrees in the nominative, so requesting
                // it is rejected rather than silently dropped.
                let nominal_only = |errors: &mut Vec<ValidationError>| {
                    if *pred_case == PredCase::Instrumental {
                        push(
                            errors,
                            format!("{path}.core.pred_case"),
                            ValidationErrorKind::IncoherentClause(
                                "instrumental predicate case is nominal-only",
                            ),
                        );
                    }
                };
                match predicate {
                    Predicate::Nominal(np) => validate_np(np, &predicate_path, errors),
                    Predicate::Adjectival(adjective) | Predicate::ShortAdjectival(adjective) => {
                        validate_leaf(adjective, "adjective", &predicate_path, errors);
                        nominal_only(errors);
                    }
                    Predicate::Graded { lemma, .. } => {
                        validate_leaf(lemma, "adjective", &predicate_path, errors);
                        nominal_only(errors);
                    }
                    Predicate::Participial(verb) => {
                        validate_leaf(verb, "participle lemma", &predicate_path, errors);
                        nominal_only(errors);
                    }
                    Predicate::Prepositional(pp) => {
                        // The PP owns its own case through its
                        // preposition, so predicate case cannot apply.
                        validate_pp(pp, &predicate_path, errors);
                        nominal_only(errors);
                    }
                }
            }
            (false, true)
        }
    };

    let subject_surfaces = !imperative && !clause.prodrop;
    if let Some(WhFront::Slot(reference)) = &clause.wh {
        let exists = match *reference {
            SlotRef::Subject => subject_surfaces,
            SlotRef::Recipient => has_recipient_slot,
            SlotRef::Object => has_object_slot,
        };
        if !exists {
            push(
                errors,
                format!("{path}.wh"),
                ValidationErrorKind::MissingInformationSlot(*reference),
            );
        }
    }
    for (name, reference) in [("topic", clause.topic), ("focus", clause.focus)] {
        if let Some(reference) = reference {
            let exists = match reference {
                SlotRef::Subject => subject_surfaces,
                SlotRef::Recipient => has_recipient_slot,
                SlotRef::Object => has_object_slot,
            };
            if !exists {
                push(
                    errors,
                    format!("{path}.{name}"),
                    ValidationErrorKind::MissingInformationSlot(reference),
                );
            }
        }
    }
    if clause.topic.is_some() && clause.topic == clause.focus {
        push(
            errors,
            format!("{path}.focus"),
            ValidationErrorKind::DuplicateInformationSlot(clause.focus.expect("checked as some")),
        );
    }
}

fn validate_vp(vp: &VerbPhrase, path: &str, voice: Voice, errors: &mut Vec<ValidationError>) {
    validate_leaf(&vp.verb, "verb", &format!("{path}.verb"), errors);
    if let Some(infinitive) = &vp.infinitive {
        // The infinitive is non-finite: tense, mood, voice, and force all
        // belong to the finite verb governing it. Passing `Voice::Active`
        // keeps the object check meaningful without letting a passive
        // matrix clause silently strip the infinitive's own object.
        validate_vp(
            infinitive,
            &format!("{path}.infinitive"),
            Voice::Active,
            errors,
        );
    }
    if let Some(sub) = &vp.complement_clause {
        if sub.position != AdjunctPosition::Final {
            push(
                errors,
                format!("{path}.complement_clause.position"),
                ValidationErrorKind::InvalidSubordinate(
                    "a verb's complement clause follows it; only clause adverbials front",
                ),
            );
        }
        validate_subordinate(sub, &format!("{path}.complement_clause"), errors);
    }
    if voice != Voice::Active && vp.object.is_some() {
        push(
            errors,
            format!("{path}.object"),
            ValidationErrorKind::IncoherentClause(
                "a passive clause promotes the patient; it cannot retain an object",
            ),
        );
    }
    if let Some(recipient) = &vp.recipient {
        validate_nominal(
            &recipient.nominal,
            &format!("{path}.recipient.nominal"),
            errors,
        );
    }
    if let Some(object) = &vp.object {
        validate_nominal(&object.nominal, &format!("{path}.object.nominal"), errors);
    }
    for (index, adverb) in vp.adverbs.iter().enumerate() {
        validate_leaf(adverb, "adverb", &format!("{path}.adverb[{index}]"), errors);
    }
    for (index, pp) in vp.pps.iter().enumerate() {
        validate_pp(pp, &format!("{path}.pp[{index}]"), errors);
    }
    for (index, oblique) in vp.obliques.iter().enumerate() {
        validate_nominal(
            &oblique.nominal,
            &format!("{path}.oblique[{index}].nominal"),
            errors,
        );
    }
}

fn validate_pp(pp: &PrepPhrase, path: &str, errors: &mut Vec<ValidationError>) {
    validate_leaf(
        &pp.preposition,
        "preposition",
        &format!("{path}.preposition"),
        errors,
    );
    validate_preposition(&pp.preposition, pp.case, path, errors);
    validate_nominal(&pp.object, &format!("{path}.object"), errors);
}

fn validate_preposition(
    preposition: &str,
    requested: Option<interslavic::Case>,
    path: &str,
    errors: &mut Vec<ValidationError>,
) {
    let Some(allowed) = preposition_cases(preposition) else {
        push(
            errors,
            format!("{path}.preposition"),
            ValidationErrorKind::UnknownPreposition(preposition.to_string()),
        );
        return;
    };
    match requested {
        Some(used) if !allowed.contains(&used) => push(
            errors,
            format!("{path}.case"),
            ValidationErrorKind::InvalidPrepositionCase {
                preposition: preposition.to_string(),
                allowed: allowed.to_vec(),
                used,
            },
        ),
        None if allowed.len() != 1 => push(
            errors,
            format!("{path}.case"),
            ValidationErrorKind::AmbiguousPreposition {
                preposition: preposition.to_string(),
                allowed: allowed.to_vec(),
            },
        ),
        _ => {}
    }
}

fn validate_nominal(nominal: &Nominal, path: &str, errors: &mut Vec<ValidationError>) {
    match nominal {
        Nominal::Np(np) => validate_np(np, path, errors),
        Nominal::Pron { .. } => {}
        Nominal::Name { text, .. } => validate_leaf(text, "name", path, errors),
        Nominal::Coord(coordination) => {
            if coordination.items.is_empty() {
                push(errors, path, ValidationErrorKind::EmptyCoordination);
            }
            for (index, nominal) in coordination.items.iter().enumerate() {
                validate_nominal(nominal, &format!("{path}.item[{index}]"), errors);
            }
        }
    }
}

fn validate_np(np: &NounPhrase, path: &str, errors: &mut Vec<ValidationError>) {
    validate_leaf(&np.head, "noun", &format!("{path}.head"), errors);
    // A numeral already determines number — and determines it by a rule
    // (`2..=4` against `5+`) that an explicit number could contradict.
    // Rather than silently letting one win, reject the combination.
    if np.count.is_some() && np.number.is_some() {
        push(
            errors,
            format!("{path}.number"),
            ValidationErrorKind::IncoherentClause(
                "a counted noun phrase already has its number; drop `:pl`/`:sg`",
            ),
        );
    }
    if let Some(determiner) = &np.determiner {
        validate_leaf(
            determiner,
            "determiner",
            &format!("{path}.determiner"),
            errors,
        );
    }
    for (index, adjective) in np.adjectives.iter().enumerate() {
        validate_leaf(
            adjective,
            "adjective",
            &format!("{path}.adjective[{index}]"),
            errors,
        );
    }
    if np.entity.as_ref().is_some_and(|id| id.trim().is_empty()) {
        push(
            errors,
            format!("{path}.entity"),
            ValidationErrorKind::EmptyLeaf("entity id"),
        );
    }
    if let Some(relative) = &np.relative {
        if np.referential != ReferentialForm::Full {
            push(
                errors,
                format!("{path}.referential"),
                ValidationErrorKind::InvalidReferentialForm(
                    "a pronominal reference cannot suppress a relative proposition",
                ),
            );
        }
        validate_relative(relative, &format!("{path}.relative"), errors);
    }
}

fn validate_relative(relative: &RelClause, path: &str, errors: &mut Vec<ValidationError>) {
    match &relative.gap {
        GapRole::Subject => {
            if relative.subject.is_some() {
                push(
                    errors,
                    format!("{path}.subject"),
                    ValidationErrorKind::InvalidRelative(
                        "a subject gap cannot also carry an overt subject",
                    ),
                );
            }
        }
        GapRole::Object { .. } => {
            if relative.subject.is_none() {
                push(
                    errors,
                    format!("{path}.subject"),
                    ValidationErrorKind::InvalidRelative("an object gap requires an overt subject"),
                );
            }
            if relative.vp.object.is_some() {
                push(
                    errors,
                    format!("{path}.vp.object"),
                    ValidationErrorKind::InvalidRelative(
                        "an object gap cannot also carry an overt object",
                    ),
                );
            }
        }
        GapRole::PpObject { preposition, case } => {
            if relative.subject.is_none() {
                push(
                    errors,
                    format!("{path}.subject"),
                    ValidationErrorKind::InvalidRelative("a PP gap requires an overt subject"),
                );
            }
            validate_preposition(preposition, Some(*case), &format!("{path}.gap"), errors);
        }
    }
    if let Some(subject) = &relative.subject {
        validate_nominal(subject, &format!("{path}.subject"), errors);
    }
    validate_vp(&relative.vp, &format!("{path}.vp"), Voice::Active, errors);
}

fn validate_leaf(value: &str, what: &'static str, path: &str, errors: &mut Vec<ValidationError>) {
    if value.trim().is_empty() {
        push(errors, path, ValidationErrorKind::EmptyLeaf(what));
    }
}
