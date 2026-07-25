//! Source conformance against Steen's published sample texts.
//!
//! Nine of the ten sample-text pages publish each text three times in
//! parallel — etymological (flavored) Latin, standard Latin, Cyrillic.
//! That is the strongest evidence available for this generator, because
//! `expected` is not invented here: it is the etymological text as
//! published. `source_text` is the standard-Latin version, and
//! `normalization` records every difference between the two.
//!
//! `maly_princ.html` is the exception. It pairs standard-spelling
//! Interslavic with an English gloss and has no etymological column, so
//! `expected` there comes from the pipeline and `normalization` records
//! the respelling. Those rows are marked in `flavored_published`.
//!
//! Every case id must appear in `corpus/steen_samples.tsv` with a
//! matching `fixture:` disposition — asserted below — so a fixture
//! cannot exist without a ledger row accounting for it, and the ledger
//! cannot claim coverage a fixture does not deliver.
//!
//! The sample texts are republished on the maintained mirror with the
//! permission of their original author, Jan van Steenbergen. The texts
//! themselves have their own upstream authors and translators, cited per
//! page in `STEEN_CONFORMANCE.md`.

mod support;

use interslavic_phrase::{
    CliticStyle, RealizeOpts, clause_from_str, print, realize_with_lead_in, validate,
};

struct SteenCase {
    /// Matches the `fixture:` disposition of an inventory row.
    id: &'static str,
    source: &'static str,
    /// The text exactly as the standard-Latin version prints it.
    source_text: &'static str,
    /// Whether `expected` is the source's own etymological text (true)
    /// or this pipeline's output for a standard-only page (false).
    flavored_published: bool,
    normalization: &'static str,
    /// Discourse connective, realized through the crate's own lead-in
    /// path rather than smuggled into the tree.
    lead_in: Option<&'static str>,
    clitics: CliticStyle,
    sexpr: &'static str,
    expected: &'static str,
}

const WREN: &str = "https://steen.free.fr/interslavic/wren.html";
const VOLK: &str = "https://steen.free.fr/interslavic/volk_i_pes.html";
const SELO: &str = "https://steen.free.fr/interslavic/selo.html";
const PRINC: &str = "https://steen.free.fr/interslavic/maly_princ.html";
const JOKES: &str = "https://steen.free.fr/interslavic/jokes.html";
const SCHLEICHER: &str = "https://steen.free.fr/interslavic/schleicher.html";

