#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]
use assert_cmd::Command;
use std::fs;
use tempfile::TempDir;

fn cmd() -> Command {
    Command::cargo_bin("kotowari").unwrap()
}

fn make_project(tmp: &std::path::Path) {
    fs::create_dir_all(tmp.join(".kotowari")).unwrap();
    fs::create_dir_all(tmp.join("docs/decisions")).unwrap();
    fs::write(
        tmp.join(".kotowari/config.yaml"),
        "decisions:\n  documents: docs/decisions\n",
    )
    .unwrap();
}

fn parse_json(output: &std::process::Output) -> serde_json::Value {
    let stdout = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str(&stdout).expect("valid JSON")
}

fn findings_by_kind(v: &serde_json::Value, kind: &str) -> Vec<serde_json::Value> {
    v["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["kind"] == kind)
        .cloned()
        .collect()
}

// --- 文字の出力に使う種類の語 ---

// @kotowari[C-core-output-8]
#[test]
fn req_025_finding_kind_displays_as_its_own_kind_word() {
    // --format text の出力は "{kind}" を通して種類の語を書く（CLI の text の出力）。
    // その Display の実装が種類ごとの語を書くことを直接確かめる。
    assert_eq!(
        kotowari_core::FindingKind::MissingTitle.to_string(),
        "missing_title",
        "Display should write the kind word, not an empty string"
    );
    assert_eq!(
        kotowari_core::FindingKind::DuplicateId.to_string(),
        "duplicate_id"
    );
}

// --- FindingKind と str の等価 ---

// @kotowari[C-core-forder-1]
#[test]
fn req_024_a_finding_kind_equals_only_its_own_name() {
    // `==` に書いた文字列リテラルは `PartialEq<&str>` を通る（既存のテストで検査済み）。
    // ここでは `PartialEq<str>` 側の実装を `.eq()` で直接呼び、
    // 自分の種類の名前だけに真を返すことを確かめる。
    assert!(
        kotowari_core::FindingKind::DuplicateId.eq("duplicate_id"),
        "should equal its own kind name"
    );
    assert!(
        !kotowari_core::FindingKind::DuplicateId.eq("missing_title"),
        "should not equal a different kind name"
    );
}

// --- 誤りの種類の detail ---

// --- 警告の種類の detail ---

// --- 注意は表の種類だけ ---

// @kotowari[C-core-findings-1]
#[test]
fn req_031_only_the_kinds_of_the_table_are_notices() {
    // 本体が注意にする種類（"kotowari mutants" の2種類は check の出力には現れない）
    let notices: std::collections::BTreeSet<&str> = kotowari_core::FindingKind::ALL
        .iter()
        .filter(|kind| kind.severity() == "notice")
        .map(|kind| kind.as_str())
        .collect();
    assert_eq!(
        notices,
        std::collections::BTreeSet::from([
            "equivalent_stale",
            "guide_stale",
            "mutant_timeout",
            "surface_unspecified_stale",
        ])
    );
    // ほかの種類はすべて誤り
    for kind in kotowari_core::FindingKind::ALL {
        if !notices.contains(kind.as_str()) {
            assert_eq!(kind.severity(), "error", "{kind} should be an error");
        }
    }
}

// @kotowari[C-core-findings-1]
#[test]
fn req_031_check_reports_notices_only_for_the_notice_kinds() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // 指紋の古いガイドの印（guide_stale）と、問題の節の無い決定（誤り）を出す
    fs::write(
        tmp.path().join("docs/decisions/a.md"),
        "# Title\n\nScope.\n\n## Greet\n\nStatus: implemented\n\n### Decision\n\nGreet.\n",
    )
    .unwrap();
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "decisions:\n  documents: docs/decisions\nguides:\n  files:\n    - \"guide.md\"\n",
    )
    .unwrap();
    fs::write(
        tmp.path().join("guide.md"),
        "# Guide\n\n<!-- @kotowari[docs/decisions/a.md#greet:00000000] -->\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let findings = v["findings"].as_array().unwrap();
    let notices: Vec<_> = findings
        .iter()
        .filter(|f| f["severity"] == "notice")
        .collect();
    // 警告が実際に存在すること
    assert!(
        !notices.is_empty(),
        "test should produce at least one notice"
    );
    // 注意は、重大度が注意の種類のどれかだけ
    let notice_kinds: std::collections::BTreeSet<&str> = kotowari_core::FindingKind::ALL
        .iter()
        .filter(|kind| kind.severity() == "notice")
        .map(|kind| kind.as_str())
        .collect();
    for w in &notices {
        let kind = w["kind"].as_str().unwrap();
        assert!(
            notice_kinds.contains(kind),
            "unexpected notice kind: {kind}"
        );
    }
    // 警告でない指摘はすべて error
    for f in findings {
        let sev = f["severity"].as_str().unwrap();
        if sev != "notice" {
            assert_eq!(
                sev, "error",
                "non-notice finding should be error, got {sev}: {:?}",
                f
            );
        }
    }
}

