//! スキーマ言語・検証・抽出のコア。CLI のフレームワークには依存しない。
//!
//! このクレートの仕様は `docs/ir/schema/` の IR ただ1つである。コメントから
//! 仕様を参照するときは、そこの要求・決定表・具体例・性質の ID
//! （`REQ-schema-nnn`・`TBL-schema-nnn`・`EX-schema-nnn`・`PROP-schema-nnn`）を書く。
//! `docs/spec/` は IR から起こした人間向けのビューなので、その節番号（`R<n>`）は
//! 参照しない。節番号は文書を書き直すたびに指す先が変わる。

pub mod ast;
pub mod document;
pub mod extract;
pub mod finding;
pub mod frontmatter;
pub mod schema;
pub mod validate;