const CASES: &[SteenCase] = &[
    // -- Strižik (The Wren) -------------------------------------------
    SteenCase {
        id: "wren-001",
        source: WREN,
        source_text: "Strižik iměl svoje gnězdo v garažu.",
        flavored_published: true,
        normalization: "None; the two published versions are identical here.",
        lead_in: None,
        clitics: CliticStyle::Postverbal,
        sexpr: "(clause (name Strižik :m) \
                  (vp (v iměti) (object (np (det svoj) (n gnězdo))) \
                      (pp (prep v) :case loc (np (n garaž)))) \
                  :tense past)",
        expected: "Strižik iměl svoje gnězdo v garažu.",
    },
    SteenCase {
        id: "wren-015",
        source: WREN,
        source_text: "Ja go shvaču.",
        flavored_published: true,
        normalization: "`shvaču`→`shvaćų`. The sentence sits inside direct \
                        speech; the surrounding quotation's delimiters \
                        belong to that span, not to this sentence.",
        lead_in: None,
        // Steen puts the accusative clitic before the verb here, which is
        // the crate's documented second-position style rather than its
        // postverbal default. Both orders are attested in Steen: the
        // pronouns page has `Ja myju se`, this text has `Ja go shvaću`.
        clitics: CliticStyle::SecondPosition,
        sexpr: "(clause (pron :1 :sg :m) \
                  (vp (v shvatiti) (object (pron :3 :sg :n :clitic))))",
        expected: "Ja go shvaćų.",
    },
    SteenCase {
        id: "wren-016",
        source: WREN,
        source_text: "Potom on odletěl za čudoviščem.",
        flavored_published: true,
        normalization: "None; `Potom` is realized as a discourse lead-in.",
        lead_in: Some("Potom"),
        clitics: CliticStyle::Postverbal,
        sexpr: "(clause (pron :3 :sg :m) \
                  (vp (v odletěti) (pp (prep za) :case ins (np (n čudovišče)))) \
                  :tense past)",
        expected: "Potom on odletěl za čudoviščem.",
    },
    SteenCase {
        id: "wren-018",
        source: WREN,
        source_text: "Ale strižik ne iměl strah.",
        flavored_published: true,
        normalization: "None; `Ale` is realized as a discourse lead-in.",
        lead_in: Some("Ale"),
        clitics: CliticStyle::Postverbal,
        sexpr: "(clause (name strižik :m) \
                  (vp (v iměti) (object (np (n strah)))) \
                  :tense past :neg)",
        expected: "Ale strižik ne iměl strah.",
    },
    // -- Vȯlk i pės (The Wolf and the Dog) -----------------------------
    SteenCase {
        id: "volk_i_pes-002",
        source: VOLK,
        source_text: "Ale ty jesi veliky i tolsty.",
        flavored_published: true,
        normalization: "`tolsty`→`tȯlsty`; `Ale` is realized as a discourse \
                        lead-in. The sentence sits inside direct speech.",
        lead_in: Some("Ale"),
        clitics: CliticStyle::Postverbal,
        // One copula, two coordinated predicates.
        sexpr: "(clause (pron :2 :sg :m) (pred (adj veliky) (adj tȯlsty)))",
        expected: "Ale ty jesi veliky i tȯlsty.",
    },
    SteenCase {
        id: "volk_i_pes-022",
        source: VOLK,
        source_text: "Verigy sut želězne, a želězo jest tvrdo.",
        flavored_published: true,
        normalization: "`sut`→`sųt`; `tvrdo`→`tvŕdo`. The sentence sits \
                        inside direct speech.",
        lead_in: None,
        clitics: CliticStyle::Postverbal,
        // Two full clauses with distinct subjects, joined by `a`. Not
        // verb-phrase coordination, which shares one subject.
        sexpr: "(clause (np :pl (n veriga)) (pred (adj želězny)) \
                  (and-clause :conj a \
                    (clause (np (n želězo)) (pred (adj tvŕdy)))))",
        expected: "Verigy sųt želězne, a želězo jest tvŕdo.",
    },
    SteenCase {
        id: "volk_i_pes-003",
        source: VOLK,
        source_text: "Kde dostavaješ svoju jedu?",
        flavored_published: true,
        normalization: "`svoju jedu`→`svojų jedų`.",
        lead_in: None,
        clitics: CliticStyle::Postverbal,
        sexpr: "(clause (pron :2 :sg :m) \
                  (vp (v dostavati) (object (np (det svoj) (n jeda)))) \
                  :force wh :wh-adv kde :prodrop)",
        expected: "Kde dostavaješ svojų jedų?",
    },
    SteenCase {
        id: "volk_i_pes-004",
        source: VOLK,
        source_text: "Kde živeš?",
        flavored_published: true,
        normalization: "None; the two published versions are identical here.",
        lead_in: None,
        clitics: CliticStyle::Postverbal,
        sexpr: "(clause (pron :2 :sg :m) (vp (v žiti)) :force wh :wh-adv kde :prodrop)",
        expected: "Kde živeš?",
    },
    SteenCase {
        id: "volk_i_pes-005",
        source: VOLK,
        source_text: "Kde ješ?",
        flavored_published: true,
        normalization: "None; the two published versions are identical here. \
                        The sentence sits inside direct speech.",
        lead_in: None,
        clitics: CliticStyle::Postverbal,
        sexpr: "(clause (pron :2 :sg :m) (vp (v jesti)) :force wh :wh-adv kde :prodrop)",
        expected: "Kde ješ?",
    },
    SteenCase {
        id: "volk_i_pes-012",
        source: VOLK,
        source_text: "Moja žena i moje děti umirajut od glada.",
        flavored_published: true,
        normalization: "`umirajut`→`umirajųt`; `glada`→`glåda`.",
        lead_in: None,
        clitics: CliticStyle::Postverbal,
        sexpr: "(clause (coord i (np (det moj) (n žena)) (np :pl (det moj) (n dětę))) \
                  (vp (v umirati) (pp (prep od) (np (n glåd)))))",
        expected: "Moja žena i moje děti umirajųt od glåda.",
    },
    SteenCase {
        id: "volk_i_pes-017",
        source: VOLK,
        source_text: "Imajut li vsi psi šije bez vlasov?",
        flavored_published: true,
        normalization: "`Imajut`→`Imajųt`; `vlasov`→`vlåsov`. The sentence \
                        sits inside direct speech.",
        lead_in: None,
        clitics: CliticStyle::Postverbal,
        sexpr: "(clause (np :pl (det vsi) (n pės)) \
                  (vp (v iměti) (object (np :pl (n šija))) \
                      (pp (prep bez) (np :pl (n vlås)))) \
                  :force li)",
        expected: "Imajųt li vsi psi šije bez vlåsov?",
    },
    SteenCase {
        id: "volk_i_pes-015",
        source: VOLK,
        source_text: "Naglo volk uviděl pesju šiju.",
        flavored_published: true,
        normalization: "`volk`→`vȯlk`; `pesju šiju`→`pėśjų šijų`; `Naglo` \
                        is realized as a discourse lead-in.",
        lead_in: Some("Naglo"),
        clitics: CliticStyle::Postverbal,
        sexpr: "(clause (np (n vȯlk)) \
                  (vp (v uviděti) (object (np (adj pėśji) (n šija)))) \
                  :tense past)",
        expected: "Naglo vȯlk uviděl pėśjų šijų.",
    },
    // -- Tri směšky (Three jokes) --------------------------------------
    SteenCase {
        id: "jokes-003",
        source: JOKES,
        source_text: "Ja myslju, že ne mogu iměti děti.",
        flavored_published: true,
        normalization: "`myslju`→`mysljų`; `mogu`→`mogų`.",
        lead_in: None,
        clitics: CliticStyle::Postverbal,
        // A `že` complement clause governing an infinitive complement —
        // the two constructions of this PR meeting in one sentence.
        sexpr: "(clause (pron :1 :sg :m) \
                  (vp (v mysliti) \
                      (sub :comp že (clause (pron :1 :sg :m) \
                                       (vp (v mogti) \
                                           (inf (v iměti) (object (np :pl (n dětę))))) \
                                       :neg :prodrop))))",
        expected: "Ja mysljų, že ne mogų iměti děti.",
    },
    SteenCase {
        id: "jokes-020",
        source: JOKES,
        source_text: "Zato oni idut k Isusu.",
        flavored_published: true,
        normalization: "`idut`→`idųt`; `Zato` is realized as a discourse lead-in.",
        lead_in: Some("Zato"),
        clitics: CliticStyle::Postverbal,
        sexpr: "(clause (pron :3 :pl :m) (vp (v idti) (pp (prep k) (name Isus :m))))",
        expected: "Zato oni idųt k Isusu.",
    },
    // -- Ovca i konji (Schleicher's tale) ------------------------------
    SteenCase {
        id: "schleicher-005",
        source: SCHLEICHER,
        source_text: "A ovca jest bez volny.",
        flavored_published: true,
        normalization: "`volny`→`vȯlny`; `A` is realized as a discourse \
                        lead-in. The sentence sits inside direct speech.",
        lead_in: Some("A"),
        clitics: CliticStyle::Postverbal,
        // A prepositional phrase as copular predicate; the PP owns its
        // own case, so predicate case never reaches it.
        sexpr: "(clause (np (n ovca)) (pred (pp (prep bez) (np (n vȯlna)))))",
        expected: "A ovca jest bez vȯlny.",
    },
    // -- Naše selo (Our village) ---------------------------------------
    SteenCase {
        id: "selo-006",
        source: SELO,
        source_text: "Put ne bude dolgy.",
        flavored_published: true,
        normalization: "`Put`→`Pųť`; `bude`→`bųde`; `dolgy`→`dȯlgy`.",
        lead_in: None,
        clitics: CliticStyle::Postverbal,
        sexpr: "(clause (np (n pųť)) (pred (adj dȯlgy)) :tense future :neg)",
        expected: "Pųť ne bųde dȯlgy.",
    },
    // -- Maly princ (standard spelling; no published flavored column) ---
    SteenCase {
        id: "maly_princ-001",
        source: PRINC,
        source_text: "Togda pojavila se lisica.",
        flavored_published: false,
        normalization: "`se`→`sę`; `Togda` is realized as a discourse \
                        lead-in. The source's verb-before-subject order is \
                        `:focus subj`.",
        lead_in: Some("Togda"),
        clitics: CliticStyle::Postverbal,
        sexpr: "(clause (np (n lisica)) (vp (v pojaviti sę)) :tense past :focus subj)",
        expected: "Togda pojavila sę lisica.",
    },
    SteenCase {
        id: "maly_princ-011",
        source: PRINC,
        source_text: "Ne jesm odomašnjeny.",
        flavored_published: false,
        normalization: "None. The sentence sits inside direct speech.",
        lead_in: None,
        clitics: CliticStyle::Postverbal,
        sexpr: "(clause (pron :1 :sg :m) (pred (part odomašniti)) :neg :prodrop)",
        expected: "Ne jesm odomašnjeny.",
    },
    SteenCase {
        id: "maly_princ-017",
        source: PRINC,
        source_text: "Oni takože razvodet kury.",
        flavored_published: false,
        normalization: "`razvodet`→`razvodęt`.",
        lead_in: None,
        clitics: CliticStyle::Postverbal,
        sexpr: "(clause (pron :3 :pl :m) \
                  (vp (v razvoditi) (adv takože) (object (np :pl (n kura)))))",
        expected: "Oni takože razvodęt kury.",
    },
    SteenCase {
        id: "maly_princ-044",
        source: PRINC,
        source_text: "Oni kupujut gotove prědmety od trgovcev.",
        flavored_published: false,
        normalization: "`kupujut`→`kupujųt`.",
        lead_in: None,
        clitics: CliticStyle::Postverbal,
        sexpr: "(clause (pron :3 :pl :m) \
                  (vp (v kupovati) (object (np :pl (adj gotovy) (n prědmet))) \
                      (pp (prep od) (np :pl (n trgovec)))))",
        expected: "Oni kupujųt gotove prědmety od trgovcev.",
    },
    SteenCase {
        id: "maly_princ-054",
        source: PRINC,
        source_text: "Tako maly princ odomašnil lisicu.",
        flavored_published: false,
        normalization: "`lisicu`→`lisicų`; `Tako` is realized as a \
                        discourse lead-in.",
        lead_in: Some("Tako"),
        clitics: CliticStyle::Postverbal,
        sexpr: "(clause (np (adj maly) (n princ)) \
                  (vp (v odomašniti) (object (np (n lisica)))) \
                  :tense past)",
        expected: "Tako maly princ odomašnil lisicų.",
    },
    SteenCase {
        id: "maly_princ-083",
        source: PRINC,
        source_text: "I on vratil se k lisici.",
        flavored_published: false,
        normalization: "`se`→`sę`; `I` is realized as a discourse lead-in.",
        lead_in: Some("I"),
        clitics: CliticStyle::Postverbal,
        sexpr: "(clause (pron :3 :sg :m) \
                  (vp (v vratiti sę) (pp (prep k) (np (n lisica)))) \
                  :tense past)",
        expected: "I on vratil sę k lisici.",
    },
];

