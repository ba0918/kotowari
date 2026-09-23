# 壁打ちの記録: Rust 以外の言語のテストを見つける

## Context

kotowari がテストを見つけられるのは ".rs" だけで（[REQ-core-081](../../ir/core/test-discovery.md#REQ-core-081)、[A128（records）](./records.md#A128)）、ほかの言語は印を拾うだけで test_without_id を出せない。利用者が使う見込みの高い TS/JS、Python、PHP に広げたい。言語ごとに判定のコードを手で書く今の形（"crates/kotowari-core/src/tests_discovery.rs" は Rust の構文木を手で辿る）は言語を足すたびに重いので、ast-grep（tree-sitter の上で構文木のパターンとルールを書ける道具）をライブラリとして使い、言語ごとの判定をルールに寄せる案から始める。

事前の実測（2026-09-24、ast-grep 0.45.3）: ast-grep-core、ast-grep-config、ast-grep-language（既定の機能を切り、rust、python、typescript、javascript、php だけ）を組み合わせた小さな crate がビルドできた（tree-sitter 0.27）。YAML のルールで Python の "def test_*"（クラスの中のメソッドを含む）、TS/TSX の "it(...)" と "test(...)"、PHP の "test*" メソッドと "#[Test]" 属性の付いたメソッドが当たり、"$NAME" のメタ変数で名前（TS は引用符ごとの文字列）が取れた。構文の誤りは tree-sitter の has_error で分かる。拡張子から言語への対応は ".ts .mts .cts" が TypeScript、".tsx" が Tsx、".js .jsx .mjs .cjs" が JavaScript、".py" が Python、".php" が Php、".rs" が Rust で、".PY" は言語なし。

Position: 照合を3回終えた。3回目に残った2件（EX-core-301、EX-core-320）は照合役が名指しした出典を足すだけの直しで、FLAG にしなかった。承認待ち。

## Agreements

- A1 テストを見つける仕組みを ast-grep に一本化する。Rust も構文木を手で辿る今の形をやめてルールに移し、"tests.rust.macros" のマクロの中身の読み直しだけを Rust のコードに残す。依存に ast-grep-core、ast-grep-config、ast-grep-language（いずれも MIT）を足し、tree-sitter を 0.27 に上げることを受け入れる
  - why: Rust だけ今の形に残すと、印をテストに結び付ける処理が2通りになり、言語によって結び付き方がずれる元になる。版の組み合わせは実測でビルドできた（Context）
  - decided_by: 利用者（推奨を採用）
- A2 何をテストと数えるかのルールは、kotowari が言語ごとの既定を同梱し、そのうえでプロジェクトが設定でルールを足せるようにする。[A128（records）](./records.md#A128) の「設定で問い合わせを足せないことを禁止の要求として書く」と [REQ-core-121](../../ir/core/cli-environment.md#REQ-core-121) を改める
  - why: 何をテストと数えるかは採用しているライブラリで変わる（[A26（records）](./records.md#A26)）。Rust でも "tests.rust.attributes" を足せるようにしている。同じことが jest/vitest/mocha、pytest/unittest、PHPUnit/Pest でも起き、同梱だけだとライブラリが増えるたびに kotowari 本体を直すことになる。設定の正規表現を退けた理由（[R3（records）](./records.md#R3)）は行を正規表現で探すと誤検出することで、構文木の節に当てるルールはこれに当たらない
  - decided_by: 利用者（推奨を採用）
- A3 読める言語は絞らず、一般的な言語はある程度広く読めるようにする
  - why: ルールで判定する形なら言語を足す手間が小さく、読める言語が多いほど kotowari を使える場面が広がる（利用者の言葉「間口が広くなるので」）
  - decided_by: 利用者
- A4 用語「問い合わせ」「問い合わせのある言語」「問い合わせの無い言語」は残し、「問い合わせ」の意味を言語ごとのテストの見つけ方のルールに書き換える
  - why: JSON 出力の "query" の鍵がこの言葉に対応していて、改名すると出力の互換が壊れる
  - decided_by: 利用者（推奨を採用）
- A5 "tests.files" の既定は "src/**/*.rs" と "tests/**/*.rs" のまま変えない。ほかの言語のプロジェクトは自分で glob を書く
  - why: テストの置き場所の慣習は言語ごとにばらばらで（"__tests__/"、"*.test.ts"、"test_*.py"、"tests/Unit/"）、既定で当てにいくと外すか余計に拾う
  - decided_by: 利用者（推奨を採用）
- A6 文法は ast-grep-language の全27言語を入れる
  - why: release ビルドの実測で、5言語 10.9MB、プログラミング言語16個 39.1MB、全27個 45.8MB（今の kotowari は 9.3MB）。16個との差は 6.7MB で、どれを選ぶかの判断が要らなくなる。文法が入っていれば、使う人が設定でルールを足すだけで（A2）その言語を使える
  - decided_by: 利用者（推奨を採用）
  - superseded_by: [A46](#A46)
- A7 `問い合わせのある言語`は、同梱か設定のルールが1つ以上ある言語とする。文法があってもルールが1つも無い言語は`問い合わせの無い言語`のままで、印を全部拾うだけで構文木を読まない
  - why: 文法を入れただけで振る舞いが変わると、たとえば ".md" を glob に入れている利用者に unparsable_file が急に出る
  - decided_by: 利用者（推奨を採用）
- A8 この仕事で既定のルールを同梱するのは Rust、TS/JS、Python、PHP の4つ。ほかの言語は設定でルールを足す。既定を増やすのは後から言語ごとに決める
  - why: 既定のルールには「確実なものだけ既定にする」（[A26（records）](./records.md#A26)）が当たり、言語ごとに何が確実かを決める壁打ちと仕様とテストが要る。利用者が使う言語に絞る
  - decided_by: 利用者（推奨を採用）
- A9 設定でルールを足す鍵は "tests.rules" で、ast-grep のルールの YAML ファイルのパス（基準のディレクトリからの相対）を並べる。ファイルの中身は ast-grep のルールの形をそのまま使う
  - why: ルールのファイルがそのまま "ast-grep scan -r" で試せ、何が当たるかを kotowari を通さずに確かめられる。設定ファイルの中にルールを書く案は試しにくい
  - decided_by: 利用者（推奨を採用）
- A10 IR には「ast-grep のルールの形」を契約として書き、形を自作しない。kotowari の側の約束として、ルールが当たった節を`テスト`、メタ変数 "$NAME" に入ったものを`テスト`の名前にすることを書く
  - why: 使う人がルールを書くので、その形は契約になる。道具の語を要求に漏らさない決め（[A12（mutation-tests）](./2026-09-17-mutation-tests.md#A12)）は道具を差し替えられるようにするためで、使う人が直接書く形には当たらない
  - decided_by: 利用者（推奨を採用）
- A11 同梱のルールと設定のルールの関係は足すだけで、設定で同梱のルールを外したり置き換えたりはできない
  - why: "tests.rust.attributes" と同じ扱い。同梱のルールが誤検出しても、テストのファイルは "tests.files" の glob で絞れるので実害は小さい
  - decided_by: 利用者（推奨を採用）
- A12 "tests.rust.attributes" と "tests.rust.macros" は設定の形を変えずに残す
  - why: kotowari 自身の "kani::proof" と "proptest" を含め、既存の設定を壊さない
  - decided_by: 利用者（推奨を採用）
- A13 `テスト`の名前は、"$NAME" が文字列リテラルなら引用符を外した中身、それ以外なら節の文字そのままにする。ルールが "$NAME" を捕まえなければ名前は null にする
  - why: TS の "it(\"does x\", ...)" は引用符ごとの文字列が捕まる（Context の実測）。list の "name" は今も null を取れる（[問い合わせの無い言語の扱い](../../ir/core/list.md)）
  - decided_by: 利用者（推奨を採用）
- A14 今まで`問い合わせの無い言語`だった ".py"、".ts"、".php" などの`テストのファイル`は、既定のルールが付くことで`テスト`の外の印を無視し、印の無い`テスト`に test_without_id を出すようになる。この変化を受け入れる
  - why: それがこの仕事の目的。既存のプロジェクトで kotowari を上げると指摘が増えうる
  - decided_by: 利用者（推奨を採用）
- A15 `印`の結び付けは`テスト`の直前のコメントの塊だけにし、関数の本体の先頭のコメントの`印`は結び付けない（[A39（records）](./records.md#A39) と [TBL-core-016](../../ir/core/test-markers.md#TBL-core-016) を改める）
  - why: 本体の先頭を言語に依存せずに決めるのが難しい（PSR-12 の次の行の "{"、Python のコメントが "block" の手前に入るクセ）。kotowari 自身のテストの1124個の印で本体の先頭にあるのは1個で、それも説明のコメントに印の文字が入っていただけ。結び付かない印のテストには test_without_id が出るので黙って消えない
  - decided_by: 利用者（推奨を採用）
- A16 直前のコメントの塊は、`テスト`の節の最初の行の直前から上に向かって、空行が来るまで続くコメントだけの行の塊とする。コメントは tree-sitter の extra の節で見分ける
  - why: 4言語の実測で、コメントはどれも extra の節だった。行で決めれば、Python のようにコメントが構文木のどこに入っても同じ結果になる
  - decided_by: 利用者（推奨を採用）
- A17 コメントの塊と`テスト`の間に挟まってよい行は言語ごとに kotowari が同梱し、Rust は属性、Python はデコレータとする。設定でルールを足しただけの言語はコメントの行だけ
  - why: Java、C#、PHP、Kotlin のようにアノテーションや属性が`テスト`の節の中に入る言語は表が要らない（PHP の "#[Test]" はメソッドの節の中にあることを実測した）。外に出るのは Rust と Python
  - decided_by: 利用者（推奨を採用）
- A18 TS/JS の既定のルールは、"it(...)" と "test(...)" の呼び出し、"it." か "test." の後に名前が続く形（"it.only"、"it.skip"、"it.todo"、"test.concurrent"）、"it.each(表)(名前, 関数)" の形を数え、"describe" は数えない。名前は最初の引数
  - why: jest、vitest、mocha、node:test、bun で "it" と "test" は共通で確実。skip したテストもテストの定義。"regex.test(s)" は "it"、"test" が呼ぶ側でないので当たらない
  - decided_by: 利用者（推奨を採用）
- A19 Python の既定のルールは、名前が "test" で始まる関数とメソッド（ファイルの最上位の関数と、クラスの中のメソッド）を数え、関数の中に入れ子になった関数は数えない
  - why: pytest の既定の探し方と unittest の "test*" メソッドの両方を拾える
  - decided_by: 利用者（推奨を採用）
- A20 PHP の既定のルールは、名前が "test" で始まるメソッド、パスの最後の要素が "Test" の属性の付いたメソッド（"#[\PHPUnit\Framework\Attributes\Test]" を含む）、docblock に "@test" のあるメソッド、Pest の "test(...)" と "it(...)" の呼び出しを数える
  - why: "@test" の docblock は PHPUnit 12 で消えたが書いてあるコードは多い。テストのファイルで "test('...', fn)" を呼んでいれば Pest でほぼ確実
  - decided_by: 利用者（推奨を採用）
- A21 TypeScript と TSX は ast-grep と同じく別の言語として扱い、"language: typescript" のルールは ".tsx" に当たらない。同梱のルールは両方に付ける
  - why: ast-grep のルールの形を契約にした（A10）ので、kotowari だけ読み替えるとずれる
  - decided_by: 利用者（推奨を採用）
- A22 "tests.rules" のファイルが無い、YAML として読めない、ast-grep のルールとして読めない、"language" が入れた文法に無い、のいずれかなら設定の誤りとして`停止`する
  - why: 今の設定の誤りと同じ扱い
  - decided_by: 利用者（推奨を採用）
- A23 `問い合わせのある言語`の`テストのファイル`に構文の誤りが1つでもあれば、今と同じく unparsable_file を出してそのファイルを飛ばす。`問い合わせの無い言語`のファイルは構文木を読まないので出さない
  - why: 今の振る舞い（[A58（records）](./records.md#A58)、[A120（records）](./records.md#A120)）を言語を問わず保つ
  - decided_by: 利用者（推奨を採用）
- A24 JSON 出力の "tests" の "query" は、その拡張子から決まる言語に同梱か設定のルールが1つ以上あれば true にする
  - why: `問い合わせのある言語`の定義（A7）に合わせる
  - decided_by: 利用者（推奨を採用）
- A25 入れ子の "describe" の中の "it" の名前は、"it" 自身の最初の引数だけにし、外の "describe" の名前は付けない
  - why: 名前は list に出す情報で印の結び付けには使わない。つなげると区切り文字などの決めごとが増える
  - decided_by: 利用者（推奨を採用）
- A26 "tests.rust.macros" のマクロの中の関数の印は、マクロの外と同じ行の規則（A16、A17）で結び付ける。マクロの中の最上位の関数ごとに数えることは変えない
  - why: 結び付けの規則を1つにする（A1 と同じ理由）
  - decided_by: 利用者（推奨を採用）
- A27 同じ節に複数のルールが当たったら`テスト`は1つと数え、`テスト`の節の中にさらに当たった節は別の`テスト`として数える
  - why: 同梱のルールと設定のルールが同じ "it(...)" に当たって2つに数えるのを防ぐ
  - decided_by: 利用者（推奨を採用）
- A28 "tests.rules" の既定は空の一覧で、値は基準のディレクトリからの相対パスの一覧。glob は使えない
  - why: "mutants.equivalents" と同じくパスそのものを書く形にそろえる
  - decided_by: 利用者（推奨を採用）
- A29 ast-grep のルールの "files" と "ignores" は守り、パスは基準のディレクトリからの相対とする。"fix"、"message"、"severity"、"note"、"metadata" は kotowari では使わない（"severity: off" でもルールは効く）
  - why: "files" でルールをテストのファイルだけに絞れると便利。ルールを止めたいときは "tests.rules" から外す
  - decided_by: 利用者（推奨を採用）
- A30 スキル kotowari の references（"mark.md" の印の結び付けと Rust 以外の言語の節、"config.md" の設定の表と setup の YAML）の書き直しもこの仕事に入れる
  - why: 既定のある鍵をすべて setup の YAML に書くことを [REQ-core-126](../../ir/core/skill-references.md#REQ-core-126) が確かめるので、"tests.rules" を足さないとテストが落ちる。"mark.md" もそのままでは本体と食い違う
  - decided_by: 利用者（推奨を採用）
- A31 名前が null の`テスト`に test_without_id を出すときの detail は、`テスト`の節の最初の行の文字から前後の空白を除いたものにする
  - why: 空の文字列ではどのテストか分からない。ほかの種類でも行の文字そのままを detail にしている（[A150（records）](./records.md#A150)）
  - decided_by: 利用者（推奨を採用）
- A32 test_without_id の "line" は、言語を問わず`テスト`の節の最初の行にする。Rust と Python は属性とデコレータが節の外にあるので "fn" と "def" の行、PHP の "#[Test]" の付いたメソッドは節の中に属性があるので "#[Test]" の行になる
  - why: 「関数の宣言の行」は関数でない`テスト`（TS の "it(...)"）に当てはまらない
  - decided_by: 利用者（推奨を採用）

- A36 `テスト`の名前は、"$NAME" の節の文字の最初と最後が同じ引用符（一重引用符、二重引用符、バッククォートのいずれか）ならその1文字ずつを外し、それ以外の形（Python の三重引用符や "f" の接頭辞の付いた文字列）はそのまま名前にする（A13 を具体にする）
  - why: 文字列リテラルを言語に依存せずに見分けるのは難しい。名前が文字列になるのは TS/JS と Pest の "it('...')" くらいで、この規則で足りる
  - decided_by: 利用者（推奨を採用）
- A37 TS/JS の既定のルールの「"it" か "test" に "." と名前が続く形」は、"." と名前が1つ以上続く形とする（"it.only.each" を含む）。最後の要素が "each" の呼び出しの結果をさらに呼ぶ形では外側の呼び出しだけを数え、内側の "it.each(表)" は数えない（A18 を具体にする）
  - why: 内側の "it.each(表)" も "." の形に当たり、1つのテストを2つに数えてしまう（照合と敵対的レビューの指摘）
  - decided_by: 利用者（推奨を採用）
- A38 "tests.rust.macros" のマクロの中の関数の`テスト`の名前は、今のまま関数の名前にする
  - why: マクロの中の関数はルールが当たった節でないので "$NAME" の規則が効かず、書かないと名前が null になる
  - decided_by: 利用者（推奨を採用）
- A39 コメントだけの行は、前後の空白を除いた行の文字がすべてコメント（tree-sitter の extra の節）の文字である行とし、複数行のコメントの途中の行を含め、コードと同じ行にあるコメントの行は含めない
  - why: A16 の「コメントだけの行」が定義されていなかった（敵対的レビューの指摘）
  - decided_by: 利用者（推奨を採用）
- A40 コメントの行と、挟んでよい行（Rust の属性、Python のデコレータ）が空行なしで混ざって続けば1つの塊とし、複数行にわたる属性とデコレータはその全部の行を挟んでよい行とする（A16、A17 を広げる）
  - why: Rust は今もコメントと属性が混ざっても結び付く（[A39（records）](./records.md#A39)）。複数行を許さないと "@pytest.mark.parametrize(...)" のよくある書き方で印が結び付かない
  - decided_by: 利用者（推奨を採用）
- A41 `テスト`の節の中にある別の`テスト`の直前のコメントの塊の印は、その内側の`テスト`に結び付ける。「`テスト`の節の中の印は無視する」からは、ほかの`テスト`の直前のコメントの塊を除く
  - why: 入れ子のテスト（A27）で、結び付ける行と無視する行の両方が当たっていた（敵対的レビューの指摘）
  - decided_by: 利用者（推奨を採用）
- A42 名前が null の`テスト`の detail（A31）は、`テスト`の節の最初の行の全体の文字から前後の空白を除いたものとする
  - why: 節が行の途中から始まる形（"const t = test(...)"）で差が出る。"line" も行で決めている（A32）
  - decided_by: 利用者（推奨を採用）
- A43 "tests.rules" のパスがファイルでない、読めない、UTF-8 でない、同じパスが2回並んでいる、のいずれかも設定の誤りとして`停止`する（A22 に足す）。ルールの "id" の重なりは見ない
  - why: どれにも当てはまらずに黙って読み飛ばすことになり、[REQ-core-120](../../ir/core/cli-environment.md#REQ-core-120) に反する。kotowari はルールの "id" を使わない
  - decided_by: 利用者（推奨を採用）
- A44 ルールの "language" の値は ast-grep と同じく大文字小文字を区別せずに突き合わせ、"ts"、"py" のような別名も受ける
  - why: ast-grep のルールの形をそのまま契約にした（A10）
  - decided_by: 利用者（推奨を採用）
- A45 ルールの "files" と "ignores" の glob は ast-grep と同じ読み方にし、"tests.files" の読み方には合わせない
  - why: ast-grep のルールの形をそのまま契約にした（A10）。"ast-grep scan" で試したときと同じ結果になる
  - decided_by: 利用者（推奨を採用）
- A46 入れる文法は ast-grep-language の全27個で、言語は28個とする（TypeScript の文法が TypeScript と Tsx の2言語を持つ）。A6 の「全27言語」を訂正する
  - why: 照合で、拡張子と言語の対応の表が28行あり A6 の数と合わないと指摘された
  - decided_by: 利用者（推奨を採用）

- A47 拡張子と言語の対応は、入れる ast-grep-language が持つ対応をそのまま使い、IR の表（[TBL-core-031](../../ir/core/test-discovery.md#TBL-core-031)）に写す
  - why: ast-grep を契約にした（A10）ので、拡張子の読み方も ast-grep とずれないようにする。"ast-grep scan" と同じファイルが同じ言語になる
  - decided_by: 利用者（推奨を採用）
- A48 同梱のルールの`テスト`の名前は、Python と Php のメソッドと関数では関数やメソッドの名前、Pest の "test(...)" と "it(...)" では最初の引数とする
  - why: 同梱のルールが "$NAME" に何を入れるかを決めていなかった（照合の指摘）。TS/JS の最初の引数（A18）、Rust の関数の名前と同じ考え方
  - decided_by: 利用者（推奨を採用）
- A49 結び付けない`印`（関数の本体の先頭を含む`テスト`の節の中の印）には invalid_marker も unresolved_reference も出さない
  - why: 今もテストの外と本体の途中の印は無視して指摘を出さない（[A67（records）](./records.md#A67)）。本体の先頭の印も結び付けなくなった（A15）ので同じ扱いにそろえる
  - decided_by: 利用者（推奨を採用）

## Rejected

- A33 Rust だけ構文木を手で辿る今の形に残し、新しい言語だけ ast-grep にする
  - why: 印の結び付けの処理が2通りになる（A1）
- A34 同梱のルールだけにし、設定でルールを足せないままにする
  - why: 採用しているライブラリが増えるたびに kotowari 本体を直すことになる（A2）
- A35 関数の本体の先頭のコメントの印を結び付け続ける
  - why: 本体の先頭を言語に依存せずに決められない（A15）

## Revisions

- A2 は [A128（records）](./records.md#A128) の「設定で問い合わせを足せないことを禁止の要求として書く」と、[A58（records）](./records.md#A58) の「言語を足すのは kotowari 自体の変更」を改める
- A1 は [A24（records）](./records.md#A24) の「言語を足すときは問い合わせのファイルを足す」を、ast-grep のルールを足す形に改める
- A15 は [A39（records）](./records.md#A39) と [A26（records）](./records.md#A26) の「関数の本体の最初のコメントの印も結び付ける」を改める
- A7 と A8 は [A128（records）](./records.md#A128) の「第1版で問い合わせのある言語は Rust だけ」を改める
- A46 は A6 の「全27言語」を「文法27個、言語28個」に訂正する
- A36、A37 は A13、A18 を具体にする。A40 は A16、A17 を広げる。A42 は A31 を具体にする。A43 は A22 に足す
