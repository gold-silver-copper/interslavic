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

The expanded pass added every unique complete source sentence in the following
families:

| Source construction | Added sentences | Grammar support exercised |
| --- | ---: | --- |
| long and short predicative adjectives | 4 | `(adj …)` and facade-backed `(short-adj …)` |
| overt/pro-drop and reciprocal-reflexive subjects | 3 | existing subject policy and reflexive clitic domain |
| aspect, motion, perfect, and both pluperfects | 10 | dictionary aspect/motion forms plus `:tense pluperfect` and `:tense compound-pluperfect` |
| past conditional, masculine and feminine | 2 | `:mood cond-perfect` |
| third-person optative `nehaj` | 2 | `:force optative`, including Steen's postverbal adverb order |
| adverbial active participle | 1 | `(initial-participle …)` plus a bare instrumental oblique |
| passive tense/mood combinations | 7 | past/present passive participles across present, past, future, conditional, and past conditional |
| constituent questions | 5 | explicit `:wh` slots and `:wh-adv`, including passive disambiguation |
| present-passive pizza alternative | 1 | `:voice passive-present` |

That leaves this explicit skip ledger:

| Source construction | Source examples | Why it is skipped |
| --- | ---: | --- |
| final clauses with `že by` / `da by` | 4 | genuine sentences, but they require recursive subordinate-clause and punctuation/clitic ownership |
| parallel clause ellipsis | 1 | `Jedni ljudi …, drugi …` omits the second finite predicate and needs an explicit ellipsis model |
| strong focused reflexive | 1 | `Ja myju jedino sebe` needs a full reflexive-pronoun paradigm plus focus-particle attachment |
| expanded reciprocal | 1 | `Oni bijut se jedin drugogo` needs a reciprocal nominal whose two parts receive different agreement/case |
| parenthesized conditional paradigm notation | 1 | `ja byh dělal(a)` is two gender alternatives encoded as notation, not one genuine surface sentence |

The remaining multiword examples on the grammar site are paradigms, noun
phrases, preposition phrases, or metalinguistic fragments rather than
sentences, so they are intentionally outside a sentence-generation suite.

The recipient slice was selected because the source pair fixes all of the
otherwise policy-sensitive decisions directly: the recipient is dative, it
precedes the accusative theme in neutral order, and both possessive modifiers
agree with that theme. It extends the existing case-edge and clitic-domain
architecture without inventing a general valency lexicon that the facade does
not yet contain.

## Grammar contract

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

The extended corpus adds these author-declared forms:

```lisp
; constituent question
(clause SUBJECT (vp ...) :force wh :wh obj)
(clause SUBJECT (vp ...) :force wh :wh-adv kde)

; optative and historical/perfect forms
(clause THIRD-PERSON-SUBJECT (vp ...) :force optative)
(clause SUBJECT (vp ...) :tense pluperfect)
(clause SUBJECT (vp ...) :tense compound-pluperfect)
(clause SUBJECT (vp ...) :mood cond-perfect)

; passive participle choice
(clause PATIENT (vp ...) :voice passive)
(clause PATIENT (vp ...) :voice passive-present)

; clause-level participial adjunct and bare case-marked adjunct
(initial-participle (v idti) (pp ...))
(oblique :case ins NOMINAL)

; optional short predicative adjective
(pred (short-adj veliky))
```

These remain forward-generation instructions. No sentence-to-tree parser or
heuristic recovery path is introduced.

## The sample-text corpus

The grammar pages are close to exhausted; what remains on them is
paradigms and fragments. The second source vein is Steen's ten
**sample-text** pages, mined in full into `corpus/steen_samples.tsv`.

Nine of those pages publish each text three times in parallel —
etymological (flavored) Latin, standard Latin, Cyrillic. That is exactly
this corpus's fixture convention, so for those pages the expected output
is **the source's own etymological text**, used verbatim and asserted by
`published_flavored_text_is_used_verbatim_where_it_exists`. It is not
this pipeline's opinion of what the sentence should look like.
`maly_princ.html` is the exception: standard spelling plus an English
gloss, no etymological column, so its expected output comes from the
pipeline with the respelling recorded.

### Attribution

