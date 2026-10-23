//! 文書名の参照の検査（TBL-core-014）。crate の中だけの check_document_references を直接呼ぶ

// --- check_document_references の行番号 ---

// @kotowari[REQ-core-069, TBL-core-014]
#[test]
fn req_069_doc_ref_line_number_is_correct() {
    use std::collections::BTreeSet;
    let content = "# Title\n\nScope.\n\nSee nonexistent.md here.\n";
    let doc = kotowari_core::ir::parse_document("a.md", content).unwrap();
    let ir_filenames: BTreeSet<String> = [doc.filename().to_owned()].into_iter().collect();
    let mut findings = Vec::new();
    kotowari_core::terms::check_document_references(
        &[doc],
        "docs/ir",
        &ir_filenames,
        &mut findings,
    );
    let md: Vec<_> = findings
        .iter()
        .filter(|f| f.kind() == "missing_document")
        .collect();
    assert!(
        md.iter()
            .any(|f| f.detail() == "nonexistent.md" && f.line() == Some(5)),
        "missing_document on line 5: {:?}",
        md
    );
}

// --- find_doc_refs の境界 ---

// @kotowari[REQ-core-069, TBL-core-014]
#[test]
fn req_069_doc_ref_at_line_end() {
    use std::collections::BTreeSet;
    let content = "# Title\n\nScope with nonexistent.md\n";
    let doc = kotowari_core::ir::parse_document("a.md", content).unwrap();
    let ir_filenames: BTreeSet<String> = [doc.filename().to_owned()].into_iter().collect();
    let mut findings = Vec::new();
    kotowari_core::terms::check_document_references(
        &[doc],
        "docs/ir",
        &ir_filenames,
        &mut findings,
    );
    let md: Vec<_> = findings
        .iter()
        .filter(|f| f.kind() == "missing_document")
        .collect();
    assert!(
        md.iter()
            .any(|f| f.detail() == "nonexistent.md" && f.line() == Some(3)),
        "doc ref at end of line should be detected on line 3: {:?}",
        md
    );
}

// @kotowari[REQ-core-069, TBL-core-014]
#[test]
fn req_069_doc_ref_at_line_start() {
    use std::collections::BTreeSet;
    let content = "# Title\n\nnot-found.md is referenced.\n";
    let doc = kotowari_core::ir::parse_document("a.md", content).unwrap();
    let ir_filenames: BTreeSet<String> = [doc.filename().to_owned()].into_iter().collect();
    let mut findings = Vec::new();
    kotowari_core::terms::check_document_references(
        &[doc],
        "docs/ir",
        &ir_filenames,
        &mut findings,
    );
    let md: Vec<_> = findings
        .iter()
        .filter(|f| f.kind() == "missing_document")
        .collect();
    assert!(
        md.iter()
            .any(|f| f.detail() == "not-found.md" && f.line() == Some(3)),
        "doc ref at line start should be detected on line 3: {:?}",
        md
    );
}

// @kotowari[REQ-core-069, TBL-core-014]
#[test]
fn req_069_doc_ref_after_punctuation() {
    use std::collections::BTreeSet;
    let content = "# Title\n\nScope.\n\nSee,not-found.md for details.\n";
    let doc = kotowari_core::ir::parse_document("a.md", content).unwrap();
    let ir_filenames: BTreeSet<String> = [doc.filename().to_owned()].into_iter().collect();
    let mut findings = Vec::new();
    kotowari_core::terms::check_document_references(
        &[doc],
        "docs/ir",
        &ir_filenames,
        &mut findings,
    );
    let md: Vec<_> = findings
        .iter()
        .filter(|f| f.kind() == "missing_document")
        .collect();
    assert!(
        md.iter()
            .any(|f| f.detail() == "not-found.md" && f.line() == Some(5)),
        "doc ref after punctuation should be detected on line 5: {:?}",
        md
    );
}

