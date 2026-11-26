// @kotowari[C-core-markers-1]
#[test]
fn req_071_marker_syntax_allows_spaces_around_commas() {
    let markers =
        kotowari_core::tests_discovery::parse_markers_in_line("// @kotowari[REQ-001 , TBL-002]", 1);
    assert_eq!(markers.len(), 1);
    assert_eq!(markers[0].ids, vec!["REQ-001", "TBL-002"]);
}

// @kotowari[C-core-markers-2]
#[test]
fn req_073_several_markers_on_one_line() {
    let markers = kotowari_core::tests_discovery::parse_markers_in_line(
        "// @kotowari[REQ-001] @kotowari[TBL-002]",
        1,
    );
    assert_eq!(markers.len(), 2);
}

// @kotowari[C-core-markers-3]
#[test]
fn req_074_marker_anywhere_in_the_line_regardless_of_comment_syntax() {
    let markers =
        kotowari_core::tests_discovery::parse_markers_in_line("/* @kotowari[REQ-001] */", 1);
    assert_eq!(markers.len(), 1);
    assert_eq!(markers[0].ids, vec!["REQ-001"]);
}

// @kotowari[C-core-markers-1]
#[test]
fn req_071_marker_line_number_is_reported() {
    let markers =
        kotowari_core::tests_discovery::parse_markers_in_line("// @kotowari[REQ-001]", 42);
    assert_eq!(markers.len(), 1);
    assert_eq!(
        markers[0].line, 42,
        "marker line should match the given line number"
    );
}
