//! 言語ごとの`問い合わせ`（REQ-core-080〜REQ-core-083、REQ-core-180〜REQ-core-182）
//!
//! 拡張子から言語を決め（TBL-core-031）、その言語の ast-grep のルールを当てて`テスト`の節を見つける。

use ast_grep_config::{GlobalRules, RuleCollection, RuleConfig, Severity, from_yaml_string};
use ast_grep_core::tree_sitter::StrDoc;
use ast_grep_core::{Language, Node};
use ast_grep_language::SupportLang;
use kotowari_core::StopReason;
use kotowari_core::config::Config;
use std::collections::BTreeSet;
use std::path::Path;

/// 同梱の`問い合わせ`（REQ-core-182）。"language" を書いたルール
const BUNDLED_RULES: &[&str] = &[
    include_str!("../queries/rust.yml"),
    include_str!("../queries/python.yml"),
    include_str!("../queries/php.yml"),
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

/// ast-grep のルールの集まり。`問い合わせ`（"tests.rules" と同梱のルール）と`面の規則`
/// （"surface.rules"）がそれぞれ別の集まりを持ち、読み方と当て方だけを共有する（REQ-core-224）
pub struct RuleSet {
    /// ルールごとの1件だけの集まり。RuleCollection は "files" のあるルールを後ろに回すので、
    /// 並びの順（REQ-core-181 の名前の順）を保つために分けて持つ
    rules: Vec<RuleCollection<SupportLang>>,
    /// ルールが1つ以上ある言語
    languages: Vec<SupportLang>,
}

impl RuleSet {
    /// `key` は誤りの詳細に出す設定の鍵の名前
    fn build(mut rules: Vec<RuleConfig<SupportLang>>, key: &str) -> Result<Self, StopReason> {
        let mut languages = Vec::new();
        for rule in &mut rules {
            if !languages.contains(&rule.language) {
                languages.push(rule.language);
            }
            // REQ-core-188、REQ-core-224: "severity" は見ない。RuleCollection は "off" のルールを外すので付け替える
            if matches!(rule.severity, Severity::Off) {
                rule.severity = Severity::Hint;
            }
        }
        let rules = rules
            .into_iter()
            .map(|rule| RuleCollection::try_new(vec![rule]))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| config_error(format!("invalid glob in {key}: {e}")))?;
        Ok(RuleSet { rules, languages })
    }

    /// 設定の鍵 `key` に並んだルールのファイルだけから集まりを作る（REQ-core-224）
    pub fn from_texts(texts: &[kotowari_core::SourceText], key: &str) -> Result<Self, StopReason> {
        Self::build(read_rule_texts(texts, key)?, key)
    }

    /// その言語のルールが1つ以上あるか
    pub fn has_language(&self, lang: SupportLang) -> bool {
        self.languages.contains(&lang)
    }

    /// その言語のルールのうち、"files" と "ignores" がそのファイルに当てることを許すもの（REQ-core-187）
    fn rules_for(&self, lang: SupportLang, rel_path: &str) -> Vec<&RuleConfig<SupportLang>> {
        self.rules
            .iter()
            .flat_map(|rules| rules.get_rule_from_lang(Path::new(rel_path), lang))
            .collect()
    }
}

/// 同梱のルールと設定から作るルールをまとめた、言語ごとの`問い合わせ`
pub struct TestQueries {
    rules: RuleSet,
}

impl TestQueries {
    /// 同梱のルール、"tests.rust.attributes" のルール、"tests.rules" のファイルのルールを読む。
    /// ルールのファイルが読めなければ設定の誤りで停止する（REQ-core-189）
    pub fn from_texts(
        config: &Config,
        texts: &[kotowari_core::SourceText],
    ) -> Result<Self, StopReason> {
        let mut rules = bundled_rules(config)?;
        rules.extend(read_rule_texts(texts, "tests.rules")?);
        Ok(TestQueries {
            rules: RuleSet::build(rules, "tests.rules")?,
        })
    }

    /// その言語が`問い合わせのある言語`か（ルールが1つ以上ある）
    pub fn has_query(&self, lang: SupportLang) -> bool {
        self.rules.has_language(lang)
    }
}

/// 設定の鍵 `key` に並んだルールのファイルを順に読む。同じパスの2回目は設定の誤り（REQ-core-189）
fn read_rule_texts(
    texts: &[kotowari_core::SourceText],
    key: &str,
) -> Result<Vec<RuleConfig<SupportLang>>, StopReason> {
    let mut rules = Vec::new();
    let mut seen = BTreeSet::new();
    for source in texts {
        let path = source.path();
        if !seen.insert(path) {
            return Err(config_error(format!("duplicate path in {key}: {path}")));
        }
        let invalid = |error| config_error(format!("invalid rule in {key}: {path}: {error}"));
        if let Some(language) = unknown_language(source.text()) {
            return Err(invalid(format!("unknown language: {language}")));
        }
        rules.extend(parse_rules(source.text()).map_err(invalid)?);
    }
    Ok(rules)
}

