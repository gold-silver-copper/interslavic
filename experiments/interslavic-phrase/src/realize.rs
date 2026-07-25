//! Agreement/government resolution and linearization.
//!
//! Every inflected form comes from the `interslavic` facade — this module
//! never post-processes forms. Exactly two string operations happen after
//! forms leave the facade, both documented here: punctuation attachment
//! and sentence-initial capitalization in sentence mode.
//!
//! Linearization remains hierarchical until the final join. A
//! [`NominalPlan`] owns nested [`RelativePlan`] values, while each
//! [`VerbDomainPlan`] owns only its direct clitic cluster. A parent can
//! therefore never extract a clitic from a descendant relative clause.
//!
//! Linearization defaults are steen's (syntax page): S–V–O neutral;
//! "modifiers usually precede the noun"; postverbal clitics (steen's own
//! examples); `li` "right after the focus point of the question, usually
//! the verb"; non-SVO orders sanctioned "when special emphasis is
//! needed", with steen's clarity caveat surfaced as a warning when
//! Nom/Acc syncretism would garden-path. Adverb position (before the
//! verb) is POLICY — the sources are silent.

use crate::ast::*;
use crate::plan::*;
use crate::resolve::{
    ResolutionErrors, ResolvedClause, ResolvedCore, ResolvedNominal, ResolvedNominalKind,
    ResolvedPredicate, ResolvedPrep, ResolvedRelative, ResolvedSubClause, ResolvedVerbPhrase,
    resolve,
};
use crate::validate::{ValidationErrors, validate};
use interslavic::{
    Animacy, Aspect, Case, Gender, Number, Person, PronounStyle, Provenance, Tense,
    active_adverbial_participle, adj, cells, conditional_parts, l_participle, noun_with,
    passive_participle, perfect_parts, personal_pronoun, present_passive_participle, pronoun,
    quantified_parts_with_info, short_adj, verb, verb_forms,
};
use std::fmt;

// ---------------------------------------------------------------------------
// Options, errors, warnings.
// ---------------------------------------------------------------------------

/// Where a verb complex's clitic cluster lands. The cluster is a property
/// of EACH verb complex (its clitic domain), not of the clause: in a
/// coordination every conjunct carries its own cluster, placed by the
/// same rule within its own domain (Franks & King 2000 treat conjuncts as
/// separate clitic domains).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CliticStyle {
    /// Immediately after the verb complex — steen's own examples
    /// ("Ja myju se"). The default.
    #[default]
    Postverbal,
    /// Wackernagel second position: after the first constituent of the
    /// clitic domain. For the first conjunct the domain starts at the
    /// clause; for later conjuncts the domain is the conjunct itself
    /// (which begins with its verb, so domain-second coincides with
    /// postverbal there). A fronted `či` is part of the domain and hosts
    /// the cluster ("Či sę krålj myl?"); a discourse lead-in (the
    /// connective of `realize_with_lead_in`) stands OUTSIDE it ("Potom
    /// krålj sę myl."). In a `li` question, `li` must remain first in its
    /// cluster even when a topic precedes the focused host — both POLICY.
    SecondPosition,
}

/// Realization options, in the `english-phrase` style.
#[derive(Debug, Clone, Copy, Default)]
pub struct RealizeOpts {
    pub sentence: bool,
    /// Treat a `Provenance::Guessed` NP head as an error instead of
    /// trusting the rule engine's gender/animacy guess.
    pub strict_guessed: bool,
    pub clitic_style: CliticStyle,
}

impl RealizeOpts {
    pub fn plain() -> Self {
        Self::default()
    }
    pub fn sentence() -> Self {
        Self {
            sentence: true,
            ..Self::default()
        }
    }
    pub fn strict(mut self) -> Self {
        self.strict_guessed = true;
        self
    }
    pub fn clitics(mut self, style: CliticStyle) -> Self {
        self.clitic_style = style;
        self
    }
}

/// Diagnostics are values; realization of a well-formed tree is total.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhraseError {
    Validation(ValidationErrors),
    Resolution(ResolutionErrors),
    GuessedHead { path: String, lemma: String },
    Unsupported { path: String, feature: &'static str },
}

impl fmt::Display for PhraseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PhraseError::Validation(errors) => write!(f, "invalid phrase tree: {errors}"),
            PhraseError::Resolution(errors) => write!(f, "cannot resolve phrase tree: {errors}"),
            PhraseError::GuessedHead { path, lemma } => {
                write!(
                    f,
                    "{path}: `{lemma}` is not a dictionary noun (gender/animacy would be guessed)"
                )
            }
            PhraseError::Unsupported { path, feature } => {
                write!(f, "{path}: unsupported: {feature}")
            }
        }
    }
}

impl std::error::Error for PhraseError {}

impl From<ValidationErrors> for PhraseError {
    fn from(value: ValidationErrors) -> Self {
        Self::Validation(value)
    }
}

impl From<ResolutionErrors> for PhraseError {
    fn from(value: ResolutionErrors) -> Self {
        Self::Resolution(value)
    }
}

/// Warnings never change output; they surface steen's caveats and the
/// dictionary's opinions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhraseWarning {
    /// A perfective verb in the present tense reads as future/completed
    /// in Slavic — grammatical, but probably not "present".
    PerfectivePresent { path: String, verb: String },
    /// An explicit object case contradicts the dictionary's government
    /// annotation.
    GovernsConflict {
        path: String,
        verb: String,
        dictionary: Case,
        used: Case,
    },
    /// The realized order inverts subject and object while neither
    /// distinguishes Nom from Acc on the surface — steen's own clarity
    /// caveat, detected on the ACTUAL final order.
    AmbiguousOrder { path: String },
}

/// The result of checked realization: the text plus any warnings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Realized {
    pub text: String,
    pub warnings: Vec<PhraseWarning>,
}

/// One clean surface form: byform cells resolve to the first variant and
/// citation accents are stripped — via `cells::variants`, the documented
/// normalization step.
fn surface(form: &str) -> String {
    cells::variants(form).into_iter().next().unwrap_or_default()
}

struct Ctx<'o> {
    opts: &'o RealizeOpts,
    warnings: Vec<PhraseWarning>,
}

// ---------------------------------------------------------------------------
// Nominal rendering.
// ---------------------------------------------------------------------------

