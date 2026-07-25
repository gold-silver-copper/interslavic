//! Workspace task runner (`cargo xtask <command>`).
//!
//! The runtime crates never parse dictionary TSV data. `refresh-data` delegates
//! to `interslavic-extractor` to regenerate committed Rust metadata, while
//! `check-registry` verifies that the generated files are current.

use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::process::{self, Command};

fn workspace_root() -> Result<PathBuf, Box<dyn Error>> {
    Ok(PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("refresh-data") => run_extractor(false),
        Some("check-registry") => run_extractor(true),
        Some("check-all") => check_all(),
        Some("examples") => run_examples(),
        Some("speed") => run_speedmark(),
        Some("accuracy") => run_accuracy(),
        Some("dump-paradigms") => dump_paradigms(&mut args),
        Some("diff-fingerprint") => diff_fingerprint(&mut args),
        Some("phrase-check") => phrase_check(),
        Some("-h") | Some("--help") | Some("help") | None => {
            print_usage();
            Ok(())
        }
        Some(command) => Err(format!("unknown xtask command: {command}").into()),
    }
}

fn check_all() -> Result<(), Box<dyn Error>> {
    run_cargo(&["fmt", "--all", "--", "--check"])?;
    run_cargo(&["test", "--workspace"])?;
    run_extractor(true)
}

fn run_examples() -> Result<(), Box<dyn Error>> {
    for example in ["basic", "verb_paradigm", "sentence"] {
        run_cargo(&["run", "-p", "interslavic", "--example", example])?;
    }
    Ok(())
}

fn run_speedmark() -> Result<(), Box<dyn Error>> {
    run_cargo(&[
        "run",
        "-p",
        "interslavic",
        "--example",
        "speedmark",
        "--release",
    ])
}

fn run_accuracy() -> Result<(), Box<dyn Error>> {
    let root = workspace_root()?;
    let status = Command::new("node")
        .current_dir(&root)
        .arg("tools/compare-latest-sonic.js")
        .status()?;
    exit_if_failed(status.code(), status.success());
    Ok(())
}

/// Write the whole-dictionary paradigm dump (`lemma\tcell\tform`) to
/// `target/paradigm-fingerprint/<name>.tsv` (default `dump.tsv`; pass a
/// name to keep an old dump around for diffing).
fn dump_paradigms(args: &mut impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
    let root = workspace_root()?;
    let name = args.next().unwrap_or_else(|| "dump".to_string());
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let output = Command::new(cargo)
        .current_dir(&root)
        .args([
            "run",
            "-q",
            "-p",
            "interslavic",
            "--example",
            "dump_paradigms",
        ])
        .output()?;
    exit_if_failed(output.status.code(), output.status.success());
    let dir = root.join("target/paradigm-fingerprint");
    std::fs::create_dir_all(&dir)?;
    let path = dir.join(format!("{name}.tsv"));
    std::fs::write(&path, &output.stdout)?;
    let hash = fnv1a(&output.stdout);
    eprintln!(
        "wrote {} ({} cells, fingerprint {hash:#018x})",
        path.display(),
        output.stdout.iter().filter(|&&b| b == b'\n').count()
    );
    Ok(())
}

