//! スキーマの `extract` に沿って値を組み立てる。

use crate::document::{Block, Document, Item as DocItem, LineSpan, RawLine, Section as DocSection};
use crate::schema::{
    Bullets, Children, CodeBlock, Extract, Field, OfKind, Schema, Statement, Table,
    is_declared_field, item_internals_declare_extract,
};
use serde_json::{Map, Value};

/// `values` の出力。スキーマの`読み方`で読んだ文書の木から、配置パスに沿った入れ子の JSON を作る。
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
        extract_table(
            preamble.table.as_ref(),
            &[blocks.as_slice()],
            document,
            &mut root,
        );
        extract_codeblock(preamble.codeblock.as_ref(), &blocks, document, &mut root);
    }
    for def in &schema.document.sections {
        extract_section(def, document, &mut root);
    }
    if let Some(item) = &schema.document.item {
        // 文書の直下の項目（REQ-schema-061）
        let items = document.preamble_items();
        let items: Vec<&DocItem> = items.iter().collect();
        extract_items(item, &items, document, &mut root);
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
    // pattern が捕まえた部分である（REQ-schema-048）
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
        // 節は要素に分けない。繰り返すときは配置パスの直下に段ができる（TBL-schema-008、PROP-schema-004）
        let elements: Vec<Value> = occurrences
            .iter()
            .map(|s| {
                element_value(
                    extract,
                    document,
                    Value::String(section_body(s, def, document)),
                    &Derived {
                        end: Some(document.end_line(s.line, 2)),
                        ..Derived::at(s.line)
                    },
                )
            })
            .collect();
        place_occurrences(root, extract, elements, def.repeat.is_some());
    }

    let blocks: Vec<&Block> = occurrences.iter().flat_map(|s| s.blocks.iter()).collect();
    extract_fields(&def.fields, &blocks, document, root);
    extract_statement(def.statement.as_ref(), &blocks, document, root);
    extract_bullets(&def.fields, def.bullets.as_ref(), &blocks, document, root);
    // select は節の出現ごとに選ぶ。検査も出現ごとに選ぶので、両者が同じ表を指す（REQ-schema-059）
    let per_occurrence: Vec<Vec<&Block>> = occurrences
        .iter()
        .map(|s| s.blocks.iter().collect())
        .collect();
    let groups: Vec<&[&Block]> = per_occurrence.iter().map(Vec::as_slice).collect();
    extract_table(def.table.as_ref(), &groups, document, root);
    extract_codeblock(def.codeblock.as_ref(), &blocks, document, root);

    if let Some(item) = &def.item {
        let items: Vec<&DocItem> = occurrences.iter().flat_map(|s| s.items.iter()).collect();
        extract_items(item, &items, document, root);
    }
}

/// 項目の並びを、項目の規則の `extract` に沿って置く。
fn extract_items(
    item: &crate::schema::Item,
    items: &[&DocItem],
    document: &Document,
    root: &mut Map<String, Value>,
) {
    let Some(extract) = &item.extract else {
        return;
    };
    if items.is_empty() {
        // 0件のときはキーを省略する（REQ-schema-038）
        return;
    }
    // 内部が extract を宣言したか、項目が value か of を宣言していれば
    // 項目ごとのオブジェクトにする（REQ-schema-047）
    let as_object =
        item_internals_declare_extract(item) || extract.has_of() || extract.value().is_some();
    let one = |i: &DocItem| {
        if as_object {
            item_object(i, item, document)
        } else {
            item_value(i, item, document)
        }
    };
    let value = if item.repeat.is_some() {
        Value::Array(items.iter().map(|i| one(i)).collect())
    } else {
        one(items[0])
    };
    place(root, extract.path(), value);
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
            // フィールド行は要素に分けない。繰り返すときは配置パスの直下に段ができる（TBL-schema-008、PROP-schema-004）
            let elements: Vec<Value> = occurrences
                .iter()
                .map(|b| {
                    element_value(
                        extract,
                        doc,
                        field_single(field, b, doc),
                        &Derived::at(b.line()),
                    )
                })
                .collect();
            place_occurrences(root, extract, elements, field.repeat.is_some());
        }
    }
}

