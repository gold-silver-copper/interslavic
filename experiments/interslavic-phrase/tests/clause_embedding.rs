//! Subordinate clauses: structure, ownership, and the failure modes.
//!
//! The source fixtures live with the corpus; this file pins the grammar
//! contract itself — that an embedded clause is a full clause, owns its
//! own clitic domain, cannot carry sentence force, and takes its comma
//! from structure rather than from string assembly.

use interslavic_phrase::*;

fn realized(sexpr: &str) -> String {
    let tree = clause_from_str(sexpr).unwrap_or_else(|error| panic!("{sexpr}\n  parse: {error}"));
    realize(&tree, RealizeOpts::sentence())
        .unwrap_or_else(|error| panic!("{sexpr}\n  realize: {error}"))
}

fn roundtrips(sexpr: &str) {
    let tree = clause_from_str(sexpr).unwrap();
    let printed = print(&tree).unwrap();
    let reparsed = clause_from_str(&printed).unwrap();
    assert_eq!(
        tree, reparsed,
        "canonical print did not round-trip:\n{printed}"
    );
}

// ---------------------------------------------------------------------------
// The embedded clause is a clause.
// ---------------------------------------------------------------------------

/// A `že` complement clause is a verb argument, so it sits on the VP and
/// follows the verb's other complements.
#[test]
fn declarative_complement_clause() {
    assert_eq!(
        realized(
            "(clause (pron :3 :sg :m) \
               (vp (v uviděti) \
                   (sub :comp že (clause (np (n ljėv)) (vp (v idti)) :focus subj))) \
               :tense past)"
        ),
        "On uviděl, že ide ljėv."
    );
}

/// `da` supplies only the complementizer; the irrealis `by` comes from
/// the embedded clause's own conditional mood, which is what person-marks
/// it. 3sg `by` here, 1pl `byhmo` below.
#[test]
fn purpose_clause_takes_its_by_from_conditional_mood() {
    assert_eq!(
        realized(
            "(clause (name Bog :m) \
               (vp (v uviděti) \
                   (sub :comp da (clause (pron :3 :sg :m) \
                                         (vp (v uviděti) (object (np (n gråd)))) \
                                         :mood cond :prodrop))) \
               :tense past)"
        ),
        "Bog uviděl, da by uviděl gråd."
    );
}

#[test]
fn purpose_clause_auxiliary_agrees_in_person_and_number() {
    let first_plural = realized(
        "(clause (pron :1 :pl :m) (vp (v govoriti) \
           (sub :comp da (clause (pron :1 :pl :m) (vp (v idti)) :mood cond :prodrop))) :tense past)",
    );
    assert!(
        first_plural.contains("da byhmo idli") || first_plural.contains("da byhmo"),
        "1pl purpose clause should carry `byhmo`: {first_plural}"
    );
    let first_singular = realized(
        "(clause (pron :1 :sg :m) (vp (v govoriti) \
           (sub :comp da (clause (pron :1 :sg :m) (vp (v idti)) :mood cond :prodrop))) :tense past)",
    );
    assert!(
        first_singular.contains("da byh"),
        "1sg purpose clause should carry `byh`: {first_singular}"
    );
}

/// Both attested orders, with the comma on the inside edge either way.
#[test]
fn adverbial_clauses_front_and_follow() {
    assert_eq!(
        realized(
            "(clause (pron :3 :pl :m) (vp (v najdti) (object (np (n råvnina)))) \
               (sub :comp kogda :pos initial \
                 (clause (np (n ljudi)) (vp (v prěměstiti sę)) :tense past)) \
               :tense past)"
        ),
        "Kȯgda ljudi prěměstili sę, oni našli råvninų."
    );
    assert_eq!(
        realized(
            "(clause (np (n sŕdce)) (vp (v bolěti)) \
               (sub :comp kogda (clause (pron :1 :sg :m) (vp (v viděti)))))"
        ),
        "Sŕdce bolěje, kȯgda ja viđų."
    );
}

