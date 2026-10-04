use kotowari_core::{config::Config, tests_discovery::DiscoveredTest};
use std::fs;
use tempfile::TempDir;
/// `kotowari check` と同じ入口（`TestQueries::load` と `discover_tests`）で、Rust のファイルの
/// `テスト`を発見する。"tests.rules" は空なので、基準のディレクトリは読まない
fn discover_in_rust_file(
    content: &str,
    file_rel: &str,
    config: &Config,
) -> Result<Vec<DiscoveredTest>, String> {
    let analyzer = kotowari_source_analysis::Analyzer::new(config.clone(), vec![], vec![])
        .map_err(|error| error.to_string())?;
    let source =
        kotowari_core::SourceText::new(file_rel, content).map_err(|error| error.to_string())?;
    let result = analyzer.tests(source).map_err(|error| error.to_string())?;
    if result.findings.is_empty() {
        Ok(result.tests)
    } else {
        Err(format!("syntax error in {file_rel}"))
    }
}

// @kotowari[REQ-core-082]
#[test]
fn req_082_macro_reparse_byte_offset_reflects_delimiter_position() {
    // マクロの呼び出しと開き波括弧が別の行にあるとき、行番号はその波括弧の行を基準にする
    let mut config = kotowari_core::config::Config::default();
    config.tests.rust.macros = vec!["my_macro".to_string()];

    let brace_on_own_line = "my_macro!\n{\n    // @kotowari[REQ-999]\n    fn t() {}\n}\n";
    let tests = discover_in_rust_file(brace_on_own_line, "test_a.rs", &config).expect("valid rust");
    assert_eq!(tests.len(), 1);
    assert_eq!(
        tests[0].line, 4,
        "fn line should reflect '{{' on its own line: {:?}",
        tests
    );
    assert_eq!(
        tests[0].marker_ids,
        vec![("REQ-999".to_string(), 3)],
        "marker line should reflect '{{' on its own line: {:?}",
        tests
    );

    // 波括弧以外の区切り記号（丸括弧）でも、中に波括弧のブロックがあれば同じ規則で行番号が付く
    let paren_wrapped_block = "// leading\n// leading\nmy_macro!(\n    {\n        // @kotowari[REQ-999]\n        fn t() {}\n    }\n);\n";
    let tests2 =
        discover_in_rust_file(paren_wrapped_block, "test_b.rs", &config).expect("valid rust");
    assert_eq!(tests2.len(), 1);
    assert_eq!(
        tests2[0].line, 6,
        "fn line should reflect the real position after leading lines: {:?}",
        tests2
    );
    assert_eq!(
        tests2[0].marker_ids,
        vec![("REQ-999".to_string(), 5)],
        "marker line should reflect the real position after leading lines: {:?}",
        tests2
    );
}

// @kotowari[REQ-core-082, REQ-core-118, REQ-core-072]
#[test]
fn req_082_macro_function_and_marker_lines_use_additive_offset() {
    // マクロの前に複数行あるとき（line_offset > 0）、関数・印・不正な印の行番号は
    // すべて「マクロの中の行番号 + line_offset」で計算される
    let mut config = kotowari_core::config::Config::default();
    config.tests.rust.macros = vec!["my_macro".to_string()];

    let content = "// leading 1\n// leading 2\n// leading 3\nmy_macro! {\n    // @kotowari[REQ-999]\n    // @kotowari[]\n    fn t() {\n        // @kotowari[REQ-888]\n        // @kotowari[]\n        assert!(true);\n    }\n}\n";
    let tests = discover_in_rust_file(content, "test_e.rs", &config).expect("valid rust");
    assert_eq!(tests.len(), 1);
    assert_eq!(tests[0].line, 7, "function line: {:?}", tests);
    // 本体の先頭の印は結び付かず、invalid_marker にも数えない（TBL-core-016）
    assert_eq!(
        tests[0].marker_ids,
        vec![("REQ-999".to_string(), 5)],
        "marker lines (only before the function): {:?}",
        tests
    );
    assert_eq!(
        tests[0].invalid_markers,
        vec![(6, "    // @kotowari[]".to_string())],
        "invalid marker lines (only before the function): {:?}",
        tests
    );
}

