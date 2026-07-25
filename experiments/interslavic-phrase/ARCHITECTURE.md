# Architecture

The design goal is to make invalid ownership impossible to express in
later phases. Validation handles raw authoring errors; each later phase
adds information without reopening decisions made earlier.

## Staged pipeline

```text
typed builders ───────┐
                      ├──> RawClause (Clause)
S-expression reader ──┘           │
                                  v
                         validate(raw)
                                  │
                                  v
                         ValidatedClause
                                  │
                  discourse planning (tree → tree)
                                  │
                                  v
                        grammar resolution
                    case + nominal profiles + valence
                                  │
                                  v
                    hierarchical linearization
          ClausePlan / VerbDomainPlan / NominalPlan / RelativePlan
                                  │
                                  v
                   one recursive flatten + stringify
```

`Clause` is the raw authoring type and is also exported as
`RawClause`. Its private fields and smart constructors prevent
accidental mutation, but constructors can still produce data-dependent
invalid states such as an empty coordination. `validate` is the only
structural invariant checker. The S-expression compiler constructs the
same raw nodes and calls it; public raw realization does the same.

`ValidatedClause` has private state. Grammar resolution is the only
consumer before planning, and `realize_validated_checked` lets callers
reuse a validation result.

## Ownership boundaries

| Concern | Owner | Consequence |
| --- | --- | --- |
| lexical NP content | `NounPhrase` | An NP never carries grammatical case. |
| recipient constraint | `Recipient` | The recipient edge assigns dative to the entire nominal and precedes the theme in neutral order. |
| direct-object constraint | `Complement` | An explicit override applies to the whole complement and can be compared with dictionary government once. |
| PP case | `PrepPhrase` | Validation checks the selected case against the preposition before resolution. |
| bare adjunct case | `Oblique` | The adjunct edge assigns one explicit case to its whole nominal. |
| predicate case | copular role edge | A predicate NP cannot override `PredCase`. |
| relative-gap case | `GapRole` plus verb government | `ktory` receives the resolved gap case; no child NP can change it. |
| finite and nominal features | central `NominalProfile` | Realization and discourse share one count/plural-only/coordination policy. |
| referential choice | `NounPhrase::referential` | Discourse requests a pronoun without replacing or impoverishing the syntax node. |
| clitic placement | `VerbDomainPlan` | Only that VP's direct recipient/object clitics and reflexive marker enter its cluster. |
| nested relative content | `RelativePlan` | Parent traversal cannot inspect or extract descendant tokens. |
| wh/optative/participial order | `ClausePlan` constituents | Fronting and particles move typed constituents, not realized strings. |
| punctuation and casing | `ClausePlan::stringify` | Surface nodes flatten once; no morphology is rewritten afterward. |

Case resolution records whether a case came from subject position, a
dative recipient edge, default accusative, dictionary government, an
explicit object edge, preposition, or predicate position. Coordination
members receive the same resolved case recursively. This prevents
recipients, PP objects, nested NPs, predicate NPs, or individual
conjuncts from reopening the decision.

## Validation and resolution

Validation returns `ValidationErrors(Vec<ValidationError>)`. Every error
has an `AstPath`, such as `clause.core.vp[0].object`. It checks:

- non-empty coordination and non-empty leaves;
- imperative/optative/conditional/passive/tense coherence;
- passive patient promotion (no retained direct object);
- predicate-case applicability;
- wh/topic/focus existence and duplicate references, including the
  independent recipient slot;
- relative gap versus overt subject/object coherence;
- ordinary and relative-gap preposition government;
- referential choices that would suppress a relative proposition.
- a shared maximum syntax depth before any recursive consumer runs.

The supported combination rule is compact:

- indicative non-imperative verbal clauses support present, past,
  imperfect, simple and compound pluperfect, and future in active,
  past-passive, or present-passive voice; copular clauses are active;
- present and past conditionals use their own auxiliary/participle
  construction and cannot carry an independent tense;
- imperatives are present, active, and indicative;
- optatives are third-person, present, active, and indicative;
- constituent questions own an explicit wh slot/adverb and cannot
  simultaneously reuse topic/focus ordering.

Dictionary valence requires lexical metadata and therefore belongs to
grammar resolution. `ResolutionErrors` is also pathful. An explicit
overt-object or object-gap case that differs from dictionary government
remains realizable and emits one `GovernsConflict` warning; an object on
a dictionary-intransitive verb is an error. Object gaps pass through the
same case, valence, and government resolver as overt objects.