/// May a pronoun in this position surface as a clitic? Clitics are
/// "weaker and always unstressed" (steen, pronouns page), so any stressed
/// or structurally isolating position forces the full form: after a
/// preposition (steen: "it is better to use the longer forms"), inside a
/// coordination conjunct, and under information-structure marking
/// (topic/focus = stress).
#[derive(Clone, Copy, PartialEq, Eq)]
enum CliticContext {
    Allowed,
    ForceFull,
}

fn render_nominal(
    nominal: &ResolvedNominal,
    style: PronounStyle,
    clitics: CliticContext,
    ctx: &mut Ctx,
) -> Result<NominalPlan, PhraseError> {
    match &nominal.kind {
        ResolvedNominalKind::Pron {
            person,
            number,
            gender,
            clitic,
        } => Ok(render_pronoun_plan(
            *person, *number, *gender, *clitic, nominal, style, clitics,
        )),
        ResolvedNominalKind::Name {
            text,
            gender,
            indeclinable,
        } => {
            let form = if *indeclinable {
                text.clone()
            } else {
                surface(&noun_with(
                    text,
                    nominal.case,
                    Number::Singular,
                    *gender,
                    Animacy::Animate,
                ))
            };
            Ok(NominalPlan {
                body: vec![word(form)],
                direct_clitic: None,
                profile: nominal.profile,
                case: nominal.case,
                case_source: nominal.case_source,
            })
        }
        ResolvedNominalKind::Np { source, relative } => {
            let info = interslavic::noun_info(&source.head);
            if ctx.opts.strict_guessed && info.provenance == Provenance::Guessed {
                return Err(PhraseError::GuessedHead {
                    path: nominal.path.clone(),
                    lemma: source.head.clone(),
                });
            }
            if source.referential != ReferentialForm::Full {
                return Ok(render_pronoun_plan(
                    Person::Third,
                    nominal.profile.referent_number,
                    nominal.profile.gender,
                    source.referential == ReferentialForm::Clitic,
                    nominal,
                    style,
                    clitics,
                ));
            }
            render_np(source, relative.as_deref(), nominal, ctx)
        }
        ResolvedNominalKind::Coord { conjunction, items } => {
            let mut rendered = Vec::new();
            for item in items {
                // Conjuncts force full pronoun forms (a clitic cannot
                // carry the stress a conjunct bears).
                rendered.push(render_nominal(item, style, CliticContext::ForceFull, ctx)?);
            }
            debug_assert!(
                rendered.iter().all(|item| {
                    item.case == nominal.case && item.case_source == nominal.case_source
                }),
                "one governing edge must assign one case and source to every conjunct"
            );
            let mut body = Vec::new();
            let last = rendered.len() - 1;
            for (index, item) in rendered.into_iter().enumerate() {
                if index > 0 {
                    if index == last {
                        body.push(word(conjunction.word()));
                    } else {
                        body.push(SurfaceNode::Punct(','));
                    }
                }
                body.extend(item.into_surface());
            }
            Ok(NominalPlan {
                body,
                direct_clitic: None,
                profile: nominal.profile,
                case: nominal.case,
                case_source: nominal.case_source,
            })
        }
    }
}

fn render_pronoun_plan(
    person: Person,
    number: Number,
    gender: Gender,
    wants_clitic: bool,
    nominal: &ResolvedNominal,
    style: PronounStyle,
    clitics: CliticContext,
) -> NominalPlan {
    let direct_clitic = (wants_clitic
        && clitics == CliticContext::Allowed
        && style != PronounStyle::AfterPreposition)
        .then(|| personal_pronoun(person, number, gender, nominal.case, PronounStyle::Clitic))
        .flatten();
    let body = if direct_clitic.is_some() {
        Vec::new()
    } else {
        let full = personal_pronoun(person, number, gender, nominal.case, style)
            .or_else(|| personal_pronoun(person, number, gender, nominal.case, PronounStyle::Full))
            .expect("full personal-pronoun cells are total");
        vec![word(full)]
    };
    NominalPlan {
        body,
        direct_clitic,
        profile: nominal.profile,
        case: nominal.case,
        case_source: nominal.case_source,
    }
}

fn render_np(
    np: &NounPhrase,
    relative: Option<&ResolvedRelative>,
    resolved: &ResolvedNominal,
    ctx: &mut Ctx,
) -> Result<NominalPlan, PhraseError> {
    let slot_case = resolved.case;
    let info = interslavic::noun_info(&np.head);
    let (gender, animacy) = (info.gender, info.animacy);

    let modifier = |lemma: &str, case: Case, number: Number| -> String {
        pronoun(lemma, case, number, gender, animacy)
            .unwrap_or_else(|| adj(lemma, case, number, gender, animacy))
    };

    let mut body = Vec::new();
    if let Some(n) = np.count {
        let parts =
            quantified_parts_with_info(n, &np.head, slot_case, gender, animacy, info.plural_only);
        if let Some(det) = &np.determiner {
            body.push(word(surface(&modifier(det, parts.case, parts.number))));
        }
        body.push(word(n.to_string()));
        for adjective in &np.adjectives {
            body.push(word(surface(&modifier(
                adjective,
                parts.case,
                parts.number,
            ))));
        }
        body.push(word(surface(&parts.noun)));
    } else {
        let noun_number = resolved.profile.inflection_number;
        if let Some(det) = &np.determiner {
            body.push(word(surface(&modifier(det, slot_case, noun_number))));
        }
        for adjective in &np.adjectives {
            body.push(word(surface(&modifier(adjective, slot_case, noun_number))));
        }
        body.push(word(surface(&noun_with(
            &np.head,
            slot_case,
            noun_number,
            gender,
            animacy,
        ))));
    }

    if let Some(relative) = relative {
        body.push(SurfaceNode::Relative(Box::new(render_relative(
            relative,
            resolved.profile.inflection_number,
            resolved.profile.gender,
            animacy,
            ctx,
        )?)));
    }

    Ok(NominalPlan {
        body,
        direct_clitic: None,
        profile: resolved.profile,
        case: resolved.case,
        case_source: resolved.case_source,
    })
}