// @kotowari[REQ-core-082]
#[test]
fn req_082_plain_test_function_line_is_one_indexed() {
    let content = "// leading 1\n// leading 2\n#[test]\nfn t() {}\n";
    let tests = discover_in_rust_file(
        content,
        "test.rs",
        &kotowari_core::config::Config::default(),
    )
    .expect("valid rust");
    assert_eq!(tests.len(), 1);
    assert_eq!(
        tests[0].line, 4,
        "function line should be the 1-indexed source line: {:?}",
        tests
    );
}

// @kotowari[REQ-core-082]
#[test]
fn req_082_function_with_unrelated_attribute_is_not_counted() {
    // #[test] でも設定された属性でもない属性しか持たない関数はテストとして数えない
    let content = "#[allow(dead_code)]\nfn not_a_test() {}\n";
    let tests = discover_in_rust_file(
        content,
        "test.rs",
        &kotowari_core::config::Config::default(),
    )
    .expect("valid rust");
    assert!(
        tests.is_empty(),
        "function with only an unrelated attribute must not count as a test: {:?}",
        tests
    );
}

// @kotowari[REQ-core-082]
#[test]
fn req_082_has_attribute_skips_block_comment_to_find_test_attribute() {
    let content = "#[test]\n/* intermediate comment */\nfn t() {}\n";
    let tests = discover_in_rust_file(
        content,
        "test.rs",
        &kotowari_core::config::Config::default(),
    )
    .expect("valid rust");
    assert!(
        tests.iter().any(|t| t.name.as_deref() == Some("t")),
        "block comment between #[test] and fn must not hide the test: {:?}",
        tests
    );
}

// @kotowari[REQ-core-082]
#[test]
fn req_082_has_configured_attribute_skips_line_comment() {
    let mut config = kotowari_core::config::Config::default();
    config.tests.rust.attributes = vec!["kani::proof".to_string()];
    let content = "#[kani::proof(unwind = 3)]\n// intermediate comment\nfn my_proof() {}\n";
    let tests = discover_in_rust_file(content, "test.rs", &config).expect("valid rust");
    assert!(
        tests.iter().any(|t| t.name.as_deref() == Some("my_proof")),
        "a line comment between a configured attribute and fn must not hide the test: {:?}",
        tests
    );
}

// @kotowari[REQ-core-082]
#[test]
fn req_082_has_configured_attribute_skips_block_comment() {
    let mut config = kotowari_core::config::Config::default();
    config.tests.rust.attributes = vec!["kani::proof".to_string()];
    let content = "#[kani::proof]\n/* note */\nfn my_proof() {}\n";
    let tests = discover_in_rust_file(content, "test.rs", &config).expect("valid rust");
    assert!(
        tests.iter().any(|t| t.name.as_deref() == Some("my_proof")),
        "a block comment between a configured attribute and fn must not hide the test: {:?}",
        tests
    );
}

// @kotowari[REQ-core-072]
#[test]
fn req_072_invalid_marker_on_second_line_of_multiline_comment_before_test() {
    // 複数行にまたがるブロックコメントの2行目にある印の行番号は、
    // コメントの開始行 + オフセット + 1 になる（コメントの1行目ではない）
    let content = "// leading\n/* note\n@kotowari[] */\n#[test]\nfn t() {}\n";
    let tests = discover_in_rust_file(
        content,
        "test.rs",
        &kotowari_core::config::Config::default(),
    )
    .expect("valid rust");
    assert_eq!(tests.len(), 1);
    assert_eq!(
        tests[0].invalid_markers,
        vec![(3, "@kotowari[] */".to_string())],
        "invalid marker line and text should come from the comment's own 2nd line: {:?}",
        tests
    );
}

// @kotowari[REQ-core-072]
#[test]
fn req_072_indented_invalid_marker_before_test_keeps_indentation() {
    // 不正な印の detail は生の行の文字（インデントを含む）であり、
    // コメント自身の文字列（インデントを含まない）ではない
    let content = "    // @kotowari[]\n    #[test]\n    fn t() {}\n";
    let tests = discover_in_rust_file(
        content,
        "test.rs",
        &kotowari_core::config::Config::default(),
    )
    .expect("valid rust");
    assert_eq!(tests.len(), 1);
    assert_eq!(
        tests[0].invalid_markers,
        vec![(1, "    // @kotowari[]".to_string())],
        "invalid marker detail should be the raw indented line: {:?}",
        tests
    );
}