The recipient edge is author-declared. The current dictionary records
direct transitivity and direct-object government, but not indirect-object
frames, so resolution cannot prove that a particular verb licenses a
recipient. Restricting the edge to a hard-coded verb list would duplicate
lexical policy in the phrase crate.

## Hierarchical clitic domains

A partially planned nominal is never a flat token vector. A
`NominalPlan` contains opaque child nodes and may expose exactly one
`direct_clitic` only when the nominal itself is a clitic pronoun.
`RelativePlan` keeps the relative body nested until its own VP has
placed its cluster.

Each `VerbDomainPlan` owns a cluster ordered `li > dative > accusative >
sę` for the arguments represented by this grammar. A direct recipient
and object therefore produce `mu go`, never separate clusters. Full
nominals use the source-attested neutral order verb–recipient–object.
Postverbal placement inserts the cluster after that VP's complex.
Second-position placement inserts it after the first typed constituent
of the domain. Coordinated VPs are separate domains; a discourse
connective is outside the first domain.

Consequently:

```text
Žena vidi mųža, ktory myje sę.
Žena vidi mųža, ktory vidi go.
```

The relative `sę` and `go` are not visible to the parent VP.

Subordinate clauses, infinitive complements, and coordinated clauses all
extend the same rule rather than adding an exception to it.

`plan_clause` is the single implementation for matrix and embedded
clauses alike. Each call places its own clitic clusters into its own
constituent vector and is then sealed into an opaque
`SurfaceNode::Subordinate`. A matrix verb therefore cannot reach a
clitic inside an embedded clause, for exactly the reason it cannot
reach one inside a relative:

```text
Ja myjų sę, že ona myje sę.
Ja viđų, že on myje sę.
Ja myjų sę, i ona myje sę.
```

The third is clause coordination — `(and-clause …)`, distinct from
verb-phrase coordination, which shares one subject. Each conjunct is a
full clause with its own subject agreement, tense, polarity, and clitic
domain; only the conjunction and its comma are added by the parent.

An infinitive complement is likewise its own domain. Steen's `mogų
slomiti ti hrėbet` puts the dative clitic with `slomiti`, not with the
finite `mogų`, which is what the per-verb rule already predicts. The
opposite pattern — clitic climbing, as in `načęl go napominati` — is
not modelled; supporting both would make placement ambiguous everywhere
on the strength of one sentence.

## Punctuation and capitalization ownership

Terminal punctuation and sentence-initial capitalization happen exactly
once, in `ClausePlan::stringify`, at the top level only. Embedded
clauses contribute `SurfaceNode`s and nothing else, so they cannot
acquire a full stop or a mid-sentence capital, and matrix force
survives a fronted subordinate:

```text
Kȯgda noč jest, či pes spi?
```

A subordinate clause's comma is structural too — a `SurfaceNode::Punct`
on the inside edge, trailing for a fronted adverbial and leading
otherwise. The single join pass handles spacing and collapses a
boundary that coincides with another, so no stage concatenates strings.

Complementizer choice is declared on the edge and resolved in
resolution, never inferred from the embedded verb. `da by` is not a
lexical unit: `da` is the complementizer and `by`/`byh`/`byhmo` comes
from the embedded clause's own conditional mood, which is what
person-marks it.

Clause recursion is bounded by `MAX_CLAUSE_DEPTH`, checked iteratively
in the S-expression preflight. The generic `MAX_STRUCTURE_DEPTH` counts
list levels, and a clause is cheap in those but expensive in
recursive-descent frames, so the generic bound alone admitted input
that exhausted the stack before producing a diagnostic.

## Discourse planning

`narrate_checked` validates every input before discourse planning.
Planning then operates on owned syntax-tree clones, and each transformed
output is validated again before resolution. Entity tracking stores
referent features from the central nominal profile and visits mentions
in typed constituent order after topic/focus movement and before
clitic-cluster placement. Token displacement inside a clitic domain does
not redefine discourse salience; this keeps microplanning independent of
the later grammar-resolution stage. On a repeated
unambiguous mention it changes only `ReferentialForm`; the NP, entity ID,
modifiers, lexical head, and the surrounding complement/PP role remain.

An NP with a relative proposition stays full. Explicitly requesting a
pronominal form on such an NP is rejected because realizing only the
pronoun would silently drop asserted content. Aggregation requires the
same explicit subject entity and is permitted only when all
surface-significant clause features and conjunction scope are compatible.

`narrate_checked` returns `Narrated { text, warnings }` and prefixes
warning paths with `sentence[index]`. `narrate` deliberately discards
warnings.

## Nominal profiles

The central profile distinguishes:

- person, gender, and animacy;
- inflection number;
- finite-agreement number and gender;
- discourse/referent number.

Uncounted nouns and count one normally use singular; 2–4 use plural;
0 and 5+ use plural nominal inflection with singular-neuter finite
agreement under the documented quantifier policy. Plural-only lexemes
always remain plural for inflection, agreement, and reference, including
count one. Coordination is plural when it contains multiple conjuncts
or a plural referent; mixed gender resolves masculine and any animate
member makes the coordination animate.

## S-expression contract

The canonical direct-object grammar is:

```text
(vp (v LEMMA)
    (adv ADVERB)*
    (recipient NOMINAL)?
    (object [:case CASE] NOMINAL)?
    (pp (prep PREPOSITION) [:case CASE] NOMINAL)*
    (oblique :case CASE NOMINAL)*)