fn render_pp(pp: &ResolvedPrep, ctx: &mut Ctx) -> Result<Vec<SurfaceNode>, PhraseError> {
    let object = render_nominal(
        &pp.object,
        PronounStyle::AfterPreposition,
        CliticContext::ForceFull,
        ctx,
    )?;
    let mut body = vec![word(pp.preposition.clone())];
    body.extend(object.into_surface());
    Ok(body)
}

// ---------------------------------------------------------------------------
// The one verb-complex builder.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct ClauseShape {
    force: Force,
    mood: Mood,
    voice: Voice,
    tense: TenseSpec,
    polarity: Polarity,
}

/// Build ONE finite verb complex — the single implementation for main
/// clauses, coordinated conjuncts, copular clauses (lemma `byti`),
/// relative clauses, imperatives, conditionals, and passives. The input
/// shape has already passed the shared validation matrix.
#[allow(clippy::too_many_arguments)]
fn build_complex(
    path: &str,
    lemma: &str,
    info: Option<&interslavic::VerbInfo>,
    shape: &ClauseShape,
    person: Person,
    number: Number,
    gender: Gender,
    subject_animacy: Animacy,
    warnings: &mut Vec<PhraseWarning>,
) -> Result<Vec<SurfaceNode>, PhraseError> {
    let imperative = matches!(shape.force, Force::Imperative(_));

    let mut tokens = Vec::new();
    if shape.polarity == Polarity::Negative {
        tokens.push(word("ne"));
    }

    if imperative {
        let slot = match shape.force {
            Force::Imperative(Addressee::You) => 0,
            Force::Imperative(Addressee::We) => 1,
            Force::Imperative(Addressee::YouAll) => 2,
            _ => unreachable!("guarded by `imperative`"),
        };
        tokens.push(word(surface(&verb_forms(lemma).imperative[slot])));
        return Ok(tokens);
    }

    if shape.voice != Voice::Active {
        // The passive IS the participial copular construction: byti in
        // the clause's tense/mood plus the passive participle agreeing
        // with the subject.
        match shape.mood {
            Mood::Indicative => {
                let copula_shape = ClauseShape {
                    voice: Voice::Active,
                    polarity: Polarity::Affirmative, // negation already emitted
                    ..*shape
                };
                let mut copula = build_complex(
                    path,
                    "byti",
                    None,
                    &copula_shape,
                    person,
                    number,
                    gender,
                    subject_animacy,
                    warnings,
                )?;
                tokens.append(&mut copula);
            }
            Mood::Conditional => {
                tokens.push(word(
                    conditional_parts("byti", person, number, gender).auxiliary,
                ));
            }
            Mood::ConditionalPerfect => {
                tokens.push(word(surface(&l_participle("byti", gender, number))));
                tokens.push(word(
                    conditional_parts("byti", person, number, gender).auxiliary,
                ));
            }
        }
        let participle = match shape.voice {
            Voice::Passive => passive_participle(lemma, Case::Nom, number, gender, subject_animacy),
            Voice::PresentPassive => {
                present_passive_participle(lemma, Case::Nom, number, gender, subject_animacy)
            }
            Voice::Active => unreachable!("guarded above"),
        }
        .ok_or(PhraseError::Unsupported {
            path: path.to_string(),
            feature: "no passive participle (intransitive verb?)",
        })?;
        tokens.push(word(surface(&participle)));
        return Ok(tokens);
    }

    match shape.mood {
        Mood::Conditional => {
            let parts = conditional_parts(lemma, person, number, gender);
            tokens.push(word(parts.auxiliary));
            tokens.push(word(parts.participle));
        }
        Mood::ConditionalPerfect => {
            tokens.push(word(surface(&l_participle("byti", gender, number))));
            let parts = conditional_parts(lemma, person, number, gender);
            tokens.push(word(parts.auxiliary));
            tokens.push(word(parts.participle));
        }
        Mood::Indicative => match shape.tense {
            TenseSpec::Present => {
                if info.and_then(|entry| entry.aspect) == Some(Aspect::Pf) {
                    warnings.push(PhraseWarning::PerfectivePresent {
                        path: path.to_string(),
                        verb: lemma.to_string(),
                    });
                }
                tokens.push(word(surface(&verb(
                    lemma,
                    person,
                    number,
                    gender,
                    Tense::Present,
                ))));
            }
            TenseSpec::Future => {
                let complex = surface(&verb(lemma, person, number, gender, Tense::Future));
                tokens.extend(complex.split_whitespace().map(word));
            }
            TenseSpec::Past => {
                let parts = perfect_parts(lemma, person, number, gender);
                if let Some(auxiliary) = parts.auxiliary {
                    tokens.push(word(auxiliary));
                }
                tokens.push(word(parts.participle));
            }
            TenseSpec::Imperfect => {
                let complex = surface(&verb(lemma, person, number, gender, Tense::Imperfect));
                tokens.extend(complex.split_whitespace().map(word));
            }
            TenseSpec::Pluperfect => {
                let complex = surface(&verb(lemma, person, number, gender, Tense::Pluperfect));
                tokens.extend(complex.split_whitespace().map(word));
            }
            TenseSpec::CompoundPluperfect => {
                tokens.push(word(surface(&l_participle("byti", gender, number))));
                let parts = perfect_parts(lemma, person, number, gender);
                if let Some(auxiliary) = parts.auxiliary {
                    tokens.push(word(auxiliary));
                }
                tokens.push(word(parts.participle));
            }
        },
    }
    Ok(tokens)
}

// ---------------------------------------------------------------------------
// VP rendering — shared by main clauses and relative clauses.
// ---------------------------------------------------------------------------

