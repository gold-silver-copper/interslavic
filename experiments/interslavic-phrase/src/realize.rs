//! Agreement/government resolution and linearization.
//!
//! Every inflected form comes from the `interslavic` facade — this module
//! never post-processes forms. Exactly two string operations happen after
//! forms leave the facade, both documented here: punctuation attachment
//! (commas and terminal marks, handled as typed [`Token::Punct`] — never
//! edits to word forms) and sentence-initial capitalization of the first
//! character in sentence mode.
//!
//! Linearization is TYPED until the final join: the clause becomes a list
//! of labeled [`Constituent`]s whose tokens distinguish words, clitics,
//! and punctuation. Placement rules (information structure, `li`, clitic
//! clusters) operate on that structure — nothing is ever re-discovered by
//! searching strings, and a clitic is a token the join must render, so it
//! cannot be silently dropped.
//!
//! Linearization defaults are steen's (syntax page): S–V–O neutral;
//! "modifiers usually precede the noun"; postverbal clitics (steen's own
//! examples); `li` "right after the focus point of the question, usually
//! the verb"; non-SVO orders sanctioned "when special emphasis is
//! needed", with steen's clarity caveat surfaced as a warning when
//! Nom/Acc syncretism would garden-path. Adverb position (before the
//! verb) is POLICY — the sources are silent.

use crate::ast::*;
use interslavic::{
    Animacy, Aspect, Case, Gender, Number, Person, PronounStyle, Provenance, Tense, VerbInfo, adj,
    cells, conditional_parts, noun_with, passive_participle, perfect_parts, personal_pronoun,
    preposition_cases, preposition_senses, pronoun, quantified_parts_with_info, verb, verb_forms,
    verb_info,
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
    UnknownPreposition(String),
    AmbiguousPreposition {
        preposition: String,
        senses: Vec<(Case, &'static str)>,
    },
    InvalidPrepositionCase {
        preposition: String,
        case: Case,
        allowed: Vec<Case>,
    },
    ObjectOfIntransitive {
        verb: String,
    },
    /// A passive clause whose verb phrase still carries an object — in
    /// the passive the patient IS the subject.
    ObjectInPassive {
        verb: String,
    },
    GuessedHead {
        lemma: String,
    },
    /// Coordination with no items, or another structurally empty node.
    EmptyCoordination,
    /// A Force/Mood/Voice combination this crate does not realize.
    /// Decided, never silently dropped.
    IncoherentClause(&'static str),
    Unsupported(&'static str),
}

impl fmt::Display for PhraseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PhraseError::UnknownPreposition(p) => write!(f, "unknown preposition `{p}`"),
            PhraseError::AmbiguousPreposition {
                preposition,
                senses,
            } => {
                write!(
                    f,
                    "preposition `{preposition}` governs several cases; pick one:"
                )?;
                for (case, gloss) in senses {
                    write!(f, " {case:?} = \"{gloss}\";")?;
                }
                Ok(())
            }
            PhraseError::InvalidPrepositionCase {
                preposition,
                case,
                allowed,
            } => write!(
                f,
                "`{preposition}` does not govern {case:?} (allowed: {allowed:?})"
            ),
            PhraseError::ObjectOfIntransitive { verb } => {
                write!(
                    f,
                    "`{verb}` is intransitive in the dictionary but has a direct object"
                )
            }
            PhraseError::ObjectInPassive { verb } => {
                write!(
                    f,
                    "passive clause of `{verb}` carries an object; the patient is the subject"
                )
            }
            PhraseError::GuessedHead { lemma } => {
                write!(
                    f,
                    "`{lemma}` is not a dictionary noun (gender/animacy would be guessed)"
                )
            }
            PhraseError::EmptyCoordination => write!(f, "coordination with no items"),
            PhraseError::IncoherentClause(what) => write!(f, "incoherent clause: {what}"),
            PhraseError::Unsupported(what) => write!(f, "unsupported: {what}"),
        }
    }
}

impl std::error::Error for PhraseError {}

/// Warnings never change output; they surface steen's caveats and the
/// dictionary's opinions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhraseWarning {
    /// A perfective verb in the present tense reads as future/completed
    /// in Slavic — grammatical, but probably not "present".
    PerfectivePresent { verb: String },
    /// An explicit object case contradicts the dictionary's government
    /// annotation.
    GovernsConflict {
        verb: String,
        dictionary: Case,
        used: Case,
    },
    /// The realized order inverts subject and object while neither
    /// distinguishes Nom from Acc on the surface — steen's own clarity
    /// caveat, detected on the ACTUAL final order.
    AmbiguousOrder,
}

/// The result of checked realization: the text plus any warnings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Realized {
    pub text: String,
    pub warnings: Vec<PhraseWarning>,
}

// ---------------------------------------------------------------------------
// The typed linearization representation.
// ---------------------------------------------------------------------------

