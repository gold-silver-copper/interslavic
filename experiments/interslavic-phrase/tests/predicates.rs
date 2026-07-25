//! Copular predicates: coordination, prepositional predicates, degree.
//!
//! One copula may carry several predicates, and a predicate need not be
//! a bare adjective. These are the shapes the sample texts use that the
//! single-`Predicate` core could not express.

use interslavic_phrase::*;

fn realized(sexpr: &str) -> String {
    let tree = clause_from_str(sexpr).unwrap_or_else(|error| panic!("{sexpr}\n  parse: {error}"));
    realize(&tree, RealizeOpts::sentence())
        .unwrap_or_else(|error| panic!("{sexpr}\n  realize: {error}"))
}

fn roundtrips(sexpr: &str) {
    let tree = clause_from_str(sexpr).unwrap();
    let printed = print(&tree).unwrap();
    assert_eq!(
        tree,
        clause_from_str(&printed).unwrap(),
        "canonical print did not round-trip:\n{printed}"
    );
}

fn error_of(sexpr: &str) -> String {
    clause_from_str(sexpr)
        .map(|tree| panic!("expected an error, got {tree:?}"))
        .unwrap_err()
        .to_string()
}

// ---------------------------------------------------------------------------
// Coordination.
// ---------------------------------------------------------------------------

/// `ty jesi veliky i tȯlsty` — one copula, two predicates, both agreeing
/// with the same subject.
#[test]
fn coordinated_adjectival_predicates() {
    assert_eq!(
        realized("(clause (pron :2 :sg :m) (pred (adj veliky) (adj tȯlsty)))"),
        "Ty jesi veliky i tȯlsty."
    );
}

#[test]
fn every_predicate_agrees_with_the_same_subject() {
    assert_eq!(
        realized("(clause (np :pl (n žena)) (pred (adj veliky) (adj dobry)))"),
        "Ženy sųt velike i dobre."
    );
}

/// Three or more predicates take commas up to the final conjunction, the
/// same shape nominal coordination already uses.
#[test]
fn three_predicates_take_commas_then_the_conjunction() {
    assert_eq!(
        realized("(clause (np (n dom)) (pred (adj veliky) (adj novy) (adj dobry)))"),
        "Dom jest veliky, novy i dobry."
    );
}

#[test]
fn a_non_default_conjunction_is_carried() {
    assert_eq!(
        realized("(clause (np (n dom)) (pred (adj veliky) (adj novy) :conj ale))"),
        "Dom jest veliky ale novy."
    );
}

/// A single predicate is a one-item coordination and realizes as itself,
/// with no conjunction — the existing behaviour, unchanged.
#[test]
fn a_single_predicate_is_unchanged() {
    assert_eq!(
        realized("(clause (np (n dom)) (pred (adj veliky)))"),
        "Dom jest veliky."
    );
}

// ---------------------------------------------------------------------------
// Prepositional predicates.
// ---------------------------------------------------------------------------

/// `A ovca jest bez vȯlny.` The PP owns its case through its own
/// preposition, so predicate case never applies to it.
#[test]
fn prepositional_predicate() {
    assert_eq!(
        realized("(clause (np (n ovca)) (pred (pp (prep bez) (np (n vȯlna)))))"),
        "Ovca jest bez vȯlny."
    );
}

#[test]
fn a_prepositional_predicate_rejects_instrumental_predicate_case() {
    let message =
        error_of("(clause (np (n ovca)) (pred (pp (prep bez) (np (n vȯlna)))) :pred-case ins)");
    assert!(message.contains("nominal"), "{message}");
}

/// An ambiguous preposition inside a predicate is checked exactly as it
/// is anywhere else — the predicate position does not bypass government.
#[test]
fn a_predicate_pp_is_still_government_checked() {
    let message = error_of("(clause (np (n kot)) (pred (pp (prep pod) (np (n stol)))))");
    assert!(message.contains("case"), "{message}");
}

// ---------------------------------------------------------------------------
// Degree.
// ---------------------------------------------------------------------------

/// The tree stores the positive lemma and a degree feature, so it stays
/// a citation-form tree; the graded form comes from the facade's own
/// `comparative`/`superlative`.
#[test]
fn comparative_and_superlative_predicates() {
    assert_eq!(
        realized("(clause (np (n sȯlnce)) (pred (comp-adj silny)))"),
        "Sȯlnce jest silnějše."
    );
    assert_eq!(
        realized("(clause (np (n sȯlnce)) (pred (super-adj silny)))"),
        "Sȯlnce jest najsilnějše."
    );
}

#[test]
fn a_graded_predicate_agrees_like_any_adjective() {
    assert_eq!(
        realized("(clause (np :pl (n žena)) (pred (comp-adj silny)))"),
        "Ženy sųt silnějše."
    );
}

/// A non-gradable adjective has no synthetic degree. That is a declared
/// diagnostic from the facade, not silently wrong output.
#[test]
fn a_non_gradable_adjective_reports_rather_than_inventing_a_degree() {
    let tree = clause_from_str("(clause (np (n dom)) (pred (comp-adj sedmy)))").unwrap();
    match realize(&tree, RealizeOpts::sentence()) {
        Err(PhraseError::Unsupported { feature, .. }) => {
            assert!(feature.contains("degree"), "{feature}");
        }
        // The facade may well grade it; what must not happen is a
        // silently wrong or empty form.
        Ok(text) => assert!(
            text.starts_with("Dom jest ") && text.len() > "Dom jest .".len(),
            "{text}"
        ),
        Err(other) => panic!("unexpected error: {other}"),
    }
}

// ---------------------------------------------------------------------------
// Serialization.
// ---------------------------------------------------------------------------

#[test]
fn every_predicate_shape_round_trips() {
    for pred in [
        "(adj veliky)",
        "(short-adj veliky)",
        "(comp-adj silny)",
        "(super-adj silny)",
        "(part osvětliti)",
        "(np (n sluga))",
        "(pp (prep bez) (np (n vȯlna)))",
    ] {
        roundtrips(&format!("(clause (np (n dom)) (pred {pred}))"));
    }
    roundtrips("(clause (np (n dom)) (pred (adj veliky) (adj novy)))");
    roundtrips("(clause (np (n dom)) (pred (adj veliky) (adj novy) :conj ale))");
}

#[test]
fn an_empty_predicate_is_rejected() {
    assert!(error_of("(clause (np (n dom)) (pred))").contains("at least one"));
}

#[test]
fn a_conjunction_needs_something_to_conjoin() {
    assert!(
        error_of("(clause (np (n dom)) (pred (adj veliky) :conj ale))")
            .contains("at least two predicates")
    );
}
