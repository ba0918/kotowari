use super::*;

fn section_blocks(doc: &Document) -> &[Block] {
    assert_eq!(doc.sections.len(), 1, "節が1つあること");
    &doc.sections[0].blocks
}

// @kotowari[REQ-schema-031]
#[test]
fn nested_list_items_become_children_of_the_parent_bullet() {
    let doc = Document::parse("## 理由\n\n- 親\n  - 子\n    - 孫\n").unwrap();
    let blocks = section_blocks(&doc);
    assert_eq!(blocks.len(), 1);
    let Block::Bullet { text, children, .. } = &blocks[0] else {
        panic!("親が Bullet になる");
    };
    assert_eq!(text, "親");
    assert_eq!(children.len(), 1);
    let Block::Bullet { text, children, .. } = &children[0] else {
        panic!("子が Bullet になる");
    };
    assert_eq!(text, "子");
    assert_eq!(children.len(), 1);
    let Block::Bullet { text, .. } = &children[0] else {
        panic!("孫が Bullet になる");
    };
    assert_eq!(text, "孫");
}

// @kotowari[REQ-schema-031]
#[test]
fn nested_list_children_keep_their_indentation() {
    let doc = Document::parse("## 理由\n\n- 親\n  - 子\n    - 孫\n").unwrap();
    let blocks = section_blocks(&doc);
    let Block::Bullet {
        line_text,
        children,
        ..
    } = &blocks[0]
    else {
        panic!("親が Bullet になる");
    };
    assert_eq!(line_text, "- 親");
    let Block::Bullet {
        line_text,
        children,
        ..
    } = &children[0]
    else {
        panic!("子が Bullet になる");
    };
    assert_eq!(line_text, "  - 子");
    let Block::Bullet { line_text, .. } = &children[0] else {
        panic!("孫が Bullet になる");
    };
    assert_eq!(line_text, "    - 孫");
}

// @kotowari[REQ-schema-030]
#[test]
fn continuation_paragraph_attaches_to_the_parent_bullet() {
    let doc = Document::parse("## 理由\n\n- 親\n\n  続きの段落\n  - 子\n").unwrap();
    let blocks = section_blocks(&doc);
    let Block::Bullet {
        continuation,
        children,
        ..
    } = &blocks[0]
    else {
        panic!("親が Bullet になる");
    };
    assert_eq!(continuation, &vec![LineSpan { first: 5, last: 5 }]);
    assert_eq!(children.len(), 1);
}

// @kotowari[REQ-schema-031, REQ-schema-034]
#[test]
fn code_block_child_of_bullet_remains_a_block() {
    let doc = Document::parse("## 理由\n\n- 親\n\n  ```python\n  x = 1\n  ```\n").unwrap();
    let blocks = section_blocks(&doc);
    let Block::Bullet { children, .. } = &blocks[0] else {
        panic!("親が Bullet になる");
    };
    assert!(matches!(children[0], Block::Code { lang: Some(ref l), .. } if l == "python"));
}

// @kotowari[REQ-schema-031, REQ-schema-033]
#[test]
fn table_child_of_bullet_remains_a_block() {
    let doc =
        Document::parse("## 理由\n\n- 親\n\n  | a | b |\n  |---|---|\n  | 1 | 2 |\n").unwrap();
    let blocks = section_blocks(&doc);
    let Block::Bullet { children, .. } = &blocks[0] else {
        panic!("親が Bullet になる");
    };
    assert!(
        matches!(children[0], Block::Table { ref header, .. } if header == &vec!["a".to_string(), "b".to_string()])
    );
}

// @kotowari[REQ-schema-032]
#[test]
fn statement_after_code_block_lead_is_a_statement() {
    let doc = Document::parse("## 理由\n\n- ```python\n  x = 1\n  ```\n\n  後続の段落\n").unwrap();
    let blocks = section_blocks(&doc);
    assert!(matches!(blocks[0], Block::Code { .. }));
    assert!(matches!(blocks[1], Block::Statement { ref text, .. } if text == "後続の段落"));
}

// @kotowari[REQ-schema-031]
#[test]
fn field_line_with_children_keeps_children() {
    let doc = Document::parse("## 理由\n\n- 状態: 承認済み\n  - 子\n").unwrap();
    let blocks = section_blocks(&doc);
    let Block::Field {
        name,
        children,
        line_text,
        ..
    } = &blocks[0]
    else {
        panic!("親の `- 名前: 値` 行が Field になる");
    };
    assert_eq!(name, "状態");
    assert_eq!(line_text, "- 状態: 承認済み");
    assert_eq!(children.len(), 1);
}

