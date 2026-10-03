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

// @kotowari[REQ-core-199, REQ-core-313, EX-core-485]
#[test]
fn overlap_precedes_guide_only_invalid_utf8_and_missing_surface_rules() {
    for later in ["content", "surface"] {
        let tmp = TempDir::new().unwrap();
        make_project(
            tmp.path(),
            "guides:\n  files: ['tests/**', 'guides/**']\nsurface:\n  files: ['src/**']\n  rules: ['missing.yaml']\n",
        );
        write(tmp.path(), "tests/z.rs", "#[test]\nfn z() {}\n");
        write(tmp.path(), "tests/a.rs", "#[test]\nfn a() {}\n");
        if later == "content" {
            std::fs::create_dir_all(tmp.path().join("guides")).unwrap();
            std::fs::write(tmp.path().join("guides/bad.md"), [0xff]).unwrap();
        }
        let (code, _, stderr) = run(tmp.path(), &["check", "--format", "json"]);
        assert_eq!(code, Some(2));
        assert!(
            stderr.contains("tests/a.rs: matched by both guides.files and tests.files"),
            "{later}: {stderr}"
        );
    }
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
    // シナリオのステップの行にも "\r" を入れない
    write(
        tmp.path(),
        "docs/ir/a.md",
        &scenario_doc(
            "@id=EX-001 @about=REQ-001 @source=docs/decision/records/r.md#A1",
            "例",
        )
        .replace('\n', "\r\n"),
    );
    assert_eq!(listed_fingerprint(tmp.path(), "EX-001"), "ec19e8a0");
}

// @kotowari[REQ-core-203, REQ-core-209]
#[test]
fn req_203_a_document_level_declaration_leads_the_fingerprint_of_its_requirements_only() {
    // 要求は "- deferred: docs/decision/records/r.md#A1" の行を先頭に加えた並び
    // （sha256sum で計算した値）。決定表とシナリオの指紋は宣言の無いときと同じ
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), "");
    let doc = ir_doc(
        REQ_001,
        "## Decision tables\n\n### TBL-001: 表\n\n- source: docs/decision/records/r.md#A1\n\n| a | b |\n|---|---|\n| 1 | 2 |\n\n## Examples\n\n```gherkin\n@id=EX-001 @about=REQ-001 @source=docs/decision/records/r.md#A1\nScenario: 例\n  # 注釈\n  Given 何か\n\n  Then 結果\n```\n",
    )
    .replace(
        "範囲。\n",
        "範囲。\n- deferred: docs/decision/records/r.md#A1\n- deferred: docs/decision/records/r.md#A1, docs/decision/records/r.md#A1\n",
    );
    write(tmp.path(), "docs/ir/a.md", &doc);
    assert_eq!(listed_fingerprint(tmp.path(), "REQ-001"), "470a8ffe");
    assert_eq!(listed_fingerprint(tmp.path(), "TBL-001"), "39edaaf8");
    assert_eq!(listed_fingerprint(tmp.path(), "EX-001"), "ec19e8a0");
}

// @kotowari[REQ-core-203, REQ-core-208]
#[test]
fn req_203_a_requirement_level_declaration_is_part_of_the_body_it_fingerprints() {
    // "- deferred:" の行は "- source:" の行と違って除かない（sha256sum で計算した値）
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), "");
    let deferred = REQ_001.replace(
        "- verification: unit\n",
        "- verification: unit\n- deferred: docs/decision/records/r.md#A1\n",
    );
    write(tmp.path(), "docs/ir/a.md", &ir_doc(&deferred, ""));
    assert_eq!(listed_fingerprint(tmp.path(), "REQ-001"), "e29882df");
}

// --- REQ-core-198、REQ-core-199、REQ-core-206: ガイドのファイルを集める ---

/// 空の置き場だけを作り、設定を `config` にする（`IR`も`判断の記録`も1つも無い）
fn make_empty_project(tmp: &Path, config: &str) {
    for dir in [
        ".kotowari",
        "docs/ir",
        "docs/decision/records",
        "docs/decision/adr",
    ] {
        std::fs::create_dir_all(tmp.join(dir)).unwrap();
    }
    write(tmp, ".kotowari/config.yaml", config);
}

/// EX-core-368 の設定。"docs/guide.md" は両方の glob に当たる
const OVERLAPPING: &str =
    "guides:\n  files:\n    - \"docs/**/*.md\"\ntests:\n  files:\n    - \"**/*\"\n";

