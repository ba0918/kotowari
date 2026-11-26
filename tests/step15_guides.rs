//! ガイドの印と、その設定の鍵 "guides.files"
#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]

use std::path::Path;
use tempfile::TempDir;

/// 置き場と設定を作る。"guides.files" は `guides` の行をそのまま設定に足す
fn make_project(tmp: &Path, guides: &str) {
    for dir in [".kotowari", "docs/decisions"] {
        std::fs::create_dir_all(tmp.join(dir)).unwrap();
    }
    std::fs::write(
        tmp.join(".kotowari/config.yaml"),
        format!(
            "decisions:\n  documents: docs/decisions\ntests:\n  files:\n    - \"tests/**/*.rs\"\n{guides}"
        ),
    )
    .unwrap();
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

// @kotowari[C-core-guides-9, C-core-libapi-11]
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

/// 決定の文書 "docs/decisions/a/a.md" の決定 "名前" を指す ID
const D_ID: &str = "docs/decisions/a/a.md#名前";

/// 決定 "名前" を1つ持つ決定の文書。決定の`指紋`は "1423c6c3"
const D_001: &str = "# 題名\n\n範囲。\n\n## 名前\n\n状態: 実装済み\n\n### 問題\n\n困っていた。\n\n\
### 決めたこと\n\n文。\n\n### 確認すること\n\n- C-a-a-1 確かめる\n";

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

// --- "guides.files" の鍵 ---

// @kotowari[C-core-config-16]
#[test]
fn req_014_an_unreadable_guides_glob_is_a_config_error() {
    let tmp = tempfile::TempDir::new().unwrap();
    for dir in [".kotowari", "docs/decisions"] {
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

// --- 指紋 ---

// @kotowari[C-core-guides-4]
#[test]
fn req_203_crlf_line_endings_do_not_change_the_fingerprint() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), "");
    write(
        tmp.path(),
        "docs/decisions/a/a.md",
        &D_001.replace('\n', "\r\n"),
    );
    assert_eq!(listed_fingerprint(tmp.path(), D_ID), "1423c6c3");
}

// --- ガイドのファイルを集める ---

/// 空の置き場だけを作り、設定を `config` にする（決定の文書は1つも無い）
fn make_empty_project(tmp: &Path, config: &str) {
    for dir in [".kotowari", "docs/decisions"] {
        std::fs::create_dir_all(tmp.join(dir)).unwrap();
    }
    write(tmp, ".kotowari/config.yaml", config);
}

/// "docs/guide.md" は両方の glob に当たる
const OVERLAPPING: &str =
    "guides:\n  files:\n    - \"docs/**/*.md\"\ntests:\n  files:\n    - \"**/*\"\n";

// @kotowari[C-core-clienv-7, C-core-guides-9]
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

// @kotowari[C-core-guides-9, C-core-status-4]
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

// @kotowari[C-core-clienv-7, C-core-guides-10]
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

// @kotowari[C-core-guides-5, C-core-output-5]
#[test]
fn ex_369_without_the_guides_key_no_guide_is_read() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), "");
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

// @kotowari[C-core-clienv-7, C-core-guides-6]
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

// @kotowari[C-core-guides-7, C-core-output-5]
#[test]
fn req_198_a_guides_glob_may_hit_the_ir_documents() {
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        "guides:\n  files:\n    - \"docs/decisions/**/*.md\"\n",
    );
    write(tmp.path(), "docs/decisions/a/a.md", D_001);
    let (code, stdout, stderr) = run(tmp.path(), &["check", "--format", "json"]);
    assert_ne!(code, Some(2), "{stderr}");
    let v = json(&stdout);
    assert_eq!(
        v["decisions"]["documents"], 1,
        "the decision document is still read as one: {v}"
    );
    assert_eq!(v["guides"]["files"], 1, "and also as a guide: {v}");
}

// @kotowari[C-core-guides-8, C-core-list-15, C-core-query-9]
#[test]
fn req_152_list_and_query_do_not_read_guides_and_do_not_stop_on_their_overlap() {
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        "guides:\n  files:\n    - \"tests/**/*.rs\"\n    - \"guides/**/*.md\"\n",
    );
    write(tmp.path(), "docs/decisions/a/a.md", D_001);
    write(
        tmp.path(),
        "tests/a.rs",
        "// @kotowari[C-a-a-1]\n#[test]\nfn req_001() {}\n",
    );
    // UTF-8 でないガイドも、list と query は読まないので止まらない
    std::fs::create_dir_all(tmp.path().join("guides")).unwrap();
    std::fs::write(tmp.path().join("guides/a.md"), [0xff, 0xfe, 0x00]).unwrap();
    for args in [&["list"][..], &["query", D_ID][..]] {
        let (code, _, stderr) = run(tmp.path(), args);
        assert_eq!(code, Some(0), "{args:?}: {stderr}");
    }
}

// --- ガイドの印を読む ---

