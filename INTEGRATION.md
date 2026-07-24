# Integrating `interslavic` — the downstream guide

Everything the first deep integration (the [mrzavec] game translation, which
renders every inflected word at runtime) learned the hard way, in one place.
Read this before wiring the crate into a text pipeline.

[mrzavec]: https://github.com/gold-silver-copper/rogue-rs

## Citation-form inputs

Every entry point takes the flavored (etymological) **citation form**:

- nouns: nominative singular (`žena`, `dom`; plural-only nouns their
  nominative plural — `noviny`);
- adjectives: masculine nominative singular long form (`dobry`, `svěži`);
- verbs: the infinitive (`pisati`, `ukrasti`);
- pronouns/numerals: the dictionary lemma (`toj`, `ja`, `dva`, `pŕvy`).

Inputs are `trim()`med but never spell-corrected: `deset` is not `desęť`.

## The `" / "` byform convention — and why its order is a contract

Cells with real byforms return one string joined with `" / "`
(`noun("oko", Nom, Plural)` → `"oči / očesa"`). Two rules:

1. **Take the first variant** when you need exactly one form — that is the
   recommended byform, and every convention in the crate puts it first.
2. **Byform order is API.** Consumers bless first-variant outputs into
   test expectations, so reordering variants is a breaking change here
   (see CHANGELOG policy). The whole dictionary's cell output is pinned by
   a fingerprint test; any release that changes any cell enumerates the
   delta in the changelog. You can rely on forms being stable across
   minor releases unless the changelog says otherwise.

## Clean vs `_raw` verb paradigms

`verb_forms()` / `try_verb_forms()` return **surface-ready** cells: the
builders' internal marker `ĵ` is flattened to `j` (`počivaj`, not
`počivaĵ`). Parenthesized conventions are still present — `"jesm
pisal(a)"`, `"osvětljený (osvětljená, osvětljenó)"` — and
`cells::variants` is the one canonical way to expand any cell into plain
forms.

`verb_forms_raw()` / `try_verb_forms_raw()` / `verb_forms_with_metadata()`
return the marker-carrying cells byte-identical to the JS reference
(`@interslavic/utils`); that is what the parity harness compares. Use them
only if you normalize cells yourself or diff against the reference.

## Pronouns: `PronounStyle` and the n- forms

Personal pronouns carry three form series, selected explicitly:

- `Full` — `mene`, `tebe`, `jego`;
- `Clitic` — only the attested set (`mę`/`mi`, `tę`/`ti`, `go`/`mu`,
  `sę`/`si`); every other cell is `None`. A `None` means "no such form
  exists" — render the full form instead, don't invent one;
- `AfterPreposition` — always safe after a preposition: the 3rd-person
  n- forms (`od njego`, `s njim`, `k njej`), plain full forms elsewhere.

The reflexive (`reflexive_pronoun`) has no nominative — that cell is
`None` by design, not a bug.

## The perfect tense: `perfect_parts`

Don't parse `"(je) ukradla"` — call
`perfect_parts(infinitive, person, number, gender)`:

- `auxiliary` is `Some("jesm"/"jesi"/"jesmo"/"jeste")` in the 1st/2nd
  persons and `None` in the 3rd (the standard drops it: "ona ukradla").
  Add the emphatic `je`/`jest`/`sųt` yourself if you want it;
- `participle` is the correctly-gendered l-participle with no bracket
  conventions, guaranteed identical to the participle inside
  `verb_forms().perfect`.

`l_participle()` is the same participle standalone, for fixed-gender past
subjects ("strěla tę ubila").

## Counting: `quantified`