/// One surface element. `Clitic` is a word the placement rules may move;
/// the join renders it like any word, so a clitic that placement misses
/// is still visible in the output — it can be misplaced, never dropped.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Word(String),
    Clitic(String),
    Punct(char),
}

/// Which movable slot a constituent fills. Information structure and the
/// syncretism guard reason over these labels, never over strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SlotKind {
    Subject,
    /// The n-th verb complex (0 = the one the clause fronts/focuses).
    Verb(usize),
    /// The n-th VP's object.
    Object(usize),
    /// Conjunctions, adjunct PPs, the fronted `či`, the `li` particle.
    Fixed,
}

#[derive(Debug, Clone)]
struct Constituent {
    slot: SlotKind,
    tokens: Vec<Token>,
}

fn word(text: impl Into<String>) -> Token {
    Token::Word(text.into())
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

/// A rendered nominal plus the agreement features it projects.
struct Rendered {
    tokens: Vec<Token>,
    person: Person,
    number: Number,
    gender: Gender,
    animacy: Animacy,
}

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
    nominal: &Nominal,
    slot_case: Case,
    style: PronounStyle,
    clitics: CliticContext,
    ctx: &mut Ctx,
) -> Result<Rendered, PhraseError> {
    match nominal {
        Nominal::Pron {
            person,
            number,
            gender,
            clitic,
        } => {
            let wants_clitic = *clitic
                && clitics == CliticContext::Allowed
                && style != PronounStyle::AfterPreposition;
            let clitic_form = if wants_clitic {
                personal_pronoun(*person, *number, *gender, slot_case, PronounStyle::Clitic)
            } else {
                None
            };
            let tokens = match clitic_form {
                Some(form) => vec![Token::Clitic(form)],
                None => {
                    let full = personal_pronoun(*person, *number, *gender, slot_case, style)
                        .or_else(|| {
                            personal_pronoun(
                                *person,
                                *number,
                                *gender,
                                slot_case,
                                PronounStyle::Full,
                            )
                        })
                        .expect("full personal-pronoun cells are total");
                    vec![word(full)]
                }
            };
            Ok(Rendered {
                tokens,
                person: *person,
                number: *number,
                gender: *gender,
                animacy: Animacy::Animate,
            })
        }
        Nominal::Name {
            text,
            gender,
            indeclinable,
        } => {
            let form = if *indeclinable {
                text.clone()
            } else {
                surface(&noun_with(
                    text,
                    slot_case,
                    Number::Singular,
                    *gender,
                    Animacy::Animate,
                ))
            };
            Ok(Rendered {
                tokens: vec![word(form)],
                person: Person::Third,
                number: Number::Singular,
                gender: *gender,
                animacy: Animacy::Animate,
            })
        }
        Nominal::Np(np) => render_np(np, slot_case, ctx),
        Nominal::Coord(coordination) => {
            if coordination.items.is_empty() {
                return Err(PhraseError::EmptyCoordination);
            }
            let mut rendered = Vec::new();
            for item in &coordination.items {
                // Conjuncts force full pronoun forms (a clitic cannot
                // carry the stress a conjunct bears).
                rendered.push(render_nominal(
                    item,
                    slot_case,
                    style,
                    CliticContext::ForceFull,
                    ctx,
                )?);
            }
            let mut tokens: Vec<Token> = Vec::new();
            let last = rendered.len() - 1;
            for (index, item) in rendered.iter().enumerate() {
                if index > 0 {
                    if index == last {
                        tokens.push(word(coordination.conjunction.word()));
                    } else {
                        // Serial style: comma between non-final items,
                        // conjunction only before the last; no comma
                        // before `i` (POLICY, Slavic convention).
                        tokens.push(Token::Punct(','));
                    }
                }
                tokens.extend(item.tokens.iter().cloned());
            }
            // Coordinated-subject agreement (POLICY, pan-Slavic
            // defaults): >1 conjunct → plural; mixed gender → masculine;
            // person resolves 1st > 2nd > 3rd; any animate conjunct
            // makes the group animate (the masculine-personal pattern),
            // an all-inanimate group stays inanimate — copular
            // adjectives and passive participles decline on this.
            let number =
                if rendered.len() > 1 || rendered.iter().any(|r| r.number == Number::Plural) {
                    Number::Plural
                } else {
                    Number::Singular
                };
            let gender = if rendered.iter().all(|r| r.gender == rendered[0].gender) {
                rendered[0].gender
            } else {
                Gender::Masculine
            };
            let person = rendered
                .iter()
                .map(|r| r.person)
                .min_by_key(|p| match p {
                    Person::First => 0,
                    Person::Second => 1,
                    Person::Third => 2,
                })
                .unwrap_or(Person::Third);
            let animacy = if rendered.iter().any(|r| r.animacy == Animacy::Animate) {
                Animacy::Animate
            } else {
                Animacy::Inanimate
            };
            Ok(Rendered {
                tokens,
                person,
                number,
                gender,
                animacy,
            })
        }
    }
}