/// 古い形の文書と、それに印を付けたテストと、`ガイド`の "guides/a.md" を作る。
/// `ガイド`を除けば`指摘`は出ない
fn write_ex_362_project(tmp: &Path, guide: &str) {
    make_project(tmp, GUIDES_MD);
    write(
        tmp,
        "tests/a.rs",
        "// @kotowari[REQ-001]\n#[test]\nfn req_001() {}\n",
    );
    write(tmp, "guides/a.md", guide);
}

/// 決定の文書 D_001 と、その確認することの行に印を付けたテストと、`ガイド`の "guides/a.md" を作る。
/// `ガイド`を除けば`指摘`は出ない
fn write_decision_project(tmp: &Path, guide: &str) {
    make_project(tmp, GUIDES_MD);
    write(tmp, "docs/decisions/a/a.md", D_001);
    write(
        tmp,
        "tests/a.rs",
        "// @kotowari[C-a-a-1]\n#[test]\nfn c_a_a_1() {}\n",
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

// @kotowari[C-core-output-5]
#[test]
fn req_200_the_ex_362_project_without_a_guide_mark_has_no_findings() {
    let tmp = TempDir::new().unwrap();
    write_decision_project(tmp.path(), "# ガイド\n");
    let (code, v) = check_json(tmp.path());
    assert_eq!(code, Some(0), "{v}");
    assert_eq!(v["findings"], serde_json::json!([]));
    assert_eq!(v["guides"], serde_json::json!({"files": 1, "marks": 0}));
}

// @kotowari[C-core-guides-12]
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

// @kotowari[C-core-findings-3, C-core-guides-15]
#[test]
fn ex_367_a_mark_without_fingerprint_and_an_uppercase_fingerprint_are_invalid() {
    let tmp = TempDir::new().unwrap();
    write_decision_project(
        tmp.path(),
        "# ガイド\n\n<!-- @kotowari[C-a-a-1] -->\n\n<!-- @kotowari[docs/decisions/a/a.md#名前:8C0D7663] -->\n",
    );
    let (code, stdout, _) = run(tmp.path(), &["check", "--format", "text"]);
    assert_eq!(code, Some(1), "{stdout}");
    assert_eq!(
        stdout,
        "guides/a.md:3 [error] invalid_marker <!-- @kotowari[C-a-a-1] -->\n\
guides/a.md:5 [error] invalid_marker <!-- @kotowari[docs/decisions/a/a.md#名前:8C0D7663] -->\n"
    );
}

// @kotowari[C-core-guides-15]
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

// @kotowari[C-core-guides-13, C-core-guides-14]
#[test]
fn ex_374_spaces_inside_the_brackets_are_allowed_and_text_after_the_comment_is_not_read() {
    let tmp = TempDir::new().unwrap();
    write_decision_project(
        tmp.path(),
        "# ガイド\n\n<!-- @kotowari[ docs/decisions/a/a.md#名前 : 1423c6c3 ] -->\n\n<!-- a --> @kotowari[docs/decisions/a/a.md#名前:00000000]\n",
    );
    let (_, v) = check_json(tmp.path());
    assert_eq!(findings_on(&v, "guides/a.md"), vec![], "{v}");
    assert_eq!(v["guides"]["marks"], 1);
}

// @kotowari[C-core-guides-11, C-core-output-5]
#[test]
fn req_200_inline_comments_and_several_marks_in_one_comment_are_all_read() {
    let tmp = TempDir::new().unwrap();
    write_decision_project(
        tmp.path(),
        concat!(
            "# ガイド\n\n",
            "本文 <!-- @kotowari[docs/decisions/a/a.md#名前:1423c6c3] --> の続き\n\n",
            "<!-- @kotowari[docs/decisions/a/a.md#名前:1423c6c3] @kotowari[docs/decisions/a/a.md#名前:1423c6c3] -->\n\n",
            "<!--\n  @kotowari[docs/decisions/a/a.md#名前:1423c6c3]\n-->\n",
        ),
    );
    let (_, v) = check_json(tmp.path());
    assert_eq!(findings_on(&v, "guides/a.md"), vec![], "{v}");
    assert_eq!(v["guides"]["marks"], 4);
}

// @kotowari[C-core-guides-13]
#[test]
fn req_200_each_comment_of_one_html_block_is_read_and_the_text_between_them_is_not() {
    let tmp = TempDir::new().unwrap();
    write_decision_project(
        tmp.path(),
        concat!(
            "# ガイド\n\n",
            "<!-- @kotowari[docs/decisions/a/a.md#名前:1423c6c3] --> @kotowari[docs/decisions/a/a.md#名前:00000000] ",
            "<!-- @kotowari[docs/decisions/a/a.md#名前:11111111] -->\n\n",
            "<!-- a <!--> @kotowari[docs/decisions/a/a.md#名前:00000000] -->\n",
        ),
    );
    let (_, v) = check_json(tmp.path());
    assert_eq!(
        findings_on(&v, "guides/a.md"),
        vec![(
            "guide_stale".to_string(),
            3,
            "docs/decisions/a/a.md#名前 11111111 1423c6c3".to_string()
        )],
        "{v}"
    );
    assert_eq!(v["guides"]["marks"], 2);
}

// @kotowari[C-core-findings-3, C-core-guides-16]
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

// --- 指紋の照合と guide_stale ---

/// `ガイドの印`
/// 決定 "名前" に今の`指紋`で付けた印の`ガイド`
const MARK_D: &str = "# ガイド\n\n<!-- @kotowari[docs/decisions/a/a.md#名前:1423c6c3] -->\n";

/// guide_stale の`指摘`の (line, detail)
fn stale_on(v: &serde_json::Value) -> Vec<(u64, String)> {
    findings_on(v, "guides/a.md")
        .into_iter()
        .filter(|(kind, _, _)| kind == "guide_stale")
        .map(|(_, line, detail)| (line, detail))
        .collect()
}

// @kotowari[C-core-output-5]
#[test]
fn ex_362_a_mark_with_the_current_fingerprint_yields_nothing() {
    let tmp = TempDir::new().unwrap();
    write_decision_project(tmp.path(), MARK_D);
    let (code, v) = check_json(tmp.path());
    assert_eq!(code, Some(0), "{v}");
    assert_eq!(v["findings"], serde_json::json!([]));
    assert_eq!(v["guides"], serde_json::json!({"files": 1, "marks": 1}));
}

// @kotowari[C-core-findings-1, C-core-findings-2, C-core-findings-7, C-core-guides-18, C-core-guides-20]
#[test]
fn ex_363_a_changed_body_makes_the_mark_a_stale_notice() {
    let tmp = TempDir::new().unwrap();
    write_decision_project(tmp.path(), MARK_D);
    write(
        tmp.path(),
        "docs/decisions/a/a.md",
        &D_001.replace("文。", "別の文。"),
    );
    let (code, stdout, _) = run(tmp.path(), &["check", "--format", "text"]);
    assert_eq!(
        code,
        Some(0),
        "a notice does not change the exit code: {stdout}"
    );
    assert_eq!(
        stdout,
        "guides/a.md:3 [notice] guide_stale docs/decisions/a/a.md#名前 1423c6c3 0ea36937\n"
    );
    let (code, stdout, _) = run(tmp.path(), &["status"]);
    assert_eq!(code, Some(0), "{stdout}");
    let v = json(&stdout);
    assert_eq!(v["complete"], true);
    assert_eq!(v["findings"]["notice"], 1);
}

// @kotowari[C-core-guides-17, C-core-output-5]
#[test]
fn ex_371_two_entries_in_one_mark_are_counted_and_matched_one_by_one() {
    let tmp = TempDir::new().unwrap();
    write_decision_project(
        tmp.path(),
        "# ガイド\n\n<!-- @kotowari[docs/decisions/a/a.md#名前:00000000, docs/decisions/a/a.md#名前:1423c6c3] -->\n",
    );
    let (code, v) = check_json(tmp.path());
    assert_eq!(code, Some(0), "{v}");
    assert_eq!(v["guides"], serde_json::json!({"files": 1, "marks": 2}));
    assert_eq!(
        stale_on(&v),
        vec![(
            3,
            "docs/decisions/a/a.md#名前 00000000 1423c6c3".to_string()
        )]
    );
}

// @kotowari[C-core-guides-18]
#[test]
fn req_204_the_line_of_guide_stale_is_where_the_mark_starts() {
    let tmp = TempDir::new().unwrap();
    write_decision_project(
        tmp.path(),
        "# ガイド\n\n<!--\n説明\n  @kotowari[docs/decisions/a/a.md#名前:00000000]\n-->\n",
    );
    let (_, v) = check_json(tmp.path());
    assert_eq!(
        stale_on(&v),
        vec![(
            5,
            "docs/decisions/a/a.md#名前 00000000 1423c6c3".to_string()
        )]
    );
}

// @kotowari[C-core-status-7, C-core-status-11]
#[test]
fn req_162_status_carries_the_guides_group() {
    let tmp = TempDir::new().unwrap();
    write_decision_project(tmp.path(), MARK_D);
    let (code, stdout, stderr) = run(tmp.path(), &["status"]);
    assert_eq!(code, Some(0), "{stderr}");
    let v = json(&stdout);
    assert_eq!(v["guides"], serde_json::json!({"files": 1, "marks": 1}));
    let (_, text, _) = run(tmp.path(), &["status", "--format", "text"]);
    let lines: Vec<&str> = text.lines().collect();
    let tests = lines.iter().position(|l| l.starts_with("tests ")).unwrap();
    assert_eq!(lines[tests + 1], "guides files=1 marks=1", "{text}");
    assert!(lines[tests + 2].starts_with("overview "), "{text}");
    assert!(lines[tests + 3].starts_with("surface "), "{text}");
}