/// The contract, end to end: literal S-expression → validation →
/// resolution → realization → the exact published sentence, then
/// canonical printing and reparsing back to the same tree.
#[test]
fn sample_corpus_realizes_exactly_and_round_trips() {
    for case in CASES {
        let tree = clause_from_str(case.sexpr)
            .unwrap_or_else(|error| panic!("{}: parse failed: {error}", case.id));

        let opts = RealizeOpts::sentence().clitics(case.clitics);
        let realized = realize_with_lead_in(&tree, case.lead_in, opts)
            .unwrap_or_else(|error| panic!("{}: realization failed: {error}", case.id));
        assert_eq!(
            realized.text, case.expected,
            "{}: generated text does not match the source\n  source: {}\n  \
             normalization: {}",
            case.id, case.source_text, case.normalization
        );

        let printed = print(&tree).unwrap_or_else(|e| panic!("{}: print failed: {e}", case.id));
        let reparsed = clause_from_str(&printed)
            .unwrap_or_else(|error| panic!("{}: reparse failed: {error}", case.id));
        assert_eq!(
            tree, reparsed,
            "{}: canonical print did not round-trip",
            case.id
        );

        validate(&tree).unwrap_or_else(|e| panic!("{}: validation failed: {e}", case.id));
    }
}

/// Every fixture is accounted for in the ledger. Without this, a fixture
/// could exist that no inventory row claims, and the per-page coverage
/// tally would understate or overstate what is covered.
#[test]
fn every_fixture_is_registered_in_the_inventory() {
    let ids: Vec<&str> = CASES.iter().map(|case| case.id).collect();
    support::assert_fixtures_registered(&ids);
}

