//! The corpus ledger's own invariants.
//!
//! These checks are what make a coverage claim about the Steen sample
//! texts falsifiable. Without them, "every sentence is accounted for" is
//! an assertion about work that was done, rather than a property of
//! committed data.

mod support;

use std::collections::BTreeMap;
use support::{Disposition, rows};

#[test]
fn every_row_is_well_formed_and_uniquely_identified() {
    let rows = rows();
    assert!(!rows.is_empty(), "the inventory is not empty");

    let mut seen = BTreeMap::new();
    for row in &rows {
        assert!(!row.page.is_empty(), "row `{}` names its page", row.id);
        assert!(
            !row.source_text.trim().is_empty(),
            "row `{}` carries its source text",
            row.id
        );
        assert_eq!(
            row.id,
            format!("{}-{:03}", row.page, row.index),
            "row id encodes page and index"
        );
        if let Some(previous) = seen.insert(row.id.clone(), row) {
            panic!("duplicate inventory id `{}` (also {previous:?})", row.id);
        }
        // Parsing a malformed disposition panics inside `disposition()`.
        let _ = row.disposition();
    }
}

#[test]
fn no_fixture_id_is_claimed_by_two_rows() {
    let mut claimed: BTreeMap<String, String> = BTreeMap::new();
    for row in rows() {
        if let Disposition::Fixture(case) = row.disposition() {
            if let Some(previous) = claimed.insert(case.clone(), row.id.clone()) {
                panic!(
                    "fixture `{case}` is claimed by both `{previous}` and `{}`",
                    row.id
                );
            }
        }
    }
}

#[test]
fn skips_state_a_reason() {
    for row in rows() {
        if let Disposition::Skip(reason) = row.disposition() {
            assert!(
                !reason.is_empty(),
                "row `{}` is skipped without a reason; a bare `skip:` hides \
                 whether the sentence is unsupported or merely unexamined",
                row.id
            );
        }
    }
}

/// No row may be left untriaged. A pending row is a sentence that was
/// extracted and then silently dropped, which is exactly the failure this
/// ledger exists to prevent: it would let a coverage claim rest on work
/// nobody did.
#[test]
fn no_candidate_is_left_untriaged() {
    let pending: Vec<String> = rows()
        .into_iter()
        .filter(|row| row.disposition() == Disposition::Pending)
        .map(|row| row.id)
        .collect();
    assert!(
        pending.is_empty(),
        "{} inventory rows have no disposition: {pending:?}",
        pending.len()
    );
}

/// The per-page ledger, printed so a coverage claim can be read off the
/// test output rather than taken on trust.
#[test]
fn report_coverage_by_page() {
    let mut tally: BTreeMap<String, (usize, usize, usize)> = BTreeMap::new();
    for row in rows() {
        let entry = tally.entry(row.page.clone()).or_default();
        match row.disposition() {
            Disposition::Fixture(_) => entry.0 += 1,
            Disposition::Skip(_) => entry.1 += 1,
            Disposition::Pending => entry.2 += 1,
        }
    }
    let (mut total_fixture, mut total_skip, mut total_pending) = (0, 0, 0);
    println!(
        "{:<14} {:>8} {:>6} {:>8}",
        "page", "fixture", "skip", "pending"
    );
    for (page, (fixture, skip, pending)) in &tally {
        println!("{page:<14} {fixture:>8} {skip:>6} {pending:>8}");
        total_fixture += fixture;
        total_skip += skip;
        total_pending += pending;
    }
    println!(
        "{:<14} {total_fixture:>8} {total_skip:>6} {total_pending:>8}",
        "TOTAL"
    );
}
