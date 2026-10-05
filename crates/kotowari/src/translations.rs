//! `対`を読む。置き場で集めたファイルを`側`に見分け、ほかの言語の`側`と`一致の記録`を
//! `先頭の言語`の`側`と同じディレクトリで名前で探す（REQ-core-336、REQ-core-337）

use kotowari_core::StopReason;
use kotowari_core::config::Config;
use kotowari_core::translations::{Classified, Pair, Pairs, Place, SideText, classify};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// 置き場をまたいで`対`を集めるもの
pub(crate) struct Assembly {
    base: PathBuf,
    languages: Vec<String>,
    /// "decisions.records" と "decisions.adr" の置き場。そこのファイルは`対`にしない（REQ-core-336）
    records: [String; 2],
    pairs: Pairs,
}

/// 集めたファイル。`基準のディレクトリ`からの相対パスと絶対パス
pub(crate) type Files = Vec<(String, PathBuf)>;

impl Assembly {
    pub(crate) fn new(base: &Path, config: &Config) -> Self {
        Self {
            base: base.to_path_buf(),
            languages: config.languages(),
            records: [
                kotowari_core::normalize_path(&config.decisions.records),
                kotowari_core::normalize_path(&config.decisions.adr),
            ],
            pairs: Pairs::default(),
        }
    }

    /// `言語の一覧`の言語が2つ以上か（REQ-core-334）
    pub(crate) fn enabled(&self) -> bool {
        self.languages.len() > 1
    }

    /// 置き場で集めたファイルを`対`に入れ、ほかの検査が読むファイルを返す。返すのは、ある
    /// `先頭の言語`の`側`と、with_sides なら`先頭の言語`の`側`のある`対`のほかの言語の`側`である。
    /// 判断の記録の置き場のファイルは`対`にせずそのまま返す。言語が1つなら集めたファイルをそのまま返す
    pub(crate) fn place(
        &mut self,
        place: Place,
        files: Files,
        with_sides: bool,
    ) -> Result<Files, StopReason> {
        if !self.enabled() {
            return Ok(files);
        }
        let (mut kept, files): (Files, Files) = files.into_iter().partition(|(path, _)| {
            self.records
                .iter()
                .any(|place| kotowari_core::sources::is_under_place(path, place))
        });
        let mut firsts = BTreeSet::new();
        for (path, _) in &files {
            let (directory, name) = path.rsplit_once('/').unwrap_or(("", path));
            let first = match classify(name, &self.languages) {
                Classified::Record => continue,
                Classified::First => path.clone(),
                Classified::Side { first, .. } => {
                    kotowari_core::join_display_path(directory, &first)
                }
            };
            firsts.insert(first);
        }
        for first in firsts {
            if self.pairs.get(&first).is_none() {
                let pair = self.read(place, &first)?;
                self.pairs.insert(pair);
            }
            let pair = self.pairs.get(&first).expect("the pair was inserted above");
            if pair.first().content().is_none() {
                continue;
            }
            let sides = if with_sides {
                pair.sides()
            } else {
                &pair.sides()[..1]
            };
            kept.extend(
                sides
                    .iter()
                    .filter(|side| side.content().is_some())
                    .map(|side| (side.path().to_string(), self.base.join(side.path()))),
            );
        }
        kept.sort_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
        Ok(kept)
    }

    fn read(&self, place: Place, first: &str) -> Result<Pair, StopReason> {
        let record = kotowari_core::translations::record_path(first);
        let record = if is_file(&self.base.join(&record)) {
            Some(
                std::fs::read(self.base.join(&record))
                    .map_err(|error| StopReason::UnreadableFile(format!("{record}: {error}")))?,
            )
        } else {
            None
        };
        Pair::read(
            place,
            first,
            &self.languages,
            |path| read_side(&self.base, path),
            record,
        )
    }

    /// `先頭の言語`の`側`のパスの`対`
    pub(crate) fn pair(&self, first: &str) -> Option<&Pair> {
        self.pairs.get(first)
    }

    /// 集めた`対`
    pub(crate) fn pairs(&self) -> &Pairs {
        &self.pairs
    }

    /// `先頭の言語`でない言語
    pub(crate) fn others(&self) -> &[String] {
        &self.languages[1..]
    }

    pub(crate) fn into_pairs(self) -> Pairs {
        self.pairs
    }
}

fn is_file(path: &Path) -> bool {
    std::fs::metadata(path).is_ok_and(|meta| meta.is_file())
}

/// `側`のファイルを読む。無ければ None。読めないか UTF-8 でなければ`停止`する
fn read_side(base: &Path, path: &str) -> Result<Option<SideText>, StopReason> {
    let absolute = base.join(path);
    if !is_file(&absolute) {
        return Ok(None);
    }
    let bytes = std::fs::read(&absolute)
        .map_err(|error| StopReason::UnreadableFile(format!("{path}: {error}")))?;
    let text =
        std::str::from_utf8(&bytes).map_err(|_| StopReason::NonUtf8File(path.to_string()))?;
    let text = kotowari_core::strip_bom(text).to_string();
    Ok(Some(SideText::new(&bytes, text)))
}
