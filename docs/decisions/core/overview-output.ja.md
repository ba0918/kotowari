# 全体像の元データの申告

[English](overview-output.md) | 日本語

全体像の元データの検査の結果を、"kotowari check" と "kotowari status" の出力にどう加えるかを扱う。検査そのものは[全体像の元データ](overview-data.ja.md)で決める。status の "overview" の群は[揃っているかの集計](status.ja.md)の[集計は、テストのマーカー、ガイド、全体像、面、指摘の数を群ごとに出す](status.ja.md#集計はテストのマーカーガイド全体像面指摘の数を群ごとに出す)で、ライブラリが検査の結果を core に渡す形は[メモリの入力と読み込んだ結果](library-inputs.ja.md)の[名前を付けた追加の指摘の群を受け取り、ほかの指摘と一緒に並べて数える](library-inputs.ja.md#名前を付けた追加の指摘の群を受け取りほかの指摘と一緒に並べて数える)で決める。

## check の JSON に、ガイドと同じ形の overview の群を出す

状態: 実装済み

### 問題

全体像の元データの検査の結果を check の出力にどう載せるかを決める必要があった。ガイドと同じ扱いにし、新しい決まりを作らない。

### 決めたこと

"kotowari check" の JSON の最上位に、"guides" と同じ形の "overview" の群を出す。"files" は読んだ全体像の元データの数、"marks" は元データの中の形の正しいマーカーの数にする。status も同じ群を出す。

### 確認すること

- C-core-ovout-1 "kotowari check --format json" の最上位の "overview" に、読んだ元データの数が "files"、元データの中の形の正しいマーカーの数が "marks" として出る

## 元データの指摘は、ほかの指摘と同じく並べて数え、終了コードと complete に効かせる

状態: 実装済み

### 問題

元データの形の崩れは、check がそれぞれの場所に誤りとして出す。check に入れれば、コミットの前の検査でも捕まる。全体像の指摘だけ別の並べ方や別の合否の決まりを持つと、出力の順番の決まりが2つに分かれる。

### 決めたこと

全体像の元データの指摘は、"kotowari check" と "kotowari status" の指摘に加える。ほかの指摘と合わせて、[指摘の並べ方と行](finding-order.ja.md)の[指摘は path、line、kind、detail の順に並べる](finding-order.ja.md#指摘は-pathlinekinddetail-の順に並べる)のとおりに並べ、"counts"、終了コード、status の "complete" にも、ほかの指摘と同じく数える。status の complete は check の誤りが0件であることを条件にするので、元データに誤りがあれば false になる。

### 確認すること

- C-core-ovout-2 元データに誤りがあると、"kotowari check --format json" の "findings" と "counts" にその誤りが入り、終了コードが1になる
- C-core-ovout-3 元データの指摘は、ほかの指摘と同じ並びの決まりで "findings" に並ぶ
- C-core-ovout-4 ほかに誤りが無くても、元データに誤りがあると、"kotowari status" の "complete" は false になる