// @kotowari[REQ-core-072, TBL-core-035]
#[test]
fn req_072_invalid_marker_line_index_stays_additive_at_boundary() {
    // 複数行のコメントの最後の行の印の行番号は、コメントの開始行からの足し算で決まる
    let content = "// leading 1\n// leading 2\n// leading 3\n/* line2\nline3\nline4\n@kotowari[] */\n#[test] fn t() {}";
    let tests = discover_in_rust_file(
        content,
        "test.rs",
        &kotowari_core::config::Config::default(),
    )
    .expect("valid rust");
    assert_eq!(tests.len(), 1);
    assert_eq!(
        tests[0].invalid_markers,
        vec![(7, "@kotowari[] */".to_string())],
        "invalid marker line should be the comment's own last line: {:?}",
        tests
    );

    // コメントの最後の行にテストのコードが続くと、その行は直前のコメントの塊に入らない
    let shared = "/* line1\n@kotowari[] */ #[test] fn t() {}";
    let tests = discover_in_rust_file(shared, "test.rs", &kotowari_core::config::Config::default())
        .expect("valid rust");
    assert_eq!(tests.len(), 1);
    assert!(tests[0].invalid_markers.is_empty(), "{:?}", tests);
}

// @kotowari[REQ-core-072, TBL-core-016]
#[test]
fn tbl_016_invalid_marker_at_body_start_is_ignored() {
    for content in [
        "#[test]\nfn t() {\n    /* note\n    @kotowari[] */\n}\n",
        "#[test]\nfn t() {\n    // @kotowari[]\n}\n",
        "#[test]\nfn t() {\n/* line2\nline3\nline4\n@kotowari[] */}\n",
    ] {
        let tests = discover_in_rust_file(
            content,
            "test.rs",
            &kotowari_core::config::Config::default(),
        )
        .expect("valid rust");
        assert_eq!(tests.len(), 1);
        assert!(tests[0].marker_ids.is_empty(), "{content:?}: {tests:?}");
        assert!(
            tests[0].invalid_markers.is_empty(),
            "{content:?}: {tests:?}"
        );
    }
}

// @kotowari[REQ-core-075, TBL-core-035]
#[test]
fn tbl_035_comments_and_multi_line_attributes_form_one_block() {
    // コメントの行と複数行にわたる属性の行が空行なしで混ざっても、1つの塊として結び付く
    let content = "// @kotowari[REQ-001]\n#[cfg_attr(\n    feature = \"x\",\n    ignore\n)]\n// note\n#[test]\nfn t() {}\n";
    let tests = discover_in_rust_file(
        content,
        "test.rs",
        &kotowari_core::config::Config::default(),
    )
    .expect("valid rust");
    assert_eq!(tests.len(), 1);
    assert_eq!(
        tests[0].marker_ids,
        vec![("REQ-001".to_string(), 1)],
        "{tests:?}"
    );
}

// @kotowari[REQ-core-075, TBL-core-035]
#[test]
fn tbl_035_blank_line_inside_a_comment_or_an_attribute_does_not_cut_the_block() {
    // 空白だけの行でも、複数行のコメントや属性の途中にあれば塊の行に数える
    let in_comment = "/* @kotowari[REQ-001]\n\n*/\n#[test]\nfn t() {}\n";
    let in_attribute =
        "// @kotowari[REQ-001]\n#[cfg_attr(\n\n    test, ignore)]\n#[test]\nfn t() {}\n";
    for content in [in_comment, in_attribute] {
        let tests = discover_in_rust_file(
            content,
            "test.rs",
            &kotowari_core::config::Config::default(),
        )
        .expect("valid rust");
        assert_eq!(tests.len(), 1);
        assert_eq!(
            tests[0].marker_ids,
            vec![("REQ-001".to_string(), 1)],
            "{content:?}: {tests:?}"
        );
    }
}