// @kotowari[REQ-core-069, TBL-core-014]
#[test]
fn req_069_mdx_extension_not_matched_but_md_after_it_is() {
    use std::collections::BTreeSet;
    let content = "# Title\n\nfoo.mdx bar.md text.\n";
    let doc = kotowari_core::ir::parse_document("a.md", content).unwrap();
    let ir_filenames: BTreeSet<String> = [doc.filename().to_owned()].into_iter().collect();
    let mut findings = Vec::new();
    kotowari_core::terms::check_document_references(
        &[doc],
        "docs/ir",
        &ir_filenames,
        &mut findings,
    );
    let md: Vec<_> = findings
        .iter()
        .filter(|f| f.kind() == "missing_document")
        .collect();
    assert!(
        !md.iter().any(|f| f.detail().contains("foo")),
        "foo.mdx should not be matched: {:?}",
        md
    );
    assert!(
        md.iter().any(|f| f.detail() == "bar.md"),
        "bar.md should be matched: {:?}",
        md
    );
}

// @kotowari[TBL-core-014]
#[test]
fn tbl_014_md_followed_by_letter_is_not_a_reference() {
    // "a.mdX" は参照でない
    use std::collections::BTreeSet;
    let content = "# Title\n\nScope with a.mdX text.\n";
    let doc = kotowari_core::ir::parse_document("a.md", content).unwrap();
    let ir_filenames: BTreeSet<String> = [doc.filename().to_owned()].into_iter().collect();
    let mut findings = Vec::new();
    kotowari_core::terms::check_document_references(
        &[doc],
        "docs/ir",
        &ir_filenames,
        &mut findings,
    );
    let md: Vec<_> = findings
        .iter()
        .filter(|f| f.kind() == "missing_document")
        .collect();
    assert!(md.is_empty(), "a.mdX should not be a reference: {:?}", md);
}

// @kotowari[TBL-core-014]
#[test]
fn tbl_014_unclosed_quote_hides_the_rest_of_the_line() {
    // 奇数の二重引用符の後は参照を拾わない
    use std::collections::BTreeSet;
    let content = "# Title\n\nScope.\n\nSee \"unclosed quote nonexistent.md here.\n";
    let doc = kotowari_core::ir::parse_document("a.md", content).unwrap();
    let ir_filenames: BTreeSet<String> = [doc.filename().to_owned()].into_iter().collect();
    let mut findings = Vec::new();
    kotowari_core::terms::check_document_references(
        &[doc],
        "docs/ir",
        &ir_filenames,
        &mut findings,
    );
    let md: Vec<_> = findings
        .iter()
        .filter(|f| f.kind() == "missing_document")
        .collect();
    assert!(
        md.is_empty(),
        "unclosed quote should hide the rest of the line: {:?}",
        md
    );
}

// @kotowari[TBL-core-014]
#[test]
fn tbl_014_text_between_the_second_and_third_quote_is_still_scanned() {
    // 二重引用符が奇数（3つ）のとき、行末を隠すのは「最後の」引用符から先だけ。
    // 最初の引用符で打ち切ってはいけない（2つ目と3つ目の間は引用符の外）。
    use std::collections::BTreeSet;
    let content = "# Title\n\nScope.\n\nSee \"note\" and outside.md here \"trail\n";
    let doc = kotowari_core::ir::parse_document("a.md", content).unwrap();
    let ir_filenames: BTreeSet<String> = [doc.filename().to_owned()].into_iter().collect();
    let mut findings = Vec::new();
    kotowari_core::terms::check_document_references(
        &[doc],
        "docs/ir",
        &ir_filenames,
        &mut findings,
    );
    let md: Vec<_> = findings
        .iter()
        .filter(|f| f.kind() == "missing_document")
        .collect();
    assert!(
        md.iter().any(|f| f.detail() == "outside.md"),
        "text between the 2nd and 3rd quote is outside quotes and should still be scanned: {:?}",
        md
    );
}