// @kotowari[REQ-core-199, TBL-core-001, TBL-core-020, EX-core-368]
#[test]
fn ex_368_check_stops_when_a_guide_is_also_a_test_file() {
    let tmp = TempDir::new().unwrap();
    make_empty_project(tmp.path(), OVERLAPPING);
    write(tmp.path(), "docs/guide.md", "# ガイド\n");
    let (code, stdout, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(2), "{stdout}");
    assert_eq!(stdout, "");
    assert!(stderr.starts_with("config error: "), "{stderr}");
    assert!(
        stderr.contains("docs/guide.md: matched by both guides.files and tests.files"),
        "{stderr}"
    );
}

// @kotowari[REQ-core-163, REQ-core-199, EX-core-368]
#[test]
fn ex_368_status_stops_when_a_guide_is_also_a_test_file() {
    let tmp = TempDir::new().unwrap();
    make_empty_project(tmp.path(), OVERLAPPING);
    write(tmp.path(), "docs/guide.md", "# ガイド\n");
    let (code, stdout, stderr) = run(tmp.path(), &["status"]);
    assert_eq!(code, Some(2), "{stdout}");
    assert_eq!(stdout, "");
    assert!(stderr.starts_with("config error: "), "{stderr}");
    assert!(
        stderr.contains("docs/guide.md: matched by both guides.files and tests.files"),
        "{stderr}"
    );
}

// @kotowari[REQ-core-199, TBL-core-020]
#[test]
fn req_199_only_the_first_overlapping_file_in_byte_order_is_reported() {
    let tmp = TempDir::new().unwrap();
    make_empty_project(
        tmp.path(),
        "guides:\n  files:\n    - \"notes/**/*.md\"\ntests:\n  files:\n    - \"notes/**/*.md\"\n",
    );
    write(tmp.path(), "notes/b.md", "b\n");
    write(tmp.path(), "notes/a.md", "a\n");
    let (code, _, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(2));
    assert!(stderr.contains("notes/a.md"), "{stderr}");
    assert!(!stderr.contains("notes/b.md"), "{stderr}");
}

// @kotowari[REQ-core-198, REQ-core-206, TBL-core-005, EX-core-369]
#[test]
fn ex_369_without_the_guides_key_no_guide_is_read() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), "");
    write(tmp.path(), "docs/ir/a.md", &ir_doc(REQ_001, ""));
    write(
        tmp.path(),
        "docs/guide.md",
        "# ガイド\n\n<!-- @kotowari[REQ-001:00000000] -->\n",
    );
    let (code, stdout, stderr) = run(tmp.path(), &["check", "--format", "json"]);
    assert_ne!(code, Some(2), "{stderr}");
    let v = json(&stdout);
    assert_eq!(v["guides"], serde_json::json!({"files": 0, "marks": 0}));
    assert!(!stdout.contains("guide_stale"), "{stdout}");
}

// @kotowari[REQ-core-198, TBL-core-001]
#[test]
fn req_198_a_guide_that_is_not_utf8_stops() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), GUIDES_MD);
    std::fs::create_dir_all(tmp.path().join("guides")).unwrap();
    std::fs::write(tmp.path().join("guides/a.md"), [0xff, 0xfe, 0x00]).unwrap();
    let (code, stdout, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(2), "{stdout}");
    assert_eq!(stderr.trim_end(), "non-UTF-8 file: guides/a.md");
}

// @kotowari[REQ-core-198, REQ-core-206]
#[test]
fn req_198_a_guides_glob_may_hit_the_ir_documents() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), "guides:\n  files:\n    - \"docs/ir/**/*.md\"\n");
    write(tmp.path(), "docs/ir/a.md", &ir_doc(REQ_001, ""));
    let (code, stdout, stderr) = run(tmp.path(), &["check", "--format", "json"]);
    assert_ne!(code, Some(2), "{stderr}");
    let v = json(&stdout);
    assert_eq!(v["files"], 1, "the IR document is still read as IR: {v}");
    assert_eq!(v["guides"]["files"], 1, "and also as a guide: {v}");
}

