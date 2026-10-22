//! gherkin の塊の中身を読む

use super::*;

/// 直前の行。ステップはシナリオ外の連続した行を一つの誤りに数えるためにも使う
#[derive(Default, PartialEq, Eq)]
pub(super) enum PreviousGherkinLine {
    #[default]
    Other,
    Tag,
    Step,
}

/// gherkin の塊1つの中身からタグとシナリオを組み立てる（REQ-core-113、A1、A6）
#[derive(Default)]
pub(super) struct GherkinBlock {
    tags: Vec<(String, String)>,
    tag_line: Option<usize>,
    tag_raw: Option<String>,
    scenario_line: Option<usize>,
    scenario_text: String,
    steps: Vec<(usize, String)>,
    previous_line: PreviousGherkinLine,
}

impl GherkinBlock {
    /// 塊の開始の行（囲みの行）と中身を読む。中身の1行目は開始の行の次の行
    pub(super) fn read(
        opening: usize,
        content: &str,
        items: &mut Vec<Item>,
        findings: &mut Vec<Finding>,
    ) {
        let mut block = GherkinBlock::default();
        for (offset, line) in content.split('\n').enumerate() {
            let line = line.strip_suffix('\r').unwrap_or(line);
            block.line(opening + 1 + offset, line, items, findings);
        }
        block.finish(items, findings);
    }

    fn invalid_tag_line(&self, findings: &mut Vec<Finding>) {
        if let Some(tl) = self.tag_line {
            findings.push(Finding::new(
                FindingKind::InvalidGherkinLine,
                String::new(),
                Some(tl),
                self.tag_raw.clone().unwrap_or_default(),
            ));
        }
    }

    fn clear_tags(&mut self) {
        self.tags.clear();
        self.tag_line = None;
        self.tag_raw = None;
    }

    fn flush_scenario(&mut self, items: &mut Vec<Item>) {
        if let Some(scenario_line) = self.scenario_line.take() {
            items.push(build_scenario(
                &self.tags,
                self.tag_line,
                scenario_line,
                &self.steps,
                &self.scenario_text,
            ));
            self.steps.clear();
        }
    }

    fn line(
        &mut self,
        line_num: usize,
        line: &str,
        items: &mut Vec<Item>,
        findings: &mut Vec<Finding>,
    ) {
        let trimmed = line.trim();
        let is_scenario_line = trimmed.starts_with("Scenario:");

        // REQ-core-113/A155: タグの行の直後が Scenario: でなければ、そのタグの行自体が invalid_gherkin_line
        if self.previous_line == PreviousGherkinLine::Tag && !is_scenario_line {
            self.invalid_tag_line(findings);
            // 結び付かないタグの検査（REQ-core-052: 結び付くかを問わない）をしてから捨てる
            check_gherkin_tags_findings(&self.tags, self.tag_line, findings);
            self.clear_tags();
        }

        if trimmed.starts_with('@') {
            // タグ行: 前のシナリオがあれば追加
            self.flush_scenario(items);
            self.tags.clear();
            self.tag_line = Some(line_num);
            self.tag_raw = Some(line.to_string());
            // タグを解析（REQ-core-052: "@" で始まる語だけを name=value に分ける）
            for part in trimmed.split_whitespace() {
                if part.starts_with('@') {
                    if let Some((tag_name, tag_value)) = part.split_once('=') {
                        self.tags
                            .push((tag_name.to_string(), tag_value.to_string()));
                    } else {
                        // "=" のない裸のタグ（@wip 等）
                        self.tags.push((part.to_string(), String::new()));
                    }
                } else {
                    // REQ-core-052: "@" で始まらない語はその語自体を detail にする
                    self.tags.push((String::new(), part.to_string()));
                }
            }
            self.previous_line = PreviousGherkinLine::Tag;
        } else if is_scenario_line {
            self.flush_scenario(items);
            // REQ-core-113: このタグは直前の行にあるときだけ、いま始まるシナリオに結び付く。
            // 結び付かない（直前がタグの行でない）ときは、前のシナリオで使い終えたタグを持ち越さない。
            if self.previous_line != PreviousGherkinLine::Tag {
                self.clear_tags();
            }
            self.scenario_line = Some(line_num);
            // A150: detail の元になる Scenario: の行は生の行（字下げを含む）を持つ
            self.scenario_text = line.to_string();
            self.previous_line = PreviousGherkinLine::Other;
        } else if ["Given ", "When ", "Then ", "And ", "But "]
            .iter()
            .any(|keyword| trimmed.starts_with(keyword))
        {
            if self.scenario_line.is_some() {
                self.steps.push((line_num, line.to_string()));
            } else if self.previous_line != PreviousGherkinLine::Step {
                // REQ-core-113/A155: 直前に Scenario: もステップも無いステップの行は invalid_gherkin_line。
                // 続く2つ目以降のステップは最初のステップの誤りに含め、別の誤りにしない
                findings.push(Finding::new(
                    FindingKind::InvalidGherkinLine,
                    String::new(),
                    Some(line_num),
                    line.to_string(),
                ));
            }
            self.previous_line = PreviousGherkinLine::Step;
        } else if trimmed.starts_with('#') || trimmed.is_empty() {
            // REQ-core-113: 注釈と空行は有効
            self.previous_line = PreviousGherkinLine::Other;
        } else {
            // REQ-core-113: それ以外は invalid_gherkin_line
            // TBL-core-008: detail は行の文字そのまま（字下げと末尾の空白を含む）
            findings.push(Finding::new(
                FindingKind::InvalidGherkinLine,
                String::new(),
                Some(line_num),
                line.to_string(),
            ));
            self.previous_line = PreviousGherkinLine::Other;
        }
    }