// @kotowari[TBL-core-014]
#[test]
fn tbl_014_long_digit_run_before_mdx_does_not_produce_a_spurious_reference() {
    // ".mdx" の直前が長い数字の並びでも、".mdx" は参照として拾わない
    use std::collections::BTreeSet;
    let content = "# Title\n\n01234567890123456789012.mdx\n";
    let doc = kotowari_core::ir::parse_document("a.md", content).unwrap();
    let ir_filenames: BTreeSet<String> = [doc.filename().to_owned()].into_iter().collect();
    let mut findings = Vec::new();
    kotowari_core::terms::check_document_references(
        &[doc],
        "docs/ir",
        &ir_filenames,
        &mut findings,
    );
    let md: Vec<_> = findings
        .iter()
        .filter(|f| f.kind() == "missing_document")
        .collect();
    assert!(
        !md.iter()
            .any(|f| f.detail().contains("01234567890123456789012")),
        "a long digit run before .mdx must not be treated as a .md reference: {:?}",
        md
    );
}

// @kotowari[TBL-core-014]
#[test]
fn tbl_014_md_followed_by_hyphen_is_not_a_reference() {
    // ".md" の直後が "-" のときは参照でない
    use std::collections::BTreeSet;
    let content = "# Title\n\na.md-suffix\n";
    let doc = kotowari_core::ir::parse_document("x.md", content).unwrap();
    let ir_filenames: BTreeSet<String> = [doc.filename().to_owned()].into_iter().collect();
    let mut findings = Vec::new();
    kotowari_core::terms::check_document_references(
        &[doc],
        "docs/ir",
        &ir_filenames,
        &mut findings,
    );
    let md: Vec<_> = findings
        .iter()
        .filter(|f| f.kind() == "missing_document")
        .collect();
    assert!(
        !md.iter().any(|f| f.detail() == "a.md"),
        "'a.md-suffix' must not be treated as a reference to 'a.md': {:?}",
        md
    );
}

// @kotowari[TBL-core-014]
#[test]
fn tbl_014_bare_dot_md_with_nothing_before_it_is_not_a_reference() {
    // 直前に文字が無い（空白の直後の）裸の ".md" は参照でない
    use std::collections::BTreeSet;
    let content = "# Title\n\nthe .md file\n";
    let doc = kotowari_core::ir::parse_document("a.md", content).unwrap();
    let ir_filenames: BTreeSet<String> = [doc.filename().to_owned()].into_iter().collect();
    let mut findings = Vec::new();
    kotowari_core::terms::check_document_references(
        &[doc],
        "docs/ir",
        &ir_filenames,
        &mut findings,
    );
    let md: Vec<_> = findings
        .iter()
        .filter(|f| f.kind() == "missing_document")
        .collect();
    assert!(
        !md.iter().any(|f| f.detail() == ".md"),
        "a bare '.md' with no name before it must not be treated as a document reference: {:?}",
        md
    );
}

// @kotowari[REQ-core-069, TBL-core-014]
#[test]
fn tbl_014_md_followed_by_hash_or_slash_is_not_a_reference() {
    let doc = kotowari_core::ir::parse_document(
        "x.md",
        "# Title\n\na.md#A12 docs/decision/records/records.md#A12 a.md/b.md a//b.md\n",
    )
    .unwrap();
    let mut findings = Vec::new();
    kotowari_core::terms::check_document_references(
        &[doc],
        "docs/ir",
        &Default::default(),
        &mut findings,
    );
    assert!(findings.is_empty(), "{:?}", findings);
}

// @kotowari[REQ-core-069, REQ-core-070, TBL-core-014]
#[test]
fn tbl_014_slash_separated_path_is_a_reference() {
    let doc =
        kotowari_core::ir::parse_document("a.md", "# Title\n\nSee network/dns/b.md.\n").unwrap();
    let mut findings = Vec::new();
    kotowari_core::terms::check_document_references(
        &[doc],
        "docs/ir",
        &Default::default(),
        &mut findings,
    );
    assert_eq!(findings.len(), 1, "{:?}", findings);
    assert_eq!(findings[0].detail(), "network/dns/b.md");
}