/// Two-word complementizers are one lexical choice.
#[test]
fn multiword_complementizers() {
    let text = realized(
        "(clause (np (n kot)) (vp (v spati)) \
           (sub :comp zato-že (clause (np (n noč)) (vp (v byti)))))",
    );
    assert!(text.contains("zato že"), "{text}");
}

#[test]
fn every_complementizer_and_position_round_trips() {
    for complementizer in ["že", "da", "kogda", "ako", "zato-že", "tomu-že"] {
        let mood = if complementizer == "da" {
            " :mood cond"
        } else {
            ""
        };
        for position in ["", " :pos initial"] {
            roundtrips(&format!(
                "(clause (np (n kot)) (vp (v spati)) \
                   (sub :comp {complementizer}{position} \
                     (clause (np (n pes)) (vp (v spati)){mood})))"
            ));
        }
    }
}

#[test]
fn a_verb_complement_clause_round_trips() {
    roundtrips(
        "(clause (pron :1 :sg :m) \
           (vp (v viděti) (sub :comp že (clause (np (n kot)) (vp (v spati))))))",
    );
}

// ---------------------------------------------------------------------------
// Ownership: clitic domains and punctuation.
// ---------------------------------------------------------------------------

/// The load-bearing invariant. Each clause places its own clitics into
/// its own constituent list before being sealed as an opaque node, so a
/// matrix verb cannot extract one from an embedded clause — the same
/// guarantee relative clauses already have.
#[test]
fn each_clause_keeps_its_own_clitics() {
    assert_eq!(
        realized(
            "(clause (pron :1 :sg :m) \
               (vp (v myti sę) \
                   (sub :comp že (clause (pron :3 :sg :f) (vp (v myti sę))))))"
        ),
        "Ja myjų sę, že ona myje sę.",
    );
}

#[test]
fn an_embedded_reflexive_does_not_migrate_to_the_matrix_verb() {
    let text = realized(
        "(clause (pron :1 :sg :m) \
           (vp (v viděti) (sub :comp že (clause (pron :3 :sg :m) (vp (v myti sę))))))",
    );
    assert_eq!(text, "Ja viđų, že on myje sę.");
    assert!(
        !text.starts_with("Ja sę") && !text.contains("viđų sę"),
        "the embedded clitic escaped its domain: {text}"
    );
}

/// Terminal punctuation and capitalization belong to the matrix
/// sentence's single stringification pass, so an embedded clause never
/// acquires a full stop or a mid-sentence capital.
#[test]
fn embedded_clauses_get_no_terminal_punctuation_or_capital() {
    let text = realized(
        "(clause (pron :1 :sg :m) \
           (vp (v slyšati) \
               (sub :comp že (clause (pron :3 :sg :f) \
                 (vp (v govoriti) \
                     (sub :comp že (clause (np (n kot)) (vp (v spati))))))))) ",
    );
    assert_eq!(text, "Ja slyšų, že ona govori, že kot spi.");
    assert_eq!(
        text.matches('.').count(),
        1,
        "one terminal stop only: {text}"
    );
}

/// A question mark belongs to the matrix clause even when a subordinate
/// clause is fronted ahead of the question particle.
#[test]
fn matrix_force_survives_a_fronted_adverbial() {
    assert_eq!(
        realized(
            "(clause (np (n pes)) (vp (v spati)) \
               (sub :comp kogda :pos initial (clause (np (n noč)) (vp (v byti)))) \
               :force či)"
        ),
        "Kȯgda noč jest, či pes spi?"
    );
}

// ---------------------------------------------------------------------------
// Failure modes.
// ---------------------------------------------------------------------------

fn validation_error(sexpr: &str) -> String {
    clause_from_str(sexpr)
        .map(|tree| panic!("expected a validation error, got {tree:?}"))
        .unwrap_err()
        .to_string()
}

#[test]
fn a_subordinate_clause_cannot_carry_sentence_force() {
    for force in [":force li", ":force či", ":force intonation", ":force imp"] {
        let message = validation_error(&format!(
            "(clause (np (n kot)) (vp (v spati)) \
               (sub :comp že (clause (np (n pes)) (vp (v spati)) {force})))"
        ));
        assert!(
            message.contains("independent sentence force"),
            "`{force}` should be rejected inside a subordinate: {message}"
        );
    }
}

