//! Rendering a [`Report`] as human-readable text or the stable `--json`
//! schema (`docs/api-contract.md`, DEC-005).
//!
//! Two entry points: [`human`] and [`json`]. Neither reorders anything — the
//! report layer (`src/report.rs`) already guarantees deterministic,
//! path-sorted sections with deterministically ordered findings; this module
//! only renders what it's given (`deterministic-stable-output`).
//!
//! The `--json` schema is built from emitter-local `#[derive(Serialize)]` DTO
//! structs (per the spec's "JSON serialization approach") so `report.rs`
//! stays serde-free and the entire wire format lives in one place.

use crate::report::{Report, Severity};
use serde::Serialize;
use std::collections::BTreeSet;

/// The `--json` schema's `schema` version marker (DEC-005: bump only on a
/// breaking change to the shape).
const SCHEMA_VERSION: u32 = 1;

/// The top-level `--json` envelope (`docs/api-contract.md`).
#[derive(Serialize)]
struct Envelope<'a> {
    tool: &'static str,
    version: &'static str,
    schema: u32,
    target: Option<&'a str>,
    summary: SummaryDto,
    sections: Vec<SectionDto<'a>>,
}

/// DTO mirror of [`crate::report::Summary`] (kept separate so `report.rs`
/// doesn't need `Serialize`).
#[derive(Serialize)]
struct SummaryDto {
    skills: usize,
    errors: usize,
    warnings: usize,
    infos: usize,
}

/// DTO mirror of [`crate::report::Section`]; `path` renders as a display
/// string (spec: "Paths render as display strings").
#[derive(Serialize)]
struct SectionDto<'a> {
    path: String,
    findings: Vec<FindingDto<'a>>,
}

/// DTO mirror of [`crate::report::Finding`]; `severity` serializes as the
/// lowercase label (`"error"`/`"warning"`/`"info"`), never the enum's Rust
/// variant name.
#[derive(Serialize)]
struct FindingDto<'a> {
    rule: &'a str,
    severity: &'static str,
    message: &'a str,
    field: Option<&'a str>,
    line: Option<usize>,
}

/// Serialize `report` as the stable `--json` schema (DEC-005). `target` is
/// `None` for now (STAGE-003 sets it via `--target`); `version` is the crate
/// version, `schema` is `1`. Never fails: `serde_json::to_string` on this DTO
/// tree cannot produce a `Serialize` error, so a failure here would be a
/// logic bug, not a runtime condition callers need to handle.
pub fn json(report: &Report, target: Option<&str>) -> String {
    let envelope = Envelope {
        tool: "skillport",
        version: env!("CARGO_PKG_VERSION"),
        schema: SCHEMA_VERSION,
        target,
        summary: SummaryDto {
            skills: report.summary.skills,
            errors: report.summary.errors,
            warnings: report.summary.warnings,
            infos: report.summary.infos,
        },
        sections: report
            .sections
            .iter()
            .map(|section| SectionDto {
                path: section.path.display().to_string(),
                findings: section
                    .findings
                    .iter()
                    .map(|f| FindingDto {
                        rule: f.rule,
                        severity: severity_label(f.severity),
                        message: &f.message,
                        field: f.field.as_deref(),
                        line: f.line,
                    })
                    .collect(),
            })
            .collect(),
    };

    serde_json::to_string(&envelope).expect("Report -> JSON envelope is always serializable")
}

/// Map [`Severity`] to the lowercase wire label. Delegates to
/// [`Severity::label`] so there is exactly one place that owns the mapping.
fn severity_label(severity: Severity) -> &'static str {
    severity.label()
}

/// The repo URL used as `tool.driver.informationUri` in `--sarif` output.
const SARIF_INFORMATION_URI: &str = "https://github.com/jysf/skillport";

/// The top-level SARIF 2.1.0 log (`docs/api-contract.md`, spec's "SARIF 2.1.0
/// shape (implement exactly)").
#[derive(Serialize)]
struct SarifLog<'a> {
    #[serde(rename = "$schema")]
    schema: &'static str,
    version: &'static str,
    runs: Vec<Run<'a>>,
}

#[derive(Serialize)]
struct Run<'a> {
    tool: Tool<'a>,
    results: Vec<SarifResult<'a>>,
}

#[derive(Serialize)]
struct Tool<'a> {
    driver: Driver<'a>,
}

#[derive(Serialize)]
struct Driver<'a> {
    name: &'static str,
    #[serde(rename = "informationUri")]
    information_uri: &'static str,
    version: &'static str,
    rules: Vec<ReportingDescriptor<'a>>,
}

#[derive(Serialize)]
struct ReportingDescriptor<'a> {
    id: &'a str,
}