/// Each fixture's `source_text` must be the inventory row's source text
/// verbatim. This is what stops a fixture from quietly narrowing a
/// sentence to the part the generator happens to handle.
#[test]
fn fixture_source_text_matches_the_inventory_row() {
    let rows = support::rows();
    for case in CASES {
        let row = rows
            .iter()
            .find(|row| row.id == case.id)
            .unwrap_or_else(|| panic!("{}: no inventory row", case.id));
        assert_eq!(
            row.source_text, case.source_text,
            "{}: fixture source text differs from the inventory row; a \
             fixture must cover the whole sentence, not a convenient part \
             of it",
            case.id
        );
    }
}

/// The cited URL must be the page the fixture's inventory row came
/// from. Fixtures are copied and edited, and a stale URL would attribute
/// a sentence to a text it does not appear in.
#[test]
fn the_cited_source_url_matches_the_page_the_row_came_from() {
    let rows = support::rows();
    for case in CASES {
        let row = rows.iter().find(|row| row.id == case.id).unwrap();
        let expected_url = format!("https://steen.free.fr/interslavic/{}.html", row.page);
        assert_eq!(
            case.source, expected_url,
            "{}: cites {} but the row comes from `{}`",
            case.id, case.source, row.page
        );
    }
}

/// Where the page publishes its own etymological version, `expected`
/// must be exactly that text — not something this pipeline decided. This
/// is the property that makes the corpus falsifiable.
#[test]
fn published_flavored_text_is_used_verbatim_where_it_exists() {
    let rows = support::rows();
    for case in CASES.iter().filter(|case| case.flavored_published) {
        let row = rows.iter().find(|row| row.id == case.id).unwrap();
        assert!(
            !row.flavored_text.is_empty(),
            "{}: marked as having published flavored text, but the \
             inventory row has none",
            case.id
        );
        let published = row.flavored_text.trim_end_matches(['»', '"']);
        assert_eq!(
            case.expected.trim_end_matches(['»', '"']),
            published,
            "{}: expected text disagrees with the source's own published \
             etymological version. That is a finding about the generator, \
             not something to normalize away.",
            case.id
        );
    }
}
