# interslavic-phrase (experiment)

Typed Interslavic syntax trees with an S-expression surface, realized
into sentences through the [`interslavic`](../../crates/interslavic)
inflection crate. Design rationale and research anchors:
[`PROPOSAL_SYNTAX.md`](../../PROPOSAL_SYNTAX.md) (GF-style
abstract/concrete split; dependency-shaped trees because Interslavic's
hard problems are agreement, government, and clitics — not word order).

```rust
use interslavic_phrase::*;

let tree = clause_from_str(
    "(clause (np (det toj) (adj dobry) (n krålj)) \
             (vp (v ukrasti) (np (num 5) (adj zlåty) (n moneta))) \
             :tense past)",
).unwrap();

assert_eq!(
    realize(&tree, RealizeOpts::sentence()).unwrap(),
    "Toj dobry krålj ukradl 5 zlåtyh monet."
);
```

The tree does the grammar: the counted object resolves through
`quantified_parts`, so the adjective agrees in Gen/Plural; pronouns
inside PPs take the prepositional n- forms; multi-case prepositions
demand an explicit case (the error lists the senses); valence,
reflexivity, aspect, and object government come from `verb_info` ("Ja
dękujų tobě" — the dative from the dictionary's (+3)); the past tense
agrees in gender via `perfect_parts`. 0.2.0 adds copular predicates
("Komnata jest osvětljena"), the passive ("Kniga byla kupjena od
otca"), imperatives and conditionals, relative clauses ("moneta, ktorų
krålj ukradl"), coordination, clitic styles, topic/focus word order
("Knigų li kupil otėc?"), and a discourse module (entity-tracked
pronominalization, aggregation, connectives). Linearization defaults
are steen's, cited at the point of use; where the sources are silent
the docs say policy.

Two authoring surfaces, one AST: typed builders (`clause(np(..), vp(..))`)
and the S-expression reader (`clause_from_str`), with a canonical printer
(`parse ∘ print = id`, property-tested).

## Validation

- `tests/goldens.rs`: steen's own example sentences, re-spelled to the
  flavored orthography and pinned byte-exact.
- `cargo xtask phrase-check`: renders every golden and runs it through
  slovowiki's independent agreement checker (local sibling checkout;
  `SLOVOWIKI_DIR` env var). Current status: 112 tokens, 0 unknown,
  0 agreement errors.

## Deliberately deferred

Genitive of negation (negated transitives keep Acc — policy, sources
permissive), the `iže` relativizer (declared-unsupported; the facade
has no paradigm for it — a known facade gap), clitic clusters beyond
li/dat/acc/sę, parsing Interslavic text back into trees (stage 5 of
`phrase-improvement.md`), and spelled-out numerals (the 0.12.0 decision
stands).

Published to crates.io as an experiment (`0.2.x`): the API is expected
to move; pin exactly if you depend on it.
