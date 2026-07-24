//! Shared raw-AST validation.
//!
//! Typed builders and the S-expression compiler both produce the same
//! raw [`Clause`](crate::Clause). Every public realization boundary
//! passes that tree through this validator before grammar resolution.

use crate::ast::*;
use interslavic::preposition_cases;
use std::fmt;

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

fn push(errors: &mut Vec<ValidationError>, path: impl Into<String>, kind: ValidationErrorKind) {
    errors.push(ValidationError {
        path: AstPath(path.into()),
        kind,
    });
}

fn validate_clause(clause: &Clause, path: &str, errors: &mut Vec<ValidationError>) {
    validate_nominal(&clause.subject, &format!("{path}.subject"), errors);

    let imperative = matches!(clause.force, Force::Imperative(_));
    if imperative && clause.mood == Mood::Conditional {
        push(
            errors,
            format!("{path}.mood"),
            ValidationErrorKind::IncoherentClause("conditional imperative"),
        );
    }
    if imperative && clause.voice == Voice::Passive {
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
    if clause.mood == Mood::Conditional && clause.tense != TenseSpec::Present {
        push(
            errors,
            format!("{path}.tense"),
            ValidationErrorKind::IncoherentClause(
                "conditional cannot carry independent past/future tense",
            ),
        );
    }

    let has_object_slot = match &clause.core {
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
            coordination
                .items
                .first()
                .is_some_and(|vp| vp.object.is_some())
        }
        ClauseCore::Copular {
            predicate,
            pred_case,
        } => {
            if clause.voice == Voice::Passive {
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
                Predicate::Adjectival(adjective) => {
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
            true
        }
    };

    let subject_surfaces = !imperative && !clause.prodrop;
    for (name, reference) in [("topic", clause.topic), ("focus", clause.focus)] {
        if let Some(reference) = reference {
            let exists = match reference {
                SlotRef::Subject => subject_surfaces,
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
    if voice == Voice::Passive && vp.object.is_some() {
        push(
            errors,
            format!("{path}.object"),
            ValidationErrorKind::IncoherentClause(
                "a passive clause promotes the patient; it cannot retain an object",
            ),
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
        GapRole::Object => {
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