/// Render one verb phrase with all checks — valence, government,
/// aspect — applied identically wherever a VP appears (main clause,
/// conjunct, relative clause). The resolved VP already owns its one
/// authoritative object case.
#[allow(clippy::too_many_arguments)]
fn render_vp(
    verb_phrase: &ResolvedVerbPhrase,
    path: &str,
    shape: &ClauseShape,
    person: Person,
    number: Number,
    gender: Gender,
    subject_animacy: Animacy,
    recipient_clitics: CliticContext,
    object_clitics: CliticContext,
    ctx: &mut Ctx,
) -> Result<VerbDomainPlan, PhraseError> {
    // Adverbs precede the verb complex by default (POLICY). Steen's
    // optative example fixes the opposite order (`nehaj žive dolgo`),
    // so that force keeps the adverb in the same constituent but places
    // it after the finite form.
    let mut complex: Vec<SurfaceNode> = if shape.force == Force::Optative {
        Vec::new()
    } else {
        verb_phrase.adverbs.iter().map(word).collect()
    };
    complex.extend(build_complex(
        path,
        &verb_phrase.bare_verb,
        verb_phrase.info.as_ref(),
        shape,
        person,
        number,
        gender,
        subject_animacy,
        &mut ctx.warnings,
    )?);
    if shape.force == Force::Optative {
        complex.extend(verb_phrase.adverbs.iter().map(word));
    }

    let mut cluster: Vec<String> = Vec::new();
    let mut recipient = None;
    if let Some(recipient_nominal) = &verb_phrase.recipient {
        let mut rendered = render_nominal(
            recipient_nominal,
            PronounStyle::Full,
            recipient_clitics,
            ctx,
        )?;
        if let Some(clitic) = rendered.direct_clitic.take() {
            debug_assert_eq!(
                recipient_nominal.case,
                Case::Dat,
                "the recipient edge owns dative case"
            );
            cluster.push(clitic);
        } else {
            recipient = Some(rendered);
        }
    }
    let mut object = None;
    if let Some(object_nominal) = &verb_phrase.object {
        let mut rendered = render_nominal(object_nominal, PronounStyle::Full, object_clitics, ctx)?;
        // Only the object nominal itself can contribute a clitic. Its
        // descendants are opaque inside `body`, so relative-clause
        // clitics cannot migrate into this domain.
        if let Some(clitic) = rendered.direct_clitic.take() {
            cluster.push(clitic);
        } else {
            object = Some(rendered);
        }
    }
    if verb_phrase.reflexive {
        cluster.push("sę".to_string());
    }

    let mut adjuncts = Vec::new();
    for adjunct in &verb_phrase.pps {
        adjuncts.push(render_pp(adjunct, ctx)?);
    }
    for oblique in &verb_phrase.obliques {
        adjuncts.push(
            render_nominal(oblique, PronounStyle::Full, CliticContext::ForceFull, ctx)?
                .into_surface(),
        );
    }

    // The complement clause is planned as a whole clause and sealed, so
    // its clitics are already placed inside it and cannot join this
    // domain's cluster.
    let opts = ctx.opts;
    let complement_clause = match &verb_phrase.complement_clause {
        Some(sub) => Some(vec![SurfaceNode::Subordinate(Box::new(
            render_subordinate(sub, &format!("{path}.complement_clause"), opts, ctx)?,
        ))]),
        None => None,
    };

    Ok(VerbDomainPlan {
        complex,
        cluster,
        recipient,
        object,
        object_case: verb_phrase.object_case,
        adjuncts,
        complement_clause,
    })
}

// ---------------------------------------------------------------------------
// Relative clauses — through the same VP machinery.
// ---------------------------------------------------------------------------

/// The relative clause: comma, (preposition,) relativizer agreeing with
/// the head in gender/number and taking its case from the gap role, then
/// the clause body rendered by the SAME VP machinery as main clauses
/// (valence, government, adverbs, aspect warnings all apply), then a
/// closing comma. Comma-delimited relatives are POLICY (pan-Slavic
/// convention; steen shows no relative punctuation examples). The
/// relative is a fresh clitic domain: its cluster attaches postverbally
/// inside it (POLICY).
fn render_relative(
    rel: &ResolvedRelative,
    head_number: Number,
    head_gender: Gender,
    head_animacy: Animacy,
    ctx: &mut Ctx,
) -> Result<RelativePlan, PhraseError> {
    if rel.relativizer == Relativizer::Iže {
        return Err(PhraseError::Unsupported {
            path: format!("{}.relativizer", rel.path),
            feature: "the iže relativizer (no facade paradigm)",
        });
    }
    let relativizer = pronoun(
        "ktory",
        rel.gap_case,
        head_number,
        head_gender,
        head_animacy,
    )
    .expect("ktory declines for every cell");

    let mut body = vec![SurfaceNode::Punct(',')];
    if let GapRole::PpObject { preposition, .. } = &rel.gap {
        body.push(word(preposition.clone()));
    }
    body.push(word(surface(&relativizer)));

    // Agreement inside the relative: a subject gap agrees with the head
    // (3rd person); otherwise with the overt subject.
    let (person, number, gender, subject_nodes) = match (&rel.gap, &rel.subject) {
        (GapRole::Subject, _) => (Person::Third, head_number, head_gender, Vec::new()),
        (_, Some(subject)) => {
            let rendered =
                render_nominal(subject, PronounStyle::Full, CliticContext::ForceFull, ctx)?;
            (
                rendered.profile.person,
                rendered.profile.agreement_number,
                rendered.profile.agreement_gender,
                rendered.into_surface(),
            )
        }
        (_, None) => unreachable!("relative subject requirements were validated"),
    };
    body.extend(subject_nodes);

    let shape = ClauseShape {
        force: Force::Declarative,
        mood: Mood::Indicative,
        voice: Voice::Active,
        tense: rel.tense,
        polarity: rel.polarity,
    };
    let vp = render_vp(
        &rel.vp,
        &format!("{}.vp", rel.path),
        &shape,
        person,
        number,
        gender,
        head_animacy,
        CliticContext::Allowed,
        CliticContext::Allowed,
        ctx,
    )?;
    body.extend(vp.complex);
    body.extend(vp.cluster.into_iter().map(word));
    if let Some(recipient) = vp.recipient {
        body.extend(recipient.into_surface());
    }
    if let Some(object) = vp.object {
        body.extend(object.into_surface());
    }
    for adjunct in vp.adjuncts {
        body.extend(adjunct);
    }
    if let Some(nodes) = vp.complement_clause {
        body.extend(nodes);
    }
    body.push(SurfaceNode::Punct(','));
    Ok(RelativePlan { body })
}

// ---------------------------------------------------------------------------
// Clause realization.
// ---------------------------------------------------------------------------