    /// 塊の終わり
    fn finish(mut self, items: &mut Vec<Item>, findings: &mut Vec<Finding>) {
        // REQ-core-113/A155: ブロックの終わりが Scenario: でなければ、直前のタグの行は invalid_gherkin_line
        if self.previous_line == PreviousGherkinLine::Tag {
            self.invalid_tag_line(findings);
        }
        if self.scenario_line.is_some() {
            self.flush_scenario(items);
        } else {
            // シナリオに結び付かなかったタグの検査
            check_gherkin_tags_findings(&self.tags, self.tag_line, findings);
        }
    }
}

/// 結び付かなかったタグの行を検査する（REQ-core-052: 結び付くかを問わない）
pub(super) fn check_gherkin_tags_findings(
    tags: &[(String, String)],
    tag_line: Option<usize>,
    findings: &mut Vec<crate::Finding>,
) {
    for (tag_name, tag_value) in tags {
        if let Some(detail) = unknown_gherkin_tag(tag_name, tag_value) {
            findings.push(crate::Finding::new(
                crate::FindingKind::UnknownTag,
                String::new(),
                tag_line,
                detail.to_string(),
            ));
        }
    }
}

pub(super) fn unknown_gherkin_tag<'a>(name: &'a str, value: &'a str) -> Option<&'a str> {
    if name.is_empty() {
        Some(value)
    } else if !["@id", "@about", "@source"].contains(&name) {
        Some(name)
    } else {
        None
    }
}

/// gherkin のシナリオを組み立てる
pub(super) fn build_scenario(
    tags: &[(String, String)],
    tag_line: Option<usize>,
    scenario_line: usize,
    steps: &[(usize, String)],
    scenario_text: &str,
) -> Item {
    let mut id = None;
    let mut about = Vec::new();
    let mut sources = Vec::new();
    for (tag_name, tag_value) in tags {
        match tag_name.as_str() {
            "@id" => {
                // REQ-core-114: @id の値が EX の ID の形でないときは定義に数えない
                if is_item_id(tag_value, IdPrefix::Ex) {
                    id = Some(tag_value.clone());
                }
                // 形に合わない値は check_item で invalid_id として報告
            }
            "@about" => about = comma_list(tag_value),
            "@source" => sources = comma_list(tag_value),
            _ => {}
        }
    }

    Item::Scenario {
        id,
        line: scenario_line,
        tag_line,
        tags: tags.to_vec(),
        about,
        sources,
        steps: steps.to_vec(),
        scenario_text: scenario_text.to_string(),
    }
}
