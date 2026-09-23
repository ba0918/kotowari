//! 文書の構造と閉じた世界の検証。

use crate::document::{Block, Document, Heading, Item};
use crate::finding::{Finding, FindingKind, RuleKind};
use crate::schema::{
    Bullets, Children, CodeBlock, Field, Item as ItemRule, Preamble, Reading, Repeat, Schema,
    Section, Statement, Table, Title, When, is_declared_field,
};
use std::collections::HashMap;

/// スキーマと文書の木から指摘を集める。`open` は閉じた世界を緩めるか（スキーマの `open` と CLI の `--open` を合わせた値）。
pub fn validate(schema: &Schema, document: &Document, open: bool) -> Vec<Finding> {
    let by_line;
    let document = match schema.reading {
        Reading::Paragraph => document,
        Reading::Line => {
            by_line = document.read_by_line();
            &by_line
        }
    };
    let mut findings = Vec::new();
    let doc_rule = &schema.document;

    if let Some(title) = &doc_rule.title {
        validate_title(title, &document.titles, &mut findings);
    } else if !open {
        // title 規則が無い `#` 見出しは閉じた世界で undeclared_heading（REQ-schema-001）
        for heading in &document.titles {
            findings.push(Finding::at(
                FindingKind::UndeclaredHeading,
                heading.line,
                format!("undeclared title heading \"{}\"", heading.text),
            ));
        }
    }

    // 宣言済みの前置部は open でも未宣言の行を誤りにする。宣言していない
    // 前置部の内側の行は閉じた世界だけで誤りにする。
    let effective_open = if doc_rule.preamble.is_some() {
        false
    } else {
        open
    };
    let rules = ContainerRules::for_preamble(doc_rule.preamble.as_ref());
    // 前置部と文書そのものは開始行を持つノードではないので、欠落の指摘は行を持たない（REQ-schema-008）
    validate_container(
        &rules,
        &document.preamble,
        effective_open,
        None,
        &mut findings,
    );

    let mut section_lines: HashMap<&str, Vec<usize>> = HashMap::new();
    for section in &document.sections {
        section_lines
            .entry(section.name.as_str())
            .or_default()
            .push(section.line);
    }

    for section in &document.sections {
        match doc_rule
            .sections
            .iter()
            .find(|def| def.name == section.name)
        {
            Some(def) => {
                let rules = ContainerRules::for_section(def);
                validate_container(
                    &rules,
                    &section.blocks,
                    false,
                    Some(section.line),
                    &mut findings,
                );
                validate_items(
                    def.item.as_ref(),
                    &section.items,
                    Some(section.line),
                    &mut findings,
                );
            }
            None => {
                // 宣言していない節は、閉じた世界で見出しと内側の行を誤りにする
                if !open {
                    findings.push(Finding::at(
                        FindingKind::UndeclaredHeading,
                        section.line,
                        format!("undeclared section heading \"{}\"", section.name),
                    ));
                    for block in &section.blocks {
                        push_undeclared_line(&mut findings, block);
                    }
                    for item in &section.items {
                        findings.push(Finding::at(
                            FindingKind::UndeclaredHeading,
                            item.line,
                            format!("undeclared item heading \"{}\"", item.id),
                        ));
                        for block in &item.blocks {
                            push_undeclared_line(&mut findings, block);
                        }
                    }
                }
            }
        }
    }

    for def in &doc_rule.sections {
        let empty: Vec<usize> = Vec::new();
        let lines = section_lines.get(def.name.as_str()).unwrap_or(&empty);
        check_occurrence(
            &Occurrence {
                lines,
                name: Some(&def.name),
                what: format!("section \"{}\"", def.name),
                container_line: None,
                rule_kind: RuleKind::Section,
            },
            bounds(def.required, def.repeat.as_ref()),
            def.repeat.is_some(),
            FindingKind::MissingRequiredSection,
            &mut findings,
        );
    }

    // 前置部領域の深さ3の見出し。題名より前の見出しは open では許し、閉じた
    // 世界では undeclared_heading（REQ-schema-001、REQ-schema-002）。題名より後の見出しは宣言済みの前置部の
    // 中の未宣言の構造として open でも undeclared_heading、その内側の行は
    // undeclared_line にする（REQ-schema-003）。前置部が未宣言のときは閉じた世界だけで
    // 同じ扱いになり、open では未宣言の構造ごと許す。
    if doc_rule.item.is_some() {
        // 文書の直下の項目を宣言したスキーマでは、最初の節より前の深さ3の見出しを
        // その項目として読む（REQ-schema-061）。含む節が無いので欠落の指摘は行を持たない
        validate_items(
            doc_rule.item.as_ref(),
            &document.preamble_items(),
            None,
            &mut findings,
        );
    }
    for stray in document
        .stray_preamble_headings
        .iter()
        .filter(|_| doc_rule.item.is_none())
    {
        let in_declared_preamble = !stray.before_title && doc_rule.preamble.is_some();
        if in_declared_preamble || !open {
            findings.push(Finding::at(
                FindingKind::UndeclaredHeading,
                stray.heading.line,
                format!("undeclared item heading \"{}\"", stray.heading.text),
            ));
            for block in &stray.blocks {
                push_undeclared_line(&mut findings, block);
            }
        }
    }

    for heading in &document.stray_headings {
        validate_stray_heading(heading, &mut findings);
    }

    // 行を持つ指摘には、その行の生の文字をそのまま添える（REQ-schema-008）
    for finding in &mut findings {
        if let Some(line) = finding.line {
            finding.raw = document.raw_line(line).map(str::to_string);
        }
    }

    findings
}

fn validate_title(title: &Title, headings: &[Heading], findings: &mut Vec<Finding>) {
    match headings.len() {
        0 => findings.push(Finding::new(
            FindingKind::MissingTitle,
            "the document has no level-1 heading".into(),
        )),
        // 2つ目以降の題名ごとに1件（REQ-schema-022）
        n if n > 1 => findings.extend(headings[1..].iter().map(|heading| {
            Finding::at(
                FindingKind::MultipleTitles,
                heading.line,
                "the document has more than one level-1 heading".into(),
            )
        })),
        _ => {
            let heading = &headings[0];
            if let Some(pattern) = &title.pattern
                && !pattern.is_match(&heading.text)
            {
                findings.push(Finding::at(
                    FindingKind::TitlePatternMismatch,
                    heading.line,
                    format!(
                        "title \"{}\" does not match pattern \"{}\"",
                        heading.text,
                        pattern.source()
                    ),
                ));
            }
        }
    }
}

fn validate_items(
    item_rule: Option<&ItemRule>,
    items: &[Item],
    container_line: Option<usize>,
    findings: &mut Vec<Finding>,
) {
    let Some(item_rule) = item_rule else {
        // 宣言済みの節の中に足された未宣言の項目。open でも見出しと内側の行を誤りにする（REQ-schema-003、REQ-schema-027）
        for item in items {
            findings.push(Finding::at(
                FindingKind::UndeclaredHeading,
                item.line,
                format!("undeclared item heading \"{}\"", item.id),
            ));
            for block in &item.blocks {
                push_undeclared_line(findings, block);
            }
        }
        return;
    };

    for item in items {
        if !item.has_id_separator {
            findings.push(Finding::at(
                FindingKind::InvalidId,
                item.line,
                format!("item heading \"{}\" has no \":\" separator", item.id),
            ));
        } else if let Some(id_pattern) = &item_rule.id
            && !id_pattern.is_match(&item.id)
        {
            findings.push(Finding::at(
                FindingKind::InvalidId,
                item.line,
                format!(
                    "item id \"{}\" does not match pattern \"{}\"",
                    item.id,
                    id_pattern.source()
                ),
            ));
        }
        let rules = ContainerRules::for_item(item_rule);
        validate_container(&rules, &item.blocks, false, Some(item.line), findings);
    }

    // 項目は宣言上の名前を持たないので、指摘にノードの名前を付けない（REQ-schema-008）
    let lines: Vec<usize> = items.iter().map(|item| item.line).collect();
    check_occurrence(
        &Occurrence {
            lines: &lines,
            name: None,
            what: "item".to_string(),
            container_line,
            rule_kind: RuleKind::Item,
        },
        bounds(item_rule.required, item_rule.repeat.as_ref()),
        true,
        FindingKind::MissingRequiredSection,
        findings,
    );
}

fn validate_stray_heading(heading: &Heading, findings: &mut Vec<Finding>) {
    // 前置部領域の深さ3の見出しは stray_preamble_headings で扱い、ここには
    // 深さ4以上の見出しだけが来る（REQ-schema-026）。深さ4以上は常に heading_level_mismatch。
    if heading.depth >= 4 {
        findings.push(Finding::at(
            FindingKind::HeadingLevelMismatch,
            heading.line,
            format!("heading at depth {} is not allowed", heading.depth),
        ));
    }
}

/// 1つの前置部・節・項目の中の行の規則。規則種別ごとの判定に使う。
struct ContainerRules<'a> {
    fields: &'a [Field],
    ordered: bool,
    statement: Option<&'a Statement>,
    bullets: Option<&'a Bullets>,
    table: Option<&'a Table>,
    codeblock: Option<&'a CodeBlock>,
}

impl<'a> ContainerRules<'a> {
    fn for_preamble(preamble: Option<&'a Preamble>) -> Self {
        match preamble {
            Some(p) => ContainerRules {
                fields: &p.fields,
                ordered: p.ordered,
                statement: p.statement.as_ref(),
                bullets: p.bullets.as_ref(),
                table: p.table.as_ref(),
                codeblock: p.codeblock.as_ref(),
            },
            None => Self::empty(),
        }
    }

    fn for_section(section: &'a Section) -> Self {
        ContainerRules {
            fields: &section.fields,
            ordered: section.ordered,
            statement: section.statement.as_ref(),
            bullets: section.bullets.as_ref(),
            table: section.table.as_ref(),
            codeblock: section.codeblock.as_ref(),
        }
    }

    fn for_item(item: &'a ItemRule) -> Self {
        ContainerRules {
            fields: &item.fields,
            ordered: item.ordered,
            statement: item.statement.as_ref(),
            bullets: item.bullets.as_ref(),
            table: item.table.as_ref(),
            codeblock: item.codeblock.as_ref(),
        }
    }

    fn empty() -> Self {
        ContainerRules {
            fields: &[],
            ordered: false,
            statement: None,
            bullets: None,
            table: None,
            codeblock: None,
        }
    }
}

/// 宣言されていない行の指摘をブロックの種別に応じた文言で作る。
/// 文の対象外の行種別（ブロック引用・水平線・画像など）は None を返す（REQ-schema-032）。
fn undeclared_line_for_block(block: &Block) -> Option<Finding> {
    // 呼ぶ側が生の行を読み直して種別を決めずに済むよう、読んだ規則種別を添える（REQ-schema-055）
    let (detail, line, rule_kind) = match block {
        Block::Field { name, line, .. } => (
            format!("undeclared field line \"{name}\""),
            *line,
            RuleKind::Field,
        ),
        Block::Bullet { text, line, .. } => (
            format!("undeclared bullet \"{text}\""),
            *line,
            RuleKind::Bullets,
        ),
        Block::OrderedList { text, line, .. } => (
            format!("undeclared ordered list \"{text}\""),
            *line,
            RuleKind::OrderedList,
        ),
        Block::Statement { text, line, .. } => (
            format!("undeclared statement \"{text}\""),
            *line,
            RuleKind::Statement,
        ),
        Block::Table { line, .. } => ("undeclared table".to_string(), *line, RuleKind::Table),
        Block::Code { line, .. } => (
            "undeclared code block".to_string(),
            *line,
            RuleKind::CodeBlock,
        ),
        Block::Other { .. } => return None,
    };
    Some(Finding::at(FindingKind::UndeclaredLine, line, detail).of_rule(rule_kind))
}

fn push_undeclared_line(findings: &mut Vec<Finding>, block: &Block) {
    if let Some(finding) = undeclared_line_for_block(block) {
        findings.push(finding);
    }
}

