# interslavic-phrase (experiment)

`interslavic-phrase` turns typed syntax trees into Interslavic text. It
chooses grammatical features and word order; every inflected word still
comes from the [`interslavic`](../../crates/interslavic) facade.

```rust
use interslavic_phrase::*;

let raw = clause_from_str(
    "(clause
       (np (det toj) (adj dobry) (n krålj))
       (vp (v ukrasti)
           (object (np (num 5) (adj zlåty) (n moneta))))
       :tense past)",
).unwrap();

assert_eq!(
    realize(&raw, RealizeOpts::sentence()).unwrap(),
    "Toj dobry krålj ukradl 5 zlåtyh monet."
);
```

Version 0.2 adds copular predicates, active/passive voice,
imperatives, conditionals, dictionary-backed government, relative
gaps, nominal and VP coordination, clitic styles, topic/focus order,
and discourse microplanning. The complete design and ownership rules
are in [ARCHITECTURE.md](ARCHITECTURE.md).

## Pipeline

Both authoring surfaces produce the same raw tree:

```text
typed builders ─┐
                ├─> RawClause ─> validate ─> ValidatedClause
S-expression ───┘                    │
                                     v
                   discourse ─> grammar resolution
                                     │
                                     v
                   hierarchical surface plan ─> one final stringify
```

`realize_checked` validates automatically and returns pathful warnings.
Call `validate` explicitly when a validated tree will be reused, then
pass it to `realize_validated_checked`. `realize` is the convenience
form that discards warnings. The discourse equivalents are
`narrate_checked` and `narrate`.

The important boundaries are structural:

- `NounPhrase` has no case. `Complement`, `PrepPhrase`, predicate
  position, and relative gaps own case constraints. Resolution assigns
  one case to the entire governed nominal, including all coordination
  members.
- Every verb and relative clause owns a clitic domain. A parent can
  extract only a direct clitic object, never tokens nested inside an NP
  or relative.
- Discourse pronominalization changes `ReferentialForm` on the existing
  NP. It does not replace the NP or discard its entity, role, case, or
  lexical content.
- One nominal-profile resolver supplies inflection, finite agreement,
  and referent number to realization and discourse.

## S-expression data surface

The canonical direct-object form exposes the grammatical edge:

```text
(object [:case nom|acc|gen|loc|dat|ins] NOMINAL)
```

Case is not legal inside `(np ...)`. Strings that are not safe bare
atoms are quoted. The reader and printer escape `"`, `\`, newline,
carriage return, and tab, and preserve spaces, leading colons,
parentheses, arbitrary Unicode, multiword names, and entity IDs.
For every valid serializable tree, `clause_from_str(&print(tree)?) ==
tree`; raw printing validates first, while `print_validated` is the
infallible boundary for `ValidatedClause`. Bounded-generative tests
exercise escaped leaves and malformed input.

The early 0.1 spelling with a nominal directly inside `(vp ...)` is
accepted for migration, but `print` always emits `(object ...)`.
Object-relative gaps may likewise own an explicit case as
`:gap obj :case CASE`.

## Validation and verification

Validation rejects empty coordination, contradictory relative gaps,
invalid preposition/case pairs, passive clauses retaining an object,
unsupported force/mood/voice/tense combinations, invalid predicate
case, missing or duplicate information-structure slots, and
pronominal references that would suppress a relative proposition, and
trees deeper than the shared `MAX_STRUCTURE_DEPTH` safety bound.
Resolution separately reports dictionary-valence errors and government
conflicts.

The package test suite includes the original 0.1 goldens, all intended
0.2 constructions, architecture regressions, an exhaustive bounded
force × mood × voice × tense matrix, generated atom roundtrips, and
generated malformed parser inputs. `cargo xtask phrase-check` sends the
golden corpus through slovowiki's independent agreement checker; set
`SLOVOWIKI_DIR` when it is not in the default sibling location.

## Deliberately unsupported

- genitive of negation (negated transitives retain their resolved case)
- the `iže` relativizer, because the facade has no paradigm
- passive imperatives
- preposition-phrasal verb government such as `bazovati na`
- clitic arguments beyond the represented direct-object/reflexive set
- parsing free Interslavic text into syntax trees
- spelled-out numerals

This crate is published as an experiment. Its 0.2 API intentionally
breaks the 0.1 struct layout.
