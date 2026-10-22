//! `対`の見分け方、`一致の記録`、`対`の`指摘`（docs/ir/core/translation-pairs.md、
//! docs/ir/core/translation-structure.md）
//!
//! ファイルを探して読むのは呼び出し側で、ここは名前と読んだ中身だけから決める。

use crate::{Finding, FindingKind};
use sha1::{Digest, Sha1};
use std::sync::Arc;

mod check;
mod ir_sides;
mod links;
mod pair;
mod paths;
mod switcher;
pub(crate) use check::*;
pub use ir_sides::*;
use links::*;
pub use pair::*;
pub use paths::*;
pub(crate) use switcher::*;