#[test]
fn da_needs_the_conditional_that_supplies_its_by() {
    let message = validation_error(
        "(clause (np (n kot)) (vp (v spati)) \
           (sub :comp da (clause (np (n pes)) (vp (v spati)))))",
    );
    assert!(message.contains("purpose clause"), "{message}");
}

#[test]
fn a_verb_complement_clause_cannot_be_fronted() {
    let message = validation_error(
        "(clause (pron :1 :sg :m) \
           (vp (v viděti) \
               (sub :comp že :pos initial (clause (np (n kot)) (vp (v spati))))))",
    );
    assert!(
        message.contains("complement clause follows it"),
        "{message}"
    );
}

#[test]
fn a_verb_takes_at_most_one_complement_clause() {
    let message = clause_from_str(
        "(clause (pron :1 :sg :m) \
           (vp (v viděti) \
               (sub :comp že (clause (np (n kot)) (vp (v spati)))) \
               (sub :comp že (clause (np (n pes)) (vp (v spati))))))",
    )
    .unwrap_err()
    .to_string();
    assert!(
        message.contains("at most one complement clause"),
        "{message}"
    );
}

#[test]
fn unknown_complementizers_are_rejected_with_the_allowed_set() {
    let message = clause_from_str(
        "(clause (np (n kot)) (vp (v spati)) \
           (sub :comp because (clause (np (n pes)) (vp (v spati)))))",
    )
    .unwrap_err()
    .to_string();
    assert!(message.contains("unknown complementizer"), "{message}");
    assert!(
        message.contains("kogda"),
        "the error lists the allowed set: {message}"
    );
}

#[test]
fn a_sub_form_needs_both_a_complementizer_and_a_clause() {
    assert!(
        clause_from_str(
            "(clause (np (n kot)) (vp (v spati)) (sub (clause (np (n pes)) (vp (v spati)))))"
        )
        .unwrap_err()
        .to_string()
        .contains(":comp")
    );
    assert!(
        clause_from_str("(clause (np (n kot)) (vp (v spati)) (sub :comp že))")
            .unwrap_err()
            .to_string()
            .contains("(clause")
    );
}

fn nested_clauses(levels: usize) -> String {
    let mut sexpr = String::from("(clause (np (n kot)) (vp (v spati)))");
    for _ in 0..levels {
        sexpr = format!("(clause (np (n kot)) (vp (v spati) (sub :comp že {sexpr})))");
    }
    sexpr
}

/// Clause embedding is bounded well below `MAX_STRUCTURE_DEPTH`, and the
/// bound is enforced in the iterative preflight — before the recursive
/// compiler runs.
///
/// This is not a theoretical limit. The generic structure bound permits
/// clause nesting deep enough to exhaust the stack inside
/// `compile_clause_raw` before any diagnostic is produced, because a
/// clause frame is large and the generic bound counts list levels, which
/// a clause is cheap in. Exceeding the bound must be an error value, not
/// a crash.
#[test]
fn deep_clause_embedding_is_a_diagnostic_not_a_stack_overflow() {
    let error = clause_from_str(&nested_clauses(MAX_CLAUSE_DEPTH + 4))
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("clause embedding depth"),
        "expected the clause-embedding bound: {error}"
    );
}

/// Depth far past the generic bound must also be caught by the clause
/// bound first, since that is the input shape that used to crash.
#[test]
fn pathologically_deep_embedding_is_rejected_before_recursion() {
    let error = clause_from_str(&nested_clauses(MAX_STRUCTURE_DEPTH * 2))
        .unwrap_err()
        .to_string();
    assert!(error.contains("depth"), "{error}");
}

/// The bound has to leave real sentences alone. The deepest sentence in
/// the Steen sample corpus embeds three clauses.
#[test]
fn embedding_well_within_the_bound_still_works() {
    let tree = clause_from_str(&nested_clauses(MAX_CLAUSE_DEPTH / 4)).unwrap();
    assert!(realize(&tree, RealizeOpts::sentence()).is_ok());
}