/// フィールド行の要素の値。名前の後から始まる lead 段落の値に、継続段落を元の行のまま
/// 付ける（REQ-schema-030、REQ-schema-063）。
fn field_single(field: &Field, block: &Block, doc: &Document) -> Value {
    let Block::Field {
        value,
        line,
        lead_end,
        continuation,
        ..
    } = block
    else {
        return Value::String(String::new());
    };
    // 継続段落の部分だけを組む。lead 段落を空の部分として先頭に置き、間の空行を数えさせる
    let lead = Piece {
        span: LineSpan {
            first: *line,
            last: *lead_end,
        },
        text: String::new(),
    };
    let tail = render(
        std::iter::once(lead).chain(continuation_pieces(continuation, doc)),
        doc,
    );
    match field.effective_separator() {
        // 区切った要素は配列にする（REQ-schema-045）。要素は前後の空白を取り除いて抽出し、
        // 空の要素は空文字列として残す。分割は継続段落を含めない値だけを対象にし、
        // 継続段落は末尾の要素に付ける（REQ-schema-046）
        Some(sep) => {
            let mut elements: Vec<String> =
                value.split(sep).map(|s| s.trim().to_string()).collect();
            if let Some(last) = elements.last_mut() {
                last.push_str(&tail);
            }
            Value::Array(elements.into_iter().map(Value::String).collect())
        }
        None => Value::String(format!("{value}{tail}")),
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
        // 0件のときはキーを省略する（REQ-schema-038）
        return;
    }
    if extract.has_of() {
        // 導かれる値を宣言したときは行が要素の単位になる（TBL-schema-008）。空行で区切って
        // 続く段落も同じ並びに入り、出現回数の宣言では入れ子にしない
        let elements: Vec<Value> = statements
            .iter()
            .flat_map(|b| match b {
                Block::Statement { raw_lines, .. } => raw_lines.as_slice(),
                Block::Field { .. }
                | Block::Bullet { .. }
                | Block::OrderedList { .. }
                | Block::Table { .. }
                | Block::Code { .. }
                | Block::Other { .. } => &[],
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
    // 直下に段ができる（TBL-schema-008、PROP-schema-004）
    if statement.repeat.is_some() {
        let elements: Vec<Value> = statements
            .iter()
            .map(|b| {
                let text = render(statement_piece(b), doc);
                element_value(extract, doc, Value::String(text), &Derived::at(b.line()))
            })
            .collect();
        place(root, extract.path(), Value::Array(elements));
        return;
    }
    let text = render(statements.iter().flat_map(|b| statement_piece(b)), doc);
    let element = element_value(
        extract,
        doc,
        Value::String(text),
        &Derived::at(statements[0].line()),
    );
    place(root, extract.path(), element);
}

/// 文の1行の要素の値。前後の空白を取り除いた行の文字にする。字下げと
/// 末尾の空白を含む行そのままは導かれる値の `raw` で取る（REQ-schema-048）。
fn statement_line_text(raw: &RawLine) -> String {
    raw.text.trim().to_string()
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
    // 未宣言の構造（REQ-schema-003）で、そこにある子フィールド名の行を拾うと正当な子
    // フィールドの値を覆い隠す。宣言された名前と一致しない `- 名前: 値` 行は
    // 箇条書きとして扱う（TBL-schema-007）。
    let bullet_blocks: Vec<&Block> = blocks
        .iter()
        .copied()
        .filter(|b| is_bullet_under(fields, b))
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
    // 子の箇条書きの行をそのままのインデントで含めた文字列（REQ-schema-031、TBL-schema-008）
    let elements: Vec<Value> = bullet_blocks
        .iter()
        .copied()
        .map(|b| {
            element_value(
                extract,
                doc,
                Value::String(render(
                    list_item_pieces(b, bullets.children.as_ref(), doc),
                    doc,
                )),
                &Derived::at(b.line()),
            )
        })
        .collect();
    if elements.is_empty() {
        // 0件のときはキーを省略する（REQ-schema-038）
        return;
    }
    place(root, extract.path(), Value::Array(elements));
    // 子フィールドは自身の extract を持てば、その配置パスに値を出す（A15）
    if let Some(children) = &bullets.children {
        extract_child_fields(children, &bullet_blocks, doc, root);
    }
}

/// 子フィールドの抽出。`children.fields` で宣言された子フィールドのうち、自身の
/// `extract` を持つものをその配置パスに置く（REQ-schema-031、REQ-schema-036）。さらに深い入れ子の
/// 子フィールドは、子の箇条書きの `children` の宣言に沿って再帰する。
fn extract_child_fields(
    children: &Children,
    blocks: &[&Block],
    doc: &Document,
    root: &mut Map<String, Value>,
) {
    let child_blocks: Vec<&Block> = blocks.iter().flat_map(|b| b.children().iter()).collect();
    extract_fields(&children.fields, &child_blocks, doc, root);
    // 子の箇条書きの下の children に再帰する。子の箇条書きは Bullet か、この
    // レベルの宣言と一致しない `- 名前: 値` 行（箇条書きとして扱う行）である（TBL-schema-007）
    if let Some(child_bullets) = children.bullets.as_deref()
        && let Some(grandchildren) = &child_bullets.children
    {
        let bullet_blocks: Vec<&Block> = child_blocks
            .into_iter()
            .filter(|c| is_bullet_under(&children.fields, c))
            .collect();
        extract_child_fields(grandchildren, &bullet_blocks, doc, root);
    }
}

fn is_bullet_under(fields: &[Field], block: &Block) -> bool {
    matches!(block, Block::Bullet { .. })
        || matches!(block, Block::Field { name, .. } if !is_declared_field(fields, name))
}

/// 値の1つの部分。元の文書の行の範囲と、その範囲から取った文字（REQ-schema-063）。
struct Piece {
    span: LineSpan,
    text: String,
}

/// 部分を元の文書の順に並べてつなぐ。間に元の文書で空行があれば空行1つ、無ければ
/// 改行1つでつなぎ、含めない部分の行は抜く。値の末尾の空白と空行は除く（REQ-schema-063）。
fn render(pieces: impl IntoIterator<Item = Piece>, doc: &Document) -> String {
    let mut pieces: Vec<Piece> = pieces.into_iter().collect();
    pieces.sort_by_key(|p| p.span.first);
    let mut out = String::new();
    let mut previous: Option<usize> = None;
    for piece in pieces {
        if let Some(last) = previous {
            let blank = (last + 1..piece.span.first)
                .any(|line| doc.raw_line(line).is_some_and(|t| t.trim().is_empty()));
            out.push_str(if blank { "\n\n" } else { "\n" });
        }
        out.push_str(&piece.text);
        previous = Some(piece.span.last);
    }
    out.trim_end().to_string()
}

/// 行の範囲の元の行を改行でつなぐ。
fn source_lines(doc: &Document, first: usize, last: usize) -> String {
    (first..=last)
        .map(|line| doc.raw_line(line).unwrap_or(""))
        .collect::<Vec<_>>()
        .join("\n")
}

fn continuation_pieces<'a>(
    continuation: &'a [LineSpan],
    doc: &'a Document,
) -> impl Iterator<Item = Piece> + 'a {
    continuation.iter().map(|span| Piece {
        span: *span,
        text: source_lines(doc, span.first, span.last),
    })
}

/// 文の段落を1つの部分にする。段落の最初の空白でない文字から始め、2行目からは元の行のまま
/// （REQ-schema-063）。
fn statement_piece(block: &Block) -> Option<Piece> {
    let Block::Statement { raw_lines, .. } = block else {
        return None;
    };
    let (first, rest) = raw_lines.split_first()?;
    let mut text = first.text.trim_start().to_string();
    for next in rest {
        text.push('\n');
        text.push_str(&next.text);
    }
    Some(Piece {
        span: LineSpan {
            first: first.line,
            last: rest.last().map_or(first.line, |r| r.line),
        },
        text,
    })
}

/// 一覧の行（箇条書きか、箇条書きとして扱うフィールド行）の部分。マーカーの行から lead 段落の
/// 終わりまでと継続段落を元の行のまま取り、子の箇条書きの部分を加える（REQ-schema-031、
/// REQ-schema-063、TBL-schema-008）。`children` の宣言でフィールド行として宣言された子と、
/// 表・コードブロックなどの子は含めない（A12）。`children` が無ければ子の一覧の行はすべて含める。
fn list_item_pieces(block: &Block, children: Option<&Children>, doc: &Document) -> Vec<Piece> {
    let mut pieces = own_pieces(block, doc);
    let declared = children.map_or(&[][..], |c| c.fields.as_slice());
    let child_rule = children
        .and_then(|c| c.bullets.as_deref())
        .and_then(|b| b.children.as_ref());
    for child in block.children() {
        match child {
            Block::Bullet { .. } => pieces.extend(list_item_pieces(child, child_rule, doc)),
            Block::Field { name, .. } if !is_declared_field(declared, name) => {
                pieces.extend(list_item_pieces(child, child_rule, doc))
            }
            Block::Field { .. }
            | Block::OrderedList { .. }
            | Block::Statement { .. }
            | Block::Table { .. }
            | Block::Code { .. }
            | Block::Other { .. } => {}
        }
    }
    pieces
}

/// 一覧の行そのものの部分。マーカーの行から lead 段落の終わりまでと継続段落で、子は含めない。
fn own_pieces(block: &Block, doc: &Document) -> Vec<Piece> {
    let (Block::Bullet {
        line_text,
        line,
        lead_end,
        continuation,
        ..
    }
    | Block::Field {
        line_text,
        line,
        lead_end,
        continuation,
        ..
    }) = block
    else {
        return Vec::new();
    };
    let mut text = line_text.clone();
    if lead_end > line {
        text.push('\n');
        text.push_str(&source_lines(doc, line + 1, *lead_end));
    }
    std::iter::once(Piece {
        span: LineSpan {
            first: *line,
            last: *lead_end,
        },
        text,
    })
    .chain(continuation_pieces(continuation, doc))
    .collect()
}

fn extract_table(
    table: Option<&Table>,
    groups: &[&[&Block]],
    doc: &Document,
    root: &mut Map<String, Value>,
) {
    let Some(table) = table else {
        return;
    };
    let Some(extract) = &table.extract else {
        return;
    };
    // select で選ばれなかった表は抽出しない（REQ-schema-059）。選ぶのはノードの出現ごと
    let tables: Vec<&Block> = groups
        .iter()
        .flat_map(|blocks| {
            let selection = table.selected_line(blocks.iter().copied());
            blocks
                .iter()
                .copied()
                .filter(move |b| matches!(b, Block::Table { line, .. } if selection.takes(*line)))
        })
        .collect();
    if tables.is_empty() {
        // 0件のときはキーを省略する（REQ-schema-038）
        return;
    }
    // 表はデータ行が要素の単位（TBL-schema-008）
    let per_table: Vec<Vec<Value>> = tables
        .iter()
        .map(|b| table_rows(b, table.header.as_deref(), extract, doc))
        .collect();
    let value = if table.repeat.is_some() && declares_element_object(extract) {
        // 繰り返す表は配置パスの直下に表ごとの段を作る（TBL-schema-008）
        Value::Array(per_table.into_iter().map(Value::Array).collect())
    } else {
        // 複数の表は現れた順に1つの配列へ連結する
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
        rows,
        row_lines,
        line,
        ..
    } = block
    else {
        return Vec::new();
    };
    rows.iter()
        .enumerate()
        .map(|(row_index, row)| {
            // 行番号は1始まりなので 0 に落とさない。0 だと `line` は 0 を返すのに
            // `raw` は行を引けず鍵ごと消え、同じ要素の2つの導かれる値が食い違う。
            // 表の開始行に落として validate 側の扱いに揃える
            let row_line = row_lines.get(row_index).copied().unwrap_or(*line);
            element_value(extract, doc, row_value(header, row), &Derived::at(row_line))
        })
        .collect()
}

/// 表の1行の要素の値。鍵はスキーマが宣言した `header` の名前で、宣言が
/// 無ければ列の位置（配列）にする。文書のヘッダ行の文字は鍵に使わない（TBL-schema-008）。
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
    // コードブロックはブロックが要素の単位で、行はフェンスの開始行（TBL-schema-008）
    let elements: Vec<Value> = codes
        .iter()
        .map(|b| {
            let value = match b {
                Block::Code { value, .. } => Value::String(value.clone()),
                Block::Field { .. }
                | Block::Bullet { .. }
                | Block::OrderedList { .. }
                | Block::Statement { .. }
                | Block::Table { .. }
                | Block::Other { .. } => Value::Null,
            };
            element_value(extract, doc, value, &Derived::at(b.line()))
        })
        .collect();
    place_occurrences(root, extract, elements, codeblock.repeat.is_some());
}

