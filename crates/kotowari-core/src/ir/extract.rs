//! スキーマの`抽出`から IR 文書と`項目`を組み立てる（REQ-core-169、REQ-core-170）

use super::*;

/// 文書1つを読む。形の検査と値の取り出しはスキーマの側が行い、kotowari はその`抽出`から
/// `項目`を組み立てる（REQ-core-169、REQ-core-170）。写せない`指摘`や値は`停止`になる
/// （REQ-core-172、REQ-core-175）
pub fn parse_document(filename: &str, content: &str) -> Result<IrDocument, StopReason> {
    let name = filename.rsplit('/').next().unwrap_or(filename);
    parse_side(filename, DocKind::of(name), content)
}

/// 文書の種類を決めて文書1つを読む。`先頭の言語`でない言語の`側`の種類は、その`対`の
/// `先頭の言語`の`側`の名前で決まる（REQ-core-033）。path は`IR`の置き場からの相対パス
pub(crate) fn parse_side(
    path: &str,
    kind: DocKind,
    content: &str,
) -> Result<IrDocument, StopReason> {
    let (directory, name) = path.rsplit_once('/').unwrap_or(("", path));
    let mut document = parse_document_mode(name, kind, content, false)?;
    document.relative_path = path.into();
    document.directory = directory.into();
    Ok(document)
}

pub(super) fn parse_document_mode(
    filename: &str,
    kind: DocKind,
    content: &str,
    partial: bool,
) -> Result<IrDocument, StopReason> {
    // BOM の読み飛ばし（read_utf8_file でも除去するが、直接呼ばれた場合にも対応）
    let content = content.strip_prefix('\u{FEFF}').unwrap_or(content);
    let (values, mut findings) = read_document(filename, kind, content)?;
    let mut items = extract_items(kind, &values, &mut findings, partial)?;
    diagnose_nameless_items(&items, &mut findings, content);
    if !partial {
        items.retain(|item| !is_nameless(item));
    }
    items.sort_by_key(Item::item_line);
    exclude_unclosed_code_block(&mut items, &mut findings, content);

    Ok(IrDocument {
        relative_path: filename.to_string(),
        directory: String::new(),
        kind,
        scope_lines: statements_of(values.get("scope"))?,
        deferred: deferral(values.get("deferred"))?,
        line_count: split_lines(content).len(),
        items,
        raw_content: content.to_string(),
        parse_findings: findings,
    })
}

pub(super) fn extract_items(
    kind: DocKind,
    values: &Value,
    findings: &mut Vec<Finding>,
    partial: bool,
) -> Result<Vec<Item>, StopReason> {
    let mut items = Vec::new();
    match kind {
        DocKind::Topic => {
            for obj in elements(values.get("requirements")) {
                items.extend(requirement(obj, findings, partial)?);
            }
            for obj in elements(values.get("tables")) {
                items.extend(decision_table(obj, partial)?);
            }
            for obj in elements(values.get("properties")) {
                items.extend(property(obj, findings, partial)?);
            }
            for obj in elements(values.get("scenarios")) {
                let opening = number(obj, "line")?;
                // REQ-core-174: gherkin でないブロックはスキーマの側が unknown_code_block にしている。
                // その中の行は gherkin として読まない
                let not_gherkin = findings
                    .iter()
                    .any(|f| f.kind == FindingKind::UnknownCodeBlock && f.line == Some(opening));
                if not_gherkin {
                    continue;
                }
                let content = string(obj, "value").unwrap_or_default();
                GherkinBlock::read(opening, &content, &mut items, findings);
            }
        }
        DocKind::Flags => {
            // 問題の記録の項目は文書の直下（"flags"）と節の下（"flags_in_section"）の両方にある。
            // 並びは下の行の順の並べ替えで揃う（TBL-core-011）
            let direct = elements(values.get("flags"));
            let in_section = elements(values.get("flags_in_section"));
            for obj in direct.into_iter().chain(in_section) {
                items.extend(flag_entry(obj, findings, partial)?);
            }
        }
        DocKind::Glossary => read_glossary(values, &mut items, findings)?,
    }
    Ok(items)
}

