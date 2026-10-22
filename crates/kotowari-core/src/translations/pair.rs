//! 読んだ`対`と、その`側`の型

use super::*;

/// `対`を読んだ置き場
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Place {
    Ir,
    Guide,
    OverviewData,
    Toc,
}

/// ある`側`の読んだ中身
#[derive(Debug, Clone)]
pub struct SideText {
    pub(super) blob: String,
    pub(super) text: Arc<str>,
}

impl SideText {
    /// bytes はファイルのバイト列そのもの、text はそれを読んだ文字（BOM を除いたもの）
    pub fn new(bytes: &[u8], text: impl Into<String>) -> Self {
        Self {
            blob: blob_hash(bytes),
            text: Arc::from(text.into()),
        }
    }
    pub fn blob(&self) -> &str {
        &self.blob
    }
    pub fn text(&self) -> &str {
        &self.text
    }
}

/// `対`の1つの言語の`側`
#[derive(Debug, Clone)]
pub struct Side {
    pub(super) language: String,
    pub(super) path: String,
    pub(super) content: Option<SideText>,
}

impl Side {
    /// path は`基準のディレクトリ`からの相対パス。ファイルが無ければ content は None
    pub fn new(
        language: impl Into<String>,
        path: impl Into<String>,
        content: Option<SideText>,
    ) -> Self {
        Self {
            language: language.into(),
            path: path.into(),
            content,
        }
    }
    pub fn language(&self) -> &str {
        &self.language
    }
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn content(&self) -> Option<&SideText> {
        self.content.as_ref()
    }
}

/// 1つの`対`。sides は`言語の一覧`の順で、最初が`先頭の言語`の`側`
#[derive(Debug, Clone)]
pub struct Pair {
    pub(super) place: Place,
    pub(super) sides: Vec<Side>,
    /// `一致の記録`のバイト列。無ければ None
    pub(super) record: Option<Vec<u8>>,
}

impl Pair {
    /// first は`先頭の言語`の`側`の`基準のディレクトリ`からの相対パス。read はパスからそのファイルの
    /// 中身を返し、ファイルが無ければ None を返す
    pub fn read<E>(
        place: Place,
        first: &str,
        languages: &[String],
        mut read: impl FnMut(&str) -> Result<Option<SideText>, E>,
        record: Option<Vec<u8>>,
    ) -> Result<Self, E> {
        let sides = languages
            .iter()
            .map(|language| {
                let path = side_path(first, language, languages);
                let content = read(&path)?;
                Ok(Side::new(language.as_str(), path, content))
            })
            .collect::<Result<_, E>>()?;
        Ok(Self {
            place,
            sides,
            record,
        })
    }
    pub fn place(&self) -> Place {
        self.place
    }
    /// `先頭の言語`の`側`のパス
    pub fn path(&self) -> &str {
        &self.sides[0].path
    }
    pub fn sides(&self) -> &[Side] {
        &self.sides
    }
    /// `先頭の言語`の`側`
    pub fn first(&self) -> &Side {
        &self.sides[0]
    }
    /// `先頭の言語`でない言語の`側`
    pub fn others(&self) -> &[Side] {
        &self.sides[1..]
    }
}

/// 読んだ`対`の並び。同じ`先頭の言語`の`側`の`対`は1つにする（REQ-core-337）
#[derive(Debug, Clone, Default)]
pub struct Pairs(std::collections::BTreeMap<String, Pair>);

impl Pairs {
    /// 加える。同じパスの`対`が既にあれば加えない
    pub fn insert(&mut self, pair: Pair) {
        self.0.entry(pair.path().to_string()).or_insert(pair);
    }
    pub fn get(&self, first: &str) -> Option<&Pair> {
        self.0.get(first)
    }
    /// `先頭の言語`の`側`のパスのバイト順
    pub fn iter(&self) -> impl Iterator<Item = &Pair> {
        self.0.values()
    }
    pub fn extend(&mut self, other: Pairs) {
        for pair in other.0.into_values() {
            self.insert(pair);
        }
    }
    /// "kotowari list" の "translations"。`先頭の言語`の`側`のパスの昇順（REQ-core-155）
    pub fn listing(&self) -> Vec<Translation> {
        self.iter()
            .map(|pair| Translation {
                path: pair.path().to_string(),
                sides: pair
                    .sides
                    .iter()
                    .map(|side| TranslationSide {
                        language: side.language.clone(),
                        path: side.path.clone(),
                        blob: side
                            .content
                            .as_ref()
                            .map(|content| content.blob().to_string()),
                    })
                    .collect(),
            })
            .collect()
    }
    /// 全体像を書かずに止める`誤り`の数。`IR`、`全体像の元データ`、`目次`の`対`の translation_missing と
    /// translation_structure_mismatch（REQ-core-294）
    pub fn overview_blockers(&self, config: &crate::config::Config) -> usize {
        let places: std::collections::BTreeMap<&str, Place> = self
            .iter()
            .flat_map(|pair| {
                pair.sides
                    .iter()
                    .map(move |side| (side.path.as_str(), pair.place))
            })
            .collect();
        self.findings(config)
            .iter()
            .filter(|finding| {
                matches!(
                    finding.kind,
                    FindingKind::TranslationMissing | FindingKind::TranslationStructureMismatch
                ) && places
                    .get(finding.path.as_str())
                    .is_some_and(|place| *place != Place::Guide)
            })
            .count()
    }
    /// すべての`対`の`指摘`
    pub(crate) fn findings(&self, config: &crate::config::Config) -> Vec<Finding> {
        let context = Context::new(self, config);
        let mut findings = Vec::new();
        for pair in self.iter() {
            pair_findings(pair, &context, &mut findings);
        }
        findings
    }
}

/// "kotowari list" の "translations" の1件（REQ-core-155）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Translation {
    pub(super) path: String,
    pub(super) sides: Vec<TranslationSide>,
}

impl Translation {
    /// `先頭の言語`の`側`の`基準のディレクトリ`からの相対パス
    pub fn path(&self) -> &str {
        &self.path
    }
    /// `言語の一覧`の順の`側`
    pub fn sides(&self) -> &[TranslationSide] {
        &self.sides
    }
}

/// "translations" の1件の1つの`側`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranslationSide {
    pub(super) language: String,
    pub(super) path: String,
    pub(super) blob: Option<String>,
}

impl TranslationSide {
    pub fn language(&self) -> &str {
        &self.language
    }
    pub fn path(&self) -> &str {
        &self.path
    }
    /// その`側`の blob hash。`側`が無ければ None
    pub fn blob(&self) -> Option<&str> {
        self.blob.as_deref()
    }
}
