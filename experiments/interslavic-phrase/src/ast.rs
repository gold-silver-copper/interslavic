//! Abstract syntax: typed nodes over citation-form leaves.
//!
//! Leaves carry the flavored citation forms the `interslavic` facade
//! expects (Nom-sg nouns, masc-Nom-sg adjectives, infinitives). A
//! [`NounPhrase`] has no case: case constraints belong to grammatical
//! role edges such as [`Complement`], [`Recipient`], [`PrepPhrase`],
//! predicate case, and relative gaps. Resolution therefore produces one
//! authoritative case for each nominal slot before linearization.
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
    /// Grammatical number when no numeral fixes it. `None` is singular.
    ///
    /// This is distinct from `count`: `vsi psi`, `moje děti`, and
    /// `vlåsy` are plural without being quantified, and before this
    /// existed the only route to a plural noun phrase was to invent a
    /// numeral it does not have.
    pub(crate) number: Option<Number>,
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
            number: None,
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
    /// An unquantified plural (`vsi psi`, `moje děti`).
    pub fn plural(mut self) -> Self {
        self.number = Some(Number::Plural);
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

/// A recipient attached to a verb by a dedicated dative role edge.
///
/// Steen's possessive-pronoun contrast uses the ditransitive frame
/// `Pjotr dal Ivanu svoju/jegovu knigu`: the recipient is dative and
/// precedes the accusative theme. The case is therefore a property of
/// this edge rather than an override stored on the nominal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recipient {
    pub(crate) nominal: Nominal,
}

/// A bare oblique nominal used adverbially, with case owned by the
/// adjunct edge rather than the noun phrase.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Oblique {
    pub(crate) case: Case,
    pub(crate) nominal: Nominal,
}

impl Oblique {
    pub fn new(case: Case, nominal: impl Into<Nominal>) -> Self {
        Self {
            case,
            nominal: nominal.into(),
        }
    }

    pub fn case(&self) -> Case {
        self.case
    }

    pub fn nominal(&self) -> &Nominal {
        &self.nominal
    }
}

impl Recipient {
    pub fn new(nominal: impl Into<Nominal>) -> Self {
        Self {
            nominal: nominal.into(),
        }
    }

    pub fn nominal(&self) -> &Nominal {
        &self.nominal
    }
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
/// optional recipient (dative), optional object (case from the verb's
/// dictionary government, defaulting to accusative), adverbs, and
/// prepositional adjuncts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerbPhrase {
    pub(crate) verb: String,
    pub(crate) recipient: Option<Recipient>,
    pub(crate) object: Option<Complement>,
    pub(crate) adverbs: Vec<String>,
    pub(crate) pps: Vec<PrepPhrase>,
    pub(crate) obliques: Vec<Oblique>,
    /// A finite complement clause governed by this verb — the `že`/`da`
    /// argument of `uviděl, že ide ljėv`. It is an argument of the verb,
    /// so it lives on the VP; clause-level adverbials live on
    /// [`Clause::adverbial_clauses`] instead.
    pub(crate) complement_clause: Option<Box<SubClause>>,
    /// A bare infinitive complement: the `spati` of `ne mogų spati`, the
    /// `prinesti jědų` of `htěli prinesti jědų svojim malym`.
    ///
    /// It is itself a [`VerbPhrase`], so it carries its own object,
    /// recipient, adjuncts, and — like every verb here — its own clitic
    /// domain. Steen's `mogų slomiti ti hrėbet` puts the infinitive's
    /// dative clitic inside that domain, which is what this models.
    /// (`načęl go napominati`, where the clitic climbs to the finite
    /// verb instead, is the opposite pattern and is not modelled.)
    pub(crate) infinitive: Option<Box<VerbPhrase>>,
}

impl VerbPhrase {
    pub fn new(verb: &str) -> Self {
        Self {
            verb: verb.trim().to_string(),
            recipient: None,
            object: None,
            adverbs: Vec::new(),
            pps: Vec::new(),
            obliques: Vec::new(),
            complement_clause: None,
            infinitive: None,
        }
    }
    /// Govern a bare infinitive complement (`mogų spati`).
    pub fn infinitive(mut self, infinitive: VerbPhrase) -> Self {
        self.infinitive = Some(Box::new(infinitive));
        self
    }
    /// Govern a finite complement clause (`že …`, `da by …`).
    pub fn complement_clause(mut self, complementizer: Complementizer, clause: Clause) -> Self {
        self.complement_clause = Some(Box::new(SubClause::new(complementizer, clause)));
        self
    }
    pub fn recipient(mut self, recipient: impl Into<Nominal>) -> Self {
        self.recipient = Some(Recipient::new(recipient));
        self
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
    pub fn oblique(mut self, case: Case, nominal: impl Into<Nominal>) -> Self {
        self.obliques.push(Oblique::new(case, nominal));
        self
    }
}

/// The complementizer introducing a subordinate clause.
///
/// The irrealis `by` of purpose clauses is NOT part of the
/// complementizer: `da by uviděl`, `da byhmo ne råzprostrånili sę`, and
/// `že byh te odomašnil` are the complementizer plus the embedded
/// clause's own conditional mood, whose auxiliary is person-marked by the
/// facade's conditional paradigm. Folding `by` into the complementizer
/// would duplicate that paradigm and lose the person agreement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Complementizer {
    /// `že` — declarative complement (`uviděl, že ide ljėv`) and, with
    /// conditional mood, purpose (`že byh te odomašnil`).
    Že,
    /// `da` — purpose, always with conditional mood in the sources
    /// (`da by uviděl gråd`).
    Da,
    /// `kȯgda` — temporal (`kȯgda viđų, kako člověk vladaje konjami`).
    Kogda,
    /// `ako` — hypothetical (`ako ty tam ješče raz prijdeš`).
    Ako,
    /// `zato že` — causal (`zato že onde Bog råzměšal język`).
    ZatoŽe,
    /// `tomu že` — causal (`Tomu že ja ju podlival jesm`).
    TomuŽe,
}