#[derive(Serialize)]
struct SarifResult<'a> {
    #[serde(rename = "ruleId")]
    rule_id: &'a str,
    level: &'static str,
    message: Message<'a>,
    locations: Vec<Location>,
}

#[derive(Serialize)]
struct Message<'a> {
    text: &'a str,
}

#[derive(Serialize)]
struct Location {
    #[serde(rename = "physicalLocation")]
    physical_location: PhysicalLocation,
}

#[derive(Serialize)]
struct PhysicalLocation {
    #[serde(rename = "artifactLocation")]
    artifact_location: ArtifactLocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<Region>,
}

#[derive(Serialize)]
struct ArtifactLocation {
    uri: String,
}

#[derive(Serialize)]
struct Region {
    #[serde(rename = "startLine")]
    start_line: usize,
}

/// Map [`Severity`] to the SARIF 2.1.0 `level` (DEC-003). Deliberately
/// separate from [`Severity::label`], which returns `"info"` for
/// [`Severity::Info`] — SARIF has no `"info"` level, only `"note"`.
fn sarif_level(severity: Severity) -> &'static str {
    match severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Info => "note",
    }
}

/// Serialize `report` as a SARIF 2.1.0 log (`docs/api-contract.md`, DEC-003,
/// DEC-005). Pure render: no reordering — `results` follow the report's
/// existing (path-sorted, severity-ordered) order, and `driver.rules` is the
/// distinct rule ids sorted via a `BTreeSet` for determinism. Never fails,
/// for the same reason [`json`] never fails.
pub fn sarif(report: &Report) -> String {
    let mut rule_ids: BTreeSet<&str> = BTreeSet::new();
    let mut results = Vec::new();

    for section in &report.sections {
        for finding in &section.findings {
            rule_ids.insert(finding.rule);
            results.push(SarifResult {
                rule_id: finding.rule,
                level: sarif_level(finding.severity),
                message: Message {
                    text: &finding.message,
                },
                locations: vec![Location {
                    physical_location: PhysicalLocation {
                        artifact_location: ArtifactLocation {
                            uri: finding.path.display().to_string(),
                        },
                        region: finding.line.map(|start_line| Region { start_line }),
                    },
                }],
            });
        }
    }

    let log = SarifLog {
        schema: "https://json.schemastore.org/sarif-2.1.0.json",
        version: "2.1.0",
        runs: vec![Run {
            tool: Tool {
                driver: Driver {
                    name: "skillport",
                    information_uri: SARIF_INFORMATION_URI,
                    version: env!("CARGO_PKG_VERSION"),
                    rules: rule_ids
                        .into_iter()
                        .map(|id| ReportingDescriptor { id })
                        .collect(),
                },
            },
            results,
        }],
    };

    serde_json::to_string(&log).expect("Report -> SARIF log is always serializable")
}

