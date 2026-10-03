//! 指摘のモデル（REQ-schema-008、TBL-schema-002）。

/// 指摘の種別。REQ-schema-008 の「種類」で、TBL-schema-002 の分類を割ったもの。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum FindingKind {
    MissingTitle,
    MultipleTitles,
    TitlePatternMismatch,
    UndeclaredHeading,
    UndeclaredLine,
    MissingRequiredField,
    MissingRequiredSection,
    MissingStatement,
    MissingBullets,
    MissingTable,
    MissingCodeblock,
    FieldPatternMismatch,
    FieldEnumInvalid,
    StatementPatternMismatch,
    StatementEnumInvalid,
    BulletPatternMismatch,
    HeadingLevelMismatch,
    InvalidId,
    FieldOrderMismatch,
    TableHeaderMismatch,
    CodeblockLangMismatch,
    CodeblockLineMismatch,
    RepeatMinNotMet,
    RepeatMaxExceeded,
}

impl FindingKind {
    /// 指摘の「種類」の文字列（REQ-schema-008）。snake_case。
    pub fn as_str(&self) -> &'static str {
        match self {
            FindingKind::MissingTitle => "missing_title",
            FindingKind::MultipleTitles => "multiple_titles",
            FindingKind::TitlePatternMismatch => "title_pattern_mismatch",
            FindingKind::UndeclaredHeading => "undeclared_heading",
            FindingKind::UndeclaredLine => "undeclared_line",
            FindingKind::MissingRequiredField => "missing_required_field",
            FindingKind::MissingRequiredSection => "missing_required_section",
            FindingKind::MissingStatement => "missing_statement",
            FindingKind::MissingBullets => "missing_bullets",
            FindingKind::MissingTable => "missing_table",
            FindingKind::MissingCodeblock => "missing_codeblock",
            FindingKind::FieldPatternMismatch => "field_pattern_mismatch",
            FindingKind::FieldEnumInvalid => "field_enum_invalid",
            FindingKind::StatementPatternMismatch => "statement_pattern_mismatch",
            FindingKind::StatementEnumInvalid => "statement_enum_invalid",
            FindingKind::BulletPatternMismatch => "bullet_pattern_mismatch",
            FindingKind::HeadingLevelMismatch => "heading_level_mismatch",
            FindingKind::InvalidId => "invalid_id",
            FindingKind::FieldOrderMismatch => "field_order_mismatch",
            FindingKind::TableHeaderMismatch => "table_header_mismatch",
            FindingKind::CodeblockLangMismatch => "codeblock_lang_mismatch",
            FindingKind::CodeblockLineMismatch => "codeblock_line_mismatch",
            FindingKind::RepeatMinNotMet => "repeat_min_not_met",
            FindingKind::RepeatMaxExceeded => "repeat_max_exceeded",
        }
    }
}

/// 規則種別。スキーマが文書の構造を記述するノードの種類（docs/ir/schema/CONTEXT.md）。
/// 宣言していない行の`指摘`と`出現回数`の`指摘`が、どの種別として読んだか・数えたかを持つ
/// （REQ-schema-055、REQ-schema-057）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum RuleKind {
    Section,
    Item,
    Field,
    Bullets,
    OrderedList,
    Statement,
    Table,
    CodeBlock,
}

impl RuleKind {
    /// 規則種別の文字列。スキーマ言語の宣言の名前に合わせる。
    pub fn as_str(&self) -> &'static str {
        match self {
            RuleKind::Section => "section",
            RuleKind::Item => "item",
            RuleKind::Field => "field",
            RuleKind::Bullets => "bullets",
            RuleKind::OrderedList => "ordered_list",
            RuleKind::Statement => "statement",
            RuleKind::Table => "table",
            RuleKind::CodeBlock => "codeblock",
        }
    }
}

/// 指摘1件。`path` と `severity` は CLI 側が付ける。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    pub kind: FindingKind,
    /// 1始まりの行番号。行を持たない指摘は None。
    pub line: Option<usize>,
    /// スキーマが宣言したノードの名前。宣言上の名前を持たないノードの指摘は None。
    pub node: Option<String>,
    /// 行番号が指す行の生の文字。字下げと末尾の空白を含み、組み立て直さない。
    /// 行を持たない指摘は None。
    pub raw: Option<String>,
    /// 宣言していない行をどの`規則種別`として読んだか（REQ-schema-055）、
    /// または`出現回数`の`指摘`がどの`規則種別`のノードを数えたか（REQ-schema-057）。
    /// そのどちらでもない指摘は None。
    pub rule_kind: Option<RuleKind>,
    /// 指摘の詳細（英語）
    pub detail: String,
}

impl Finding {
    /// 行を持たない指摘。
    pub fn new(kind: FindingKind, detail: String) -> Self {
        Finding::maybe_at(kind, None, detail)
    }

    /// 行を持つ指摘。
    pub fn at(kind: FindingKind, line: usize, detail: String) -> Self {
        Finding::maybe_at(kind, Some(line), detail)
    }

    /// 行を持つかどうかが呼ぶ側で決まる指摘。
    pub fn maybe_at(kind: FindingKind, line: Option<usize>, detail: String) -> Self {
        Finding {
            kind,
            line,
            node: None,
            raw: None,
            rule_kind: None,
            detail,
        }
    }

    /// 宣言上の名前を持つノードの指摘に、その名前を添える。
    pub fn of_node(mut self, node: &str) -> Self {
        self.node = Some(node.to_string());
        self
    }

    /// 規則種別を持つ指摘に、その種別を添える。
    pub fn of_rule(mut self, rule_kind: RuleKind) -> Self {
        self.rule_kind = Some(rule_kind);
        self
    }
}
