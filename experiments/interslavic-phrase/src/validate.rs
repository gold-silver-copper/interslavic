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
                    ClauseCore::Copular { predicate, .. } => {
                        if let Predicate::Nominal(np) = predicate {
                            stack.push((
                                StructureNode::Np(np),
                                next,
                                format!("{path}.core.predicate"),
                            ));
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

fn validate_clause(clause: &Clause, path: &str, errors: &mut Vec<ValidationError>) {
    validate_nominal(&clause.subject, &format!("{path}.subject"), errors);
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
            predicate,
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
            match predicate {
                Predicate::Nominal(np) => {
                    validate_np(np, &format!("{path}.core.predicate"), errors);
                }
                Predicate::Adjectival(adjective) | Predicate::ShortAdjectival(adjective) => {
                    validate_leaf(
                        adjective,
                        "adjective",
                        &format!("{path}.core.predicate"),
                        errors,
                    );
                    if *pred_case == PredCase::Instrumental {
                        push(
                            errors,
                            format!("{path}.core.pred_case"),
                            ValidationErrorKind::IncoherentClause(
                                "instrumental predicate case is nominal-only",
                            ),
                        );
                    }
                }
                Predicate::Participial(verb) => {
                    validate_leaf(
                        verb,
                        "participle lemma",
                        &format!("{path}.core.predicate"),
                        errors,
                    );
                    if *pred_case == PredCase::Instrumental {
                        push(
                            errors,
                            format!("{path}.core.pred_case"),
                            ValidationErrorKind::IncoherentClause(
                                "instrumental predicate case is nominal-only",
                            ),
                        );
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
