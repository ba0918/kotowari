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

/// HTML のコメントを取り除いた文章。どう取り除くかは、読み取りがそのコメントを置いた場所で決める。
/// 文の中（段落や見出しの中）のコメントは、行の区切りも含めて丸ごと取り除くので、段落は分かれない。
/// ブロックとして置かれたコメントは、取り除いた後に続く空白も取り除くので、後の文字が字下げの
/// コードブロックにならない。コメントの後に何も無く、行の初めからコメントまでが空白だけなら、その行を
/// 行の区切りごと取り除くので、前後の段落はつながったままになる。コードブロックとコードスパンの中は
/// HTML の節にならないので残る
fn without_comments(source: &str) -> String {
    let Ok(root) = markdown::to_mdast(source, &parse_options()) else {
        return source.to_string();
    };
    let mut nodes = Vec::new();
    collect_html(&root, false, &mut nodes);
    let mut out = String::with_capacity(source.len());
    let mut copied = 0;
    for (start, end, flow) in nodes {
        let mut from = start.max(copied);
        while from < end
            && let Some(open) = source[from..end].find("<!--")
        {
            let open = from + open;
            let close = source[open + 4..end]
                .find("-->")
                .map_or(end, |close| open + 4 + close + 3);
            out.push_str(&source[copied..open]);
            copied = close;
            if flow {
                copied += source[close..]
                    .find(|c| c != ' ' && c != '\t')
                    .unwrap_or(source.len() - close);
                let line = out.rsplit(['\n', '\r']).next().unwrap_or("");
                let alone = line.chars().all(|c| c == ' ' || c == '\t');
                let rest = &source[copied..];
                let line_break = if rest.starts_with("\r\n") {
                    2
                } else {
                    usize::from(rest.starts_with(['\n', '\r']))
                };
                if alone && line_break > 0 {
                    out.truncate(out.trim_end_matches([' ', '\t']).len());
                    copied += line_break;
                }
            }
            from = copied;
        }
    }
    // 切る位置は ASCII の "<!--"、"-->"、空白、行の区切りか節の終わりなので、文字を途中で切らない
    out.push_str(&source[copied..]);
    out
}

/// HTML の節の範囲と、ブロックとして置かれたか（親が文書、引用、一覧の項目、脚注の定義か）
fn collect_html(node: &Node, flow: bool, nodes: &mut Vec<(usize, usize, bool)>) {
    if let Node::Html(_) = node
        && let Some(position) = node.position()
    {
        nodes.push((position.start.offset, position.end.offset, flow));
    }
    let children_flow = matches!(
        node,
        Node::Root(_) | Node::Blockquote(_) | Node::ListItem(_) | Node::FootnoteDefinition(_)
    );
    for child in node.children().into_iter().flatten() {
        collect_html(child, children_flow, nodes);
    }
}
