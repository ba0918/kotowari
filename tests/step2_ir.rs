use kotowari_core::Finding;
use kotowari_core::config::Config;
use kotowari_core::ir::{self, IrDocument, Item};

fn default_config() -> Config {
    Config::default()
}

fn check(docs: &[IrDocument], config: &Config) -> Vec<Finding> {
    ir::check_documents(docs, config)
}

fn find_by_kind<'a>(findings: &'a [Finding], kind: &str) -> Vec<&'a Finding> {
    findings.iter().filter(|f| f.kind() == kind).collect()
}

// --- REQ-core-034: 題名が無い ---

// --- REQ-core-035: 題名が複数 ---

// --- REQ-core-036: 範囲の行が無い ---

// --- REQ-core-037: 行の数え方 ---

// --- REQ-core-038: 行数の上限 ---

// --- REQ-core-039: 要求の数の上限 ---

// --- REQ-core-040: コードブロックの中はスキップ ---

// --- REQ-core-042: すべての項目の形 ---

// --- REQ-core-043: 形に合わない見出し ---

// --- REQ-core-044: 知らない行 ---

// --- REQ-core-045: 同じ行の重複 ---

// --- REQ-core-046: 順不同、空行、コンマ ---

// --- REQ-core-047: 文が無い ---

// --- REQ-core-048: 検証の行が無い ---

// --- REQ-core-049: 検証の値の誤り ---

// --- REQ-core-050: 種類の値の誤り ---

// --- REQ-core-051: 定義の無い algorithm ---

// --- REQ-core-098: 必須の行が無い ---

// --- REQ-core-099: 決定表に表がない ---

// --- REQ-core-100: gherkin の外の Scenario は無視 ---

// --- REQ-core-042, REQ-core-053: タグなし連続シナリオ ---

// --- REQ-core-052: 知らないタグ ---

// --- REQ-core-053: 無いタグ ---

// --- REQ-core-054: 参照切れ ---

// --- REQ-core-032: ID の重複 ---

// --- REQ-core-044, TBL-core-011: 性質の知らない行 ---

// --- REQ-core-047, REQ-core-048, REQ-core-049, REQ-core-050, REQ-core-051, REQ-core-098: フィールド検査の組み合わせ ---

// --- split_lines: CRLF と空入力 ---

// --- gherkin ステップ認識 ---

// --- 用語集テーブルの解析 ---

// --- FLAG の関係と出典のフィールド読み取り ---

// --- build_scenario の @source ---

// --- check_documents の行数境界値 ---

// --- check_backtick_ids ---

// --- check_references: Property の文中のバッククォート ID ---

// --- TBL 出典が正しく読まれて missing_source にならない ---

// --- Step 3: 文書の読み込み ---

// @kotowari[REQ-core-033]
#[test]
fn req_033_uppercase_md_is_not_read() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/ir")).unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/decision/records")).unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/decision/adr")).unwrap();
    std::fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/records\n  adr: docs/decision/adr\n",
    )
    .unwrap();
    // .MD ファイルは読まない
    std::fs::write(tmp.path().join("docs/ir/README.MD"), "# Title\n\nScope.\n").unwrap();
    let output = assert_cmd::Command::cargo_bin("kotowari")
        .unwrap()
        .arg("check")
        .current_dir(tmp.path())
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    assert_eq!(v["files"], 0, ".MD file should not be read");
}

// @kotowari[REQ-core-033]
#[test]
fn req_033_file_symlink_is_read() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/ir")).unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/decision/records")).unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/decision/adr")).unwrap();
    std::fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/records\n  adr: docs/decision/adr\n",
    )
    .unwrap();
    // 実体を別の場所に作り、シンボリックリンクを ir/ に置く
    let target = tmp.path().join("target.md");
    std::fs::write(&target, "# Title\n\nScope.\n").unwrap();
    std::os::unix::fs::symlink(&target, tmp.path().join("docs/ir/link.md")).unwrap();
    let output = assert_cmd::Command::cargo_bin("kotowari")
        .unwrap()
        .arg("check")
        .current_dir(tmp.path())
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    assert_eq!(v["files"], 1, "symlinked file should be read");
}

// --- Step 4a: 項目の行の形 ---

// --- Step 4b: gherkin の行の形と ID の定義 ---

