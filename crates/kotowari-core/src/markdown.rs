use markdown::{Constructs, ParseOptions, mdast::Node, to_mdast};

pub(crate) fn parse(src: &str) -> Result<Node, String> {
    let mut constructs = Constructs::gfm();
    constructs.frontmatter = true;
    let options = ParseOptions {
        constructs,
        ..ParseOptions::default()
    };
    to_mdast(src, &options).map_err(|error| format!("{error:?}"))
}