// @kotowari[REQ-core-152, REQ-core-158]
#[test]
fn req_152_list_and_query_do_not_read_guides_and_do_not_stop_on_their_overlap() {
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        "guides:\n  files:\n    - \"tests/**/*.rs\"\n    - \"guides/**/*.md\"\n",
    );
    write(tmp.path(), "docs/ir/a.md", &ir_doc(REQ_001, ""));
    write(
        tmp.path(),
        "tests/a.rs",
        "// @kotowari[REQ-001]\n#[test]\nfn req_001() {}\n",
    );
    // UTF-8 でないガイドも、list と query は読まないので止まらない
    std::fs::create_dir_all(tmp.path().join("guides")).unwrap();
    std::fs::write(tmp.path().join("guides/a.md"), [0xff, 0xfe, 0x00]).unwrap();
    for args in [&["list"][..], &["query", "REQ-001"][..]] {
        let (code, _, stderr) = run(tmp.path(), args);
        assert_eq!(code, Some(0), "{args:?}: {stderr}");
    }
}

// --- REQ-core-200、REQ-core-201、REQ-core-202、TBL-core-036: ガイドの印を読む ---

/// EX-core-362 の`IR`と、"REQ-001" に印を付けたテストと、`ガイド`の "guides/a.md" を作る。
/// `ガイド`を除けば`指摘`は出ない
fn write_ex_362_project(tmp: &Path, guide: &str) {
    make_project(tmp, GUIDES_MD);
    write(tmp, "docs/ir/a.md", &ir_doc(REQ_001, ""));
    write(
        tmp,
        "tests/a.rs",
        "// @kotowari[REQ-001]\n#[test]\nfn req_001() {}\n",
    );
    write(tmp, "guides/a.md", guide);
}

/// "kotowari check --format json" を走らせて (終了コード, JSON) を返す
fn check_json(tmp: &Path) -> (Option<i32>, serde_json::Value) {
    let (code, stdout, stderr) = run(tmp, &["check", "--format", "json"]);
    assert_ne!(code, Some(2), "{stderr}");
    (code, json(&stdout))
}

