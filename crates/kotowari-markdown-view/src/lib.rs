//! 描画の入力（文書の並び、参照の表、目次）からページの並びを作る描画のエンジン。
//!
//! このクレートの仕様は `docs/ir/view/` の IR である。ファイル、ネットワーク、環境変数に触れず、
//! 入力の形も検査しない（REQ-view-003）。

mod html;
mod index;
mod parts;
mod text;

use std::collections::BTreeMap;

/// 1回の描画で受け取るものの全体（REQ-view-001）
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RenderInput {
    pub documents: Vec<Document>,
    pub references: Vec<Reference>,
    /// 一覧の見出しと、文書を並べる入れ子と順番
    pub toc: TocGroup,
}

/// 目次の群。目次そのものも1つの目次の群である（REQ-view-001）
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TocGroup {
    pub title: String,
    /// 省いてよい一行の説明
    pub note: Option<String>,
    /// 書かれた順の項目
    pub items: Vec<TocItem>,
}

/// 目次の群の項目
#[derive(Debug, Clone, PartialEq)]
pub enum TocItem {
    /// 文書の名前
    Document(String),
    Group(TocGroup),
}

/// 1つのページの元。ファイルではない（REQ-view-001）
#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    /// ページの名前 "<name>.html" と一覧の並びに使う
    pub name: String,
    pub title: String,
    /// 冒頭に置く lead の部品
    pub lead: Part,
    /// lead に続き、最初の節より前に置く部品の並び（REQ-view-006）
    pub preamble: Vec<Part>,
    pub sections: Vec<Section>,
}

/// "## " の見出し1つとその下のブロックの並び
#[derive(Debug, Clone, PartialEq)]
pub struct Section {
    pub heading: String,
    /// IR が変わった後にまだ見直していない節か（REQ-view-009）
    pub stale: bool,
    pub blocks: Vec<Block>,
}

/// 節の中身の単位
#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    /// Markdown の文章の塊
    Markdown(String),
    Part(Part),
}

/// 種類の決まった描画の単位。値は種類のスキーマに合うものとして描く
#[derive(Debug, Clone, PartialEq)]
pub struct Part {
    pub kind: String,
    pub value: serde_json::Value,
}

/// 参照の表の1件
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    /// 部品の "refs" か "ref" の欄に書かれた文字列
    pub key: String,
    pub label: String,
    pub body: String,
    pub state: ReferenceState,
}

/// 参照の状態（REQ-view-001）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceState {
    Current,
    Superseded,
    Deferred,
}

/// view が返す1つのファイルの名前と中身
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Page {
    pub name: String,
    pub content: String,
}

const STYLE: &str = include_str!("style.css");
const INDEX: &str = "index.html";
const STALE_MARK: &str = "<span class=\"stale-mark\">IR が変わった後、まだ見直していない節</span>";
/// アウトラインの項目に付ける、STALE_MARK と同じ意味の短い印（REQ-view-022）
const OUTLINE_STALE_MARK: &str = "<span class=\"outline-stale\">見直していない</span>";

/// 描画の入力からページの並びを返す。ページは名前のバイト順に並ぶ（REQ-view-002）
pub fn render(input: &RenderInput) -> Vec<Page> {
    let refs = parts::Refs::new(&input.references);
    let documents: index::Documents = input
        .documents
        .iter()
        .map(|document| (document.name.as_str(), document))
        .collect();
    let mut pages: BTreeMap<String, String> = BTreeMap::new();
    pages.insert(INDEX.into(), index::page(&input.toc, &documents));
    pages.insert("style.css".into(), STYLE.into());
    let places = index::places(&input.toc);
    for document in documents.values() {
        let place = places.get(document.name.as_str());
        pages.insert(
            page_name(document),
            document_page(document, place, &documents, &refs),
        );
    }
    pages
        .into_iter()
        .map(|(name, content)| Page { name, content })
        .collect()
}

fn page_name(document: &Document) -> String {
    format!("{}.html", document.name)
}

/// 文書のページ。目次の中の位置、題名、節のアウトライン、冒頭の lead、lead に続く冒頭の部品、節、
/// 同じ目次の群の文書の順に描く（REQ-view-006、REQ-view-019、REQ-view-020、REQ-view-022）
fn document_page(
    document: &Document,
    place: Option<&index::Place>,
    documents: &index::Documents,
    refs: &parts::Refs,
) -> String {
    let outline = outline(&document.sections);
    // アウトラインのあるページは、広い画面でアウトラインを本文の左に置く（REQ-view-023）
    let class = if outline.is_empty() {
        "page"
    } else {
        "page with-outline"
    };
    let mut body = format!(
        "{}<main class=\"{class}\">\n<h1>{}</h1>\n{outline}<div class=\"content\">\n",
        crumbs(place),
        html::escape(&document.title)
    );
    body.push_str(&parts::part(&document.lead, refs));
    for part in &document.preamble {
        body.push_str(&parts::part(part, refs));
    }
    for (index, section) in document.sections.iter().enumerate() {
        // REQ-view-009: 古い節の見出しの隣に、まだ見直していないことを示す印を描く
        let mark = if section.stale { STALE_MARK } else { "" };
        body.push_str(&format!(
            "<section class=\"section\" id=\"{}\">\n<h2>{}{mark}</h2>\n",
            section_anchor(index),
            html::escape(&section.heading)
        ));
        body.push_str(&blocks(&section.blocks, refs));
        body.push_str("</section>\n");
    }
    if let Some(place) = place {
        body.push_str(&siblings(&document.name, place.group, documents));
    }
    body.push_str("</div>\n</main>\n");
    html::shell(&document.title, &body)
}