// --- 文書全体への指摘は null ---

// @kotowari[C-core-forder-4]
#[test]
fn req_027_document_wide_findings_have_null_line() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(tmp.path().join("docs/decisions/a.md"), "No title\n").unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let mt = findings_by_kind(&v, "missing_title");
    assert!(
        mt[0]["line"].is_null(),
        "missing_title should have null line"
    );
}

// --- 行は1始まり ---

// @kotowari[C-core-forder-3]
#[test]
fn req_028_lines_start_at_one() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // 7行目の見出しで unknown_heading を出す
    // line 1: # Title
    // line 2: (空)
    // line 3: Scope.
    // line 4: (空)
    // line 5: ## Requirements（決定の見出し）
    // line 6: (空)
    // line 7: ### Bad Heading
    fs::write(
        tmp.path().join("docs/decisions/a.md"),
        "# Title\n\nScope.\n\n## Requirements\n\n### Bad Heading\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let uh = findings_by_kind(&v, "unknown_heading");
    assert!(!uh.is_empty());
    let line = uh[0]["line"].as_u64().unwrap();
    assert_eq!(line, 7, "### Bad Heading is on line 7 (1-indexed)");
}

// --- 指摘の並び ---

// @kotowari[C-core-forder-1]
#[test]
fn req_024_sorted_by_path_line_kind_detail() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // 複数の文書に指摘を出す
    fs::write(tmp.path().join("docs/decisions/b.md"), "no title\n").unwrap();
    fs::write(
        tmp.path().join("docs/decisions/a.md"),
        "# A\n\nScope.\n\n## Greet\n\nStatus: implemented\n\n### Decision\n\n適切に挨拶する。など。\n\n### Confirmations\n\n- C-a-2 Two\n- C-a-1 One\n- C-a-1 Again\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let findings = v["findings"].as_array().unwrap();
    assert!(findings.len() >= 5, "{v}");

    // path でソートされていること
    for i in 1..findings.len() {
        let prev_path = findings[i - 1]["path"].as_str().unwrap();
        let curr_path = findings[i]["path"].as_str().unwrap();
        let prev_line = findings[i - 1]["line"].as_u64();
        let curr_line = findings[i]["line"].as_u64();
        let prev_kind = findings[i - 1]["kind"].as_str().unwrap();
        let curr_kind = findings[i]["kind"].as_str().unwrap();
        let prev_detail = findings[i - 1]["detail"].as_str().unwrap();
        let curr_detail = findings[i]["detail"].as_str().unwrap();

        let path_cmp = prev_path.cmp(curr_path);
        let line_cmp = match (prev_line, curr_line) {
            (None, None) => std::cmp::Ordering::Equal,
            (None, Some(_)) => std::cmp::Ordering::Less,
            (Some(_), None) => std::cmp::Ordering::Greater,
            (Some(a), Some(b)) => a.cmp(&b),
        };
        let kind_cmp = prev_kind.cmp(curr_kind);
        let detail_cmp = prev_detail.cmp(curr_detail);

        let overall = path_cmp.then(line_cmp).then(kind_cmp).then(detail_cmp);
        assert!(
            overall != std::cmp::Ordering::Greater,
            "findings not sorted at index {i}: prev=({prev_path},{prev_line:?},{prev_kind},{prev_detail}) curr=({curr_path},{curr_line:?},{curr_kind},{curr_detail})"
        );
    }
}

// --- findings は並んでいる ---