impl Complementizer {
    /// The surface words, in order. Two-word complementizers are one
    /// lexical choice, not a coordination.
    ///
    /// These are flavored citation forms, like every other leaf in this
    /// crate, and are spelled as the dictionary spells them — `kȯgda`,
    /// not `kogda`.
    pub fn words(self) -> &'static [&'static str] {
        match self {
            Complementizer::Že => &["že"],
            Complementizer::Da => &["da"],
            Complementizer::Kogda => &["kȯgda"],
            Complementizer::Ako => &["ako"],
            Complementizer::ZatoŽe => &["zato", "že"],
            Complementizer::TomuŽe => &["tomu", "že"],
        }
    }
}

/// Where a clause-level adverbial subordinate clause sits relative to the
/// matrix clause. Both orders are attested and the comma goes on the
/// inside edge either way: `Ale kȯgda ljudi prěměstili sę …, oni našli …`
/// against `Boli mně sŕdce, kȯgda viđų …`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AdjunctPosition {
    Initial,
    #[default]
    Final,
}

/// A subordinate clause: a complementizer plus a full clause.
///
/// The embedded clause is a [`Clause`], not a reduced structure, so it
/// owns its own subject agreement, tense, polarity, mood, information
/// structure, and — critically — its own clitic domain. A matrix verb can
/// never extract a clitic from inside one, for the same structural reason
/// it cannot reach into a relative clause.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubClause {
    pub(crate) complementizer: Complementizer,
    pub(crate) position: AdjunctPosition,
    pub(crate) clause: Box<Clause>,
}

impl SubClause {
    pub fn new(complementizer: Complementizer, clause: Clause) -> Self {
        Self {
            complementizer,
            position: AdjunctPosition::default(),
            clause: Box::new(clause),
        }
    }

    /// Place a clause-level adverbial before the matrix clause. Ignored
    /// for verb-complement clauses, which always follow their verb.
    pub fn position(mut self, position: AdjunctPosition) -> Self {
        self.position = position;
        self
    }

    pub fn complementizer(&self) -> Complementizer {
        self.complementizer
    }

    pub fn clause(&self) -> &Clause {
        &self.clause
    }
}

/// A clause coordinated with the matrix clause: `Verigy sųt želězne, a
/// želězo jest tvŕdo.`
///
/// This is NOT verb-phrase coordination, which shares one subject and
/// already exists as `Coordination<VerbPhrase>`. Each coordinate clause
/// has its own subject, tense, polarity, and — as every clause here does
/// — its own clitic domain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoordClause {
    pub(crate) conjunction: Conj,
    pub(crate) clause: Box<Clause>,
}

impl CoordClause {
    pub fn new(conjunction: Conj, clause: Clause) -> Self {
        Self {
            conjunction,
            clause: Box::new(clause),
        }
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

/// A clause-initial adverbial present active participle and its
/// dependents, as in `Idųći do raboty, ona ...`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParticipialAdjunct {
    pub(crate) verb: String,
    pub(crate) pps: Vec<PrepPhrase>,
}

impl ParticipialAdjunct {
    pub fn new(verb: &str) -> Self {
        Self {
            verb: verb.trim().to_string(),
            pps: Vec::new(),
        }
    }

