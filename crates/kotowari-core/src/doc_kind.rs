/// IR の文書の種類
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocKind {
    /// 話題ごとの文書
    Topic,
    /// 用語集（CONTEXT.md）
    Glossary,
    /// 問題の記録（FLAGS.md）
    Flags,
}

impl DocKind {
    /// 文書名から種類を決める
    pub(crate) fn of(filename: &str) -> Self {
        match filename {
            "CONTEXT.md" => DocKind::Glossary,
            "FLAGS.md" => DocKind::Flags,
            _ => DocKind::Topic,
        }
    }
}