/// 箇条書きとして1本を検証する。箇条書きが宣言されていれば本数を数え、
/// pattern を照合する。無ければ undeclared_line（閉じた世界）。
/// `Block::Bullet` と、宣言された名前と一致しない `- 名前: 値` 行（TBL-schema-007）が共有する。
fn validate_bullet(
    rules: &ContainerRules,
    block: &Block,
    blocks: &[Block],
    bullet_lines: &mut Vec<usize>,
    open: bool,
    findings: &mut Vec<Finding>,
) {
    let (text, line) = match block {
        Block::Bullet { text, line, .. } => (text.as_str(), *line),
        Block::Field { text, line, .. } => (text.as_str(), *line),
        _ => return,
    };
    match rules.bullets {
        Some(bullets) => {
            bullet_lines.push(line);
            if when_allows(bullets.when.as_ref(), rules.fields, blocks)
                && let Some(pattern) = &bullets.pattern
                && !pattern.is_match(text)
            {
                findings.push(Finding::at(
                    FindingKind::BulletPatternMismatch,
                    line,
                    format!(
                        "bullet \"{text}\" does not match pattern \"{}\"",
                        pattern.source()
                    ),
                ));
            }
            // 親の bullets 規則が宣言されたとき、子は親の children の宣言に照合する（REQ-schema-031）。
            // when は required / pattern / enum の制約にだけ効く（REQ-schema-020）。照合は常に実行する。
            validate_children(block, bullets.children.as_ref(), findings);
        }
        None => {
            if !open {
                // 宣言されていない箇条書きと、その内側の子の行を undeclared_line にする。
                // 宣言していない構造の内側の行も undeclared_line（REQ-schema-001）
                push_undeclared_line(findings, block);
                push_undeclared_children(findings, block);
            }
        }
    }
}

/// 箇条書きの子の行を、親の `children` の宣言に照合する（REQ-schema-031）。
/// `children.fields` で宣言された名前と一致する `- 名前: 値` はフィールド行として
/// 検証し、一致しない `- 名前: 値` は箇条書きとして検証する。`children` に宣言が
/// 無い子、または親が `children` を持たないのに子リストがある場合は
/// undeclared_line にする。親の bullets 規則が宣言されているので、open でも
/// 宣言済みの構造の中の未宣言の子は undeclared_line になる（REQ-schema-003）。
fn validate_children(block: &Block, children: Option<&Children>, findings: &mut Vec<Finding>) {
    let child_blocks = block.children();
    let Some(children) = children else {
        push_undeclared_children(findings, block);
        return;
    };

    let mut child_field_lines: HashMap<&str, Vec<usize>> = HashMap::new();
    let mut child_bullet_lines: Vec<usize> = Vec::new();
    for child in child_blocks {
        match child {
            Block::Field {
                name, value, line, ..
            } if is_declared_field(&children.fields, name) => {
                child_field_lines
                    .entry(name.as_str())
                    .or_default()
                    .push(*line);
                let field = children
                    .fields
                    .iter()
                    .find(|f| f.name == *name)
                    .expect("is_declared_field が一致を保証する");
                if when_allows(field.when.as_ref(), &children.fields, child_blocks) {
                    validate_field_value(field, value, *line, findings);
                }
                // 子フィールドも children 宣言を持たない。宣言済み子フィールド行の
                // 下の子リストは undeclared_line（REQ-schema-001）
                push_undeclared_children(findings, child);
            }
            // 一致しない `- 名前: 値` 行は箇条書きとして検証する（TBL-schema-007）
            Block::Field { .. } | Block::Bullet { .. } => {
                validate_child_bullet(
                    child,
                    children,
                    child_blocks,
                    &mut child_bullet_lines,
                    findings,
                );
            }
            // 文・順序付きリスト・コードブロック・表などの子。children に規則が
            // 無いので undeclared_line（閉じた世界の対象外の行種別は無視）
            other => {
                push_undeclared_line(findings, other);
                push_undeclared_children(findings, other);
            }
        }
    }

    for field in &children.fields {
        if when_allows(field.when.as_ref(), &children.fields, child_blocks) {
            let empty: Vec<usize> = Vec::new();
            let lines = child_field_lines.get(field.name.as_str()).unwrap_or(&empty);
            check_occurrence(
                &Occurrence {
                    lines,
                    name: Some(&field.name),
                    what: format!("field \"{}\"", field.name),
                    container_line: Some(block.line()),
                    rule_kind: RuleKind::Field,
                },
                bounds(field.required, field.repeat.as_ref()),
                field.repeat.is_some(),
                FindingKind::MissingRequiredField,
                findings,
            );
        }
    }
    // 子は親の本数には数えず、children.bullets の規則で別に数える（REQ-schema-031）
    if let Some(child_bullets) = children.bullets.as_deref()
        && when_allows(child_bullets.when.as_ref(), &children.fields, child_blocks)
    {
        check_occurrence(
            &Occurrence {
                lines: &child_bullet_lines,
                name: None,
                what: "bullets".to_string(),
                container_line: Some(block.line()),
                rule_kind: RuleKind::Bullets,
            },
            bounds(child_bullets.required, child_bullets.repeat.as_ref()),
            child_bullets.repeat.is_some(),
            FindingKind::MissingBullets,
            findings,
        );
    }
}

/// 子の箇条書きを1本検証する。`children.bullets` が宣言されていれば本数を数え、
/// pattern を照合して、さらに深い入れ子を再帰する。宣言されていなければ
/// undeclared_line（REQ-schema-001）。`sibling_blocks` は同じ children ノードの下の兄弟の
/// 行で、when の探索スコープに使う（REQ-schema-021）。
fn validate_child_bullet(
    block: &Block,
    children: &Children,
    sibling_blocks: &[Block],
    bullet_lines: &mut Vec<usize>,
    findings: &mut Vec<Finding>,
) {
    let Some(bullets) = children.bullets.as_deref() else {
        // children に宣言が無い子は undeclared_line（REQ-schema-001）
        push_undeclared_line(findings, block);
        push_undeclared_children(findings, block);
        return;
    };
    let (text, line) = match block {
        Block::Bullet { text, line, .. } => (text.as_str(), *line),
        Block::Field { text, line, .. } => (text.as_str(), *line),
        _ => return,
    };
    bullet_lines.push(line);
    if when_allows(bullets.when.as_ref(), &children.fields, sibling_blocks)
        && let Some(pattern) = &bullets.pattern
        && !pattern.is_match(text)
    {
        findings.push(Finding::at(
            FindingKind::BulletPatternMismatch,
            line,
            format!(
                "bullet \"{text}\" does not match pattern \"{}\"",
                pattern.source()
            ),
        ));
    }
    validate_children(block, bullets.children.as_ref(), findings);
}

/// 箇条書きの子の行を undeclared_line にする。子がさらに子を持つときも再帰する。
fn push_undeclared_children(findings: &mut Vec<Finding>, block: &Block) {
    for child in block.children() {
        push_undeclared_line(findings, child);
        push_undeclared_children(findings, child);
    }
}

fn validate_container(
    rules: &ContainerRules,
    blocks: &[Block],
    open: bool,
    container_line: Option<usize>,
    findings: &mut Vec<Finding>,
) {
    let mut field_lines: HashMap<&str, Vec<usize>> = HashMap::new();
    let mut statement_lines: Vec<usize> = Vec::new();
    let mut bullet_lines: Vec<usize> = Vec::new();
    let mut table_lines: Vec<usize> = Vec::new();
    let mut code_lines: Vec<usize> = Vec::new();
    let mut ordered_seen: Vec<(usize, usize)> = Vec::new();
    let table_selection = rules.table.map(|table| table.selected_line(blocks));

    for block in blocks {
        match block {
            Block::Field {
                name, value, line, ..
            } => {
                match rules.fields.iter().position(|f| f.name == *name) {
                    Some(idx) => {
                        field_lines.entry(name.as_str()).or_default().push(*line);
                        ordered_seen.push((idx, *line));
                        let field = &rules.fields[idx];
                        if when_allows(field.when.as_ref(), rules.fields, blocks) {
                            validate_field_value(field, value, *line, findings);
                        }
                        // フィールド行は children 宣言を持たない。宣言済みフィールド行の
                        // 下の子リストは undeclared_line（REQ-schema-001）
                        push_undeclared_children(findings, block);
                    }
                    None => {
                        // TBL-schema-007: 宣言された名前と一致しない `- 名前: 値` 行は
                        // 箇条書きとして扱う。pattern は元の行に適用する
                        validate_bullet(rules, block, blocks, &mut bullet_lines, open, findings);
                    }
                }
            }
            Block::Bullet { .. } => {
                validate_bullet(rules, block, blocks, &mut bullet_lines, open, findings)
            }
            Block::Statement { text, line, .. } => match rules.statement {
                Some(statement) => {
                    statement_lines.push(*line);
                    if when_allows(statement.when.as_ref(), rules.fields, blocks) {
                        validate_statement_value(statement, text, *line, findings);
                    }
                }
                None => {
                    if !open {
                        push_undeclared_line(findings, block);
                    }
                }
            },
            Block::Table {
                header,
                rows,
                line,
                row_lines,
            } => match rules.table {
                // select で選ばれなかった表は宣言していない表として扱う（REQ-schema-059）
                Some(table) if table_selection.is_none_or(|s| s.takes(*line)) => {
                    table_lines.push(*line);
                    validate_table_shape(table, header, rows, *line, row_lines, findings);
                }
                _ => {
                    if !open {
                        push_undeclared_line(findings, block);
                    }
                }
            },
            Block::Code { lang, value, line } => match rules.codeblock {
                Some(codeblock) => {
                    code_lines.push(*line);
                    validate_codeblock_shape(codeblock, lang.as_deref(), value, *line, findings);
                }
                None => {
                    if !open {
                        push_undeclared_line(findings, block);
                    }
                }
            },
            // ブロック引用・水平線・画像などの文の対象外の行種別は閉じた世界でも無視する（REQ-schema-032）
            Block::Other { .. } => {}
            // 順序付きリストはどの規則種別にも属さない。閉じた世界では undeclared_line にする（TBL-schema-007）
            Block::OrderedList { .. } => {
                if !open {
                    push_undeclared_line(findings, block);
                }
            }
        }
    }

    for field in rules.fields {
        if when_allows(field.when.as_ref(), rules.fields, blocks) {
            let empty: Vec<usize> = Vec::new();
            let lines = field_lines.get(field.name.as_str()).unwrap_or(&empty);
            check_occurrence(
                &Occurrence {
                    lines,
                    name: Some(&field.name),
                    what: format!("field \"{}\"", field.name),
                    container_line,
                    rule_kind: RuleKind::Field,
                },
                bounds(field.required, field.repeat.as_ref()),
                field.repeat.is_some(),
                FindingKind::MissingRequiredField,
                findings,
            );
        }
    }
    if let Some(statement) = rules.statement
        && when_allows(statement.when.as_ref(), rules.fields, blocks)
    {
        check_occurrence(
            &Occurrence {
                lines: &statement_lines,
                name: None,
                what: "statement".to_string(),
                container_line,
                rule_kind: RuleKind::Statement,
            },
            bounds(statement.required, statement.repeat.as_ref()),
            statement.repeat.is_some(),
            FindingKind::MissingStatement,
            findings,
        );
    }
    if let Some(bullets) = rules.bullets
        && when_allows(bullets.when.as_ref(), rules.fields, blocks)
    {
        check_occurrence(
            &Occurrence {
                lines: &bullet_lines,
                name: None,
                what: "bullets".to_string(),
                container_line,
                rule_kind: RuleKind::Bullets,
            },
            bounds(bullets.required, bullets.repeat.as_ref()),
            bullets.repeat.is_some(),
            FindingKind::MissingBullets,
            findings,
        );
    }
    if let Some(table) = rules.table {
        check_occurrence(
            &Occurrence {
                lines: &table_lines,
                name: None,
                what: "table".to_string(),
                container_line,
                rule_kind: RuleKind::Table,
            },
            bounds(table.required, table.repeat.as_ref()),
            table.repeat.is_some(),
            FindingKind::MissingTable,
            findings,
        );
    }
    if let Some(codeblock) = rules.codeblock {
        check_occurrence(
            &Occurrence {
                lines: &code_lines,
                name: None,
                what: "code block".to_string(),
                container_line,
                rule_kind: RuleKind::CodeBlock,
            },
            bounds(codeblock.required, codeblock.repeat.as_ref()),
            codeblock.repeat.is_some(),
            FindingKind::MissingCodeblock,
            findings,
        );
    }

    if rules.ordered {
        let mut prev: Option<usize> = None;
        for (idx, line) in &ordered_seen {
            if let Some(prev_idx) = prev
                && *idx < prev_idx
            {
                findings.push(
                    Finding::at(
                        FindingKind::FieldOrderMismatch,
                        *line,
                        "fields are not in the declared order".into(),
                    )
                    .of_node(&rules.fields[*idx].name),
                );
                break;
            }
            prev = Some(*idx);
        }
    }
}