    pub fn pp(mut self, pp: PrepPhrase) -> Self {
        self.pps.push(pp);
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
    /// Steen's optional short predicative form (`dom jest velik`).
    ShortAdjectival(String),
    Participial(String),
    /// A prepositional phrase as predicate: `A ovca jest bez vȯlny.`
    /// The PP owns its own case, as every [`PrepPhrase`] does, so
    /// predicate case does not apply to it.
    Prepositional(PrepPhrase),
    /// A comparative or superlative adjective, built by the facade's
    /// `comparative`/`superlative` from the positive lemma stored here.
    /// The lemma is the positive form so the tree stays a citation-form
    /// tree; the degree is a grammatical feature, not a second lexeme.
    Graded {
        lemma: String,
        degree: Degree,
    },
}

/// Degree of comparison for a graded predicate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Degree {
    Comparative,
    Superlative,
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
        /// One copula may carry several coordinated predicates:
        /// `ty jesi veliky i tȯlsty`, `Vsi ljudi rodęt sę svobodni i
        /// råvni`. This reuses `Coordination<T>` rather than adding a
        /// second coordination mechanism; a single predicate is a
        /// one-item coordination and realizes as itself.
        predicates: Coordination<Predicate>,
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

/// The generic `SlotRef::Recipient` denotes the first recipient that
/// exists in surface VP order.
pub(crate) fn information_recipient_index(core: &ClauseCore) -> Option<usize> {
    match core {
        ClauseCore::Verbal(coordination) => coordination
            .items
            .iter()
            .position(|vp| vp.recipient.is_some()),
        ClauseCore::Copular { .. } => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TenseSpec {
    Present,
    Past,
    /// Steen's optional simple past, which merges the historical
    /// imperfect and aorist roles.
    Imperfect,
    /// Simple pluperfect (`běh dělal`).
    Pluperfect,
    /// Analytic alternative `byl jesm dělal` documented alongside the
    /// simple pluperfect `běh dělal`.
    CompoundPluperfect,
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
    /// Past conditional: past `byti` participle + conditional
    /// auxiliary + the lexical verb's L-participle.
    ConditionalPerfect,
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
    /// `byti` plus the present passive participle (`-omy`/`-imy`).
    PresentPassive,
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

/// Clause force. The three overt yes/no question strategies and
/// constituent-question fronting are steen's syntax-page forms; `li`
/// attaches after the clause's focus — the finite verb unless `:focus`
/// marks another constituent. The imperative omits its subject by
/// default. Imperative and optative clauses take `!`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Force {
    Declarative,
    IntonationQuestion,
    CiQuestion,
    LiQuestion,
    /// A constituent or interrogative adverb is fronted explicitly via
    /// [`Clause::wh_slot`] or [`Clause::wh_adverb`].
    WhQuestion,
    /// Third-person optative with the clause-initial particle `nehaj`.
    Optative,
    Imperative(Addressee),
}

/// A constituent reference for information-structure marking.
/// `Recipient` and `Object` select the first matching edge that exists
/// in surface VP order; a copular predicate occupies the object slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotRef {
    Subject,
    Recipient,
    Object,
}

/// The explicitly fronted interrogative phrase in a constituent
/// question. A slot reorders an existing constituent; an adverb is a
/// clause-level leaf such as `kde`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhFront {
    Slot(SlotRef),
    Adverb(String),
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
    pub(crate) wh: Option<WhFront>,
    pub(crate) initial_participles: Vec<ParticipialAdjunct>,
    /// Clause-level adverbial subordinate clauses (`kogda …`, `ako …`,
    /// `zato že …`). Each carries its own position; verb-argument
    /// complement clauses live on [`VerbPhrase::complement_clause`].
    pub(crate) adverbial_clauses: Vec<SubClause>,
    /// Clauses coordinated with this one, each with its own subject.
    pub(crate) coordinate_clauses: Vec<CoordClause>,
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
            wh: None,
            initial_participles: Vec::new(),
            adverbial_clauses: Vec::new(),
            coordinate_clauses: Vec::new(),
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
    pub fn conditional_perfect(mut self) -> Self {
        self.mood = Mood::ConditionalPerfect;
        self
    }
    pub fn passive(mut self) -> Self {
        self.voice = Voice::Passive;
        self
    }
    pub fn present_passive(mut self) -> Self {
        self.voice = Voice::PresentPassive;
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
    pub fn wh_slot(mut self, slot: SlotRef) -> Self {
        self.force = Force::WhQuestion;
        self.wh = Some(WhFront::Slot(slot));
        self
    }
    pub fn wh_adverb(mut self, adverb: &str) -> Self {
        self.force = Force::WhQuestion;
        self.wh = Some(WhFront::Adverb(adverb.trim().to_string()));
        self
    }
    pub fn initial_participle(mut self, adjunct: ParticipialAdjunct) -> Self {
        self.initial_participles.push(adjunct);
        self
    }
    /// Attach a clause-level adverbial subordinate clause.
    pub fn adverbial_clause(mut self, adjunct: SubClause) -> Self {
        self.adverbial_clauses.push(adjunct);
        self
    }
    /// Coordinate another full clause with this one.
    pub fn coordinate_clause(mut self, conjunction: Conj, clause: Clause) -> Self {
        self.coordinate_clauses
            .push(CoordClause::new(conjunction, clause));
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
pub fn participial_adjunct(verb: &str) -> ParticipialAdjunct {
    ParticipialAdjunct::new(verb)
}
pub fn sub(complementizer: Complementizer, clause: Clause) -> SubClause {
    SubClause::new(complementizer, clause)
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
            predicates: Coordination::single(predicate),
            pred_case: PredCase::default(),
        },
    )
}
