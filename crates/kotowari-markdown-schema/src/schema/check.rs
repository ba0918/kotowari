//! スキーマの意味の検査（TBL-schema-009）。型に読めたスキーマが、宣言として成り立つかを見る。

use super::*;

pub(super) fn validate_schema(schema: &Schema) -> Result<(), SchemaError> {
    if let Some(title) = &schema.document.title {
        validate_title(title)?;
    }
    if let Some(preamble) = &schema.document.preamble {
        validate_preamble(preamble)?;
    }
    reject_duplicate_names(
        schema.document.sections.iter().map(|s| s.name.as_str()),
        "section",
    )?;
    for section in &schema.document.sections {
        validate_section(section)?;
    }
    if let Some(item) = &schema.document.item {
        validate_item(item)?;
    }
    reject_colliding_root_paths(&schema.document)?;
    reject_root_paths_on_the_type_key(schema)
}

/// 要素オブジェクトの外、文書の値の根に置く配置パスの衝突を停止にする（TBL-schema-009）。
/// 題名・前置部・節・項目と、それらの直下のノードの配置パスはすべて同じ置き場に並ぶ。
fn reject_colliding_root_paths(document: &Document) -> Result<(), SchemaError> {
    let mut paths: Vec<String> = Vec::new();
    collect_root_paths(document, &mut paths);
    let keys: Vec<&str> = paths.iter().map(String::as_str).collect();
    reject_colliding_keys(&keys, "document")
}

/// 文書の値の根に置く配置パスを集める。題名・前置部・節・項目と、それらの直下のノードと、
/// 箇条書きの子フィールドの配置パスが同じ置き場に並ぶ
fn collect_root_paths(document: &Document, paths: &mut Vec<String>) {
    let mut push = |extract: Option<&Extract>| {
        if let Some(extract) = extract {
            paths.push(extract.path().to_string());
        }
    };
    push(document.title.as_ref().and_then(|t| t.extract.as_ref()));
    if let Some(preamble) = &document.preamble {
        for field in &preamble.fields {
            push(field.extract.as_ref());
        }
        push(preamble.statement.as_ref().and_then(|s| s.extract.as_ref()));
        push(preamble.bullets.as_ref().and_then(|b| b.extract.as_ref()));
        push(preamble.table.as_ref().and_then(|t| t.extract.as_ref()));
        push(preamble.codeblock.as_ref().and_then(|c| c.extract.as_ref()));
    }
    for section in &document.sections {
        push(section.extract.as_ref());
        for field in &section.fields {
            push(field.extract.as_ref());
        }
        push(section.statement.as_ref().and_then(|s| s.extract.as_ref()));
        push(section.bullets.as_ref().and_then(|b| b.extract.as_ref()));
        push(section.table.as_ref().and_then(|t| t.extract.as_ref()));
        push(section.codeblock.as_ref().and_then(|c| c.extract.as_ref()));
        push(section.item.as_ref().and_then(|i| i.extract.as_ref()));
    }
    push(document.item.as_ref().and_then(|i| i.extract.as_ref()));
    // 箇条書きの子フィールドも、その箇条書きの置き場（根）に値を出す
    if let Some(bullets) = document.preamble.as_ref().and_then(|p| p.bullets.as_ref()) {
        collect_children_extract_paths(bullets, paths);
    }
    for section in &document.sections {
        if let Some(bullets) = &section.bullets {
            collect_children_extract_paths(bullets, paths);
        }
    }
}

/// "ast --schema" は根の "type" に "name" を置くので、"name" を宣言した`スキーマ`で根の
/// 配置パスが "type" の鍵を使うと "name" が消える。衝突と同じく停止にする
/// （TBL-schema-009、2026-09-24-review2-gaps の A2）
fn reject_root_paths_on_the_type_key(schema: &Schema) -> Result<(), SchemaError> {
    if schema.name.is_none() {
        return Ok(());
    }
    let mut paths: Vec<String> = Vec::new();
    collect_root_paths(&schema.document, &mut paths);
    match paths
        .iter()
        .find(|path| path.as_str() == "type" || path.starts_with("type."))
    {
        Some(path) => Err(SchemaError(format!(
            "document has the key \"{path}\", which conflicts with \"type\" that ast --schema sets from \"name\""
        ))),
        None => Ok(()),
    }
}

fn validate_title(title: &Title) -> Result<(), SchemaError> {
    reject_of_words(title.extract.as_ref(), "title", OUTSIDE_ITEM_OF)?;
    reject_duplicate_element_keys(title.extract.as_ref(), "title", &[])?;
    let Some(group) = title.extract.as_ref().and_then(Extract::group) else {
        return Ok(());
    };
    // `group` を宣言した抽出は pattern の名前付きキャプチャを取る。pattern が無い、
    // または pattern が指定の名前付きキャプチャを含まない題名は schema_invalid（REQ-schema-048）
    let Some(pattern) = &title.pattern else {
        return Err(SchemaError(
            "title capture extract requires a pattern".into(),
        ));
    };
    if !pattern.has_capture_group(group) {
        return Err(SchemaError(format!(
            "title pattern does not contain a named capture group \"{group}\""
        )));
    }
    Ok(())
}