```

The source-backed ditransitive extension is:

```text
(vp (v LEMMA)
    (recipient NOMINAL)
    (object [:case CASE] NOMINAL))
```

`recipient` is a distinct role edge with dative case. It has its own
information-structure spelling, `:topic recipient` or
`:focus recipient`, and is serialized before the direct object.

Clause-level source-backed forms use `:force wh` with `:wh SLOT` or
`:wh-adv ATOM`, `:force optative`, `:mood cond-perfect`,
`:voice passive-present`, the historical/perfect `:tense` values, and
`(initial-participle (v LEMMA) PP*)`. Short predicative adjectives use
`(pred (short-adj LEMMA))`.

Legacy direct nominal children of `(vp ...)` remain readable, but the
printer emits `(object ...)`. Case is illegal inside `(np ...)`.
Referential choice uses `:refer pron` or `:refer clitic`; full is the
default.

Safe atoms print bare. Empty strings, digit-only values, leading-colon
values, whitespace, parentheses, quotes, and backslashes trigger quoted
output. Quoted atoms escape:

| Value | Encoding |
| --- | --- |
| quote | `\"` |
| backslash | `\\` |
| newline | `\n` |
| carriage return | `\r` |
| tab | `\t` |

All other Unicode is preserved. Validation imposes
`MAX_STRUCTURE_DEPTH` before recursive consumers run, and the reader
uses a larger derived list-depth bound. This applies equally to typed
trees, parsed input, and callers that construct `Value` directly. For
every valid serializable tree, `clause_from_str(&print(tree)?) == tree`.
Raw printing validates first; only `print_validated(&ValidatedClause)`
is infallible. A deterministic bounded
generator covers hundreds of arbitrary combinations in noun,
determiner, adjective, entity, name, verb, adverb, predicate, wh-adverb,
oblique, participial-adjunct, and relative fields. Another generator
proves malformed inputs do not panic.

## Morphology boundary

The phrase crate selects facade APIs and grammatical features, orders
typed constituents, attaches punctuation, and capitalizes the first
character in sentence mode. Nouns, adjectives, pronouns, verbs,
participles, conditional auxiliaries, quantified forms, and dictionary
metadata all come from `interslavic`. Surface forms are never repaired
or rewritten after the facade returns them.

## Migration from 0.1

- Use `Clause::core`/`ClauseCore` for verbal coordination and copular
  predicates.
- Replace `np.case(CASE)` on direct objects with
  `vp.object_case(CASE, nominal)` or `Complement::new(nominal).case(CASE)`.
- Use `vp.recipient(nominal)` / `(recipient NOMINAL)` for the dative
  recipient in a ditransitive frame.
- Use `RelClause::object_gap_case(CASE, subject, vp)` for an explicitly
  marked relative object gap.
- Handle `BuildError` from `Clause::and_vp` and `Clause::conj`; these
  builders reject copular cores instead of panicking or doing nothing.
- In S-expressions, replace a direct `(np ...)` object with
  `(object (np ...))`; the reader accepts the old spelling, while
  canonical output uses the new edge.
- Consume `PhraseError::Validation` and `PhraseError::Resolution` for
  structured failures. Warning variants now contain paths.
- Use `realize_checked`/`narrate_checked` when warnings must not be
  discarded.
