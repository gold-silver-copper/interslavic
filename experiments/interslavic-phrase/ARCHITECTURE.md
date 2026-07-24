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
| direct-object constraint | `Complement` | An explicit override applies to the whole complement and can be compared with dictionary government once. |
| PP case | `PrepPhrase` | Validation checks the selected case against the preposition before resolution. |
| predicate case | copular role edge | A predicate NP cannot override `PredCase`. |
| relative-gap case | `GapRole` plus verb government | `ktory` receives the resolved gap case; no child NP can change it. |
| finite and nominal features | central `NominalProfile` | Realization and discourse share one count/plural-only/coordination policy. |
| referential choice | `NounPhrase::referential` | Discourse requests a pronoun without replacing or impoverishing the syntax node. |
| clitic placement | `VerbDomainPlan` | Only that VP's direct clitic object and reflexive marker enter its cluster. |
| nested relative content | `RelativePlan` | Parent traversal cannot inspect or extract descendant tokens. |
| punctuation and casing | `ClausePlan::stringify` | Surface nodes flatten once; no morphology is rewritten afterward. |

Case resolution records whether a case came from subject position,
default accusative, dictionary government, an explicit object edge,
preposition, or predicate position. Coordination members receive the
same resolved case recursively. This prevents PP objects, nested NPs,
predicate NPs, or individual conjuncts from reopening the decision.

## Validation and resolution

Validation returns `ValidationErrors(Vec<ValidationError>)`. Every error
has an `AstPath`, such as `clause.core.vp[0].object`. It checks:

- non-empty coordination and non-empty leaves;
- imperative/conditional/passive/tense coherence;
- passive patient promotion (no retained direct object);
- predicate-case applicability;
- topic/focus existence and duplicate references;
- relative gap versus overt subject/object coherence;
- ordinary and relative-gap preposition government;
- referential choices that would suppress a relative proposition.
- a shared maximum syntax depth before any recursive consumer runs.

The supported combination rule is compact:

- indicative non-imperative verbal clauses support present, past, and
  future in active or passive voice; copular clauses are active;
- conditionals use their own auxiliary/participle construction and
  cannot carry independent past/future tense;
- imperatives are present, active, and indicative;
- declarative and all three question forces can combine with supported
  non-imperative shapes.

Dictionary valence requires lexical metadata and therefore belongs to
grammar resolution. `ResolutionErrors` is also pathful. An explicit
overt-object or object-gap case that differs from dictionary government
remains realizable and emits one `GovernsConflict` warning; an object on
a dictionary-intransitive verb is an error. Object gaps pass through the
same case, valence, and government resolver as overt objects.

## Hierarchical clitic domains

A partially planned nominal is never a flat token vector. A
`NominalPlan` contains opaque child nodes and may expose exactly one
`direct_clitic` only when the nominal itself is a clitic pronoun.
`RelativePlan` keeps the relative body nested until its own VP has
placed its cluster.

Each `VerbDomainPlan` owns a cluster ordered `li > dative > accusative >
sę` for the arguments represented by this grammar. Postverbal placement
inserts it after that VP's complex. Second-position placement inserts it
after the first typed constituent of the domain. Coordinated VPs are
separate domains; a discourse connective is outside the first domain.

Consequently:

```text
Žena vidi mųža, ktory myje sę.
Žena vidi mųža, ktory vidi go.
```

The relative `sę` and `go` are not visible to the parent VP.

## Discourse planning

`narrate_checked` validates every input before discourse planning.
Planning then operates on owned syntax-tree clones, and each transformed
output is validated again before resolution. Entity tracking stores
referent features from the central nominal profile and visits mentions
in typed surface order, including topic/focus movement. On a repeated
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
    (object [:case CASE] NOMINAL)
    (adv ADVERB)*
    (pp (prep PREPOSITION) [:case CASE] NOMINAL)*)
```

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
every valid serializable tree, `clause_from_str(print(tree)) == tree`.
A deterministic bounded
generator covers hundreds of arbitrary combinations in noun,
determiner, adjective, entity, name, verb, adverb, predicate, and
relative fields. Another generator proves malformed inputs do not
panic.

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