/// その path への`指摘`の (kind, line, detail)
fn findings_on(v: &serde_json::Value, path: &str) -> Vec<(String, u64, String)> {
    v["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["path"] == path)
        .map(|f| {
            (
                f["kind"].as_str().unwrap().to_string(),
                f["line"].as_u64().unwrap(),
                f["detail"].as_str().unwrap().to_string(),
            )
        })
        .collect()
}

// @kotowari[REQ-core-198, REQ-core-200, REQ-core-206]
#[test]
fn req_200_the_ex_362_project_without_a_guide_mark_has_no_findings() {
    let tmp = TempDir::new().unwrap();
    write_ex_362_project(tmp.path(), "# ガイド\n");
    let (code, v) = check_json(tmp.path());
    assert_eq!(code, Some(0), "{v}");
    assert_eq!(v["findings"], serde_json::json!([]));
    assert_eq!(v["guides"], serde_json::json!({"files": 1, "marks": 0}));
}

// @kotowari[REQ-core-200, EX-core-366]
#[test]
fn ex_366_marks_outside_comments_and_inside_code_are_not_read() {
    let tmp = TempDir::new().unwrap();
    write_ex_362_project(
        tmp.path(),
        concat!(
            "# ガイド\n\n",
            "```\n<!-- @kotowari[REQ-001] -->\n```\n\n",
            "@kotowari[REQ-001] の形で書く\n\n",
            // 字下げの形のコードブロックとコードスパンも読まない
            "    <!-- @kotowari[REQ-001] -->\n\n",
            "本文の `<!-- @kotowari[REQ-001] -->` は例\n",
        ),
    );
    let (_, v) = check_json(tmp.path());
    assert_eq!(findings_on(&v, "guides/a.md"), vec![], "{v}");
    assert_eq!(v["guides"]["marks"], 0);
}

// @kotowari[REQ-core-201, REQ-core-202, TBL-core-036, TBL-core-008, TBL-core-019, EX-core-367]
#[test]
fn ex_367_a_mark_without_fingerprint_and_an_uppercase_fingerprint_are_invalid() {
    let tmp = TempDir::new().unwrap();
    write_ex_362_project(
        tmp.path(),
        "# ガイド\n\n<!-- @kotowari[REQ-001] -->\n\n<!-- @kotowari[REQ-001:8C0D7663] -->\n",
    );
    let (code, stdout, _) = run(tmp.path(), &["check", "--format", "text"]);
    assert_eq!(code, Some(1), "{stdout}");
    assert_eq!(
        stdout,
        "guides/a.md:3 [error] invalid_marker <!-- @kotowari[REQ-001] -->\n\
guides/a.md:5 [error] invalid_marker <!-- @kotowari[REQ-001:8C0D7663] -->\n"
    );
}

// @kotowari[REQ-core-201, REQ-core-202, TBL-core-036, EX-core-373]
#[test]
fn ex_373_one_malformed_entry_leaves_the_whole_mark_unmatched() {
    let tmp = TempDir::new().unwrap();
    let line = "<!-- @kotowari[REQ-001:00000000, foo:51b1f3da] -->";
    write_ex_362_project(tmp.path(), &format!("# ガイド\n\n{line}\n"));
    let (_, v) = check_json(tmp.path());
    assert_eq!(
        findings_on(&v, "guides/a.md"),
        vec![("invalid_marker".to_string(), 3, line.to_string())]
    );
    assert_eq!(v["guides"]["marks"], 0);
}

// @kotowari[REQ-core-200, REQ-core-201, TBL-core-036, EX-core-374]
#[test]
fn ex_374_spaces_inside_the_brackets_are_allowed_and_text_after_the_comment_is_not_read() {
    let tmp = TempDir::new().unwrap();
    write_ex_362_project(
        tmp.path(),
        "# ガイド\n\n<!-- @kotowari[ REQ-001 : 51b1f3da ] -->\n\n<!-- a --> @kotowari[REQ-001:00000000]\n",
    );
    let (_, v) = check_json(tmp.path());
    assert_eq!(findings_on(&v, "guides/a.md"), vec![], "{v}");
    assert_eq!(v["guides"]["marks"], 1);
}

// @kotowari[REQ-core-200, REQ-core-206]
#[test]
fn req_200_inline_comments_and_several_marks_in_one_comment_are_all_read() {
    let tmp = TempDir::new().unwrap();
    write_ex_362_project(
        tmp.path(),
        concat!(
            "# ガイド\n\n",
            "本文 <!-- @kotowari[REQ-001:51b1f3da] --> の続き\n\n",
            "<!-- @kotowari[REQ-001:51b1f3da] @kotowari[REQ-001:51b1f3da] -->\n\n",
            "<!--\n  @kotowari[REQ-001:51b1f3da]\n-->\n",
        ),
    );
    let (_, v) = check_json(tmp.path());
    assert_eq!(findings_on(&v, "guides/a.md"), vec![], "{v}");
    assert_eq!(v["guides"]["marks"], 4);
}

// @kotowari[REQ-core-200, REQ-core-204]
#[test]
fn req_200_each_comment_of_one_html_block_is_read_and_the_text_between_them_is_not() {
    let tmp = TempDir::new().unwrap();
    write_ex_362_project(
        tmp.path(),
        concat!(
            "# ガイド\n\n",
            "<!-- @kotowari[REQ-001:51b1f3da] --> @kotowari[REQ-001:00000000] ",
            "<!-- @kotowari[REQ-001:11111111] -->\n\n",
            "<!-- a <!--> @kotowari[REQ-001:00000000] -->\n",
        ),
    );
    let (_, v) = check_json(tmp.path());
    assert_eq!(
        findings_on(&v, "guides/a.md"),
        vec![(
            "guide_stale".to_string(),
            3,
            "REQ-001 11111111 51b1f3da".to_string()
        )],
        "{v}"
    );
    assert_eq!(v["guides"]["marks"], 2);
}

// @kotowari[REQ-core-202, TBL-core-008]
#[test]
fn req_202_an_empty_mark_and_a_mark_without_its_closing_bracket_are_invalid() {
    let tmp = TempDir::new().unwrap();
    write_ex_362_project(
        tmp.path(),
        "# ガイド\n\n<!-- @kotowari[ , ] -->\n\n<!--\n  @kotowari[REQ-001:51b1f3da\n  ]\n-->\n",
    );
    let (_, v) = check_json(tmp.path());
    assert_eq!(
        findings_on(&v, "guides/a.md"),
        vec![
            (
                "invalid_marker".to_string(),
                3,
                "<!-- @kotowari[ , ] -->".to_string()
            ),
            (
                "invalid_marker".to_string(),
                6,
                "  @kotowari[REQ-001:51b1f3da".to_string()
            ),
        ]
    );
    assert_eq!(v["guides"]["marks"], 0);
}

// --- REQ-core-204、REQ-core-031、REQ-core-162: 指紋の照合と guide_stale ---

/// EX-core-362 の`ガイドの印`
const MARK_362: &str = "# ガイド\n\n<!-- @kotowari[REQ-001:51b1f3da] -->\n";

/// guide_stale の`指摘`の (line, detail)
fn stale_on(v: &serde_json::Value) -> Vec<(u64, String)> {
    findings_on(v, "guides/a.md")
        .into_iter()
        .filter(|(kind, _, _)| kind == "guide_stale")
        .map(|(_, line, detail)| (line, detail))
        .collect()
}

// @kotowari[REQ-core-198, REQ-core-200, REQ-core-203, REQ-core-204, REQ-core-206, EX-core-362]
#[test]
fn ex_362_a_mark_with_the_current_fingerprint_yields_nothing() {
    let tmp = TempDir::new().unwrap();
    write_ex_362_project(tmp.path(), MARK_362);
    let (code, v) = check_json(tmp.path());
    assert_eq!(code, Some(0), "{v}");
    assert_eq!(v["findings"], serde_json::json!([]));
    assert_eq!(v["guides"], serde_json::json!({"files": 1, "marks": 1}));
}

// @kotowari[REQ-core-204, REQ-core-031, TBL-core-009, TBL-core-019, EX-core-363]
#[test]
fn ex_363_a_changed_body_makes_the_mark_a_stale_notice() {
    let tmp = TempDir::new().unwrap();
    write_ex_362_project(tmp.path(), MARK_362);
    write(
        tmp.path(),
        "docs/ir/a.md",
        &ir_doc(&REQ_001.replace("文。", "別の文。"), ""),
    );
    let (code, stdout, _) = run(tmp.path(), &["check", "--format", "text"]);
    assert_eq!(
        code,
        Some(0),
        "a notice does not change the exit code: {stdout}"
    );
    assert_eq!(
        stdout,
        "guides/a.md:3 [notice] guide_stale REQ-001 51b1f3da e4f95a33\n"
    );
    let (code, stdout, _) = run(tmp.path(), &["status"]);
    assert_eq!(code, Some(0), "{stdout}");
    let v = json(&stdout);
    assert_eq!(v["complete"], true);
    assert_eq!(v["findings"]["notice"], 1);
}

// @kotowari[REQ-core-203, REQ-core-204, EX-core-364]
#[test]
fn ex_364_renaming_the_heading_and_adding_a_source_keeps_the_mark_fresh() {
    let tmp = TempDir::new().unwrap();
    write_ex_362_project(tmp.path(), MARK_362);
    let renamed = REQ_001
        .replace("名前", "別の名前")
        .replace("r.md#A1\n", "r.md#A1, docs/decision/records/r.md#A1\n");
    write(tmp.path(), "docs/ir/a.md", &ir_doc(&renamed, ""));
    let (_, v) = check_json(tmp.path());
    assert_eq!(stale_on(&v), vec![], "{v}");
    assert_eq!(v["guides"]["marks"], 1);
}

// @kotowari[REQ-core-204, EX-core-365, TBL-core-006]
#[test]
fn ex_365_a_mark_on_an_id_missing_from_the_ir_is_a_stale_notice() {
    let tmp = TempDir::new().unwrap();
    write_ex_362_project(
        tmp.path(),
        "# ガイド\n\n<!-- @kotowari[REQ-009:51b1f3da] -->\n",
    );
    let (_, v) = check_json(tmp.path());
    assert_eq!(
        findings_on(&v, "guides/a.md"),
        vec![(
            "guide_stale".to_string(),
            3,
            "REQ-009 51b1f3da -".to_string()
        )],
        "no unresolved_reference: {v}"
    );
}

// @kotowari[REQ-core-204, EX-core-370]
#[test]
fn ex_370_matching_any_item_of_a_duplicated_id_keeps_the_mark_fresh() {
    let tmp = TempDir::new().unwrap();
    write_ex_362_project(tmp.path(), MARK_362);
    write(
        tmp.path(),
        "docs/ir/a.md",
        &ir_doc(&REQ_001.replace("文。", "別の文。"), ""),
    );
    write(tmp.path(), "docs/ir/b.md", &ir_doc(REQ_001, ""));
    let (_, v) = check_json(tmp.path());
    assert!(
        v["findings"]
            .as_array()
            .unwrap()
            .iter()
            .any(|f| f["kind"] == "duplicate_id"),
        "{v}"
    );
    assert_eq!(stale_on(&v), vec![], "{v}");
    assert_eq!(v["guides"]["marks"], 1);
}

// @kotowari[REQ-core-204, REQ-core-032]
#[test]
fn req_204_a_duplicated_id_matching_neither_item_reports_the_first_fingerprint() {
    let tmp = TempDir::new().unwrap();
    write_ex_362_project(
        tmp.path(),
        "# ガイド\n\n<!-- @kotowari[REQ-001:00000000] -->\n",
    );
    // 1つ目（パスのバイト順で先の docs/ir/a.md）の指紋は "e4f95a33"、2つ目は "51b1f3da"
    write(
        tmp.path(),
        "docs/ir/a.md",
        &ir_doc(&REQ_001.replace("文。", "別の文。"), ""),
    );
    write(tmp.path(), "docs/ir/b.md", &ir_doc(REQ_001, ""));
    let (_, v) = check_json(tmp.path());
    assert_eq!(
        stale_on(&v),
        vec![(3, "REQ-001 00000000 e4f95a33".to_string())]
    );
}

/// "EX-001"（`指紋`は "ec19e8a0"）を足した EX-core-362 の`IR`と、両方に印を付けたテスト
fn write_ex_001_project(tmp: &Path, tags: &str, name: &str, guide: &str) {
    write_ex_362_project(tmp, guide);
    write(tmp, "docs/ir/a.md", &scenario_doc(tags, name));
    write(
        tmp,
        "tests/a.rs",
        "// @kotowari[REQ-001, EX-001]\n#[test]\nfn req_001() {}\n",
    );
}

const EX_001_TAGS: &str = "@id=EX-001 @about=REQ-001 @source=docs/decision/records/r.md#A1";

// @kotowari[REQ-core-200, REQ-core-201, REQ-core-204, REQ-core-206, TBL-core-036, EX-core-371]
#[test]
fn ex_371_two_entries_in_one_mark_are_counted_and_matched_one_by_one() {
    let tmp = TempDir::new().unwrap();
    write_ex_001_project(
        tmp.path(),
        EX_001_TAGS,
        "例",
        "# ガイド\n\n<!-- @kotowari[REQ-001:51b1f3da, EX-001:51b1f3da] -->\n",
    );
    let (code, v) = check_json(tmp.path());
    assert_eq!(code, Some(0), "{v}");
    assert_eq!(v["guides"], serde_json::json!({"files": 1, "marks": 2}));
    assert_eq!(
        stale_on(&v),
        vec![(3, "EX-001 51b1f3da ec19e8a0".to_string())]
    );
}

// @kotowari[REQ-core-203, REQ-core-204, EX-core-375]
#[test]
fn ex_375_renaming_a_scenario_and_changing_its_source_tag_keeps_the_mark_fresh() {
    let tmp = TempDir::new().unwrap();
    write_ex_001_project(
        tmp.path(),
        "@id=EX-001 @about=REQ-001 @source=docs/decision/records/r.md#A1,docs/decision/records/r.md#A1",
        "別の名前",
        "# ガイド\n\n<!-- @kotowari[EX-001:ec19e8a0] -->\n",
    );
    let (_, v) = check_json(tmp.path());
    assert_eq!(stale_on(&v), vec![], "{v}");
    assert_eq!(v["guides"]["marks"], 1);
}

// @kotowari[REQ-core-204, TBL-core-019]
#[test]
fn req_204_the_line_of_guide_stale_is_where_the_mark_starts() {
    let tmp = TempDir::new().unwrap();
    write_ex_362_project(
        tmp.path(),
        "# ガイド\n\n<!--\n説明\n  @kotowari[REQ-009:51b1f3da]\n-->\n",
    );
    let (_, v) = check_json(tmp.path());
    assert_eq!(stale_on(&v), vec![(5, "REQ-009 51b1f3da -".to_string())]);
}

// @kotowari[REQ-core-162, TBL-core-028]
#[test]
fn req_162_status_carries_the_guides_group() {
    let tmp = TempDir::new().unwrap();
    write_ex_362_project(tmp.path(), MARK_362);
    let (code, stdout, stderr) = run(tmp.path(), &["status"]);
    assert_eq!(code, Some(0), "{stderr}");
    let v = json(&stdout);
    assert_eq!(v["guides"], serde_json::json!({"files": 1, "marks": 1}));
    let (_, text, _) = run(tmp.path(), &["status", "--format", "text"]);
    let lines: Vec<&str> = text.lines().collect();
    let tests = lines.iter().position(|l| l.starts_with("tests ")).unwrap();
    assert_eq!(lines[tests + 1], "guides files=1 marks=1", "{text}");
    assert!(lines[tests + 2].starts_with("surface "), "{text}");
}