/// `separator` で分けた要素を、前後の空白を取り除いて返す（REQ-schema-029）。
/// 空の要素は空文字列として残す。
/// 文を、宣言された pattern・enum に照らす。
fn validate_statement_value(
    statement: &Statement,
    text: &str,
    line: usize,
    findings: &mut Vec<Finding>,
) {
    if let Some(pattern) = &statement.pattern
        && !pattern.is_match(text)
    {
        findings.push(Finding::at(
            FindingKind::StatementPatternMismatch,
            line,
            format!(
                "statement \"{text}\" does not match pattern \"{}\"",
                pattern.source()
            ),
        ));
    }
    if let Some(allowed) = &statement.r#enum
        && !allowed.iter().any(|e| e == text)
    {
        findings.push(Finding::at(
            FindingKind::StatementEnumInvalid,
            line,
            format!("statement \"{text}\" is not one of {allowed:?}"),
        ));
    }
}

/// 表を、宣言されたヘッダに照らす（REQ-schema-033）。header を宣言しないときは
/// ヘッダも列数も検査しない。
fn validate_table_shape(
    table: &Table,
    header: &[String],
    rows: &[Vec<String>],
    line: usize,
    row_lines: &[usize],
    findings: &mut Vec<Finding>,
) {
    let Some(expected) = &table.header else {
        return;
    };
    if header != expected.as_slice() {
        findings.push(Finding::at(
            FindingKind::TableHeaderMismatch,
            line,
            format!("table header {header:?} does not match expected {expected:?}"),
        ));
    }
    // ヘッダより多いセルは文書を読むときに捨ててあるので、足りない行だけが残る（REQ-schema-033）
    for (index, row) in rows.iter().enumerate() {
        if row.len() < header.len() {
            // 違反したのはその行なので、ヘッダの行ではなくその行を指す（REQ-schema-008）
            let row_line = row_lines.get(index).copied().unwrap_or(line);
            findings.push(Finding::at(
                FindingKind::TableHeaderMismatch,
                row_line,
                format!(
                    "row has {} columns but the header has {}",
                    row.len(),
                    header.len()
                ),
            ));
        }
    }
}

/// コードブロックを、宣言された言語と行の pattern に照らす（REQ-schema-034）。
/// 空行は行の照合の対象外。
fn validate_codeblock_shape(
    codeblock: &CodeBlock,
    lang: Option<&str>,
    value: &str,
    line: usize,
    findings: &mut Vec<Finding>,
) {
    if let Some(expected) = &codeblock.lang
        && lang != Some(expected.as_str())
    {
        findings.push(Finding::at(
            FindingKind::CodeblockLangMismatch,
            line,
            format!("code block language {lang:?} does not match \"{expected}\""),
        ));
    }
    let Some(patterns) = &codeblock.lines else {
        return;
    };
    for (i, code_line) in value.lines().enumerate() {
        // REQ-schema-034: 取り除くのは行頭の空白だけ。末尾まで落とすと "...$" の照合が変わる
        let code_line = code_line.trim_start();
        if code_line.is_empty() {
            continue;
        }
        if !patterns.iter().any(|p| p.is_match(code_line)) {
            findings.push(Finding::at(
                FindingKind::CodeblockLineMismatch,
                line,
                format!("line {} does not match any allowed pattern", i + 1),
            ));
        }
    }
}

/// フィールド行の値を、宣言された区切り・pattern・enum に照らす（REQ-schema-029）。
/// 前置部と節のフィールド行にも、箇条書きの子フィールド行にも同じ規則を当てる。
fn validate_field_value(field: &Field, value: &str, line: usize, findings: &mut Vec<Finding>) {
    // 制約は区切った要素ごとに課す（REQ-schema-029）。照合の前に前後の空白を取り除く
    let values: Vec<String> = match field.effective_separator() {
        Some(sep) => split_trimmed(value, sep),
        None => vec![value.to_string()],
    };
    for v in &values {
        if let Some(pattern) = &field.pattern
            && !pattern.is_match(v)
        {
            findings.push(
                Finding::at(
                    FindingKind::FieldPatternMismatch,
                    line,
                    format!(
                        "value \"{v}\" does not match pattern \"{}\"",
                        pattern.source()
                    ),
                )
                .of_node(&field.name),
            );
        }
        if let Some(allowed) = &field.r#enum
            && !allowed.iter().any(|e| e == v)
        {
            findings.push(
                Finding::at(
                    FindingKind::FieldEnumInvalid,
                    line,
                    format!("value \"{v}\" is not one of {allowed:?}"),
                )
                .of_node(&field.name),
            );
        }
    }
}

fn split_trimmed(value: &str, sep: &str) -> Vec<String> {
    value.split(sep).map(|s| s.trim().to_string()).collect()
}

/// `when` の条件を評価する。参照フィールドが無いとき eq は偽、ne は真。
/// 宣言された名前と一致しない `- 名前: 値` 行はフィールド行ではなく箇条書きなので、
/// 参照フィールドとしては数えない（TBL-schema-007、REQ-schema-021）。
fn when_allows(when: Option<&When>, fields: &[Field], blocks: &[Block]) -> bool {
    let Some(when) = when else {
        return true;
    };
    let value = blocks.iter().find_map(|b| match b {
        Block::Field { name, value, .. }
            if name == &when.field && is_declared_field(fields, name) =>
        {
            Some(value.as_str())
        }
        _ => None,
    });
    match value {
        Some(value) => match (&when.eq, &when.ne) {
            (Some(eq), _) => value == eq,
            (_, Some(ne)) => value != ne,
            _ => unreachable!("parse_schema が when の演算子を保証する"),
        },
        None => when.eq.is_none(),
    }
}

fn bounds(required: Option<bool>, repeat: Option<&Repeat>) -> (u64, Option<u64>) {
    if let Some(repeat) = repeat {
        (repeat.min.unwrap_or(0), repeat.max)
    } else if required == Some(false) {
        (0, Some(1))
    } else {
        (1, Some(1))
    }
}

/// 出現回数を数える対象のノード。
struct Occurrence<'a> {
    /// 現れた行（1始まり）。出現回数はこの長さ
    lines: &'a [usize],
    /// スキーマが宣言したノードの名前。節とフィールド行だけが持つ（REQ-schema-008）
    name: Option<&'a str>,
    /// detail の中でノードを指す言い回し
    what: String,
    /// それを含むノードの開始行。欠落の指摘の行に使う（REQ-schema-008）
    container_line: Option<usize>,
    /// 数えたノードの規則種別。出現回数の指摘に添える（REQ-schema-057）
    rule_kind: RuleKind,
}

