use kotowari::presentation::list::*;
/// REQ-core-155: 1つの項目を1行で出し、その直後に "tests" の1件ごとの行を字下げして続ける。
/// 名前と検証の値はエスケープせずそのまま出す
pub fn print_text(result: &ListResult) {
    for item in &result.items {
        print_item_text(item);
    }
}

/// REQ-core-155: 1つの項目の1行と、その "tests" の1件ごとの行。
/// query の "text" もこの2種類の行から始まる（REQ-core-161）
pub fn print_item_text(item: &ListItem) {
    let p = text_parts(item);
    // REQ-core-155: "deferred" が true の1件は行の末尾に " deferred" を付ける
    let deferred = if p.deferred { " deferred" } else { "" };
    println!(
        "{} {} {} {}:{} tests={}{deferred}",
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
            test.path,
            test.line,
            test.name.as_deref().unwrap_or("-")
        );
    }
}

/// REQ-core-155: 1行に出す値。"verification" は要求以外では "-"
struct TextParts<'a> {
    id: &'a str,
    verification: &'a str,
    name: &'a str,
    path: &'a str,
    line: usize,
    tests: &'a [TestRef],
    deferred: bool,
}

fn text_parts(item: &ListItem) -> TextParts<'_> {
    match item {
        ListItem::Requirement(i) => TextParts {
            id: &i.id,
            // REQ-core-155: "- verification:" の行の無い要求も "-"
            verification: i.verification.as_deref().unwrap_or("-"),
            name: &i.name,
            path: &i.path,
            line: i.line,
            tests: &i.tests,
            deferred: i.deferred,
        },
        ListItem::WithExamples(i) => TextParts {
            id: &i.id,
            verification: "-",
            name: &i.name,
            path: &i.path,
            line: i.line,
            tests: &i.tests,
            deferred: i.deferred,
        },
        ListItem::Scenario(i) => TextParts {
            id: &i.id,
            verification: "-",
            name: &i.name,
            path: &i.path,
            line: i.line,
            tests: &i.tests,
            deferred: i.deferred,
        },
        ListItem::Flag(i) => TextParts {
            id: &i.id,
            verification: "-",
            name: &i.name,
            path: &i.path,
            line: i.line,
            tests: &i.tests,
            deferred: i.deferred,
        },
    }
}