// @kotowari[C-core-forder-2]
#[test]
fn prop_003_findings_are_sorted() {
    // run_check を通して、ライブラリが返す findings が
    // path → line → kind → detail の順で並んでいることを検証する。
    //
    // 決定の文書の指摘（後のファイル名の文書）とテスト発見の指摘（前のファイル
    // 名のテストファイル）が交互に追加されるため、ソートしないと壊れる。
    // 文書の数と項目の数の組は6通りしかないので、乱択せずにすべて試す
    for (n_docs, n_items) in
        (2..5usize).flat_map(|n_docs| (1..3usize).map(move |n_items| (n_docs, n_items)))
    {
        let tmp = tempfile::TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
        fs::create_dir_all(tmp.path().join("docs/decisions")).unwrap();
        fs::create_dir_all(tmp.path().join("tests")).unwrap();
        fs::write(tmp.path().join(".kotowari/config.yaml"), "").unwrap();
        for i in 0..n_docs {
            // 決定ごとに問題の節を欠き、確認することの行はテストの無い行になる
            let mut content = format!("# Title {i}\n\nScope {i}.\n\n");
            for j in 0..n_items {
                let id_num = i * 10 + j + 1;
                content.push_str(&format!(
                    "## D{id_num}\n\nStatus: implemented\n\n### Decision\n\nD.\n\n### Confirmations\n\n- C-x-{id_num} Checked\n\n"
                ));
            }
            fs::write(
                tmp.path().join(format!("docs/decisions/doc{i}.md")),
                content,
            )
            .unwrap();
        }
        // 印の無いテストファイルを置く（test_without_id が tests/ パスで出る）
        fs::write(
            tmp.path().join("tests/check.rs"),
            "#[test]\nfn unmarked() {}\n",
        )
        .unwrap();
        let result = kotowari::Project::new(kotowari::ProjectOptions::new(tmp.path()))
            .unwrap()
            .check()
            .expect("run_check should succeed");
        // findings が 順で並んでいることを検証する
        let findings = result.findings();
        for i in 1..findings.len() {
            let a = &findings[i - 1];
            let b = &findings[i];
            let cmp = a
                .path()
                .cmp(b.path())
                .then_with(|| match (a.line(), b.line()) {
                    (None, None) => std::cmp::Ordering::Equal,
                    (None, Some(_)) => std::cmp::Ordering::Less,
                    (Some(_), None) => std::cmp::Ordering::Greater,
                    (Some(al), Some(bl)) => al.cmp(&bl),
                })
                .then_with(|| a.kind().as_str().cmp(b.kind().as_str()))
                .then_with(|| a.detail().cmp(b.detail()));
            assert!(
                cmp != std::cmp::Ordering::Greater,
                "findings not sorted at index {}: prev=({},{:?},{},{}) curr=({},{:?},{},{})",
                i,
                a.path(),
                a.line(),
                a.kind(),
                a.detail(),
                b.path(),
                b.line(),
                b.kind(),
                b.detail()
            );
        }
    }
}

// --- Step 7: 指摘の種類と行の表 ---

// @kotowari[C-core-markers-7]
#[test]
fn tbl_019_marker_findings_line_is_the_marker_line() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    // invalid_marker の line は印の行
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[]\n#[test]\nfn test_a() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let im = findings_by_kind(&v, "invalid_marker");
    assert!(!im.is_empty(), "should have invalid_marker");
    assert_eq!(
        im[0]["line"], 1,
        "invalid_marker line should be the marker line (1)"
    );
}

// --- 宣言の外の行・コードブロック・用語集の題名 ---

// @kotowari[C-core-clienv-6]
#[test]
fn ex_core_269_a_document_with_a_form_finding_still_takes_the_cross_document_checks() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // a.md の決定は問題の節を欠き、その確認することの ID が b.md の行と重なる
    let decision = |problem: &str| {
        format!(
            "# T\n\nScope.\n\n## Greet\n\nStatus: implemented\n\n{problem}### Decision\n\nGreet.\n\n### Confirmations\n\n- C-x-1 Shown\n"
        )
    };
    fs::write(tmp.path().join("docs/decisions/a.md"), decision("")).unwrap();
    fs::write(
        tmp.path().join("docs/decisions/b.md"),
        decision("### Problem\n\nTrouble.\n\n"),
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let missing = findings_by_kind(&v, "decision_section_missing");
    assert_eq!(missing.len(), 1, "{:?}", v["findings"]);
    assert_eq!(missing[0]["path"], "docs/decisions/a.md");
    let duplicates = findings_by_kind(&v, "duplicate_id");
    assert_eq!(duplicates.len(), 1, "{:?}", v["findings"]);
    assert_eq!(duplicates[0]["path"], "docs/decisions/b.md");
    assert_eq!(duplicates[0]["detail"], "C-x-1");
}