pub(super) fn is_nameless(item: &Item) -> bool {
    heading_name(item).is_some_and(|name| name.trim().is_empty())
}

pub(super) fn diagnose_nameless_items(items: &[Item], findings: &mut Vec<Finding>, content: &str) {
    // REQ-core-043: "### ID:" の後に名前の無い見出しは "### ID: 名前" の形でない。
    // その ID は定義に数えず、見出しの行を detail にした unknown_heading にする（review8-gaps の A2）
    let lines = split_lines(content);
    for item in items {
        let nameless = is_nameless(item);
        let line = item.item_line();
        // コロンの無い見出しは、スキーマの側が既に unknown_heading にしている
        let flagged = findings
            .iter()
            .any(|f| f.kind == FindingKind::UnknownHeading && f.line == Some(line));
        if nameless && !flagged {
            let raw = lines.get(line - 1).copied().unwrap_or_default();
            findings.push(Finding::new(
                FindingKind::UnknownHeading,
                String::new(),
                Some(line),
                raw.to_string(),
            ));
        }
    }
}

pub(super) fn exclude_unclosed_code_block(
    items: &mut Vec<Item>,
    findings: &mut Vec<Finding>,
    content: &str,
) {
    // REQ-core-112: 閉じないコードブロックは、開始から文書の終わりまでを検査の対象から外す
    if let Some((opening, raw)) = unclosed_code_block(content) {
        let before_opening = |line: usize| line < opening;
        findings.retain(|f| f.line.is_none_or(before_opening));
        items.retain(|item| before_opening(item.item_line()));
        findings.push(Finding::new(
            FindingKind::UnclosedCodeBlock,
            String::new(),
            Some(opening),
            raw,
        ));
    }
}

/// 見出しで始まる`項目`の見出しの名前。見出しを持たない`シナリオ`と`用語`は None
pub(super) fn heading_name(item: &Item) -> Option<&str> {
    match item {
        Item::Requirement { name, .. }
        | Item::DecisionTable { name, .. }
        | Item::Property { name, .. }
        | Item::FlagEntry { name, .. } => Some(name),
        Item::Scenario { .. } | Item::GlossaryTerm { .. } => None,
    }
}

// --- `抽出`の値から型へ写す ---

/// 抽出の値を型へ写せない（A20、REQ-core-175）
pub(super) fn unmappable(what: &str) -> StopReason {
    StopReason::MappingError(format!("extracted value has no {what}"))
}

/// 抽出の値の並び。IR のスキーマはどの要素にも`出現回数`を宣言しているので、
/// 要素の値は1つだけのときも配列で来る（TBL-schema-008）
pub(super) fn elements(value: Option<&Value>) -> Vec<&Map<String, Value>> {
    value
        .and_then(Value::as_array)
        .map(|values| values.iter().filter_map(Value::as_object).collect())
        .unwrap_or_default()
}

pub(super) fn string(obj: &Map<String, Value>, key: &str) -> Option<String> {
    obj.get(key).and_then(Value::as_str).map(str::to_string)
}

pub(super) fn number(obj: &Map<String, Value>, key: &str) -> Result<usize, StopReason> {
    obj.get(key)
        .and_then(Value::as_u64)
        .map(|n| n as usize)
        .ok_or_else(|| unmappable(key))
}

/// `項目`の`ID`・名前・見出しの行
pub(super) fn head(obj: &Map<String, Value>) -> Result<(String, String, usize), StopReason> {
    let id = string(obj, "id").ok_or_else(|| unmappable("id"))?;
    let name = string(obj, "name").ok_or_else(|| unmappable("name"))?;
    Ok((id, name, number(obj, "line")?))
}

