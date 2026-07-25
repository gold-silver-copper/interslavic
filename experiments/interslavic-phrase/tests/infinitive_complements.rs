//! Bare infinitive complements: modal, phasal, and obligation frames.
//!
//! An infinitive complement is a verb phrase in every respect but
//! finiteness. It compiles, prints, resolves, and realizes through the
//! same code as a finite VP, and it owns its own clitic domain.

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

/// `ne mogų spati` — modal plus intransitive infinitive, with the
/// negation on the finite verb where the source puts it.
#[test]
fn modal_with_a_bare_infinitive() {
    assert_eq!(
        realized("(clause (pron :1 :sg :m) (vp (v mogti) (inf (v spati))) :neg)"),
        "Ja ne mogų spati."
    );
}

/// `načęl dųti` — a phasal verb governing an infinitive.
#[test]
fn phasal_verb_with_an_infinitive() {
    assert_eq!(
        realized("(clause (np (adj sěverny) (n větr)) (vp (v načęti) (inf (v dųti))) :tense past)"),
        "Sěverny větr načęl dųti."
    );
}

/// The infinitive carries its own complements, not the matrix verb's.
#[test]
fn the_infinitive_owns_its_object() {
    assert_eq!(
        realized(
            "(clause (pron :3 :pl :m) \
               (vp (v htěti) (inf (v prinesti) (object (np (n jěda))))) \
               :tense past)"
        ),
        "Oni htěli prinesti jědų."
    );
}

/// The load-bearing case, and the one the source settles.
///
/// Steen writes `mogų slomiti ti hrėbet`: the dative clitic sits after
/// the infinitive, inside the infinitive's own clitic domain, not
/// climbing to the finite verb. That is what this models — consistent
/// with every other verb here owning its own domain.
///
/// The corpus is not unanimous: `načęl go napominati` puts the
/// infinitive's object clitic before the infinitive instead. Clitic
/// climbing is deliberately NOT modelled, and that sentence is in the
/// skip ledger rather than being fitted with a second rule.
#[test]
fn an_infinitives_clitic_stays_in_the_infinitives_domain() {
    let text = realized(
        "(clause (pron :1 :sg :m) \
           (vp (v mogti) \
               (inf (v slomiti) (recipient (pron :2 :sg :m :clitic)) \
                    (object (np (n hrėbet))))))",
    );
    assert_eq!(text, "Ja mogų slomiti ti hrėbet.");
    assert!(
        !text.contains("mogų ti slomiti"),
        "the clitic climbed to the finite verb: {text}"
    );
}

#[test]
fn a_reflexive_infinitive_keeps_its_own_marker() {
    assert_eq!(
        realized("(clause (pron :1 :sg :m) (vp (v mogti) (inf (v myti sę))))"),
        "Ja mogų myti sę."
    );
}

/// Infinitives nest, since an infinitive is just a verb phrase.
#[test]
fn infinitives_nest() {
    assert_eq!(
        realized("(clause (pron :1 :sg :m) (vp (v mogti) (inf (v načęti) (inf (v pisati)))))"),
        "Ja mogų načęti pisati."
    );
}

#[test]
fn infinitive_complements_round_trip() {
    roundtrips("(clause (pron :1 :sg :m) (vp (v mogti) (inf (v spati))))");
    roundtrips(
        "(clause (pron :1 :sg :m) \
           (vp (v mogti) (inf (v slomiti) (object (np (n hrėbet))) \
                              (pp (prep v) :case loc (np (n sekunda))))))",
    );
    roundtrips("(clause (pron :1 :sg :m) (vp (v mogti) (inf (v načęti) (inf (v pisati)))))");
}

/// The infinitive is not a second finite verb, so it must not acquire
/// tense, mood, or agreement of its own — it is spelled by the citation
/// lemma regardless of what the matrix clause carries.
#[test]
fn the_infinitive_is_invariant_across_matrix_features() {
    for suffix in ["", " :tense past", " :tense future", " :mood cond", " :neg"] {
        let text = realized(&format!(
            "(clause (pron :3 :sg :f) (vp (v mogti) (inf (v spati))){suffix})"
        ));
        assert!(
            text.contains("spati"),
            "the infinitive should stay a bare infinitive under `{suffix}`: {text}"
        );
    }
}

#[test]
fn a_verb_takes_at_most_one_infinitive_complement() {
    let message = clause_from_str(
        "(clause (pron :1 :sg :m) (vp (v mogti) (inf (v spati)) (inf (v pisati))))",
    )
    .unwrap_err()
    .to_string();
    assert!(message.contains("at most one `(inf …)`"), "{message}");
}

#[test]
fn an_infinitive_needs_a_verb() {
    let message = clause_from_str("(clause (pron :1 :sg :m) (vp (v mogti) (inf)))")
        .unwrap_err()
        .to_string();
    assert!(message.contains("needs a `(v …)`"), "{message}");
}

/// An infinitive's own object is checked against its own valence, not
/// the matrix verb's.
#[test]
fn the_infinitives_valence_is_its_own() {
    let tree = clause_from_str(
        "(clause (pron :1 :sg :m) (vp (v mogti) (inf (v spati) (object (np (n kniga))))))",
    );
    match tree {
        Err(error) => assert!(error.to_string().contains("object"), "{error}"),
        Ok(tree) => {
            let error = realize(&tree, RealizeOpts::sentence()).unwrap_err();
            assert!(
                format!("{error}").contains("infinitive"),
                "an intransitive infinitive with an object should fault at the \
                 infinitive's own path: {error}"
            );
        }
    }
}