/// Is this nominal's rendered core (head + agreeing modifiers, relatives
/// excluded — their words are case-invariant) identical between Nom and
/// Acc? PURE: no context, no warnings, no relative rendering.
fn nom_acc_syncretic(nominal: &ResolvedNominal) -> bool {
    fn np_core(
        np: &NounPhrase,
        profile: crate::profile::NominalProfile,
        case: Case,
    ) -> Vec<String> {
        let info = interslavic::noun_info(&np.head);
        let (gender, animacy) = (info.gender, info.animacy);
        let number = profile.inflection_number;
        let modifier = |lemma: &str, modifier_case: Case, modifier_number: Number| -> String {
            pronoun(lemma, modifier_case, modifier_number, gender, animacy)
                .unwrap_or_else(|| adj(lemma, modifier_case, modifier_number, gender, animacy))
        };
        let mut out = Vec::new();
        if let Some(n) = np.count {
            let parts =
                quantified_parts_with_info(n, &np.head, case, gender, animacy, info.plural_only);
            if let Some(det) = &np.determiner {
                out.push(surface(&modifier(det, parts.case, parts.number)));
            }
            for adjective in &np.adjectives {
                out.push(surface(&modifier(adjective, parts.case, parts.number)));
            }
            out.push(surface(&parts.noun));
        } else {
            if let Some(det) = &np.determiner {
                out.push(surface(&modifier(det, case, number)));
            }
            for adjective in &np.adjectives {
                out.push(surface(&modifier(adjective, case, number)));
            }
            out.push(surface(&noun_with(&np.head, case, number, gender, animacy)));
        }
        out
    }
    let pronoun_is_syncretic = |person, number, gender| {
        let form = |case| {
            personal_pronoun(person, number, gender, case, PronounStyle::Full)
                .expect("full personal-pronoun cells are total")
        };
        form(Case::Nom) == form(Case::Acc)
    };
    match &nominal.kind {
        ResolvedNominalKind::Pron {
            person,
            number,
            gender,
            ..
        } => pronoun_is_syncretic(*person, *number, *gender),
        ResolvedNominalKind::Name {
            text,
            gender,
            indeclinable,
        } => {
            *indeclinable
                || surface(&noun_with(
                    text,
                    Case::Nom,
                    Number::Singular,
                    *gender,
                    Animacy::Animate,
                )) == surface(&noun_with(
                    text,
                    Case::Acc,
                    Number::Singular,
                    *gender,
                    Animacy::Animate,
                ))
        }
        ResolvedNominalKind::Np { source, .. } if source.referential != ReferentialForm::Full => {
            pronoun_is_syncretic(
                Person::Third,
                nominal.profile.referent_number,
                nominal.profile.gender,
            )
        }
        ResolvedNominalKind::Np { source, .. } => {
            np_core(source, nominal.profile, Case::Nom)
                == np_core(source, nominal.profile, Case::Acc)
        }
        ResolvedNominalKind::Coord { items, .. } => items.iter().all(nom_acc_syncretic),
    }
}

/// Realize a clause to text + warnings — the checked entry point.
pub fn realize_checked(clause: &Clause, opts: RealizeOpts) -> Result<Realized, PhraseError> {
    realize_with_lead_in(clause, None, opts)
}

/// [`realize_checked`] with an optional sentence-initial lead-in word
/// (discourse connectives). The lead-in flows through the SAME
/// capitalization and punctuation pipeline as everything else — this is
/// the only sentence-assembly entry point in the crate.
pub fn realize_with_lead_in(
    clause: &Clause,
    lead_in: Option<&str>,
    opts: RealizeOpts,
) -> Result<Realized, PhraseError> {
    let validated = validate(clause)?;
    realize_validated_with_lead_in(&validated, lead_in, opts)
}

/// Realize an already validated clause. This makes the
/// validation→resolution→planning boundary available to callers that
/// retain validated trees.
pub fn realize_validated_checked(
    clause: &crate::validate::ValidatedClause,
    opts: RealizeOpts,
) -> Result<Realized, PhraseError> {
    realize_validated_with_lead_in(clause, None, opts)
}

pub(crate) fn realize_validated_with_lead_in(
    validated: &crate::validate::ValidatedClause,
    lead_in: Option<&str>,
    opts: RealizeOpts,
) -> Result<Realized, PhraseError> {
    let resolution = resolve(validated)?;
    let mut ctx = Ctx {
        opts: &opts,
        warnings: resolution
            .conflicts
            .iter()
            .map(|conflict| PhraseWarning::GovernsConflict {
                path: conflict.path.clone(),
                verb: conflict.verb.clone(),
                dictionary: conflict.dictionary,
                used: conflict.used,
            })
            .collect(),
    };
    let clause = &resolution.clause;
    let constituents = plan_clause(clause, "clause", &opts, &mut ctx)?;

    // --- The single recursive flatten + stringification ----------------
    let text = ClausePlan {
        lead_in: lead_in.map(str::to_string),
        constituents,
        force: clause.force,
        sentence: opts.sentence,
    }
    .stringify();
    Ok(Realized {
        text,
        warnings: ctx.warnings,
    })
}