fn config_error(detail: String) -> StopReason {
    StopReason::ConfigError(detail)
}

fn bundled_rules(config: &Config) -> Result<Vec<RuleConfig<SupportLang>>, StopReason> {
    let mut rules = Vec::new();
    for yaml in BUNDLED_RULES {
        rules.extend(parse_rules(yaml).map_err(config_error)?);
    }
    for (yaml, languages) in BUNDLED_SHARED_RULES {
        for language in *languages {
            let yaml = format!("language: {language}\n{yaml}");
            rules.extend(parse_rules(&yaml).map_err(config_error)?);
        }
    }
    for attribute in &config.tests.rust.attributes {
        rules.extend(parse_rules(&configured_attribute_rule(attribute)).map_err(config_error)?);
    }
    Ok(rules)
}

// ルールのファイルを1つ読む。無い、ファイルでない、読めない、UTF-8 でない、
// ルールとして読めないときは設定の誤りで、詳細は `key` とそのパス（REQ-core-189、REQ-core-225）

/// ルールの "language" の値だけを読むための形。ほかのキーは読み捨てる
#[derive(serde::Deserialize)]
struct LanguageOnly {
    language: Option<String>,
}

/// ルールのうち、"language" が知らない言語の最初のものの値（REQ-core-189）。
/// ast-grep と同じ `SupportLang` の読み方で突き合わせるので、大文字小文字と別名の受け方は変わらない。
/// "language" の値だけを読めないときは、ルールとして読むほうの誤りに任せる
fn unknown_language(yaml: &str) -> Option<String> {
    let rules: Vec<LanguageOnly> = serde_saphyr::from_multiple(yaml).ok()?;
    rules
        .into_iter()
        .filter_map(|rule| rule.language)
        .find(|language| language.parse::<SupportLang>().is_err())
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
    /// 構文の誤りが1つでもあれば None（REQ-core-083）。構文木を作れなかったときも None
    pub fn parse(content: &str, lang: SupportLang) -> Option<Self> {
        let root = ast_grep_core::AstGrep::try_new(content, lang).ok()?;
        if root.root().get_inner_node().has_error() {
            return None;
        }
        Some(ParsedFile { root })
    }

    pub fn root(&self) -> Node<'_, StrDoc<SupportLang>> {
        self.root.root()
    }

    /// その言語の`問い合わせ`をすべて当てる。同じ節に複数当たっても1つと数える（REQ-core-181）
    pub fn find_tests(
        &self,
        queries: &TestQueries,
        lang: SupportLang,
        rel_path: &str,
    ) -> Vec<TestNode<'_>> {
        let root = self.root.root();
        let mut found: Vec<TestNode<'_>> = Vec::new();
        for rule in queries.rules.rules_for(lang, rel_path) {
            for m in root.find_all(&rule.matcher) {
                let name = m
                    .get_env()
                    .get_match("NAME")
                    .map(|n| strip_quotes(&n.text()));
                // 同じ節は1つと数え、名前は "$NAME" を捕まえた最初の`問い合わせ`のものにする
                if let Some(t) = found.iter_mut().find(|t| t.node.node_id() == m.node_id()) {
                    if t.name.is_none() {
                        t.name = name;
                    }
                    continue;
                }
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

/// 1つのルールが1つの節に当たり、"$NAME" を捕まえたもの
pub struct NamedMatch {
    /// 当てたルールの "id"
    pub rule_id: String,
    /// "$NAME" の文字（引用符を1組外したもの）
    pub name: String,
    /// 当たった節の最初の行（1始まり）
    pub line: usize,
}

impl ParsedFile {
    /// その言語の規則をすべて当て、"$NAME" を捕まえた当たりをルールごとに返す（REQ-core-223）
    pub fn find_named(
        &self,
        rules: &RuleSet,
        lang: SupportLang,
        rel_path: &str,
    ) -> Vec<NamedMatch> {
        let root = self.root.root();
        let mut found = Vec::new();
        for rule in rules.rules_for(lang, rel_path) {
            for m in root.find_all(&rule.matcher) {
                let Some(name) = m
                    .get_env()
                    .get_match("NAME")
                    .map(|n| strip_quotes(&n.text()))
                else {
                    continue;
                };
                found.push(NamedMatch {
                    rule_id: rule.id.clone(),
                    name,
                    line: m.start_pos().line() + 1,
                });
            }
        }
        found
    }
}

/// 最初と最後が同じ引用符なら、その1文字ずつを外す（REQ-core-180、REQ-core-223）
fn strip_quotes(text: &str) -> String {
    for quote in ['\'', '"', '`'] {
        if text.len() >= 2 && text.starts_with(quote) && text.ends_with(quote) {
            return text[1..text.len() - 1].to_string();
        }
    }
    text.to_string()
}
