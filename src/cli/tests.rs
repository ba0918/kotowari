use super::*;
fn args(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| s.to_string()).collect()
}
// @kotowari[REQ-core-002]
#[test]
fn req_core_002_plan_takes_the_format_before_or_after_the_path() {
    for list in [
        ["plan", "--format", "text", "a.md"],
        ["plan", "a.md", "--format", "text"],
        ["--format", "text", "plan", "a.md"],
    ] {
        let parsed = parse_args(&args(&list));
        assert!(
            matches!(parsed, Ok(Cli::Plan { ref path, .. }) if path == Path::new("a.md")),
            "{list:?}: {parsed:?}"
        );
    }
}

/// ヘッダの1列目が `header` の Markdown の表の、ヘッダと区切りの行を除いた1列目
fn first_column_of_table(markdown: &str, header: &str) -> std::collections::BTreeSet<String> {
    let mut found = std::collections::BTreeSet::new();
    let mut in_table = false;
    for line in markdown.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with('|') {
            in_table = false;
            continue;
        }
        let first = trimmed.split('|').nth(1).unwrap_or("").trim().to_string();
        if first == header {
            in_table = true;
            continue;
        }
        if in_table && !first.chars().all(|c| c == '-' || c == ':') {
            found.insert(first);
        }
    }
    found
}

// @kotowari[REQ-core-127]
#[test]
fn req_127_findings_reference_stop_wordings_match_the_code() {
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("agent/skills/kotowari/references/findings.md");
    let markdown = std::fs::read_to_string(&path).unwrap();
    let in_reference = first_column_of_table(&markdown, "Message");
    // 標準エラーの1行目は、詳細の前がこの文言になる。core の停止の理由、ライブラリの失敗、CLI の停止の理由
    let in_code: std::collections::BTreeSet<String> = kotowari_core::StopReason::WORDINGS
        .iter()
        .chain(kotowari::Error::WORDINGS)
        .chain(StopReason::WORDINGS)
        .map(|wording| wording.to_string())
        .collect();
    assert_eq!(
        in_reference, in_code,
        "the stop wordings in references/findings.md should be exactly the wordings the code prints"
    );
}

// @kotowari[REQ-core-297]
#[test]
fn req_core_297_serve_uses_port_4590_unless_given() {
    assert_eq!(DEFAULT_PORT, 4590);
    let parsed = parse_args(&args(&["overview", "serve"]));
    assert!(
        matches!(parsed, Ok(Cli::OverviewServe { port: 4590, .. })),
        "{parsed:?}"
    );
    let parsed = parse_args(&args(&["overview", "serve", "--port", "65535"]));
    assert!(
        matches!(parsed, Ok(Cli::OverviewServe { port: 65535, .. })),
        "{parsed:?}"
    );
}

// @kotowari[REQ-core-002]
#[test]
fn req_core_002_check_takes_allow_test_findings_without_a_value() {
    // 値を取らないので、すぐ後の語はコマンドやほかのオプションとして読まれる
    for list in [
        &["check", "--allow-test-findings"][..],
        &["--allow-test-findings", "check"],
        &["check", "--allow-test-findings", "--format", "text"],
        &["--allow-test-findings", "--format", "text", "check"],
    ] {
        let parsed = parse_args(&args(list));
        assert!(
            matches!(
                parsed,
                Ok(Cli::Check {
                    allow_test_findings: true,
                    ..
                })
            ),
            "{list:?}: {parsed:?}"
        );
    }
    let parsed = parse_args(&args(&[
        "check",
        "--allow-test-findings",
        "--format",
        "text",
    ]));
    assert!(
        matches!(
            parsed,
            Ok(Cli::Check {
                format: Format::Text,
                ..
            })
        ),
        "{parsed:?}"
    );
    let parsed = parse_args(&args(&["check"]));
    assert!(
        matches!(
            parsed,
            Ok(Cli::Check {
                allow_test_findings: false,
                ..
            })
        ),
        "{parsed:?}"
    );
}

fn assert_argument_error(list: &[&str]) {
    let parsed = parse_args(&args(list));
    assert!(
        matches!(parsed, Err(StopReason::ArgumentError(_))),
        "{list:?}: {parsed:?}"
    );
}

// @kotowari[REQ-core-004]
#[test]
fn req_core_004_allow_test_findings_on_a_command_other_than_check_stops() {
    for list in [
        &[
            "changes",
            "--base",
            "HEAD",
            "--head",
            "HEAD",
            "--phase",
            "review",
            "--allow-test-findings",
        ][..],
        &[
            "--allow-test-findings",
            "mutants",
            "--tool",
            "cargo-mutants",
            "outcomes.json",
        ],
        &["plan", "a.md", "--allow-test-findings"],
        &["list", "--allow-test-findings"],
        &["query", "REQ-001", "--allow-test-findings"],
        &["status", "--allow-test-findings"],
        &["overview", "build", "--allow-test-findings"],
        &["overview", "serve", "--allow-test-findings"],
    ] {
        assert_argument_error(list);
    }
}

// @kotowari[REQ-core-004]
#[test]
fn req_core_004_allow_test_findings_twice_on_check_stops() {
    assert_argument_error(&["check", "--allow-test-findings", "--allow-test-findings"]);
}