fn validate_preamble(preamble: &Preamble) -> Result<(), SchemaError> {
    validate_fields(&preamble.fields)?;
    validate_statement(preamble.statement.as_ref())?;
    validate_bullets(preamble.bullets.as_ref(), false)?;
    validate_table(preamble.table.as_ref())?;
    validate_codeblock(preamble.codeblock.as_ref())?;
    Ok(())
}

fn validate_section(section: &Section) -> Result<(), SchemaError> {
    if let Some(repeat) = &section.repeat {
        repeat.validate()?;
    }
    reject_capture_extract(section.extract.as_ref(), "section")?;
    reject_of_words(section.extract.as_ref(), "section", SECTION_OF)?;
    reject_duplicate_element_keys(section.extract.as_ref(), "section", &[])?;
    validate_fields(&section.fields)?;
    validate_statement(section.statement.as_ref())?;
    validate_bullets(section.bullets.as_ref(), false)?;
    validate_table(section.table.as_ref())?;
    validate_codeblock(section.codeblock.as_ref())?;
    if let Some(item) = &section.item {
        validate_item(item)?;
    }
    Ok(())
}

fn validate_item(item: &Item) -> Result<(), SchemaError> {
    if let Some(repeat) = &item.repeat {
        repeat.validate()?;
    }
    reject_capture_extract(item.extract.as_ref(), "item")?;
    // 項目の内部に extract を宣言するなら、項目自身も extract を持つ必要がある。
    // 内部の配置パスは項目オブジェクトの中の相対パスなので、置き場が要る（REQ-schema-039、REQ-schema-047）
    let internal = item_internal_extract_paths(item);
    if item.extract.is_none() && !internal.is_empty() {
        return Err(SchemaError(
            "item internals declare extract but the item itself does not".into(),
        ));
    }
    // 内側の配置パスも項目オブジェクトの中の鍵なので、重複の判定に含める（TBL-schema-009）
    reject_duplicate_element_keys(item.extract.as_ref(), "item", &internal)?;
    validate_table(item.table.as_ref())?;
    validate_codeblock(item.codeblock.as_ref())?;
    validate_fields(&item.fields)?;
    validate_statement(item.statement.as_ref())?;
    validate_bullets(item.bullets.as_ref(), false)?;
    Ok(())
}

/// 表の規則を検査する。
fn validate_table(table: Option<&Table>) -> Result<(), SchemaError> {
    if let Some(table) = table {
        if let Some(repeat) = &table.repeat {
            repeat.validate()?;
        }
        // どの表を選ぶかはヘッダで決めるので、ヘッダの無い select は停止にする（TBL-schema-009）
        if table.select.is_some() && table.header.is_none() {
            return Err(SchemaError("table select requires a header".into()));
        }
        if let Some(header) = &table.header {
            reject_duplicate_names(header.iter().map(String::as_str), "table header column")?;
        }
        reject_capture_extract(table.extract.as_ref(), "table")?;
        reject_of_words(table.extract.as_ref(), "table", OUTSIDE_ITEM_OF)?;
        reject_duplicate_element_keys(table.extract.as_ref(), "table", &[])?;
    }
    Ok(())
}

/// 1つの要素オブジェクトの中で鍵が重複するスキーマを停止にする（TBL-schema-009）。鍵は
/// `value`、`of` の鍵、そして内側のノードの配置パス（`internal` で渡す）である。
/// 判定は配置パス全体で行い、`a.b` と `a.c` は別の鍵、`a` と `a.b` は重複とする。
fn reject_duplicate_element_keys(
    extract: Option<&Extract>,
    node: &str,
    internal: &[String],
) -> Result<(), SchemaError> {
    let mut keys: Vec<&str> = Vec::new();
    if let Some(extract) = extract {
        keys.extend(extract.value());
        keys.extend(extract.of().iter().map(|(key, _)| key.as_str()));
    }
    keys.extend(internal.iter().map(String::as_str));
    reject_colliding_keys(&keys, &format!("{node} element object"))
}

/// 同じ置き場の配置パスのうち、同じパスか一方が他方の手前の段にあたるものを停止にする
/// （TBL-schema-009）。`a.b` と `a.c` は別の鍵、`a` と `a.b` は衝突とする。
fn reject_colliding_keys(keys: &[&str], place: &str) -> Result<(), SchemaError> {
    for (index, key) in keys.iter().enumerate() {
        for other in &keys[index + 1..] {
            if key == other || is_ancestor_path(key, other) || is_ancestor_path(other, key) {
                return Err(SchemaError(format!(
                    "{place} has the key \"{key}\" twice (conflicts with \"{other}\")"
                )));
            }
        }
    }
    Ok(())
}

/// `outer` が `inner` の親の配置パスか。`a` は `a.b` の親で、`a` と `ab` は無関係。
fn is_ancestor_path(outer: &str, inner: &str) -> bool {
    inner
        .strip_prefix(outer)
        .is_some_and(|rest| rest.starts_with('.'))
}

