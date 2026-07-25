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

/// The defect this ledger was re-mined to fix.
///
/// The first mining pass split paragraphs at sentence punctuation without
/// tracking quotation depth, so 42 rows carried stray `«`/`»` — a row
/// that opened a quotation and never closed it, or closed one it never
/// opened. Those row boundaries were not sentence boundaries, and a
/// coverage claim over them meant nothing.
///
/// The invariant is NOT per-row balance. A quotation in these texts
/// routinely spans several sentences, so a sentence that closes a
/// quotation opened three sentences earlier is correct, not broken;
/// demanding per-row balance would force quotations to be glued into
/// paragraph-sized rows that no fixture could ever reproduce. The real
/// property is that a sentence with no quotative frame of its own is
/// clean: the surrounding quotation's delimiters belong to the span, not
/// to the sentence.
#[test]
fn only_framed_rows_carry_quotation_marks() {
    for row in rows() {
        if !row.framed {
            assert!(
                !row.source_text.contains(['«', '»', '„', '”']),
                "row `{}` is not framed but carries quotation marks, so it is a \
                 fragment of a quotation rather than a sentence: {}",
                row.id,
                row.source_text
            );
        }
    }
}

/// Direct speech has to be opened by something. A row marked as sitting
/// inside a quotation must be preceded, on its own page, by a row
/// carrying a quotative frame.
///
/// Counting `«` against `»` across a page would NOT be a valid check
/// here, and its failure would be a false alarm: a quoted-interior row
/// has its delimiters stripped by design, so the marks deliberately do
/// not balance in the ledger even though they balance in the source.
#[test]
fn quoted_rows_follow_a_frame_that_opened_the_speech() {
    let mut framed_seen: BTreeMap<String, bool> = BTreeMap::new();
    for row in rows() {
        let seen = framed_seen.entry(row.page.clone()).or_default();
        if row.quoted {
            assert!(
                *seen,
                "row `{}` is marked as inside direct speech, but nothing \
                 earlier on `{}` opened any: {}",
                row.id, row.page, row.source_text
            );
        }
        if row.framed {
            *seen = true;
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
