//! Abstract syntax: typed nodes over citation-form leaves.
//!
//! Leaves carry the flavored citation forms the `interslavic` facade
//! expects (Nom-sg nouns, masc-Nom-sg adjectives, infinitives). A
//! [`NounPhrase`] has no case: case constraints belong to grammatical
//! role edges such as [`Complement`], [`PrepPhrase`], predicate case,
//! and relative gaps. Resolution therefore produces one authoritative
//! case for each nominal slot before linearization.
//!
//! 0.2.0 is a breaking revision of the 0.1.0 AST: `Clause.vp` became
//! [`Clause::core`] (verbal cores are coordinations; copular cores are
//! new), direct objects gained an explicit [`Complement`] edge, `Pron`
//! gained `clitic`, `NounPhrase` gained `relative`/`entity`/
//! `referential`, and `Force` gained `Imperative`.

use interslavic::{Case, Gender, Number, Person};
use std::fmt;

/// A noun phrase: optional determiner and count, adjective modifiers,
/// the head noun that fixes gender/animacy for the whole phrase, an
/// optional relative clause, and an optional discourse entity tag
/// (consumed by [`crate::discourse`]; plain realization ignores it).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NounPhrase {
    pub(crate) determiner: Option<String>,
    pub(crate) count: Option<u64>,
    pub(crate) adjectives: Vec<String>,
    pub(crate) head: String,
    pub(crate) relative: Option<Box<RelClause>>,
    pub(crate) entity: Option<String>,
    pub(crate) referential: ReferentialForm,
}

impl NounPhrase {
    pub fn new(head: &str) -> Self {
        Self {
            determiner: None,
            count: None,
            adjectives: Vec::new(),
            head: head.trim().to_string(),
            relative: None,
            entity: None,
            referential: ReferentialForm::Full,
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
    pub fn relative(mut self, rel: RelClause) -> Self {
        self.relative = Some(Box::new(rel));
        self
    }
    pub fn entity(mut self, id: &str) -> Self {
        self.entity = Some(id.to_string());
        self
    }
    pub fn referential(mut self, form: ReferentialForm) -> Self {
        self.referential = form;
        self
    }

    pub fn head(&self) -> &str {
        &self.head
    }
}

/// How an entity-bearing lexical NP should be referred to at the
/// discourse-planning boundary. Changing this never discards the NP's
/// lexical content, modifiers, relative propositions, entity identity,
/// or the case constraint owned by its grammatical role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReferentialForm {
    #[default]
    Full,
    Pronoun,
    Clitic,
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
    Object { requested_case: Option<Case> },
    PpObject { preposition: String, case: Case },
}

/// A relative clause: a clause body with one argument gapped.
/// `subject` is `None` exactly when the gap IS the subject.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelClause {
    pub(crate) gap: GapRole,
    pub(crate) subject: Option<Nominal>,
    pub(crate) vp: VerbPhrase,
    pub(crate) tense: TenseSpec,
    pub(crate) polarity: Polarity,
    pub(crate) relativizer: Relativizer,
}

impl RelClause {
    /// Construct a raw relative clause. Prefer the gap-specific smart
    /// constructors below for ordinary authoring; this constructor exists
    /// for data-driven front ends whose output is checked by [`crate::validate`].
    pub fn new(gap: GapRole, subject: Option<Nominal>, vp: VerbPhrase) -> Self {
        Self {
            gap,
            subject,
            vp,
            tense: TenseSpec::Present,
            polarity: Polarity::Affirmative,
            relativizer: Relativizer::Ktory,
        }
    }

    pub fn subject_gap(vp: VerbPhrase) -> Self {
        Self::new(GapRole::Subject, None, vp)
    }
    pub fn object_gap(subject: impl Into<Nominal>, vp: VerbPhrase) -> Self {
        Self::new(
            GapRole::Object {
                requested_case: None,
            },
            Some(subject.into()),
            vp,
        )
    }
    pub fn object_gap_case(case: Case, subject: impl Into<Nominal>, vp: VerbPhrase) -> Self {
        Self::new(
            GapRole::Object {
                requested_case: Some(case),
            },
            Some(subject.into()),
            vp,
        )
    }
    pub fn pp_gap(
        preposition: &str,
        case: Case,
        subject: impl Into<Nominal>,
        vp: VerbPhrase,
    ) -> Self {
        Self::new(
            GapRole::PpObject {
                preposition: preposition.trim().to_string(),
                case,
            },
            Some(subject.into()),
            vp,
        )
    }
    pub fn tense(mut self, tense: TenseSpec) -> Self {
        self.tense = tense;
        self
    }
    pub fn past(self) -> Self {
        self.tense(TenseSpec::Past)
    }
    pub fn polarity(mut self, polarity: Polarity) -> Self {
        self.polarity = polarity;
        self
    }
    pub fn negated(self) -> Self {
        self.polarity(Polarity::Negative)
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
    pub(crate) conjunction: Conj,
    pub(crate) items: Vec<T>,
}

impl<T> Coordination<T> {
    pub fn single(item: T) -> Self {
        Self {
            conjunction: Conj::I,
            items: vec![item],
        }
    }

