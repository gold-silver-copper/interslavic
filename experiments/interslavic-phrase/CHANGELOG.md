# interslavic-phrase changelog

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
  agreement. `iže` is explicitly unsupported until the facade exposes
  its paradigm.
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
  dedicated realization entry point.
- Added quoted S-expression atoms with escaping. Canonical object output
  is now `(object [:case CASE] NOMINAL)`; `:case` is no longer accepted
  on an NP. Referential choice is serialized as `:refer pron|clitic`.
  Valid trees now satisfy `clause_from_str(print(tree)) == tree`,
  including single-item nominal coordination.

### Testing

- Preserved the 0.1 output goldens and the intended stage 1–4 behavior.
- Added direct regressions for nested relative clitics, edge-owned case,
  coordinated government, discourse case preservation, plural-only
  agreement/reference, actual-form ambiguity checks, shared validation,
  and pathful nested warnings.
- Added a complete bounded force × mood × voice × tense matrix,
  generated realization panic checks, 500+ generated escaped-atom
  roundtrips across all free-text positions, and 2,000+ generated
  malformed parser inputs.

See [ARCHITECTURE.md](ARCHITECTURE.md) for ownership, supported
combinations, serialization, and migration details.

## 0.1.0 — 2026-07-22

Initial experiment: declaratives, questions (`li`/`či`/intonation),
quantified phrases, reflexives, negation, and an S-expression surface.
