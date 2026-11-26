//! 取得した入力の受け入れと、読み取りと検査の段取り

use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
    sync::Arc,
};

mod admission;
mod inspect;
mod read;
mod report;
mod types;
use admission::*;
pub use inspect::*;
pub use read::*;
pub use report::*;
pub use types::*;