    pub fn new(conjunction: Conj, items: Vec<T>) -> Self {
        Self { conjunction, items }
    }

    pub fn items(&self) -> &[T] {
        &self.items
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

/// A nominal attached to a verb-complement edge. The optional requested
/// case is distinct from dictionary/default government and is resolved
/// exactly once during grammar resolution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Complement {
    pub(crate) nominal: Nominal,
    pub(crate) requested_case: Option<Case>,
}

impl Complement {
    pub fn new(nominal: impl Into<Nominal>) -> Self {
        Self {
            nominal: nominal.into(),
            requested_case: None,
        }
    }

    pub fn case(mut self, case: Case) -> Self {
        self.requested_case = Some(case);
        self
    }

    pub fn nominal(&self) -> &Nominal {
        &self.nominal
    }

    pub fn requested_case(&self) -> Option<Case> {
        self.requested_case
    }
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
    pub(crate) verb: String,
    pub(crate) object: Option<Complement>,
    pub(crate) adverbs: Vec<String>,
    pub(crate) pps: Vec<PrepPhrase>,
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
        self.object = Some(Complement::new(object));
        self
    }
    pub fn object_case(mut self, case: Case, object: impl Into<Nominal>) -> Self {
        self.object = Some(Complement::new(object).case(case));
        self
    }
    pub fn complement(mut self, complement: Complement) -> Self {
        self.object = Some(complement);
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
    pub(crate) preposition: String,
    pub(crate) case: Option<Case>,
    pub(crate) object: Nominal,
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

/// The generic `SlotRef::Object` denotes the first object that actually
/// exists in surface order, rather than being tied to VP index zero.
/// A copular predicate occupies object slot zero.
pub(crate) fn information_object_index(core: &ClauseCore) -> Option<usize> {
    match core {
        ClauseCore::Verbal(coordination) => {
            coordination.items.iter().position(|vp| vp.object.is_some())
        }
        ClauseCore::Copular { .. } => Some(0),
    }
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
/// `Object` selects the first object that exists in surface VP order
/// (or the predicate of a copular clause).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotRef {
    Subject,
    Object,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause {
    pub(crate) subject: Nominal,
    pub(crate) core: ClauseCore,
    pub(crate) tense: TenseSpec,
    pub(crate) polarity: Polarity,
    pub(crate) force: Force,
    pub(crate) mood: Mood,
    pub(crate) voice: Voice,
    /// Pro-drop is OFF by default: steen (pronouns page) recommends
    /// keeping the subject pronoun ("ja čitaju" over "čitaju").
    pub(crate) prodrop: bool,
    /// Fronted constituent (theme). Unmarked clauses keep neutral SVO.
    pub(crate) topic: Option<SlotRef>,
    /// Clause-final constituent (rheme); also the attachment point of
    /// `li` (steen: "right after the focus point of the question").
    pub(crate) focus: Option<SlotRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildError {
    VerbalCoreRequired(&'static str),
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildError::VerbalCoreRequired(operation) => {
                write!(f, "`{operation}` requires a verbal clause core")
            }
        }
    }
}

impl std::error::Error for BuildError {}

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
    pub fn and_vp(mut self, vp: VerbPhrase) -> Result<Self, BuildError> {
        match &mut self.core {
            ClauseCore::Verbal(coordination) => coordination.items.push(vp),
            ClauseCore::Copular { .. } => return Err(BuildError::VerbalCoreRequired("and_vp")),
        }
        Ok(self)
    }
    pub fn conj(mut self, conjunction: Conj) -> Result<Self, BuildError> {
        let ClauseCore::Verbal(coordination) = &mut self.core else {
            return Err(BuildError::VerbalCoreRequired("conj"));
        };
        coordination.conjunction = conjunction;
        Ok(self)
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
    Nominal::Coord(Coordination::new(conjunction, items))
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
