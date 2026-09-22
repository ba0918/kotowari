//! スキーマの `extract` に沿って値を組み立てる。

use crate::document::{
    Block, Document, Item as DocItem, RawLine, Section as DocSection, join_continuation,
};
use crate::schema::{
    Bullets, Children, CodeBlock, Extract, Field, OfKind, Schema, Statement, Table,
    is_declared_field, item_internals_declare_extract,
};
use serde_json::{Map, Value};

/// `values` の出力。配置パスに沿った入れ子の JSON。
pub fn extract_values(schema: &Schema, document: &Document) -> Value {
    let mut root = Map::new();
    extract_title(schema, document, &mut root);
    if let Some(preamble) = &schema.document.preamble {
        let blocks: Vec<&Block> = document.preamble.iter().collect();
        extract_fields(&preamble.fields, &blocks, document, &mut root);
        extract_statement(preamble.statement.as_ref(), &blocks, document, &mut root);
        extract_bullets(
            &preamble.fields,
            preamble.bullets.as_ref(),
            &blocks,
            document,
            &mut root,
        );
        extract_table(preamble.table.as_ref(), &blocks, document, &mut root);
        extract_codeblock(preamble.codeblock.as_ref(), &blocks, document, &mut root);
    }
    for def in &schema.document.sections {
        extract_section(def, document, &mut root);
    }
    Value::Object(root)
}

/// `ast --schema` の出力。抽出値に、スキーマの `name` を `type` として添える。
pub fn extract_typed(schema: &Schema, document: &Document) -> Value {
    let values = extract_values(schema, document);
    let Value::Object(values) = values else {
        return values;
    };
    let mut out = Map::new();
    if let Some(name) = &schema.name {
        out.insert("type".to_string(), Value::String(name.clone()));
    }
    for (key, value) in values {
        out.insert(key, value);
    }
    Value::Object(out)
}

fn extract_title(schema: &Schema, document: &Document, root: &mut Map<String, Value>) {
    let Some(title) = &schema.document.title else {
        return;
    };
    let Some(extract) = &title.extract else {
        return;
    };
    let Some(heading) = document.titles.first() else {
        return;
    };
    // 名前付きキャプチャを宣言した題名の要素の値は、題名の文字ではなく
    // pattern が捕まえた部分である（R16）
    let value = match extract.group() {
        Some(group) => {
            let Some(captured) = title
                .pattern
                .as_ref()
                .and_then(|p| p.captures(&heading.text))
                .and_then(|caps| caps.name(group).map(|m| m.as_str().to_string()))
            else {
                return;
            };
            Value::String(captured)
        }
        None => Value::String(heading.text.clone()),
    };
    let element = element_value(extract, document, value, &Derived::at(heading.line));
    place(root, extract.path(), element);
}

fn extract_section(
    def: &crate::schema::Section,
    document: &Document,
    root: &mut Map<String, Value>,
) {
    let occurrences: Vec<&DocSection> = document
        .sections
        .iter()
        .filter(|s| s.name == def.name)
        .collect();

    if let Some(extract) = &def.extract {
        // 節は要素に分けない。繰り返すときは配置パスの直下に段ができる（R17）
        let elements: Vec<Value> = occurrences
            .iter()
            .map(|s| {
                element_value(
                    extract,
                    document,
                    Value::String(section_body(s, def)),
                    &Derived::at(s.line),
                )
            })
            .collect();
        place_occurrences(root, extract, elements, def.repeat.is_some());
    }

    let blocks: Vec<&Block> = occurrences.iter().flat_map(|s| s.blocks.iter()).collect();
    extract_fields(&def.fields, &blocks, document, root);
    extract_statement(def.statement.as_ref(), &blocks, document, root);
    extract_bullets(&def.fields, def.bullets.as_ref(), &blocks, document, root);
    extract_table(def.table.as_ref(), &blocks, document, root);
    extract_codeblock(def.codeblock.as_ref(), &blocks, document, root);

    if let Some(item) = &def.item
        && let Some(extract) = &item.extract
    {
        let items: Vec<&DocItem> = occurrences.iter().flat_map(|s| s.items.iter()).collect();
        if items.is_empty() {
            // 0件のときはキーを省略する（R16）
        } else {
            // 内部が extract を宣言したか、項目が value か of を宣言していれば
            // 項目ごとのオブジェクトにする（R16）
            let as_object = item_internals_declare_extract(item)
                || extract.has_of()
                || extract.value().is_some();
            let one = |i: &DocItem| {
                if as_object {
                    item_object(i, item, document)
                } else {
                    item_value(i, item)
                }
            };
            let value = if item.repeat.is_some() {
                Value::Array(items.iter().map(|i| one(i)).collect())
            } else {
                one(items[0])
            };
            place(root, extract.path(), value);
        }
    }
}

fn extract_fields(
    fields: &[Field],
    blocks: &[&Block],
    doc: &Document,
    root: &mut Map<String, Value>,
) {
    for field in fields {
        if let Some(extract) = &field.extract {
            let occurrences: Vec<&Block> = blocks
                .iter()
                .copied()
                .filter(|b| matches!(b, Block::Field { name, .. } if name == &field.name))
                .collect();
            // フィールド行は要素に分けない。繰り返すときは配置パスの直下に段ができる（R17）
            let elements: Vec<Value> = occurrences
                .iter()
                .map(|b| {
                    element_value(extract, doc, field_single(field, b), &Derived::at(b.line()))
                })
                .collect();
            place_occurrences(root, extract, elements, field.repeat.is_some());
        }
    }
}

fn field_single(field: &Field, block: &Block) -> Value {
    let (value, continuation) = match block {
        Block::Field {
            value,
            continuation,
            ..
        } => (value.as_str(), continuation.as_slice()),
        _ => ("", &[][..]),
    };
    match field.effective_separator() {
        // 区切った要素は前後の空白を取り除いて抽出し、空の要素は空文字列として残す（R8）。
        // 分割は継続段落を含めない値だけを対象にし、継続段落は末尾の要素に改行で付ける（R8・R16）
        Some(sep) => {
            let mut elements: Vec<String> =
                value.split(sep).map(|s| s.trim().to_string()).collect();
            if let Some(last) = elements.last_mut() {
                join_continuation(last, continuation);
            }
            Value::Array(elements.into_iter().map(Value::String).collect())
        }
        None => {
            // 継続段落はフィールド行の一部で、値と改行でつなぐ（R8・R16）。複数あるときは
            // 箇条書きと同じく空行でつなぐ（R10）
            let mut full = value.to_string();
            join_continuation(&mut full, continuation);
            Value::String(full)
        }
    }
}

fn extract_statement(
    statement: Option<&Statement>,
    blocks: &[&Block],
    doc: &Document,
    root: &mut Map<String, Value>,
) {
    let Some(statement) = statement else {
        return;
    };
    let Some(extract) = &statement.extract else {
        return;
    };
    let statements: Vec<&Block> = blocks
        .iter()
        .copied()
        .filter(|b| matches!(b, Block::Statement { .. }))
        .collect();
    if statements.is_empty() {
        // 0件のときはキーを省略する（R16）
        return;
    }
    if extract.has_of() {
        // 導かれる値を宣言したときは行が要素の単位になる（R2）。空行で区切って
        // 続く段落も同じ並びに入り、出現回数の宣言では入れ子にしない
        let elements: Vec<Value> = statements
            .iter()
            .flat_map(|b| match b {
                Block::Statement { raw_lines, .. } => raw_lines.as_slice(),
                _ => &[],
            })
            .map(|raw| {
                element_value(
                    extract,
                    doc,
                    Value::String(statement_line_text(raw)),
                    &Derived::at(raw.line),
                )
            })
            .collect();
        place(root, extract.path(), Value::Array(elements));
        return;
    }
    // 導かれる値を宣言しない文は行に分けない。繰り返すときは配置パスの
    // 直下に段ができる（R2・R17）
    let texts: Vec<String> = statements
        .iter()
        .map(|b| match b {
            Block::Statement { text, .. } => text.clone(),
            _ => String::new(),
        })
        .collect();
    if statement.repeat.is_some() {
        let elements: Vec<Value> = statements
            .iter()
            .zip(texts)
            .map(|(b, text)| {
                element_value(extract, doc, Value::String(text), &Derived::at(b.line()))
            })
            .collect();
        place(root, extract.path(), Value::Array(elements));
        return;
    }
    let element = element_value(
        extract,
        doc,
        Value::String(texts.join("\n\n")),
        &Derived::at(statements[0].line()),
    );
    place(root, extract.path(), element);
}