/// Plan ONE clause into labeled constituents — the single implementation
/// for matrix clauses and every embedded clause.
///
/// Subordinate clauses recurse through here, so an embedded clause gets
/// the same agreement, ordering, information structure, and clitic
/// placement as a matrix clause. Crucially, each call places its OWN
/// clitic clusters into its OWN constituent vector before returning; the
/// result is then sealed into an opaque `SurfaceNode::Subordinate`. A
/// matrix verb therefore cannot extract a clitic from inside an embedded
/// clause, exactly as it cannot from a relative.
///
/// Terminal punctuation and sentence-initial capitalization are NOT done
/// here: they belong to `ClausePlan::stringify`, which runs once, at the
/// top level only. That is what keeps an embedded clause from acquiring a
/// sentence's full stop or a capital letter mid-sentence.
fn plan_clause(
    clause: &ResolvedClause,
    path: &str,
    opts: &RealizeOpts,
    ctx: &mut Ctx,
) -> Result<Vec<Constituent>, PhraseError> {
    let shape = ClauseShape {
        force: clause.force,
        mood: clause.mood,
        voice: clause.voice,
        tense: clause.tense,
        polarity: clause.polarity,
    };
    let imperative = matches!(clause.force, Force::Imperative(_));

    let subject = render_nominal(
        &clause.subject,
        PronounStyle::Full,
        CliticContext::ForceFull,
        ctx,
    )?;
    let (person, number, gender) = if let Force::Imperative(addressee) = clause.force {
        match addressee {
            Addressee::You => (Person::Second, Number::Singular, Gender::Masculine),
            Addressee::We => (Person::First, Number::Plural, Gender::Masculine),
            Addressee::YouAll => (Person::Second, Number::Plural, Gender::Masculine),
        }
    } else {
        (
            subject.profile.person,
            subject.profile.agreement_number,
            subject.profile.agreement_gender,
        )
    };

    // Information-structure marking = stress: a marked complement
    // renders a full pronoun form (a clitic cannot be topicalized or
    // focused).
    let recipient_marked = clause.topic == Some(SlotRef::Recipient)
        || clause.focus == Some(SlotRef::Recipient)
        || clause.wh == Some(WhFront::Slot(SlotRef::Recipient));
    let object_marked = clause.topic == Some(SlotRef::Object)
        || clause.focus == Some(SlotRef::Object)
        || clause.wh == Some(WhFront::Slot(SlotRef::Object));
    let information_recipient_index = match &clause.core {
        ResolvedCore::Verbal { vps, .. } => vps.iter().position(|vp| vp.recipient.is_some()),
        ResolvedCore::Copular(_) => None,
    };
    let information_object_index = match &clause.core {
        ResolvedCore::Verbal { vps, .. } => vps.iter().position(|vp| vp.object.is_some()),
        ResolvedCore::Copular(_) => Some(0),
    };
    let information_recipient_slot = information_recipient_index.map(SlotKind::Recipient);
    let information_object_slot = information_object_index.map(SlotKind::Object);

    // Build labeled constituents.
    let mut constituents: Vec<Constituent> = Vec::new();
    for (index, adjunct) in clause.initial_participles.iter().enumerate() {
        let participle =
            active_adverbial_participle(&adjunct.verb).ok_or(PhraseError::Unsupported {
                path: format!("clause.initial_participle[{index}].verb"),
                feature: "no present active adverbial participle (perfective verb?)",
            })?;
        let mut nodes = vec![word(surface(&participle))];
        for pp in &adjunct.pps {
            nodes.extend(render_pp(pp, ctx)?);
        }
        nodes.push(SurfaceNode::Punct(','));
        constituents.push(Constituent {
            slot: SlotKind::InitialAdjunct(index),
            nodes,
        });
    }
    if !imperative && !clause.prodrop {
        constituents.push(Constituent {
            slot: SlotKind::Subject,
            nodes: subject.clone().into_surface(),
        });
    }

    // Per-VP clusters, held aside for placement after ordering.
    let mut clusters: Vec<(usize, Vec<String>)> = Vec::new();
    // The case of the generic information-structure object. The
    // syncretism guard fires only for a genuinely accusative object.
    let mut information_object_case: Option<Case> = None;

    match &clause.core {
        ResolvedCore::Copular(predicate) => {
            let complex = build_complex(
                "clause.core.copula",
                "byti",
                None,
                &shape,
                person,
                number,
                gender,
                subject.profile.animacy,
                &mut ctx.warnings,
            )?;
            constituents.push(Constituent {
                slot: SlotKind::Verb(0),
                nodes: complex,
            });
            let nodes = match predicate {
                ResolvedPredicate::Nominal(nominal) => {
                    render_nominal(nominal, PronounStyle::Full, CliticContext::ForceFull, ctx)?
                        .into_surface()
                }
                ResolvedPredicate::Adjectival(adjective) => vec![word(surface(&adj(
                    adjective,
                    Case::Nom,
                    number,
                    gender,
                    subject.profile.animacy,
                )))],
                ResolvedPredicate::ShortAdjectival(adjective) => vec![word(surface(&short_adj(
                    adjective,
                    Case::Nom,
                    number,
                    gender,
                    subject.profile.animacy,
                )))],
                ResolvedPredicate::Participial(infinitive) => {
                    let participle = passive_participle(
                        infinitive,
                        Case::Nom,
                        number,
                        gender,
                        subject.profile.animacy,
                    )
                    .ok_or(PhraseError::Unsupported {
                        path: "clause.core.predicate".to_string(),
                        feature: "no passive participle (intransitive verb?)",
                    })?;
                    vec![word(surface(&participle))]
                }
            };
            constituents.push(Constituent {
                slot: SlotKind::Object(0),
                nodes,
            });
        }
        ResolvedCore::Verbal { conjunction, vps } => {
            for (index, verb_phrase) in vps.iter().enumerate() {
                if index > 0 {
                    constituents.push(Constituent {
                        slot: SlotKind::Fixed,
                        nodes: vec![if index == vps.len() - 1 {
                            word(conjunction.word())
                        } else {
                            SurfaceNode::Punct(',')
                        }],
                    });
                }
                let vp = render_vp(
                    verb_phrase,
                    &format!("clause.core.vp[{index}]"),
                    &shape,
                    person,
                    number,
                    gender,
                    subject.profile.animacy,
                    if recipient_marked && information_recipient_index == Some(index) {
                        CliticContext::ForceFull
                    } else {
                        CliticContext::Allowed
                    },
                    if object_marked && information_object_index == Some(index) {
                        CliticContext::ForceFull
                    } else {
                        CliticContext::Allowed
                    },
                    ctx,
                )?;
                constituents.push(Constituent {
                    slot: SlotKind::Verb(index),
                    nodes: vp.complex,
                });
                if information_object_index == Some(index) {
                    information_object_case = vp.object_case;
                }
                if !vp.cluster.is_empty() {
                    clusters.push((index, vp.cluster));
                }
                if let Some(recipient) = vp.recipient {
                    constituents.push(Constituent {
                        slot: SlotKind::Recipient(index),
                        nodes: recipient.into_surface(),
                    });
                }
                if let Some(object) = vp.object {
                    constituents.push(Constituent {
                        slot: SlotKind::Object(index),
                        nodes: object.into_surface(),
                    });
                }
                for adjunct in vp.adjuncts {
                    constituents.push(Constituent {
                        slot: SlotKind::Fixed,
                        nodes: adjunct,
                    });
                }
                if let Some(nodes) = vp.complement_clause {
                    constituents.push(Constituent {
                        slot: SlotKind::Fixed,
                        nodes,
                    });
                }
            }
        }
    }

    // --- Ordering (labels, not strings) --------------------------------
    order_constituents(
        &mut constituents,
        clause,
        information_recipient_slot.unwrap_or(SlotKind::Recipient(0)),
        information_object_slot.unwrap_or(SlotKind::Object(0)),
    );
    if let Some(WhFront::Adverb(adverb)) = &clause.wh {
        let index = constituents
            .iter()
            .take_while(|item| matches!(item.slot, SlotKind::InitialAdjunct(_)))
            .count();
        constituents.insert(
            index,
            Constituent {
                slot: SlotKind::Fixed,
                nodes: vec![word(adverb.clone())],
            },
        );
    }
    if clause.force == Force::Optative {
        let index = constituents
            .iter()
            .take_while(|item| matches!(item.slot, SlotKind::InitialAdjunct(_)))
            .count();
        constituents.insert(
            index,
            Constituent {
                slot: SlotKind::Fixed,
                nodes: vec![word("nehaj")],
            },
        );
    }

    // Syncretism guard on the ACTUAL final order: warn only when subject
    // and the information object genuinely inverted, the object was rendered in
    // the accusative (an oblique form already disambiguates), and both
    // are Nom/Acc-ambiguous (steen's clarity caveat). Pure probe: no
    // re-rendering side effects.
    let position = |slot: SlotKind| constituents.iter().position(|c| c.slot == slot);
    if let (Some(subject_at), Some(object_at)) = (
        position(SlotKind::Subject),
        information_object_slot.and_then(position),
    ) {
        if object_at < subject_at
            && information_object_case == Some(Case::Acc)
            && nom_acc_syncretic(&clause.subject)
            && resolved_information_object(clause, information_object_index)
                .is_some_and(nom_acc_syncretic)
        {
            ctx.warnings.push(PhraseWarning::AmbiguousOrder {
                path: "clause.order".to_string(),
            });
        }
    }

    // --- li, či, and clitic placement (structural) ----------------------
    if clause.force == Force::LiQuestion {
        let focus_slot = match clause.focus {
            Some(SlotRef::Recipient) => {
                information_recipient_slot.expect("validated recipient focus has a target")
            }
            Some(SlotRef::Object) => {
                information_object_slot.expect("validated object focus has a target")
            }
            Some(SlotRef::Subject) => SlotKind::Subject,
            None => SlotKind::Verb(0),
        };
        let index = constituents
            .iter()
            .position(|c| c.slot == focus_slot)
            .expect("focus references validated above; Verb(0) always exists");
        constituents.insert(
            index + 1,
            Constituent {
                slot: SlotKind::QuestionParticle(QuestionParticle::Li),
                nodes: vec![word("li")],
            },
        );
    }
    // `či` joins the constituent list BEFORE cluster placement: the
    // fronted particle is the clause's first constituent and hosts a
    // second-position cluster ("Či sę krålj myl?" — POLICY, parallel to
    // the pan-Slavic particle-as-host pattern).
    if clause.force == Force::CiQuestion {
        let index = constituents
            .iter()
            .take_while(|item| matches!(item.slot, SlotKind::InitialAdjunct(_)))
            .count();
        constituents.insert(
            index,
            Constituent {
                slot: SlotKind::QuestionParticle(QuestionParticle::Ci),
                nodes: vec![word("či")],
            },
        );
    }
    for (vp_index, cluster) in clusters {
        place_cluster(&mut constituents, vp_index, cluster, opts.clitic_style);
    }

    // Clause-level adverbial subordinates. Each is planned as a complete
    // clause and sealed, so its clitics are already placed inside it.
    for (index, adjunct) in clause.adverbial_clauses.iter().enumerate() {
        let plan = render_subordinate(
            adjunct,
            &format!("{path}.adverbial_clause[{index}]"),
            opts,
            ctx,
        )?;
        let nodes = vec![SurfaceNode::Subordinate(Box::new(plan))];
        match adjunct.position {
            AdjunctPosition::Initial => {
                let at = leading_adjunct_count(&constituents);
                constituents.insert(
                    at,
                    Constituent {
                        slot: SlotKind::InitialAdjunct(usize::MAX - index),
                        nodes,
                    },
                );
            }
            AdjunctPosition::Final => constituents.push(Constituent {
                slot: SlotKind::Fixed,
                nodes,
            }),
        }
    }

    Ok(constituents)
}

