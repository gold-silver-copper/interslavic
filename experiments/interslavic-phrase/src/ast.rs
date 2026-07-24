//! Abstract syntax: typed nodes over citation-form leaves.
//!
//! Leaves carry the flavored citation forms the `interslavic` facade
//! expects (Nom-sg nouns, masc-Nom-sg adjectives, infinitives). Case is
//! assigned by the *slot* a nominal occupies (subject → Nom, object →
//! the verb's dictionary government or Acc, PP → the preposition's
//! government); [`NounPhrase::case_override`] exists only for marked
//! constructions.
//!
//! 0.2.0 is a breaking revision of the 0.1.0 AST (the crate README
//! warned 0.1.x would move): `Clause.vp` became [`Clause::core`]
//! (verbal cores are coordinations; copular cores are new), `Pron`
//! gained `clitic`, `NounPhrase` gained `relative`/`entity`, and
//! `Force` gained `Imperative`.

use interslavic::{Case, Gender, Number, Person};

/// A noun phrase: optional determiner and count, adjective modifiers,
/// the head noun that fixes gender/animacy for the whole phrase, an
/// optional relative clause, and an optional discourse entity tag
/// (consumed by [`crate::discourse`]; plain realization ignores it).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NounPhrase {
    pub determiner: Option<String>,
    pub count: Option<u64>,
    pub adjectives: Vec<String>,
    pub head: String,
    pub case_override: Option<Case>,
    pub relative: Option<Box<RelClause>>,
    pub entity: Option<String>,
}

impl NounPhrase {
    pub fn new(head: &str) -> Self {
        Self {
            determiner: None,
            count: None,
            adjectives: Vec::new(),
            head: head.trim().to_string(),
            case_override: None,
            relative: None,
            entity: None,
        }
    }
    pub fn det(mut self, determiner: &str) -> Self {
        self.determiner = Some(determiner.trim().to_string());
        self
    }
    pub fn count(mut self, n: u64) -> Self {
        self.count = Some(n);
        self
    }
    pub fn adj(mut self, adjective: &str) -> Self {
        self.adjectives.push(adjective.trim().to_string());
        self
    }
    pub fn case(mut self, case: Case) -> Self {
        self.case_override = Some(case);
        self
    }
    pub fn relative(mut self, rel: RelClause) -> Self {
        self.relative = Some(Box::new(rel));
        self
    }
    pub fn entity(mut self, id: &str) -> Self {
        self.entity = Some(id.to_string());
        self
    }
}

/// The relativizer lexeme. `Ktory` is the neutral default; `Iže` is the
/// bookish register steen mentions — currently the facade has no `iže`
/// paradigm, so requesting it is a declared [`Unsupported`]
/// diagnostic (a facade finding, not silently wrong output).
///
/// [`Unsupported`]: crate::PhraseError::Unsupported
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Relativizer {
    #[default]
    Ktory,
    Iže,
}

/// The role the head plays inside its relative clause: the relativizer
/// takes its CASE from this role while agreeing with the head in
/// gender/number — the classic agreement/government split.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GapRole {
    Subject,
    Object,
    PpObject { preposition: String, case: Case },
}

/// A relative clause: a clause body with one argument gapped.
/// `subject` is `None` exactly when the gap IS the subject.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelClause {
    pub gap: GapRole,
    pub subject: Option<Nominal>,
    pub vp: VerbPhrase,
    pub tense: TenseSpec,
    pub polarity: Polarity,
    pub relativizer: Relativizer,
}

impl RelClause {
    pub fn subject_gap(vp: VerbPhrase) -> Self {
        Self {
            gap: GapRole::Subject,
            subject: None,
            vp,
            tense: TenseSpec::Present,
            polarity: Polarity::Affirmative,
            relativizer: Relativizer::Ktory,
        }
    }
    pub fn object_gap(subject: impl Into<Nominal>, vp: VerbPhrase) -> Self {
        Self {
            gap: GapRole::Object,
            subject: Some(subject.into()),
            vp,
            tense: TenseSpec::Present,
            polarity: Polarity::Affirmative,
            relativizer: Relativizer::Ktory,
        }
    }
    pub fn tense(mut self, tense: TenseSpec) -> Self {
        self.tense = tense;
        self
    }
    pub fn past(self) -> Self {
        self.tense(TenseSpec::Past)
    }
    pub fn relativizer(mut self, relativizer: Relativizer) -> Self {
        self.relativizer = relativizer;
        self
    }
}

/// Coordinating conjunctions — a closed table, each entry a dictionary
/// `conj.` row (preposition-table discipline): `i` "and" (row 718),
/// `ili` "or" (row 724), `a` "and/while" (row 1855), `ale` "but"
/// (row 134).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Conj {
    I,
    Ili,
    A,
    Ale,
}