/// 文の1行の要素の値。
fn statement_line_text(raw: &RawLine) -> String {
    raw.text.clone()
}

fn extract_bullets(
    fields: &[Field],
    bullets: Option<&Bullets>,
    blocks: &[&Block],
    doc: &Document,
    root: &mut Map<String, Value>,
) {
    let Some(bullets) = bullets else {
        return;
    };
    // 子フィールドの抽出は箇条書きの下からだけ行う。宣言済みフィールド行の下は
    // 未宣言の構造（R13）で、そこにある子フィールド名の行を拾うと正当な子
    // フィールドの値を覆い隠す。宣言された名前と一致しない `- 名前: 値` 行は
    // 箇条書きとして扱う（R8）。
    let bullet_blocks: Vec<&Block> = blocks
        .iter()
        .copied()
        .filter(|b| {
            matches!(b, Block::Bullet { .. })
                || matches!(b, Block::Field { name, .. } if !is_declared_field(fields, name))
        })
        .collect();
    let Some(extract) = &bullets.extract else {
        // 子フィールドの抽出は、親の抽出が無くても子フィールド自身の extract に
        // 沿って行う（A15）
        if let Some(children) = &bullets.children {
            extract_child_fields(children, &bullet_blocks, doc, root);
        }
        return;
    };
    // 箇条書きは行が要素の単位。要素の値は元のマーカー行（元のマーカーを保つ）と
    // 子の箇条書きの行をそのままのインデントで含めた文字列（R10・R17）
    let elements: Vec<Value> = bullet_blocks
        .iter()
        .copied()
        .map(|b| {
            element_value(
                extract,
                doc,
                Value::String(element_with_children(b, bullets.children.as_ref())),
                &Derived::at(b.line()),
            )
        })
        .collect();
    if elements.is_empty() {
        // 0件のときはキーを省略する（R16）
        return;
    }
    place(root, extract.path(), Value::Array(elements));
    // 子フィールドは自身の extract を持てば、その配置パスに値を出す（A15）
    if let Some(children) = &bullets.children {
        extract_child_fields(children, &bullet_blocks, doc, root);
    }
}

/// 子フィールドの抽出。`children.fields` で宣言された子フィールドのうち、自身の
/// `extract` を持つものをその配置パスに置く（R10・R16）。さらに深い入れ子の
/// 子フィールドは、子の箇条書きの `children` の宣言に沿って再帰する。
fn extract_child_fields(
    children: &Children,
    blocks: &[&Block],
    doc: &Document,
    root: &mut Map<String, Value>,
) {
    for field in &children.fields {
        if let Some(extract) = &field.extract {
            let occurrences: Vec<&Block> = blocks
                .iter()
                .flat_map(|b| b.children().iter())
                .filter(|c| matches!(c, Block::Field { name, .. } if name == &field.name))
                .collect();
            let elements: Vec<Value> = occurrences
                .iter()
                .map(|b| {
                    element_value(extract, doc, field_single(field, b), &Derived::at(b.line()))
                })
                .collect();
            place_occurrences(root, extract, elements, field.repeat.is_some());
        }
    }
    // 子の箇条書きの下の children に再帰する。子の箇条書きは Bullet か、この
    // レベルの宣言と一致しない `- 名前: 値` 行（箇条書きとして扱う行）である（R8）
    if let Some(child_bullets) = children.bullets.as_deref()
        && let Some(grandchildren) = &child_bullets.children
    {
        let declared: Vec<&str> = children.fields.iter().map(|f| f.name.as_str()).collect();
        let bullet_blocks: Vec<&Block> = blocks
            .iter()
            .flat_map(|b| b.children().iter())
            .filter(|c| {
                matches!(c, Block::Bullet { .. })
                    || matches!(
                        c,
                        Block::Field { name, .. } if !declared.contains(&name.as_str())
                    )
            })
            .collect();
        extract_child_fields(grandchildren, &bullet_blocks, doc, root);
    }
}

/// 箇条書きの抽出要素を、子の箇条書きの行を含めて組み立てる（R10・R16）。
/// 元の行、継続段落、子の箇条書きの行を改行でつなぐ。子の箇条書きの行は
/// そのままのインデントで含める。`children` の宣言でフィールド行として宣言された
/// 子の行は含めない（A12）。`children` が無ければ子はすべて箇条書きとして含める。
fn element_with_children(block: &Block, children: Option<&Children>) -> String {
    let base = match block {
        Block::Bullet { .. } => block.bullet_element(),
        Block::Field { .. } => block.field_element(),
        _ => return String::new(),
    };
    let child_blocks = block.children();
    if child_blocks.is_empty() {
        return base;
    }
    let declared: Vec<&str> = children
        .map(|c| c.fields.iter().map(|f| f.name.as_str()).collect())
        .unwrap_or_default();
    let child_rule = children.and_then(|c| c.bullets.as_deref());
    let child_lines: Vec<String> = child_blocks
        .iter()
        .filter_map(|child| match child {
            Block::Bullet { .. } => Some(element_with_children(
                child,
                child_rule.and_then(|b| b.children.as_ref()),
            )),
            Block::Field { name, .. } if !declared.contains(&name.as_str()) => Some(
                element_with_children(child, child_rule.and_then(|b| b.children.as_ref())),
            ),
            _ => None,
        })
        .collect();
    if child_lines.is_empty() {
        base
    } else {
        base + "\n" + &child_lines.join("\n")
    }
}

fn extract_table(
    table: Option<&Table>,
    blocks: &[&Block],
    doc: &Document,
    root: &mut Map<String, Value>,
) {
    let Some(table) = table else {
        return;
    };
    let Some(extract) = &table.extract else {
        return;
    };
    let tables: Vec<&Block> = blocks
        .iter()
        .copied()
        .filter(|b| matches!(b, Block::Table { .. }))
        .collect();
    if tables.is_empty() {
        // 0件のときはキーを省略する（R16）
        return;
    }
    // 表はデータ行が要素の単位（R17）
    let per_table: Vec<Vec<Value>> = tables
        .iter()
        .map(|b| table_rows(b, table.header.as_deref(), extract, doc))
        .collect();
    let value = if table.repeat.is_some() && declares_element_object(extract) {
        // 繰り返す表は配置パスの直下に表ごとの段を作る（R1）
        Value::Array(per_table.into_iter().map(Value::Array).collect())
    } else {
        // 複数の表は現れた順に1つの配列へ連結する（R16）
        Value::Array(per_table.into_iter().flatten().collect())
    };
    place(root, extract.path(), value);
}

/// 表のデータ行を要素にする。`header` はスキーマが宣言した列の名前。
fn table_rows(
    block: &Block,
    header: Option<&[String]>,
    extract: &Extract,
    doc: &Document,
) -> Vec<Value> {
    let Block::Table {
        rows, row_lines, ..
    } = block
    else {
        return Vec::new();
    };
    rows.iter()
        .enumerate()
        .map(|(row_index, row)| {
            let line = row_lines.get(row_index).copied().unwrap_or_default();
            element_value(extract, doc, row_value(header, row), &Derived::at(line))
        })
        .collect()
}

/// 表の1行の要素の値。鍵はスキーマが宣言した `header` の名前で、宣言が
/// 無ければ列の位置（配列）にする。文書のヘッダ行の文字は鍵に使わない（R1）。
/// 宣言した名前の数を正とし、文書の列が足りなければその鍵を省き、
/// 多ければ余りを捨てる。
fn row_value(header: Option<&[String]>, row: &[String]) -> Value {
    let Some(header) = header else {
        return Value::Array(row.iter().map(|cell| Value::String(cell.clone())).collect());
    };
    let mut cells = Map::new();
    for (name, cell) in header.iter().zip(row) {
        cells.insert(name.clone(), Value::String(cell.clone()));
    }
    Value::Object(cells)
}