/// How many constituents at the front are clause-initial adjuncts. The
/// fronted particles (`či`, `nehaj`, a wh-adverb) and a second-position
/// clitic cluster all attach after them.
fn leading_adjunct_count(constituents: &[Constituent]) -> usize {
    constituents
        .iter()
        .take_while(|item| matches!(item.slot, SlotKind::InitialAdjunct(_)))
        .count()
}

/// Render a subordinate clause: the comma on the inside edge, the
/// complementizer, and the embedded clause planned by the same
/// `plan_clause` used for matrix clauses.
///
/// Comma placement is structural, not textual: a fronted adverbial takes
/// its comma at the end (`Ale kȯgda ljudi prěměstili sę …, oni našli …`),
/// everything else takes it at the front (`uviděl, že ide ljėv`). Because
/// the comma is a `SurfaceNode::Punct`, the single `join_flat` pass
/// handles spacing and collapses a boundary that coincides with another.
fn render_subordinate(
    sub: &ResolvedSubClause,
    path: &str,
    opts: &RealizeOpts,
    ctx: &mut Ctx,
) -> Result<SubordinatePlan, PhraseError> {
    let inner = plan_clause(&sub.clause, path, opts, ctx)?;
    let mut body = Vec::new();
    if sub.position == AdjunctPosition::Final {
        body.push(SurfaceNode::Punct(','));
    }
    body.extend(
        sub.complementizer
            .words()
            .iter()
            .map(|word_text| word(*word_text)),
    );
    for constituent in inner {
        body.extend(constituent.nodes);
    }
    if sub.position == AdjunctPosition::Initial {
        body.push(SurfaceNode::Punct(','));
    }
    Ok(SubordinatePlan { body })
}

