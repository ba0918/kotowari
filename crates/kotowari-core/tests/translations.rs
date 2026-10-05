//! 言語の一覧、UI の文字、対の読み方（docs/ir/core/translation-pairs.md、docs/ir/core/overview-languages.md）

use crate::config::Config;

/// TBL-core-046 の鍵と英語の文字
const ENGLISH: [(&str, &str); 17] = [
    ("language_name", "English"),
    ("index_link", "Overview"),
    ("pages", "{n} pages"),
    ("stale_sections", "{n} sections to review"),
    ("open_items", "{n} open"),
    ("planned_items", "{n} planned"),
    ("stale_mark", "Not reviewed since the IR changed"),
    ("outline_stale", "not reviewed"),
    ("superseded", "(superseded)"),
    ("deferred", "(deferred)"),
    ("compare_before", "Before"),
    ("compare_after", "After"),
    ("compare_why", "Why"),
    ("state_decided", "Decided"),
    ("state_planned", "Planned"),
    ("state_open", "Open"),
    ("state_dropped", "Dropped"),
];

/// 17の鍵すべてに、鍵の名前に prefix を付けた値（数を入れる鍵は " {n}" を足す）を持つ labels の1言語分
fn complete_labels(tag: &str, prefix: &str) -> String {
    let mut out = format!("  {tag}:\n");
    for (key, english) in ENGLISH {
        let count = if english.contains("{n}") { " {n}" } else { "" };
        out.push_str(&format!("    {key}: \"{prefix}{key}{count}\"\n"));
    }
    out
}

// @kotowari[REQ-core-351, TBL-core-046]
#[test]
fn req_core_351_english_is_the_built_in_text_with_overrides_and_others_are_the_labels() {
    let yaml = format!(
        "languages: [ja, en]\nlabels:\n  en:\n    pages: \"{{n}} docs\"\n{}",
        complete_labels("ja", "J")
    );
    let config = Config::parse(&yaml).unwrap();
    let english = config.ui_text("en");
    for (key, text) in ENGLISH {
        let expected = if key == "pages" { "{n} docs" } else { text };
        assert_eq!(english.get(key), Some(expected), "{key}");
    }
    let japanese = config.ui_text("ja");
    for (key, text) in ENGLISH {
        let count = if text.contains("{n}") { " {n}" } else { "" };
        assert_eq!(japanese.get(key), Some(format!("J{key}{count}").as_str()));
    }
    assert_eq!(japanese.get("unknown"), None);
}

// @kotowari[REQ-core-334]
#[test]
fn req_core_334_the_language_list_is_the_languages_key_and_english_when_absent_or_empty() {
    let ja = complete_labels("ja", "J");
    let config = Config::parse(&format!("languages: [ja, en]\nlabels:\n{ja}")).unwrap();
    assert_eq!(config.languages(), ["ja", "en"]);
    assert_eq!(Config::parse("ir: docs/ir\n").unwrap().languages(), ["en"]);
    assert_eq!(
        Config::parse("languages: []\n").unwrap().languages(),
        ["en"]
    );
    // validated は読んだ値をそのまま保つ
    assert_eq!(config.validated().unwrap(), config);
}

// @kotowari[REQ-core-335]
#[test]
fn req_core_335_an_empty_malformed_or_repeated_tag_stops() {
    let ja = complete_labels("ja", "J");
    for list in [
        "[ja, \"\"]",
        "[ja, en_us]",
        "[ja, \"e n\"]",
        "[ja, ja]",
        "[en, en]",
    ] {
        assert!(
            matches!(
                Config::parse(&format!("languages: {list}\nlabels:\n{ja}")),
                Err(crate::StopReason::ConfigError(_))
            ),
            "{list} should stop"
        );
    }
    assert!(
        Config::parse(&format!(
            "languages: [ja, zh-hant, en2]\nlabels:\n{ja}{}{}",
            complete_labels("zh-hant", "Z"),
            complete_labels("en2", "E")
        ))
        .is_ok()
    );
}

// @kotowari[REQ-core-352]
#[test]
fn req_core_352_unknown_keys_and_a_doubled_number_placeholder_stop() {
    let ja = complete_labels("ja", "J");
    for labels in [
        format!("labels:\n{ja}    extra: \"x\"\n"),
        "labels:\n  en:\n    unknown: \"x\"\n".to_string(),
        "labels:\n  en:\n    open_items: \"{n} of {n}\"\n".to_string(),
    ] {
        assert!(
            matches!(
                Config::parse(&format!("languages: [ja, en]\n{labels}")),
                Err(crate::StopReason::ConfigError(_))
            ),
            "{labels} should stop"
        );
    }
}
