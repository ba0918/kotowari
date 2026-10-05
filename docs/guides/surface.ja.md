# 面の検査

[English](surface.md) | 日本語

<!-- @kotowari[REQ-core-223:418e5055, REQ-core-227:9203538c] -->

IR に書かないまま作った機能を見つけるための検査です。
利用者から見える振る舞い（CLI のサブコマンドやフラグ、設定の鍵、HTTP のルートなど）を、設定に書いた ast-grep の規則でコードから取り出します。
取り出した1つを「面」と呼びます。
面の名前が IR に引用されて出てこなければ、`kotowari check` が `surface_without_spec` の誤りにします。

面の規則を書かないプロジェクトでは、この検査は何もしません。

## 設定

<!-- @kotowari[REQ-core-224:f13b971e, REQ-core-225:be4cdd0b] -->

```yaml
surface:
  files:
    - "src/**/*.rs"
  rules:
    - "rules/surface.yml"
  unspecified: "docs/surface-unspecified.yaml"   # 書かなくてもよい
```

| キー | 中身 |
|---|---|
| `surface.files` | 面を探すファイル（面のファイル）の glob。読み方と走査は `tests.files` と同じ。読むのは面の規則の言語のファイルだけで、それが読めないか UTF-8 でないときはテストのファイルと同じく止まる |
| `surface.rules` | 面の規則（ast-grep のルールの YAML ファイル）のパス。空の一覧でなければ検査する |
| `surface.unspecified` | 未記載の面の一覧のファイル（[下の節](#未記載の面の一覧)） |

`surface.files` と `surface.rules` は組で書きます。
片方だけのとき、`surface.rules` が空なのに `surface.unspecified` を書いたときは、設定を読むどのコマンドでも設定の誤りで止まります。
検査しているつもりで何もしていない状態を作らないためです。
キーの全部は [設定ファイル](config.ja.md) にあります。

## 面の規則を書く

<!-- @kotowari[REQ-core-223:418e5055, REQ-core-224:f13b971e, REQ-core-236:45fa5c5f] -->

面の規則は、テストを見つける `tests.rules` と同じ ast-grep のルールです（[テストに印を付ける](marks.ja.md)）。
ただしテストの問い合わせには加わらず、別に当てます。

- 規則が当たった節1つが面1つです。面の種類は規則の `id`、名前は `$NAME` で捕まえた文字です。`$NAME` を捕まえない当たりは面になりません。
- 名前の前後が同じ引用符（`'`、`"`、バッククォート）なら、1組だけ外します。それ以外は変えません。`--` を足したり、大文字小文字を変えたりはしません。
- `language` は大文字小文字を区別せず、ast-grep の別名（`ts`、`py`）も受けます。規則の言語の拡張子のファイルにだけ当てます。
- `files` と `ignores` は当てるファイルを絞ります。`severity` は見ないので、`severity: off` の規則も当てます。
- 規則の言語のファイルだけを読んで構文木にします。構文の誤りがあれば `unparsable_file` になり、そのファイルは飛ばします。ほかのファイルは読まないので、読めなくても UTF-8 でなくても止まりません。`src/**` のように広く書いて画像などに当たってもかまいません。
- 規則のファイルが無い、読めない、壊れている、同じパスが2回あるときは、`check` と `status` が設定の誤りで止まります。
- kotowari は面の規則を同梱しません。

clap の builder API で書いた CLI のフラグとサブコマンドを取り出す例です。
コードは `src/main.rs`、規則は `rules/surface.yml` に置きます。

```rust
use clap::{Arg, ArgAction, Command};

fn cli() -> Command {
    Command::new("tool")
        .arg(Arg::new("verbose").long("verbose").action(ArgAction::SetTrue))
        .subcommand(Command::new("check").arg(Arg::new("format").long("format")))
        .subcommand(Command::new("list"))
}
```

```yaml
# rules/surface.yml
id: flag
language: rust
rule:
  pattern: $ARG.long($NAME)
---
id: subcommand
language: rust
rule:
  pattern: Command::new($NAME)
  inside:
    kind: arguments
    stopBy: end
    inside:
      pattern: $PARENT.subcommand($$$)
```

`flag verbose`、`flag format`、`subcommand check`、`subcommand list` の4つが面になります。
根の `Command::new("tool")` は `subcommand` の呼び出しの中に無いので取り出しません。
フラグの名前は `format` で、`--format` ではありません。
IR には取り出した名前のとおりに `"format"` と書くか、`--format` の形で捕まえる規則を書きます。

規則は `ast-grep scan -r rules/surface.yml src/main.rs` で当たりを確かめてから設定に足すと早く済みます。

## IR にあるとする場所

<!-- @kotowari[REQ-core-226:86d88ed4, EX-core-410:d4987877, EX-core-411:0e1012fa, EX-core-429:1a617a65] -->

面の名前が、話題ごとの文書の次のどこかに、二重引用符の対かバッククォートの対で囲んだ中身として出てくれば、その面は IR にあります。

- 要求の文（後回しの要求の文も数える）
- 決定表の表のセル（見出しの行のセルも数える）
- シナリオのステップの行

囲んだ中身が名前と前後の空白まで同じときだけ数えます。
`"--verbose true"` や `" --verbose"` は、面 `--verbose` を含みません。
二重引用符は行の左から順に対にし、バッククォートは二重引用符の外のものだけを対にします。
バッククォートで囲むと用語の検査も受けるので、用語集に無い名前は二重引用符で囲みます。

性質の文、文書が扱う範囲の行、項目の見出しの下の `- ` で始まる行（`- how_to_verify:` を含む）、問題の記録（`FLAGS.md`）、用語集には、名前があっても数えません。

## 指摘

<!-- @kotowari[REQ-core-227:9203538c, EX-core-408:eaffb4f4, EX-core-409:4eb051b9] -->

上の例の CLI に、`"check"` と `"--format"` を要求の文に書いた IR を合わせた結果です。

```console
$ kotowari check --format text
src/main.rs:5 [error] surface_without_spec flag verbose
src/main.rs:6 [error] surface_without_spec flag format
src/main.rs:7 [error] surface_without_spec subcommand list
surface: unspecified=0
$ echo $?
1
```

`surface_without_spec` の detail は `種類 名前` です。
同じ種類と名前の面が何か所にあっても、パスのバイト順、行の順で最初の1か所に1件だけ出ます。
`flag format` は、IR が `"--format"` と書いていて名前の `format` と一致しないので出ています。

直し方は2つです。

- その面を決める要求に、名前を引用して書く。面が IR に無いのは仕様の抜けなので、壁打ちで要求を決めます。
- 今は仕様にしないなら、理由を付けて未記載の面の一覧に載せる。一覧に足すのも仕様の判断なので、壁打ちか導入のときだけにします。実装の途中で一覧に足して誤りを消してはいけません。IR に無い面は壁打ちに戻します。

## 未記載の面の一覧

<!-- @kotowari[REQ-core-231:d03e71cc, REQ-core-232:3989b803, REQ-core-233:f788681d, REQ-core-234:8a2d9f6e] -->

`surface.unspecified` が指す YAML のファイルです。
1件は `kind`、`name`、`why` のちょうど3つの鍵を持ち、値はどれも文字列で、`why` は空にできません。
`kind` と `name` が面の種類と名前に前後の空白まで同じ文字列で一致すると、その面は IR に無くても誤りになりません。

```yaml
- kind: "flag"
  name: "verbose"
  why: "出力の話題を導入するときに仕様にする"
- kind: "subcommand"
  name: "list"
  why: ""
- kind: "flag"
  name: "old"
  why: "消した"
```

```console
$ kotowari check --format text
docs/surface-unspecified.yaml:- [error] surface_unspecified_invalid subcommand list
docs/surface-unspecified.yaml:- [notice] surface_unspecified_stale flag old
src/main.rs:6 [error] surface_without_spec flag format
src/main.rs:7 [error] surface_without_spec subcommand list
surface: unspecified=1
```

| 指摘 | 起きること | 直し方 |
|---|---|---|
| `surface_unspecified_invalid`（誤り） | 1件の形が違う（鍵が足りない、余計な鍵、文字列でない値、空白だけの `why`）。その1件はどの面も外さない | 形を直す |
| `surface_unspecified_stale`（注意） | 1件に一致する面がコードに無いか、一致する面が IR に書かれた | その1件を消す。名前を変えたなら `name` を直す |

一覧の指摘の `line` はいつも null です。
同じ内容の1件が2つあることは検査しません。

鍵が無いか、ファイルが空（0バイトか注釈だけ）なら、一覧は0件です。
指す先が無いか読めないときは `unreadable file`、UTF-8 でないときは `non-UTF-8 file`、YAML として読めないか最上位が並びでないときは一覧のパスを付けた `config error` で止まります。

## 外した数を見る

<!-- @kotowari[REQ-core-228:095b2109, REQ-core-229:7909d808, TBL-core-028:9645c008] -->

一覧は借りで、置き場ではありません。
借りた量が毎回見えるように、`surface.rules` を書いたプロジェクトでは、`check` が一覧で外した面の数を出します。

- `--format text` では、指摘の行の後の最後の1行に `surface: unspecified=数`。指摘が0件でも、数が0でも出ます。
- JSON では、最上位の `surface` に `unspecified` の鍵1つ。

数えるのは種類と名前の組で、IR にある面は一覧に載っていても数えません。

`status` は `surface` の群に3つの数を出します。
`surface.rules` が空の一覧なら3つとも 0 です。

```console
$ kotowari status --format text | grep '^surface'
surface total=4 specified=1 unspecified=1
```

| 鍵 | 数 |
|---|---|
| `total` | 面の種類と名前の組の数 |
| `specified` | そのうち IR にあるもの |
| `unspecified` | IR になく、一覧で外したもの |

`surface_without_spec` は誤りなので、残っていれば `status` の `complete` は false です。

面のファイル、面の規則のファイル、未記載の面の一覧を読むのは `check` と `status` だけです。
`list`、`query`、`plan`、`mutants` はそれらを読まず、それらによる停止もしません。

## 既存のプロジェクトに入れる

<!-- @kotowari[REQ-core-228:095b2109] -->

面の規則を初めて書くと、まだ IR に書いていない面がまとめて `surface_without_spec` になります。
そのときは、出た面をすべて理由付きで未記載の面の一覧に載せ、話題ごとに要求を書くたびに一覧から外していきます。
`kotowari status` の `unspecified` が、残りの量です。