`quantified(n, lemma, case, gender, animacy)` returns the correctly
governed noun form for a counted phrase; **you render `n` as digits** ("5
zlåtnikov") — spelled-out numerals are deliberately out of scope, and the
grammar itself recommends digits. The rules (1 → nom sg, 2–4 → nom pl,
5+/0/compounds → gen pl in direct cases; phrase case in oblique slots;
the *masculine* animate accusative of 2–4 goes genitive, agreeing with
`dvoh`/`trěh`, while feminine/neuter animates stay accusative to agree
with `dvě`) are documented on the function, including which parts are
sourced and which are stated policy.

Plural-only nouns take collective government (gen pl — "dvoje novin",
never \*"dvě noviny"). Detection is automatic for dictionary lemmas; for
out-of-dictionary pluralia tantum pass `quantified_with_info(…,
plural_only: true)` — the canonical example `dveri` is itself not a
dictionary row.

Counting a *phrase* (agreeing adjective + noun)? Use
`quantified_parts()`: it returns the governed noun **plus** the case and
number the slot resolved to, which is what the adjective must decline
with (`quantified_parts(2, "mųž", Acc, M, Animate)` reports `Gen`/`Plural`
— "vidžų **dvoh velikyh mųžev**"). Never re-derive the government rules
locally to decline a modifier.

`numeral()` declines the numerals themselves, including the animate
accusatives (`dvoh`, `trěh`) selected by the `animacy` parameter.

## Dictionary metadata: `verb_info` / `noun_info`

What the inflection API consumes internally is also queryable:

- `verb_info(infinitive)` → aspect (`Ipf`/`Pf`/`Biaspectual`),
  transitivity (`Some(true)`/`Some(false)`/`None` where the dictionary
  row carries no marker), reflexivity, and object government
  (`governs: Option<Case>` from the dictionary's `(+N)` annotation).
  Pure reflexive constructions are queried by their full lemma
  (`verb_info("ostrěgati sę")`); preposition-bearing phrase rows are
  deliberately excluded because one `Option<Case>` cannot represent
  their richer structure. `None` = not an extracted dictionary verb.
- `noun_info(lemma)` → gender, animacy, plural-/singular-only,
  indeclinability, with `Provenance::Dictionary` vs `Provenance::Guessed`
  — the guess is exactly what `noun()` inflects with, so you can decide
  whether to trust it or override via `noun_with()`.

## Prepositions: government and senses

`preposition_cases("s")` → the governed case set; `preposition_senses("s")`
→ the cases with their English glosses (`s+Gen` "off, down from" vs
`s+Ins` "with"). Drive government linting from the senses table
("multiple senses exist → warn on ambiguous pairings") instead of
hand-curating suspicious pairs.

## The single source of truth — never post-process forms

The paradigm/compound-tense path is the one implementation of stem
derivation; `l_participle`, `perfect_parts`, `conditional_parts`, and the
participle decliners all derive from it and cannot disagree with
`verb_forms`. `conditional_parts()` returns the person-marked auxiliary
and gender-/number-resolved l-participle as data, so consumers never
parse a formatted paradigm cell. The same contract applies to you:
**the crate's output is final**. If a form looks wrong, the fix belongs
here (with the steen grammar and the JS reference
as arbiters) — never in a downstream string edit, which would silently
drift from every other consumer.

## Verifying output against slovowiki

If you cross-check forms with slovowiki's `check-text`/form API, read a
hit's `lemmas`/analyses fields before crediting it to a lemma: index keys
are folded surfaces, and a homograph hit can belong to an unrelated word
(`staje` is the noun `staja`'s gen.sg/nom.pl/acc.pl, not a verb form —
a misattribution that once shipped in this crate's release notes).

## A worked pattern: runtime message assembly

mrzavec's `speak()` marker engine (`mrzavec/src/lang.rs`) is one proven
shape: templates carry typed markers (`⟨acc:strěla⟩`, `⟨vpf3:ukrasti⟩`)
that resolve to `noun_with`/`quantified`/`perfect_parts` calls at render
time, so no inflected form is ever stored. Borrow the idea, not the code:
the crate's contract is that every form you need is producible from a
citation form plus grammatical features at the moment you need it.

```rust
use interslavic::*;

// "The guard stole 5 gold coins." — assembled, not stored.
// (This example is CI-tested in tests/readme_examples.rs.)
let count = 5u64;
let coin = quantified(count, "zlåtnik", Case::Acc, Gender::Masculine, Animacy::Inanimate);
let stole = perfect_parts("ukrasti", Person::Third, Number::Singular, Gender::Masculine);
let sentence = format!("Straž {} {} {}.", stole.participle, count, coin);
assert_eq!(sentence, "Straž ukradl 5 zlåtnikov.");
```
