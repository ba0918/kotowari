//! スキーマの側の指摘を kotowari の指摘へ写す（REQ-core-171、REQ-core-172、TBL-core-029、TBL-core-030）。
//!
//! 写し先は TBL-core-030 が決める。表に行の無い種類と、写し先を「発生しない」と書いた行の
//! 種類を受けたときは、黙って捨てずに`停止`する（REQ-core-172）。写せない値を受けたときも
//! 同じ理由で`停止`する（REQ-core-175）。

use crate::doc_kind::DocKind;
use crate::schema::schema_for;
use crate::{Finding, FindingKind, StopReason};
use kotowari_markdown_schema::finding::{FindingKind as EngineKind, RuleKind};
struct EngineFinding {
    kind: EngineKind,
    line: Option<usize>,
    node: Option<String>,
    raw: Option<String>,
    rule_kind: Option<RuleKind>,
}
impl From<kotowari_markdown_schema::finding::Finding> for EngineFinding {
    fn from(value: kotowari_markdown_schema::finding::Finding) -> Self {
        Self {
            kind: value.kind(),
            line: value.line(),
            node: value.node().map(str::to_owned),
            raw: value.raw().map(str::to_owned),
            rule_kind: value.rule_kind(),
        }
    }
}
#[cfg(test)]
impl EngineFinding {
    fn maybe_at(kind: EngineKind, line: Option<usize>, _detail: String) -> Self {
        Self {
            kind,
            line,
            node: None,
            raw: None,
            rule_kind: None,
        }
    }
    fn of_node(mut self, node: &str) -> Self {
        self.node = Some(node.into());
        self
    }
}
use kotowari_markdown_schema::{Document, Schema, ValidationOptions};
fn extract_values(schema: &Schema, document: &Document) -> Value {
    kotowari_markdown_schema::extract_partial(schema, document, ValidationOptions::default())
        .values()
        .clone()
}
fn validate(schema: &Schema, document: &Document, relax: bool) -> Vec<EngineFinding> {
    kotowari_markdown_schema::validate(schema, document, ValidationOptions { relax })
        .into_iter()
        .map(EngineFinding::from)
        .collect()
}
use serde_json::Value;

/// 用語集の表の行を`抽出`が置く配置パス（`.kotowari/schemas/context.yaml`）。
const GLOSSARY_ROWS: &str = "glossary";

/// `抽出`から拾った`項目`。`指摘`の行で突き合わせる。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtractedItem {
    /// 見出しの行
    pub line: usize,
    /// 見出しの ID
    pub id: String,
    /// "- kind:" の値
    pub kind: Option<String>,
    /// "- verification:" の値
    pub verification: Option<String>,
}

/// 写し先を決めるのに要る、文書ごとの手がかり。
#[derive(Debug, Clone)]
pub struct MapContext {
    /// 指摘に付ける文書のパス
    pub path: String,
    /// 文書名。detail の材料になる
    pub filename: String,
    /// どのスキーマで検証したか（TBL-core-029 の「ノードの名前」の列）
    pub doc_kind: DocKind,
    /// `抽出`の`題名`
    pub title: Option<String>,
    /// `抽出`の`項目`。見出しの行の昇順
    pub items: Vec<ExtractedItem>,
    /// `用語集`の表のデータ行の行番号
    pub glossary_rows: Vec<usize>,
}

impl MapContext {
    /// `抽出`の結果から手がかりを組み立てる。
    pub fn new(path: &str, filename: &str, doc_kind: DocKind, values: &Value) -> Self {
        let mut items = Vec::new();
        collect_items(values, &mut items);
        items.sort_by_key(|item| item.line);
        MapContext {
            path: path.to_string(),
            filename: filename.to_string(),
            doc_kind,
            title: values
                .get("title")
                .and_then(Value::as_str)
                .map(str::to_string),
            items,
            glossary_rows: row_lines(values.get(GLOSSARY_ROWS)),
        }
    }

    /// その行を含む`項目`。見出しの行がその行以下で最も大きいもの。
    fn item_at(&self, line: usize) -> Option<&ExtractedItem> {
        self.items.iter().rev().find(|item| item.line <= line)
    }
}

