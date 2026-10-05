/// 描いた HTML の中のリンクの、リンク先と見える文字の並び。属性の並びや文字を包む要素は問わない
pub fn links(html: &str) -> Vec<(String, String)> {
    html.split("<a ")
        .skip(1)
        .map(|link| {
            let (tag, rest) = link.split_once('>').expect("tag end");
            let href = tag
                .split_once("href=\"")
                .and_then(|(_, value)| value.split_once('"'))
                .map(|(value, _)| value.to_string())
                .unwrap_or_default();
            let inner = &rest[..rest.find("</a>").expect("a end")];
            (href, text(inner))
        })
        .collect()
}

/// ページの中の場所へのリンク（リンク先が "#" で始まるもの）の、ページの中の位置、場所の名前、見える文字。
/// 文書のページでは、アウトラインの項目がこのリンクになる（REQ-view-022）
pub fn in_page_links(html: &str) -> Vec<(usize, String, String)> {
    html.match_indices("<a ")
        .filter_map(|(at, _)| {
            let (href, text) = links(&html[at..]).into_iter().next()?;
            let id = href.strip_prefix('#')?.to_string();
            Some((at, id, text))
        })
        .collect()
}

/// ページの中の、その名前の場所の位置。場所はページの中に1つだけある
pub fn place(html: &str, id: &str) -> usize {
    let attribute = format!("id=\"{id}\"");
    assert_eq!(html.matches(&attribute).count(), 1, "place {id}");
    html.find(&attribute).expect("place")
}

/// 要素の印を除いた文字
fn text(html: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    out.trim().to_string()
}