fn section_body(section: &DocSection, def: &crate::schema::Section, doc: &Document) -> String {
    body_from_blocks(
        &section.blocks,
        &def.fields,
        def.bullets.as_ref(),
        false,
        doc,
    )
}

/// 項目をオブジェクトに組み立てる。内部のノードの抽出は項目オブジェクトの中の
/// 相対パスへ置き、項目自身の `of` を添えた書式も同じオブジェクトの中へ置く（REQ-schema-047）。
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
    extract_table(
        item_rule.table.as_ref(),
        &[blocks.as_slice()],
        doc,
        &mut object,
    );
    extract_codeblock(item_rule.codeblock.as_ref(), &blocks, doc, &mut object);
    if let Some(extract) = &item_rule.extract {
        // path は項目オブジェクトそのものの置き場なので、ここでは置かない
        let got = Derived {
            line: Some(item.line),
            id: Some(item.id.clone()),
            name: Some(item.title.clone()),
            end: Some(doc.end_line(item.line, 3)),
        };
        if let Some(key) = extract.value() {
            place(
                &mut object,
                key,
                Value::String(item_value_text(item, item_rule, doc)),
            );
        }
        place_derived(&mut object, extract, doc, &got);
    }
    Value::Object(object)
}

fn item_value(item: &DocItem, item_rule: &crate::schema::Item, doc: &Document) -> Value {
    Value::String(item_value_text(item, item_rule, doc))
}

