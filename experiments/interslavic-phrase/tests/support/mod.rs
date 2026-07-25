//! Shared access to the committed Steen candidate inventory.
//!
//! The inventory is the coverage ledger: every mechanically extracted
//! candidate sentence is a row, and every row must end up dispositioned
//! either as a fixture or as an explicit skip. Fixture files assert that
//! their own case ids appear here, and `corpus_inventory.rs` asserts that
//! the ledger as a whole is complete and internally consistent. Neither
//! check is meaningful without the other: the first prevents fixtures that
//! nothing accounts for, the second prevents rows quietly dropped.

#![allow(dead_code)]

pub const INVENTORY: &str = include_str!("../../corpus/steen_samples.tsv");

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub id: String,
    pub page: String,
    pub index: usize,
    pub source_text: String,
    pub flavored_text: String,
    pub disposition: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Disposition {
    /// Realized byte-exactly by the fixture with this case id.
    Fixture(String),
    /// Deliberately not fixtured, for the stated reason.
    Skip(String),
    /// Not yet triaged — a defect once the corpus work is complete.
    Pending,
}

impl Row {
    pub fn disposition(&self) -> Disposition {
        match self.disposition.split_once(':') {
            _ if self.disposition.trim().is_empty() => Disposition::Pending,
            Some(("fixture", id)) => Disposition::Fixture(id.trim().to_string()),
            Some(("skip", reason)) => Disposition::Skip(reason.trim().to_string()),
            _ => panic!(
                "row `{}` has a malformed disposition `{}` \
                 (expected `fixture:<case id>` or `skip:<reason>`)",
                self.id, self.disposition
            ),
        }
    }
}

/// Parse the committed inventory. Panics on a malformed file: the ledger
/// is checked-in data, so a parse failure is a defect, not an input error.
pub fn rows() -> Vec<Row> {
    let mut out = Vec::new();
    let mut lines = INVENTORY
        .lines()
        .filter(|line| !line.starts_with('#') && !line.trim().is_empty());
    let header = lines.next().expect("inventory has a header row");
    assert_eq!(
        header, "id\tpage\tindex\tsource_text\tflavored_text\tdisposition",
        "inventory header changed; update tests/support/mod.rs"
    );
    for (line_number, line) in lines.enumerate() {
        let fields: Vec<&str> = line.split('\t').collect();
        assert_eq!(
            fields.len(),
            6,
            "inventory line {} has {} fields, expected 6",
            line_number + 2,
            fields.len()
        );
        out.push(Row {
            id: fields[0].to_string(),
            page: fields[1].to_string(),
            index: fields[2].parse().expect("index is a number"),
            source_text: fields[3].to_string(),
            flavored_text: fields[4].to_string(),
            disposition: fields[5].to_string(),
        });
    }
    out
}

/// Assert that every one of `ids` is dispositioned as a fixture in the
/// ledger. Fixture files call this so a case can never exist without a
/// corresponding inventory row.
pub fn assert_fixtures_registered(ids: &[&str]) {
    let rows = rows();
    for id in ids {
        let found = rows
            .iter()
            .any(|row| row.disposition() == Disposition::Fixture((*id).to_string()));
        assert!(
            found,
            "fixture `{id}` has no `fixture:{id}` row in corpus/steen_samples.tsv"
        );
    }
}
