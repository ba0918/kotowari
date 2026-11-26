use super::list;
use kotowari::{ListItem, QueryReport};
/// 1件ごとに "kotowari list" と同じ1行目と "tests" の行を出し、
/// 続けて本文の各行を2つの半角空白で字下げし、最後に逆引きの1件ごとの行を出す
pub fn print_text(result: &QueryReport) {
    for item in result.items() {
        list::print_item_text(item.item());
        for line in item.body() {
            println!("  {line}");
        }
        for reference in item.referenced_by() {
            println!(
                "  <- {} {} {}:{}",
                reference.id(),
                reference.via(),
                reference.path(),
                reference.line()
            );
        }
        // 確認することの行は、それを持つ決定の状態と見出しを最後に出す
        if let ListItem::Confirmation(confirmation) = item.item() {
            println!(
                "  decision {} {}",
                confirmation.state().unwrap_or("-"),
                confirmation.decision()
            );
        }
    }
}
