use interslavic::*;

#[test]
fn second_conjugation_drops_extra_j_after_soft_consonants() {
    assert_eq!(
        interslavic::verb(
            "učiti",
            Person::First,
            Number::Singular,
            Gender::Feminine,
            Tense::Present,
        ),
        "učų"
    );
}

#[test]
fn dictionary_present_hints_match_sonic_reference() {
    assert_eq!(
        interslavic::verb(
            "pisati",
            Person::First,
            Number::Singular,
            Gender::Feminine,
            Tense::Present,
        ),
        "pišų"
    );
    assert_eq!(
        interslavic::verb(
            "dělati",
            Person::Third,
            Number::Singular,
            Gender::Feminine,
            Tense::Present,
        ),
        "dělaje"
    );
    assert_eq!(
        interslavic::verb(
            "byti",
            Person::Third,
            Number::Plural,
            Gender::Feminine,
            Tense::Present,
        ),
        "sųt"
    );
}

#[test]
fn ovati_and_evati_verbs_use_uj_present_stem() {
    assert_eq!(
        interslavic::verb(
            "aranževati",
            Person::First,
            Number::Singular,
            Gender::Feminine,
            Tense::Present,
        ),
        "aranžujų"
    );
    assert_eq!(
        interslavic::verb(
            "abdikovati",
            Person::Second,
            Number::Plural,
            Gender::Feminine,
            Tense::Present,
        ),
        "abdikujete"
    );
}

#[test]
fn raw_phrase_strings_are_not_conjugated_as_core_lemmas() {
    for phrase in ["bazovati na", "ostrěgati sę"] {
        assert_eq!(
            interslavic::verb(
                phrase,
                Person::First,
                Number::Singular,
                Gender::Feminine,
                Tense::Present,
            ),
            phrase
        );
        assert_eq!(
            interslavic::try_verb(
                phrase,
                Person::First,
                Number::Singular,
                Gender::Feminine,
                Tense::Present,
            ),
            None
        );
        assert_eq!(interslavic::try_verb_forms(phrase), None);
    }
}

#[test]
fn verb_lookup_trims_input_before_dictionary_lookup() {
    assert_eq!(
        interslavic::verb(
            " pisati ",
            Person::First,
            Number::Singular,
            Gender::Feminine,
            Tense::Present,
        ),
        "pišų"
    );
}

#[test]
fn explicit_present_hint_selects_duplicate_dictionary_verb_rows() {
    assert_eq!(
        interslavic::verb_with_present_hint(
            "bolěti",
            "(boli)",
            Person::First,
            Number::Singular,
            Gender::Masculine,
            Tense::Present,
        ),
        "boljų"
    );
    assert_eq!(
        interslavic::verb_with_present_hint(
            "bolěti",
            "(bolěje)",
            Person::First,
            Number::Singular,
            Gender::Masculine,
            Tense::Present,
        ),
        "bolějų"
    );
    assert_eq!(
        interslavic::verb_with_present_hint(
            "pisati",
            "(piše)",
            Person::Third,
            Number::Singular,
            Gender::Feminine,
            Tense::Perfect,
        ),
        "(je) pisala"
    );
}

#[test]
fn intransitive_dictionary_verbs_do_not_get_passive_participles() {
    let forms = interslavic::verb_forms("cvěsti");
    assert_eq!(forms.prpp, None);
    assert_eq!(forms.pfpp, None);
}

#[test]
fn full_verb_paradigm_includes_compound_tenses_and_participles() {
    assert_eq!(
        interslavic::verb(
            "pisati",
            Person::First,
            Number::Singular,
            Gender::Masculine,
            Tense::Future,
        ),
        "bųdų pisatì"
    );
    assert_eq!(
        interslavic::verb(
            "pisati",
            Person::Third,
            Number::Singular,
            Gender::Feminine,
            Tense::Perfect,
        ),
        "(je) pisala"
    );

    let forms = interslavic::verb_forms_with_metadata("pisati", "(piše)", true, true);
    assert_eq!(forms.infinitive, "pisatì");
    assert_eq!(forms.imperfect[0], "pisah");
    assert_eq!(forms.future[0], "bųdų pisatì");
    assert_eq!(forms.perfect[0], "jesm pisal(a)");
    assert_eq!(forms.pluperfect[5], "běhmo pisali");
    assert_eq!(forms.conditional[7], "by pisali");
    assert_eq!(forms.imperative, vec!["piši", "pišimo", "pišite"]);
    assert_eq!(forms.prap.as_deref(), Some("pišųćí (pišųćá, pišųćé)"));
    assert_eq!(forms.prpp.as_deref(), Some("pišemý (pišemá, pišemœ)"));
    assert_eq!(forms.pfap, "pisavši (pisavšá, pisavšé)");
    assert_eq!(forms.pfpp.as_deref(), Some("pisaný (pisaná, pisanó)"));
    assert_eq!(forms.gerund, "pisańje");
}