/// Render `report` as human-readable, path-grouped text with a trailing
/// severity summary. Close to `examples/lint_demo.rs`'s format, refined and
/// pinned by the unit tests below.
pub fn human(report: &Report) -> String {
    let mut out = String::new();

    if report.sections.is_empty() {
        out.push_str("no skills found\n");
    }

    for section in &report.sections {
        out.push_str(&section.path.display().to_string());
        out.push('\n');

        if section.findings.is_empty() {
            out.push_str("  no findings\n");
        }

        for f in &section.findings {
            let loc = f
                .field
                .as_deref()
                .map(|k| format!(" [{k}]"))
                .unwrap_or_default();
            out.push_str(&format!(
                "  {:<7} {}{} — {}\n",
                severity_label(f.severity),
                f.rule,
                loc,
                f.message
            ));
        }
        out.push('\n');
    }

    let s = &report.summary;
    out.push_str(&format!(
        "{} skill(s): {} error(s), {} warning(s), {} info(s)\n",
        s.skills, s.errors, s.warnings, s.infos
    ));

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::report::{Finding, Section, Summary};
    use serde_json::Value;
    use std::path::PathBuf;

    /// A hand-built two-section report: one clean, one with an error finding
    /// — enough to exercise both emitters without going through `walk`/`lint_skill`.
    fn sample_report() -> Report {
        Report {
            sections: vec![
                Section {
                    path: PathBuf::from("lint-fixtures/bad/My-Skill/SKILL.md"),
                    findings: vec![Finding {
                        rule: "name.charset",
                        severity: Severity::Error,
                        message: "'name' may only contain lowercase letters, digits, and hyphens (invalid: MS)".to_string(),
                        path: PathBuf::from("lint-fixtures/bad/My-Skill/SKILL.md"),
                        field: Some("name".to_string()),
                        line: None,
                    }],
                },
                Section {
                    path: PathBuf::from("lint-fixtures/good/data-analysis/SKILL.md"),
                    findings: Vec::new(),
                },
            ],
            summary: Summary {
                skills: 2,
                errors: 1,
                warnings: 0,
                infos: 0,
            },
        }
    }

    #[test]
    fn json_has_the_documented_envelope() {
        let report = sample_report();

        let rendered = json(&report, None);
        let value: Value = serde_json::from_str(&rendered).expect("valid JSON");

        assert_eq!(value["tool"], "skillport");
        assert_eq!(value["schema"], 1);
        assert_eq!(value["target"], Value::Null);
        assert_eq!(value["version"], env!("CARGO_PKG_VERSION"));
        assert_eq!(value["summary"]["errors"], 1);
        assert_eq!(value["summary"]["skills"], 2);

        let first_finding = &value["sections"][0]["findings"][0];
        assert_eq!(first_finding["severity"], "error");
        assert_eq!(first_finding["rule"], "name.charset");
    }

    #[test]
    fn json_severities_are_lowercase_strings() {
        let report = Report {
            sections: vec![Section {
                path: PathBuf::from("a/SKILL.md"),
                findings: vec![
                    Finding {
                        rule: "test.error",
                        severity: Severity::Error,
                        message: "m".to_string(),
                        path: PathBuf::from("a/SKILL.md"),
                        field: None,
                        line: None,
                    },
                    Finding {
                        rule: "test.warn",
                        severity: Severity::Warning,
                        message: "m".to_string(),
                        path: PathBuf::from("a/SKILL.md"),
                        field: None,
                        line: None,
                    },
                    Finding {
                        rule: "test.info",
                        severity: Severity::Info,
                        message: "m".to_string(),
                        path: PathBuf::from("a/SKILL.md"),
                        field: None,
                        line: None,
                    },
                ],
            }],
            summary: Summary {
                skills: 1,
                errors: 1,
                warnings: 1,
                infos: 1,
            },
        };

        let rendered = json(&report, None);
        let value: Value = serde_json::from_str(&rendered).expect("valid JSON");

        let severities: Vec<&str> = value["sections"][0]["findings"]
            .as_array()
            .unwrap()
            .iter()
            .map(|f| f["severity"].as_str().unwrap())
            .collect();
        assert_eq!(severities, vec!["error", "warning", "info"]);
    }

    #[test]
    fn json_sections_preserve_report_order() {
        let report = sample_report();

        let rendered = json(&report, None);
        let value: Value = serde_json::from_str(&rendered).expect("valid JSON");

        let paths: Vec<&str> = value["sections"]
            .as_array()
            .unwrap()
            .iter()
            .map(|s| s["path"].as_str().unwrap())
            .collect();
        assert_eq!(
            paths,
            vec![
                "lint-fixtures/bad/My-Skill/SKILL.md",
                "lint-fixtures/good/data-analysis/SKILL.md",
            ]
        );
    }

    #[test]
    fn human_output_contains_rule_id_severity_and_message_for_a_finding() {
        let report = sample_report();

        let rendered = human(&report);

        assert!(rendered.contains("name.charset"));
        assert!(rendered.contains("error"));
        assert!(rendered.contains(
            "'name' may only contain lowercase letters, digits, and hyphens (invalid: MS)"
        ));
    }

    #[test]
    fn human_marks_a_clean_section_as_having_no_findings() {
        let report = sample_report();

        let rendered = human(&report);

        let lines: Vec<&str> = rendered.lines().collect();
        let idx = lines
            .iter()
            .position(|l| *l == "lint-fixtures/good/data-analysis/SKILL.md")
            .expect("clean section path present");
        assert_eq!(lines[idx + 1].trim(), "no findings");
    }

    #[test]
    fn human_notes_no_skills_found_for_an_empty_report() {
        let report = Report {
            sections: Vec::new(),
            summary: Summary::default(),
        };

        let rendered = human(&report);

        assert!(rendered.contains("no skills found"));
    }

    #[test]
    fn json_is_still_valid_for_an_empty_report() {
        let report = Report {
            sections: Vec::new(),
            summary: Summary::default(),
        };

        let rendered = json(&report, None);
        let value: Value = serde_json::from_str(&rendered).expect("valid JSON");

        assert_eq!(value["sections"].as_array().unwrap().len(), 0);
        assert_eq!(value["summary"]["skills"], 0);
    }

    /// A report with one Error (with `line`), one Warning, one Info finding —
    /// enough to exercise the full level mapping and the `region` presence
    /// rule in one shot.
    fn sarif_sample_report() -> Report {
        Report {
            sections: vec![
                Section {
                    path: PathBuf::from("lint-fixtures/bad/My-Skill/SKILL.md"),
                    findings: vec![
                        Finding {
                            rule: "name.charset",
                            severity: Severity::Error,
                            message: "'name' may only contain lowercase letters, digits, and hyphens (invalid: MS!)".to_string(),
                            path: PathBuf::from("lint-fixtures/bad/My-Skill/SKILL.md"),
                            field: Some("name".to_string()),
                            line: Some(3),
                        },
                        Finding {
                            rule: "description.required",
                            severity: Severity::Warning,
                            message: "warn message".to_string(),
                            path: PathBuf::from("lint-fixtures/bad/My-Skill/SKILL.md"),
                            field: None,
                            line: None,
                        },
                    ],
                },
                Section {
                    path: PathBuf::from("lint-fixtures/bad/other/SKILL.md"),
                    findings: vec![Finding {
                        rule: "name.charset",
                        severity: Severity::Info,
                        message: "info message".to_string(),
                        path: PathBuf::from("lint-fixtures/bad/other/SKILL.md"),
                        field: None,
                        line: None,
                    }],
                },
            ],
            summary: Summary {
                skills: 2,
                errors: 1,
                warnings: 1,
                infos: 1,
            },
        }
    }

    #[test]
    fn sarif_envelope_has_version_schema_and_driver_name_version() {
        let report = sarif_sample_report();

        let rendered = sarif(&report);
        let value: Value = serde_json::from_str(&rendered).expect("valid SARIF JSON");

        assert_eq!(value["version"], "2.1.0");
        assert!(value["$schema"].is_string());
        assert_eq!(value["runs"][0]["tool"]["driver"]["name"], "skillport");
        assert_eq!(
            value["runs"][0]["tool"]["driver"]["version"],
            env!("CARGO_PKG_VERSION")
        );
    }

    #[test]
    fn sarif_finding_becomes_result_with_ruleid_level_message_uri() {
        let report = sarif_sample_report();

        let rendered = sarif(&report);
        let value: Value = serde_json::from_str(&rendered).expect("valid SARIF JSON");

        let results = value["runs"][0]["results"].as_array().unwrap();
        assert_eq!(results.len(), 3);

        let levels: Vec<&str> = results
            .iter()
            .map(|r| r["level"].as_str().unwrap())
            .collect();
        assert_eq!(levels, vec!["error", "warning", "note"]);

        let first = &results[0];
        assert_eq!(first["ruleId"], "name.charset");
        assert_eq!(
            first["message"]["text"],
            "'name' may only contain lowercase letters, digits, and hyphens (invalid: MS!)"
        );
        assert_eq!(
            first["locations"][0]["physicalLocation"]["artifactLocation"]["uri"],
            "lint-fixtures/bad/My-Skill/SKILL.md"
        );
    }

    #[test]
    fn sarif_line_becomes_region_startline_when_present_absent_otherwise() {
        let report = sarif_sample_report();

        let rendered = sarif(&report);
        let value: Value = serde_json::from_str(&rendered).expect("valid SARIF JSON");

        let results = value["runs"][0]["results"].as_array().unwrap();

        // First finding has line: Some(3).
        assert_eq!(
            results[0]["locations"][0]["physicalLocation"]["region"]["startLine"],
            3
        );

        // Second finding has line: None -> no "region" key at all.
        assert!(results[1]["locations"][0]["physicalLocation"]
            .get("region")
            .is_none());
    }

    #[test]
    fn sarif_driver_rules_are_distinct_sorted_rule_ids() {
        let report = sarif_sample_report();

        let rendered = sarif(&report);
        let value: Value = serde_json::from_str(&rendered).expect("valid SARIF JSON");

        let rules: Vec<&str> = value["runs"][0]["tool"]["driver"]["rules"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["id"].as_str().unwrap())
            .collect();

        // "name.charset" appears twice in the report (Error + Info) but must
        // be deduped; "description.required" also present; sorted by id.
        assert_eq!(rules, vec!["description.required", "name.charset"]);
    }

    #[test]
    fn sarif_results_preserve_report_order() {
        let report = sarif_sample_report();

        let rendered = sarif(&report);
        let value: Value = serde_json::from_str(&rendered).expect("valid SARIF JSON");

        let rule_ids: Vec<&str> = value["runs"][0]["results"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["ruleId"].as_str().unwrap())
            .collect();

        // Report order: section 1's two findings (error, warning), then
        // section 2's one finding (info) — not sorted by rule id or level.
        assert_eq!(
            rule_ids,
            vec!["name.charset", "description.required", "name.charset"]
        );
    }

    #[test]
    fn sarif_clean_report_has_empty_results_and_rules() {
        let report = Report {
            sections: Vec::new(),
            summary: Summary::default(),
        };

        let rendered = sarif(&report);
        let value: Value = serde_json::from_str(&rendered).expect("valid SARIF JSON");

        assert_eq!(value["runs"][0]["results"].as_array().unwrap().len(), 0);
        assert_eq!(
            value["runs"][0]["tool"]["driver"]["rules"]
                .as_array()
                .unwrap()
                .len(),
            0
        );
    }
}
