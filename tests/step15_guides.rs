//! ガイドの印（REQ-core-198〜REQ-core-206、TBL-core-036）と、その設定の鍵 "guides.files"

use kotowari_core::config::Config;
use std::path::Path;
use tempfile::TempDir;

/// 置き場と設定を作る。"guides.files" は `guides` の行をそのまま設定に足す
fn make_project(tmp: &Path, guides: &str) {
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
        format!("tests:\n  files:\n    - \"tests/**/*.rs\"\n{guides}"),
    )
    .unwrap();
    write(
        tmp,
        "docs/decision/records/r.md",
        "# Records\n\n## Agreements\n\n- A1 Agreement\n",
    );
}

/// ファイルを書く（親のディレクトリは作る）
fn write(tmp: &Path, rel: &str, content: &str) {
    let path = tmp.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, content).unwrap();
}

/// コマンドを走らせて (終了コード, 標準出力, 標準エラー) を返す
fn run(tmp: &Path, args: &[&str]) -> (Option<i32>, String, String) {
    let output = assert_cmd::Command::cargo_bin("kotowari")
        .unwrap()
        .args(args)
        .current_dir(tmp)
        .output()
        .unwrap();
    (
        output.status.code(),
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

/// 標準出力を JSON として読む
fn json(stdout: &str) -> serde_json::Value {
    serde_json::from_str(stdout).unwrap_or_else(|e| panic!("{e}: {stdout}"))
}

/// "guides.files" を "guides/**/*.md" にする設定の行
const GUIDES_MD: &str = "guides:\n  files:\n    - \"guides/**/*.md\"\n";

/// EX-core-362 の "REQ-001" の本文。`指紋`は "51b1f3da"
const REQ_001: &str = "### REQ-001: 名前\n\n- kind: ubiquitous\n- source: docs/decision/records/r.md#A1\n- verification: unit\n\n文。\n\n";

/// 要求の節と例の節から IR の文書を作る
fn ir_doc(requirements: &str, rest: &str) -> String {
    format!("# 題名\n\n範囲。\n\n## Requirements\n\n{requirements}{rest}")
}

/// "kotowari list" の、その `ID` の1件の "fingerprint"
fn listed_fingerprint(tmp: &Path, id: &str) -> String {
    let (code, stdout, stderr) = run(tmp, &["list"]);
    assert_eq!(code, Some(0), "{stderr}");
    let v = json(&stdout);
    let item = v["items"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["id"] == id)
        .unwrap_or_else(|| panic!("no {id} in {v}"));
    item["fingerprint"]
        .as_str()
        .unwrap_or_else(|| panic!("no fingerprint: {item}"))
        .to_string()
}

// --- REQ-core-014、TBL-core-004: "guides.files" の鍵 ---

// @kotowari[TBL-core-004]
#[test]
fn tbl_004_guides_files_defaults_to_an_empty_list() {
    assert!(Config::default().guides.files.is_empty());
    let cfg = Config::parse("ir: docs/ir\n").unwrap();
    assert!(
        cfg.guides.files.is_empty(),
        "without the key there are no guides"
    );
}

// @kotowari[TBL-core-004]
#[test]
fn tbl_004_guides_files_reads_a_list_of_globs() {
    let cfg = Config::parse("guides:\n  files:\n    - \"guides/**/*.md\"\n    - \"README.md\"\n")
        .unwrap();
    assert_eq!(cfg.guides.files, vec!["guides/**/*.md", "README.md"]);
}

// @kotowari[REQ-core-014]
#[test]
fn req_014_an_unreadable_guides_glob_is_a_config_error() {
    let tmp = tempfile::TempDir::new().unwrap();
    for dir in [
        ".kotowari",
        "docs/ir",
        "docs/decision/records",
        "docs/decision/adr",
    ] {
        std::fs::create_dir_all(tmp.path().join(dir)).unwrap();
    }
    std::fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "guides:\n  files:\n    - \"[invalid\"\n",
    )
    .unwrap();
    let output = assert_cmd::Command::cargo_bin("kotowari")
        .unwrap()
        .arg("check")
        .current_dir(tmp.path())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout, b"");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.starts_with("config error: "), "{stderr}");
    // 知らない鍵としてではなく、glob として読めない要素として止まる
    assert!(
        stderr.contains("invalid glob pattern: [invalid"),
        "{stderr}"
    );
}

