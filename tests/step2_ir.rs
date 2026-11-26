#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]
// --- Step 3: 文書の読み込み ---

// @kotowari[C-core-document-2]
#[test]
fn uppercase_md_is_not_read() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/decisions")).unwrap();
    std::fs::write(tmp.path().join(".kotowari/config.yaml"), "").unwrap();
    // .MD ファイルは読まない
    std::fs::write(
        tmp.path().join("docs/decisions/README.MD"),
        "# Title\n\nScope.\n",
    )
    .unwrap();
    let output = assert_cmd::Command::cargo_bin("kotowari")
        .unwrap()
        .arg("check")
        .current_dir(tmp.path())
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    assert_eq!(
        v["decisions"]["documents"], 0,
        ".MD file should not be read"
    );
}

// @kotowari[C-core-document-4]
#[test]
fn file_symlink_is_read() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/decisions")).unwrap();
    std::fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "decisions:\n  documents: docs/decisions\n",
    )
    .unwrap();
    // 実体を別の場所に作り、シンボリックリンクを決定の文書の置き場に置く
    let target = tmp.path().join("target.md");
    std::fs::write(&target, "# Title\n\nScope.\n").unwrap();
    std::os::unix::fs::symlink(&target, tmp.path().join("docs/decisions/link.md")).unwrap();
    let output = assert_cmd::Command::cargo_bin("kotowari")
        .unwrap()
        .arg("check")
        .current_dir(tmp.path())
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    assert_eq!(
        v["decisions"]["documents"], 1,
        "symlinked file should be read"
    );
}

// --- Step 4b: gherkin の行の形と ID の定義 ---

// @kotowari[C-core-document-4]
#[test]
#[cfg(unix)]
fn broken_symlink_in_the_decision_document_dir_stops() {
    use std::os::unix::fs::symlink;
    let tmp = tempfile::TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/decisions")).unwrap();
    std::fs::write(tmp.path().join(".kotowari/config.yaml"), "").unwrap();
    symlink(
        tmp.path().join("nowhere.md"),
        tmp.path().join("docs/decisions/broken.md"),
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
        "a broken symlink in the place of decision documents must stop: {:?}",
        output
    );
}

// --- 見ないもの ---

/// CLI を通して検査するプロジェクトを一時ディレクトリに作る
fn make_cli_project(tmp: &std::path::Path) {
    for dir in [".kotowari", "docs/decisions"] {
        std::fs::create_dir_all(tmp.join(dir)).unwrap();
    }
    std::fs::write(
        tmp.join(".kotowari/config.yaml"),
        "decisions:\n  documents: docs/decisions\n",
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

// --- ir-document.md の具体例 ---

// @kotowari[C-core-document-1]
#[test]
fn a_document_deep_in_the_tree_is_read_and_an_empty_directory_is_silent() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    make_cli_project(tmp.path());
    std::fs::create_dir_all(tmp.path().join("docs/decisions/network/dns")).unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/decisions/network/empty")).unwrap();
    std::fs::write(
        tmp.path().join("docs/decisions/network/dns/timeout.md"),
        "# Title\n\nScope.\n\n## Decision\n\nStatus: implemented\n\n### Problem\n\nTrouble.\n\n### Decision\n\nDo it.\n\n### Confirmations\n\n- C-network-timeout-1 Checked\n",
    )
    .unwrap();
    let v = run_cli(tmp.path());
    let missing = find_kind_in_json(&v, "confirmation_without_test");
    assert_eq!(missing.len(), 1, "the deep document is read: {v}");
    assert_eq!(missing[0]["path"], "docs/decisions/network/dns/timeout.md");
    assert!(
        v["findings"].as_array().unwrap().iter().all(|f| {
            !f["path"]
                .as_str()
                .unwrap()
                .starts_with("docs/decisions/network/empty")
        }),
        "an empty directory gets no finding: {v}"
    );
}

// @kotowari[C-core-document-3]
#[test]
#[cfg(unix)]
fn hidden_directory_and_directory_symlink_deep_in_the_tree_are_not_read() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    make_cli_project(tmp.path());
    std::fs::create_dir_all(tmp.path().join("docs/decisions/network/.draft")).unwrap();
    // 読まれれば題名も範囲も無い文書として指摘が出る中身
    std::fs::write(tmp.path().join("docs/decisions/network/.draft/a.md"), "bad").unwrap();
    std::os::unix::fs::symlink(
        tmp.path().join("docs/decisions"),
        tmp.path().join("docs/decisions/network/link"),
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
    assert_eq!(
        v["decisions"]["documents"], 0,
        "no document under either place is read: {v}"
    );
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

// --- findings.md と ir-items.md の具体例 ---

// @kotowari[C-core-findings-8]
#[test]
fn three_places_yield_two_duplicates_and_none_on_the_first() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    make_cli_project(tmp.path());
    std::fs::write(
        tmp.path().join("docs/decisions/a.md"),
        "# Title\n\nScope.\n\n## Decision\n\nStatus: implemented\n\n### Problem\n\nTrouble.\n\n\
### Decision\n\nDo it.\n\n### Confirmations\n\n- C-t-1 One\n- C-t-1 Two\n- C-t-1 Three\n",
    )
    .unwrap();
    let v = run_cli(tmp.path());
    let duplicates = find_kind_in_json(&v, "duplicate_id");
    assert_eq!(
        duplicates.len(),
        2,
        "three places yield two duplicates: {v}"
    );
    assert!(duplicates.iter().all(|f| f["detail"] == "C-t-1"));
    // 1つ目の行は 19 行目
    assert!(
        duplicates.iter().all(|f| f["line"] != 19),
        "the first row gets no duplicate_id: {v}"
    );
}

// --- 見出しの下の行は1行ずつ文として読む ---

// @kotowari[C-core-forder-4, C-core-document-5]
#[test]
fn three_titles_give_one_multiple_titles_per_extra_title() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    make_cli_project(tmp.path());
    std::fs::write(
        tmp.path().join("docs/decisions/a.md"),
        "# 一\n\n範囲。\n\n# 二\n\n#  三  \n",
    )
    .unwrap();
    let v = run_cli(tmp.path());
    let multiple: Vec<_> = v["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["path"] == "docs/decisions/a.md" && f["kind"] == "multiple_titles")
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
