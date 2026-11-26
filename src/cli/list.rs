use kotowari::{ListItem, ReadList, TestRef};
/// 1つの項目を1行で出し、その直後に "tests" の1件ごとの行を字下げして続ける。
/// 名前と検証の値はエスケープせずそのまま出す
pub fn print_text(result: &ReadList) {
    for item in result.items() {
        print_item_text(item);
    }
    // "translations" の1件を1行で、無い側は "言語タグ=-"
    for translation in result.translations().into_iter().flatten() {
        let sides: String = translation
            .sides()
            .iter()
            .map(|side| format!(" {}={}", side.language(), side.blob().unwrap_or("-")))
            .collect();
        println!("{}{sides}", translation.path());
    }
}

/// 1つの項目の1行と、その "tests" の1件ごとの行（docs/decisions/core/list.md）。
/// query の "text" もこの2種類の行から始まる
pub fn print_item_text(item: &ListItem) {
    let p = text_parts(item);
    println!(
        "{} {} {} {}:{} tests={}",
        p.id,
        p.verification,
        p.name,
        p.path,
        p.line,
        p.tests.len()
    );
    for test in p.tests {
        println!(
            "  {}:{} {}",
            test.path(),
            test.line(),
            test.name().as_deref().unwrap_or("-")
        );
    }
}

/// 1行に出す値。"verification" は「（読んで確認）」の行では "reading"、ほかは "-"
struct TextParts<'a> {
    id: &'a str,
    verification: &'a str,
    name: &'a str,
    path: &'a str,
    line: usize,
    tests: &'a [TestRef],
}

fn text_parts(item: &ListItem) -> TextParts<'_> {
    match item {
        ListItem::Confirmation(i) => TextParts {
            id: i.id(),
            verification: if i.by_reading() { "reading" } else { "-" },
            name: i.name(),
            path: i.path(),
            line: i.line(),
            tests: i.tests(),
        },
        // 決定を指すテストの印は無い
        ListItem::Decision(i) => TextParts {
            id: i.id(),
            verification: "-",
            name: i.name(),
            path: i.path(),
            line: i.line(),
            tests: &[],
        },
    }
}