// --- REQ-core-203、TBL-core-026: 指紋 ---

// @kotowari[REQ-core-203, TBL-core-026, EX-core-372]
#[test]
fn ex_372_list_shows_the_fingerprint_of_a_requirement() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), "");
    write(tmp.path(), "docs/ir/a.md", &ir_doc(REQ_001, ""));
    assert_eq!(listed_fingerprint(tmp.path(), "REQ-001"), "51b1f3da");
}

// @kotowari[REQ-core-203, TBL-core-026]
#[test]
fn req_203_a_table_keeps_its_leading_blank_line_after_the_source_line_is_dropped() {
    // 本文は出典の行、空の行、表の3行。出典の行を除いた後で先頭の空の行を除き直さないので、
    // 指紋は "\n| a | b |\n|---|---|\n| 1 | 2 |" の SHA-256 の先頭8文字（sha256sum で計算した値）
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), "");
    write(
        tmp.path(),
        "docs/ir/a.md",
        &ir_doc(
            REQ_001,
            "## Decision tables\n\n### TBL-001: 表\n\n- source: docs/decision/records/r.md#A1\n\n| a | b |\n|---|---|\n| 1 | 2 |\n",
        ),
    );
    assert_eq!(listed_fingerprint(tmp.path(), "TBL-001"), "39edaaf8");
}

/// 注釈の行と空の行を挟んだ`シナリオ`の "EX-001" を持つ文書
fn scenario_doc(tags: &str, name: &str) -> String {
    ir_doc(
        REQ_001,
        &format!(
            "## Examples\n\n```gherkin\n{tags}\nScenario: {name}\n  # 注釈\n  Given 何か\n\n  Then 結果\n```\n"
        ),
    )
}

// @kotowari[REQ-core-203, TBL-core-026]
#[test]
fn req_203_a_scenario_fingerprint_is_its_step_lines_only() {
    // ステップの2行だけ: "  Given 何か\n  Then 結果" の SHA-256 の先頭8文字（sha256sum で計算した値）
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), "");
    write(
        tmp.path(),
        "docs/ir/a.md",
        &scenario_doc(
            "@id=EX-001 @about=REQ-001 @source=docs/decision/records/r.md#A1",
            "例",
        ),
    );
    assert_eq!(listed_fingerprint(tmp.path(), "EX-001"), "ec19e8a0");
}

// @kotowari[REQ-core-203]
#[test]
fn req_203_heading_name_and_source_do_not_change_the_fingerprint() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), "");
    let renamed = REQ_001
        .replace("名前", "別の名前")
        .replace("r.md#A1\n", "r.md#A1, docs/decision/records/r.md#A1\n");
    write(tmp.path(), "docs/ir/a.md", &ir_doc(&renamed, ""));
    assert_eq!(listed_fingerprint(tmp.path(), "REQ-001"), "51b1f3da");
}

// @kotowari[REQ-core-203]
#[test]
fn req_203_scenario_name_and_tags_do_not_change_the_fingerprint() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), "");
    write(
        tmp.path(),
        "docs/ir/a.md",
        &scenario_doc(
            "@id=EX-001 @about=REQ-001 @source=docs/decision/records/r.md#A1,docs/decision/records/r.md#A1",
            "別の名前",
        ),
    );
    assert_eq!(listed_fingerprint(tmp.path(), "EX-001"), "ec19e8a0");
}

// @kotowari[REQ-core-203]
#[test]
fn req_203_crlf_line_endings_do_not_change_the_fingerprint() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), "");
    write(
        tmp.path(),
        "docs/ir/a.md",
        &ir_doc(REQ_001, "").replace('\n', "\r\n"),
    );
    assert_eq!(listed_fingerprint(tmp.path(), "REQ-001"), "51b1f3da");
}
