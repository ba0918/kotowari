//! 文書の構造と閉じた世界の検証。

use crate::document::{Block, Document, Heading, Item};
use crate::finding::{Finding, FindingKind, RuleKind};
use crate::schema::{
    Bullets, Children, CodeBlock, Field, Item as ItemRule, Preamble, Repeat, Schema, Section,
    Statement, Table, Title, When, is_declared_field,
};
use std::collections::HashMap;

/// スキーマと、その`読み方`で読んだ文書の木から指摘を集める。`open` は閉じた世界を緩めるか
/// （スキーマの `open` と CLI の `--open` を合わせた値）。
pub fn validate(schema: &Schema, document: &Document, open: bool) -> Vec<Finding> {
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
                        push_undeclared_item(&mut findings, item.line, &item.id, &item.blocks);
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
            push_undeclared_item(
                &mut findings,
                stray.heading.line,
                &stray.heading.text,
                &stray.blocks,
            );
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
    let Some(first) = headings.first() else {
        findings.push(Finding::new(
            FindingKind::MissingTitle,
            "the document has no level-1 heading".into(),
        ));
        return;
    };
    // 題名が2つ以上あっても、1つ目の題名は照合する（2026-09-24-review2-gaps の A1）
    if let Some(pattern) = &title.pattern
        && !pattern.is_match(&first.text)
    {
        findings.push(Finding::at(
            FindingKind::TitlePatternMismatch,
            first.line,
            format!(
                "title \"{}\" does not match pattern \"{}\"",
                first.text,
                pattern.source()
            ),
        ));
    }
    // 2つ目以降の題名ごとに1件（REQ-schema-022）
    findings.extend(headings[1..].iter().map(|heading| {
        Finding::at(
            FindingKind::MultipleTitles,
            heading.line,
            "the document has more than one level-1 heading".into(),
        )
    }));
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
            push_undeclared_item(findings, item.line, &item.id, &item.blocks);
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

/// 宣言の無い項目の見出しと、その内側の行をすべて誤りにする（REQ-schema-001、REQ-schema-003）
fn push_undeclared_item(findings: &mut Vec<Finding>, line: usize, name: &str, blocks: &[Block]) {
    findings.push(Finding::at(
        FindingKind::UndeclaredHeading,
        line,
        format!("undeclared item heading \"{name}\""),
    ));
    for block in blocks {
        push_undeclared_line(findings, block);
    }
}

fn validate_stray_heading(heading: &Heading, findings: &mut Vec<Finding>) {
    // 文書の木は深さ1〜3の見出しをそれぞれの場所に置くので、ここには深さ4以上の見出しだけが来る
    // （REQ-schema-026）。深さ4以上は常に heading_level_mismatch
    findings.push(Finding::at(
        FindingKind::HeadingLevelMismatch,
        heading.line,
        format!("heading at depth {} is not allowed", heading.depth),
    ));
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
    let Some(bullet) = BulletLine::of(block) else {
        return;
    };
    match rules.bullets {
        Some(bullets) => validate_declared_bullet(
            bullet,
            bullets,
            rules.fields,
            blocks,
            bullet_lines,
            findings,
        ),
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
            other @ (Block::OrderedList { .. }
            | Block::Statement { .. }
            | Block::Table { .. }
            | Block::Code { .. }
            | Block::Other { .. }) => {
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
    let Some(bullet) = BulletLine::of(block) else {
        return;
    };
    validate_declared_bullet(
        bullet,
        bullets,
        &children.fields,
        sibling_blocks,
        bullet_lines,
        findings,
    );
}

/// 検証する箇条書きの1本。ブロックと、そこから取り出した本文と行番号
struct BulletLine<'a> {
    block: &'a Block,
    text: &'a str,
    line: usize,
}

impl<'a> BulletLine<'a> {
    /// 箇条書きとして検証する行（箇条書きか、箇条書きとして扱うフィールド行）。ほかのブロックは None
    fn of(block: &'a Block) -> Option<Self> {
        let (text, line) = match block {
            Block::Bullet { text, line, .. } | Block::Field { text, line, .. } => (text, *line),
            Block::OrderedList { .. }
            | Block::Statement { .. }
            | Block::Table { .. }
            | Block::Code { .. }
            | Block::Other { .. } => return None,
        };
        Some(Self { block, text, line })
    }
}

fn validate_declared_bullet(
    BulletLine { block, text, line }: BulletLine<'_>,
    bullets: &Bullets,
    fields: &[Field],
    sibling_blocks: &[Block],
    bullet_lines: &mut Vec<usize>,
    findings: &mut Vec<Finding>,
) {
    bullet_lines.push(line);
    let text = marker_line_part(text);
    if when_allows(bullets.when.as_ref(), fields, sibling_blocks)
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
    // when は required / pattern / enum にだけ効く。子の照合は常に行う（REQ-schema-020）。
    validate_children(block, bullets.children.as_ref(), findings);
}

/// lead のうちマーカーの行にある部分。段落の読み方では遅延継続の行も lead 段落に
/// 入るが、箇条書きの照合はマーカーの行だけに当てる（REQ-schema-054）
fn marker_line_part(lead: &str) -> &str {
    lead.lines().next().unwrap_or_default().trim_end()
}

/// 箇条書きの子の行を undeclared_line にする。子がさらに子を持つときも再帰する。
fn push_undeclared_children(findings: &mut Vec<Finding>, block: &Block) {
    for child in block.children() {
        push_undeclared_line(findings, child);
        push_undeclared_children(findings, child);
    }
}

#[derive(Default)]
struct ContainerOccurrences<'a> {
    fields: HashMap<&'a str, Vec<usize>>,
    statements: Vec<usize>,
    bullets: Vec<usize>,
    tables: Vec<usize>,
    codeblocks: Vec<usize>,
    ordered_fields: Vec<(usize, usize)>,
}

fn validate_container(
    rules: &ContainerRules,
    blocks: &[Block],
    open: bool,
    container_line: Option<usize>,
    findings: &mut Vec<Finding>,
) {
    let mut seen = ContainerOccurrences::default();
    let table_selection = rules.table.map(|table| table.selected_line(blocks));

    for block in blocks {
        match block {
            Block::Field {
                name, value, line, ..
            } => {
                match rules.fields.iter().position(|f| f.name == *name) {
                    Some(idx) => {
                        seen.fields.entry(name.as_str()).or_default().push(*line);
                        seen.ordered_fields.push((idx, *line));
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
                        validate_bullet(rules, block, blocks, &mut seen.bullets, open, findings);
                    }
                }
            }
            Block::Bullet { .. } => {
                validate_bullet(rules, block, blocks, &mut seen.bullets, open, findings)
            }
            Block::Statement { text, line, .. } => match rules.statement {
                Some(statement) => {
                    seen.statements.push(*line);
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
                    seen.tables.push(*line);
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
                    seen.codeblocks.push(*line);
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

    check_container_occurrences(rules, blocks, &seen, container_line, findings);

    if rules.ordered {
        let mut prev: Option<usize> = None;
        for (idx, line) in &seen.ordered_fields {
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

fn check_container_occurrences(
    rules: &ContainerRules,
    blocks: &[Block],
    seen: &ContainerOccurrences<'_>,
    container_line: Option<usize>,
    findings: &mut Vec<Finding>,
) {
    for field in rules.fields {
        if when_allows(field.when.as_ref(), rules.fields, blocks) {
            let empty: Vec<usize> = Vec::new();
            let lines = seen.fields.get(field.name.as_str()).unwrap_or(&empty);
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
                lines: &seen.statements,
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
                lines: &seen.bullets,
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
                lines: &seen.tables,
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
                lines: &seen.codeblocks,
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
}

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
        Block::Field { .. }
        | Block::Bullet { .. }
        | Block::OrderedList { .. }
        | Block::Statement { .. }
        | Block::Table { .. }
        | Block::Code { .. }
        | Block::Other { .. } => None,
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
mod tests;