impl Conj {
    pub fn word(self) -> &'static str {
        match self {
            Conj::I => "i",
            Conj::Ili => "ili",
            Conj::A => "a",
            Conj::Ale => "ale",
        }
    }
}

/// A coordination of like constituents; `items` is non-empty and a
/// single item realizes as itself (no conjunction).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coordination<T> {
    pub conjunction: Conj,
    pub items: Vec<T>,
}

impl<T> Coordination<T> {
    pub fn single(item: T) -> Self {
        Self {
            conjunction: Conj::I,
            items: vec![item],
        }
    }
}

/// Anything that can fill a nominal slot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Nominal {
    Np(NounPhrase),
    Pron {
        person: Person,
        number: Number,
        gender: Gender,
        /// Prefer the clitic series where one exists (`go`, `mu`, `mi`);
        /// full forms are the default (steen marks clitics as unstressed
        /// variants without mandating them). Clitics join the clause's
        /// clitic cluster for placement.
        clitic: bool,
    },
    /// A proper name: capitalized as written; declined like a noun by
    /// default (Slavic names inflect), `indeclinable` for foreign names.
    Name {
        text: String,
        gender: Gender,
        indeclinable: bool,
    },
    Coord(Coordination<Nominal>),
}

impl From<NounPhrase> for Nominal {
    fn from(np: NounPhrase) -> Self {
        Nominal::Np(np)
    }
}

/// A verb phrase: the verb (an infinitive; a trailing ` sę` marks it
/// reflexive, as does the dictionary's own `v.refl.` metadata), an
/// optional object (case from the verb's dictionary government,
/// defaulting to accusative), adverbs, and prepositional adjuncts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerbPhrase {
    pub verb: String,
    pub object: Option<Nominal>,
    pub adverbs: Vec<String>,
    pub pps: Vec<PrepPhrase>,
}

impl VerbPhrase {
    pub fn new(verb: &str) -> Self {
        Self {
            verb: verb.trim().to_string(),
            object: None,
            adverbs: Vec::new(),
            pps: Vec::new(),
        }
    }
    pub fn object(mut self, object: impl Into<Nominal>) -> Self {
        self.object = Some(object.into());
        self
    }
    pub fn adv(mut self, adverb: &str) -> Self {
        self.adverbs.push(adverb.trim().to_string());
        self
    }
    pub fn pp(mut self, pp: PrepPhrase) -> Self {
        self.pps.push(pp);
        self
    }
}

/// A prepositional phrase. `case` may be omitted only for prepositions
/// that govern a single case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrepPhrase {
    pub preposition: String,
    pub case: Option<Case>,
    pub object: Nominal,
}

impl PrepPhrase {
    pub fn case(mut self, case: Case) -> Self {
        self.case = Some(case);
        self
    }
}

/// A copular predicate: "X jest Y" with a nominal, adjectival, or
/// participial Y. The participial form is the passive-participle
/// construction ("Komnata jest osvětljena") via the facade's
/// `passive_participle`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Predicate {
    Nominal(NounPhrase),
    Adjectival(String),
    Participial(String),
}

/// The predicate case of a NOMINAL copular predicate. Nominative is the
/// default; the instrumental predicate ("on byl kråljem") exists across
/// Slavic — steen's pages state no preference (verified), so the choice
/// is exposed and the default is POLICY. Adjectival and participial
/// predicates always agree in the nominative: requesting the
/// instrumental on them is rejected — by the realizer and the
/// S-expression reader both — never silently applied or dropped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PredCase {
    #[default]
    Nominative,
    Instrumental,
}

