//! 描画の入力（文書の並びと参照の表）からページの並びを作る描画のエンジン。
//!
//! このクレートの仕様は `docs/ir/view/` の IR である。ファイル、ネットワーク、環境変数に触れず、
//! 入力の形も検査しない（REQ-view-003）。

mod html;
mod parts;
mod text;

use std::collections::BTreeMap;

/// 1回の描画で受け取るものの全体（REQ-view-001）
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RenderInput {
    pub documents: Vec<Document>,
    pub references: Vec<Reference>,
}

/// 1つのページの元。ファイルではない（REQ-view-001）
#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    /// ページの名前 "<name>.html" と一覧の並びに使う
    pub name: String,
    pub title: String,
    /// 冒頭に置く lead の部品
    pub lead: Part,
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

/// 描画の入力からページの並びを返す。ページは名前のバイト順に並ぶ（REQ-view-002）
pub fn render(input: &RenderInput) -> Vec<Page> {
    let refs = parts::Refs::new(&input.references);
    let mut documents: Vec<&Document> = input.documents.iter().collect();
    documents.sort_by(|left, right| left.name.as_bytes().cmp(right.name.as_bytes()));
    let mut pages: BTreeMap<String, String> = BTreeMap::new();
    pages.insert(INDEX.into(), index(&documents));
    pages.insert("style.css".into(), STYLE.into());
    for document in documents {
        pages.insert(page_name(document), document_page(document, &refs));
    }
    pages
        .into_iter()
        .map(|(name, content)| Page { name, content })
        .collect()
}

fn page_name(document: &Document) -> String {
    format!("{}.html", document.name)
}

/// 一覧のページ（REQ-view-005）
fn index(documents: &[&Document]) -> String {
    let mut body = String::from("<main class=\"page index\">\n<ul class=\"documents\">\n");
    for document in documents {
        body.push_str(&format!(
            "<li><a href=\"{}\"><span class=\"title\">{}</span></a><p class=\"conclusion\">{}</p></li>\n",
            html::href(&page_name(document)),
            html::escape(&document.title),
            html::escape(parts::conclusion(&document.lead)),
        ));
    }
    body.push_str("</ul>\n</main>\n");
    html::shell("Overview", &body)
}

/// 文書のページ。題名、冒頭の lead、節の順に描く（REQ-view-006）
fn document_page(document: &Document, refs: &parts::Refs) -> String {
    let mut body = format!(
        "<nav class=\"crumbs\"><a href=\"{INDEX}\">Overview</a></nav>\n<main class=\"page\">\n<h1>{}</h1>\n",
        html::escape(&document.title)
    );
    body.push_str(&parts::part(&document.lead, refs));
    for section in &document.sections {
        body.push_str(&format!(
            "<section class=\"section\">\n<h2>{}</h2>\n",
            html::escape(&section.heading)
        ));
        body.push_str(&blocks(&section.blocks, refs));
        body.push_str("</section>\n");
    }
    body.push_str("</main>\n");
    html::shell(&document.title, &body)
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
