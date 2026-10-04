//! Markdown の文章の塊を HTML にする（REQ-view-007）

use markdown::mdast::Node;

fn parse_options() -> markdown::ParseOptions {
    markdown::ParseOptions {
        constructs: markdown::Constructs {
            // 画像はページの外から読み込ませる（REQ-view-010）ので、"![...](...)" はリンクとして読む
            label_start_image: false,
            ..markdown::Constructs::gfm()
        },
        ..markdown::ParseOptions::gfm()
    }
}

/// CommonMark と GFM の表として HTML にする。生の HTML は文字として出し、HTML のコメントは出さない
pub(crate) fn to_html(source: &str) -> String {
    let options = markdown::Options {
        parse: parse_options(),
        compile: markdown::CompileOptions {
            allow_dangerous_html: false,
            allow_dangerous_protocol: false,
            ..markdown::CompileOptions::gfm()
        },
    };
    // GFM の読み方は MDX の構文を持たないので誤りを返さない
    markdown::to_html_with_options(&without_comments(source), &options).unwrap_or_default()
}

/// HTML の節の中の "<!--" から "-->" までを、行の区切りを残して空白に置き換える。
/// コードブロックとコードスパンの中は HTML の節にならないので残る
fn without_comments(source: &str) -> String {
    let Ok(root) = markdown::to_mdast(source, &parse_options()) else {
        return source.to_string();
    };
    let mut ranges = Vec::new();
    collect_html(&root, &mut ranges);
    let mut bytes = source.as_bytes().to_vec();
    for (start, end) in ranges {
        let mut from = start;
        while let Some(open) = source[from..end].find("<!--") {
            let open = from + open;
            let close = source[open + 4..end]
                .find("-->")
                .map_or(end, |close| open + 4 + close + 3);
            for byte in &mut bytes[open..close] {
                if *byte != b'\n' && *byte != b'\r' {
                    *byte = b' ';
                }
            }
            from = close;
        }
    }
    // 範囲の両端は ASCII の "<!--" と "-->" か節の終わりなので、文字を途中で切らない
    String::from_utf8(bytes).unwrap_or_else(|_| source.to_string())
}

fn collect_html(node: &Node, ranges: &mut Vec<(usize, usize)>) {
    if let Node::Html(_) = node
        && let Some(position) = node.position()
    {
        ranges.push((position.start.offset, position.end.offset));
    }
    for child in node.children().into_iter().flatten() {
        collect_html(child, ranges);
    }
}