/// Realize a clause — the warnings-discarding convenience.
///
/// ```
/// use interslavic_phrase::*;
///
/// let tree = clause(
///     np("krålj").det("toj").adj("dobry"),
///     vp("ukrasti").object(np("moneta").count(5).adj("zlåty")),
/// )
/// .past();
/// assert_eq!(
///     realize(&tree, RealizeOpts::sentence()).unwrap(),
///     "Toj dobry krålj ukradl 5 zlåtyh monet."
/// );
///
/// let tree = clause(np("kot").count(5), vp("spati"));
/// assert_eq!(realize(&tree, RealizeOpts::sentence()).unwrap(), "5 kotov spi.");
///
/// let tree = clause(np("kot"), vp("spati").pp(pp("pod", np("stol"))));
/// let err = realize(&tree, RealizeOpts::sentence()).unwrap_err();
/// assert!(matches!(err, PhraseError::Validation(_)));
/// ```
pub fn realize(clause: &Clause, opts: RealizeOpts) -> Result<String, PhraseError> {
    realize_checked(clause, opts).map(|realized| realized.text)
}

fn resolved_information_object(
    clause: &ResolvedClause,
    index: Option<usize>,
) -> Option<&ResolvedNominal> {
    match &clause.core {
        ResolvedCore::Verbal { vps, .. } => index.and_then(|index| vps.get(index)?.object.as_ref()),
        ResolvedCore::Copular(_) => None,
    }
}

/// Order constituents by information structure: default S V recipient
/// O; `:topic` fronts its slot, `:focus` moves its slot last
/// (theme-first, rheme-last — functional sentence perspective).
/// LiQuestions front an explicit topic, then the focused slot (default:
/// the verb), and follow with verb–subject–recipient–object (extending
/// Steen's verb–subject–object example order).
fn order_constituents(
    constituents: &mut Vec<Constituent>,
    clause: &ResolvedClause,
    information_recipient_slot: SlotKind,
    information_object_slot: SlotKind,
) {
    let mut initial = Vec::new();
    let mut index = 0;
    while index < constituents.len() {
        if matches!(constituents[index].slot, SlotKind::InitialAdjunct(_)) {
            initial.push(constituents.remove(index));
        } else {
            index += 1;
        }
    }

    let take = |constituents: &mut Vec<Constituent>, slot: SlotKind| -> Option<Constituent> {
        constituents
            .iter()
            .position(|c| c.slot == slot)
            .map(|index| constituents.remove(index))
    };
    let slot_of = |slot: SlotRef| match slot {
        SlotRef::Subject => SlotKind::Subject,
        SlotRef::Recipient => information_recipient_slot,
        SlotRef::Object => information_object_slot,
    };

    if let Some(WhFront::Slot(slot)) = &clause.wh {
        let front = take(constituents, slot_of(*slot));
        let mut ordered = Vec::new();
        ordered.extend(front);
        if *slot == SlotRef::Object {
            ordered.extend(take(constituents, SlotKind::Verb(0)));
            ordered.extend(take(constituents, SlotKind::Subject));
            ordered.extend(take(constituents, SlotKind::Recipient(0)));
        }
        ordered.append(constituents);
        initial.append(&mut ordered);
        *constituents = initial;
        return;
    }

    if clause.force == Force::LiQuestion {
        let topic = clause
            .topic
            .and_then(|topic| take(constituents, slot_of(topic)));
        let focus_slot = clause.focus.map(slot_of).unwrap_or(SlotKind::Verb(0));
        let focus = take(constituents, focus_slot);
        let verb = take(constituents, SlotKind::Verb(0));
        let subject = take(constituents, SlotKind::Subject);
        let recipient = take(constituents, SlotKind::Recipient(0));
        // Only VP0's object belongs to the special default
        // verb0–subject–recipient–object sequence. Later complements
        // stay with their own conjunct unless explicitly selected as
        // topic/focus.
        let object = take(constituents, SlotKind::Object(0));
        let mut ordered = Vec::new();
        ordered.extend(topic);
        ordered.extend(focus);
        for item in [verb, subject, recipient, object].into_iter().flatten() {
            ordered.push(item);
        }
        ordered.append(constituents);
        initial.append(&mut ordered);
        *constituents = initial;
        return;
    }

    if let Some(topic) = clause.topic {
        if let Some(constituent) = take(constituents, slot_of(topic)) {
            constituents.insert(0, constituent);
        }
    }
    if let Some(focus) = clause.focus {
        if let Some(constituent) = take(constituents, slot_of(focus)) {
            constituents.push(constituent);
        }
    }
    initial.append(constituents);
    *constituents = initial;
}

/// Place one VP's clitic cluster structurally. Postverbal: directly
/// after that VP's complex constituent (and after a directly following
/// `li` — li is first in the cluster, Franks & King order). Second
/// position: after the first constituent of the cluster's domain — the
/// clause for VP 0, the conjunct (verb-initial) for later VPs.
fn place_cluster(
    constituents: &mut Vec<Constituent>,
    vp_index: usize,
    cluster: Vec<String>,
    style: CliticStyle,
) {
    let is_li_particle = |constituent: &Constituent| {
        constituent.slot == SlotKind::QuestionParticle(QuestionParticle::Li)
    };
    let after_li = |constituents: &[Constituent], mut index: usize| -> usize {
        if constituents.get(index).is_some_and(is_li_particle) {
            index += 1;
        }
        index
    };
    let insert_at = match (style, vp_index) {
        (CliticStyle::SecondPosition, 0) => {
            let domain_start = constituents
                .iter()
                .take_while(|item| matches!(item.slot, SlotKind::InitialAdjunct(_)))
                .count();
            constituents
                .iter()
                .enumerate()
                .skip(domain_start)
                .find(|(_, constituent)| is_li_particle(constituent))
                .map_or_else(
                    || (domain_start + 1).min(constituents.len()),
                    |(li_at, _)| after_li(constituents, li_at),
                )
        }
        _ => match constituents
            .iter()
            .position(|c| c.slot == SlotKind::Verb(vp_index))
        {
            Some(verb_at) => after_li(constituents, verb_at + 1),
            // The complex must exist; if a future refactor removes it,
            // the clitics still surface (misplaced, never dropped).
            None => constituents.len(),
        },
    };
    constituents.insert(
        insert_at,
        Constituent {
            slot: SlotKind::Fixed,
            nodes: cluster.into_iter().map(word).collect(),
        },
    );
}