fn check_occurrence(
    occurrence: &Occurrence,
    bounds: (u64, Option<u64>),
    repeat_style: bool,
    missing_kind: FindingKind,
    findings: &mut Vec<Finding>,
) {
    let count = occurrence.lines.len() as u64;
    let what = &occurrence.what;
    let (min, max) = bounds;
    let finding = if count < min {
        // 欠落したノードには行が無いので、それを含むノードの開始行を指す（REQ-schema-008）
        let line = occurrence.container_line;
        if repeat_style {
            // 下限の指摘は数えたノードの規則種別を持つ（REQ-schema-057）
            Finding::maybe_at(
                FindingKind::RepeatMinNotMet,
                line,
                format!("{what} appears {count} time(s), minimum is {min}"),
            )
            .of_rule(occurrence.rule_kind)
        } else {
            Finding::maybe_at(
                missing_kind,
                line,
                format!("{what} is required but missing"),
            )
        }
    } else if let Some(max) = max
        && count > max
    {
        // 上限を超えた最初のノードが違反したノードなので、その行を指す（REQ-schema-008）
        let line = usize::try_from(max)
            .ok()
            .and_then(|index| occurrence.lines.get(index))
            .copied();
        // 上限の指摘は数えたノードの規則種別を持つ（REQ-schema-057）
        Finding::maybe_at(
            FindingKind::RepeatMaxExceeded,
            line,
            format!("{what} appears {count} time(s), maximum is {max}"),
        )
        .of_rule(occurrence.rule_kind)
    } else {
        return;
    };
    findings.push(match occurrence.name {
        Some(name) => finding.of_node(name),
        None => finding,
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::Document;
    use crate::schema::parse_schema;

    fn validate_src(schema_yaml: &str, doc: &str, open: bool) -> Vec<Finding> {
        let schema = parse_schema(schema_yaml).unwrap();
        let document = Document::parse(doc).unwrap();
        validate(&schema, &document, open)
    }

    fn kinds(findings: &[Finding]) -> Vec<FindingKind> {
        findings.iter().map(|f| f.kind).collect()
    }

    const SCHEMA: &str = r#"
name: adr
document:
  title:
    pattern: "^ADR-\\d{4}:"
  preamble:
    fields:
      - name: 状態
  sections:
    - name: 状況
      statement:
        required: false
    - name: 決定
      statement:
        required: false
"#;

    fn ok_body() -> &'static str {
        "# ADR-0001: 印\n\n- 状態: 承認済み\n\n## 状況\n\n背景。\n\n## 決定\n\n判断。\n"
    }

    // @kotowari[REQ-schema-022]
    #[test]
    fn missing_title_is_found() {
        let doc = "## 状況\n\n背景。\n\n## 決定\n\n判断。\n";
        let findings = validate_src(SCHEMA, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::MissingTitle));
    }

    // @kotowari[REQ-schema-022, EX-schema-010]
    #[test]
    fn multiple_titles_is_found() {
        let doc = "# ADR-0001: a\n\n# ADR-0002: b\n\n## 状況\n\n背景。\n\n## 決定\n\n判断。\n";
        let findings = validate_src(SCHEMA, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::MultipleTitles));
    }

    // @kotowari[REQ-schema-022]
    #[test]
    fn title_pattern_mismatch_is_found() {
        let doc = "# テストの印\n\n- 状態: 承認済み\n\n## 状況\n\n背景。\n\n## 決定\n\n判断。\n";
        let findings = validate_src(SCHEMA, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::TitlePatternMismatch));
    }

    // @kotowari[REQ-schema-001, REQ-schema-024]
    #[test]
    fn undeclared_heading_is_found() {
        let doc =
            "# ADR-0001: a\n\n## 状況\n\n背景。\n\n## 決定\n\n判断。\n\n## 補足\n\n余計な節。\n";
        let findings = validate_src(SCHEMA, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::UndeclaredHeading));
    }

    // @kotowari[REQ-schema-001]
    #[test]
    fn undeclared_line_is_found() {
        let doc = "# ADR-0001: a\n\n## 状況\n\n- 宣言外の箇条書き\n\n## 決定\n\n判断。\n";
        let findings = validate_src(SCHEMA, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::UndeclaredLine));
    }

    // @kotowari[REQ-schema-030]
    #[test]
    fn continuation_paragraph_is_part_of_the_bullet_not_undeclared() {
        let schema = "document:\n  sections:\n    - name: 理由\n      bullets:\n        repeat: { min: 0 }\n";
        let doc = "## 理由\n\n- 親\n\n  続きの段落\n";
        let findings = validate_src(schema, doc, false);
        assert!(!kinds(&findings).contains(&FindingKind::UndeclaredLine));
    }

    // @kotowari[REQ-schema-030, REQ-schema-032]
    #[test]
    fn continuation_paragraph_does_not_count_as_statement() {
        let schema = "document:\n  sections:\n    - name: 理由\n      statement:\n        required: true\n      bullets:\n        repeat: { min: 0 }\n";
        let doc = "## 理由\n\n- 親\n\n  続きの段落\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::MissingStatement));
    }

    // @kotowari[REQ-schema-030]
    #[test]
    fn bullet_pattern_is_not_applied_to_continuation_paragraphs() {
        let schema = "document:\n  sections:\n    - name: 理由\n      bullets:\n        repeat: { min: 0 }\n        pattern: \"^親\"\n";
        let doc = "## 理由\n\n- 親\n\n  続きの段落はパターンに合わない\n";
        let findings = validate_src(schema, doc, false);
        assert!(!kinds(&findings).contains(&FindingKind::BulletPatternMismatch));
    }

    // @kotowari[REQ-schema-001, REQ-schema-031]
    #[test]
    fn code_block_child_of_list_item_is_undeclared_in_closed_world() {
        let schema = "document:\n  sections:\n    - name: 理由\n";
        let doc = "## 理由\n\n- 親\n\n  ```\n  x = 1\n  ```\n";
        let findings = validate_src(schema, doc, false);
        let details: Vec<&str> = findings
            .iter()
            .filter(|f| f.kind == FindingKind::UndeclaredLine)
            .map(|f| f.detail.as_str())
            .collect();
        assert!(
            details.contains(&"undeclared code block"),
            "リスト項目の中のコードブロックが undeclared_line になる: {details:?}"
        );
    }

    // @kotowari[REQ-schema-001, REQ-schema-031]
    #[test]
    fn code_block_as_first_child_of_list_item_is_undeclared_in_closed_world() {
        let schema = "document:\n  sections:\n    - name: 理由\n      bullets:\n        repeat: { min: 0 }\n";
        let doc = "## 理由\n\n- ```python\n  x = 1\n  ```\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            kinds(&findings).contains(&FindingKind::UndeclaredLine),
            "先頭がコードブロックのリスト項目は undeclared_line になる: {:?}",
            kinds(&findings)
        );
    }

    // @kotowari[REQ-schema-001, REQ-schema-031]
    #[test]
    fn table_as_first_child_of_list_item_is_undeclared_in_closed_world() {
        let schema = "document:\n  sections:\n    - name: 理由\n      bullets:\n        repeat: { min: 0 }\n";
        let doc = "## 理由\n\n- | a | b |\n  |---|---|\n  | 1 | 2 |\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            kinds(&findings).contains(&FindingKind::UndeclaredLine),
            "先頭が表のリスト項目は undeclared_line になる: {:?}",
            kinds(&findings)
        );
    }

    // @kotowari[REQ-schema-001, REQ-schema-031]
    #[test]
    fn table_child_of_list_item_is_undeclared_in_closed_world() {
        let schema = "document:\n  sections:\n    - name: 理由\n";
        let doc = "## 理由\n\n- 親\n\n  | a | b |\n  |---|---|\n  | 1 | 2 |\n";
        let findings = validate_src(schema, doc, false);
        let details: Vec<&str> = findings
            .iter()
            .filter(|f| f.kind == FindingKind::UndeclaredLine)
            .map(|f| f.detail.as_str())
            .collect();
        assert!(
            details.contains(&"undeclared table"),
            "リスト項目の中の表が undeclared_line になる: {details:?}"
        );
    }

    // @kotowari[REQ-schema-001, REQ-schema-032]
    #[test]
    fn paragraph_after_code_block_lead_is_undeclared_line_in_closed_world() {
        let schema = "document:\n  sections:\n    - name: 理由\n      bullets:\n        repeat: { min: 0 }\n      codeblock:\n        required: false\n        lang: python\n";
        let doc = "## 理由\n\n- ```python\n  x = 1\n  ```\n\n  後続の段落\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            kinds(&findings).contains(&FindingKind::UndeclaredLine),
            "先頭がコードブロックのリスト項目の後続段落は undeclared_line になる: {:?}",
            kinds(&findings)
        );
    }

    // @kotowari[REQ-schema-001, REQ-schema-032]
    #[test]
    fn paragraph_after_table_lead_is_undeclared_line_in_closed_world() {
        let schema = "document:\n  sections:\n    - name: 理由\n      table:\n        required: false\n        header: [a, b]\n      bullets:\n        repeat: { min: 0 }\n";
        let doc = "## 理由\n\n- | a | b |\n  |---|---|\n  | 1 | 2 |\n\n  後続の段落\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            kinds(&findings).contains(&FindingKind::UndeclaredLine),
            "先頭が表のリスト項目の後続段落は undeclared_line になる: {:?}",
            kinds(&findings)
        );
    }

    // @kotowari[REQ-schema-032]
    #[test]
    fn paragraph_after_code_block_lead_counts_as_a_statement() {
        let schema = "document:\n  sections:\n    - name: 状況\n      statement:\n        required: true\n      codeblock:\n        required: false\n        lang: python\n      bullets:\n        repeat: { min: 0 }\n";
        let doc = "## 状況\n\n- ```python\n  x = 1\n  ```\n\n  後続の段落\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            !kinds(&findings).contains(&FindingKind::MissingStatement),
            "先頭がコードブロックのリスト項目の後続段落は文として数える: {:?}",
            kinds(&findings)
        );
    }

    // @kotowari[REQ-schema-032]
    #[test]
    fn paragraph_after_code_block_lead_is_subject_to_statement_pattern() {
        let schema = "document:\n  sections:\n    - name: 状況\n      statement:\n        pattern: \"^状況\"\n      codeblock:\n        required: false\n        lang: python\n      bullets:\n        repeat: { min: 0 }\n";
        let doc = "## 状況\n\n- ```python\n  x = 1\n  ```\n\n  後続の段落\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            kinds(&findings).contains(&FindingKind::StatementPatternMismatch),
            "先頭がコードブロックのリスト項目の後続段落に文の pattern を適用する: {:?}",
            kinds(&findings)
        );
    }

    // @kotowari[REQ-schema-032]
    #[test]
    fn image_only_paragraph_after_code_block_lead_is_not_a_statement() {
        let schema = "document:\n  sections:\n    - name: 状況\n      statement:\n        required: true\n      codeblock:\n        required: false\n        lang: python\n      bullets:\n        repeat: { min: 0 }\n";
        let doc = "## 状況\n\n- ```python\n  x = 1\n  ```\n\n  ![alt](img.png)\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            kinds(&findings).contains(&FindingKind::MissingStatement),
            "先頭がコードブロックのリスト項目の画像だけの段落は文に数えない（REQ-schema-032）: {:?}",
            kinds(&findings)
        );
        assert!(
            !kinds(&findings).contains(&FindingKind::UndeclaredLine),
            "先頭がコードブロックのリスト項目の画像だけの段落は undeclared_line にしない（REQ-schema-032）: {:?}",
            kinds(&findings)
        );
    }

    // @kotowari[REQ-schema-031]
    #[test]
    fn nested_list_items_are_not_counted_as_top_level_bullets() {
        // 親の bullets の本数はトップレベルの親だけを数え、子は数えない（REQ-schema-031）。
        let schema = "document:\n  sections:\n    - name: 理由\n      bullets:\n        repeat: { min: 2 }\n";
        let doc = "## 理由\n\n- 親\n  - 子\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            kinds(&findings).contains(&FindingKind::RepeatMinNotMet),
            "子は親の本数に数えず、min: 2 が満たされない（REQ-schema-031）: {:?}",
            kinds(&findings)
        );
    }

    // @kotowari[REQ-schema-019]
    #[test]
    fn two_top_level_bullets_satisfy_min_two() {
        let schema = "document:\n  sections:\n    - name: 理由\n      bullets:\n        repeat: { min: 2 }\n";
        let doc = "## 理由\n\n- 親\n  - 子\n- 別\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            !kinds(&findings).contains(&FindingKind::RepeatMinNotMet),
            "トップレベルの親が2本あれば満たす: {:?}",
            kinds(&findings)
        );
    }

    // @kotowari[REQ-schema-031]
    #[test]
    fn declared_child_field_passes() {
        let schema = r#"
document:
  sections:
    - name: 決定
      bullets:
        repeat: { min: 0 }
        children:
          fields:
            - name: superseded_by
"#;
        let doc = "## 決定\n\n- A22 判断の記録\n  - superseded_by: [A5]\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            !kinds(&findings).contains(&FindingKind::UndeclaredLine),
            "宣言された子フィールドは通る（REQ-schema-031）: {:?}",
            kinds(&findings)
        );
        assert!(!kinds(&findings).contains(&FindingKind::MissingRequiredField));
    }

    // @kotowari[REQ-schema-028, REQ-schema-031]
    #[test]
    fn undeclared_child_field_name_is_a_bullet_and_undeclared_without_rule() {
        let schema = r#"
document:
  sections:
    - name: 決定
      bullets:
        repeat: { min: 0 }
        children:
          fields:
            - name: superseded_by
"#;
        let doc = "## 決定\n\n- A22 判断の記録\n  - 備考: 補足\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            kinds(&findings).contains(&FindingKind::UndeclaredLine),
            "一致しない `- 名前: 値` は箇条書きとして扱い、children.bullets が無いので undeclared_line（TBL-schema-007、REQ-schema-031）: {:?}",
            kinds(&findings)
        );
    }

    // @kotowari[REQ-schema-031]
    #[test]
    fn child_list_without_children_rule_is_undeclared_line() {
        let schema = "document:\n  sections:\n    - name: 理由\n      bullets:\n        repeat: { min: 0 }\n";
        let doc = "## 理由\n\n- 親\n  - 子\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            kinds(&findings).contains(&FindingKind::UndeclaredLine),
            "親が children を持たないのに子リストがある場合は undeclared_line（REQ-schema-001）: {:?}",
            kinds(&findings)
        );
    }

    // @kotowari[REQ-schema-031]
    #[test]
    fn child_list_under_declared_field_is_undeclared_line() {
        // フィールド行は children 宣言を持たない。宣言済みフィールド行の下の
        // 子リストは、閉じた世界では undeclared_line（REQ-schema-001）。
        let schema = r#"
document:
  sections:
    - name: 状況
      fields:
        - name: 状態
      statement:
        required: false
"#;
        let doc = "## 状況\n\n- 状態: 承認済み\n  - 子箇条書き\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            kinds(&findings).contains(&FindingKind::UndeclaredLine),
            "宣言済みフィールド行の下の子リストは undeclared_line（REQ-schema-001）: {:?}",
            kinds(&findings)
        );
    }

    // @kotowari[REQ-schema-031]
    #[test]
    fn child_list_under_declared_child_field_is_undeclared_line() {
        // 子フィールド（children.fields の宣言）も children 宣言を持たない。
        // その下の子リストは undeclared_line（REQ-schema-001）。
        let schema = r#"
document:
  sections:
    - name: 決定
      bullets:
        repeat: { min: 0 }
        children:
          fields:
            - name: superseded_by
"#;
        let doc = "## 決定\n\n- A22 判断の記録\n  - superseded_by: [A5]\n    - さらに子\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            kinds(&findings).contains(&FindingKind::UndeclaredLine),
            "宣言済みの子フィールド行の下の子リストは undeclared_line（REQ-schema-001）: {:?}",
            kinds(&findings)
        );
    }

    // @kotowari[REQ-schema-031, REQ-schema-028]
    #[test]
    fn a_child_line_with_an_undeclared_name_is_checked_as_a_child_bullet() {
        let schema = r#"
document:
  sections:
    - name: 決定
      bullets:
        repeat: { min: 0 }
        children:
          bullets:
            repeat: { min: 0 }
            pattern: "^子"
"#;
        let doc = "## 決定\n\n- 親\n  - 名前: 値\n";
        let findings = validate_src(schema, doc, false);
        assert_eq!(
            kinds(&findings),
            vec![FindingKind::BulletPatternMismatch],
            "宣言していない名前の子の行は子の箇条書きとして照合する"
        );
    }

    // @kotowari[REQ-schema-021]
    #[test]
    fn child_bullet_when_references_a_sibling_field() {
        // children.bullets の when は、同じ children ノードの下の兄弟の
        // children.fields を参照する（REQ-schema-021）。when が真のときだけ pattern が効く。
        let schema = r#"
document:
  sections:
    - name: 決定
      bullets:
        repeat: { min: 0 }
        children:
          fields:
            - name: 種類
          bullets:
            repeat: { min: 0 }
            pattern: "^子"
            when: { field: 種類, eq: algorithm }
"#;
        let doc = "## 決定\n\n- 親\n  - 種類: algorithm\n  - 違反\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            kinds(&findings).contains(&FindingKind::BulletPatternMismatch),
            "children.bullets の when は兄弟の children.fields を参照する（REQ-schema-021）: {:?}",
            kinds(&findings)
        );
    }

    // @kotowari[REQ-schema-020, REQ-schema-031]
    #[test]
    fn children_are_validated_when_parent_bullets_when_is_true() {
        // 親の bullets の when が真のとき、子は children の宣言に照合する（REQ-schema-031）。
        let schema = r#"
document:
  sections:
    - name: 決定
      fields:
        - name: 種類
      bullets:
        repeat: { min: 0 }
        when: { field: 種類, eq: algorithm }
        children:
          fields:
            - name: superseded_by
"#;
        let doc = "## 決定\n\n- 種類: algorithm\n\n- A22 判断の記録\n  - 備考: 補足\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            kinds(&findings).contains(&FindingKind::UndeclaredLine),
            "when が真のとき子の未宣言行は undeclared_line（REQ-schema-031）: {:?}",
            kinds(&findings)
        );
    }

    // @kotowari[REQ-schema-020, REQ-schema-031]
    #[test]
    fn children_are_validated_when_parent_bullets_when_is_false() {
        // 親の bullets の when は required / pattern / enum にだけ効く（REQ-schema-020）。
        // children の照合（REQ-schema-031）は when の真偽に関わらず実行する。
        let schema = r#"
document:
  sections:
    - name: 決定
      fields:
        - name: 種類
      bullets:
        repeat: { min: 0 }
        when: { field: 種類, eq: algorithm }
        children:
          fields:
            - name: superseded_by
"#;
        let doc = "## 決定\n\n- 種類: other\n\n- A22 判断の記録\n  - 備考: 補足\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            kinds(&findings).contains(&FindingKind::UndeclaredLine),
            "親の bullets の when が偽でも子は children の宣言に照合する（REQ-schema-031）: {:?}",
            kinds(&findings)
        );
    }

    // @kotowari[REQ-schema-031]
    #[test]
    fn recursive_children_bullets_match() {
        let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        children:
          bullets:
            repeat: { min: 0 }
            pattern: "^子"
            children:
              bullets:
                repeat: { min: 0 }
                pattern: "^孫"
"#;
        let doc = "## 理由\n\n- 親\n  - 子\n    - 孫\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            !kinds(&findings).contains(&FindingKind::BulletPatternMismatch),
            "再帰的な children.bullets に照合する（REQ-schema-031）: {:?}",
            kinds(&findings)
        );
    }

    // @kotowari[REQ-schema-031]
    #[test]
    fn recursive_children_bullets_pattern_is_enforced() {
        let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        children:
          bullets:
            repeat: { min: 0 }
            pattern: "^子"
            children:
              bullets:
                repeat: { min: 0 }
                pattern: "^孫"
"#;
        let doc = "## 理由\n\n- 親\n  - 子\n    - 違反\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            kinds(&findings).contains(&FindingKind::BulletPatternMismatch),
            "子の子の箇条書きにも children.bullets の pattern を適用する（REQ-schema-031）: {:?}",
            kinds(&findings)
        );
    }

    // @kotowari[REQ-schema-019, REQ-schema-031]
    #[test]
    fn child_bullets_are_counted_by_children_rule_not_parent_repeat() {
        let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 1 }
        children:
          bullets:
            repeat: { min: 2 }
