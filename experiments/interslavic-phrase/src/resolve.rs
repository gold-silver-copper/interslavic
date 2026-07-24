//! Grammar resolution.
//!
//! Validation proves structural coherence. This stage assigns one
//! authoritative case and one central nominal profile to every nominal
//! role before any surface planning begins.

use crate::ast::*;
use crate::profile::{NominalProfile, nominal_profile};
use crate::validate::ValidatedClause;
use interslavic::{Case, VerbInfo, preposition_cases, verb_info};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CaseSource {
    Subject,
    DefaultAccusative,
    Dictionary,
    ExplicitComplement,
    Preposition,
    Predicate,
}

#[derive(Debug, Clone)]
pub(crate) struct ResolvedNominal {
    pub kind: ResolvedNominalKind,
    pub case: Case,
    pub case_source: CaseSource,
    pub profile: NominalProfile,
}

#[derive(Debug, Clone)]
pub(crate) enum ResolvedNominalKind {
    Np {
        source: NounPhrase,
        relative: Option<Box<ResolvedRelative>>,
    },
    Pron {
        person: interslavic::Person,
        number: interslavic::Number,
        gender: interslavic::Gender,
        clitic: bool,
    },
    Name {
        text: String,
        gender: interslavic::Gender,
        indeclinable: bool,
    },
    Coord {
        conjunction: Conj,
        items: Vec<ResolvedNominal>,
    },
}

#[derive(Debug, Clone)]
pub(crate) struct ResolvedPrep {
    pub preposition: String,
    pub object: ResolvedNominal,
}

#[derive(Debug, Clone)]
pub(crate) struct ResolvedVerbPhrase {
    pub bare_verb: String,
    pub reflexive: bool,
    pub info: Option<VerbInfo>,
    pub object: Option<ResolvedNominal>,
    pub object_case: Option<Case>,
    pub adverbs: Vec<String>,
    pub pps: Vec<ResolvedPrep>,
}

#[derive(Debug, Clone)]
pub(crate) struct ResolvedRelative {
    pub path: String,
    pub gap: GapRole,
    pub gap_case: Case,
    pub subject: Option<ResolvedNominal>,
    pub vp: ResolvedVerbPhrase,
    pub tense: TenseSpec,
    pub polarity: Polarity,
    pub relativizer: Relativizer,
}

#[derive(Debug, Clone)]
pub(crate) enum ResolvedPredicate {
    Nominal(ResolvedNominal),
    Adjectival(String),
    Participial(String),
}

#[derive(Debug, Clone)]
pub(crate) enum ResolvedCore {
    Verbal {
        conjunction: Conj,
        vps: Vec<ResolvedVerbPhrase>,
    },
    Copular(ResolvedPredicate),
}