fn fnv1a(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for &byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

/// Print the cell-level delta between two paradigm dumps, so the
/// changelog enumeration for a fingerprint change is mechanical.
fn diff_fingerprint(args: &mut impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
    let (Some(old_path), Some(new_path)) = (args.next(), args.next()) else {
        return Err("usage: cargo xtask diff-fingerprint <old.tsv> <new.tsv>".into());
    };
    let load = |path: &str| -> Result<std::collections::BTreeMap<String, String>, Box<dyn Error>> {
        let text = std::fs::read_to_string(path)?;
        Ok(text
            .lines()
            .filter_map(|line| {
                let mut parts = line.splitn(3, '\t');
                let lemma = parts.next()?;
                let cell = parts.next()?;
                let form = parts.next()?;
                Some((format!("{lemma}\t{cell}"), form.to_string()))
            })
            .collect())
    };
    let old = load(&old_path)?;
    let new = load(&new_path)?;
    let mut changed = 0usize;
    for (key, old_form) in &old {
        match new.get(key) {
            Some(new_form) if new_form != old_form => {
                println!("changed\t{key}\t{old_form}\t->\t{new_form}");
                changed += 1;
            }
            None => {
                println!("removed\t{key}\t{old_form}");
                changed += 1;
            }
            _ => {}
        }
    }
    for (key, new_form) in &new {
        if !old.contains_key(key) {
            println!("added\t{key}\t{new_form}");
            changed += 1;
        }
    }
    eprintln!(
        "{changed} cell(s) differ ({} -> {} cells)",
        old.len(),
        new.len()
    );
    Ok(())
}

/// Render the interslavic-phrase golden sentences and run them through
/// slovowiki's independent agreement checker (`check-text --json`). A
/// manual/local gate like `accuracy`: it needs a slovowiki checkout,
/// found via the SLOVOWIKI_DIR env var (default: ../slovowiki next to
/// this repository). Fails if any token is unknown or any agreement
/// error is reported. Interpret hits per the folded-key rule: a surface
/// match can belong to a homograph — the JSON's `lemmas` field is the
/// truth, and this gate only checks statuses, not lemma attribution.
/// Tokens slovowiki's lexicon does not carry, which are nonetheless the
/// forms Steen's own sample texts print and this repository's dictionary
/// produces. Each is a gap in the external checker, not in the generator,
/// so the sentence stays under agreement checking and only the
/// unknown-word verdict is waived — for these exact tokens and no others.
///
/// Adding an entry here is a claim that the form is attested in the
/// source. Cite where.
const SLOVOWIKI_LEXICON_GAPS: &[(&str, &str)] = &[
    // `Imajųt li vsi psi šije bez vlåsov?` — volk_i_pes.html. Nominative
    // plural of `pės`; slovowiki suggests `pri`/`pa`/`pan`.
    ("psi", "volk_i_pes.html, nom.pl of pės"),
    // `Naglo vȯlk uviděl pėśjų šijų.` — volk_i_pes.html. Feminine
    // accusative singular of the possessive adjective `pėśji`.
    ("pėśjų", "volk_i_pes.html, f.acc.sg of pėśji"),
];

fn phrase_check() -> Result<(), Box<dyn Error>> {
    let root = workspace_root()?;
    let slovowiki = env::var("SLOVOWIKI_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| root.join("../slovowiki"));
    if !slovowiki.join("Cargo.toml").exists() {
        return Err(format!(
            "no slovowiki checkout at {} (set SLOVOWIKI_DIR)",
            slovowiki.display()
        )
        .into());
    }
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let goldens = Command::new(&cargo)
        .current_dir(&root)
        .args([
            "run",
            "-q",
            "-p",
            "interslavic-phrase",
            "--example",
            "goldens",
        ])
        .output()?;
    exit_if_failed(goldens.status.code(), goldens.status.success());
    let dir = root.join("target/phrase-check");
    std::fs::create_dir_all(&dir)?;
    let text_path = dir.join("goldens.txt");
    std::fs::write(&text_path, &goldens.stdout)?;
    // Plain `cargo` (not the +toolchain-resolved $CARGO of this workspace):
    // slovowiki pins its own toolchain via rust-toolchain.toml.
    let check = Command::new("cargo")
        .current_dir(&slovowiki)
        .args(["run", "--release", "-q", "--", "check-text"])
        .arg(&text_path)
        .arg("--json")
        .output()?;
    exit_if_failed(check.status.code(), check.status.success());
    let json = String::from_utf8_lossy(&check.stdout);
    // Dependency-free verdict: count status fields in the token array.
    let count = |needle: &str| json.matches(needle).count();
    let unknown = count("\"unknown\"");
    let tokens = count("\"token\"");
    let agreement_errors = count("\"agreement_error\"");
    // Waive the unknown-word verdict for the named lexicon gaps, and only
    // for those. A waived token must still appear as unknown in the
    // report; if slovowiki later learns the form, the entry becomes stale
    // and is reported so it can be removed rather than lingering.
    let mut waived = 0;
    let mut stale = Vec::new();
    for (token, source) in SLOVOWIKI_LEXICON_GAPS {
        let occurrences = json
            .matches(&format!("\"token\":\"{token}\",\"status\":\"unknown\""))
            .count();
        if occurrences == 0 {
            stale.push(format!("`{token}` ({source})"));
        }
        waived += occurrences;
    }
    let unwaived = unknown.saturating_sub(waived);
    println!(
        "{} tokens checked, {} unknown ({} waived as slovowiki lexicon gaps), \
         {} agreement errors",
        tokens, unknown, waived, agreement_errors
    );
    if !stale.is_empty() {
        println!(
            "note: no longer unknown, drop from SLOVOWIKI_LEXICON_GAPS: {}",
            stale.join(", ")
        );
    }
    if unwaived > 0 || agreement_errors > 0 {
        std::fs::write(dir.join("check.json"), check.stdout)?;
        return Err(format!(
            "phrase-check failed; full report at {}",
            dir.join("check.json").display()
        )
        .into());
    }
    Ok(())
}

fn run_extractor(check_only: bool) -> Result<(), Box<dyn Error>> {
    let root = workspace_root()?;
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let mut command = Command::new(cargo);
    command
        .current_dir(&root)
        .args(["run", "-p", "interslavic-extractor", "--"])
        .arg("--input")
        .arg(root.join("crates/interslavic/data/dictionary_metadata.tsv"))
        .arg("--output-dir")
        .arg(root.join("crates/interslavic/generated"));
    if check_only {
        command.arg("--check-only");
    }

    let status = command.status()?;
    exit_if_failed(status.code(), status.success());
    Ok(())
}

fn run_cargo(args: &[&str]) -> Result<(), Box<dyn Error>> {
    let root = workspace_root()?;
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(cargo).current_dir(root).args(args).status()?;
    exit_if_failed(status.code(), status.success());
    Ok(())
}

fn exit_if_failed(code: Option<i32>, success: bool) {
    if !success {
        process::exit(code.unwrap_or(1));
    }
}

fn print_usage() {
    eprintln!("Usage: cargo xtask <command>");
    eprintln!();
    eprintln!("Commands:");
    eprintln!("  refresh-data     Regenerate static dictionary metadata");
    eprintln!("  check-registry   Verify generated dictionary metadata is current");
    eprintln!("  check-all        Run fmt check, workspace tests, and check-registry");
    eprintln!("  examples         Run lightweight public API examples");
    eprintln!("  speed            Run the release speedmark example");
    eprintln!("  accuracy         Run sonic16x parity comparison");
    eprintln!("  dump-paradigms   Write the whole-dictionary paradigm dump [name]");
    eprintln!("  diff-fingerprint Print the cell delta between two dumps <old> <new>");
    eprintln!("  phrase-check     Run interslavic-phrase goldens through slovowiki (local)");
}