Republished on the maintained mirror with the permission of their
original author, Jan van Steenbergen. The texts themselves are by, or
translated from, other hands: Wilhelm Wisser (*Strižik*), Mary Russell
Mitford (*Naše selo*), August Schleicher (*Ovca i konji*), Antoine de
Saint-Exupéry (*Maly princ*), Aesop (*Sěverny Větr i Sȯlnce*), the
Universal Declaration of Human Rights (*declaration*), and the biblical
and liturgical texts behind *Věža Babelja* and *Otče naš*.

### Coverage

| Page | Candidates | Fixtures | Skipped |
| --- | ---: | ---: | ---: |
| `babel_text` | 12 | 0 | 12 |
| `declaration` | 2 | 0 | 2 |
| `jokes` | 24 | 2 | 22 |
| `maly_princ` | 96 | 6 | 90 |
| `northwind` | 6 | 0 | 6 |
| `otcze_nasz` | 10 | 0 | 10 |
| `schleicher` | 6 | 1 | 5 |
| `selo` | 7 | 1 | 6 |
| `volk_i_pes` | 31 | 8 | 23 |
| `wren` | 28 | 4 | 24 |
| **total** | **222** | **22** | **200** |

Twenty-two fixtures out of 222 candidates is a deliberately conservative
count. A fixture must reproduce an entire inventory row, enforced by
`fixture_source_text_matches_the_inventory_row`. Several sentences
realize byte-exactly as *fragments* of longer periods — `Pŕvy tęgal
tęžky voz` out of a three-clause sentence, `Ja nošų verigų` out of a
`kȯgda` sentence, `Verigy sųt želězne` out of a coordinated pair — and
counting those would have claimed rows the generator cannot produce.

### Skip ledger

| Reason | Rows | What it would take |
| --- | ---: | --- |
| `quotative-frame` | 55 | direct speech plus its `rěkl X` frame, including split frames; one uniform decision, not a mix |
| `adverb-position` | 17 | Steen places these adverbs after the verb; this generator places adverbs before it |
| `spelled-out-numerals` | 15 | a documented non-goal |
| `fronted-adjunct` | 13 | a clause-initial prepositional or adverbial adjunct ahead of the subject |
| `degree` | 12 | comparative forms where the facade and the sample texts disagree, and `neželi` standards |
| `postposed-possessive-and-address` | 9 | verse-order possessives after the noun; named imperative addressees |
| `fragment` | 9 | an elliptical turn with no finite predicate; an explicit ellipsis model |
| `relative-clause-shape` | 8 | gap roles beyond subj/obj/pp |
| `connective-fragment` | 7 | a subordinate clause standing as a whole sentence, or one ending in a colon |
| `dash-turn` | 7 | a dialogue turn introduced by an em dash |
| `negative-concord` | 5 | a negative pronoun alongside clausal `ne` |
| `vocative-edge` | 4 | the facade has the vocative; the phrase layer has no address slot yet |
| `dative-experiencer` | 4 | a dative argument of a predicate adjective or impersonal verb |
| `correlative` | 3 | paired correlative clauses (`kako … tako`, `čim … tym`) |
| `coordination-shape` | 3 | three-or-more-way or nested coordination |
| `long-period` | 2 | a sentence beyond what one clause tree expresses |
| `degree-adverb` | 2 | a degree adverb modifying a predicate |
| `facade-orthography` | 2 | realized correctly but for one letter; see Findings |
| `clitic-climbing` | 2 | the infinitive's clitic raised to the finite verb; deliberately not modelled |
| `nominal-complement` | 2 | a genitive complement inside a predicate noun phrase |
| `oblique-in-copular` | 2 | a bare oblique adverbial in a copular clause, which has no verb phrase to host it |
| `past-adverbial-participle` | 2 | `Uslyšavši to, …`; only present active participles exist |
| `existential` | 2 | an existential `byti` whose subject follows the locative phrase |
| `impersonal-modal` | 2 | the impersonal `možno`/`trěba` frame with an infinitive |
| `reflexive-pronoun` | 2 | the full reflexive pronoun, as against the `sę` clitic |
| `adverb-phrase` | 2 | multiword and parenthetical adverbials |
| `adjunct-order` | 1 | interleaving obliques, adverbs, and prepositional phrases in source order |
| `participle-with-object` | 1 | an adverbial participle governing an object |
| `verb-phrase-negation` | 1 | coordinated verb phrases each carrying their own `ne`, while polarity is clause-level |
| `intensifier` | 1 | the postposed intensifier `sam` |
| `conditional-clause-plus-imperative` | 1 | an `ako` clause whose matrix is an imperative |
| `purposive-infinitive` | 1 | an infinitive of purpose after an imperative of motion |
| `predicate-complement` | 1 | a predicate adjective governing a prepositional complement |