fn extract_codeblock(
    codeblock: Option<&CodeBlock>,
    blocks: &[&Block],
    doc: &Document,
    root: &mut Map<String, Value>,
) {
    let Some(codeblock) = codeblock else {
        return;
    };
    let Some(extract) = &codeblock.extract else {
        return;
    };
    let codes: Vec<&Block> = blocks
        .iter()
        .copied()
        .filter(|b| matches!(b, Block::Code { .. }))
        .collect();
    // コードブロックはブロックが要素の単位で、行はフェンスの開始行（R17）
    let elements: Vec<Value> = codes
        .iter()
        .map(|b| {
            let value = match b {
                Block::Code { value, .. } => Value::String(value.clone()),
                _ => Value::Null,
            };
            element_value(extract, doc, value, &Derived::at(b.line()))
        })
        .collect();
    place_occurrences(root, extract, elements, codeblock.repeat.is_some());
}

fn section_body(section: &DocSection, def: &crate::schema::Section) -> String {
    body_from_blocks(&section.blocks, &def.fields, def.bullets.as_ref(), false)
}

/// 項目をオブジェクトに組み立てる。内部のノードの抽出は項目オブジェクトの中の
/// 相対パスへ置き、項目自身の `of` を添えた書式も同じオブジェクトの中へ置く（R16）。
fn item_object(item: &DocItem, item_rule: &crate::schema::Item, doc: &Document) -> Value {
    let mut object = Map::new();
    let blocks: Vec<&Block> = item.blocks.iter().collect();
    extract_fields(&item_rule.fields, &blocks, doc, &mut object);
    extract_statement(item_rule.statement.as_ref(), &blocks, doc, &mut object);
    extract_bullets(
        &item_rule.fields,
        item_rule.bullets.as_ref(),
        &blocks,
        doc,
        &mut object,
    );
    extract_table(item_rule.table.as_ref(), &blocks, doc, &mut object);
    extract_codeblock(item_rule.codeblock.as_ref(), &blocks, doc, &mut object);
    if let Some(extract) = &item_rule.extract {
        // path は項目オブジェクトそのものの置き場なので、ここでは置かない
        let got = Derived {
            line: Some(item.line),
            id: Some(item.id.clone()),
            name: Some(item.title.clone()),
        };
        if let Some(key) = extract.value() {
            place(
                &mut object,
                key,
                Value::String(item_value_text(item, item_rule)),
            );
        }
        place_derived(&mut object, extract, doc, &got);
    }
    Value::Object(object)
}

fn item_value(item: &DocItem, item_rule: &crate::schema::Item) -> Value {
    Value::String(item_value_text(item, item_rule))
}

/// 項目の要素の値。見出しと本文をつないだ文字列（R17）。
fn item_value_text(item: &DocItem, item_rule: &crate::schema::Item) -> String {
    let heading = if item.title.is_empty() {
        item.id.clone()
    } else {
        format!("{}: {}", item.id, item.title)
    };
    let body = body_from_blocks(
        &item.blocks,
        &item_rule.fields,
        item_rule.bullets.as_ref(),
        true,
    );
    if body.is_empty() {
        heading
    } else {
        format!("{heading}\n{body}")
    }
}

/// 本文を組み立てる。文と箇条書き（必要ならフィールド行も）を含み、
/// 表・コードブロック・項目は含めない。文どうしは空行、それ以外の
/// 隣接（文と箇条書き・文とフィールド行・箇条書きどうしなど）は改行で
/// つなぐ。箇条書きは R10 の抽出要素（継続段落と子の箇条書きの行を含む
/// 文字列）を使う。宣言された名前と一致しない `- 名前: 値` 行は箇条書きとして
/// 本文に含める（R8）。
fn body_from_blocks(
    blocks: &[Block],
    fields: &[Field],
    bullets: Option<&Bullets>,
    include_fields: bool,
) -> String {
    let mut out = String::new();
    let mut last_was_statement = false;
    for block in blocks {
        let (text, is_statement): (String, bool) = match block {
            Block::Statement { text, .. } => (text.clone(), true),
            Block::Bullet { .. } => (
                element_with_children(block, bullets.and_then(|b| b.children.as_ref())),
                false,
            ),
            Block::Field { name, .. } if include_fields && is_declared_field(fields, name) => {
                (block.field_element(), false)
            }
            Block::Field { name, .. } if include_fields || !is_declared_field(fields, name) => (
                element_with_children(block, bullets.and_then(|b| b.children.as_ref())),
                false,
            ),
            _ => continue,
        };
        if !out.is_empty() {
            out.push_str(if is_statement && last_was_statement {
                "\n\n"
            } else {
                "\n"
            });
        }
        out.push_str(&text);
        last_was_statement = is_statement;
    }
    out
}

/// 1つの要素の導かれる値の材料（R17）。
#[derive(Default)]
struct Derived {
    line: Option<usize>,
    id: Option<String>,
    name: Option<String>,
}

impl Derived {
    /// 行から導けるもの（`line` と `raw`）だけを持つ要素。
    fn at(line: usize) -> Self {
        Derived {
            line: Some(line),
            ..Default::default()
        }
    }
}

/// ノードが要素オブジェクトを作る宣言をしているか。`value` も導かれる値も
/// 宣言していなければ、要素そのものが値になる（R16）。
fn declares_element_object(extract: &Extract) -> bool {
    extract.value().is_some() || extract.has_of()
}

/// 1つの要素を、宣言に応じて素の値か要素オブジェクトにする（R16・R17）。
fn element_value(extract: &Extract, doc: &Document, value: Value, got: &Derived) -> Value {
    if !declares_element_object(extract) {
        return value;
    }
    let mut object = Map::new();
    if let Some(key) = extract.value() {
        place(&mut object, key, value);
    }
    place_derived(&mut object, extract, doc, got);
    Value::Object(object)
}

/// 要素に分けないノードの出現を置く。繰り返すノードは配置パスの直下に
/// 段を作り、繰り返さないノードは最初の出現だけを置く（R17）。0件のときは
/// キーを省略する（R16）。
fn place_occurrences(
    root: &mut Map<String, Value>,
    extract: &Extract,
    elements: Vec<Value>,
    repeats: bool,
) {
    if elements.is_empty() {
        return;
    }
    let value = if repeats {
        Value::Array(elements)
    } else {
        elements.into_iter().next().unwrap_or(Value::Null)
    };
    place(root, extract.path(), value);
}

/// 宣言された導かれる値を、その鍵へ置く。`raw` は文書の生の行から取る（R17）。
fn place_derived(map: &mut Map<String, Value>, extract: &Extract, doc: &Document, got: &Derived) {
    for (key, kind) in extract.of() {
        let value = match kind {
            OfKind::Line => got.line.map(Value::from),
            OfKind::Raw => got
                .line
                .and_then(|line| doc.raw_line(line))
                .map(|raw| Value::String(raw.to_string())),
            OfKind::Id => got.id.clone().map(Value::String),
            OfKind::Name => got.name.clone().map(Value::String),
        };
        if let Some(value) = value {
            place(map, key, value);
        }
    }
}