// @kotowari[REQ-core-033]
#[test]
#[cfg(unix)]
fn req_033_broken_symlink_in_ir_dir_stops() {
    use std::os::unix::fs::symlink;
    let tmp = tempfile::TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/ir")).unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/decision/records")).unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/decision/adr")).unwrap();
    std::fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/records\n  adr: docs/decision/adr\n",
    )
    .unwrap();
    symlink(
        tmp.path().join("nowhere.md"),
        tmp.path().join("docs/ir/broken.md"),
    )
    .unwrap();
    let output = assert_cmd::Command::cargo_bin("kotowari")
        .unwrap()
        .arg("check")
        .current_dir(tmp.path())
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(2),
        "a broken symlink in the IR dir must stop: {:?}",
        output
    );
}

// --- 汎用化の実装レビューで見つかった食い違いの回帰テスト ---

// --- TBL-core-010: split_lines の \r\n 処理 ---

// --- REQ-core-117: 用語集の区切り行の判定 ---

// --- REQ-core-043: "####" 系見出しの検査 ---

// --- REQ-core-112: 閉じないコードブロックの前の指摘・項目は残る ---

// --- REQ-core-098: 値が空でも「知らない行」は無いものとして扱わない ---

// --- REQ-core-044: ". " を含む行の数字接頭辞の判定 ---

// --- REQ-core-043: 形に合わない TBL-/PROP-/FLAG- の ID ---

// --- TBL-core-011: 性質の "- source:" 行の読み方 ---

// --- REQ-core-052: 結び付かないタグの行の行番号 ---

// --- REQ-core-114: シナリオの id に使う @id の値の形 ---

// --- REQ-core-059: @source タグが在るが値が使い物にならないとき ---

// --- REQ-core-098: 問題の記録の必須の行 ---

// --- TBL-core-019: 複数の行を持つ項目での参照切れの行番号 ---

// --- REQ-core-054: 参照の解決 ---

// --- REQ-core-055, REQ-core-056: 見ないもの ---

/// CLI を通して検査するプロジェクトを一時ディレクトリに作る
fn make_cli_project(tmp: &std::path::Path) {
    for dir in [
        ".kotowari",
        "docs/ir",
        "docs/decision/records",
        "docs/decision/adr",
    ] {
        std::fs::create_dir_all(tmp.join(dir)).unwrap();
    }
    std::fs::write(
        tmp.join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/records\n  adr: docs/decision/adr\n",
    )
    .unwrap();
    std::fs::write(
        tmp.join("docs/decision/records/records.md"),
        "# Records\n\n## Agreements\n\n- A1 Agreement\n",
    )
    .unwrap();
}

/// CLI を走らせて JSON を返す
fn run_cli(tmp: &std::path::Path) -> serde_json::Value {
    let output = assert_cmd::Command::cargo_bin("kotowari")
        .unwrap()
        .arg("check")
        .current_dir(tmp)
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str(&stdout).expect("valid JSON")
}

/// 指定した文書の指定した行を指す指摘を集める
fn findings_on_line(v: &serde_json::Value, path: &str, line: u64) -> Vec<serde_json::Value> {
    v["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["path"] == path && f["line"] == line)
        .cloned()
        .collect()
}

// @kotowari[REQ-core-055]
#[test]
fn req_055_non_ears_statement_gets_no_finding_on_its_line() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    make_cli_project(tmp.path());
    // 13行目の文は「常に」も「とき」も持たない平叙文
    std::fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## Requirements\n\n### REQ-001: R\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: unit\n\nこの道具は文書を読む。\n",
    )
    .unwrap();
    let v = run_cli(tmp.path());
    assert!(
        findings_on_line(&v, "docs/ir/a.md", 13).is_empty(),
        "the statement line should get no finding: {v}"
    );
    // 検査そのものは動いている（要求の見出しの行にはテストのない要求の誤りが出る）
    assert!(
        !findings_on_line(&v, "docs/ir/a.md", 7).is_empty(),
        "the requirement heading should still be checked: {v}"
    );
}

// @kotowari[REQ-core-056]
#[test]
fn req_056_contradiction_flag_with_one_reading_gets_no_finding() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    make_cli_project(tmp.path());
    std::fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## Requirements\n\n### REQ-001: R\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: unit\n\nこの道具は文書を読む。\n",
    )
    .unwrap();
    // 種類 contradiction の問題の記録に、読みを1つだけ書く
    std::fs::write(
        tmp.path().join("docs/ir/FLAGS.md"),
        "# 問題の記録\n\n## Flags\n\n### FLAG-001: 読みが割れる\n\n- kind: contradiction\n- related: REQ-001\n- source: docs/decision/records/records.md#A1\n\n読みは1つだけ書いてある。\n",
    )
    .unwrap();
    let v = run_cli(tmp.path());
    let on_flags: Vec<_> = v["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["path"] == "docs/ir/FLAGS.md")
        .collect();
    assert!(
        on_flags.is_empty(),
        "the contradiction entry should get no finding: {on_flags:?}"
    );
    // 検査そのものは動いている
    assert!(
        !findings_on_line(&v, "docs/ir/a.md", 7).is_empty(),
        "the requirement heading should still be checked: {v}"
    );
}

