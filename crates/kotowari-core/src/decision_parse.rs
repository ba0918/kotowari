//! 決定の文書の解析の公開の入口（docs/decisions/core/library-inputs.md）。検査のための入力群
//! （テストの情報、ガイド、設定）は求めず、文書の文字だけを読む

use crate::{Finding, InputError, SourceText};

/// 1つの決定の文書を読んだ結果。形の誤りがあっても、読めた決定と確認することの行を持つ
#[derive(Debug)]
pub struct ParsedDecisionDocument {
    path: String,
    decisions: Vec<ParsedDecision>,
    findings: Vec<Finding>,
}

impl ParsedDecisionDocument {
    /// 渡した論理パス
    pub fn path(&self) -> &str {
        &self.path
    }
    /// 文書の順の決定
    pub fn decisions(&self) -> &[ParsedDecision] {
        &self.decisions
    }
    /// 形の誤りの指摘。並びは check と同じ
    pub fn findings(&self) -> &[Finding] {
        &self.findings
    }
}

/// 1つの決定
#[derive(Debug)]
pub struct ParsedDecision {
    heading: String,
    anchor: String,
    line: usize,
    end_line: usize,
    state: Option<&'static str>,
    confirmations: Vec<ParsedConfirmation>,
}

impl ParsedDecision {
    /// 見出しの文
    pub fn heading(&self) -> &str {
        &self.heading
    }
    /// 見出しへのリンクの行き先になる名前（ガイドの印が "<パス>#<名前>" で指す名前）
    pub fn anchor(&self) -> &str {
        &self.anchor
    }
    /// 見出しの、元の文書での行
    pub fn line(&self) -> usize {
        self.line
    }
    /// 決定の最後の行。次の "##" の見出しの前の行で、空行のこともある
    pub fn end_line(&self) -> usize {
        self.end_line
    }
    /// "proposed" か "implemented"。状態の行が無いか誤っていれば None
    pub fn state(&self) -> Option<&'static str> {
        self.state
    }
    /// 確認することの行。文書の順
    pub fn confirmations(&self) -> &[ParsedConfirmation] {
        &self.confirmations
    }
}

/// 確認することの1行
#[derive(Debug)]
pub struct ParsedConfirmation {
    id: String,
    line: usize,
    by_reading: bool,
    name: String,
}

impl ParsedConfirmation {
    /// 行の最初の語。形が誤っていても、書いたままを返す
    pub fn id(&self) -> &str {
        &self.id
    }
    /// 元の文書での行
    pub fn line(&self) -> usize {
        self.line
    }
    /// 頭に「（読んで確認）」か "(by reading)" と書いた行か
    pub fn by_reading(&self) -> bool {
        self.by_reading
    }
    /// ID と（読んで確認）のあとの、書式を除いた文
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// 決定の文書の文字を読み、決定と確認することの行を元の文書の行とともに返す（C-core-libinputs-14、
/// C-core-libinputs-15）。形の誤りは findings に入れ、読めたものは捨てない。曖昧語の一覧は設定が
/// 持つので、ここでは曖昧語を検査しない。埋め込んだスキーマで読めないときだけ InvalidInput を返す
pub fn parse_decision_document(source: &SourceText) -> Result<ParsedDecisionDocument, InputError> {
    let mut findings = Vec::new();
    let document = crate::decisions::parse(source.path(), source.text(), &[], &mut findings)
        .map_err(|stop| InputError::InvalidInput(stop.to_string()))?;
    crate::sort_findings(&mut findings);
    let decisions = document
        .decisions
        .into_iter()
        .map(|decision| ParsedDecision {
            heading: decision.heading,
            anchor: decision.anchor,
            line: decision.line,
            end_line: decision.end,
            state: decision.state.map(crate::decisions::State::as_str),
            confirmations: decision
                .confirmations
                .into_iter()
                .map(|row| ParsedConfirmation {
                    id: row.id,
                    line: row.line,
                    by_reading: row.by_reading,
                    name: row.name,
                })
                .collect(),
        })
        .collect();
    Ok(ParsedDecisionDocument {
        path: source.path().to_owned(),
        decisions,
        findings,
    })
}