/// `抽出`の木から`項目`を拾う。`ID`と行番号の両方を持つオブジェクトが`項目`。
fn collect_items(value: &Value, out: &mut Vec<ExtractedItem>) {
    match value {
        Value::Object(map) => {
            if let (Some(Value::String(id)), Some(line)) = (map.get("id"), map.get("line"))
                && let Some(line) = line.as_u64()
            {
                out.push(ExtractedItem {
                    line: line as usize,
                    id: id.clone(),
                    kind: map.get("kind").and_then(Value::as_str).map(str::to_string),
                    verification: map
                        .get("verification")
                        .and_then(Value::as_str)
                        .map(str::to_string),
                });
            }
            for child in map.values() {
                collect_items(child, out);
            }
        }
        Value::Array(items) => {
            for child in items {
                collect_items(child, out);
            }
        }
        _ => {}
    }
}

/// 表の行の並びから行番号を拾う。繰り返す表では表ごとの段を跨いで拾う。
fn row_lines(value: Option<&Value>) -> Vec<usize> {
    let mut lines = Vec::new();
    fn walk(value: &Value, lines: &mut Vec<usize>) {
        match value {
            Value::Object(map) => {
                if let Some(line) = map.get("line").and_then(Value::as_u64) {
                    lines.push(line as usize);
                }
            }
            Value::Array(items) => {
                for child in items {
                    walk(child, lines);
                }
            }
            _ => {}
        }
    }
    if let Some(value) = value {
        walk(value, &mut lines);
    }
    lines
}

/// 写しの誤りで停止する（TBL-core-018、TBL-core-020）。
fn stop(detail: String) -> StopReason {
    StopReason::MappingError(detail)
}

/// 対応表に写し先が無い（REQ-core-172）。
fn no_row(engine: &EngineFinding) -> StopReason {
    let what = match (&engine.node, engine.rule_kind) {
        (Some(node), _) => format!(" {node}"),
        (None, Some(rule)) => format!(" {}", rule.as_str()),
        (None, None) => String::new(),
    };
    stop(format!("no mapping for {}{what}", engine.kind.as_str()))
}

/// `生の行`を要る写しで、それが無い（REQ-core-175）。
fn raw_of(engine: &EngineFinding) -> Result<String, StopReason> {
    engine.raw.clone().ok_or_else(|| {
        stop(format!(
            "{} carries no raw line to map",
            engine.kind.as_str()
        ))
    })
}

/// `ノードの名前`を要る写しで、それが無い。
fn node_of(engine: &EngineFinding) -> Result<&str, StopReason> {
    engine.node.as_deref().ok_or_else(|| {
        stop(format!(
            "{} carries no node name to map",
            engine.kind.as_str()
        ))
    })
}

/// `規則種別`を要る写しで、それが無い。
fn rule_of(engine: &EngineFinding) -> Result<RuleKind, StopReason> {
    engine.rule_kind.ok_or_else(|| {
        stop(format!(
            "{} carries no rule kind to map",
            engine.kind.as_str()
        ))
    })
}

/// 行を要る写しで、それが無い。
fn line_of(engine: &EngineFinding) -> Result<usize, StopReason> {
    engine
        .line
        .ok_or_else(|| stop(format!("{} carries no line to map", engine.kind.as_str())))
}

/// その行を含む`項目`。
fn item_at<'a>(
    ctx: &'a MapContext,
    engine: &EngineFinding,
    line: usize,
) -> Result<&'a ExtractedItem, StopReason> {
    ctx.item_at(line).ok_or_else(|| {
        stop(format!(
            "{} at line {line} has no item to read",
            engine.kind.as_str()
        ))
    })
}

