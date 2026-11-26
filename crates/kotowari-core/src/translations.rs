//! `対`の見分け方、`一致の記録`、`対`の`指摘`（docs/decisions/core/translation-pairs.md、
//! docs/decisions/core/translation-structure.md）
//!
//! ファイルを探して読むのは呼び出し側で、ここは名前と読んだ中身だけから決める。

use crate::{Finding, FindingKind};
use sha1::{Digest, Sha1};
use std::sync::Arc;

mod check;
mod decision_sides;
pub use decision_sides::*;
mod links;
pub(crate) use links::{Link as MarkdownLink, links as markdown_links};
mod pair;
mod paths;
mod switcher;
pub(crate) use check::*;
use links::*;
pub use pair::*;
pub use paths::*;
pub(crate) use switcher::*;
