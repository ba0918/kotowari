use super::list;
use kotowari::presentation::query::*;
/// REQ-core-161: 1件ごとに "kotowari list" と同じ1行目と "tests" の行を出し、
/// 続けて本文の各行を2つの半角空白で字下げし、最後に逆引きの1件ごとの行を出す
pub fn print_text(result: &QueryResult) {
    for item in &result.items {
        list::print_item_text(&item.item);
        for line in &item.body {
            println!("  {line}");
        }
        for reference in &item.referenced_by {
            println!(
                "  <- {} {} {}:{}",
                reference.id, reference.via, reference.path, reference.line
            );
        }
    }
}