/// スキーマの側の指摘1件を kotowari の指摘へ写す（TBL-core-030）。写し先が「出さない」の
/// 行の指摘は None を返して捨てる（REQ-core-172）。
fn map_finding(ctx: &MapContext, engine: &EngineFinding) -> Result<Option<Finding>, StopReason> {
    let make = |kind: FindingKind, line: Option<usize>, detail: String| {
        Ok(Some(Finding::new(kind, ctx.path.clone(), line, detail)))
    };
    match engine.kind {
        EngineKind::MissingTitle => make(FindingKind::MissingTitle, None, ctx.filename.clone()),
        // 2つ目以降の題名ごとに1件。detail はその題名の行から "# " を除いた文字（TBL-core-030）
        EngineKind::MultipleTitles => {
            let line = raw_of(engine)?;
            let title = line.strip_prefix('#').unwrap_or(&line).trim().to_string();
            make(FindingKind::MultipleTitles, None, title)
        }
        // 題名にパターンを宣言しているのは用語集のスキーマだけ（TBL-core-030）
        EngineKind::TitlePatternMismatch if ctx.doc_kind == DocKind::Glossary => make(
            FindingKind::GlossaryTitleInvalid,
            engine.line,
            raw_of(engine)?,
        ),
        EngineKind::UndeclaredHeading
        | EngineKind::HeadingLevelMismatch
        | EngineKind::InvalidId => make(FindingKind::UnknownHeading, engine.line, raw_of(engine)?),
        EngineKind::UndeclaredLine => match rule_of(engine)? {
            RuleKind::Field | RuleKind::Bullets | RuleKind::OrderedList => {
                make(FindingKind::UnknownField, engine.line, raw_of(engine)?)
            }
            // select: first で用語集の表にならなかった表は写さずに捨てる（`除外`、REQ-core-117）
            RuleKind::Table if ctx.doc_kind == DocKind::Glossary => Ok(None),
            RuleKind::Statement | RuleKind::Table | RuleKind::CodeBlock => {
                make(FindingKind::UnknownLine, engine.line, raw_of(engine)?)
            }
            _ => Err(no_row(engine)),
        },
        EngineKind::MissingRequiredField => {
            let node = node_of(engine)?;
            let id = || -> Result<String, StopReason> {
                Ok(item_at(ctx, engine, line_of(engine)?)?.id.clone())
            };
            match node {
                "verification" => make(FindingKind::VerificationMissing, engine.line, id()?),
                "source" => make(FindingKind::MissingSource, engine.line, id()?),
                "kind" | "how_to_verify" | "related" => {
                    make(FindingKind::MissingField, engine.line, node.to_string())
                }
                "definition" => make(FindingKind::AlgorithmWithoutDefinition, engine.line, id()?),
                _ => Err(no_row(engine)),
            }
        }
        // 値の誤りは項目の見出しの行に付け直し、detail は抽出の項目の値から取る（TBL-core-030）
        EngineKind::FieldEnumInvalid => {
            let item = item_at(ctx, engine, line_of(engine)?)?;
            let value = |slot: &Option<String>, name: &str| -> Result<String, StopReason> {
                slot.clone().ok_or_else(|| {
                    stop(format!(
                        "field_enum_invalid {name} has no extracted value at line {}",
                        item.line
                    ))
                })
            };
            match node_of(engine)? {
                "kind" => make(
                    FindingKind::UnknownKind,
                    Some(item.line),
                    value(&item.kind, "kind")?,
                ),
                "verification" => make(
                    FindingKind::VerificationInvalid,
                    Some(item.line),
                    value(&item.verification, "verification")?,
                ),
                _ => Err(no_row(engine)),
            }
        }
        // 用語集の表のデータ行の誤りは行ごとに出す。表の開始行の食い違いは、select: first で
        // ヘッダの合わない表が宣言の外になるので発生しない（TBL-core-030）
        EngineKind::TableHeaderMismatch if ctx.doc_kind == DocKind::Glossary => {
            let line = line_of(engine)?;
            if ctx.glossary_rows.contains(&line) {
                make(FindingKind::InvalidGlossaryRow, Some(line), raw_of(engine)?)
            } else {
                Err(no_row(engine))
            }
        }
        EngineKind::CodeblockLangMismatch => {
            make(FindingKind::UnknownCodeBlock, engine.line, raw_of(engine)?)
        }
        EngineKind::RepeatMinNotMet => match (rule_of(engine)?, engine.line) {
            (RuleKind::Statement, None) => {
                make(FindingKind::MissingScope, None, ctx.filename.clone())
            }
            (RuleKind::Statement, Some(line)) => make(
                FindingKind::MissingStatement,
                Some(line),
                item_at(ctx, engine, line)?.id.clone(),
            ),
            (RuleKind::Table, None) => {
                make(FindingKind::GlossaryInvalid, None, ctx.filename.clone())
            }
            (RuleKind::Table, Some(line)) => make(
                FindingKind::MissingTable,
                Some(line),
                item_at(ctx, engine, line)?.id.clone(),
            ),
            _ => Err(no_row(engine)),
        },
        EngineKind::RepeatMaxExceeded => match rule_of(engine)? {
            RuleKind::Field => make(
                FindingKind::DuplicateField,
                engine.line,
                node_of(engine)?.to_string(),
            ),
            _ => Err(no_row(engine)),
        },
        _ => Err(no_row(engine)),
    }
}

