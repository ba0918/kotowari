//! `UI の文字`の鍵と英語の文字（TBL-core-046）と、設定からその言語の`UI の文字`を決めること（REQ-core-351）

use std::collections::BTreeMap;

/// TBL-core-046 の1行。鍵、数を入れるか、英語の文字
pub struct Key {
    pub name: &'static str,
    pub counted: bool,
    pub english: &'static str,
}

const fn key(name: &'static str, counted: bool, english: &'static str) -> Key {
    Key {
        name,
        counted,
        english,
    }
}

/// TBL-core-046 の鍵。英語の文字は kotowari だけが持つ
pub const KEYS: &[Key] = &[
    key("language_name", false, "English"),
    key("index_link", false, "Overview"),
    key("pages", true, "{n} pages"),
    key("stale_sections", true, "{n} sections to review"),
    key("open_items", true, "{n} open"),
    key("planned_items", true, "{n} planned"),
    key("stale_mark", false, "Not reviewed since the IR changed"),
    key("outline_stale", false, "not reviewed"),
    key("superseded", false, "(superseded)"),
    key("deferred", false, "(deferred)"),
    key("compare_before", false, "Before"),
    key("compare_after", false, "After"),
    key("compare_why", false, "Why"),
    key("state_decided", false, "Decided"),
    key("state_planned", false, "Planned"),
    key("state_open", false, "Open"),
    key("state_dropped", false, "Dropped"),
];

/// 英語の言語タグ。この言語だけは "labels" が鍵を欠いてもよい（TBL-core-046）
pub const ENGLISH: &str = "en";

/// 数を入れる文字の中の数の場所（REQ-core-352）
pub const NUMBER: &str = "{n}";

/// 1つの言語の`UI の文字`。鍵から文字列への対応
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct UiText(BTreeMap<String, String>);

impl UiText {
    /// 鍵の文字。TBL-core-046 に無い鍵には None
    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(String::as_str)
    }
    /// 鍵と文字の対応のすべて
    pub fn entries(&self) -> &BTreeMap<String, String> {
        &self.0
    }
    /// "en" は英語の文字に labels を上書きし、ほかの言語は labels そのもの（TBL-core-046）
    pub(crate) fn resolve(tag: &str, labels: Option<&BTreeMap<String, String>>) -> Self {
        let mut text = BTreeMap::new();
        if tag == ENGLISH {
            for key in KEYS {
                text.insert(key.name.to_string(), key.english.to_string());
            }
        }
        if let Some(labels) = labels {
            for (key, value) in labels {
                text.insert(key.clone(), value.clone());
            }
        }
        UiText(text)
    }
}

/// REQ-core-352: "labels" の誤り。言語の一覧に無い言語タグ、TBL-core-046 に無い鍵、英語でない言語の
/// 欠けた鍵、"{n}" をちょうど1つ含まない数を入れる文字
pub(crate) fn check_labels(
    languages: &[String],
    labels: &BTreeMap<String, BTreeMap<String, String>>,
) -> Result<(), String> {
    for (tag, text) in labels {
        if !languages.contains(tag) {
            return Err(format!("labels.{tag}: not in languages"));
        }
        for (name, value) in text {
            let Some(key) = KEYS.iter().find(|key| key.name == name) else {
                return Err(format!("labels.{tag}.{name}: unknown key"));
            };
            if key.counted && value.matches(NUMBER).count() != 1 {
                return Err(format!("labels.{tag}.{name}: must contain {NUMBER} once"));
            }
        }
    }
    for tag in languages.iter().filter(|tag| *tag != ENGLISH) {
        let text = labels.get(tag);
        for key in KEYS {
            if !text.is_some_and(|text| text.contains_key(key.name)) {
                return Err(format!("labels.{tag}.{}: missing", key.name));
            }
        }
    }
    Ok(())
}
