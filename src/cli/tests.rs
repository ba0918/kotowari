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
        .map(|wording| wording.to_string())
        .collect();
    assert_eq!(
        in_reference, in_code,
        "the stop wordings in references/findings.md should be exactly the wordings the code prints"
    );
}
