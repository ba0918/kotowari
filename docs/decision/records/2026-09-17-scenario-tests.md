# 壁打ちの記録: 具体例（シナリオ）ごとにテストを求める

## Context

`kotowari check` の requirement_without_test は「要求の ID を含む印を持つテストが1つでもあるか」しか見ない。要求1つに境界の場面（具体例、EX）が5つあっても、テスト1本で通る。IR にはシナリオが約 65 本あり、Given / When / Then で観測できる場面を書いているのに、テストとの対応は誰も検査していない（2026-09-17 の実測: 印に EX の ID を書いたテストは 0 本。印の ID の解決は EX にも効くので、書けば unresolved_reference は出ない）。
利用者の問題意識は「ID とテストの対応があるだけで、テストが要求の中身を縛っているかは保証できていない」。形式手法に飛ぶ前に、保証の単位を要求から場面に落とすのが一段目。変異テスト（テストが実装を縛る力の実測）は別の壁打ちに切る。
そこで、EX ごとにテストを求める指摘の種類と、その適用範囲、既存の 65 本の移行、印の意味（EX の印が @about の要求の分も満たすか）を決める。

Position: 承認済み（2026-09-17）。計画 docs/plans/scenario-tests.md のレビューで A16 を足した（計画と同時に承認）。次は cycle

## Agreements

- A1 この壁打ちの範囲は「具体例（EX）ごとにテストを求める」だけ。変異テスト（テストが実装を縛る力の実測）は別の記録に切る
  - why: EX 単位は既存の仕組み（印と ID の解決）の延長で1つの成果物になる。変異テストは外部の道具の結果を読む話で、依存と実行の設計が別
  - decided_by: 利用者（推奨を採用）

- A2 指摘の種類は `scenario_without_test` の1つ。その EX の ID を含む印を持つテストが1本も無いとき error。requirement_without_test はそのまま残す
  - why: 要求に EX が無いこともある。要求と場面の2段で見る
  - decided_by: 利用者（推奨を採用）

- A3 適用するのは、`@about` に挙げた要求のうち1つでも `検証:` が review 以外のものがある EX。全部 review の要求の場面は人が確かめる場面なので求めない
  - why: review の要求はテストを求めない（requirement_without_test と同じ線）
  - decided_by: 利用者（推奨を採用）

- A4 EX の印は、その EX の `@about` の要求の分も満たしたと数える。`@kotowari[EX-101]` だけで REQ-130 の requirement_without_test も消える。逆（要求の印が EX を満たす）は無い
  - why: EX は要求の場面なので、場面をテストしたなら要求もテストした。書き手が `REQ-130, EX-101` と二重に書かなくて済む
  - decided_by: 利用者（推奨を採用）

- A5 既存の約 65 本の EX の移行は実装と同じ cycle で行う。既存のテストの印に EX を足し、対応するテストが無い EX は新しく書く。初日に 65 件の error を抱えて放置しない。移行が終わるまではテスト側の指摘としてゲート（pre-commit）から外れる
  - why: 対応は IR の Given / Then とテストの assert の突き合わせで機械的。テストが無い EX を見つけて書くこと自体が今回の狙い
  - decided_by: 利用者（推奨を採用）

- A6 detail は EX の ID、line はタグの行（missing_tag と同じ）
  - why: TBL-019 の既存の行に相乗りできる
  - decided_by: 利用者（推奨を採用）

- A7 テスト側の指摘の一覧に scenario_without_test を足す: workflow.md の cycle / implement の段落、lefthook の pre-commit の除外、references/findings.md の担当は implementer
  - why: 直すのは implementer で、承認の時点では残ってよい種類
  - decided_by: 利用者（推奨を採用）

- A8 `@about` に要求が1つも無い EX（決定表や性質だけを挙げる）には求めない。適用は A3 の条件だけ
  - why: そういう EX は今 0 本。出てきたら、その決定表を定義に持つ要求を @about に足すほうが筋
  - decided_by: 利用者（推奨を採用）

