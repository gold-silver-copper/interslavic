//! Normalizing raw inflection cells into clean form variants.
//!
//! The paradigm builders emit a few string conventions that make a cell awkward
//! to consume as data: decorative citation-form **stress accents** on some
//! forms (`dělatì`, `dělaný`, the `œ` ligature), an internal intervocalic-`j`
//! marker `ĵ` that survives into the present-active participle and imperative,
//! and **parenthesized alternatives** packed into one string —
//!
//! - an optional infix: `generoval(a)`, `(je) generovala`,
//! - a full alternative-forms list: `dělaný (dělaná, dělanó)`,
//! - an ending-swap list: `dělajemy (-a, -o)`,
//! - a double citation: `dělajemý (-á, -œ), dělamý (-á, -œ)`,
//! - a slash byform: `den / denj`, `oka / očese` — two genuine
//!   alternative cells, emitted by the noun builders.
//!
//! [`variants`] turns any such cell into a flat list of plain forms — stress
//! marks removed, `ĵ`→`j`, `œ`→`o`, every alternative expanded to its own entry
//! — so a consumer never has to parse the conventions itself. Join the result
//! with `" / "` for a display string.

/// Expand a raw paradigm cell into its clean form variants: stress accents and
/// the `ĵ` marker stripped, and every parenthesized alternative flattened into
/// a separate entry. A cell with no conventions returns a single-element list
/// (the deaccented form). Never empty.
///
/// ```
/// use interslavic_core::cells::variants;
/// assert_eq!(variants("generoval(a)"), ["generoval", "generovala"]);
/// assert_eq!(variants("dělaný (dělaná, dělanó)"), ["dělany", "dělana", "dělano"]);
/// assert_eq!(variants("dělajemy (-a, -o)"), ["dělajemy", "dělajema", "dělajemo"]);
/// assert_eq!(variants("dělaĵųći"), ["dělajųći"]); // ĵ→j, no alternatives
/// // Slash byforms, as the noun builders emit them.
/// assert_eq!(variants("den / denj"), ["den", "denj"]);
/// assert_eq!(variants("oka / očese"), ["oka", "očese"]);
/// assert_eq!(variants("čas"), ["čas"]); // already clean
/// ```
pub fn variants(cell: &str) -> Vec<String> {
    fn deaccent(x: &str) -> String {
        x.chars()
            .map(|c| match c {
                'á' | 'à' | 'â' | 'ã' => 'a',
                'é' | 'è' | 'ê' => 'e',
                'í' | 'ì' | 'î' => 'i',
                'ó' | 'ò' | 'ô' | 'œ' => 'o',
                'ú' | 'ù' | 'û' => 'u',
                'ý' => 'y',
                // The builders' internal intervocalic-j marker (dělaĵųći).
                'ĵ' => 'j',
                c => c,
            })
            .collect()
    }
    fn squeeze(x: String) -> String {
        x.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    let cell = deaccent(cell);
    // Slash byforms "A / B": the noun builders emit these for genuine
    // alternative cells (`den / denj`, `oka / očese`, `oči / očesa`).
    // Splitting them here is the whole point of this function — a consumer
    // that skipped it would put the literal string `den / denj` into a
    // sentence as if it were one word.
    if cell.contains(" / ") {
        return cell.split(" / ").flat_map(variants).collect();
    }
    // Double-citation "A (…), B (…)": expand each citation independently and
    // concatenate — the -aje- passive participles ship this way.
    if let Some(idx) = cell.find("), ") {
        let mut out = variants(&cell[..idx + 1]);
        out.extend(variants(cell[idx + 2..].trim_start()));
        return out;
    }
    let Some(i) = cell.find('(') else {
        return vec![cell];
    };
    let Some(jrel) = cell[i..].find(')') else {
        return vec![cell];
    };
    let j = i + jrel;
    let head = &cell[..i];
    let inside = &cell[i + 1..j];
    let rest = &cell[j + 1..];
    let base = squeeze(format!("{head}{rest}"));
    let mut out: Vec<String> = Vec::new();
    if inside.contains(',') || inside.trim_start().starts_with('-') {
        // Alternative-forms list: "dělaný (dělaná, dělanó)" are full forms;
        // "dělajemy (-a, -o)" are ending swaps grafted onto the base.
        out.push(base.clone());
        for alt in inside.split(',') {
            let alt = alt.trim();
            if let Some(suffix) = alt.strip_prefix('-') {
                let mut b = base.clone();
                b.pop();
                out.push(squeeze(format!("{b}{suffix}")));
            } else {
                out.push(squeeze(format!("{alt}{rest}")));
            }
        }
    } else {
        // Optional infix: "byh generoval(a)" → with and without.
        out.push(base);
        out.push(squeeze(format!("{head}{inside}{rest}")));
    }
    // Anything still carrying parens or a bare suffix marker is an unparsed
    // convention: drop it so callers never key on garbage.
    out.retain(|v| !v.is_empty() && !v.contains(['(', ')']) && !v.starts_with('-'));
    if out.is_empty() {
        return vec![squeeze(cell.replace(['(', ')'], " "))];
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optional_infix() {
        assert_eq!(variants("generoval(a)"), ["generoval", "generovala"]);
        assert_eq!(variants("(je) generovala"), ["generovala", "je generovala"]);
    }

    #[test]
    fn alternative_forms_list() {
        assert_eq!(
            variants("dělaný (dělaná, dělanó)"),
            ["dělany", "dělana", "dělano"]
        );
    }

    #[test]
    fn ending_swap_list() {
        assert_eq!(
            variants("dělajemy (-a, -o)"),
            ["dělajemy", "dělajema", "dělajemo"]
        );
    }

    #[test]
    fn double_citation() {
        // The -aje- passive participle shape: two citations, each with swaps.
        assert_eq!(
            variants("dělajemý (-á, -œ), dělamý (-á, -œ)"),
            [
                "dělajemy",
                "dělajema",
                "dělajemo",
                "dělamy",
                "dělama",
                "dělamo"
            ]
        );
    }

    #[test]
    fn accents_and_marker_stripped_no_alternatives() {
        assert_eq!(variants("dělatì"), ["dělati"]); // grave stress on infinitive
        assert_eq!(variants("dělaĵųći"), ["dělajųći"]); // ĵ marker
        assert_eq!(variants("čas"), ["čas"]); // already clean
    }

    #[test]
    fn never_empty_no_panic() {
        assert_eq!(variants(""), [""]); // empty input → one empty variant, no panic
        // A cell that is nothing but an unparsed convention still yields a
        // (paren-stripped) fallback rather than an empty list.
        assert!(!variants("(-)").is_empty());
    }
}