// --- ir-document.md の具体例 ---

// @kotowari[REQ-core-036, EX-core-006]
#[test]
fn req_036_glossary_with_only_a_title_and_a_table_has_no_missing_scope() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    make_cli_project(tmp.path());
    std::fs::write(
        tmp.path().join("docs/ir/CONTEXT.md"),
        "# Glossary\n\n| Term | Meaning | Source |\n|---|---|---|\n",
    )
    .unwrap();
    let v = run_cli(tmp.path());
    assert!(
        find_kind_in_json(&v, "missing_scope").is_empty(),
        "a glossary needs no scope line: {v}"
    );
}

// @kotowari[REQ-core-033, EX-core-021]
#[test]
fn req_033_context_and_flags_under_a_subdirectory_have_no_missing_scope() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    make_cli_project(tmp.path());
    std::fs::create_dir_all(tmp.path().join("docs/ir/network")).unwrap();
    std::fs::write(
        tmp.path().join("docs/ir/network/CONTEXT.md"),
        "# Glossary\n\n| Term | Meaning | Source |\n|---|---|---|\n",
    )
    .unwrap();
    std::fs::write(
        tmp.path().join("docs/ir/network/FLAGS.md"),
        "# 問題の記録\n",
    )
    .unwrap();
    let v = run_cli(tmp.path());
    assert!(
        find_kind_in_json(&v, "missing_scope").is_empty(),
        "CONTEXT.md and FLAGS.md under a subdirectory are a glossary and a flags document: {v}"
    );
}

// @kotowari[REQ-core-033, EX-core-020]
#[test]
fn req_033_a_document_deep_in_the_tree_is_read_and_an_empty_directory_is_silent() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    make_cli_project(tmp.path());
    std::fs::create_dir_all(tmp.path().join("docs/ir/network/dns")).unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/ir/network/empty")).unwrap();
    std::fs::write(
        tmp.path().join("docs/ir/network/dns/timeout.md"),
        "# Title\n\nScope.\n\n## Requirements\n\n### REQ-001: R\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: unit\n\nStatement.\n",
    )
    .unwrap();
    let v = run_cli(tmp.path());
    let missing = find_kind_in_json(&v, "requirement_without_test");
    assert_eq!(missing.len(), 1, "the deep document is read: {v}");
    assert_eq!(missing[0]["path"], "docs/ir/network/dns/timeout.md");
    assert!(
        v["findings"].as_array().unwrap().iter().all(|f| {
            !f["path"]
                .as_str()
                .unwrap()
                .starts_with("docs/ir/network/empty")
        }),
        "an empty directory gets no finding: {v}"
    );
}

// @kotowari[REQ-core-033, EX-core-030]
#[test]
#[cfg(unix)]
fn req_033_hidden_directory_and_directory_symlink_deep_in_the_tree_are_not_read() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    make_cli_project(tmp.path());
    std::fs::create_dir_all(tmp.path().join("docs/ir/network/.draft")).unwrap();
    // 読まれれば題名も範囲も無い文書として指摘が出る中身
    std::fs::write(tmp.path().join("docs/ir/network/.draft/a.md"), "bad").unwrap();
    std::os::unix::fs::symlink(
        tmp.path().join("docs/ir"),
        tmp.path().join("docs/ir/network/link"),
    )
    .unwrap();
    let output = assert_cmd::Command::cargo_bin("kotowari")
        .unwrap()
        .arg("check")
        .current_dir(tmp.path())
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(0),
        "neither a finding nor a stop: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let v: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid JSON");
    assert_eq!(v["files"], 0, "no document under either place is read: {v}");
    assert!(v["findings"].as_array().unwrap().is_empty(), "{v}");
}