- A9 EX の印が満たすのは `@about` の要求の分だけ。決定表と性質には元々テストの要求が無い
  - why: requirement_without_test は要求だけを見る
  - decided_by: 利用者（推奨を採用）

- A10 問い合わせの無い言語のテストのファイルにある EX の印は、要求の印と同じ扱い（REQ-087）。ファイルの中の印を全部拾って scenario_without_test を消す側に数える
  - why: 言語ごとの扱いを2つにしない
  - decided_by: 利用者（推奨を採用）

- A11 1本のテストが複数の EX を挙げてよく、1つの EX を複数のテストが挙げてよい。数は見ない
  - why: 要求の印と同じ
  - decided_by: 利用者（推奨を採用）

- A12 IR は coverage.md に REQ-137（scenario_without_test）を足し、REQ-085 の文を「その ID を含む印、または @about にその ID を持つ EX の ID を含む印」に、REQ-087 を scenario_without_test も消す側に数える文に改める。findings.md の TBL-008 と finding-order.md の TBL-019 に行を足す
  - why: coverage.md は要求が4つで上限の中。対応の検査は1つの文書にまとまる
  - decided_by: 利用者（推奨を採用）

- A13 移行の計画は1本。ステップ1で検査を実装（約 65 件の scenario_without_test が出るが、テスト側の指摘なので pre-commit は止まらない）、ステップ2で既存のテストの印に EX を足し、対応の無い EX にテストを書く。ステップ2は機械的な突き合わせなので bulk-executor に回せる。終端は check 0 件
  - why: 検査と移行を同じ枝に置けば、マージの時点で 0 件が保てる
  - decided_by: 利用者（推奨を採用）

- A14 REQ-137 の適用は、"@about" の ID のうち要求として解決できたものの中に、検証の値が "review" 以外のものが1つでもあるとき。"- 検証:" の行が無い要求と、検証の値が4つ以外の要求は数えない（REQ-085 と同じ線）。"@id" が無いシナリオと invalid_id のシナリオには出さない
  - why: 品質レビュー（2026-09-17）で、検証の値が取れない4つの場合（決定表や性質、存在しない ID、検証の行が無い、値が形に合わない）で2通りに読めると分かった。REQ-085 は同じ問題を除外文で明示している
  - decided_by: 利用者（主セッションの提案を kemi の承認で確定。2026-09-17）

- A15 同じ ID のシナリオが2か所以上にあるとき、"@about" は REQ-032 が定める1つ目（パスのバイト順で先の文書、同じ文書では行の小さい方）のシナリオから取る
  - why: duplicate_id が出ても検査は続くので、どちらの @about を使うかを決めないと REQ-085 と REQ-137 の結果が実装者次第になる。1つ目は REQ-032 が既に定義している
  - decided_by: 利用者（主セッションの提案を kemi の承認で確定。2026-09-17）

- A16 同じ ID のシナリオが2か所以上にあって印が無いとき、scenario_without_test は REQ-032 の1つ目のシナリオのタグの行に1件だけ出す。REQ-085 の判定でも同じ1つ目の "@about" を使う
  - why: 計画のレビュー（2026-09-17）で、件数と line が REQ-137 と TBL-019 から決まらないと分かった。duplicate_id が2つ目以降に出るので、対応の指摘は1つ目に寄せると読み手が一か所で済む。REQ-085 の側は A15 が名指ししていたが IR の文に無かった
  - decided_by: 利用者（主セッションの提案を計画と同時に kemi の承認で確定。2026-09-17）

## Prohibitions

（なし）

## Undecided

- U1 第1ラウンドの Q1〜Q8。A1〜A7 で決めた（2026-09-17）。残るのは U2
  - decides: 利用者（決定済み）

- U2 具体例が無い unit の要求に「EX を1つ以上」を求める指摘を足すか。今回は足さない
  - decides: 利用者（別の判断。書く側の負担が大きい）
  - related: A2

## Delegated

（なし）

## Rejected

（なし）

## Revisions

- A12 の why「要求が4つ」は REQ-137 を足す前の数で、足した後は5つ（上限 10 の中。結論は変わらない）
