//! 描画の入力のうち、言語タグと UI の文字（REQ-view-001、TBL-view-002）

use kotowari_markdown_view::RenderInput;

/// 言語タグ "ja" と、TBL-view-002 のすべての鍵に日本語の文字を持つ入力。ほかは空
pub fn japanese() -> RenderInput {
    let ui = [
        ("language_name", "日本語"),
        ("index_link", "Overview"),
        ("pages", "{n} ページ"),
        ("stale_sections", "見直していない節 {n}"),
        ("open_items", "未決 {n}"),
        ("planned_items", "予定 {n}"),
        ("stale_mark", "IR が変わった後、まだ見直していない節"),
        ("outline_stale", "見直していない"),
        ("superseded", "（置き換え済み）"),
        ("deferred", "（後回し）"),
        ("compare_before", "前"),
        ("compare_after", "後"),
        ("compare_why", "理由"),
        ("state_decided", "決定"),
        ("state_planned", "予定"),
        ("state_open", "未決"),
        ("state_dropped", "取り下げ"),
    ];
    RenderInput {
        language: "ja".into(),
        ui: ui
            .into_iter()
            .map(|(key, text)| (key.to_string(), text.to_string()))
            .collect(),
        ..RenderInput::default()
    }
}