/// コードブロックの規則を検査する。
fn validate_codeblock(codeblock: Option<&CodeBlock>) -> Result<(), SchemaError> {
    if let Some(codeblock) = codeblock {
        if let Some(repeat) = &codeblock.repeat {
            repeat.validate()?;
        }
        reject_capture_extract(codeblock.extract.as_ref(), "codeblock")?;
        reject_of_words(codeblock.extract.as_ref(), "codeblock", OUTSIDE_ITEM_OF)?;
        reject_duplicate_element_keys(codeblock.extract.as_ref(), "codeblock", &[])?;
    }
    Ok(())
}

/// 同じ置き場で同じ名前を2度宣言したスキーマを停止にする（TBL-schema-009、
/// 2026-09-24-review4-gaps の A3、A5）。検査と抽出で効く宣言が食い違うか、値が黙って消える
fn reject_duplicate_names<'a>(
    names: impl Iterator<Item = &'a str>,
    what: &str,
) -> Result<(), SchemaError> {
    let mut seen = std::collections::BTreeSet::new();
    for name in names {
        if !seen.insert(name) {
            return Err(SchemaError(format!("{what} \"{name}\" is declared twice")));
        }
    }
    Ok(())
}

fn validate_fields(fields: &[Field]) -> Result<(), SchemaError> {
    reject_duplicate_names(fields.iter().map(|f| f.name.as_str()), "field")?;
    for field in fields {
        if let Some(repeat) = &field.repeat {
            repeat.validate()?;
        }
        if let Some(when) = &field.when {
            when.validate()?;
        }
        reject_capture_extract(field.extract.as_ref(), "field")?;
        reject_of_words(field.extract.as_ref(), "field", OUTSIDE_ITEM_OF)?;
        reject_duplicate_element_keys(field.extract.as_ref(), "field", &[])?;
    }
    Ok(())
}

fn validate_statement(statement: Option<&Statement>) -> Result<(), SchemaError> {
    if let Some(statement) = statement {
        if let Some(repeat) = &statement.repeat {
            repeat.validate()?;
        }
        if let Some(when) = &statement.when {
            when.validate()?;
        }
        reject_capture_extract(statement.extract.as_ref(), "statement")?;
        reject_of_words(statement.extract.as_ref(), "statement", OUTSIDE_ITEM_OF)?;
        reject_duplicate_element_keys(statement.extract.as_ref(), "statement", &[])?;
    }
    Ok(())
}

fn validate_bullets(bullets: Option<&Bullets>, in_children: bool) -> Result<(), SchemaError> {
    if let Some(bullets) = bullets {
        if in_children && bullets.extract.is_some() {
            return Err(SchemaError(
                "children bullets cannot declare extract".into(),
            ));
        }
        if let Some(repeat) = &bullets.repeat {
            repeat.validate()?;
        }
        if let Some(when) = &bullets.when {
            when.validate()?;
        }
        if !in_children {
            reject_capture_extract(bullets.extract.as_ref(), "bullets")?;
        }
        reject_of_words(bullets.extract.as_ref(), "bullets", OUTSIDE_ITEM_OF)?;
        reject_duplicate_element_keys(bullets.extract.as_ref(), "bullets", &[])?;
        if let Some(children) = &bullets.children {
            validate_fields(&children.fields)?;
            validate_bullets(children.bullets.as_deref(), true)?;
        }
    }
    Ok(())
}

/// 題名以外のノードには名前付きキャプチャ（`group`）の抽出を宣言できない（TBL-schema-008）。
/// 宣言すると schema_invalid の停止になる。
fn reject_capture_extract(extract: Option<&Extract>, node: &str) -> Result<(), SchemaError> {
    if extract.and_then(Extract::group).is_some() {
        return Err(SchemaError(format!(
            "{node} extract cannot use the named-group capture form"
        )));
    }
    Ok(())
}

/// 項目と節の外のノードが `of` に宣言できる語。`id` と `name` は項目だけ、`end` は項目と節だけ（REQ-schema-048）。
const OUTSIDE_ITEM_OF: &[OfKind] = &[OfKind::Line, OfKind::Raw];
/// 節が `of` に宣言できる語（REQ-schema-048）。
const SECTION_OF: &[OfKind] = &[OfKind::Line, OfKind::Raw, OfKind::End];

/// ノードが受け付けない `of` の語を schema_invalid の停止にし、受け付ける語の一覧を示す（REQ-schema-065）。
fn reject_of_words(
    extract: Option<&Extract>,
    node: &str,
    accepted: &[OfKind],
) -> Result<(), SchemaError> {
    let Some(extract) = extract else {
        return Ok(());
    };
    for (_, kind) in extract.of() {
        if !accepted.contains(kind) {
            let words: Vec<String> = accepted
                .iter()
                .map(|k| format!("\"{}\"", k.word()))
                .collect();
            return Err(SchemaError(format!(
                "{node} extract cannot use \"{}\" in of; accepted: {}",
                kind.word(),
                words.join(", ")
            )));
        }
    }
    Ok(())
}
