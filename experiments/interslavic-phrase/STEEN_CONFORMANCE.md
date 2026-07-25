# Steen S-expression conformance

This corpus treats the forward realization contract as executable data:

```text
literal S-expression → validated syntax tree → exact sentence
```

The original sources are Jan van Steenbergen's grammar pages. During this
work the original host was intermittently unreachable, so the audit also used
the maintained [Interslavic grammar republication][mirror]. Each republished
page states that it appears with the original author's permission and links
back to the corresponding Steen page.

[mirror]: https://interslavic.fun/learn/grammar/

## Coverage audit

| Source and section | Original source text | Exact generated text / normalization | Construction | Prior expressibility and result |
| --- | --- | --- | --- | --- |
| [Syntax: questions][syntax] | `Otec kupil knigu?` | `Otėc kupil knigų?` (`otec`, `knigu` respelled) | intonation question | expressible; now executed from a literal S-expression |
| [Syntax: questions][syntax] | `Či otec kupil knigu?` | `Či otėc kupil knigų?` (`otec`, `knigu` respelled) | `či` question | expressible; now executed from a literal S-expression |
| [Syntax: questions][syntax] | `Kupil li otec knigu?` | `Kupil li otėc knigų?` (`otec`, `knigu` respelled) | second-position `li` | expressible; now executed from a literal S-expression |
| [Pronouns: personal][pronouns] | `Ja myju se` | `Ja myjų sę.` (`myju`, `se` respelled; punctuation added) | reflexive clitic | expressible; now executed from a literal S-expression |
| [Pronouns: possessive][pronouns] | `Ja myju svoje avto` | `Ja myjų svoje avto.` (`myju` respelled; punctuation added) | reflexive possessive | expressible; now executed from a literal S-expression |
| [Pronouns: possessive][pronouns] | `Pjotr dal Ivanu svoju knigu` | `Pjotr dal Ivanu svojų knigų.` (`svoju`, `knigu` respelled; punctuation added) | dative recipient + accusative theme | **previously blocked:** a VP had only one complement edge; now implemented with `(recipient ...)` + `(object ...)` |
| [Pronouns: possessive][pronouns] | `Pjotr dal Ivanu jegovu knigu` | `Pjotr dal Ivanu jegovų knigų.` (`jegovu`, `knigu` respelled; punctuation added) | dative recipient + accusative theme | **previously blocked:** the same structural limit; now implemented and independently exercised |
| [Prepositions: accusative/instrumental][prepositions] | `Kot spi pod stolom` | `Kot spi pod stolom.` (punctuation added) | stable location with instrumental | expressible; now executed from a literal S-expression |
| [Prepositions: accusative/instrumental][prepositions] | `Kot poběgl pod stol` | `Kot poběgl pod stol.` (punctuation added) | motion toward with accusative | expressible; now executed from a literal S-expression |
| [Syntax: passive voice][syntax] | `Pica je dělana` | `Pica jest dělana.` (facade-preferred full copula; punctuation added) | participial passive | expressible; now executed from a literal S-expression |
| [Syntax: passive voice][syntax] | `Dělajut picu` | `Dělajųt picų.` (`dělajut`, `picu` respelled; punctuation added) | impersonal active paraphrase | expressible; now executed from a literal S-expression |
| [Syntax: passive voice][syntax] | `Pica dělaje se` | `Pica dělaje sę.` (`se` respelled; punctuation added) | reflexive passive paraphrase | expressible; now executed from a literal S-expression |

[syntax]: https://interslavic.fun/learn/grammar/syntax/
[pronouns]: https://interslavic.fun/learn/grammar/pronouns/
[prepositions]: https://interslavic.fun/learn/grammar/prepositions/

The same audit found coherent source families that remain unsupported:

| Source construction | Source examples | Architectural blocker |
| --- | ---: | --- |
| constituent questions | 5 | needs interrogative nominal/adverb roles rather than repurposed topic/focus |
| final clauses with `že by` / `da by` | 4 | needs an embedded-clause edge and subordinate clitic/punctuation ownership |
| third-person optative `nehaj` | 2 | needs a distinct optative force, not an imperative-person workaround |
| adverbial active participle | 1 | needs a clause-level participial adjunct |
| short-form predicative adjectives | 4 | needs a facade-backed short-form predicate choice |

The recipient slice was selected because the source pair fixes all of the
otherwise policy-sensitive decisions directly: the recipient is dative, it
precedes the accusative theme in neutral order, and both possessive modifiers
agree with that theme. It extends the existing case-edge and clitic-domain
architecture without inventing a general valency lexicon that the facade does
not yet contain.

## New grammar contract

The canonical ditransitive S-expression is:

```lisp
(vp (v dati)
    (recipient (name Ivan :m))
    (object (np (det svoj) (n kniga))))
```

`recipient` is a grammatical-role edge and therefore owns dative case.
`object` keeps its existing dictionary/default/explicit case resolution.
Neutral full-form order is verb–recipient–object. Direct recipient and object
clitics remain in their owning VP domain and use dative–accusative order before
the reflexive marker.

This edge is author-declared rather than inferred. The current dictionary can
check direct transitivity and direct-object government, but contains no
indirect-object frame metadata. The phrase crate therefore does not pretend to
prove that any arbitrary verb licenses a recipient.

The typed equivalent is:

```rust
vp("dati")
    .recipient(name("Ivan", Gender::Masculine))
    .object(np("kniga").det("svoj"))
```

`SlotRef::Recipient` and canonical `:topic recipient` / `:focus recipient`
allow the new constituent to participate in information structure without
being confused with the direct object.

## Executed source cases

`tests/steen_sexpr.rs` contains 12 literal source fixtures totaling 44
whitespace-delimited sentence tokens:

- 3 question examples;
- 2 possessive/reflexive examples already expressible by the old grammar;
- 2 newly expressible ditransitive possessive examples;
- 2 spatial-preposition examples;
- 3 passive/impersonal alternatives.

Each fixture stores:

- the Steen URL and original standard-orthography text;
- every normalization applied;
- the literal S-expression;
- the exact expected facade spelling.

Every case is parsed, explicitly validated, realized byte-exactly, printed
canonically, reparsed, and realized again.

## Normalization policy

The source text is preserved separately from the expected output. Expected
strings use the repository's established etymological/flavored orthography,
including `otėc`, `knigų`, `svojų`, `jegovų`, `myjų`, `sę`, and `dělajųt`.
Sentence punctuation is added where the grammar page presents a bare example.
For `Pica je dělana`, the expected output uses the facade's preferred full
copula `jest`; the source's `je` remains recorded in the fixture.

No phrase-specific inflected form or post-realization repair is used.