/// "," で分けた`フィールド行`の値と、その行。行が無ければ空の並びと None
pub(super) fn listed(value: Option<&Value>) -> Result<(Vec<String>, Option<usize>), StopReason> {
    let Some(obj) = value.and_then(Value::as_object) else {
        return Ok((Vec::new(), None));
    };
    // 区切り文字を宣言したフィールド行の値は常に配列（REQ-schema-045）
    let values = obj
        .get("value")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .flat_map(|value| value.split(','))
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .collect();
    Ok((values, Some(number(obj, "line")?)))
}

/// "- deferred:" の行。行が無ければ None
pub(super) fn deferral(value: Option<&Value>) -> Result<Option<Deferral>, StopReason> {
    let (sources, line) = listed(value)?;
    Ok(line.map(|line| Deferral { sources, line }))
}

/// `文`の行の並び（行番号と行の文字そのまま）
pub(super) fn statements_of(value: Option<&Value>) -> Result<Vec<(usize, String)>, StopReason> {
    elements(value)
        .into_iter()
        .map(|obj| {
            let raw = string(obj, "raw").ok_or_else(|| unmappable("raw"))?;
            Ok((number(obj, "line")?, raw))
        })
        .collect()
}

/// `項目`の`文`。gherkin のブロックの外の "Scenario:" の行は`文`にしない（REQ-core-100、`除外`）。
/// スキーマの側はその行も`文`に数えるので、それを除くと`文`が無くなる`項目`には
/// missing_statement を足す（REQ-core-047）
pub(super) fn item_statements(
    obj: &Map<String, Value>,
    requires_statement: bool,
    id: &str,
    line: usize,
    findings: &mut Vec<Finding>,
) -> Result<Vec<(usize, String)>, StopReason> {
    let all = statements_of(obj.get("text"))?;
    let kept: Vec<(usize, String)> = all
        .iter()
        .filter(|(_, text)| !text.trim_start().starts_with("Scenario:"))
        .cloned()
        .collect();
    if requires_statement && kept.is_empty() && !all.is_empty() {
        findings.push(Finding::new(
            FindingKind::MissingStatement,
            String::new(),
            Some(line),
            id.to_string(),
        ));
    }
    Ok(kept)
}

pub(super) fn requirement(
    obj: &Map<String, Value>,
    findings: &mut Vec<Finding>,
    partial: bool,
) -> Result<Option<Item>, StopReason> {
    let (id, name, line) = head(obj)?;
    if !partial && !is_item_id(&id, IdPrefix::Req) {
        return Ok(None);
    }
    let (sources, source_line) = listed(obj.get("sources"))?;
    let (definitions, definition_line) = listed(obj.get("definitions"))?;
    let kind = string(obj, "kind");
    let requires_statement = kind.as_deref() != Some("algorithm");
    let statements = item_statements(obj, requires_statement, &id, line, findings)?;
    Ok(Some(Item::Requirement {
        id,
        name,
        line,
        end: number(obj, "end")?,
        kind,
        sources,
        source_line,
        verification: string(obj, "verification"),
        definitions,
        definition_line,
        how_to_verify: string(obj, "how_to_verify"),
        deferred: deferral(obj.get("deferred"))?,
        statements,
    }))
}

pub(super) fn decision_table(
    obj: &Map<String, Value>,
    partial: bool,
) -> Result<Option<Item>, StopReason> {
    let (id, name, line) = head(obj)?;
    if !partial && !is_item_id(&id, IdPrefix::Tbl) {
        return Ok(None);
    }
    let (sources, source_line) = listed(obj.get("sources"))?;
    let statements = item_statements(obj, false, &id, line, &mut Vec::new())?;
    Ok(Some(Item::DecisionTable {
        id,
        name,
        line,
        end: number(obj, "end")?,
        sources,
        source_line,
        statements,
    }))
}

