use interslavic::*;

const CASES: [Case; 6] = CASE_ORDER;
const NUMBERS: [Number; 2] = [Number::Singular, Number::Plural];

#[test]
fn noun_forms_cells_equal_single_form_calls() {
    // The acceptance criterion: the struct's cells are exactly the per-cell
    // interslavic::noun results, for a spread of declension classes.
    for lemma in [
        "žena",
        "grad",
        "kosť",
        "oko",
        "mųž",
        "selo",
        "dělo",
        "pancyŕ",
        "pancyrovoz",
        "Jangcy",
    ] {
        let p = interslavic::noun_forms(lemma);
        for number in NUMBERS {
            for case in CASES {
                assert_eq!(
                    p.get(case, number),
                    interslavic::noun(lemma, case, number),
                    "{lemma} {case:?} {number:?}"
                );
            }
        }
    }
}

#[test]
fn noun_forms_with_matches_noun_with() {
    let p = interslavic::noun_forms_with("kosť", Gender::Feminine, Animacy::Inanimate);
    assert_eq!(p.gender, Gender::Feminine);
    for number in NUMBERS {
        for case in CASES {
            assert_eq!(
                p.get(case, number),
                interslavic::noun_with("kosť", case, number, Gender::Feminine, Animacy::Inanimate),
            );
        }
    }
}

#[test]
fn adj_forms_cells_equal_single_form_calls() {
    for word in ["dobry", "svěži"] {
        let p = interslavic::adj_forms(word);
        for number in NUMBERS {
            for case in CASES {
                for (gender, animacy) in [
                    (Gender::Masculine, Animacy::Animate),
                    (Gender::Masculine, Animacy::Inanimate),
                    (Gender::Feminine, Animacy::Inanimate),
                    (Gender::Neuter, Animacy::Inanimate),
                ] {
                    assert_eq!(
                        p.get(case, number, gender, animacy),
                        interslavic::adj(word, case, number, gender, animacy),
                        "{word} {case:?} {number:?} {gender:?} {animacy:?}"
                    );
                }
            }
        }
    }
}

#[test]
fn paradigm_golden_forms() {
    let z = interslavic::noun_forms("žena");
    assert_eq!(z.gender, Gender::Feminine);
    assert_eq!(z.get(Case::Gen, Number::Singular), "ženy");
    assert_eq!(z.get(Case::Ins, Number::Singular), "ženojų");
    assert_eq!(z.get(Case::Gen, Number::Plural), "žen");

    let d = interslavic::adj_forms("dobry");
    assert_eq!(
        d.get(
            Case::Gen,
            Number::Singular,
            Gender::Masculine,
            Animacy::Animate
        ),
        "dobrogo"
    );
    // Feminine and neuter ignore animacy.
    assert_eq!(
        d.get(
            Case::Nom,
            Number::Singular,
            Gender::Feminine,
            Animacy::Animate
        ),
        d.get(
            Case::Nom,
            Number::Singular,
            Gender::Feminine,
            Animacy::Inanimate
        ),
    );
    // Masculine accusative singular differs by animacy.
    assert_eq!(
        d.get(
            Case::Acc,
            Number::Singular,
            Gender::Masculine,
            Animacy::Animate
        ),
        "dobrogo"
    );
    assert_eq!(
        d.get(
            Case::Acc,
            Number::Singular,
            Gender::Masculine,
            Animacy::Inanimate
        ),
        "dobry"
    );
}

#[test]
fn paradigm_shapes_are_well_formed() {
    let n = interslavic::noun_forms("grad");
    assert_eq!(n.singular.len(), 6);
    assert_eq!(n.plural.len(), 6);
    let a = interslavic::adj_forms("dobry");
    for col in [
        &a.masculine_animate,
        &a.masculine_inanimate,
        &a.feminine,
        &a.neuter,
    ] {
        assert_eq!(col[0].len(), 6);
        assert_eq!(col[1].len(), 6);
    }
}
