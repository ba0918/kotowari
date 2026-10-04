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

/// コメントを置き換える印。中身が私用領域の1文字なので、利用者の文章とは重ならない
const MARK: &str = "<!--\u{E000}-->";
/// 生の HTML を文字として出したときの印
const ESCAPED_MARK: &str = "&lt;!--\u{E000}--&gt;";

/// CommonMark と GFM の表として HTML にする。生の HTML は文字として出し、HTML のコメントは出さない
pub(crate) fn to_html(source: &str) -> String {
    let options = markdown::Options {
        parse: parse_options(),
        // 生の HTML と危険なプロトコルのリンクは、既定のまま許さない（allow_dangerous_html と
        // allow_dangerous_protocol は false）
        compile: markdown::CompileOptions::gfm(),
    };
    // GFM の読み方は MDX の構文を持たないので誤りを返さない
    markdown::to_html_with_options(&marked_comments(source), &options)
        .unwrap_or_default()
        .replace(ESCAPED_MARK, "")
}

/// HTML のコメントを、中身の無い1行のコメントの印に置き換えた文章。
/// コメントを消してから読むと、コメントが段落を区切っていた所でブロックがつながり、コメントの後の
/// 空白が字下げのコードブロックになる。印に置き換えればコメントがある場合と同じ構造に読まれ、
/// 描いた後で文字になった印だけを取り除ける。コメントが行をまたいでいても印は1行なので、文の中の
/// コメントで段落が分かれない。コードブロックとコードスパンの中は HTML の節にならないので残る
fn marked_comments(source: &str) -> String {
    let Ok(root) = markdown::to_mdast(source, &parse_options()) else {
        return source.to_string();
    };
    let mut nodes = Vec::new();
    collect_html(&root, &mut nodes);
    let mut out = String::with_capacity(source.len());
    let mut copied = 0;
    for (start, end) in nodes {
        let mut from = start.max(copied);
        // 節は重ならないので from は end を越えない
        while let Some(open) = source[from..end].find("<!--") {
            let open = from + open;
            let close = source[open + 4..end]
                .find("-->")
                .map_or(end, |close| open + 4 + close + 3);
            out.push_str(&source[copied..open]);
            out.push_str(MARK);
            copied = close;
            from = copied;
        }
    }
    // 切る位置は ASCII の "<!--"、"-->" か節の終わりなので、文字を途中で切らない
    out.push_str(&source[copied..]);
    out
}

/// HTML の節の範囲
fn collect_html(node: &Node, nodes: &mut Vec<(usize, usize)>) {
    if let Node::Html(_) = node
        && let Some(position) = node.position()
    {
        nodes.push((position.start.offset, position.end.offset));
    }
    for child in node.children().into_iter().flatten() {
        collect_html(child, nodes);
    }
}
