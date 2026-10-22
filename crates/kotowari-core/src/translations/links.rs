//! `側`のリンクの行き先（REQ-core-347、TBL-core-044）

use super::*;

/// CommonMark のリンクと画像の行き先と、リンクの参照の定義の行き先
pub(super) struct Link {
    pub(super) destination: String,
    pub(super) line: usize,
}

pub(super) fn links(root: &markdown::mdast::Node) -> Vec<Link> {
    fn definitions<'a>(
        node: &'a markdown::mdast::Node,
        out: &mut std::collections::BTreeMap<&'a str, &'a String>,
    ) {
        if let markdown::mdast::Node::Definition(definition) = node {
            out.entry(&definition.identifier).or_insert(&definition.url);
        }
        for child in node.children().into_iter().flatten() {
            definitions(child, out);
        }
    }
    fn walk(
        node: &markdown::mdast::Node,
        definitions: &std::collections::BTreeMap<&str, &String>,
        out: &mut Vec<Link>,
    ) {
        use markdown::mdast::Node;
        #[expect(
            clippy::wildcard_enum_match_arm,
            reason = "markdown::mdast::Node is a foreign enum: the remaining node kinds are deliberately handled alike"
        )]
        let destination = match node {
            Node::Link(link) => Some(&link.url),
            Node::Image(image) => Some(&image.url),
            Node::Definition(definition) => Some(&definition.url),
            Node::LinkReference(link) => definitions.get(link.identifier.as_str()).copied(),
            Node::ImageReference(image) => definitions.get(image.identifier.as_str()).copied(),
            _ => None,
        };
        if let (Some(destination), Some(position)) = (destination, node.position()) {
            out.push(Link {
                destination: destination.clone(),
                line: position.start.line,
            });
        }
        for child in node.children().into_iter().flatten() {
            walk(child, definitions, out);
        }
    }
    let mut targets = std::collections::BTreeMap::new();
    definitions(root, &mut targets);
    let mut out = Vec::new();
    walk(root, &targets, &mut out);
    out
}

/// 検査するリンクの行き先を、その`側`のあるディレクトリから辿った`基準のディレクトリ`からの相対パスに
/// する。スキームか "#" で始まる行き先は検査しないので None（REQ-core-347）
pub(super) fn target(side: &str, destination: &str) -> Option<String> {
    let mut chars = destination.chars();
    let scheme = chars.next().is_some_and(|c| c.is_ascii_alphabetic())
        && chars
            .take_while(|c| *c != ':')
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '.' | '-'))
        && destination.contains(':');
    if scheme || destination.starts_with('#') {
        return None;
    }
    let path = destination.split(['#', '?']).next().unwrap_or(destination);
    let (directory, _) = split_directory(side);
    Some(crate::normalize_path(&format!("{directory}{path}")))
}

/// `骨組み`の link の部分。ほかの言語の`側`を指す行き先はその`対`の`先頭の言語`の`側`に読み替える
/// （TBL-core-044）
pub(super) fn link_part(
    path: &str,
    root: &markdown::mdast::Node,
    context: &Context,
) -> Vec<crate::skeleton::Element> {
    links(root)
        .into_iter()
        .filter_map(|link| {
            let target = target(path, &link.destination)?;
            let target = context
                .sides
                .get(&target)
                .map_or(target, |(_, first)| first.clone());
            Some(crate::skeleton::Element {
                value: target,
                line: Some(link.line),
            })
        })
        .collect()
}
