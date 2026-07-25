# interslavic-phrase changelog

## Unreleased

### Grammar

- Added a dedicated dative `Recipient` role and canonical
  `(recipient NOMINAL)` S-expression edge. Ditransitive VPs realize
  full complements in the source-attested verb–recipient–object order.
- Recipient and direct-object clitics share one VP domain in
  dative–accusative order. `SlotRef::Recipient` and
  `:topic recipient` / `:focus recipient` expose the new constituent to
  information structure, while passive clauses retain it after theme
  promotion.
- Discourse traversal now tracks recipients in their typed surface
  position and changes only their referential form, preserving the
  dative role edge.
- Added source-backed constituent questions (`:force wh` with `:wh` or
  `:wh-adv`) and third-person optatives (`:force optative`).
- Added the optional simple past, simple and compound pluperfect, past
  conditional, and present-passive voice. Passive realization now
  selects the documented past or present passive participle across
  tense and conditional combinations.
- Added clause-initial active adverbial participles, case-owning bare
  obliques, and optional short predicative adjectives. The morphology
  facade now exposes present-passive and active-adverbial participle
  helpers plus short adjective forms.

- Added finite subordinate clauses: `(sub :comp že|da|kȯgda|ako|zato-že|
  tomu-že [:pos initial|final] CLAUSE)`, as a verb's complement inside
  `(vp …)` or a clause adverbial inside `(clause …)`. An embedded clause
  is a full clause and recurses through the same validation, resolution,
  and planning as a matrix clause. It owns its own clitic domain, and
  terminal punctuation and capitalization remain with the matrix
  sentence's single stringification pass. `da by` is not a unit: the
  complementizer supplies `da` and the embedded clause's conditional
  mood supplies the person-marked `by` / `byh` / `byhmo`.
- Added `MAX_CLAUSE_DEPTH`, enforced in the S-expression preflight
  before the recursive compiler runs. The generic `MAX_STRUCTURE_DEPTH`
  counts list levels, which a clause is cheap in, so it permitted
  nesting that exhausted the stack instead of producing a diagnostic.
- Added bare infinitive complements, `(inf …)`, taking exactly the
  children of `(vp …)`. The infinitive owns its object, valence check,
  government, and clitic domain — Steen's `mogų slomiti ti hrėbet`.
  Clitic climbing (`načęl go napominati`) is deliberately not modelled.
- Added unquantified plural noun phrases, `(np :pl …)`. Bare plurals
  (`vsi psi`, `moje děti`, `vlåsy`) previously required attaching a
  numeral the phrase does not have. A numeral still determines number;
  combining the two is rejected rather than silently resolved.
- Fixed the `kȯgda` complementizer, which had been hardcoded in
  standard spelling in a crate whose every leaf is a flavored citation
  form. The other hardcoded function words were audited against the
  dictionary and are correct.

### Facade

- Added `vocative()` and `vocative_with()`. Deliberately not a seventh
  `Case`: the source states the vocative "is not a real case", has no
  plural, and never affects neuter nouns, adjectives, or pronouns.
  `None` encodes the source's own advice to use the nominative instead.

### Conformance

- Added the Steen sample-text corpus: a committed ledger of all 220
  candidate sentences (`corpus/steen_samples.tsv`) and 14 fixtures.
  Every row carries a `fixture:` or `skip:<reason>` disposition and
  `tests/corpus_inventory.rs` fails on any untriaged row, so coverage
  is a property of committed data. Ten of the fourteen check their
  expected output against the page's own published etymological text.
  A fixture must reproduce a whole inventory row, so sentences that
  realize exactly only as fragments are skips, not fixtures.
- `cargo xtask phrase-check` now also covers the sample fixtures: 175
  tokens, 0 unknown, 0 agreement errors.
- Added 47 literal Steen S-expression fixtures (172 sentence tokens).
  Every fixture records its source text and normalization, realizes
  byte-exactly, then survives canonical print/reparse/rerealization.
- Added focused regressions for recipient case and order, combined
  dative/accusative clitics, information structure, passive retention,
  discourse pronominalization, malformed syntax, and full-vocabulary
  S-expression round-trip.
