kotowari 0.1.0 の仕様に基づく

設定ファイルは `.kotowari/config.yaml`。基準のディレクトリの直下にある。空の設定ファイル（0バイトか注釈だけ）は既定の値で検査を行う。設定ファイルが無いときも既定の値で行う。

| キー | 値 | 既定 |
|---|---|---|
| ir | ディレクトリのパス（1つの文字列） | docs/ir |
| decisions.records | ディレクトリのパス（1つの文字列）。その下のファイルの決定を出典に指せる。判断の記録でない Markdown（形の契約、補足の文書）も置ける | docs/decision/brainstorm |
| decisions.adr | ディレクトリのパス（1つの文字列） | docs/decision/adr |
| tests.files | glob の一覧 | src/\*\*/\*.rs、tests/\*\*/\*.rs |
| tests.rust.attributes | "#[test]" に足す属性のパスの一覧 | 空の一覧 |
| tests.rust.macros | マクロの名前の一覧 | 空の一覧 |
| limits.lines | 数（負の数と0は不可） | 120 |
| limits.requirements | 数（負の数と0は不可） | 10 |
| vague_words | 語の一覧 | 「適切に」「必要に応じて」「通常は」「など」の4語 |

一覧のキーに書いた一覧は既定の一覧を置き換える。空の一覧は要素の無い一覧として扱う。入れ子のキーは YAML の入れ子の形で書く。

基準のディレクトリ: カレントディレクトリから上に向かって `.kotowari/` のディレクトリのあるディレクトリを探す。最初に見つかればそのディレクトリ、見つからなければカレントディレクトリが基準になる。`.kotowari` という名前のファイルは無視して上に進む。

setup の手順: 以下を順に行う。既にあるファイルやディレクトリは上書きせず、あることを人に言う（`AGENTS.md` への節の追加だけは、上書きでなく追記なので行う）。

1. `.kotowari/config.yaml` を既定の値で書く。値を既定から変えるのはテストの glob か置き場を変えるときだけ。

```yaml
ir: docs/ir
decisions:
  records: docs/decision/brainstorm
  adr: docs/decision/adr
tests:
  files:
    - "src/**/*.rs"
    - "tests/**/*.rs"
  rust:
    attributes: []
    macros: []
limits:
  lines: 120
  requirements: 10
vague_words:
  - "適切に"
  - "必要に応じて"
  - "通常は"
  - "など"
```

2. 置き場のディレクトリを作る: `docs/ir/`、`docs/decision/brainstorm/`、`docs/decision/adr/`

3. `docs/ir/CONTEXT.md` を4行で作る:

```
# 用語集

| 用語 | 意味 | 出典 |
|---|---|---|
```

4. `AGENTS.md` に `## kotowari` の節を足す。`AGENTS.md` が無ければ作り、あれば末尾に節を足す。`## kotowari` の見出しが既にあれば足さない。雛形:

```markdown
## kotowari

このプロジェクトの仕様は IR（`docs/ir/`）で管理する。brainstorm、plan、cycle、implement の各席では `kotowari` スキルを読み、場面に応じた reference に従う。
```