/// The clause core: a (possibly coordinated) verb phrase, or a copula
/// plus predicate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClauseCore {
    Verbal(Coordination<VerbPhrase>),
    Copular {
        predicate: Predicate,
        pred_case: PredCase,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TenseSpec {
    Present,
    Past,
    Future,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Polarity {
    Affirmative,
    Negative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mood {
    #[default]
    Indicative,
    /// The `by` + l-participle conditional; person-marked auxiliaries
    /// come from the facade's own conditional paradigm row.
    Conditional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Voice {
    #[default]
    Active,
    /// `byti` + passive participle agreeing with the subject; the agent,
    /// when expressed, is an ordinary PP — steen (syntax page): "either
    /// in the instrumental case or preceded by the preposition od with
    /// the genitive".
    Passive,
}

/// The imperative addressee: the three cells the facade's imperative
/// row provides (2sg / 1pl / 2pl).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Addressee {
    #[default]
    You,
    We,
    YouAll,
}

/// Clause force. The three overt yes/no question strategies are steen's
/// (syntax page); `li` attaches after the clause's focus — the finite
/// verb unless `:focus` marks another constituent. The imperative omits
/// its subject by default (the one construction where pro-drop is the
/// norm) and takes `!`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Force {
    Declarative,
    IntonationQuestion,
    CiQuestion,
    LiQuestion,
    Imperative(Addressee),
}

/// A constituent reference for information-structure marking.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotRef {
    Subject,
    Object,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause {
    pub subject: Nominal,
    pub core: ClauseCore,
    pub tense: TenseSpec,
    pub polarity: Polarity,
    pub force: Force,
    pub mood: Mood,
    pub voice: Voice,
    /// Pro-drop is OFF by default: steen (pronouns page) recommends
    /// keeping the subject pronoun ("ja čitaju" over "čitaju").
    pub prodrop: bool,
    /// Fronted constituent (theme). Unmarked clauses keep neutral SVO.
    pub topic: Option<SlotRef>,
    /// Clause-final constituent (rheme); also the attachment point of
    /// `li` (steen: "right after the focus point of the question").
    pub focus: Option<SlotRef>,
}

impl Clause {
    pub fn new(subject: impl Into<Nominal>, vp: VerbPhrase) -> Self {
        Self::with_core(subject, ClauseCore::Verbal(Coordination::single(vp)))
    }
    pub fn with_core(subject: impl Into<Nominal>, core: ClauseCore) -> Self {
        Self {
            subject: subject.into(),
            core,
            tense: TenseSpec::Present,
            polarity: Polarity::Affirmative,
            force: Force::Declarative,
            mood: Mood::Indicative,
            voice: Voice::Active,
            prodrop: false,
            topic: None,
            focus: None,
        }
    }
    /// Add a coordinated verb phrase (default conjunction `i`).
    pub fn and_vp(mut self, vp: VerbPhrase) -> Self {
        match &mut self.core {
            ClauseCore::Verbal(coordination) => coordination.items.push(vp),
            ClauseCore::Copular { .. } => {
                panic!("and_vp on a copular clause; build a verbal clause instead")
            }
        }
        self
    }
    pub fn conj(mut self, conjunction: Conj) -> Self {
        if let ClauseCore::Verbal(coordination) = &mut self.core {
            coordination.conjunction = conjunction;
        }
        self
    }
    pub fn tense(mut self, tense: TenseSpec) -> Self {
        self.tense = tense;
        self
    }
    pub fn past(self) -> Self {
        self.tense(TenseSpec::Past)
    }
    pub fn future(self) -> Self {
        self.tense(TenseSpec::Future)
    }
    pub fn negated(mut self) -> Self {
        self.polarity = Polarity::Negative;
        self
    }
    pub fn force(mut self, force: Force) -> Self {
        self.force = force;
        self
    }
    pub fn conditional(mut self) -> Self {
        self.mood = Mood::Conditional;
        self
    }
    pub fn passive(mut self) -> Self {
        self.voice = Voice::Passive;
        self
    }
    pub fn prodrop(mut self) -> Self {
        self.prodrop = true;
        self
    }
    pub fn topic(mut self, slot: SlotRef) -> Self {
        self.topic = Some(slot);
        self
    }
    pub fn focus(mut self, slot: SlotRef) -> Self {
        self.focus = Some(slot);
        self
    }
}

/// Builder shorthands mirroring `english-phrase`'s lowercase entry points.
pub fn np(head: &str) -> NounPhrase {
    NounPhrase::new(head)
}
pub fn vp(verb: &str) -> VerbPhrase {
    VerbPhrase::new(verb)
}
pub fn pp(preposition: &str, object: impl Into<Nominal>) -> PrepPhrase {
    PrepPhrase {
        preposition: preposition.trim().to_string(),
        case: None,
        object: object.into(),
    }
}
pub fn pron(person: Person, number: Number, gender: Gender) -> Nominal {
    Nominal::Pron {
        person,
        number,
        gender,
        clitic: false,
    }
}
pub fn pron_clitic(person: Person, number: Number, gender: Gender) -> Nominal {
    Nominal::Pron {
        person,
        number,
        gender,
        clitic: true,
    }
}
pub fn name(text: &str, gender: Gender) -> Nominal {
    Nominal::Name {
        text: text.trim().to_string(),
        gender,
        indeclinable: false,
    }
}
pub fn coordinate(conjunction: Conj, items: Vec<Nominal>) -> Nominal {
    Nominal::Coord(Coordination { conjunction, items })
}
pub fn clause(subject: impl Into<Nominal>, vp: VerbPhrase) -> Clause {
    Clause::new(subject, vp)
}
pub fn copular(subject: impl Into<Nominal>, predicate: Predicate) -> Clause {
    Clause::with_core(
        subject,
        ClauseCore::Copular {
            predicate,
            pred_case: PredCase::default(),
        },
    )
}