- Expanded the bounded force × mood × voice × tense matrix from 84 to
  486 combinations and documented the eight deliberately skipped Steen
  examples: four embedded final clauses, three ellipsis/complex
  reflexive constructions, and one paradigm notation string.

## 0.2.0 — 2026-07-24

Breaking redesign of the experimental phrase API. Existing 0.1 builder
entry points remain where their semantics are unambiguous; struct layout
and S-expression canonical output changed.

### Language features

- Copular clauses with nominal, adjectival, and passive-participial
  predicates; nominal predicates default to nominative and may request
  instrumental.
- Active/passive voice, imperative addressees, conditional mood, aspect
  warnings, adverbs, proper names, and multiple prepositional phrases.
- Dictionary-backed verb government, including pure reflexive
  constructions, with explicit conflict diagnostics.
- Subject, object, and prepositional relative gaps with `ktory`
  agreement. Object gaps may explicitly override case and share the
  overt-object government resolver. `iže` is explicitly unsupported
  until the facade exposes its paradigm.
- Nominal and VP coordination, postverbal and second-position clitic
  styles, per-verb clitic domains, topic/focus ordering, and pathful
  ambiguity warnings.
- Entity-aware discourse pronominalization, interference, safe VP
  aggregation, connectives, and a checked API that preserves warnings.

### Architecture and correctness

- Added the explicit raw AST → shared validation → resolved grammar →
  hierarchical surface plan → final stringification pipeline.
- Moved case authority from `NounPhrase` to grammatical-role edges.
  `Complement` represents an optional explicit direct-object case;
  prepositions, predicates, subjects, and relative gaps retain their
  own authority. Every resolved nominal slot has one case and source.
- Replaced recursive flat-token clitic extraction with nested
  `NominalPlan`, `RelativePlan`, and per-VP `VerbDomainPlan` values.
  Relative and coordinated-VP clitics cannot migrate into a parent
  domain, and second-position placement cannot split a constituent.
- Made discourse referential choice non-lossy: an entity-bearing NP is
  retained and marked `Full`, `Pronoun`, or `Clitic`; its grammatical
  role and explicit complement case remain outside that choice.
- Centralized person, gender, animacy, inflection number, finite
  agreement, and referent number in one nominal-profile resolver.
  Plural-only nouns remain grammatically and referentially plural even
  under an explicit count of one.
- Added structured, pathful validation and resolution diagnostics.
  Invalid raw trees fail before planning; already validated trees have a
  dedicated realization entry point. A shared structure-depth invariant
  protects typed trees, parsed input, and direct `Value` compilation.
- Added quoted S-expression atoms with escaping. Canonical object output
  is now `(object [:case CASE] NOMINAL)`; `:case` is no longer accepted
  on an NP. Referential choice is serialized as `:refer pron|clitic`.
  Valid trees now satisfy `clause_from_str(&print(tree)?) == tree`;
  raw printing validates first and `print_validated` accepts the sealed
  validated type. This includes single-item nominal coordination and
  quoted delimiter atoms.
- Made generic object topic/focus target the first object that actually
  exists, kept later objects with their owning VP under default `li`
  order, and made discourse salience follow typed constituent order
  after information-structure movement.
- Coalesced coincident punctuation boundaries, such as a relative-clause
  closing comma that is also a coordination delimiter.

### Testing

- Preserved the 0.1 output goldens and the intended stage 1–4 behavior.
- Added direct regressions for nested relative clitics, edge-owned case,
  coordinated government, discourse case preservation, plural-only
  agreement/reference, actual-form ambiguity checks, shared validation,
  and pathful nested warnings.
- Added a complete bounded force × mood × voice × tense matrix,
  generated realization panic checks, 500+ generated escaped-atom
  roundtrips across all free-text positions, and 2,000+ generated
  malformed S-expression inputs.

See [ARCHITECTURE.md](ARCHITECTURE.md) for ownership, supported
combinations, serialization, and migration details.

## 0.1.0 — 2026-07-22

Initial experiment: declaratives, questions (`li`/`či`/intonation),
quantified phrases, reflexives, negation, and an S-expression surface.
