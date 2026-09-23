//! 言語ごとの`問い合わせ`（REQ-core-080〜REQ-core-083、REQ-core-180〜REQ-core-182）
//!
//! 拡張子から言語を決め（TBL-core-031）、その言語の ast-grep のルールを当てて`テスト`の節を見つける。

use crate::StopReason;
use crate::config::Config;
use ast_grep_config::{GlobalRules, RuleConfig, from_yaml_string};
use ast_grep_core::tree_sitter::{LanguageExt, StrDoc};
use ast_grep_core::{Language, Node};
use ast_grep_language::SupportLang;

/// 同梱の`問い合わせ`（REQ-core-182）。"language" を書いたルール
const BUNDLED_RULES: &[&str] = &[
    include_str!("../queries/rust.yml"),
    include_str!("../queries/python.yml"),
];

/// 同梱の`問い合わせ`のうち、同じ中身を複数の言語に付けるもの（"language" の行を足して読む）
const BUNDLED_SHARED_RULES: &[(&str, &[&str])] = &[(
    include_str!("../queries/ts_js.yml"),
    &["typescript", "tsx", "javascript"],
)];

/// 拡張子から言語を決める。表に無い拡張子は言語が決まらない（TBL-core-031）
pub fn language_of(path: &str) -> Option<SupportLang> {
    SupportLang::from_path(path)
}

/// 同梱のルールと設定から作るルールをまとめた、言語ごとの`問い合わせ`
pub struct TestQueries {
    rules: Vec<RuleConfig<SupportLang>>,
}

impl TestQueries {
    /// 同梱のルールと "tests.rust.attributes" のルールを読む
    pub fn new(config: &Config) -> Result<Self, StopReason> {
        let mut rules = Vec::new();
        for yaml in BUNDLED_RULES {
            rules.extend(parse_rules(yaml).map_err(StopReason::ConfigError)?);
        }
        for (yaml, languages) in BUNDLED_SHARED_RULES {
            for language in *languages {
                let yaml = format!("language: {language}\n{yaml}");
                rules.extend(parse_rules(&yaml).map_err(StopReason::ConfigError)?);
            }
        }
        for attribute in &config.tests.rust.attributes {
            rules.extend(
                parse_rules(&configured_attribute_rule(attribute))
                    .map_err(StopReason::ConfigError)?,
            );
        }
        Ok(TestQueries { rules })
    }

    /// その言語が`問い合わせのある言語`か（ルールが1つ以上ある）
    pub fn has_query(&self, lang: SupportLang) -> bool {
        self.rules.iter().any(|rule| rule.language == lang)
    }

    fn rules_for(&self, lang: SupportLang) -> impl Iterator<Item = &RuleConfig<SupportLang>> {
        self.rules.iter().filter(move |rule| rule.language == lang)
    }
}

fn parse_rules(yaml: &str) -> Result<Vec<RuleConfig<SupportLang>>, String> {
    from_yaml_string::<SupportLang>(yaml, &GlobalRules::default()).map_err(|e| e.to_string())
}

/// "tests.rust.attributes" の1つを、その属性の付いた関数に当たるルールにする。
/// 属性から "#["、"]"、引数を除いたパスの文字が完全に一致するものだけに当てる（TBL-core-017）
fn configured_attribute_rule(path: &str) -> String {
    let pattern = format!("^{}$", regex::escape(path)).replace('\'', "''");
    format!(
        "id: kotowari-rust-configured-attribute
language: rust
rule:
  kind: function_item
  has:
    field: name
    pattern: $NAME
  follows:
    stopBy:
      not:
        any:
          - kind: attribute_item
          - kind: line_comment
          - kind: block_comment
    kind: attribute_item
    has:
      kind: attribute
      has:
        nthChild: 1
        regex: '{pattern}'
"
    )
}

/// `問い合わせ`が当たった`テスト`の節
pub struct TestNode<'r> {
    pub node: Node<'r, StrDoc<SupportLang>>,
    /// "$NAME" の文字（引用符を外したもの）。捕まえなければ None
    pub name: Option<String>,
}

/// 構文木を読んだ`テストのファイル`
pub struct ParsedFile {
    root: ast_grep_core::AstGrep<StrDoc<SupportLang>>,
}

impl ParsedFile {
    /// 構文の誤りが1つでもあれば None（REQ-core-083）
    pub fn parse(content: &str, lang: SupportLang) -> Option<Self> {
        let root = lang.ast_grep(content);
        if root.root().get_inner_node().has_error() {
            return None;
        }
        Some(ParsedFile { root })
    }

    pub fn root(&self) -> Node<'_, StrDoc<SupportLang>> {
        self.root.root()
    }

    /// その言語の`問い合わせ`をすべて当てる。同じ節に複数当たっても1つと数える（REQ-core-181）
    pub fn find_tests(&self, queries: &TestQueries, lang: SupportLang) -> Vec<TestNode<'_>> {
        let root = self.root.root();
        let mut found: Vec<TestNode<'_>> = Vec::new();
        for rule in queries.rules_for(lang) {
            for m in root.find_all(&rule.matcher) {
                if found.iter().any(|t| t.node.node_id() == m.node_id()) {
                    continue;
                }
                let name = m
                    .get_env()
                    .get_match("NAME")
                    .map(|n| strip_quotes(&n.text()));
                found.push(TestNode {
                    node: m.get_node().clone(),
                    name,
                });
            }
        }
        found.sort_by_key(|t| t.node.range().start);
        found
    }
}

/// 最初と最後が同じ引用符なら、その1文字ずつを外す（REQ-core-180）
fn strip_quotes(text: &str) -> String {
    for quote in ['\'', '"', '`'] {
        if text.len() >= 2 && text.starts_with(quote) && text.ends_with(quote) {
            return text[1..text.len() - 1].to_string();
        }
    }
    text.to_string()
}