"#;
        let ok = validate_src(schema, "## 理由\n\n- 親\n  - 子1\n  - 子2\n", false);
        assert!(
            !kinds(&ok).contains(&FindingKind::RepeatMinNotMet),
            "親1本・子2本で両方満たす（REQ-schema-019、REQ-schema-031）: {:?}",
            kinds(&ok)
        );
        let short = validate_src(schema, "## 理由\n\n- 親\n  - 子1\n", false);
        assert!(
            kinds(&short).contains(&FindingKind::RepeatMinNotMet),
            "子が1本しか無ければ children.bullets の min を満たさない（REQ-schema-019、REQ-schema-031）: {:?}",
            kinds(&short)
        );
    }

    // @kotowari[REQ-schema-003, REQ-schema-031]
    #[test]
    fn undeclared_child_is_undeclared_even_when_open() {
        let schema = r#"
open: true
document:
  sections:
    - name: 決定
      bullets:
        repeat: { min: 0 }
        children:
          fields:
            - name: superseded_by
"#;
        let doc = "## 決定\n\n- A22 判断の記録\n  - 備考: 補足\n";
        let findings = validate_src(schema, doc, true);
        assert!(
            kinds(&findings).contains(&FindingKind::UndeclaredLine),
            "宣言済みの構造の中の未宣言の子は open でも undeclared_line（REQ-schema-003）: {:?}",
            kinds(&findings)
        );
    }

    // @kotowari[REQ-schema-029, REQ-schema-031]
    #[test]
    fn declared_child_field_pattern_is_enforced() {
        let schema = r#"
document:
  sections:
    - name: 決定
      bullets:
        repeat: { min: 0 }
        children:
          fields:
            - name: superseded_by
              pattern: "^\\["
"#;
        let doc = "## 決定\n\n- A22 判断の記録\n  - superseded_by: 未指定\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            kinds(&findings).contains(&FindingKind::FieldPatternMismatch),
            "子フィールドの pattern は値に適用する（REQ-schema-029）: {:?}",
            kinds(&findings)
        );
    }

    // @kotowari[REQ-schema-004, REQ-schema-031]
    #[test]
    fn missing_required_child_field_is_found() {
        let schema = r#"
document:
  sections:
    - name: 決定
      bullets:
        repeat: { min: 0 }
        children:
          fields:
            - name: superseded_by
"#;
        let doc = "## 決定\n\n- A22 判断の記録\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            kinds(&findings).contains(&FindingKind::MissingRequiredField),
            "宣言された子フィールドが無ければ missing_required_field（REQ-schema-004、REQ-schema-031）: {:?}",
            kinds(&findings)
        );
    }

    // @kotowari[REQ-schema-001, REQ-schema-031]
    #[test]
    fn nested_list_items_are_each_reported_in_closed_world() {
        let schema = "document:\n  sections:\n    - name: 理由\n";
        let doc = "## 理由\n\n- 親\n  - 子\n";
        let findings = validate_src(schema, doc, false);
        let undeclared = findings
            .iter()
            .filter(|f| f.kind == FindingKind::UndeclaredLine)
            .count();
        assert_eq!(undeclared, 2);
    }

    // @kotowari[REQ-schema-028]
    #[test]
    fn ordered_list_is_not_a_bullet_and_is_undeclared_in_closed_world() {
        let schema =
            "document:\n  sections:\n    - name: 理由\n      bullets:\n        pattern: \"^親\"\n";
        let doc = "## 理由\n\n1. 子\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            !kinds(&findings).contains(&FindingKind::BulletPatternMismatch),
            "順序付きリストは箇条書きの対象外なので pattern を適用しない（TBL-schema-007）"
        );
        assert!(kinds(&findings).contains(&FindingKind::UndeclaredLine));
    }

    // @kotowari[REQ-schema-028]
    #[test]
    fn ordered_list_does_not_satisfy_required_bullets() {
        let schema =
            "document:\n  sections:\n    - name: 理由\n      bullets:\n        required: true\n";
        let doc = "## 理由\n\n1. 順序付き\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::MissingBullets));
    }

    // @kotowari[REQ-schema-030]
    #[test]
    fn field_continuation_is_part_of_the_field_not_undeclared_or_statement() {
        let schema = "document:\n  preamble:\n    fields:\n      - name: 状態\n    statement:\n      required: true\n";
        let doc = "# 題名\n\n- 状態: 承認済み\n\n  継続の段落\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::MissingStatement));
        assert!(
            !kinds(&findings).contains(&FindingKind::UndeclaredLine),
            "継続段落はフィールド行の一部で undeclared_line にしない（REQ-schema-030）"
        );
    }

    // @kotowari[REQ-schema-004, REQ-schema-024]
    #[test]
    fn missing_required_section_is_found() {
        let doc = "# ADR-0001: a\n\n- 状態: 承認済み\n\n## 状況\n\n背景。\n";
        let findings = validate_src(SCHEMA, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::MissingRequiredSection));
    }

    // @kotowari[REQ-schema-026]
    #[test]
    fn heading_level_mismatch_is_found() {
        let doc = "# ADR-0001: a\n\n## 状況\n\n#### 深すぎ\n\n## 決定\n\n判断。\n";
        let findings = validate_src(SCHEMA, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::HeadingLevelMismatch));
    }

    // @kotowari[REQ-schema-025, EX-schema-009]
    #[test]
    fn invalid_item_id_is_found() {
        let schema = r#"
document:
  sections:
    - name: 要求
      item:
        id: "REQ-\\d{3,}"
        statement:
          required: false
"#;
        let doc = "## 要求\n\n### XYZ-001: 名前\n\n本文。\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::InvalidId));
    }

    // @kotowari[REQ-schema-028]
    #[test]
    fn declared_field_line_passes() {
        let findings = validate_src(SCHEMA, ok_body(), false);
        assert!(!kinds(&findings).contains(&FindingKind::UndeclaredLine));
        assert!(!kinds(&findings).contains(&FindingKind::MissingRequiredField));
    }

    // @kotowari[REQ-schema-028, EX-schema-011]
    #[test]
    fn undeclared_field_name_line_counts_as_a_bullet() {
        let schema = "document:\n  sections:\n    - name: 理由\n      bullets:\n        repeat: { min: 1 }\n";
        let doc = "## 理由\n\n- 判断の記録かどうかの見分け（A134: 決定の節の見出しを1つ以上持つファイル）\n";
        let findings = validate_src(schema, doc, false);
        let ks = kinds(&findings);
        assert!(
            !ks.contains(&FindingKind::UndeclaredLine),
            "未宣言の名前の `- 名前: 値` 行は箇条書きとして扱う（TBL-schema-007）: {ks:?}"
        );
        assert!(
            !ks.contains(&FindingKind::RepeatMinNotMet),
            "箇条書きとして本数に数える（TBL-schema-007）: {ks:?}"
        );
    }

    // @kotowari[REQ-schema-028]
    #[test]
    fn undeclared_field_name_line_is_subject_to_bullet_pattern() {
        let schema = "document:\n  sections:\n    - name: 理由\n      bullets:\n        repeat: { min: 0 }\n        pattern: \"^A134\"\n";
        let doc = "## 理由\n\n- 判断の記録かどうかの見分け（A134: 決定の節の見出しを1つ以上持つファイル）\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            kinds(&findings).contains(&FindingKind::BulletPatternMismatch),
            "箇条書きとして pattern を適用する（TBL-schema-007）"
        );
    }

    // @kotowari[REQ-schema-028]
    #[test]
    fn undeclared_field_name_line_matches_bullet_pattern_on_original_text() {
        let schema = "document:\n  sections:\n    - name: 理由\n      bullets:\n        repeat: { min: 0 }\n        pattern: \"^名前:値$\"\n";
        let doc = "## 理由\n\n- 名前:値\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            !kinds(&findings).contains(&FindingKind::BulletPatternMismatch),
            "pattern は元の行に適用する: {:?}",
            kinds(&findings)
        );
    }

    // @kotowari[REQ-schema-001, REQ-schema-028]
    #[test]
    fn undeclared_field_name_line_is_undeclared_when_no_bullets_declared() {
        let schema =
            "document:\n  sections:\n    - name: 理由\n      statement:\n        required: false\n";
        let doc = "## 理由\n\n- 判断の記録かどうかの見分け（A134: 決定の節の見出しを1つ以上持つファイル）\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::UndeclaredLine));
    }

    // @kotowari[REQ-schema-002, REQ-schema-004]
    #[test]
    fn open_allows_undeclared_heading_and_line_but_not_missing_required() {
        let schema = r#"
open: true
document:
  title:
    pattern: "^ADR-\\d{4}:"
  sections:
    - name: 状況
      statement:
        required: false
    - name: 決定
      statement:
        required: false
"#;
        let doc = "# ADR-0001: a\n\n## 状況\n\n背景。\n\n## 補足\n\n- 宣言外\n";
        let findings = validate_src(schema, doc, true);
        let ks = kinds(&findings);
        assert!(!ks.contains(&FindingKind::UndeclaredHeading));
        assert!(!ks.contains(&FindingKind::UndeclaredLine));
        assert!(ks.contains(&FindingKind::MissingRequiredSection));
    }

    // @kotowari[REQ-schema-001, REQ-schema-022]
    #[test]
    fn title_without_rule_is_undeclared_heading_in_closed_world() {
        let schema =
            "document:\n  sections:\n    - name: 状況\n      statement:\n        required: false\n";
        let doc = "# 題名\n\n## 状況\n\n背景。\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::UndeclaredHeading));
    }

    // @kotowari[REQ-schema-002]
    #[test]
    fn title_without_rule_is_allowed_when_open() {
        let schema =
            "document:\n  sections:\n    - name: 状況\n      statement:\n        required: false\n";
        let doc = "# 題名\n\n## 状況\n\n背景。\n";
        let findings = validate_src(schema, doc, true);
        assert!(!kinds(&findings).contains(&FindingKind::UndeclaredHeading));
    }

    // @kotowari[REQ-schema-001]
    #[test]
    fn undeclared_section_lines_are_flagged_in_closed_world() {
        let schema =
            "document:\n  sections:\n    - name: 状況\n      statement:\n        required: false\n";
        let doc = "# 題名\n\n## 状況\n\n背景。\n\n## 補足\n\n- 余計な箇条書き\n\n### 補足の項目\n\n中身。\n";
        let findings = validate_src(schema, doc, false);
        let headings = findings
            .iter()
            .filter(|f| f.kind == FindingKind::UndeclaredHeading)
            .count();
        let lines = findings
            .iter()
            .filter(|f| f.kind == FindingKind::UndeclaredLine)
            .count();
        assert!(
            headings >= 2,
            "節と項目の見出しが undeclared_heading になる"
        );
        assert!(
            lines >= 2,
            "節の中の箇条書きと項目の中の行が undeclared_line になる"
        );
    }

    // @kotowari[REQ-schema-001, REQ-schema-023]
    #[test]
    fn undeclared_preamble_lines_are_flagged_in_closed_world() {
        let schema =
            "document:\n  sections:\n    - name: 状況\n      statement:\n        required: false\n";
        let doc = "# 題名\n\n- 状態: 承認済み\n\n## 状況\n\n背景。\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::UndeclaredLine));
    }

    // @kotowari[REQ-schema-002, REQ-schema-023]
    #[test]
    fn undeclared_preamble_lines_are_allowed_when_open() {
        let schema =
            "document:\n  sections:\n    - name: 状況\n      statement:\n        required: false\n";
        let doc = "# 題名\n\n- 状態: 承認済み\n\n## 状況\n\n背景。\n";
        let findings = validate_src(schema, doc, true);
        assert!(!kinds(&findings).contains(&FindingKind::UndeclaredLine));
    }

    // @kotowari[REQ-schema-002]
    #[test]
    fn undeclared_section_lines_are_allowed_when_open() {
        let schema =
            "document:\n  sections:\n    - name: 状況\n      statement:\n        required: false\n";
        let doc = "# 題名\n\n## 状況\n\n背景。\n\n## 補足\n\n- 余計な箇条書き\n";
        let findings = validate_src(schema, doc, true);
        assert!(!kinds(&findings).contains(&FindingKind::UndeclaredHeading));
        assert!(!kinds(&findings).contains(&FindingKind::UndeclaredLine));
    }

    // @kotowari[REQ-schema-003, REQ-schema-027]
    #[test]
    fn undeclared_item_inside_declared_section_is_flagged_even_when_open() {
        let schema = r#"
document:
  sections:
    - name: 要求
      statement:
        required: false
"#;
        let doc = "## 要求\n\n### REQ-001: 名前\n\n本文。\n";
        let findings = validate_src(schema, doc, true);
        assert!(kinds(&findings).contains(&FindingKind::UndeclaredHeading));
        assert!(kinds(&findings).contains(&FindingKind::UndeclaredLine));
    }

    // @kotowari[REQ-schema-003, REQ-schema-023]
    #[test]
    fn stray_heading_in_declared_preamble_is_undeclared_even_when_open() {
        let schema = r#"
document:
  preamble:
    statement:
      required: false
"#;
        let doc = "# 題名\n\n### 補足\n\n中身。\n";
        let findings = validate_src(schema, doc, true);
        assert!(
            kinds(&findings).contains(&FindingKind::UndeclaredHeading),
            "宣言済みの前置部の中の ### 見出しは open でも undeclared_heading（REQ-schema-003）: {:?}",
            kinds(&findings)
        );
    }

    // @kotowari[REQ-schema-003, REQ-schema-023]
    #[test]
    fn stray_heading_lines_in_declared_preamble_are_undeclared_even_when_open() {
        let schema = r#"
document:
  preamble:
    statement:
      required: false
"#;
        let doc = "# 題名\n\n### 補足\n\n中身。\n";
        let findings = validate_src(schema, doc, true);
        assert!(
            kinds(&findings).contains(&FindingKind::UndeclaredLine),
            "宣言済みの前置部の中の見出しの内側の行は open でも undeclared_line（REQ-schema-003）: {:?}",
            kinds(&findings)
        );
    }

    // @kotowari[REQ-schema-002]
    #[test]
    fn stray_heading_before_title_is_allowed_when_open() {
        let schema = r#"
document:
  preamble:
    statement:
      required: false
"#;
        let doc = "### 前置\n\n補足。\n\n# 題名\n";
        let findings = validate_src(schema, doc, true);
        assert!(
            !kinds(&findings).contains(&FindingKind::UndeclaredHeading),
            "題名より前の見出しは open では許す（REQ-schema-002）: {:?}",
            kinds(&findings)
        );
        assert!(
            !kinds(&findings).contains(&FindingKind::UndeclaredLine),
            "題名より前の見出しの内側の行も open では許す（REQ-schema-002）: {:?}",
            kinds(&findings)
        );
    }

    // @kotowari[REQ-schema-001]
    #[test]
    fn stray_heading_before_title_is_undeclared_in_closed_world() {
        let schema = r#"
document:
  preamble:
    statement:
      required: false
"#;
        let doc = "### 前置\n\n補足。\n\n# 題名\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            kinds(&findings).contains(&FindingKind::UndeclaredHeading),
            "題名より前の見出しは閉じた世界で undeclared_heading（REQ-schema-001）: {:?}",
            kinds(&findings)
        );
        assert!(
            kinds(&findings).contains(&FindingKind::UndeclaredLine),
            "題名より前の見出しの内側の行は閉じた世界で undeclared_line（REQ-schema-001）: {:?}",
            kinds(&findings)
        );
    }

    // @kotowari[REQ-schema-003, EX-schema-002]
    #[test]
    fn undeclared_line_in_declared_section_is_flagged_even_when_open() {
        let schema = r#"
document:
  sections:
    - name: 状況
      statement:
        required: false
"#;
        let doc = "## 状況\n\n本文。\n\n- 宣言外の箇条書き\n";
        let findings = validate_src(schema, doc, true);
        assert!(kinds(&findings).contains(&FindingKind::UndeclaredLine));
    }

    // ---- 行の規則（TBL-schema-007、REQ-schema-029〜REQ-schema-034） ----

    // @kotowari[REQ-schema-004]
    #[test]
    fn missing_required_field_is_found() {
        let schema = "document:\n  preamble:\n    fields:\n      - name: 状態\n";
        let doc = "# 題名\n\n## 状況\n\n本文。\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::MissingRequiredField));
    }

    // @kotowari[REQ-schema-029]
    #[test]
    fn field_pattern_mismatch_is_found() {
        let schema = r#"
document:
  preamble:
    fields:
      - name: 日付
        pattern: "^\\d{4}-\\d{2}-\\d{2}"
"#;
        let doc = "# 題名\n\n- 日付: yesterday\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::FieldPatternMismatch));
    }

    // @kotowari[REQ-schema-029]
    #[test]
    fn field_enum_invalid_is_found() {
        let schema = r#"
document:
  preamble:
    fields:
      - name: 状態
        enum: [承認済み, 却下]
"#;
        let doc = "# 題名\n\n- 状態: 保留\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::FieldEnumInvalid));
    }

    // @kotowari[REQ-schema-029]
    #[test]
    fn separator_splits_value_before_enum_check() {
        let schema = r#"
document:
  preamble:
    fields:
      - name: タグ
        separator: ","
        enum: [a, b]
"#;
        let doc = "# 題名\n\n- タグ: a,c\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::FieldEnumInvalid));
    }

    // @kotowari[REQ-schema-029]
    #[test]
    fn separator_elements_are_trimmed_before_enum_check() {
        let schema = r#"
document:
  preamble:
    fields:
      - name: タグ
        separator: ","
        enum: [a, b, c]
"#;
        let doc = "# 題名\n\n- タグ: a, c\n";
        let findings = validate_src(schema, doc, false);
        assert!(!kinds(&findings).contains(&FindingKind::FieldEnumInvalid));
    }

    // @kotowari[REQ-schema-004, REQ-schema-032]
    #[test]
    fn missing_statement_is_found() {
        let schema = "document:\n  sections:\n    - name: 状況\n      statement: {}\n";
        let doc = "## 状況\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::MissingStatement));
    }

    // @kotowari[REQ-schema-054]
    #[test]
    fn statement_pattern_mismatch_is_found() {
        let schema = r#"
document:
  sections:
    - name: 状況
      statement:
        pattern: "^状況"
"#;
        let doc = "## 状況\n\n背景。\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::StatementPatternMismatch));
    }

    // @kotowari[REQ-schema-054]
    #[test]
    fn statement_enum_invalid_is_found() {
        let schema = r#"
document:
  sections:
    - name: 状況
      statement:
        enum: [a, b]
"#;
        let doc = "## 状況\n\n本文。\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::StatementEnumInvalid));
    }

    // @kotowari[REQ-schema-004, REQ-schema-019]
    #[test]
    fn missing_bullets_is_found() {
        let schema = "document:\n  sections:\n    - name: 理由\n      bullets: {}\n";
        let doc = "## 理由\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::MissingBullets));
    }

    // @kotowari[REQ-schema-054]
    #[test]
    fn bullet_pattern_mismatch_is_found() {
        let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        pattern: "^理由"
"#;
        let doc = "## 理由\n\n- その他\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::BulletPatternMismatch));
    }

    // @kotowari[REQ-schema-033]
    #[test]
    fn missing_table_is_found() {
        let schema = r#"
document:
  sections:
    - name: 用語集
      table:
        header: [用語, 意味]
"#;
        let doc = "## 用語集\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::MissingTable));
    }

    // @kotowari[REQ-schema-033]
    #[test]
    fn table_header_mismatch_is_found() {
        let schema = r#"
document:
  sections:
    - name: 用語集
      table:
        header: [用語, 意味]
"#;
        let doc = "## 用語集\n\n| 用語 |\n|---|\n| a |\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::TableHeaderMismatch));
    }

    // @kotowari[REQ-schema-034]
    #[test]
    fn missing_codeblock_is_found() {
        let schema = r#"
document:
  sections:
    - name: 具体例
      codeblock:
        lang: gherkin
"#;
        let doc = "## 具体例\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::MissingCodeblock));
    }

    // @kotowari[REQ-schema-034]
    #[test]
    fn codeblock_lang_mismatch_is_found() {
        let schema = r#"
document:
  sections:
    - name: 具体例
      codeblock:
        lang: gherkin
"#;
        let doc = "## 具体例\n\n```python\nx = 1\n```\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::CodeblockLangMismatch));
    }

    // @kotowari[REQ-schema-034]
    #[test]
    fn codeblock_line_mismatch_is_found() {
        let schema = r#"
document:
  sections:
    - name: 具体例
      codeblock:
        lang: gherkin
        lines: ["^Scenario:", "^Given "]
"#;
        let doc = "## 具体例\n\n```gherkin\nScenario: 印を書く\nBad line\n```\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::CodeblockLineMismatch));
    }

    // @kotowari[REQ-schema-034]
    #[test]
    fn codeblock_line_matching_keeps_trailing_whitespace() {
        let schema = r#"
document:
  sections:
    - name: 具体例
      codeblock:
        lang: gherkin
        lines: ["^Scenario:$"]
"#;
        // 末尾の空白まで取り除くと "$" で閉じた正規表現が通ってしまう
        let doc = "## 具体例\n\n```gherkin\nScenario:  \n```\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::CodeblockLineMismatch));
    }

    // @kotowari[REQ-schema-034]
    #[test]
    fn codeblock_line_matching_trims_leading_whitespace_and_skips_blank_lines() {
        let schema = r#"
document:
  sections:
    - name: 具体例
      codeblock:
        lang: gherkin
        lines: ["^Scenario:", "^Given "]
"#;
        let doc = "## 具体例\n\n```gherkin\n  Scenario: 印を書く\n\n   Given 文\n```\n";
        let findings = validate_src(schema, doc, false);
        assert!(!kinds(&findings).contains(&FindingKind::CodeblockLineMismatch));
    }

    // @kotowari[REQ-schema-032]
    #[test]
    fn blockquote_and_thematic_break_lines_are_ignored_in_closed_world() {
        let schema =
            "document:\n  sections:\n    - name: 状況\n      statement:\n        required: false\n";
        let doc = "## 状況\n\n> 引用\n\n---\n\n本文。\n";
        let findings = validate_src(schema, doc, false);
        assert!(!kinds(&findings).contains(&FindingKind::UndeclaredLine));
    }

    // @kotowari[REQ-schema-032]
    #[test]
    fn image_only_line_is_not_counted_as_statement() {
        let schema =
            "document:\n  sections:\n    - name: 状況\n      statement:\n        required: true\n";
        let doc = "## 状況\n\n![alt](img.png)\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::MissingStatement));
        assert!(!kinds(&findings).contains(&FindingKind::UndeclaredLine));
    }

    // ---- 出現回数・条件付き規則・並び順（REQ-schema-019、REQ-schema-020、REQ-schema-041） ----

    // @kotowari[REQ-schema-019]
    #[test]
    fn repeat_min_not_met_for_section_is_found() {
        let schema = r#"
document:
  sections:
    - name: 理由
      repeat: { min: 2 }
"#;
        let doc = "## 理由\n\n本文。\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::RepeatMinNotMet));
    }

    // @kotowari[REQ-schema-056]
    #[test]
    fn missing_required_item_is_reported_as_repeat_min_not_met() {
        let schema = r#"
document:
  sections:
    - name: 要求
      item:
        id: "REQ-\\d{3}"
        repeat: { min: 1 }
"#;
        let doc = "## 要求\n";
        let findings = validate_src(schema, doc, false);
        let ks = kinds(&findings);
        // 項目の欠落は出現回数の下限として出る。節の欠落の種類は使わない
        assert!(ks.contains(&FindingKind::RepeatMinNotMet));
        assert!(!ks.contains(&FindingKind::MissingRequiredSection));
    }

    // @kotowari[REQ-schema-019]
    #[test]
    fn repeat_min_not_met_for_field_is_found() {
        let schema = r#"
document:
  preamble:
    fields:
      - name: タグ
        repeat: { min: 2 }
"#;
        let doc = "# 題名\n\n- タグ: a\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::RepeatMinNotMet));
    }

    // @kotowari[REQ-schema-019]
    #[test]
    fn repeat_max_exceeded_for_optional_section_is_found() {
        let schema = r#"
document:
  sections:
    - name: 補足
      required: false
      statement:
        required: false
"#;
        let doc = "## 補足\n\n1つ目。\n\n## 補足\n\n2つ目。\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::RepeatMaxExceeded));
    }

    // @kotowari[REQ-schema-019]
    #[test]
    fn repeat_max_exceeded_for_field_is_found() {
        let schema = r#"
document:
  preamble:
    fields:
      - name: タグ
        repeat: { max: 2 }
"#;
        let doc = "# 題名\n\n- タグ: a\n- タグ: b\n- タグ: c\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::RepeatMaxExceeded));
    }

    // @kotowari[REQ-schema-019]
    #[test]
    fn repeat_with_max_only_omits_min_as_zero() {
        let schema =
            "document:\n  preamble:\n    fields:\n      - name: タグ\n        repeat: { max: 2 }\n";
        let doc = "# 題名\n";
        let findings = validate_src(schema, doc, false);
        assert!(!kinds(&findings).contains(&FindingKind::RepeatMinNotMet));
    }

    // @kotowari[REQ-schema-019]
    #[test]
    fn item_with_no_repeat_requires_exactly_one() {
        let schema = r#"
document:
  sections:
    - name: 要求
      item:
        id: "REQ-\\d{3,}"
        statement:
          required: false
"#;
        let doc = "## 要求\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::RepeatMinNotMet));
    }

    // @kotowari[REQ-schema-019]
    #[test]
    fn item_repeat_max_exceeded_is_found() {
        let schema = r#"
document:
  sections:
    - name: 要求
      item:
        id: "REQ-\\d{3,}"
        repeat: { max: 1 }
        statement:
          required: false
"#;
        let doc = "## 要求\n\n### REQ-001: a\n\n本文。\n\n### REQ-002: b\n\n本文。\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::RepeatMaxExceeded));
    }

    // @kotowari[REQ-schema-019]
    #[test]
    fn item_with_required_false_is_optional() {
        let schema = r#"
document:
  sections:
    - name: 要求
      item:
        id: "REQ-\\d{3,}"
        required: false
        statement:
          required: false
"#;
        let doc = "## 要求\n";
        let findings = validate_src(schema, doc, false);
        assert!(!kinds(&findings).contains(&FindingKind::RepeatMinNotMet));
        assert!(!kinds(&findings).contains(&FindingKind::MissingRequiredSection));
    }

    // @kotowari[REQ-schema-025, EX-schema-009]
    #[test]
    fn item_heading_without_colon_is_invalid_id() {
        let schema = r#"
document:
  sections:
    - name: 要求
      item:
        statement:
          required: false
"#;
        let doc = "## 要求\n\n### REQ-001\n\n本文。\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::InvalidId));
    }

    // @kotowari[REQ-schema-041]
    #[test]
    fn field_order_mismatch_is_found() {
        let schema = r#"
document:
  sections:
    - name: 要求
      ordered: true
      fields:
        - name: 種類
        - name: 定義
"#;
        let doc = "## 要求\n\n- 定義: REQ-001\n- 種類: algorithm\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::FieldOrderMismatch));
    }

    // @kotowari[REQ-schema-041]
    #[test]
    fn a_repeated_field_in_a_row_is_still_in_declared_order() {
        let schema = r#"
document:
  sections:
    - name: 要求
      ordered: true
      fields:
        - name: 種類
          repeat: { max: 2 }
        - name: 定義
"#;
        let doc = "## 要求\n\n- 種類: algorithm\n- 種類: ubiquitous\n- 定義: REQ-001\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            !kinds(&findings).contains(&FindingKind::FieldOrderMismatch),
            "同じフィールド行が続いても書いた順に反しない: {:?}",
            kinds(&findings)
        );
    }

    // @kotowari[REQ-schema-041]
    #[test]
    fn fields_in_declared_order_pass() {
        let schema = r#"
document:
  sections:
    - name: 要求
      ordered: true
      fields:
        - name: 種類
        - name: 定義
"#;
        let doc = "## 要求\n\n- 種類: algorithm\n- 定義: REQ-001\n";
        let findings = validate_src(schema, doc, false);
        assert!(!kinds(&findings).contains(&FindingKind::FieldOrderMismatch));
    }

    // @kotowari[REQ-schema-020, EX-schema-007]
    #[test]
    fn when_eq_true_requires_the_field() {
        let schema = r#"
document:
  sections:
    - name: 要求
      fields:
        - name: 種類
        - name: 定義
          when: { field: 種類, eq: algorithm }
"#;
        let doc = "## 要求\n\n- 種類: algorithm\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::MissingRequiredField));
    }

    // @kotowari[REQ-schema-020]
    #[test]
    fn when_eq_false_skips_the_requirement() {
        let schema = r#"
document:
  sections:
    - name: 要求
      fields:
        - name: 種類
        - name: 定義
          when: { field: 種類, eq: algorithm }
"#;
        let doc = "## 要求\n\n- 種類: ubiquitous\n";
        let findings = validate_src(schema, doc, false);
        assert!(!kinds(&findings).contains(&FindingKind::MissingRequiredField));
    }

    // @kotowari[REQ-schema-021]
    #[test]
    fn when_with_missing_reference_makes_eq_false() {
        let schema = r#"
document:
  sections:
    - name: 要求
      fields:
        - name: 定義
          when: { field: 種類, eq: algorithm }
"#;
        let doc = "## 要求\n";
        let findings = validate_src(schema, doc, false);
        assert!(!kinds(&findings).contains(&FindingKind::MissingRequiredField));
    }

    // @kotowari[REQ-schema-021]
    #[test]
    fn when_with_missing_reference_makes_ne_true() {
        let schema = r#"
document:
  sections:
    - name: 要求
      fields:
        - name: 定義
          when: { field: 種類, ne: algorithm }
"#;
        let doc = "## 要求\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::MissingRequiredField));
    }

    // @kotowari[REQ-schema-021]
    #[test]
    fn when_eq_is_false_when_reference_name_is_undeclared() {
        let schema = r#"
document:
  sections:
    - name: 要求
      statement:
        required: true
        when: { field: 種類, eq: algorithm }
      bullets:
        repeat: { min: 0 }
"#;
        let doc = "## 要求\n\n- 種類: algorithm\n";
        let findings = validate_src(schema, doc, false);
        assert!(!kinds(&findings).contains(&FindingKind::MissingStatement));
    }

    // @kotowari[REQ-schema-021]
    #[test]
    fn when_ne_is_true_when_reference_name_is_undeclared() {
        let schema = r#"
document:
  sections:
    - name: 要求
      statement:
        required: true
        when: { field: 種類, ne: algorithm }
      bullets:
        repeat: { min: 0 }
"#;
        let doc = "## 要求\n\n- 種類: algorithm\n";
        let findings = validate_src(schema, doc, false);
        assert!(kinds(&findings).contains(&FindingKind::MissingStatement));
    }

    // @kotowari[REQ-schema-033]
    #[test]
    fn table_declared_on_an_item_is_not_undeclared() {
        let schema = r#"
document:
  sections:
    - name: 決定表
      item:
        id: "TBL-\\d{3,}"
        repeat: { min: 0 }
        table:
          header: [用語, 意味]
"#;
        let doc =
            "## 決定表\n\n### TBL-001: 名前\n\n| 用語 | 意味 |\n|---|---|\n| 印 | テストの印 |\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            !kinds(&findings).contains(&FindingKind::UndeclaredLine),
            "項目に宣言した表は undeclared_line にしない（TBL-schema-004）"
        );
    }

    // @kotowari[REQ-schema-033]
    #[test]
    fn table_header_mismatch_on_an_item_is_found() {
        let schema = r#"
document:
  sections:
    - name: 決定表
      item:
        id: "TBL-\\d{3,}"
        repeat: { min: 0 }
        table:
          header: [用語, 意味]
"#;
        let doc = "## 決定表\n\n### TBL-001: 名前\n\n| 語 | 訳 |\n|---|---|\n| 印 | mark |\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            kinds(&findings).contains(&FindingKind::TableHeaderMismatch),
            "項目の表のヘッダ不一致は table_header_mismatch にする（TBL-schema-004、REQ-schema-033）"
        );
    }

    // @kotowari[REQ-schema-034]
    #[test]
    fn code_block_declared_on_an_item_is_not_undeclared() {
        let schema = r#"
document:
  sections:
    - name: 具体例
      item:
        id: "EX-\\d{3,}"
        repeat: { min: 0 }
        codeblock:
          lang: gherkin
"#;
        let doc = "## 具体例\n\n### EX-001: 名前\n\n```gherkin\nScenario: 印を書く\n```\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            !kinds(&findings).contains(&FindingKind::UndeclaredLine),
            "項目に宣言したコードブロックは undeclared_line にしない（TBL-schema-004）"
        );
    }

    // @kotowari[REQ-schema-033, EX-schema-012]
    #[test]
    fn table_without_declared_header_accepts_any_header() {
        let schema = "document:\n  sections:\n    - name: 決定表\n      table: {}\n";
        let doc =
            "## 決定表\n\n| 項目 | 置く場所 | 文 |\n|---|---|---|\n| 要求 | 節の下 | 持つ |\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            !kinds(&findings).contains(&FindingKind::TableHeaderMismatch),
            "header を宣言しないときはヘッダと列数を検査しない（REQ-schema-033）"
        );
    }

    // @kotowari[REQ-schema-033]
    #[test]
    fn table_without_declared_header_is_still_required() {
        let schema = "document:\n  sections:\n    - name: 決定表\n      table: {}\n";
        let doc = "## 決定表\n\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            kinds(&findings).contains(&FindingKind::MissingTable),
            "header を宣言しなくても表の有無は検査する（REQ-schema-033）"
        );
    }

    // @kotowari[REQ-schema-023, REQ-schema-033]
    #[test]
    fn table_declared_on_the_preamble_is_not_undeclared() {
        let schema = "document:\n  preamble:\n    table:\n      header: [用語, 意味]\n";
        let doc = "# 用語集\n\n| 用語 | 意味 |\n|---|---|\n| 印 | テストの印 |\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            !kinds(&findings).contains(&FindingKind::UndeclaredLine),
            "前置部に宣言した表は undeclared_line にしない"
        );
    }

    // @kotowari[REQ-schema-023, REQ-schema-034]
    #[test]
    fn code_block_declared_on_the_preamble_is_not_undeclared() {
        let schema = "document:\n  preamble:\n    codeblock:\n      lang: gherkin\n";
        let doc = "# 具体例\n\n```gherkin\nScenario: 印を書く\n```\n";
        let findings = validate_src(schema, doc, false);
        assert!(
            !kinds(&findings).contains(&FindingKind::UndeclaredLine),
            "前置部に宣言したコードブロックは undeclared_line にしない"
        );
    }
    const ITEM_SCHEMA: &str = r#"