/// 項目の要素の値。見出しと本文をつないだ文字列（TBL-schema-008）。
fn item_value_text(item: &DocItem, item_rule: &crate::schema::Item, doc: &Document) -> String {
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
        doc,
    );
    if body.is_empty() {
        heading
    } else {
        format!("{heading}\n{body}")
    }
}

/// 本文を組み立てる。文と箇条書き（必要ならフィールド行も）を含み、
/// 表・コードブロック・項目は含めない。含めた部分は REQ-schema-063 のとおりにつなぐ。
/// 宣言された名前と一致しない `- 名前: 値` 行は箇条書きとして本文に含める（TBL-schema-007）。
fn body_from_blocks(
    blocks: &[Block],
    fields: &[Field],
    bullets: Option<&Bullets>,
    include_fields: bool,
    doc: &Document,
) -> String {
    let children = bullets.and_then(|b| b.children.as_ref());
    let pieces = blocks.iter().flat_map(|block| match block {
        Block::Statement { .. } => statement_piece(block).into_iter().collect(),
        Block::Bullet { .. } => list_item_pieces(block, children, doc),
        Block::Field { name, .. } if include_fields && is_declared_field(fields, name) => {
            own_pieces(block, doc)
        }
        Block::Field { name, .. } if include_fields || !is_declared_field(fields, name) => {
            list_item_pieces(block, children, doc)
        }
        Block::Field { .. }
        | Block::OrderedList { .. }
        | Block::Table { .. }
        | Block::Code { .. }
        | Block::Other { .. } => Vec::new(),
    });
    render(pieces, doc)
}