// @kotowari[REQ-schema-028]
#[test]
fn ordered_list_item_children_are_not_attached() {
    let doc = Document::parse("## 理由\n\n1. 順序付き\n   - 箇条書き\n").unwrap();
    let blocks = section_blocks(&doc);
    assert!(matches!(blocks[0], Block::OrderedList { .. }));
    assert!(
        matches!(blocks[1], Block::Bullet { .. }),
        "順序付きの入れ子は対象外でトップレベルに残る"
    );
}
// @kotowari[REQ-schema-008]
#[test]
fn title_heading_keeps_the_raw_line_including_inline_code() {
    let src = "# 題名 `インライン` と [リンク](https://example.com/)\n";
    let doc = Document::parse(src).unwrap();
    let raw = doc.raw_line(doc.titles[0].line).unwrap();
    assert_eq!(
        raw, "# 題名 `インライン` と [リンク](https://example.com/)",
        "見出しの行の生の文字は src の行と一文字も違わない"
    );
    assert_ne!(
        raw, doc.titles[0].text,
        "組み立て直した見出しの文字とは違う"
    );
}

// @kotowari[REQ-schema-035]
#[test]
fn statement_keeps_each_raw_line_with_its_line_number() {
    let src = "## 節\n\n 文の1行目\n  字下げの2行目\n   3行目\n";
    let doc = Document::parse(src).unwrap();
    let blocks = section_blocks(&doc);
    let Block::Statement { raw_lines, .. } = &blocks[0] else {
        panic!("段落が文になる");
    };
    assert_eq!(raw_lines.len(), 3);
    assert_eq!(raw_lines[0].line, 3);
    assert_eq!(raw_lines[0].text, " 文の1行目");
    assert_eq!(raw_lines[1].line, 4);
    assert_eq!(raw_lines[1].text, "  字下げの2行目");
    assert_eq!(raw_lines[2].line, 5);
    assert_eq!(raw_lines[2].text, "   3行目");
}

// @kotowari[REQ-schema-048]
#[test]
fn table_keeps_the_line_number_of_each_data_row() {
    let src = "## 節\n\n| a | b |\n|---|---|\n| 1 | 2 |\n| 3 | 4 |\n| 5 | 6 |\n";
    let doc = Document::parse(src).unwrap();
    let blocks = section_blocks(&doc);
    let Block::Table {
        rows,
        row_lines,
        line,
        ..
    } = &blocks[0]
    else {
        panic!("表になる");
    };
    assert_eq!(*line, 3, "表の開始行はヘッダの行");
    assert_eq!(rows.len(), 3);
    assert_eq!(
        row_lines,
        &vec![5, 6, 7],
        "ヘッダの行と区切りの行は数えない"
    );
}

// @kotowari[REQ-schema-062]
#[test]
fn end_line_does_not_count_a_heading_shaped_line_inside_a_code_block() {
    let src = "## 節\n\n### A-1: a\n\n```\n## 中\n```\n\n### A-2: b\n";
    let doc = Document::parse(src).unwrap();
    assert_eq!(
        doc.end_line(3, 3),
        8,
        "コードブロックの中の \"## 中\" では終わらない"
    );
    assert_eq!(
        doc.end_line(1, 2),
        9,
        "文書の最後の行。末尾の区切りの後ろは数えない"
    );
    let by_line = doc.read_by_line();
    assert_eq!(by_line.end_line(3, 3), 8, "行の読み方でも同じ");
}

// @kotowari[REQ-schema-062]
#[test]
fn end_line_is_the_last_line_when_the_document_does_not_end_with_a_break() {
    let doc = Document::parse("## 節\n\n### A-1: a\n\n本文").unwrap();
    assert_eq!(
        doc.end_line(3, 3),
        5,
        "最後の行に区切りが無くてもその行が最後の行"
    );
    assert_eq!(doc.read_by_line().end_line(3, 3), 5, "行の読み方でも同じ");
}

// @kotowari[REQ-schema-030]
#[test]
fn a_continuation_paragraph_of_an_ordered_item_belongs_to_that_item() {
    let doc = Document::parse("## 理由\n\n1. 順序付き\n\n   続きの段落\n").unwrap();
    let blocks = section_blocks(&doc);
    assert_eq!(blocks.len(), 1, "継続段落は文にならない: {blocks:?}");
    assert!(
        matches!(blocks[0], Block::OrderedList { .. }),
        "順序付きの行になる: {blocks:?}"
    );
}

// @kotowari[REQ-schema-035]
#[test]
fn a_lone_cr_breaks_a_line_the_same_way_as_lf_and_crlf() {
    let statement_lines = |src: &str| {
        let doc = Document::parse(src).unwrap();
        let blocks = section_blocks(&doc);
        let Block::Statement { raw_lines, .. } = &blocks[0] else {
            panic!("段落が文になる");
        };
        raw_lines
            .iter()
            .map(|raw| (raw.line, raw.text.clone()))
            .collect::<Vec<_>>()
    };
    let lf = statement_lines("## 節\n\nAAA\nBBB\nCCC\n");
    assert_eq!(
        statement_lines("## 節\r\n\r\nAAA\r\nBBB\r\nCCC\r\n"),
        lf,
        "CRLF の文書も LF と同じ行に分かれる"
    );
    assert_eq!(
        statement_lines("## 節\r\rAAA\rBBB\rCCC\r"),
        lf,
        "単独の CR の文書も LF と同じ行に分かれる"
    );
}