pub(super) fn property(
    obj: &Map<String, Value>,
    findings: &mut Vec<Finding>,
    partial: bool,
) -> Result<Option<Item>, StopReason> {
    let (id, name, line) = head(obj)?;
    if !partial && !is_item_id(&id, IdPrefix::Prop) {
        return Ok(None);
    }
    let (sources, source_line) = listed(obj.get("sources"))?;
    let statements = item_statements(obj, true, &id, line, findings)?;
    Ok(Some(Item::Property {
        id,
        name,
        line,
        end: number(obj, "end")?,
        sources,
        source_line,
        statements,
    }))
}

pub(super) fn flag_entry(
    obj: &Map<String, Value>,
    findings: &mut Vec<Finding>,
    partial: bool,
) -> Result<Option<Item>, StopReason> {
    let (id, name, line) = head(obj)?;
    if !partial && !is_item_id(&id, IdPrefix::Flag) {
        return Ok(None);
    }
    let (relations, relation_line) = listed(obj.get("relations"))?;
    let (sources, source_line) = listed(obj.get("sources"))?;
    let body = item_statements(obj, true, &id, line, findings)?;
    Ok(Some(Item::FlagEntry {
        id,
        name,
        line,
        end: number(obj, "end")?,
        kind: string(obj, "kind"),
        relations,
        relation_line,
        sources,
        source_line,
        body,
    }))
}

/// `用語集`の表の行から`用語`を組み立てる（REQ-core-117、REQ-core-122、REQ-core-123）。
/// 表そのものの誤りと列の数の誤りはスキーマの側が出す。`用語`のセルが空の行と、
/// 同じ文書の中で重なった`用語`は kotowari が見る（REQ-core-173）
pub(super) fn read_glossary(
    values: &Value,
    items: &mut Vec<Item>,
    findings: &mut Vec<Finding>,
) -> Result<(), StopReason> {
    // REQ-core-117: 表の形が無ければ、その用語集の用語は0語
    if findings
        .iter()
        .any(|f| f.kind == FindingKind::GlossaryInvalid)
    {
        return Ok(());
    }
    // REQ-core-122: 列の数の合わない行は用語にしない
    let invalid_rows: BTreeSet<usize> = findings
        .iter()
        .filter(|f| f.kind == FindingKind::InvalidGlossaryRow)
        .filter_map(|f| f.line)
        .collect();
    // REQ-core-117: 用語になるのは最初の表の行だけ。用語集のスキーマは表に`出現回数`を
    // 宣言しているので、表ごとの段が必ずできる（TBL-schema-008）
    let rows = elements(values.get("glossary").and_then(|tables| tables.get(0)));
    let mut seen_terms = BTreeSet::new();
    for row in rows {
        let line = number(row, "line")?;
        if invalid_rows.contains(&line) {
            continue;
        }
        let cells = row.get("value").and_then(Value::as_object);
        let cell = |name: &str| {
            cells
                .and_then(|cells| string(cells, name))
                .unwrap_or_default()
        };
        let term = cell("Term");
        if term.is_empty() {
            let raw = string(row, "raw").ok_or_else(|| unmappable("raw"))?;
            findings.push(Finding::new(
                FindingKind::InvalidGlossaryRow,
                String::new(),
                Some(line),
                raw,
            ));
        } else if !seen_terms.insert(term.clone()) {
            // REQ-core-123: 同じ用語の2つ目以降の行は duplicate_term だけを出し、用語にしない
            findings.push(Finding::new(
                FindingKind::DuplicateTerm,
                String::new(),
                Some(line),
                term,
            ));
        } else {
            let sources = comma_list(&cell("Source"));
            items.push(Item::GlossaryTerm {
                term,
                meaning: cell("Meaning"),
                sources,
                line,
            });
        }
    }
    Ok(())
}

// --- 生の行を読む2つ: 閉じないコードブロックと gherkin の中身 ---

/// "," で分けたタグの値の、空でない要素
pub(super) fn comma_list(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}