/// 節の場所。節の並びの中の位置から作るので、同じ見出しの節が2つあっても重ならない（REQ-view-022）
fn section_anchor(index: usize) -> String {
    format!("section-{index}")
}

/// 節の見出しを並べ、それぞれをその節へのリンクにしたアウトライン。節が無ければ何も描かない
/// （REQ-view-022）
fn outline(sections: &[Section]) -> String {
    if sections.is_empty() {
        return String::new();
    }
    let items: String = sections
        .iter()
        .enumerate()
        .map(|(index, section)| {
            let mark = if section.stale {
                OUTLINE_STALE_MARK
            } else {
                ""
            };
            format!(
                "<li><a href=\"#{}\">{}</a>{mark}</li>\n",
                section_anchor(index),
                html::escape(&section.heading)
            )
        })
        .collect();
    format!("<nav class=\"outline\">\n<ul>\n{items}</ul>\n</nav>\n")
}

/// 目次の中の位置。目次に名前が無ければ一覧へのリンクだけ（REQ-view-019）
fn crumbs(place: Option<&index::Place>) -> String {
    let links = match place {
        Some(place) => place
            .chain
            .iter()
            .map(|(title, anchor)| {
                format!("<a href=\"{INDEX}#{anchor}\">{}</a>", html::escape(title))
            })
            .collect::<Vec<_>>()
            .join("<span class=\"crumb-separator\">/</span>"),
        None => format!("<a href=\"{INDEX}\">Overview</a>"),
    };
    format!("<nav class=\"crumbs\">{links}</nav>\n")
}

/// 同じ目次の群の直下の、ほかの名前の文書へのリンク（REQ-view-020）
fn siblings(name: &str, group: &TocGroup, documents: &index::Documents) -> String {
    let links: String = group
        .items
        .iter()
        .filter_map(|item| match item {
            TocItem::Document(other) if other != name => documents.get(other.as_str()),
            _ => None,
        })
        .map(|other| {
            format!(
                "<li><a href=\"{}\">{}</a></li>",
                html::href(&page_name(other)),
                html::escape(&other.title)
            )
        })
        .collect();
    if links.is_empty() {
        return links;
    }
    format!(
        "<nav class=\"siblings\">\n<p class=\"siblings-title\">{}</p>\n<ul>{links}</ul>\n</nav>\n",
        html::escape(&group.title)
    )
}

/// 節の中身を描く。続く半分の幅の部品は先頭から2つずつ組にして左右に並べ、
/// 組にならずに残った1つとほかのブロックは幅いっぱいに描く（REQ-view-015）
fn blocks(blocks: &[Block], refs: &parts::Refs) -> String {
    let mut out = String::new();
    let mut index = 0;
    while index < blocks.len() {
        if let (Some(Block::Part(left)), Some(Block::Part(right))) =
            (blocks.get(index), blocks.get(index + 1))
            && parts::is_half(left)
            && parts::is_half(right)
        {
            out.push_str("<div class=\"row\">\n");
            out.push_str(&parts::part(left, refs));
            out.push_str(&parts::part(right, refs));
            out.push_str("</div><!-- row -->\n");
            index += 2;
            continue;
        }
        match &blocks[index] {
            Block::Markdown(source) => out.push_str(&text::to_html(source)),
            Block::Part(part) => out.push_str(&parts::part(part, refs)),
        }
        index += 1;
    }
    out
}

/// 描ける部品の種類の名前（TBL-view-001）
pub const PART_KINDS: &[&str] = &[
    "lead",
    "flow",
    "steps",
    "cards",
    "status",
    "compare",
    "decisions",
    "quiz",
];

/// 種類の名前から、その種類の部品のスキーマ（JSON Schema の文字列）を返す。
/// スキーマはクレートに埋め込んであり、知らない名前には何も返さない（REQ-view-012）
pub fn part_schema(kind: &str) -> Option<&'static str> {
    Some(match kind {
        "lead" => include_str!("../schemas/lead.json"),
        "flow" => include_str!("../schemas/flow.json"),
        "steps" => include_str!("../schemas/steps.json"),
        "cards" => include_str!("../schemas/cards.json"),
        "status" => include_str!("../schemas/status.json"),
        "compare" => include_str!("../schemas/compare.json"),
        "decisions" => include_str!("../schemas/decisions.json"),
        "quiz" => include_str!("../schemas/quiz.json"),
        _ => return None,
    })
}