/// 文書1つをスキーマの側に読ませ、`抽出`の値と、写した`指摘`を返す
/// （REQ-core-168、REQ-core-171）。`指摘`のパスは空で、呼ぶ側が入れる。
pub fn read_document(
    filename: &str,
    doc_kind: DocKind,
    content: &str,
) -> Result<(Value, Vec<Finding>), StopReason> {
    // 取り込んだスキーマは組み立ての時点で決まっている。読めないことは利用者の入力では
    // 起こらないので、スキーマを読めないことを理由とする停止は持たない（REQ-core-175）
    let schema = schema_for(doc_kind).map_err(|e| stop(format!("{filename}: schema: {}", e.0)))?;
    let document =
        Document::parse(content).map_err(|e| stop(format!("{filename}: document: {e}")))?;
    let values = extract_values(&schema, &document);
    let ctx = MapContext::new("", filename, doc_kind, &values);
    let findings = validate(&schema, &document, schema.is_open())
        .iter()
        .filter_map(|engine| map_finding(&ctx, engine).transpose())
        .collect::<Result<Vec<Finding>, StopReason>>()?;
    Ok((values, findings))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn ctx(doc_kind: DocKind) -> MapContext {
        MapContext::new(
            "docs/ir/core/a.md",
            "a.md",
            doc_kind,
            &json!({
                "title": "題名",
                "requirements": [
                    { "id": "REQ-core-001", "line": 10, "kind": "ubiquitous", "verification": "unit" },
                    { "id": "REQ-core-002", "line": 20, "kind": "bogus", "verification": "nope" }
                ],
                "glossary": [
                    { "value": { "Term": "印" }, "line": 5 },
                    { "value": { "Term": "" }, "line": 6 }
                ]
            }),
        )
    }

    fn engine(kind: EngineKind, line: Option<usize>) -> EngineFinding {
        EngineFinding::maybe_at(kind, line, "detail".to_string())
    }

    fn mapped(f: EngineFinding) -> Finding {
        map_finding(&ctx(DocKind::Topic), &f)
            .expect("写せるはず")
            .expect("捨てずに写すはず")
    }

    fn stopped(doc_kind: DocKind, f: EngineFinding) -> String {
        map_finding(&ctx(doc_kind), &f)
            .expect_err("停止するはず")
            .to_string()
    }

    // --- TBL-core-030 の写し先を持つ 25 行 ---

    // @kotowari[REQ-core-171]
    #[test]
    fn missing_title_becomes_missing_title_with_the_document_name() {
        let f = mapped(engine(EngineKind::MissingTitle, None));
        assert_eq!(f.kind, FindingKind::MissingTitle);
        assert_eq!(f.line, None);
        assert_eq!(f.detail, "a.md");
        assert_eq!(f.path, "docs/ir/core/a.md");
    }

    // @kotowari[REQ-core-171]
    #[test]
    fn multiple_titles_becomes_multiple_titles_with_that_title() {
        let mut e = engine(EngineKind::MultipleTitles, Some(3));
        e.raw = Some("#  二つ目の題名  ".to_string());
        let f = mapped(e);
        assert_eq!(f.kind, FindingKind::MultipleTitles);
        assert_eq!(f.line, None);
        assert_eq!(
            f.detail, "二つ目の題名",
            "\"# \" と前後の空白を除いたその題名の文字"
        );
    }

    // @kotowari[REQ-core-171]
    #[test]
    fn title_pattern_mismatch_in_the_glossary_becomes_glossary_title_invalid() {
        let mut e = engine(EngineKind::TitlePatternMismatch, Some(1));
        e.raw = Some("# 用語の一覧".to_string());
        let f = map_finding(&ctx(DocKind::Glossary), &e).unwrap().unwrap();
        assert_eq!(f.kind, FindingKind::GlossaryTitleInvalid);
        assert_eq!(f.line, Some(1));
        assert_eq!(f.detail, "# 用語の一覧");
    }

    // @kotowari[REQ-core-171]
    #[test]
    fn heading_findings_become_unknown_heading_with_the_raw_line() {
        for kind in [
            EngineKind::UndeclaredHeading,
            EngineKind::HeadingLevelMismatch,
            EngineKind::InvalidId,
        ] {
            let mut e = engine(kind, Some(7));
            e.raw = Some("## 余計な節".to_string());
            let f = mapped(e);
            assert_eq!(f.kind, FindingKind::UnknownHeading, "{kind:?}");
            assert_eq!(f.line, Some(7));
            assert_eq!(f.detail, "## 余計な節");
        }
    }

    // @kotowari[REQ-core-171]
    #[test]
    fn undeclared_list_lines_become_unknown_field_with_the_raw_line() {
        for rule in [RuleKind::Field, RuleKind::Bullets, RuleKind::OrderedList] {
            let mut e = engine(EngineKind::UndeclaredLine, Some(12));
            e.raw = Some("- 余計: 値".to_string());
            e.rule_kind = Some(rule);
            let f = mapped(e);
            assert_eq!(f.kind, FindingKind::UnknownField, "{rule:?}");
            assert_eq!(f.line, Some(12));
            assert_eq!(f.detail, "- 余計: 値");
        }
    }

    // @kotowari[REQ-core-171]
    #[test]
    fn undeclared_statements_tables_and_code_blocks_become_unknown_line() {
        for rule in [RuleKind::Statement, RuleKind::Table, RuleKind::CodeBlock] {
            let mut e = engine(EngineKind::UndeclaredLine, Some(13));
            e.raw = Some("余計な文".to_string());
            e.rule_kind = Some(rule);
            let f = mapped(e);
            assert_eq!(f.kind, FindingKind::UnknownLine, "{rule:?}");
            assert_eq!(f.line, Some(13));
            assert_eq!(f.detail, "余計な文");
        }
    }

    // @kotowari[REQ-core-171]
    #[test]
    fn a_missing_field_line_splits_by_the_name_of_the_field() {
        let cases = [
            (
                "verification",
                FindingKind::VerificationMissing,
                "REQ-core-001",
            ),
            ("source", FindingKind::MissingSource, "REQ-core-001"),
            ("kind", FindingKind::MissingField, "kind"),
            ("how_to_verify", FindingKind::MissingField, "how_to_verify"),
            ("related", FindingKind::MissingField, "related"),
            (
                "definition",
                FindingKind::AlgorithmWithoutDefinition,
                "REQ-core-001",
            ),
        ];
        for (node, kind, detail) in cases {
            let e = engine(EngineKind::MissingRequiredField, Some(10)).of_node(node);
            let f = mapped(e);
            assert_eq!(f.kind, kind, "{node}");
            assert_eq!(f.line, Some(10), "{node}");
            assert_eq!(f.detail, detail, "{node}");
        }
    }

    // @kotowari[REQ-core-171]
    #[test]
    fn a_field_value_outside_the_allowed_list_is_repointed_at_the_item_heading() {
        let e = engine(EngineKind::FieldEnumInvalid, Some(22)).of_node("kind");
        let f = mapped(e);
        assert_eq!(f.kind, FindingKind::UnknownKind);
        assert_eq!(f.line, Some(20), "項目の見出しの行に付け直す");
        assert_eq!(f.detail, "bogus", "detail は抽出の項目の該当の値");

        let e = engine(EngineKind::FieldEnumInvalid, Some(23)).of_node("verification");
        let f = mapped(e);
        assert_eq!(f.kind, FindingKind::VerificationInvalid);
        assert_eq!(f.line, Some(20));
        assert_eq!(f.detail, "nope");
    }

    // @kotowari[REQ-core-171]
    #[test]
    fn a_glossary_data_row_with_the_wrong_columns_becomes_invalid_glossary_row() {
        let mut e = engine(EngineKind::TableHeaderMismatch, Some(6));
        e.raw = Some("| | Meaning |".to_string());
        let f = map_finding(&ctx(DocKind::Glossary), &e).unwrap().unwrap();
        assert_eq!(f.kind, FindingKind::InvalidGlossaryRow);
        assert_eq!(f.line, Some(6));
        assert_eq!(f.detail, "| | Meaning |");
    }

    // @kotowari[REQ-core-171]
    #[test]
    fn a_glossary_table_header_mismatch_at_the_table_start_stops() {
        // select: first でヘッダの合わない表は宣言の外になるので、表の開始行の食い違いは発生しない
        let mut e = engine(EngineKind::TableHeaderMismatch, Some(4));
        e.raw = Some("| Term | Meaning |".to_string());
        assert!(
            stopped(DocKind::Glossary, e).contains("no mapping for table_header_mismatch"),
            "発生しない行の指摘は停止する"
        );
    }

    // @kotowari[REQ-core-172]
    #[test]
    fn an_undeclared_table_in_the_glossary_is_dropped_and_elsewhere_is_unknown_line() {
        let mut e = engine(EngineKind::UndeclaredLine, Some(9));
        e.raw = Some("| a | b |".to_string());
        e.rule_kind = Some(RuleKind::Table);
        assert!(
            map_finding(&ctx(DocKind::Glossary), &e).unwrap().is_none(),
            "用語集の表にならなかった表は捨てる"
        );
        assert_eq!(mapped(e).kind, FindingKind::UnknownLine);
    }

    // @kotowari[REQ-core-171]
    #[test]
    fn a_code_block_with_the_wrong_language_becomes_unknown_code_block() {
        let mut e = engine(EngineKind::CodeblockLangMismatch, Some(30));
        e.raw = Some("```python".to_string());
        let f = mapped(e);
        assert_eq!(f.kind, FindingKind::UnknownCodeBlock);
        assert_eq!(f.line, Some(30));
        assert_eq!(f.detail, "```python");
    }

    // @kotowari[REQ-core-171]
    #[test]
    fn a_statement_below_its_minimum_splits_by_whether_the_finding_has_a_line() {
        let mut e = engine(EngineKind::RepeatMinNotMet, None);
        e.rule_kind = Some(RuleKind::Statement);
        let f = mapped(e);
        assert_eq!(f.kind, FindingKind::MissingScope);
        assert_eq!(f.line, None);
        assert_eq!(f.detail, "a.md");

        let mut e = engine(EngineKind::RepeatMinNotMet, Some(20));
        e.rule_kind = Some(RuleKind::Statement);
        let f = mapped(e);
        assert_eq!(f.kind, FindingKind::MissingStatement);
        assert_eq!(f.line, Some(20));
        assert_eq!(f.detail, "REQ-core-002");
    }

    // @kotowari[REQ-core-171]
    #[test]
    fn a_table_below_its_minimum_splits_by_whether_the_finding_has_a_line() {
        let mut e = engine(EngineKind::RepeatMinNotMet, None);
        e.rule_kind = Some(RuleKind::Table);
        let f = mapped(e);
        assert_eq!(f.kind, FindingKind::GlossaryInvalid);
        assert_eq!(f.line, None);
        assert_eq!(f.detail, "a.md");

        let mut e = engine(EngineKind::RepeatMinNotMet, Some(10));
        e.rule_kind = Some(RuleKind::Table);
        let f = mapped(e);
        assert_eq!(f.kind, FindingKind::MissingTable);
        assert_eq!(f.line, Some(10));
        assert_eq!(f.detail, "REQ-core-001");
    }

    // @kotowari[REQ-core-171]
    #[test]
    fn a_field_line_above_its_maximum_becomes_duplicate_field_with_the_field_name() {
        let mut e = engine(EngineKind::RepeatMaxExceeded, Some(11)).of_node("source");
        e.rule_kind = Some(RuleKind::Field);
        let f = mapped(e);
        assert_eq!(f.kind, FindingKind::DuplicateField);
        assert_eq!(f.line, Some(11));
        assert_eq!(f.detail, "source");
    }

    // --- REQ-core-172: 写し先の無い指摘は停止する ---

    // @kotowari[REQ-core-172, EX-core-265]
    #[test]
    fn a_kind_whose_row_says_it_cannot_happen_stops() {
        let rows: &[(EngineKind, Option<RuleKind>)] = &[
            (EngineKind::MissingRequiredSection, None),
            (EngineKind::MissingStatement, None),
            (EngineKind::MissingBullets, None),
            (EngineKind::MissingTable, None),
            (EngineKind::MissingCodeblock, None),
            (EngineKind::FieldPatternMismatch, None),
            (EngineKind::StatementPatternMismatch, None),
            (EngineKind::StatementEnumInvalid, None),
            (EngineKind::BulletPatternMismatch, None),
            (EngineKind::FieldOrderMismatch, None),
            (EngineKind::CodeblockLineMismatch, None),
            (EngineKind::RepeatMinNotMet, Some(RuleKind::Field)),
            (EngineKind::RepeatMaxExceeded, Some(RuleKind::Bullets)),
        ];
        for (kind, rule) in rows {
            let mut e = engine(*kind, Some(4));
            e.rule_kind = *rule;
            let detail = stopped(DocKind::Topic, e);
            assert!(
                detail.starts_with("mapping error: no mapping for"),
                "{kind:?} は停止する: {detail}"
            );
        }
    }

    // @kotowari[REQ-core-172, EX-core-265]
    #[test]
    fn a_kind_whose_row_is_chosen_by_a_name_it_does_not_have_stops() {
        // 用語集のスキーマ以外に題名のパターンは無いので、対応表に行が無い
        let mut e = engine(EngineKind::TitlePatternMismatch, Some(1));
        e.raw = Some("# 題名".to_string());
        assert!(stopped(DocKind::Topic, e).contains("no mapping for title_pattern_mismatch"));

        // 表のヘッダの食い違いの行は用語集にしか無い
        let mut e = engine(EngineKind::TableHeaderMismatch, Some(4));
        e.raw = Some("| a | b |".to_string());
        assert!(stopped(DocKind::Topic, e).contains("no mapping for table_header_mismatch"));

        let e = engine(EngineKind::MissingRequiredField, Some(10)).of_node("知らない名前");
        let detail = stopped(DocKind::Topic, e);
        assert!(
            detail.contains("no mapping for missing_required_field 知らない名前"),
            "停止の詳細は写せなかった種類とノードの名前を持つ: {detail}"
        );
    }

    // @kotowari[REQ-core-172]
    #[test]
    fn a_table_header_mismatch_outside_the_glossary_stops_even_on_a_glossary_row_line() {
        // invalid_glossary_row の行は用語集の文書にだけある。用語集でない文書では、
        // その行番号が用語集の表のデータ行と同じでも写し先が無い
        for doc_kind in [DocKind::Topic, DocKind::Flags] {
            let mut e = engine(EngineKind::TableHeaderMismatch, Some(5));
            e.raw = Some("| a |".to_string());
            let detail = stopped(doc_kind, e);
            assert!(
                detail.contains("no mapping for table_header_mismatch"),
                "{doc_kind:?}: {detail}"
            );
        }
    }

    // --- REQ-core-175: 写せない値は停止する ---

    // @kotowari[REQ-core-175, EX-core-268]
    #[test]
    fn a_value_that_cannot_be_mapped_stops_with_the_mapping_error_wording() {
        // 生の行を要る写しに生の行が無い
        let e = engine(EngineKind::UndeclaredHeading, Some(7));
        let detail = stopped(DocKind::Topic, e);
        assert!(
            detail.starts_with("mapping error: "),
            "停止の1行目の文言は mapping error: {detail}"
        );
        assert!(detail.contains("carries no raw line"), "{detail}");

        // 規則種別を要る写しに規則種別が無い
        let mut e = engine(EngineKind::UndeclaredLine, Some(7));
        e.raw = Some("行".to_string());
        assert!(stopped(DocKind::Topic, e).contains("carries no rule kind"));

        // 項目の値を要る写しに、その行を含む項目が無い
        let e = engine(EngineKind::MissingRequiredField, Some(2)).of_node("source");
        assert!(stopped(DocKind::Topic, e).contains("has no item to read"));
    }

    // @kotowari[REQ-core-171]
    #[test]
    fn the_item_of_a_finding_is_the_last_one_that_starts_at_or_above_its_line() {
        let ctx = ctx(DocKind::Topic);
        assert_eq!(ctx.item_at(9), None);
        assert_eq!(ctx.item_at(10).map(|i| i.id.as_str()), Some("REQ-core-001"));
        assert_eq!(ctx.item_at(19).map(|i| i.id.as_str()), Some("REQ-core-001"));
        assert_eq!(ctx.item_at(20).map(|i| i.id.as_str()), Some("REQ-core-002"));
        assert_eq!(ctx.item_at(99).map(|i| i.id.as_str()), Some("REQ-core-002"));
    }
}
