//! Coordination of full clauses, each with its own subject.
//!
//! Distinct from verb-phrase coordination, which shares one subject and
//! already exists as `Coordination<VerbPhrase>`. A coordinate clause
//! extends the per-clause ownership rules rather than excepting them.

use interslavic_phrase::*;

fn realized(sexpr: &str) -> String {
    let tree = clause_from_str(sexpr).unwrap_or_else(|error| panic!("{sexpr}\n  parse: {error}"));
    realize(&tree, RealizeOpts::sentence())
        .unwrap_or_else(|error| panic!("{sexpr}\n  realize: {error}"))
}

/// `Verigy sųt želězne, a želězo jest tvŕdo.` Two subjects, two copulas,
/// one sentence.
#[test]
fn two_clauses_with_distinct_subjects() {
    assert_eq!(
        realized(
            "(clause (np :pl (n veriga)) (pred (adj želězny)) \
               (and-clause :conj a (clause (np (n želězo)) (pred (adj tvŕdy)))))"
        ),
        "Verigy sųt želězne, a želězo jest tvŕdo."
    );
}

/// Each conjunct agrees with its OWN subject. Verb-phrase coordination
/// cannot express this, which is why clause coordination exists.
#[test]
fn each_conjunct_agrees_with_its_own_subject() {
    assert_eq!(
        realized(
            "(clause (pron :1 :sg :m) (vp (v čitati)) \
               (and-clause (clause (np :pl (n žena)) (vp (v pisati)))))"
        ),
        "Ja čitajų, i ženy pišųt."
    );
}

/// The ownership rule, again: each clause places its own clitics before
/// being sealed, so neither conjunct can reach the other's.
#[test]
fn each_conjunct_keeps_its_own_clitics() {
    assert_eq!(
        realized(
            "(clause (pron :1 :sg :m) (vp (v myti sę)) \
               (and-clause (clause (pron :3 :sg :f) (vp (v myti sę)))))"
        ),
        "Ja myjų sę, i ona myje sę."
    );
}

/// Conjuncts carry their own tense and polarity.
#[test]
fn conjuncts_carry_independent_tense_and_polarity() {
    assert_eq!(
        realized(
            "(clause (np (n kot)) (vp (v spati)) :tense past \
               (and-clause :conj ale (clause (np (n pes)) (vp (v spati)) :neg)))"
        ),
        "Kot spal, ale pes ne spi."
    );
}

#[test]
fn more_than_two_clauses_chain() {
    let text = realized(
        "(clause (np (n kot)) (vp (v spati)) \
           (and-clause (clause (np (n pes)) (vp (v spati)))) \
           (and-clause :conj a (clause (np (n vȯlk)) (vp (v spati)))))",
    );
    assert_eq!(text, "Kot spi, i pes spi, a vȯlk spi.");
}

/// Terminal punctuation still belongs to the sentence, not to each
/// conjunct, and only the sentence's first word is capitalized.
#[test]
fn the_sentence_owns_its_punctuation_and_capital() {
    let text = realized(
        "(clause (np (n kot)) (vp (v spati)) \
           (and-clause (clause (np (n pes)) (vp (v spati)))))",
    );
    assert_eq!(text.matches('.').count(), 1, "{text}");
    assert!(text.starts_with("Kot"), "{text}");
    assert!(text.contains(", i pes"), "{text}");
}

#[test]
fn coordinate_clauses_round_trip() {
    for conj in ["", " :conj a", " :conj ale", " :conj ili"] {
        let sexpr = format!(
            "(clause (np (n kot)) (vp (v spati)) \
               (and-clause{conj} (clause (np (n pes)) (vp (v spati)))))"
        );
        let tree = clause_from_str(&sexpr).unwrap();
        let printed = print(&tree).unwrap();
        assert_eq!(
            tree,
            clause_from_str(&printed).unwrap(),
            "did not round-trip:\n{printed}"
        );
    }
}

/// Sentence force belongs to the sentence. A conjunct asserting its own
/// force would mean two sentences, not one.
#[test]
fn a_conjunct_cannot_carry_its_own_sentence_force() {
    let message = clause_from_str(
        "(clause (np (n kot)) (vp (v spati)) \
           (and-clause (clause (np (n pes)) (vp (v spati)) :force li)))",
    )
    .unwrap_err()
    .to_string();
    assert!(message.contains("own sentence force"), "{message}");
}

#[test]
fn an_and_clause_needs_a_clause() {
    assert!(
        clause_from_str("(clause (np (n kot)) (vp (v spati)) (and-clause :conj a))")
            .unwrap_err()
            .to_string()
            .contains("(clause")
    );
}

/// Verb-phrase coordination is unchanged and still shares one subject —
/// the two mechanisms must not have merged.
#[test]
fn verb_phrase_coordination_still_shares_one_subject() {
    assert_eq!(
        realized("(clause (np (n krålj)) (vp (v kupiti)) (vp (v čitati) (object (np (n kniga)))))"),
        "Krålj kupi i čitaje knigų."
    );
}
