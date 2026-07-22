#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DictionaryGender {
    Masculine,
    Feminine,
    Neuter,
    MasculineFeminine,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct DictionaryEntry {
    pub lemma: &'static str,
    pub addition: &'static str,
    pub gender: DictionaryGender,
    pub animate: bool,
    pub plural_only: bool,
    pub singular_only: bool,
    pub indeclinable: bool,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VerbDictionaryEntry {
    pub lemma: &'static str,
    pub addition: &'static str,
    /// Conjugation booleans, historical semantics (`transitive` gates
    /// pfpp/prpp, `imperfective` gates prap): a biaspectual row is
    /// `imperfective = true` here, exactly as before.
    pub transitive: bool,
    pub imperfective: bool,
    /// Metadata mirroring the reference parser (`@interslavic/utils`
    /// `parsePos`): biaspectual `ipf./pf.` rows are imperfective AND
    /// perfective; reflexivity is its own flag. Not consumed by the
    /// conjugation path.
    pub perfective: bool,
    pub reflexive: bool,
    pub intransitive: bool,
    /// Object-government case code from the dictionary's `(+N)`
    /// annotation (2=Gen, 3=Dat, 4=Acc, 5=Ins, 7=Loc); `None` = unmarked.
    pub governs: Option<u8>,
}

mod noun_generated {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/generated/noun_metadata_phf.rs"
    ));
}

mod verb_generated {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/generated/verb_metadata_phf.rs"
    ));
}

static EMPTY: &[DictionaryEntry] = &[];

pub(crate) fn lookup_nouns_by_lemma(word: &str) -> &'static [DictionaryEntry] {
    if let Some(entries) = noun_generated::get_nouns(word) {
        return entries;
    }

    let lower = word.to_lowercase();
    if lower != word {
        noun_generated::get_nouns(&lower).unwrap_or(EMPTY)
    } else {
        EMPTY
    }
}

pub(crate) fn lookup_verbs_by_lemma(word: &str) -> &'static [VerbDictionaryEntry] {
    if let Some(entries) = verb_generated::get_verbs(word) {
        return entries;
    }

    let lower = word.to_lowercase();
    if lower != word {
        verb_generated::get_verbs(&lower).unwrap_or(EMPTY_VERBS)
    } else {
        EMPTY_VERBS
    }
}

static EMPTY_VERBS: &[VerbDictionaryEntry] = &[];

/// Every verb lemma in the embedded dictionary — test-only, used by the
/// l-participle self-consistency sweep.
#[cfg(test)]
pub(crate) fn all_verb_lemmas() -> impl Iterator<Item = &'static str> {
    verb_generated::VERB_METADATA.keys().copied()
}
