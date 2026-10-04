//! Markdown の文章の塊を HTML にする（REQ-view-007）

/// CommonMark と GFM の表として HTML にする
pub(crate) fn to_html(source: &str) -> String {
    let options = markdown::Options {
        parse: markdown::ParseOptions::gfm(),
        compile: markdown::CompileOptions {
            allow_dangerous_html: false,
            allow_dangerous_protocol: false,
            ..markdown::CompileOptions::gfm()
        },
    };
    // GFM の読み方は MDX の構文を持たないので誤りを返さない
    markdown::to_html_with_options(source, &options).unwrap_or_default()
}