/// JSON の findings から種類で絞る
fn find_kind_in_json(v: &serde_json::Value, kind: &str) -> Vec<serde_json::Value> {
    v["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["kind"] == kind)
        .cloned()
        .collect()
}

// --- ir-references.md の具体例 ---

// @kotowari[REQ-core-052, EX-core-009]
#[test]
fn req_052_retired_tag_on_a_scenario_is_unknown_tag() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    make_cli_project(tmp.path());
    std::fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## Requirements\n\n### REQ-001: R\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: review\n\nStatement.\n\n## Examples\n\n```gherkin\n@id=EX-001 @about=REQ-001 @source=docs/decision/records/records.md#A1 @requirement=REQ-001\nScenario: Test\n  Given a\n  When b\n  Then c\n```\n",
    )
    .unwrap();
    let v = run_cli(tmp.path());
    let unknown = find_kind_in_json(&v, "unknown_tag");
    assert_eq!(
        unknown.len(),
        1,
        "the retired @requirement tag is unknown: {v}"
    );
    assert_eq!(unknown[0]["detail"], "@requirement");
}

// @kotowari[REQ-core-054, EX-core-010]
#[test]
fn req_054_scenario_about_an_unknown_requirement_is_unresolved() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    make_cli_project(tmp.path());
    std::fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## Examples\n\n```gherkin\n@id=EX-001 @about=REQ-999 @source=docs/decision/records/records.md#A1\nScenario: Test\n  Given a\n  When b\n  Then c\n```\n",
    )
    .unwrap();
    let v = run_cli(tmp.path());
    let unresolved = find_kind_in_json(&v, "unresolved_reference");
    assert_eq!(unresolved.len(), 1, "@about points nowhere: {v}");
    assert_eq!(unresolved[0]["detail"], "REQ-999");
}

// --- findings.md と ir-items.md の具体例 ---

// @kotowari[REQ-core-032, EX-core-005]
#[test]
fn req_032_three_places_yield_two_duplicates_and_none_on_the_first() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    make_cli_project(tmp.path());
    let requirement = "### REQ-001: R\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: review\n\nStatement.\n\n";
    std::fs::write(
        tmp.path().join("docs/ir/a.md"),
        format!("# Title\n\nScope.\n\n## Requirements\n\n{requirement}{requirement}{requirement}"),
    )
    .unwrap();
    let v = run_cli(tmp.path());
    let duplicates = find_kind_in_json(&v, "duplicate_id");
    assert_eq!(
        duplicates.len(),
        2,
        "three places yield two duplicates: {v}"
    );
    assert!(duplicates.iter().all(|f| f["detail"] == "REQ-001"));
    // 1つ目の見出しは 7 行目
    assert!(
        duplicates.iter().all(|f| f["line"] != 7),
        "the first heading gets no duplicate_id: {v}"
    );
}

// @kotowari[REQ-core-044, EX-core-008]
#[test]
fn req_044_priority_line_under_a_requirement_is_an_unknown_field() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    make_cli_project(tmp.path());
    std::fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## Requirements\n\n### REQ-001: R\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: review\n- 優先度: 高\n\nStatement.\n",
    )
    .unwrap();
    let v = run_cli(tmp.path());
    let unknown = find_kind_in_json(&v, "unknown_field");
    assert_eq!(
        unknown.len(),
        1,
        "the 優先度 line is not a known field: {v}"
    );
    assert_eq!(unknown[0]["detail"], "- 優先度: 高");
}

// --- REQ-core-178: 見出しの下の行は1行ずつ文として読む ---

/// 要求を1つ持つ話題の文書。`fields` は "- kind:" と "- source:" の後に続く行
fn topic_with_requirement(fields: &str) -> String {
    format!(
        "# Title\n\nScope.\n\n## Requirements\n\n### REQ-001: R\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n{fields}"
    )
}

/// 指定した種類の指摘のうち、指定した文書を指すもの
fn kinds_on(v: &serde_json::Value, path: &str) -> Vec<String> {
    v["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["path"] == path)
        .map(|f| f["kind"].as_str().unwrap().to_string())
        .collect()
}

// @kotowari[EX-core-273, REQ-core-178]
#[test]
fn ex_core_273_statement_right_after_a_field_line_is_not_part_of_its_value() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    make_cli_project(tmp.path());
    std::fs::write(
        tmp.path().join("docs/ir/a.md"),
        topic_with_requirement(
            "- verification: review\n検証の次の文。\n- how_to_verify: 見る\n確かめ方の次の文。\n",
        ),
    )
    .unwrap();
    let v = run_cli(tmp.path());
    let kinds = kinds_on(&v, "docs/ir/a.md");
    for kind in [
        "missing_statement",
        "verification_invalid",
        "requirement_without_test",
    ] {
        assert!(!kinds.iter().any(|k| k == kind), "{kind} が出た: {v}");
    }
}