fn render_np(np: &NounPhrase, slot_case: Case, ctx: &mut Ctx) -> Result<Rendered, PhraseError> {
    let slot_case = np.case_override.unwrap_or(slot_case);
    let info = interslavic::noun_info(&np.head);
    if ctx.opts.strict_guessed && info.provenance == Provenance::Guessed {
        return Err(PhraseError::GuessedHead {
            lemma: np.head.clone(),
        });
    }
    let (gender, animacy) = (info.gender, info.animacy);

    let modifier = |lemma: &str, case: Case, number: Number| -> String {
        pronoun(lemma, case, number, gender, animacy)
            .unwrap_or_else(|| adj(lemma, case, number, gender, animacy))
    };

    let mut tokens = Vec::new();
    let (number, agr_gender);
    if let Some(n) = np.count {
        let parts =
            quantified_parts_with_info(n, &np.head, slot_case, gender, animacy, info.plural_only);
        if let Some(det) = &np.determiner {
            tokens.push(word(surface(&modifier(det, parts.case, parts.number))));
        }
        tokens.push(word(n.to_string()));
        for adjective in &np.adjectives {
            tokens.push(word(surface(&modifier(
                adjective,
                parts.case,
                parts.number,
            ))));
        }
        tokens.push(word(surface(&parts.noun)));
        // Verb agreement with a counted subject: steen states 3sg-neuter
        // for collective subjects; extending it to all gen-pl quantified
        // subjects is POLICY.
        (number, agr_gender) = match n {
            1 => (Number::Singular, gender),
            2..=4 => (Number::Plural, gender),
            _ => (Number::Singular, Gender::Neuter),
        };
    } else {
        let noun_number = if info.plural_only {
            Number::Plural
        } else {
            Number::Singular
        };
        if let Some(det) = &np.determiner {
            tokens.push(word(surface(&modifier(det, slot_case, noun_number))));
        }
        for adjective in &np.adjectives {
            tokens.push(word(surface(&modifier(adjective, slot_case, noun_number))));
        }
        tokens.push(word(surface(&noun_with(
            &np.head,
            slot_case,
            noun_number,
            gender,
            animacy,
        ))));
        (number, agr_gender) = (noun_number, gender);
    }

    if let Some(rel) = &np.relative {
        tokens.extend(render_relative(rel, number, agr_gender, animacy, ctx)?);
    }

    Ok(Rendered {
        tokens,
        person: Person::Third,
        number,
        gender: agr_gender,
        animacy,
    })
}

/// Resolve a preposition plus an optional explicit case against the
/// curated table — the ONE government check for every PP the crate
/// realizes, adjunct or relative gap alike.
fn resolve_preposition_case(
    preposition: &str,
    explicit: Option<Case>,
) -> Result<Case, PhraseError> {
    let allowed = preposition_cases(preposition)
        .ok_or_else(|| PhraseError::UnknownPreposition(preposition.to_string()))?;
    match explicit {
        Some(case) if allowed.contains(&case) => Ok(case),
        Some(case) => Err(PhraseError::InvalidPrepositionCase {
            preposition: preposition.to_string(),
            case,
            allowed: allowed.to_vec(),
        }),
        None if allowed.len() == 1 => Ok(allowed[0]),
        None => Err(PhraseError::AmbiguousPreposition {
            preposition: preposition.to_string(),
            senses: preposition_senses(preposition)
                .expect("senses table agrees with cases table")
                .to_vec(),
        }),
    }
}

fn render_pp(pp: &PrepPhrase, ctx: &mut Ctx) -> Result<Vec<Token>, PhraseError> {
    let case = resolve_preposition_case(&pp.preposition, pp.case)?;
    let object = render_nominal(
        &pp.object,
        case,
        PronounStyle::AfterPreposition,
        CliticContext::ForceFull,
        ctx,
    )?;
    let mut tokens = vec![word(pp.preposition.clone())];
    tokens.extend(object.tokens);
    Ok(tokens)
}

// ---------------------------------------------------------------------------
// The one verb-complex builder.
// ---------------------------------------------------------------------------

fn strip_reflexive(verb_lemma: &str) -> (&str, bool) {
    match verb_lemma.strip_suffix(" sę") {
        Some(bare) => (bare, true),
        None => (verb_lemma, false),
    }
}

/// Dictionary metadata for THIS VP's construction: a reflexive VP
/// consults its own `X sę` row first (that row carries the reflexive
/// construction's government — "ostrěgati sę (+2)"), falling back to the
/// bare row; a plain VP reads the bare row only. Returns the bare lemma
/// (what the conjugator inflects), whether the lemma marked reflexivity,
/// and the selected metadata.
fn vp_verb_info(verb_lemma: &str) -> (&str, bool, Option<VerbInfo>) {
    let (bare, lemma_reflexive) = strip_reflexive(verb_lemma.trim());
    let info = if lemma_reflexive {
        verb_info(&format!("{bare} sę")).or_else(|| verb_info(bare))
    } else {
        verb_info(bare)
    };
    (bare, lemma_reflexive, info)
}