/// 1つの要素の導かれる値の材料（REQ-schema-048）。
#[derive(Default)]
struct Derived {
    line: Option<usize>,
    id: Option<String>,
    name: Option<String>,
    /// 要素の最後の行。項目と節だけが持つ（REQ-schema-062）
    end: Option<usize>,
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
/// 宣言していなければ、要素そのものが値になる（TBL-schema-008）。
fn declares_element_object(extract: &Extract) -> bool {
    extract.value().is_some() || extract.has_of()
}

/// 1つの要素を、宣言に応じて素の値か要素オブジェクトにする（TBL-schema-008、REQ-schema-048）。
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
/// 段を作り、繰り返さないノードは最初の出現だけを置く（PROP-schema-004）。0件のときは
/// キーを省略する（REQ-schema-038）。
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

/// 宣言された導かれる値を、その鍵へ置く。`raw` は文書の生の行から取る（REQ-schema-048）。
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
            OfKind::End => got.end.map(Value::from),
        };
        if let Some(value) = value {
            place(map, key, value);
        }
    }
}

fn place(root: &mut Map<String, Value>, path: &str, value: Value) {
    let keys: Vec<&str> = path.split('.').collect();
    let (last, parents) = keys
        .split_last()
        .expect("split always yields at least one piece");
    let mut current = root;
    for key in parents {
        let entry = current
            .entry(key.to_string())
            .or_insert_with(|| Value::Object(Map::new()));
        if !entry.is_object() {
            *entry = Value::Object(Map::new());
        }
        current = entry
            .as_object_mut()
            .expect("the entry was made an object just above");
    }
    current.insert((*last).to_string(), value);
}

#[cfg(test)]
mod tests;