// @kotowari[EX-core-274]
#[test]
fn ex_core_274_indented_quote_and_html_lines_are_statements_checked_for_backticks() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    make_cli_project(tmp.path());
    // 行番号: 字下げした行が13行目、引用が14行目、HTML が15行目
    std::fs::write(
        tmp.path().join("docs/ir/a.md"),
        topic_with_requirement("- verification: unit\n\n  `字下げ\n> `引用\n<div>`HTML</div>\n"),
    )
    .unwrap();
    let v = run_cli(tmp.path());
    for line in [13, 14, 15] {
        let on_line: Vec<_> = findings_on_line(&v, "docs/ir/a.md", line)
            .into_iter()
            .filter(|f| f["kind"] == "unclosed_backtick")
            .collect();
        assert_eq!(on_line.len(), 1, "{line} 行目: {v}");
    }
}

// @kotowari[EX-core-275]
#[test]
fn ex_core_275_pipe_line_that_is_not_a_table_is_checked_as_a_statement() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    make_cli_project(tmp.path());
    // 行番号: 縦棒の行が13行目
    std::fs::write(
        tmp.path().join("docs/ir/a.md"),
        topic_with_requirement("- verification: unit\n\n| a | `x |\n"),
    )
    .unwrap();
    let v = run_cli(tmp.path());
    let on_line = findings_on_line(&v, "docs/ir/a.md", 13);
    assert_eq!(
        on_line
            .iter()
            .filter(|f| f["kind"] == "unclosed_backtick")
            .count(),
        1,
        "{v}"
    );
    assert!(
        !kinds_on(&v, "docs/ir/a.md")
            .iter()
            .any(|k| k == "unknown_line"),
        "{v}"
    );
}

// @kotowari[EX-core-276]
#[test]
fn ex_core_276_flag_entries_without_a_section_are_read() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    make_cli_project(tmp.path());
    std::fs::write(
        tmp.path().join("docs/ir/a.md"),
        topic_with_requirement("- verification: review\n- how_to_verify: 人が読む\n\n文である。\n"),
    )
    .unwrap();
    let flag = |id: &str| {
        format!(
            "### {id}: 例\n\n- kind: gap\n- related: REQ-001\n- source: docs/decision/records/records.md#A1\n\n本文。\n"
        )
    };
    std::fs::write(
        tmp.path().join("docs/ir/FLAGS.md"),
        format!("# 問題の記録\n\n{}", flag("FLAG-001")),
    )
    .unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/ir/sub")).unwrap();
    std::fs::write(
        tmp.path().join("docs/ir/sub/FLAGS.md"),
        format!(
            "# 問題の記録\n\n{}\n## Flags\n\n{}",
            flag("FLAG-002"),
            flag("FLAG-003")
        ),
    )
    .unwrap();
    let v = run_cli(tmp.path());
    for path in ["docs/ir/FLAGS.md", "docs/ir/sub/FLAGS.md"] {
        let kinds = kinds_on(&v, path);
        for kind in ["unknown_heading", "unknown_field", "unknown_line"] {
            assert!(!kinds.iter().any(|k| k == kind), "{path} に {kind}: {v}");
        }
    }
    let output = assert_cmd::Command::cargo_bin("kotowari")
        .unwrap()
        .arg("status")
        .current_dir(tmp.path())
        .output()
        .unwrap();
    let status: serde_json::Value =
        serde_json::from_str(&String::from_utf8_lossy(&output.stdout)).expect("valid JSON");
    assert_eq!(status["items"]["flag"], 3, "{status}");
}

// @kotowari[EX-core-278]
#[test]
fn ex_core_278_three_titles_give_one_multiple_titles_per_extra_title() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    make_cli_project(tmp.path());
    std::fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# 一\n\n範囲。\n\n# 二\n\n#  三  \n",
    )
    .unwrap();
    let v = run_cli(tmp.path());
    let multiple: Vec<_> = v["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["path"] == "docs/ir/a.md" && f["kind"] == "multiple_titles")
        .cloned()
        .collect();
    assert!(multiple.iter().all(|f| f["line"].is_null()), "{v}");
    let mut details: Vec<&str> = multiple
        .iter()
        .map(|f| f["detail"].as_str().unwrap())
        .collect();
    details.sort_unstable();
    assert_eq!(
        details,
        vec!["三", "二"],
        "detail は2つ目と3つ目の題名: {v}"
    );
}