struct ClauseShape {
    force: Force,
    mood: Mood,
    voice: Voice,
    tense: TenseSpec,
    polarity: Polarity,
}

/// Build ONE finite verb complex — the single implementation for main
/// clauses, coordinated conjuncts, copular clauses (lemma `byti`),
/// relative clauses, imperatives, conditionals, and passives. Force,
/// Mood, and Voice coherence is decided here, in one place: every
/// combination either realizes or errors — none is silently ignored.
#[allow(clippy::too_many_arguments)]
fn build_complex(
    lemma: &str,
    info: Option<&VerbInfo>,
    shape: &ClauseShape,
    person: Person,
    number: Number,
    gender: Gender,
    subject_animacy: Animacy,
    warnings: &mut Vec<PhraseWarning>,
) -> Result<Vec<Token>, PhraseError> {
    let imperative = matches!(shape.force, Force::Imperative(_));

    // Coherence decisions (documented, not silent):
    if imperative && shape.mood == Mood::Conditional {
        return Err(PhraseError::IncoherentClause(
            "conditional imperative (no such construction)",
        ));
    }
    if imperative && shape.voice == Voice::Passive {
        // "Bųdi kupjen…" exists in Slavic but is unimplemented here —
        // declared, not silently active.
        return Err(PhraseError::Unsupported(
            "passive imperative (not yet implemented)",
        ));
    }
    if imperative && shape.tense != TenseSpec::Present {
        return Err(PhraseError::IncoherentClause(
            "imperative with past or future tense",
        ));
    }
    if shape.mood == Mood::Conditional && shape.tense != TenseSpec::Present {
        return Err(PhraseError::IncoherentClause(
            "conditional mood with an independently specified past or future tense",
        ));
    }

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

    if shape.voice == Voice::Passive {
        // The passive IS the participial copular construction: byti in
        // the clause's tense/mood plus the passive participle agreeing
        // with the subject.
        let copula_shape = ClauseShape {
            voice: Voice::Active,
            polarity: Polarity::Affirmative, // negation already emitted
            ..*shape
        };
        let mut copula = build_complex(
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
        let participle = passive_participle(lemma, Case::Nom, number, gender, subject_animacy)
            .ok_or(PhraseError::Unsupported(
                "no passive participle (intransitive verb?)",
            ))?;
        tokens.push(word(surface(&participle)));
        return Ok(tokens);
    }

    match shape.mood {
        Mood::Conditional => {
            let parts = conditional_parts(lemma, person, number, gender);
            tokens.push(word(parts.auxiliary));
            tokens.push(word(parts.participle));
        }
        Mood::Indicative => match shape.tense {
            TenseSpec::Present => {
                if info.and_then(|entry| entry.aspect) == Some(Aspect::Pf) {
                    warnings.push(PhraseWarning::PerfectivePresent {
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
        },
    }
    Ok(tokens)
}

// ---------------------------------------------------------------------------
// VP rendering — shared by main clauses and relative clauses.
// ---------------------------------------------------------------------------

/// A rendered verb phrase: the complex, its clitic cluster (in the cited
/// order dat > acc > sę; `li` is placed by the force rules and always
/// precedes the cluster), its object, and its adjuncts.
struct VpRender {
    complex: Vec<Token>,
    cluster: Vec<Token>,
    object: Option<Vec<Token>>,
    /// The case the object was ACTUALLY rendered in (override or
    /// dictionary government or the accusative default) — the
    /// syncretism guard reasons over this, never over an assumption.
    object_case: Option<Case>,
    adjuncts: Vec<Vec<Token>>,
}

/// Render one verb phrase with all checks — valence, government,
/// aspect — applied identically wherever a VP appears (main clause,
/// conjunct, relative clause). `gap` omits the gapped argument.
#[allow(clippy::too_many_arguments)]
fn render_vp(
    verb_phrase: &VerbPhrase,
    gap: Option<&GapRole>,
    shape: &ClauseShape,
    person: Person,
    number: Number,
    gender: Gender,
    subject_animacy: Animacy,
    object_clitics: CliticContext,
    ctx: &mut Ctx,
) -> Result<VpRender, PhraseError> {
    let (bare, lemma_reflexive, info) = vp_verb_info(&verb_phrase.verb);
    let reflexive = lemma_reflexive || info.as_ref().is_some_and(|entry| entry.reflexive);

    if gap == Some(&GapRole::Object) && verb_phrase.object.is_some() {
        return Err(PhraseError::IncoherentClause(
            "object-gap relative clause also supplies an object",
        ));
    }
    let object_present = verb_phrase.object.is_some();
    if object_present || gap == Some(&GapRole::Object) {
        if shape.voice == Voice::Passive {
            return Err(PhraseError::ObjectInPassive {
                verb: bare.to_string(),
            });
        }
        // "Intransitive" in the dictionary means no ACCUSATIVE object; a
        // (+N) government annotation licenses an oblique object.
        if info.as_ref().and_then(|entry| entry.transitive) == Some(false)
            && info.as_ref().and_then(|entry| entry.governs).is_none()
        {
            return Err(PhraseError::ObjectOfIntransitive {
                verb: bare.to_string(),
            });
        }
    }

    // Adverbs precede the verb complex (POLICY; the sources are silent
    // on neutral adverb position). They are part of the complex
    // constituent so placement can never split them from their verb.
    let mut complex: Vec<Token> = verb_phrase.adverbs.iter().map(word).collect();
    complex.extend(build_complex(
        bare,
        info.as_ref(),
        shape,
        person,
        number,
        gender,
        subject_animacy,
        &mut ctx.warnings,
    )?);

    let mut cluster: Vec<Token> = Vec::new();
    let mut object = None;
    let mut resolved_object_case = None;
    if object_present {
        let object_nominal = verb_phrase.object.as_ref().expect("checked");
        let dictionary = info.as_ref().and_then(|entry| entry.governs);
        let explicit = match object_nominal {
            Nominal::Np(np) => np.case_override,
            _ => None,
        };
        let object_case = match (explicit, dictionary) {
            (Some(used), Some(marked)) if used != marked => {
                ctx.warnings.push(PhraseWarning::GovernsConflict {
                    verb: bare.to_string(),
                    dictionary: marked,
                    used,
                });
                used
            }
            (Some(used), _) => used,
            (None, Some(marked)) => marked,
            (None, None) => Case::Acc,
        };
        resolved_object_case = Some(object_case);
        let rendered = render_nominal(
            object_nominal,
            object_case,
            PronounStyle::Full,
            object_clitics,
            ctx,
        )?;
        // A clitic-form object joins the cluster (dative before
        // accusative — the cited order); other tokens fill the object
        // slot.
        let (clitic_tokens, word_tokens): (Vec<_>, Vec<_>) = rendered
            .tokens
            .into_iter()
            .partition(|token| matches!(token, Token::Clitic(_)));
        if object_case == Case::Dat {
            let mut reordered = clitic_tokens;
            reordered.append(&mut cluster);
            cluster = reordered;
        } else {
            cluster.extend(clitic_tokens);
        }
        if !word_tokens.is_empty() {
            object = Some(word_tokens);
        }
    }
    if reflexive {
        cluster.push(Token::Clitic("sę".to_string()));
    }

    let mut adjuncts = Vec::new();
    for adjunct in &verb_phrase.pps {
        adjuncts.push(render_pp(adjunct, ctx)?);
    }

    Ok(VpRender {
        complex,
        cluster,
        object,
        object_case: resolved_object_case,
        adjuncts,
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
    rel: &RelClause,
    head_number: Number,
    head_gender: Gender,
    head_animacy: Animacy,
    ctx: &mut Ctx,
) -> Result<Vec<Token>, PhraseError> {
    if rel.relativizer == Relativizer::Iže {
        return Err(PhraseError::Unsupported(
            "the iže relativizer (no facade paradigm)",
        ));
    }
    if rel.gap == GapRole::Subject && rel.subject.is_some() {
        return Err(PhraseError::IncoherentClause(
            "subject-gap relative clause also supplies a subject",
        ));
    }
    let rel_case = match &rel.gap {
        GapRole::Subject => Case::Nom,
        GapRole::Object => vp_verb_info(&rel.vp.verb)
            .2
            .and_then(|info| info.governs)
            .unwrap_or(Case::Acc),
        // The gap's preposition passes the SAME government check as an
        // ordinary PP — a relative clause is no escape hatch.
        GapRole::PpObject { preposition, case } => {
            resolve_preposition_case(preposition, Some(*case))?
        }
    };
    let relativizer = pronoun("ktory", rel_case, head_number, head_gender, head_animacy)
        .expect("ktory declines for every cell");

    let mut tokens = vec![Token::Punct(',')];
    if let GapRole::PpObject { preposition, .. } = &rel.gap {
        tokens.push(word(preposition.clone()));
    }
    tokens.push(word(surface(&relativizer)));

    // Agreement inside the relative: a subject gap agrees with the head
    // (3rd person); otherwise with the overt subject.
    let (person, number, gender, subject_tokens) = match (&rel.gap, &rel.subject) {
        (GapRole::Subject, _) => (Person::Third, head_number, head_gender, Vec::new()),
        (_, Some(subject)) => {
            let rendered = render_nominal(
                subject,
                Case::Nom,
                PronounStyle::Full,
                CliticContext::ForceFull,
                ctx,
            )?;
            (
                rendered.person,
                rendered.number,
                rendered.gender,
                rendered.tokens,
            )
        }
        (_, None) => {
            return Err(PhraseError::Unsupported(
                "a non-subject gap needs an overt subject",
            ));
        }
    };
    tokens.extend(subject_tokens);

    let shape = ClauseShape {
        force: Force::Declarative,
        mood: Mood::Indicative,
        voice: Voice::Active,
        tense: rel.tense,
        polarity: rel.polarity,
    };
    let vp = render_vp(
        &rel.vp,
        Some(&rel.gap),
        &shape,
        person,
        number,
        gender,
        head_animacy,
        CliticContext::Allowed,
        ctx,
    )?;
    tokens.extend(vp.complex);
    tokens.extend(vp.cluster); // postverbal within the relative domain
    if let Some(object) = vp.object {
        tokens.extend(object);
    }
    for adjunct in vp.adjuncts {
        tokens.extend(adjunct);
    }
    tokens.push(Token::Punct(','));
    Ok(tokens)
}

// ---------------------------------------------------------------------------
// Clause realization.
// ---------------------------------------------------------------------------

/// Is this nominal's rendered core (head + agreeing modifiers, relatives
/// excluded — their words are case-invariant) identical between Nom and
/// Acc? PURE: no context, no warnings, no relative rendering.
fn nom_acc_syncretic(nominal: &Nominal) -> bool {
    fn np_core(np: &NounPhrase, case: Case) -> Vec<String> {
        let info = interslavic::noun_info(&np.head);
        let (gender, animacy) = (info.gender, info.animacy);
        let number = if info.plural_only {
            Number::Plural
        } else {
            Number::Singular
        };
        let modifier = |lemma: &str| -> String {
            pronoun(lemma, case, number, gender, animacy)
                .unwrap_or_else(|| adj(lemma, case, number, gender, animacy))
        };
        let mut out = Vec::new();
        if let Some(n) = np.count {
            let parts =
                quantified_parts_with_info(n, &np.head, case, gender, animacy, info.plural_only);
            out.push(parts.noun);
        } else {
            if let Some(det) = &np.determiner {
                out.push(modifier(det));
            }
            for adjective in &np.adjectives {
                out.push(modifier(adjective));
            }
            out.push(noun_with(&np.head, case, number, gender, animacy));
        }
        out
    }
    match nominal {
        Nominal::Pron { .. } => false,
        Nominal::Name {
            text,
            gender,
            indeclinable,
        } => {
            *indeclinable
                || noun_with(text, Case::Nom, Number::Singular, *gender, Animacy::Animate)
                    == noun_with(text, Case::Acc, Number::Singular, *gender, Animacy::Animate)
        }
        Nominal::Np(np) => np_core(np, Case::Nom) == np_core(np, Case::Acc),
        Nominal::Coord(coordination) => coordination.items.iter().all(nom_acc_syncretic),
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
    let mut ctx = Ctx {
        opts: &opts,
        warnings: Vec::new(),
    };

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
        Case::Nom,
        PronounStyle::Full,
        CliticContext::ForceFull,
        &mut ctx,
    )?;
    let (person, number, gender) = if let Force::Imperative(addressee) = clause.force {
        match addressee {
            Addressee::You => (Person::Second, Number::Singular, Gender::Masculine),
            Addressee::We => (Person::First, Number::Plural, Gender::Masculine),
            Addressee::YouAll => (Person::Second, Number::Plural, Gender::Masculine),
        }
    } else {
        (subject.person, subject.number, subject.gender)
    };

    // Information-structure marking = stress: a marked object renders
    // full pronoun forms (a clitic cannot be topicalized or focused).
    let object_marked =
        clause.topic == Some(SlotRef::Object) || clause.focus == Some(SlotRef::Object);
    let object_clitics = if object_marked {
        CliticContext::ForceFull
    } else {
        CliticContext::Allowed
    };

    // Build labeled constituents.
    let mut constituents: Vec<Constituent> = Vec::new();
    if !imperative && !clause.prodrop {
        constituents.push(Constituent {
            slot: SlotKind::Subject,
            tokens: subject.tokens.clone(),
        });
    }

    // Per-VP clusters, held aside for placement after ordering.
    let mut clusters: Vec<(usize, Vec<Token>)> = Vec::new();
    // The case VP 0's object was actually rendered in — the syncretism
    // guard fires only for a genuinely accusative object.
    let mut first_object_case: Option<Case> = None;

    match &clause.core {
        ClauseCore::Copular {
            predicate,
            pred_case,
        } => {
            let complex = build_complex(
                "byti",
                None,
                &shape,
                person,
                number,
                gender,
                subject.animacy,
                &mut ctx.warnings,
            )?;
            constituents.push(Constituent {
                slot: SlotKind::Verb(0),
                tokens: complex,
            });
            // `PredCase` is a NOMINAL-predicate option; adjectival and
            // participial predicates agree in the nominative. An
            // instrumental request on those variants is incoherent —
            // reported, never silently applied or dropped.
            let tokens = match predicate {
                Predicate::Nominal(np) => {
                    let case = match pred_case {
                        PredCase::Nominative => Case::Nom,
                        PredCase::Instrumental => Case::Ins,
                    };
                    render_np(np, case, &mut ctx)?.tokens
                }
                Predicate::Adjectival(adjective) => {
                    if *pred_case == PredCase::Instrumental {
                        return Err(PhraseError::IncoherentClause(
                            "instrumental predicate case on an adjectival predicate \
                             (a nominal-only option)",
                        ));
                    }
                    vec![word(surface(&adj(
                        adjective,
                        Case::Nom,
                        number,
                        gender,
                        subject.animacy,
                    )))]
                }
                Predicate::Participial(infinitive) => {
                    if *pred_case == PredCase::Instrumental {
                        return Err(PhraseError::IncoherentClause(
                            "instrumental predicate case on a participial predicate \
                             (a nominal-only option)",
                        ));
                    }
                    let participle =
                        passive_participle(infinitive, Case::Nom, number, gender, subject.animacy)
                            .ok_or(PhraseError::Unsupported(
                                "no passive participle (intransitive verb?)",
                            ))?;
                    vec![word(surface(&participle))]
                }
            };
            constituents.push(Constituent {
                slot: SlotKind::Object(0),
                tokens,
            });
        }
        ClauseCore::Verbal(coordination) => {
            if coordination.items.is_empty() {
                return Err(PhraseError::EmptyCoordination);
            }
            for (index, verb_phrase) in coordination.items.iter().enumerate() {
                if index > 0 {
                    constituents.push(Constituent {
                        slot: SlotKind::Fixed,
                        tokens: vec![if index == coordination.items.len() - 1 {
                            word(coordination.conjunction.word())
                        } else {
                            Token::Punct(',')
                        }],
                    });
                }
                let vp = render_vp(
                    verb_phrase,
                    None,
                    &shape,
                    person,
                    number,
                    gender,
                    subject.animacy,
                    if index == 0 {
                        object_clitics
                    } else {
                        CliticContext::Allowed
                    },
                    &mut ctx,
                )?;
                constituents.push(Constituent {
                    slot: SlotKind::Verb(index),
                    tokens: vp.complex,
                });
                if index == 0 {
                    first_object_case = vp.object_case;
                }
                if !vp.cluster.is_empty() {
                    clusters.push((index, vp.cluster));
                }
                if let Some(object) = vp.object {
                    constituents.push(Constituent {
                        slot: SlotKind::Object(index),
                        tokens: object,
                    });
                }
                for adjunct in vp.adjuncts {
                    constituents.push(Constituent {
                        slot: SlotKind::Fixed,
                        tokens: adjunct,
                    });
                }
            }
        }
    }

    // Information-structure references must denote constituents that
    // exist: a topicalized/focused slot that was never built (no object;
    // a dropped or imperative subject) is an incoherent tree — reported,
    // never a silently ignored instruction. Checked before ordering so
    // every force, `li` included, passes through it.
    let have =
        |constituents: &[Constituent], slot: SlotKind| constituents.iter().any(|c| c.slot == slot);
    for reference in [clause.topic, clause.focus].into_iter().flatten() {
        let (slot, missing) = match reference {
            SlotRef::Subject => (
                SlotKind::Subject,
                "topic/focus on a subject the clause does not surface \
                 (prodrop or imperative)",
            ),
            SlotRef::Object => (
                SlotKind::Object(0),
                "topic/focus on an object the clause does not have",
            ),
        };
        if !have(&constituents, slot) {
            return Err(PhraseError::IncoherentClause(missing));
        }
    }

    // --- Ordering (labels, not strings) --------------------------------
    order_constituents(&mut constituents, clause);

    // Syncretism guard on the ACTUAL final order: warn only when subject
    // and first object genuinely inverted, the object was rendered in
    // the accusative (an oblique form already disambiguates), and both
    // are Nom/Acc-ambiguous (steen's clarity caveat). Pure probe: no
    // re-rendering side effects.
    let position = |slot: SlotKind| constituents.iter().position(|c| c.slot == slot);
    if let (Some(subject_at), Some(object_at)) =
        (position(SlotKind::Subject), position(SlotKind::Object(0)))
    {
        if object_at < subject_at
            && first_object_case == Some(Case::Acc)
            && nom_acc_syncretic(&clause.subject)
            && clause_first_object(clause).is_some_and(nom_acc_syncretic)
        {
            ctx.warnings.push(PhraseWarning::AmbiguousOrder);
        }
    }

    // --- li, či, and clitic placement (structural) ----------------------
    if clause.force == Force::LiQuestion {
        let focus_slot = match clause.focus {
            Some(SlotRef::Object) => SlotKind::Object(0),
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
                slot: SlotKind::Fixed,
                tokens: vec![word("li")],
            },
        );
    }
    // `či` joins the constituent list BEFORE cluster placement: the
    // fronted particle is the clause's first constituent and hosts a
    // second-position cluster ("Či sę krålj myl?" — POLICY, parallel to
    // the pan-Slavic particle-as-host pattern).
    if clause.force == Force::CiQuestion {
        constituents.insert(
            0,
            Constituent {
                slot: SlotKind::Fixed,
                tokens: vec![word("či")],
            },
        );
    }
    for (vp_index, cluster) in clusters {
        place_cluster(&mut constituents, vp_index, cluster, opts.clitic_style);
    }

    // --- The single stringification ------------------------------------
    let mut tokens: Vec<Token> = Vec::new();
    if let Some(lead) = lead_in {
        tokens.push(word(lead));
    }
    for constituent in constituents {
        tokens.extend(constituent.tokens);
    }
    let text = join(&tokens, clause.force, opts.sentence);
    Ok(Realized {
        text,
        warnings: ctx.warnings,
    })
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
/// assert!(matches!(err, PhraseError::AmbiguousPreposition { .. }));
/// ```
pub fn realize(clause: &Clause, opts: RealizeOpts) -> Result<String, PhraseError> {
    realize_checked(clause, opts).map(|realized| realized.text)
}

fn clause_first_object(clause: &Clause) -> Option<&Nominal> {
    match &clause.core {
        ClauseCore::Verbal(coordination) => {
            coordination.items.first().and_then(|vp| vp.object.as_ref())
        }
        ClauseCore::Copular { .. } => None,
    }
}

/// Order constituents by information structure: default S V O; `:topic`
/// fronts its slot, `:focus` moves its slot last (theme-first,
/// rheme-last — functional sentence perspective). LiQuestions front an
/// explicit topic, then the focused slot (default: the verb), and follow
/// with verb–subject–object (steen's own example order).
fn order_constituents(constituents: &mut Vec<Constituent>, clause: &Clause) {
    let take = |constituents: &mut Vec<Constituent>, slot: SlotKind| -> Option<Constituent> {
        constituents
            .iter()
            .position(|c| c.slot == slot)
            .map(|index| constituents.remove(index))
    };
    let slot_of = |slot: SlotRef| match slot {
        SlotRef::Subject => SlotKind::Subject,
        SlotRef::Object => SlotKind::Object(0),
    };

    if clause.force == Force::LiQuestion {
        let topic = clause
            .topic
            .and_then(|topic| take(constituents, slot_of(topic)));
        let focus_slot = clause.focus.map(slot_of).unwrap_or(SlotKind::Verb(0));
        let focus = take(constituents, focus_slot);
        let verb = take(constituents, SlotKind::Verb(0));
        let subject = take(constituents, SlotKind::Subject);
        let object = take(constituents, SlotKind::Object(0));
        let mut ordered = Vec::new();
        ordered.extend(topic);
        ordered.extend(focus);
        for item in [verb, subject, object].into_iter().flatten() {
            ordered.push(item);
        }
        ordered.append(constituents);
        *constituents = ordered;
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
}

/// Place one VP's clitic cluster structurally. Postverbal: directly
/// after that VP's complex constituent (and after a directly following
/// `li` — li is first in the cluster, Franks & King order). Second
/// position: after the first constituent of the cluster's domain — the
/// clause for VP 0, the conjunct (verb-initial) for later VPs.
fn place_cluster(
    constituents: &mut Vec<Constituent>,
    vp_index: usize,
    cluster: Vec<Token>,
    style: CliticStyle,
) {
    let is_li_particle = |constituent: &Constituent| {
        constituent.slot == SlotKind::Fixed && constituent.tokens == vec![word("li")]
    };
    let after_li = |constituents: &[Constituent], mut index: usize| -> usize {
        if constituents.get(index).is_some_and(is_li_particle) {
            index += 1;
        }
        index
    };
    let insert_at = match (style, vp_index) {
        (CliticStyle::SecondPosition, 0) => {
            constituents.iter().position(is_li_particle).map_or_else(
                || 1.min(constituents.len()),
                |li_at| after_li(constituents, li_at),
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
            tokens: cluster,
        },
    );
}

/// The single stringification: words spaced, punctuation attached to the
/// preceding word, terminal mark appended — a comma trailing directly
/// before the terminal mark is dropped (the ONLY comma the join ever
/// removes; no global string replacement).
fn join(tokens: &[Token], force: Force, sentence: bool) -> String {
    let mut out = String::new();
    let mut trailing_comma = false;
    for token in tokens {
        match token {
            Token::Punct(mark) => {
                out.push(*mark);
                trailing_comma = *mark == ',';
            }
            Token::Word(text) | Token::Clitic(text) => {
                if !out.is_empty() {
                    out.push(' ');
                }
                out.push_str(text);
                trailing_comma = false;
            }
        }
    }
    if sentence {
        if trailing_comma {
            out.pop();
        }
        let mut chars = out.chars();
        if let Some(first) = chars.next() {
            out = first.to_uppercase().collect::<String>() + chars.as_str();
        }
        out.push(match force {
            Force::Declarative => '.',
            Force::Imperative(_) => '!',
            _ => '?',
        });
    } else {
        while out.ends_with([',', ' ']) {
            out.pop();
        }
    }
    out
}