fn place(root: &mut Map<String, Value>, path: &str, value: Value) {
    let keys: Vec<&str> = path.split('.').collect();
    if keys.is_empty() {
        return;
    }
    let mut current = root;
    for key in &keys[..keys.len() - 1] {
        let entry = current
            .entry(key.to_string())
            .or_insert_with(|| Value::Object(Map::new()));
        if !entry.is_object() {
            *entry = Value::Object(Map::new());
        }
        current = entry.as_object_mut().unwrap();
    }
    current.insert(keys[keys.len() - 1].to_string(), value);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::Document;
    use crate::schema::parse_schema;
    use serde_json::json;

    fn values(schema_yaml: &str, doc: &str) -> serde_json::Value {
        let schema = parse_schema(schema_yaml).unwrap();
        let document = Document::parse(doc).unwrap();
        extract_values(&schema, &document)
    }

    const ADR: &str = r#"
name: adr
document:
  title:
    pattern: "^ADR-\\d{4}:"
    extract: title
  preamble:
    fields:
      - name: ID
        extract: id
      - name: 状態
        extract: status
      - name: 日付
        extract: date
  sections:
    - name: 状況
      extract: sections.context
      statement:
        required: false
    - name: 決定
      extract: sections.decision
      statement:
        required: false
    - name: 理由
      bullets:
        repeat: { min: 0 }
        extract: sections.reasons
"#;

    fn adr_doc() -> &'static str {
        "# ADR-0001: 印の話\n\n- ID: ADR-0001\n- 状態: 承認済み\n- 日付: 2026-09-16\n\n## 状況\n\n背景。\n\n## 決定\n\n判断。\n\n## 理由\n\n- 理由1\n- 理由2\n"
    }

    // @kotowari[REQ-schema-036]
    #[test]
    fn adr_values_are_nested_by_path() {
        let v = values(ADR, adr_doc());
        assert_eq!(v["id"], "ADR-0001");
        assert_eq!(v["title"], "ADR-0001: 印の話");
        assert_eq!(v["status"], "承認済み");
        assert_eq!(v["date"], "2026-09-16");
        assert_eq!(v["sections"]["context"], "背景。");
        assert_eq!(v["sections"]["decision"], "判断。");
        assert_eq!(v["sections"]["reasons"], json!(["- 理由1", "- 理由2"]));
    }

    // @kotowari[REQ-schema-028, REQ-schema-035]
    #[test]
    fn undeclared_field_name_line_is_extracted_as_a_bullet() {
        let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        extract: reasons
"#;
        let doc = "## 理由\n\n- 判断の記録かどうかの見分け（A134: 決定の節の見出しを1つ以上持つファイル）\n";
        let v = values(schema, doc);
        assert_eq!(
            v["reasons"],
            json!(["- 判断の記録かどうかの見分け（A134: 決定の節の見出しを1つ以上持つファイル）"]),
            "未宣言の名前の `- 名前: 値` 行は箇条書きとして抽出する（R8・R16）"
        );
    }

    // @kotowari[REQ-schema-028, REQ-schema-035]
    #[test]
    fn section_body_includes_undeclared_field_name_line_as_a_bullet() {
        let schema = r#"
document:
  sections:
    - name: 状況
      extract: sections.body
      statement:
        required: false
      bullets:
        repeat: { min: 0 }
"#;
        let doc = "## 状況\n\n- 判断の記録かどうかの見分け（A134: 決定の節の見出しを1つ以上持つファイル）\n";
        let v = values(schema, doc);
        assert_eq!(
            v["sections"]["body"],
            "- 判断の記録かどうかの見分け（A134: 決定の節の見出しを1つ以上持つファイル）",
            "節の本文はフィールド行を含めず、箇条書きとして含める（R16）"
        );
    }

    // @kotowari[REQ-schema-035]
    #[test]
    fn undeclared_field_name_line_extract_preserves_original_marker() {
        let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        extract: reasons
"#;
        let doc = "## 理由\n\n* 判断の記録（A134: 決定の節）\n";
        let v = values(schema, doc);
        assert_eq!(
            v["reasons"],
            json!(["* 判断の記録（A134: 決定の節）"]),
            "未宣言の名前の `* 名前: 値` 行は元のマーカーを保って箇条書きとして抽出する（R10）"
        );
    }

    // @kotowari[REQ-schema-035]
    #[test]
    fn section_body_includes_undeclared_field_name_line_with_original_marker() {
        let schema = r#"
document:
  sections:
    - name: 状況
      extract: sections.body
      statement:
        required: false
      bullets:
        repeat: { min: 0 }
"#;
        let doc = "## 状況\n\n* 判断の記録（A134: 決定の節）\n";
        let v = values(schema, doc);
        assert_eq!(
            v["sections"]["body"], "* 判断の記録（A134: 決定の節）",
            "節の本文の箇条書き要素は元のマーカーを保つ（R16）"
        );
    }

    // @kotowari[REQ-schema-035]
    #[test]
    fn title_named_group_capture_is_extracted() {
        let schema = r#"
document:
  title:
    pattern: "^ADR-(?<id>\\d{4}):"
    extract: { path: id, group: id }
"#;
        let v = values(schema, "# ADR-0042: 話\n");
        assert_eq!(v["id"], "0042");
    }

    // @kotowari[REQ-schema-019, REQ-schema-038]
    #[test]
    fn repeated_node_is_an_array_when_present_and_omits_key_when_absent() {
        let schema = r#"
document:
  preamble:
    fields:
      - name: タグ
        repeat: { min: 0, max: 1 }
        extract: tags
"#;
        let present = values(schema, "# 題名\n\n- タグ: a\n");
        assert_eq!(present["tags"], json!(["a"]));
        let absent = values(schema, "# 題名\n");
        assert!(absent.get("tags").is_none(), "0件のときはキーを省略する");
    }

    // @kotowari[REQ-schema-019, REQ-schema-038]
    #[test]
    fn non_repeated_node_is_single_and_omits_key_when_missing() {
        let schema = r#"
document:
  preamble:
    fields:
      - name: 状態
        extract: status
"#;
        let present = values(schema, "# 題名\n\n- 状態: 承認済み\n");
        assert_eq!(present["status"], "承認済み");
        let absent = values(schema, "# 題名\n");
        assert!(absent.get("status").is_none());
    }

    // @kotowari[REQ-schema-038]
    #[test]
    fn non_repeated_bullets_omit_key_when_missing() {
        let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        extract: reasons
"#;
        let v = values(schema, "## 理由\n");
        assert!(v.get("reasons").is_none());
    }

    // @kotowari[REQ-schema-038]
    #[test]
    fn repeated_bullets_omit_key_when_missing() {
        let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        extract: reasons
"#;
        let v = values(schema, "## 理由\n");
        assert!(
            v.get("reasons").is_none(),
            "箇条書き0件のときはキーを省略する"
        );
    }

    // @kotowari[REQ-schema-029, REQ-schema-035]
    #[test]
    fn separator_trims_elements_and_keeps_empty_ones() {
        let schema = r#"
document:
  preamble:
    fields:
      - name: タグ
        separator: ","
        extract: tags
"#;
        let v = values(schema, "# 題名\n\n- タグ: a, b,,c,\n");
        assert_eq!(v["tags"], json!(["a", "b", "", "c", ""]));
    }

    // @kotowari[REQ-schema-019, REQ-schema-035, REQ-schema-045]
    #[test]
    fn repeated_field_with_separator_extracts_array_of_arrays() {
        let schema = r#"
document:
  preamble:
    fields:
      - name: タグ
        repeat: { min: 0 }
        separator: ","
        extract: tags
"#;
        let v = values(schema, "# 題名\n\n- タグ: a,b\n- タグ: c\n");
        assert_eq!(v["tags"], json!([["a", "b"], ["c"]]));
    }

    // @kotowari[REQ-schema-030, REQ-schema-035, REQ-schema-046]
    #[test]
    fn separator_split_excludes_continuation_paragraphs() {
        let schema = r#"
document:
  preamble:
    fields:
      - name: タグ
        separator: ","
        extract: tags
"#;
        let doc = "# 題名\n\n- タグ: a,b\n\n  継続,の段落\n";
        let v = values(schema, doc);
        assert_eq!(
            v["tags"],
            json!(["a", "b\n継続,の段落"]),
            "継続段落は値の分割に含めず、末尾の要素に改行で付ける（R8・R16）"
        );
    }

    // @kotowari[REQ-schema-029, REQ-schema-035]
    #[test]
    fn wrapped_line_is_part_of_value_and_subject_to_separator_split() {
        let schema = r#"
document:
  preamble:
    fields:
      - name: タグ
        separator: ","
        extract: tags
"#;
        let doc = "# 題名\n\n- タグ: a,b\n折り返し,c\n";
        let v = values(schema, doc);
        assert_eq!(
            v["tags"],
            json!(["a", "b\n折り返し", "c"]),
            "空行なしの折り返し行は値の一部で、separator の分割対象になる（R8）。継続段落のように末尾要素に付かない"
        );
    }

    // @kotowari[REQ-schema-029, REQ-schema-037, REQ-schema-045]
    #[test]
    fn separator_splits_value_into_string_elements() {
        let schema = r#"
document:
  preamble:
    fields:
      - name: タグ
        separator: ","
        extract: tags
"#;
        let v = values(schema, "# 題名\n\n- タグ: a,b,c\n");
        assert_eq!(v["tags"], json!(["a", "b", "c"]));
    }

    // @kotowari[REQ-schema-032, REQ-schema-035]
    #[test]
    fn paragraph_after_code_block_lead_is_extracted_as_a_statement() {
        let schema = r#"
document:
  sections:
    - name: 状況
      statement:
        required: false
        extract: note
      codeblock:
        required: false
        lang: python
      bullets:
        repeat: { min: 0 }
"#;
        let doc = "## 状況\n\n- ```python\n  x = 1\n  ```\n\n  後続の段落\n";
        let v = values(schema, doc);
        assert_eq!(
            v["note"], "後続の段落",
            "先頭がコードブロックのリスト項目の後続段落は文として抽出する"
        );
    }

    // @kotowari[REQ-schema-035]
    #[test]
    fn section_body_includes_statements_and_bullets_only() {
        let schema = r#"
document:
  sections:
    - name: 状況
      extract: sections.body
      statement:
        required: false
      bullets:
        repeat: { min: 0 }
      table:
        required: false
        header: [a]
      codeblock:
        required: false
        lang: gherkin
"#;
        let doc = "## 状況\n\n背景。\n\n- 箇条1\n- 箇条2\n\n| a |\n|---|\n| x |\n\n```gherkin\nScenario: 例\n```\n";
        let v = values(schema, doc);
        // 文と箇条書きの間は改行、文どうしだけ空行でつなぐ
        assert_eq!(v["sections"]["body"], "背景。\n- 箇条1\n- 箇条2");
    }

    // @kotowari[REQ-schema-030, REQ-schema-035]
    #[test]
    fn section_body_includes_continuation_paragraphs_and_nested_bullets() {
        let schema = r#"
document:
  sections:
    - name: 状況
      extract: sections.body
      statement:
        required: false
      bullets:
        repeat: { min: 0 }
"#;
        let doc = "## 状況\n\n- 理由1\n\n  続きの段落\n  - 入れ子\n";
        let v = values(schema, doc);
        // 子の箇条書きの行は、そのままのインデントで親の抽出要素に含める（R10）
        assert_eq!(v["sections"]["body"], "- 理由1\n続きの段落\n  - 入れ子");
    }

    // @kotowari[REQ-schema-028, REQ-schema-035]
    #[test]
    fn bullet_extract_preserves_original_markers() {
        let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        extract: reasons
"#;
        let doc = "## 理由\n\n* 星\n\n+ プラス\n\n- ハイフン\n";
        let v = values(schema, doc);
        assert_eq!(
            v["reasons"],
            json!(["* 星", "+ プラス", "- ハイフン"]),
            "箇条書きの抽出は元のマーカーを保つ（R10）"
        );
    }

    // @kotowari[REQ-schema-030, REQ-schema-035]
    #[test]
    fn bullet_extract_preserves_marker_with_continuation() {
        let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        extract: reasons
"#;
        let doc = "## 理由\n\n* 親\n\n  続きの段落\n";
        let v = values(schema, doc);
        assert_eq!(
            v["reasons"],
            json!(["* 親\n続きの段落"]),
            "継続段落を付けるときも元のマーカーを保つ（R10）"
        );
    }

    // @kotowari[REQ-schema-035]
    #[test]
    fn bullet_extract_preserves_whitespace_after_marker() {
        let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        extract: reasons
"#;
        let doc = "## 理由\n\n-  親\n";
        let v = values(schema, doc);
        assert_eq!(
            v["reasons"],
            json!(["-  親"]),
            "抽出の1要素は元の行（マーカーとその直後の空白を含む）を使う（R10）"
        );
    }

    // @kotowari[REQ-schema-030, REQ-schema-035]
    #[test]
    fn bullet_extract_preserves_whitespace_after_marker_with_continuation() {
        let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        extract: reasons
"#;
        let doc = "## 理由\n\n-  親\n\n    続きの段落\n";
        let v = values(schema, doc);
        assert_eq!(
            v["reasons"],
            json!(["-  親\n続きの段落"]),
            "継続段落とつなぐときも元の行のマーカー直後の空白を保つ（R10）"
        );
    }

    // @kotowari[REQ-schema-035]
    #[test]
    fn bullet_extract_includes_lead_paragraph_on_a_separate_line_from_the_marker() {
        let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        extract: reasons
"#;
        let doc = "## 理由\n\n- \n  親\n- 次\n";
        let v = values(schema, doc);
        assert_eq!(
            v["reasons"],
            json!(["- \n親", "- 次"]),
            "マーカー行と別の行にある lead 段落の内容を抽出要素が欠落させない（R10）"
        );
    }

    // @kotowari[REQ-schema-035]
    #[test]
    fn bullet_extract_keeps_trailing_whitespace_of_the_marker_line() {
        let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        extract: reasons
"#;
        let doc = "## 理由\n\n- 親  \n続き\n";
        let v = values(schema, doc);
        assert_eq!(
            v["reasons"],
            json!(["- 親  \n続き"]),
            "ハード改行の末尾空白がマーカー行から剥がれない（R10）"
        );
    }

    // @kotowari[REQ-schema-035]
    #[test]
    fn undeclared_field_name_line_extract_preserves_whitespace_after_marker() {
        let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        extract: reasons
"#;
        let doc = "## 理由\n\n-  判断の記録（A134: 決定の節）\n";
        let v = values(schema, doc);
        assert_eq!(
            v["reasons"],
            json!(["-  判断の記録（A134: 決定の節）"]),
            "未宣言の名前の行を箇条書きとして抽出するときも元の行を使う（R8・R10）"
        );
    }

    // @kotowari[REQ-schema-030, REQ-schema-035]
    #[test]
    fn bullet_extract_includes_marker_and_continuation() {
        let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        extract: reasons
"#;
        let doc = "## 理由\n\n- 親\n\n  続きの段落\n\n- 次\n";
        let v = values(schema, doc);
        assert_eq!(v["reasons"], json!(["- 親\n続きの段落", "- 次"]));
    }

    // @kotowari[REQ-schema-030, REQ-schema-035]
    #[test]
    fn bullet_with_multiple_continuations_joins_them_with_blank_line() {
        let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        extract: reasons
"#;
        let doc = "## 理由\n\n- 親\n\n  続き1\n\n  続き2\n";
        let v = values(schema, doc);
        assert_eq!(v["reasons"], json!(["- 親\n続き1\n\n続き2"]));
    }

    // @kotowari[REQ-schema-028, REQ-schema-035]
    #[test]
    fn ordered_list_is_not_extracted_as_a_bullet() {
        let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        extract: reasons
"#;
        let doc = "## 理由\n\n1. 順序付き\n\n- 箇条書き\n";
        let v = values(schema, doc);
        assert_eq!(
            v["reasons"],
            json!(["- 箇条書き"]),
            "順序付きリストは箇条書きの対象外で抽出に含めない（R10）"
        );
    }

    // @kotowari[REQ-schema-030, REQ-schema-035, REQ-schema-046]
    #[test]
    fn field_extract_includes_continuation_paragraph() {
        let schema = r#"
document:
  preamble:
    fields:
      - name: 状態
        extract: status
"#;
        let doc = "# 題名\n\n- 状態: 承認済み\n\n  継続の段落\n";
        let v = values(schema, doc);
        assert_eq!(
            v["status"], "承認済み\n継続の段落",
            "フィールド行の継続段落は値に改行でつなぐ（R8・R16）"
        );
    }

    // @kotowari[REQ-schema-030, REQ-schema-035]
    #[test]
    fn item_body_field_line_includes_continuation_paragraph() {
        let schema = r#"
document:
  sections:
    - name: 要求
      item:
        repeat: { min: 0 }
        extract: items
        fields:
          - name: 種類
        statement:
          required: false
"#;
        let doc = "## 要求\n\n### REQ-001: 名前\n\n- 種類: algorithm\n\n  継続の説明\n";
        let v = values(schema, doc);
        // 継続段落はフィールド行の一部なので、項目の本文のフィールド行にも含める（R8・R16）
        assert_eq!(
            v["items"],
            json!(["REQ-001: 名前\n- 種類: algorithm\n継続の説明"])
        );
    }

    // @kotowari[REQ-schema-035]
    #[test]
    fn item_extract_is_heading_plus_body() {
        let schema = r#"
document:
  sections:
    - name: 要求
      item:
        repeat: { min: 0 }
        extract: items
        fields:
          - name: 種類
        statement:
          required: false
"#;
        let doc = "## 要求\n\n### REQ-001: 名前\n\n- 種類: algorithm\n\n本文。\n";
        let v = values(schema, doc);
        // 見出しと本文は改行でつなぎ、本文内は文どうしだけ空行でつなぐ
        assert_eq!(
            v["items"],
            json!(["REQ-001: 名前\n- 種類: algorithm\n本文。"])
        );
    }

    // @kotowari[REQ-schema-033, REQ-schema-035]
    #[test]
    fn table_extract_concatenates_multiple_tables_in_document_order() {
        let schema = r#"
document:
  sections:
    - name: 用語集
      table:
        required: false
        header: [a, b]
        extract: glossary
"#;
        let doc =
            "## 用語集\n\n| a | b |\n|---|---|\n| 1 | 2 |\n\n| a | b |\n|---|---|\n| 3 | 4 |\n";
        let v = values(schema, doc);
        assert_eq!(
            v["glossary"],
            json!([{ "a": "1", "b": "2" }, { "a": "3", "b": "4" }])
        );
    }

    // @kotowari[REQ-schema-033, REQ-schema-035]
    #[test]
    fn repeated_table_extracts_flat_row_objects_without_nesting() {
        let schema = r#"
document:
  sections:
    - name: 用語集
      table:
        repeat: { min: 0 }
        header: [a, b]
        extract: glossary
"#;
        let doc = "## 用語集\n\n| a | b |\n|---|---|\n| 1 | 2 |\n";
        let v = values(schema, doc);
        assert_eq!(v["glossary"], json!([{ "a": "1", "b": "2" }]));
    }

    // @kotowari[REQ-schema-035, REQ-schema-048]
    #[test]
    fn statement_with_a_derived_value_extracts_one_object_per_line() {
        let schema = r#"
document:
  sections:
    - name: 記録
      statement:
        extract:
          path: lines
          value: text
          of: { line: line }
"#;
        let doc = "## 記録\n\n 文の1行目\n  字下げの2行目\n   3行目\n";
        let v = values(schema, doc);
        assert_eq!(
            v["lines"],
            json!([
                { "text": " 文の1行目", "line": 3 },
                { "text": "  字下げの2行目", "line": 4 },
                { "text": "   3行目", "line": 5 }
            ]),
            "行の数と同じ数のオブジェクトが、生の行と行番号を持つ"
        );
    }

    // @kotowari[REQ-schema-035, REQ-schema-048]
    #[test]
    fn statement_with_a_derived_value_lists_paragraphs_separated_by_a_blank_line() {
        let schema = r#"
document:
  sections:
    - name: 記録
      statement:
        repeat: { min: 0 }
        extract:
          path: lines
          value: text
          of: { line: line }
"#;
        let doc = "## 記録\n\n1行目\n\n次の段落\n";
        let v = values(schema, doc);
        assert_eq!(
            v["lines"],
            json!([
                { "text": "1行目", "line": 3 },
                { "text": "次の段落", "line": 5 }
            ]),
            "空行を挟んで続く段落も同じ並びに入る（R16）"
        );
    }

    // @kotowari[REQ-schema-033, REQ-schema-048]
    #[test]
    fn table_with_a_derived_value_puts_the_row_line_into_each_row_object() {
        let schema = r#"
document:
  sections:
    - name: 用語集
      table:
        header: [a, b]
        extract:
          path: glossary
          value: cells
          of: { line: line }
"#;
        let doc = "## 用語集\n\n| a | b |\n|---|---|\n| 1 | 2 |\n| 3 | 4 |\n| 5 | 6 |\n";
        let v = values(schema, doc);
        assert_eq!(
            v["glossary"],
            json!([
                { "cells": { "a": "1", "b": "2" }, "line": 5 },
                { "cells": { "a": "3", "b": "4" }, "line": 6 },
                { "cells": { "a": "5", "b": "6" }, "line": 7 }
            ]),
            "行ごとのオブジェクトが要素の値と行番号を持つ（R16・R17）"
        );
    }

    // @kotowari[REQ-schema-033, REQ-schema-048]
    #[test]
    fn repeated_table_with_a_derived_value_nests_rows_per_table() {
        let schema = r#"
document:
  sections:
    - name: 用語集
      table:
        repeat: { min: 0 }
        header: [a, b]
        extract:
          path: glossary
          value: cells
          of: { line: line }
"#;
        let doc =
            "## 用語集\n\n| a | b |\n|---|---|\n| 1 | 2 |\n\n| a | b |\n|---|---|\n| 3 | 4 |\n";
        let v = values(schema, doc);
        assert_eq!(
            v["glossary"],
            json!([
                [{ "cells": { "a": "1", "b": "2" }, "line": 5 }],
                [{ "cells": { "a": "3", "b": "4" }, "line": 9 }]
            ]),
            "繰り返す表は配置パスの直下に表ごとの段を作る（R1）"
        );
    }

    // @kotowari[REQ-schema-033, REQ-schema-035]
    #[test]
    fn table_without_a_declared_header_gives_each_row_as_an_array() {
        let schema = r#"
document:
  sections:
    - name: 決定表
      table:
        extract: rows
"#;
        let doc = "## 決定表\n\n| 順 | 条件 |\n|---|---|\n| 1 | あれ |\n| 2 | これ |\n";
        let v = values(schema, doc);
        assert_eq!(
            v["rows"],
            json!([["1", "あれ"], ["2", "これ"]]),
            "header を宣言しない表の行は列の位置の配列になる（R1）"
        );
    }

    // @kotowari[REQ-schema-033, REQ-schema-035]
    #[test]
    fn table_with_empty_or_repeated_header_cells_keeps_every_column() {
        let empty = "## 決定表\n\n|  |  |\n|---|---|\n| 1 | 2 |\n";
        let repeated = "## 決定表\n\n| a | a |\n|---|---|\n| 1 | 2 |\n";
        let schema = r#"
document:
  sections:
    - name: 決定表
      table:
        extract: rows
"#;
        assert_eq!(
            values(schema, empty)["rows"],
            json!([["1", "2"]]),
            "ヘッダのセルが空でも列の値は1つも失われない（R1）"
        );
        assert_eq!(
            values(schema, repeated)["rows"],
            json!([["1", "2"]]),
            "同じ名前の列が2つあっても列の値は1つも失われない（R1）"
        );
    }

    // @kotowari[REQ-schema-033, REQ-schema-035]
    #[test]
    fn declared_header_decides_the_row_keys_when_the_column_count_differs() {
        let schema = r#"
document:
  sections:
    - name: 用語集
      table:
        header: [用語, 意味, 出典]
        extract: glossary
"#;
        let fewer = "## 用語集\n\n| 用語 | 意味 |\n|---|---|\n| 印 | しるし |\n";
        let more = "## 用語集\n\n| 用語 | 意味 | 出典 | 余り |\n|---|---|---|---|\n| 印 | しるし | a.md | 捨てる |\n";
        assert_eq!(
            values(schema, fewer)["glossary"],
            json!([{ "用語": "印", "意味": "しるし" }]),
            "文書の列が足りなければその鍵を省く（R1）"
        );
        assert_eq!(
            values(schema, more)["glossary"],
            json!([{ "用語": "印", "意味": "しるし", "出典": "a.md" }]),
            "文書の列が多ければ余りを捨てる（R1）"
        );
    }

    // @kotowari[REQ-schema-038]
    #[test]
    fn table_omits_key_when_no_tables() {
        let schema = r#"
document:
  sections:
    - name: 用語集
      table:
        repeat: { min: 0 }
        header: [a, b]
        extract: glossary
"#;
        let v = values(schema, "## 用語集\n");
        assert!(v.get("glossary").is_none(), "表0件のときはキーを省略する");
    }

    // @kotowari[REQ-schema-033, REQ-schema-035]
    #[test]
    fn table_extract_keys_by_header_and_later_column_overrides() {
        let schema = r#"
document:
  sections:
    - name: 用語集
      table:
        header: [a, b, a]
        extract: glossary
"#;
        let doc = "## 用語集\n\n| a | b | a |\n|---|---|---|\n| 1 | 2 | 3 |\n";
        let v = values(schema, doc);
        assert_eq!(v["glossary"], json!([{ "a": "3", "b": "2" }]));
    }

    // @kotowari[REQ-schema-019, REQ-schema-035]
    #[test]
    fn repeated_section_extracts_array_of_bodies() {
        let schema = r#"
document:
  sections:
    - name: 状況
      repeat: { min: 0 }
      extract: contexts
      statement:
        required: false
"#;
        let doc = "## 状況\n\n1つ目。\n\n## 状況\n\n2つ目。\n";
        let v = values(schema, doc);
        assert_eq!(v["contexts"], json!(["1つ目。", "2つ目。"]));
    }

    fn typed(schema_yaml: &str, doc: &str) -> serde_json::Value {
        let schema = parse_schema(schema_yaml).unwrap();
        let document = Document::parse(doc).unwrap();
        extract_typed(&schema, &document)
    }

    // @kotowari[REQ-schema-036]
    #[test]
    fn typed_ast_includes_type_when_schema_is_named() {
        let schema = "name: adr\ndocument:\n  title:\n    extract: title\n";
        let v = typed(schema, "# 題名\n");
        assert_eq!(v["type"], "adr");
        assert_eq!(v["title"], "題名");
    }

    // @kotowari[REQ-schema-036]
    #[test]
    fn typed_ast_omits_type_when_schema_is_unnamed() {
        let schema = "document:\n  title:\n    extract: title\n";
        let v = typed(schema, "# 題名\n");
        assert!(v.get("type").is_none());
        assert_eq!(v["title"], "題名");
    }

    // @kotowari[REQ-schema-031, REQ-schema-035]
    #[test]
    fn parent_bullet_element_includes_child_bullet_lines_with_indentation() {
        let schema = r#"
document:
  sections:
    - name: 決定
      bullets:
        repeat: { min: 0 }
        extract: decisions
        children:
          bullets:
            repeat: { min: 0 }
"#;
        let doc = "## 決定\n\n- A22 判断の記録\n  - 補足の子\n";
        let v = values(schema, doc);
        assert_eq!(
            v["decisions"],
            json!(["- A22 判断の記録\n  - 補足の子"]),
            "親の抽出要素に子の箇条書きの行をそのままのインデントで含める（R10・R16）"
        );
    }

    // @kotowari[REQ-schema-031, REQ-schema-035]
    #[test]
    fn declared_child_field_is_excluded_from_the_parent_element() {
        let schema = r#"
document:
  sections:
    - name: 決定
      bullets:
        repeat: { min: 0 }
        extract: decisions
        children:
          fields:
            - name: superseded_by
"#;
        let doc = "## 決定\n\n- A22 判断の記録\n  - superseded_by: [A5]\n";
        let v = values(schema, doc);
        assert_eq!(
            v["decisions"],
            json!(["- A22 判断の記録"]),
            "子フィールド行は親の抽出要素に含めない（A12）"
        );
    }

    // @kotowari[REQ-schema-036]
    #[test]
    fn child_field_with_own_extract_is_placed_at_its_path() {
        let schema = r#"
document:
  sections:
    - name: 決定
      bullets:
        repeat: { min: 0 }
        extract: decisions
        children:
          fields:
            - name: superseded_by
              extract: superseded_by
"#;
        let doc = "## 決定\n\n- A22 判断の記録\n  - superseded_by: [A5]\n";
        let v = values(schema, doc);
        assert_eq!(
            v["decisions"],
            json!(["- A22 判断の記録"]),
            "子フィールド行は親の抽出要素に含めない（A12）"
        );
        assert_eq!(
            v["superseded_by"], "[A5]",
            "子フィールドは自身の extract で配置パスに値を出す（A15）"
        );
    }

    // @kotowari[REQ-schema-031]
    #[test]
    fn child_of_declared_field_does_not_shadow_declared_child_field() {
        // 宣言済みフィールド行の下の子は未宣言の構造（R13）なので、子フィールドの
        // 抽出で拾わない。拾うと正当な子フィールドの値を覆い隠す。
        let schema = r#"
document:
  sections:
    - name: 決定
      fields:
        - name: 種類
      bullets:
        repeat: { min: 0 }
        extract: decisions
        children:
          fields:
            - name: superseded_by
              extract: superseded_by
"#;
        let doc = "## 決定\n\n- 種類: algorithm\n  - superseded_by: [余計]\n- A22 判断の記録\n  - superseded_by: [A5]\n";
        let v = values(schema, doc);
        assert_eq!(
            v["superseded_by"], "[A5]",
            "宣言済みフィールド行の下の子は、宣言済みの子フィールドの値を覆い隠さない（R13）"
        );
    }

    // @kotowari[REQ-schema-031, REQ-schema-035]
    #[test]
    fn recursive_nesting_is_included_in_the_parent_element() {
        let schema = r#"
document:
  sections:
    - name: 決定
      bullets:
        repeat: { min: 0 }
        extract: decisions
        children:
          bullets:
            repeat: { min: 0 }
            children:
              bullets:
                repeat: { min: 0 }
"#;
        let doc = "## 決定\n\n- 親\n  - 子\n    - 孫\n";
        let v = values(schema, doc);
        assert_eq!(
            v["decisions"],
            json!(["- 親\n  - 子\n    - 孫"]),
            "再帰的な入れ子が親の抽出要素に含まれる（R16）"
        );
    }

    // @kotowari[REQ-schema-031, REQ-schema-035]
    #[test]
    fn section_body_includes_child_bullet_lines() {
        let schema = r#"
document:
  sections:
    - name: 決定
      extract: sections.body
      bullets:
        repeat: { min: 0 }
        children:
          bullets:
            repeat: { min: 0 }
"#;
        let doc = "## 決定\n\n- 親\n  - 子\n";
        let v = values(schema, doc);
        assert_eq!(
            v["sections"]["body"], "- 親\n  - 子",
            "節の本文は子の箇条書きの行を含む（R16）"
        );
    }

    // @kotowari[REQ-schema-031, REQ-schema-035]
    #[test]
    fn item_body_includes_child_bullet_lines() {
        let schema = r#"
document:
  sections:
    - name: 要求
      item:
        repeat: { min: 0 }
        extract: items
        bullets:
          repeat: { min: 0 }
"#;
        let doc = "## 要求\n\n### REQ-001: 名前\n\n- 親\n  - 子\n";
        let v = values(schema, doc);
        assert_eq!(
            v["items"],
            json!(["REQ-001: 名前\n- 親\n  - 子"]),
            "項目の本文は子の箇条書きの行を含む（R16）"
        );
    }

    // @kotowari[REQ-schema-036]
    #[test]
    fn deep_child_field_extract_is_placed_at_its_path() {
        let schema = r#"
document:
  sections:
    - name: 決定
      bullets:
        repeat: { min: 0 }
        extract: decisions
        children:
          bullets:
            repeat: { min: 0 }
            children:
              fields:
                - name: 補足
                  extract: notes
"#;
        let doc = "## 決定\n\n- 親\n  - 子\n    - 補足: 深い\n";
        let v = values(schema, doc);
        assert_eq!(
            v["decisions"],
            json!(["- 親\n  - 子"]),
            "深いレベルの宣言された子フィールドも親の抽出要素に含めない（R10）"
        );
        assert_eq!(
            v["notes"], "深い",
            "深いレベルの子フィールドも自身の extract で配置パスに値を出す（A15）"
        );
    }

    // @kotowari[REQ-schema-035]
    #[test]
    fn item_body_excludes_table_and_code_block() {
        let schema = r#"
document:
  sections:
    - name: 決定表
      item:
        id: "TBL-\\d{3,}"
        repeat: { min: 0 }
        extract: tables
        fields:
          - name: 出典
        table:
          header: [用語, 意味]
        codeblock:
          lang: gherkin
"#;
        let doc = "## 決定表\n\n### TBL-001: 名前\n\n- 出典: docs/a.md\n\n| 用語 | 意味 |\n|---|---|\n| 印 | テストの印 |\n\n```gherkin\nScenario: 印を書く\n```\n";
        let v = values(schema, doc);
        assert_eq!(
            v["tables"],
            json!(["TBL-001: 名前\n- 出典: docs/a.md"]),
            "項目の抽出の本文に表を含めない（R16）"
        );
    }

    // @kotowari[REQ-schema-023, REQ-schema-033]
    #[test]
    fn preamble_table_is_extracted_as_row_objects() {
        let schema = "document:\n  preamble:\n    table:\n      header: [用語, 意味]\n      extract: glossary\n";
        let doc = "# 用語集\n\n| 用語 | 意味 |\n|---|---|\n| 印 | テストの印 |\n";
        let v = values(schema, doc);
        assert_eq!(
            v["glossary"],
            json!([{"用語": "印", "意味": "テストの印"}]),
            "前置部の表もヘッダをキーにしたオブジェクトの配列で抽出する（R5・R16）"
        );
    }

    // @kotowari[REQ-schema-048]
    #[test]
    fn of_line_places_the_line_number_as_a_number() {
        let schema = "document:\n  sections:\n    - name: 記録\n      fields:\n        - name: 状態\n          extract: { path: status, of: { status_line: line } }\n";
        let doc = "## 記録\n\n- 状態: ok\n";
        let v = values(schema, doc);
        assert_eq!(
            v["status"],
            json!({ "status_line": 3 }),
            "value を省いた要素オブジェクトは導かれる値の鍵だけを持ち、行番号は数値（R16・R17）"
        );
    }

    // @kotowari[REQ-schema-047, REQ-schema-048]
    #[test]
    fn item_extract_becomes_an_object_when_internals_declare_extract() {
        let schema = r#"
document:
  sections:
    - name: 要求
      item:
        id: "REQ-\\d{3,}"
        repeat: { min: 0 }
        extract:
          path: requirements
          of: { id: id, name: name, line: line }
        fields:
          - name: 種類
            extract: kind
          - name: 出典
            separator: ","
            extract: sources
        statement:
          extract: text
"#;
        let doc =
            "## 要求\n\n### REQ-001: 印の構文\n\n- 種類: algorithm\n- 出典: a.md, b.md\n\n本文。\n";
        let v = values(schema, doc);
        assert_eq!(
            v["requirements"],
            json!([{
                "kind": "algorithm",
                "sources": ["a.md", "b.md"],
                "text": "本文。",
                "id": "REQ-001",
                "name": "印の構文",
                "line": 3
            }]),
            "内部が extract を宣言したら項目はオブジェクトになる（R16）"
        );
    }

    // @kotowari[REQ-schema-047]
    #[test]
    fn item_extract_stays_a_string_without_internal_extract() {
        let schema = "document:\n  sections:\n    - name: 要求\n      item:\n        id: \"REQ-\\\\d{3,}\"\n        repeat: { min: 0 }\n        extract: requirements\n        fields:\n          - name: 種類\n";
        let doc = "## 要求\n\n### REQ-001: 印の構文\n\n- 種類: algorithm\n";
        let v = values(schema, doc);
        assert_eq!(
            v["requirements"],
            json!(["REQ-001: 印の構文\n- 種類: algorithm"]),
            "内部に extract が無ければ今までどおり文字列（R16）"
        );
    }

    // @kotowari[REQ-schema-035, REQ-schema-048]
    #[test]
    fn codeblock_with_a_derived_value_gives_one_object_per_block() {
        let schema = r#"
document:
  sections:
    - name: 具体例
      codeblock:
        lang: gherkin
        repeat: { min: 0 }
        extract: { path: scenarios, value: code, of: { line: line } }
"#;
        let doc =
            "## 具体例\n\n```gherkin\nScenario: 1つ目\n```\n\n```gherkin\nScenario: 2つ目\n```\n";
        let v = values(schema, doc);
        assert_eq!(
            v["scenarios"],
            json!([
                { "code": "Scenario: 1つ目", "line": 3 },
                { "code": "Scenario: 2つ目", "line": 7 }
            ]),
            "コードブロックはブロックが要素の単位で、行はフェンスの開始行（R17）"
        );
    }

    // @kotowari[REQ-schema-035, REQ-schema-048]
    #[test]
    fn bullets_with_a_derived_value_give_one_object_per_line() {
        let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        extract: { path: reasons, value: text, of: { line: line } }
"#;
        let doc = "## 理由\n\n- 理由1\n- 理由2\n";
        let v = values(schema, doc);
        assert_eq!(
            v["reasons"],
            json!([
                { "text": "- 理由1", "line": 3 },
                { "text": "- 理由2", "line": 4 }
            ]),
            "箇条書きは行が要素の単位になる（R17）"
        );
    }

    // @kotowari[REQ-schema-035, REQ-schema-048]
    #[test]
    fn title_with_a_raw_derived_value_gives_the_heading_line() {
        let schema =
            "document:\n  title:\n    extract: { path: title, value: text, of: { raw: raw } }\n";
        let doc = "# 題名 `インライン`\n";
        let v = values(schema, doc);
        assert_eq!(
            v["title"]["raw"], "# 題名 `インライン`",
            "raw は見出しの行そのままで、組み立て直さない（R17）"
        );
        assert_eq!(
            v["title"]["text"], "題名 インライン",
            "要素の値は今までの題名の文字のまま"
        );
    }

    // @kotowari[REQ-schema-035, REQ-schema-048]
    #[test]
    fn field_with_a_value_and_a_derived_value_nests_under_the_path() {
        let schema = "document:\n  sections:\n    - name: 記録\n      fields:\n        - name: 状態\n          extract: { path: status, value: text, of: { line: line } }\n";
        let doc = "## 記録\n\n- 状態: ok\n";
        let v = values(schema, doc);
        assert_eq!(
            v["status"],
            json!({ "text": "ok", "line": 3 }),
            "分けないノードでも value か of を宣言すれば配置パスの下にオブジェクトができる（R17）"
        );
    }

    // @kotowari[REQ-schema-047, REQ-schema-048]
    #[test]
    fn item_without_a_value_keeps_the_inner_placement_paths() {
        let schema = r#"
document:
  sections:
    - name: 要求
      item:
        id: "REQ-\\d{3,}"
        repeat: { min: 0 }
        extract: { path: requirements, of: { line: line } }
        fields:
          - name: 種類
            extract: kind
"#;
        let doc = "## 要求\n\n### REQ-001: 印の構文\n\n- 種類: algorithm\n";
        let v = values(schema, doc);
        assert_eq!(
            v["requirements"],
            json!([{ "kind": "algorithm", "line": 3 }]),
            "value を省いた要素オブジェクトは導かれる値と内側の配置パスだけを持つ（R16）"
        );
    }

    // @kotowari[REQ-schema-047]
    #[test]
    fn item_object_keys_come_from_the_schema_not_the_document() {
        // 同じ配置パスを宣言していれば、文書の見出しの語とフィールド行の名前が
        // 変わっても出力のキーは変わらない。出力の契約はスキーマが持つ（A22）。
        let japanese = "document:\n  sections:\n    - name: 蔵書\n      item:\n        repeat: { min: 0 }\n        extract: { path: books, of: { code: id } }\n        fields:\n          - name: 著者\n            extract: author\n";
        let english = "document:\n  sections:\n    - name: Books\n      item:\n        repeat: { min: 0 }\n        extract: { path: books, of: { code: id } }\n        fields:\n          - name: Author\n            extract: author\n";
        let japanese_doc = "## 蔵書\n\n### ISBN-1: 吾輩は猫である\n\n- 著者: 夏目漱石\n";
        let english_doc = "## Books\n\n### ISBN-1: I Am a Cat\n\n- Author: Soseki Natsume\n";

        let from_japanese = values(japanese, japanese_doc);
        let from_english = values(english, english_doc);

        assert_eq!(
            from_japanese["books"],
            json!([{ "author": "夏目漱石", "code": "ISBN-1" }])
        );
        assert_eq!(
            from_english["books"],
            json!([{ "author": "Soseki Natsume", "code": "ISBN-1" }])
        );
        assert_eq!(
            from_japanese["books"][0]
                .as_object()
                .unwrap()
                .keys()
                .collect::<Vec<_>>(),
            from_english["books"][0]
                .as_object()
                .unwrap()
                .keys()
                .collect::<Vec<_>>(),
            "文書の語が違っても出力のキーは同じ"
        );
    }
}