// @kotowari[REQ-core-072, TBL-core-008, TBL-core-010]
#[test]
fn tbl_008_invalid_marker_detail_of_a_crlf_test_file_has_no_carriage_return() {
    let content = "// @kotowari[]\r\n#[test]\r\nfn t() {}\r\n";
    let tests = discover_in_rust_file(
        content,
        "test.rs",
        &kotowari_core::config::Config::default(),
    )
    .expect("valid rust");
    assert_eq!(tests.len(), 1);
    assert_eq!(
        tests[0].invalid_markers,
        vec![(1, "// @kotowari[]".to_string())],
        "{tests:?}"
    );
}

// @kotowari[REQ-core-075, TBL-core-035]
#[test]
fn tbl_035_comment_after_code_on_the_same_line_breaks_the_block() {
    let content = "// @kotowari[REQ-001]\nconst N: u8 = 1; // note\n#[test]\nfn t() {}\n";
    let tests = discover_in_rust_file(
        content,
        "test.rs",
        &kotowari_core::config::Config::default(),
    )
    .expect("valid rust");
    assert_eq!(tests.len(), 1);
    assert!(tests[0].marker_ids.is_empty(), "{tests:?}");
}

// @kotowari[REQ-core-082, TBL-core-017]
#[test]
fn tbl_017_macro_not_in_the_configuration_is_not_reread() {
    let mut config = kotowari_core::config::Config::default();
    config.tests.rust.macros = vec!["my_macro".to_string()];
    let content = "other_macro! {\n    fn t() {}\n}\nmy_macro! {\n    fn u() {}\n}\n";
    let tests = discover_in_rust_file(content, "test.rs", &config).expect("valid rust");
    let names: Vec<_> = tests.iter().map(|t| t.name.as_deref()).collect();
    assert_eq!(names, vec![Some("u")]);
}

// @kotowari[REQ-core-181]
#[test]
fn req_181_test_inside_a_test_is_counted_apart() {
    let content =
        "#[test]\nfn outer() {\n    // @kotowari[REQ-001]\n    #[test]\n    fn inner() {}\n}\n";
    let tests = discover_in_rust_file(
        content,
        "test.rs",
        &kotowari_core::config::Config::default(),
    )
    .expect("valid rust");
    let names: Vec<_> = tests.iter().map(|t| t.name.as_deref()).collect();
    assert_eq!(names, vec![Some("outer"), Some("inner")]);
    // 外の`テスト`の節の中でも、内側の`テスト`の直前の印はその内側に結び付く
    assert!(tests[0].marker_ids.is_empty(), "{tests:?}");
    assert_eq!(
        tests[1].marker_ids,
        vec![("REQ-001".to_string(), 3)],
        "{tests:?}"
    );
}

// @kotowari[REQ-core-181, REQ-core-180]
#[test]
fn req_181_one_node_hit_by_two_queries_is_one_test() {
    // 同梱の "#[test]" のルールと設定の属性のルールが同じ関数に当たる
    let mut config = kotowari_core::config::Config::default();
    // "my::check" は設定のルールにしか当たらないので、設定のルールが効いていることも分かる
    config.tests.rust.attributes = vec!["tokio::test".to_string(), "my::check".to_string()];
    let content = "#[tokio::test]\nasync fn t() {}\n#[my::check]\nfn u() {}\n";
    let tests = discover_in_rust_file(content, "test.rs", &config).expect("valid rust");
    let names: Vec<_> = tests.iter().map(|t| t.name.as_deref()).collect();
    assert_eq!(names, vec![Some("t"), Some("u")], "{tests:?}");
}

// @kotowari[REQ-core-082, TBL-core-017]
#[test]
fn tbl_017_macro_with_parentheses_or_brackets_is_reread() {
    let mut config = kotowari_core::config::Config::default();
    config.tests.rust.macros = vec!["my_macro".to_string()];
    let content = "my_macro!( fn parenthesized() {} );\nmy_macro![ fn bracketed() {} ];\n";
    let tests = discover_in_rust_file(content, "test.rs", &config).expect("valid rust");
    let names: Vec<_> = tests.iter().map(|t| t.name.as_deref()).collect();
    assert_eq!(
        names,
        vec![Some("parenthesized"), Some("bracketed")],
        "{tests:?}"
    );
}