document:
  sections:
    - name: 要求
      item:
        id: "REQ-\\d+"
        repeat: { min: 0 }
        fields:
          - name: 種類
"#;

    fn only(findings: &[Finding], kind: FindingKind) -> &Finding {
        let found: Vec<&Finding> = findings.iter().filter(|f| f.kind == kind).collect();
        assert_eq!(found.len(), 1, "{kind:?} が1件だけ出る: {findings:?}");
        found[0]
    }

    // @kotowari[REQ-schema-008]
    #[test]
    fn missing_required_field_points_at_the_item_heading_with_the_field_name() {
        let doc = "## 要求\n\n### REQ-001: 名前\n";
        let findings = validate_src(ITEM_SCHEMA, doc, false);
        let finding = only(&findings, FindingKind::MissingRequiredField);
        assert_eq!(finding.line, Some(3), "欠落は含む項目の見出しの行を指す");
        assert_eq!(finding.node.as_deref(), Some("種類"));
        assert_eq!(finding.raw.as_deref(), Some("### REQ-001: 名前"));
    }

    // @kotowari[REQ-schema-008]
    #[test]
    fn a_field_line_declared_twice_points_at_the_second_line() {
        let schema = "document:\n  preamble:\n    fields:\n      - name: 状態\n";
        let doc = "- 状態: a\n- 状態: b\n";
        let findings = validate_src(schema, doc, true);
        let finding = only(&findings, FindingKind::RepeatMaxExceeded);
        assert_eq!(finding.line, Some(2), "上限を超えた2つ目の行を指す");
        assert_eq!(finding.node.as_deref(), Some("状態"));
        assert_eq!(finding.raw.as_deref(), Some("- 状態: b"));
    }

    // @kotowari[REQ-schema-008, REQ-schema-033]
    #[test]
    fn a_table_row_with_the_wrong_column_count_points_at_that_row() {
        let schema =
            "document:\n  sections:\n    - name: 用語集\n      table:\n        header: [a, b]\n";
        let doc = "## 用語集\n\n| a | b |\n|---|---|\n| 1 | 2 |\n| 3 |\n";
        let findings = validate_src(schema, doc, false);
        let finding = only(&findings, FindingKind::TableHeaderMismatch);
        assert_eq!(finding.line, Some(6), "ヘッダの行ではなくその行を指す");
        assert_eq!(finding.raw.as_deref(), Some("| 3 |"));
        assert_eq!(finding.node, None, "表は宣言上の名前を持たない");
    }

    // @kotowari[REQ-schema-033]
    #[test]
    fn rows_are_counted_against_the_documents_header_not_the_declared_one() {
        let schema =
            "document:\n  sections:\n    - name: 用語集\n      table:\n        header: [a, b]\n";
        let doc = "## 用語集\n\n| a | b | c |\n|---|---|---|\n| 1 | 2 | 3 |\n| 4 | 5 | 6 |\n";
        let findings = validate_src(schema, doc, false);
        let finding = only(&findings, FindingKind::TableHeaderMismatch);
        assert_eq!(
            finding.line,
            Some(3),
            "文書のヘッダ行と合うデータ行は指摘せず、ヘッダの食い違いの1件だけを出す"
        );
    }

    // @kotowari[REQ-schema-008]
    #[test]
    fn missing_title_has_neither_a_line_nor_a_node_name() {
        let doc = "## 状況\n\n背景。\n\n## 決定\n\n判断。\n";
        let findings = validate_src(SCHEMA, doc, false);
        let finding = only(&findings, FindingKind::MissingTitle);
        assert_eq!(finding.line, None, "含むノードに行が無いので省く");
        assert_eq!(finding.node, None);
        assert_eq!(finding.raw, None);
    }

    // @kotowari[REQ-schema-008]
    #[test]
    fn invalid_id_carries_the_raw_heading_line() {
        let doc = "## 要求\n\n### REQ-BAD: `名前`\n\n- 種類: x\n";
        let findings = validate_src(ITEM_SCHEMA, doc, false);
        let finding = only(&findings, FindingKind::InvalidId);
        assert_eq!(finding.line, Some(3));
        assert_eq!(
            finding.raw.as_deref(),
            Some("### REQ-BAD: `名前`"),
            "生の行は src の行と一文字も違わない"
        );
    }

    // @kotowari[REQ-schema-008]
    #[test]
    fn missing_required_section_carries_the_section_name() {
        let schema =
            "document:\n  sections:\n    - name: 状況\n      statement:\n        required: false\n";
        let doc = "";
        let findings = validate_src(schema, doc, true);
        let finding = only(&findings, FindingKind::MissingRequiredSection);
        assert_eq!(finding.node.as_deref(), Some("状況"));
        assert_eq!(finding.line, None, "文書そのものには含むノードの行が無い");
    }

    // @kotowari[REQ-schema-008]
    #[test]
    fn field_order_mismatch_carries_the_name_of_the_field_out_of_order() {
        let schema = "document:\n  sections:\n    - name: 要求\n      ordered: true\n      fields:\n        - name: 種類\n        - name: 定義\n";
        let doc = "## 要求\n\n- 定義: REQ-001\n- 種類: algorithm\n";
        let findings = validate_src(schema, doc, false);
        let finding = only(&findings, FindingKind::FieldOrderMismatch);
        assert_eq!(finding.line, Some(4), "宣言の順に反した行を指す");
        assert_eq!(finding.node.as_deref(), Some("種類"));
        assert_eq!(finding.raw.as_deref(), Some("- 種類: algorithm"));
    }

    // @kotowari[REQ-schema-055]
    #[test]
    fn undeclared_field_line_carries_the_field_rule_kind() {
        let schema = "document:\n  sections:\n    - name: 理由\n";
        let doc = "## 理由\n\n- 種類: ubiquitous\n";
        let findings = validate_src(schema, doc, false);
        let finding = only(&findings, FindingKind::UndeclaredLine);
        assert_eq!(finding.rule_kind, Some(RuleKind::Field));
    }

    // @kotowari[REQ-schema-055]
    #[test]
    fn undeclared_bullet_carries_the_bullets_rule_kind() {
        let schema = "document:\n  sections:\n    - name: 理由\n";
        let doc = "## 理由\n\n- ただの箇条書き\n";
        let findings = validate_src(schema, doc, false);
        let finding = only(&findings, FindingKind::UndeclaredLine);
        assert_eq!(finding.rule_kind, Some(RuleKind::Bullets));
    }

    // @kotowari[REQ-schema-001]
    #[test]
    fn an_empty_bullet_item_is_an_undeclared_line() {
        let schema = "document:\n  sections:\n    - name: 理由\n";
        let doc = "## 理由\n\n-\n";
        let findings = validate_src(schema, doc, false);
        let finding = only(&findings, FindingKind::UndeclaredLine);
        assert_eq!(finding.rule_kind, Some(RuleKind::Bullets));
        assert_eq!(finding.line, Some(3));
        assert_eq!(finding.raw.as_deref(), Some("-"));
    }

    // @kotowari[REQ-schema-001]
    #[test]
    fn an_empty_ordered_list_item_is_an_undeclared_line() {
        let schema = "document:\n  sections:\n    - name: 理由\n";
        let doc = "## 理由\n\n1.\n";
        let findings = validate_src(schema, doc, false);
        let finding = only(&findings, FindingKind::UndeclaredLine);
        assert_eq!(finding.rule_kind, Some(RuleKind::OrderedList));
        assert_eq!(finding.line, Some(3));
    }

    // @kotowari[REQ-schema-055]
    #[test]
    fn undeclared_table_carries_the_table_rule_kind() {
        let schema = "document:\n  sections:\n    - name: 理由\n";
        let doc = "## 理由\n\n| a | b |\n|---|---|\n| 1 | 2 |\n";
        let findings = validate_src(schema, doc, false);
        let finding = only(&findings, FindingKind::UndeclaredLine);
        assert_eq!(finding.rule_kind, Some(RuleKind::Table));
    }

    // @kotowari[REQ-schema-057]
    #[test]
    fn repeat_min_not_met_carries_the_counted_rule_kind() {
        let schema = "document:\n  sections:\n    - name: 理由\n      statement:\n        repeat: { min: 1 }\n";
        let doc = "## 理由\n";
        let findings = validate_src(schema, doc, false);
        let finding = only(&findings, FindingKind::RepeatMinNotMet);
        assert_eq!(finding.rule_kind, Some(RuleKind::Statement));
    }
}