There is no `untriaged` category. Every row names a specific missing
capability, and two tests keep it that way: `skips_state_a_reason`
rejects a bare `skip:`, and `no_skip_is_left_as_the_untriaged_placeholder`
rejects the placeholder that earlier stood in for "extracted but not yet
analysed".

### Row boundaries

Rows are sentences, mined with quotation tracking. An earlier pass split
paragraphs at sentence punctuation without tracking quotation depth, so
42 rows carried stray `«`/`»` and 41 of them sat under
`skip:quotative-frame` without belonging there. A sentence inside direct
speech is still a sentence, so it is its own row; the surrounding
quotation's delimiters belong to the span, not to it, and are stripped.
`framed` marks a sentence carrying a quotative frame of its own, where
the marks are structural and stay.

Per-row `«`/`»` balance is deliberately NOT asserted, and would be the
wrong check: a quotation routinely spans several sentences, so a sentence
closing one opened three sentences earlier is correct. Demanding per-row
balance would force quotations into paragraph-sized rows that no fixture
could reproduce. Page-level balance is not asserted either, and its
failure would be a false alarm, since stripping quoted-interior
delimiters unbalances the ledger by design. What is asserted is that a
row with no frame of its own carries no quotation marks at all, and that
a row inside direct speech is preceded on its page by a frame.

### Findings

Places where the generator and the source disagree, recorded rather than
normalized away:

| Finding | Detail |
| --- | --- |
| `vųglom` vs `vųglȯm` (`wren-017`) | otherwise byte-exact; the instrumental of `vųgȯl` loses the `ȯ` |
| `hoće` vs `hȯće` (`jokes-010`) | otherwise byte-exact; the generator follows this repo's dictionary, whose present-stem hint for `hotěti` is `(hoće)`. The disagreement is between the dictionary and Steen's sample-text orthography |
| `da byhmo ne …` | Steen puts the irrealis auxiliary before the negation; this generator emits `ne` clause-initially. One witness, so no rule was added |
| `psi` and `pėśjų` unknown to slovowiki | forms Steen writes and this dictionary produces; the external checker's lexicon lacks them. Waived per-token in `xtask`'s `SLOVOWIKI_LEXICON_GAPS` with a source citation each, so those sentences stay under agreement checking; a stale entry is reported so it gets removed |
| `cells::variants` and `" / "` byforms | FIXED. `noun.rs` emits slash byforms for genuine alternative cells, and `variants` did not split them, so `den / denj` and `očami / očesami` reached output as single tokens. Fixed at the normalization boundary rather than in the caller |
| `tęžši` vs Steen's `tęžejša` | the facade's synthetic comparative of `tęžky`; same class as `hoće`/`hȯće` — repository data against sample-text orthography |
| clitic order is not uniform in Steen | `Ja myju se` (pronouns page) is postverbal, `Ja go shvaću` (wren) is second-position. Both are supported styles; each fixture records which it uses |

## Executed source cases

`tests/steen_sexpr.rs` and `tests/steen_new_grammar.rs` contain 47 literal
source fixtures totaling 172 whitespace-delimited generated sentence tokens:

- the original 12-question/pronoun/preposition/passive slice;
- 35 newly expressible sentences from the adjective, pronoun, verb, and syntax
  pages.

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
The same policy accounts for `je`→`jest`, `budu`→`bųdų`, nasal-vowel
spellings, the facade's auxiliary-before-participle perfect order, and its
explicit first-person perfect auxiliary in `Ja jesm byl neseny`.

No phrase-specific inflected form or post-realization repair is used.
