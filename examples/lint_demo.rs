//! Demo — NOT the real `lint` CLI (that's a pending STAGE-002 spec).
//!
//! This example exercises the *shipped* library end-to-end so you can watch the
//! substrate validate real skills before the CLI (arg parsing, `--json`,
//! `--strict`, exit codes) exists. It only uses skillport's public API:
//! `walk` (SPEC-002) -> `Report::from_collection(.., lint_skill)` (SPEC-003/004).
//!
//! Run it:
//!     just demo                      # lints ./lint-fixtures (good + bad)
//!     just demo path/to/skills       # lints any file / folder / tree
//!     cargo run --example lint_demo -- <path>
//!
//! Rules implemented so far (SPEC-004): frontmatter presence, name.*,
//! description.*, compatibility.length. metadata.*, allowed-tools.*, body.*,
//! and frontmatter.unknown arrive in SPEC-005.

use std::path::Path;

use skillport::{lint_skill, walk, Report};

fn main() {
    let arg = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "lint-fixtures".to_string());
    let path = Path::new(&arg);

    let collection = walk(path);
    let report = Report::from_collection(&collection, lint_skill);

    println!("skillport demo — linting {}\n", path.display());

    for section in &report.sections {
        println!("{}", section.path.display());
        if section.findings.is_empty() {
            println!("  ✓ no findings");
        }
        for f in &section.findings {
            let loc = f
                .field
                .as_deref()
                .map(|k| format!(" [{k}]"))
                .unwrap_or_default();
            // severity is padded so the columns line up
            println!(
                "  {:<7} {}{} — {}",
                f.severity.label(),
                f.rule,
                loc,
                f.message
            );
        }
        println!();
    }

    let s = &report.summary;
    println!(
        "{} skill(s): {} error(s), {} warning(s), {} info(s)",
        s.skills, s.errors, s.warnings, s.infos
    );

    // The real `lint` CLI (pending) will exit with this. The demo just prints
    // it and always exits 0, so `just demo` doesn't look like a failure.
    println!(
        "would-be CI exit code: {} (non-strict) / {} (--strict)",
        report.exit_code(false),
        report.exit_code(true),
    );
}