#[derive(Debug, Clone)]
pub(crate) struct ResolvedClause {
    pub subject: ResolvedNominal,
    pub core: ResolvedCore,
    pub tense: TenseSpec,
    pub polarity: Polarity,
    pub force: Force,
    pub mood: Mood,
    pub voice: Voice,
    pub prodrop: bool,
    pub topic: Option<SlotRef>,
    pub focus: Option<SlotRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GovernmentConflict {
    pub path: String,
    pub verb: String,
    pub dictionary: Case,
    pub used: Case,
}

#[derive(Debug, Clone)]
pub(crate) struct Resolution {
    pub clause: ResolvedClause,
    pub conflicts: Vec<GovernmentConflict>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolutionErrorKind {
    ObjectOfIntransitive { verb: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolutionError {
    pub path: String,
    pub kind: ResolutionErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolutionErrors(pub Vec<ResolutionError>);

impl fmt::Display for ResolutionErrors {
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

impl std::error::Error for ResolutionErrors {}

pub(crate) fn resolve(validated: &ValidatedClause) -> Result<Resolution, ResolutionErrors> {
    let clause = &validated.clause;
    let mut conflicts = Vec::new();
    let mut errors = Vec::new();
    let subject = resolve_nominal(
        &clause.subject,
        Case::Nom,
        CaseSource::Subject,
        "clause.subject",
        &mut conflicts,
        &mut errors,
    );
    let core = match &clause.core {
        ClauseCore::Verbal(coordination) => ResolvedCore::Verbal {
            conjunction: coordination.conjunction,
            vps: coordination
                .items
                .iter()
                .enumerate()
                .map(|(index, vp)| {
                    resolve_vp(
                        vp,
                        None,
                        &format!("clause.core.vp[{index}]"),
                        &mut conflicts,
                        &mut errors,
                    )
                })
                .collect(),
        },
        ClauseCore::Copular {
            predicate,
            pred_case,
        } => {
            let predicate = match predicate {
                Predicate::Nominal(np) => {
                    let case = match pred_case {
                        PredCase::Nominative => Case::Nom,
                        PredCase::Instrumental => Case::Ins,
                    };
                    ResolvedPredicate::Nominal(resolve_nominal(
                        &Nominal::Np(np.clone()),
                        case,
                        CaseSource::Predicate,
                        "clause.core.predicate",
                        &mut conflicts,
                        &mut errors,
                    ))
                }
                Predicate::Adjectival(adjective) => {
                    ResolvedPredicate::Adjectival(adjective.clone())
                }
                Predicate::Participial(verb) => ResolvedPredicate::Participial(verb.clone()),
            };
            ResolvedCore::Copular(predicate)
        }
    };
    let resolution = Resolution {
        clause: ResolvedClause {
            subject,
            core,
            tense: clause.tense,
            polarity: clause.polarity,
            force: clause.force,
            mood: clause.mood,
            voice: clause.voice,
            prodrop: clause.prodrop,
            topic: clause.topic,
            focus: clause.focus,
        },
        conflicts,
    };
    if errors.is_empty() {
        Ok(resolution)
    } else {
        Err(ResolutionErrors(errors))
    }
}

fn resolve_nominal(
    nominal: &Nominal,
    case: Case,
    source: CaseSource,
    path: &str,
    conflicts: &mut Vec<GovernmentConflict>,
    errors: &mut Vec<ResolutionError>,
) -> ResolvedNominal {
    let profile = nominal_profile(nominal);
    let kind = match nominal {
        Nominal::Np(np) => ResolvedNominalKind::Np {
            source: np.clone(),
            relative: np
                .relative
                .as_ref()
                .map(|relative| Box::new(resolve_relative(relative, path, conflicts, errors))),
        },
        Nominal::Pron {
            person,
            number,
            gender,
            clitic,
        } => ResolvedNominalKind::Pron {
            person: *person,
            number: *number,
            gender: *gender,
            clitic: *clitic,
        },
        Nominal::Name {
            text,
            gender,
            indeclinable,
        } => ResolvedNominalKind::Name {
            text: text.clone(),
            gender: *gender,
            indeclinable: *indeclinable,
        },
        Nominal::Coord(coordination) => ResolvedNominalKind::Coord {
            conjunction: coordination.conjunction,
            items: coordination
                .items
                .iter()
                .enumerate()
                .map(|(index, item)| {
                    resolve_nominal(
                        item,
                        case,
                        source,
                        &format!("{path}.item[{index}]"),
                        conflicts,
                        errors,
                    )
                })
                .collect(),
        },
    };
    ResolvedNominal {
        kind,
        case,
        case_source: source,
        profile,
    }
}

fn resolve_vp(
    vp: &VerbPhrase,
    gap: Option<&GapRole>,
    path: &str,
    conflicts: &mut Vec<GovernmentConflict>,
    errors: &mut Vec<ResolutionError>,
) -> ResolvedVerbPhrase {
    let (bare_verb, lemma_reflexive, info) = verb_context(&vp.verb);
    let reflexive = lemma_reflexive || info.as_ref().is_some_and(|entry| entry.reflexive);
    let dictionary = info.as_ref().and_then(|entry| entry.governs);
    let has_direct_object = vp.object.is_some() || gap == Some(&GapRole::Object);
    if has_direct_object
        && info.as_ref().and_then(|entry| entry.transitive) == Some(false)
        && dictionary.is_none()
    {
        errors.push(ResolutionError {
            path: format!("{path}.object"),
            kind: ResolutionErrorKind::ObjectOfIntransitive {
                verb: bare_verb.clone(),
            },
        });
    }
    let (object, object_case) = if gap == Some(&GapRole::Object) {
        (None, dictionary.or(Some(Case::Acc)))
    } else if let Some(complement) = &vp.object {
        let (case, source) = match (complement.requested_case, dictionary) {
            (Some(used), Some(marked)) if used != marked => {
                conflicts.push(GovernmentConflict {
                    path: format!("{path}.object"),
                    verb: bare_verb.clone(),
                    dictionary: marked,
                    used,
                });
                (used, CaseSource::ExplicitComplement)
            }
            (Some(used), _) => (used, CaseSource::ExplicitComplement),
            (None, Some(marked)) => (marked, CaseSource::Dictionary),
            (None, None) => (Case::Acc, CaseSource::DefaultAccusative),
        };
        (
            Some(resolve_nominal(
                &complement.nominal,
                case,
                source,
                &format!("{path}.object.nominal"),
                conflicts,
                errors,
            )),
            Some(case),
        )
    } else {
        (None, None)
    };
    let pps = vp
        .pps
        .iter()
        .enumerate()
        .map(|(index, pp)| {
            let allowed = preposition_cases(&pp.preposition)
                .expect("preposition validated before resolution");
            let case = pp.case.unwrap_or(allowed[0]);
            ResolvedPrep {
                preposition: pp.preposition.clone(),
                object: resolve_nominal(
                    &pp.object,
                    case,
                    CaseSource::Preposition,
                    &format!("{path}.pp[{index}].object"),
                    conflicts,
                    errors,
                ),
            }
        })
        .collect();
    ResolvedVerbPhrase {
        bare_verb,
        reflexive,
        info,
        object,
        object_case,
        adverbs: vp.adverbs.clone(),
        pps,
    }
}

fn resolve_relative(
    relative: &RelClause,
    parent_path: &str,
    conflicts: &mut Vec<GovernmentConflict>,
    errors: &mut Vec<ResolutionError>,
) -> ResolvedRelative {
    let path = format!("{parent_path}.relative");
    let vp = resolve_vp(
        &relative.vp,
        Some(&relative.gap),
        &format!("{path}.vp"),
        conflicts,
        errors,
    );
    let gap_case = match &relative.gap {
        GapRole::Subject => Case::Nom,
        GapRole::Object => vp
            .object_case
            .expect("object gaps receive a case during resolution"),
        GapRole::PpObject { case, .. } => *case,
    };
    let subject = relative.subject.as_ref().map(|subject| {
        resolve_nominal(
            subject,
            Case::Nom,
            CaseSource::Subject,
            &format!("{path}.subject"),
            conflicts,
            errors,
        )
    });
    ResolvedRelative {
        path: path.clone(),
        gap: relative.gap.clone(),
        gap_case,
        subject,
        vp,
        tense: relative.tense,
        polarity: relative.polarity,
        relativizer: relative.relativizer,
    }
}

pub(crate) fn verb_context(verb_lemma: &str) -> (String, bool, Option<VerbInfo>) {
    let trimmed = verb_lemma.trim();
    let (bare, lemma_reflexive) = match trimmed.strip_suffix(" sę") {
        Some(bare) => (bare, true),
        None => (trimmed, false),
    };
    let info = if lemma_reflexive {
        verb_info(&format!("{bare} sę")).or_else(|| verb_info(bare))
    } else {
        verb_info(bare)
    };
    (bare.to_string(), lemma_reflexive, info)
}
