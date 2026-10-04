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

/// HTML の節の中の "<!--" から "-->" までを、行の区切りだけを残して取り除く。行の初めから空白と
/// コメントだけが続いた後の空白も取り除き、コメントの後の文字が字下げのコードブロックにならないようにする。
/// コードブロックとコードスパンの中は HTML の節にならないので残る
fn without_comments(source: &str) -> String {
    let Ok(root) = markdown::to_mdast(source, &parse_options()) else {
        return source.to_string();
    };
    let mut ranges = Vec::new();
    collect_html(&root, &mut ranges);
    let mut out = String::with_capacity(source.len());
    let mut copied = 0;
    for (start, end) in ranges {
        let mut from = start;
        while let Some(open) = source[from..end].find("<!--") {
            let open = from + open;
            let close = source[open + 4..end]
                .find("-->")
                .map_or(end, |close| open + 4 + close + 3);
            out.push_str(&source[copied..open]);
            out.extend(
                source[open..close]
                    .chars()
                    .filter(|c| matches!(c, '\n' | '\r')),
            );
            copied = close;
            let line = out.rsplit(['\n', '\r']).next().unwrap_or("");
            if line.chars().all(|c| c == ' ' || c == '\t') {
                copied += source[close..]
                    .find(|c| c != ' ' && c != '\t')
                    .unwrap_or(source.len() - close);
            }
            from = copied.min(end).max(close);
        }
    }
    // 切る位置は ASCII の "<!--"、"-->"、空白か節の終わりなので、文字を途中で切らない
    out.push_str(&source[copied..]);
    out
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
