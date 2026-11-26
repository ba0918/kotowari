# 3つの置き場すべてに当たるファイルの停止の詳細

## Context

[全体像の書き出しでテストのファイルの中身を読まない変更](./2026-10-23-overview-test-contents.md)の整合のフェーズで、コードと IR を読み合わせた。
[REQ-core-280](../../ir/core/overview-data.md#REQ-core-280) は、[REQ-core-199](../../ir/core/guides.md#REQ-core-199) の重なりを先に判定すると書く一方で、1つのファイルが "overview.files"、"guides.files"、"tests.files" の3つすべてに当たるときは ": matched by both overview.files and guides.files" を出すと書いていた。
3つすべてに当たるファイルはガイドとテストの重なりでもあるので、先に判定する REQ-core-199 の停止が必ず先に起きる。2つの文は同じ場面で別の詳細を求めており、両方は成り立たない。
check でも build でも、実際に出るのは REQ-core-199 の詳細である。

Position: 整合のフェーズで決めた（2026-10-23）。

## Agreements

- A1 1つのファイルが "overview.files"、"guides.files"、"tests.files" の3つすべてに当たるとき、kotowari は REQ-core-199 の停止をし、詳細は ": matched by both guides.files and tests.files" で終わる。[REQ-core-280](../../ir/core/overview-data.md#REQ-core-280) の最後の文をこの形に直し、コードは変えない
  - why: [A73](./2026-10-02-whole-picture.md#A73) と [A75](./2026-10-02-whole-picture.md#A75) はどちらもガイドとテストの重なりを先に判定すると決めており、[REQ-core-199](../../ir/core/guides.md#REQ-core-199) はその重なりの詳細を ": matched by both guides.files and tests.files" と決めている。先に判定する以上、3つすべてに当たるファイルで元データの重なりの文言が出る場面は無い。IR の部分どうしの食い違いなので、先に判定することと REQ-core-199 に合う側に IR を揃える。A75 の「3つすべてに当たれば guides の文言を出す」は、元データの重なりの文言を選ぶ順としては残るが、利用者からは観測できない
  - decided_by: 整合のフェーズ（根拠: 3つの glob がすべて "tests/**" に当たり "tests/a.md" がある使い捨ての場で、`kotowari overview build` と `kotowari check` がどちらも終了コード 2、標準エラーの1行目 "config error: tests/a.md: matched by both guides.files and tests.files" を出した。同じ場面を "tests/step18_overview.rs" の `req_core_280_the_guides_and_tests_overlap_is_judged_first_and_guides_win_when_all_three_match`（check）と "tests/step19_overview_build.rs" の `req_core_280_build_judges_the_guides_and_tests_overlap_first`（build）が同じ文字列で確かめている。コードは "crates/kotowari/src/acquisition.rs" の `load_overview` と `read_guides_and_overview` が `guides::collect` で REQ-core-199 を判定してから "crates/kotowari/src/overview.rs" の `validate_overlap` を呼ぶ）

## Rejected

- A2 REQ-core-280 の書いたとおりに、3つすべてに当たるときは ": matched by both overview.files and guides.files" を出すようコードを変える
  - why: ガイドとテストの重なりを先に判定する決定（A73、A75）と REQ-core-199 に反する。どちらかの文を捨てなければ実現できない
