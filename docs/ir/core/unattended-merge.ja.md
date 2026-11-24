# 人を待たずにマージする

[English](unattended-merge.md) | 日本語

工程の skill が、1つの依頼を仕様の承認からマージまで人を待たずに通すやり方を扱う。計画を書かなくなった cycle が何を入力にするか、人がマージを確認するかを決める線、マージのしかた、その回について記録するもの。

## Requirements

### REQ-core-364: 承認から実装へ

- kind: ubiquitous
- source: docs/decision/records/2026-11-24-unattended-merge.md#A4, docs/decision/records/2026-11-24-unattended-merge.md#A22, docs/decision/records/2026-11-24-unattended-merge.md#A24, docs/decision/records/2026-11-24-unattended-merge.md#A27, docs/decision/records/2026-11-24-unattended-merge.md#A33, docs/decision/records/2026-11-24-unattended-merge.md#A37
- verification: review
- how_to_verify: "agent/skills/kotowari-brainstorm/"、"agent/skills/kotowari-cycle/"、"agent/skills/kotowari-implement/"、"agent/skills/kotowari-iterate/" と "agent/skills/kotowari-using-workflow/" を読み、次のすべてが書いてあることを確かめる。計画を書く工程も、その承認も無く、"agent/skills/kotowari-plan/" が存在しないこと。人が承認済みの仕様をコミットした、またはコミットしてよいと言ったとき、人が後回しにすると言わない限り、壁打ちは同じ流れで cycle に進むこと。cycle は、仕様の置き場と、壁打ちが足した・変えた要求の ID を壁打ちから受け取り、あとから始めるときや再開のときは、枝の差分のうち仕様の置き場にある部分から数え直すこと。cycle は、後回しの ID を外してから実装役に渡すこと。cycle は再開のための状態を新しく持たないこと。実装の途中の再開は、git の履歴と、その要求（読んで確かめる種類のものを除く）のうちまだ印の付いたテストが無いものから実装役が決め、実装役の作業メモがあればそれも読むこと。プルリクエストを作ったあとの再開は、プルリクエストがあるか、自動のマージを頼んであるか、検査の状態、マージ済みかを、ホスティングのサービスから読むこと。テストのコマンドは、プロジェクトの指示か、その言語の標準の道具で決め、どちらでも決まらなければ人に聞くこと。順番と触るファイルは実装役が決め、作業メモは要ると判断したときだけ書くこと。メモはコミットせず、人は読まないこと

kotowari-cycle は常に、仕様のある依頼を、その仕様の承認から人を待たずに始め、仕様の置き場と、壁打ちが足した・変えた`要求`の`ID`を入力にし、あいだに`計画書`を書く工程を置かない。

### REQ-core-365: 線はパスを git で判定する

- kind: ubiquitous
- source: docs/decision/records/2026-11-24-unattended-merge.md#A3, docs/decision/records/2026-11-24-unattended-merge.md#A9, docs/decision/records/2026-11-24-unattended-merge.md#A16, docs/decision/records/2026-11-24-unattended-merge.md#A19, docs/decision/records/2026-11-24-unattended-merge.md#A20, docs/decision/records/2026-11-24-unattended-merge.md#A39, docs/decision/records/2026-11-24-unattended-merge.md#A44
- verification: review
- how_to_verify: "agent/skills/kotowari-cycle/" を読み、次のすべてが書いてあることを確かめる。人が確認する変更と、確認なしでマージする変更の線は、git だけで判定すること。枝がマージ先から分かれたコミットから枝の先頭までに変わったファイルのパスを、".kotowari/confirm-paths" に1行に1つ書いたパスの指定と照らすこと。マージ先は、プルリクエストの向き先の枝で、決めていなければリポジトリの既定の枝であること。移したファイルは、元のパスと先のパスの両方で数えること。プロジェクトがこのファイルに書くのは、検査の定義（CI、フック、検査を走らせるスクリプト、変異テストの設定、静的解析と道具の版の設定）であり、仕様と判断の記録の置き場は書かないこと。この手順を持つ skill の文の元を持つリポジトリは、その元のパスもそこに書くこと。".kotowari/confirm-paths" 自身は、ファイルに書かれていてもいなくても、常に当たりとすること。ファイルが無いときは、当たりと同じ結果にすること。結果は当たりか当たりでないかの2つだけで、LLM は判定しないこと。cycle がプルリクエストの枝へコミットを足すたびに、いつも差分の全体で判定し直し、あとの変更で差分から消えた当たりは当たりでなくなること。判定に kotowari のコマンドを使わないこと

kotowari-cycle は常に、枝がマージ先から分かれたコミットからの差分が、".kotowari/confirm-paths" に並んだパスか、そのファイル自身に触れたかを、git だけで、枝へコミットを足すたびに判定する。

### REQ-core-366: 人を待たずにマージする

- kind: ubiquitous
- source: docs/decision/records/2026-11-24-unattended-merge.md#A1, docs/decision/records/2026-11-24-unattended-merge.md#A12, docs/decision/records/2026-11-24-unattended-merge.md#A23, docs/decision/records/2026-11-24-unattended-merge.md#A24, docs/decision/records/2026-11-24-unattended-merge.md#A28, docs/decision/records/2026-11-24-unattended-merge.md#A35, docs/decision/records/2026-11-24-unattended-merge.md#A39, docs/decision/records/2026-11-24-unattended-merge.md#A41, docs/decision/records/2026-11-24-unattended-merge.md#A43, docs/decision/records/2026-11-24-unattended-merge.md#A44, docs/decision/records/2026-11-24-unattended-merge-consistency.md#A1
- verification: review
- how_to_verify: "agent/skills/kotowari-cycle/" と "agent/skills/kotowari-iterate/" を読み、次のすべてが書いてあることを確かめる。マージへ進むのは、収束の終わり方に着いた回だけであること。ほかの終わり方（往復の上限、進まない、差し戻し）に着いた回は、これまでどおり人に報告し、マージを頼まないこと。回が収束し、線に当たらず、REQ-core-367 のどれにも当たらないとき、cycle はプルリクエストを作り、必須の検査が通ったら自動でマージするようホスティングのサービスに頼むこと。cycle はマージを自分で行わないこと。頼むのは、リポジトリがその形のマージを許していることと、マージ先が検査を1つ以上必須にしていることの両方を確かめたときだけで、どちらかが欠けているか確かめられないときは、プルリクエストで止まり、自動のマージが使えないことを報告し、人がマージすること。プルリクエストを作れないホスティングでは、枝を push したところで止まること。cycle はリポジトリの設定を変えないこと。頼んだあとは、枝へコミットを足す前に頼みを取り消し、足したあとの先頭について、回が収束し、線に当たらず、REQ-core-367 のどれにも当たらないことを確かめてから頼み直すこと。このどれかが崩れたときと、プルリクエストを人に渡すときも、頼みを取り消すこと。プルリクエストを作ったあとは、プルリクエストで止まる回でも、必須の検査の結果まで見届けること。落ちた必須の検査は、指摘として指摘のファイルに入れ、いまある修正の往復と終わり方に従い、それでも通らなければ cycle はプルリクエストを人に渡すこと。見届けは、cycle がプルリクエストのマージを見たとき、検査が落ちてプルリクエストを人に渡したとき、検査は通ったのにホスティングのサービスがマージしないと返したとき（cycle は理由を付けて人に渡す）、プルリクエストで止まる回で検査が通ったときに終わり、マージ先に必須の検査が1つも無いときはすぐに終わること。cycle は見届けに自分の時間切れを持たないこと。kotowari-iterate で回す小さい作業も同じ形でマージすること。主セッションが直接編集する変更は、これまでどおりコミットで終わること

kotowari-cycle は常に、回が収束し、線に当たらず、人の決めることが残っていないとき、プルリクエストを作り、必須の検査が通ったらマージするようホスティングのサービスに頼み、人を待たずにその検査の結果まで見届ける。

### REQ-core-367: 人が確認するとき

- kind: ubiquitous
- source: docs/decision/records/2026-11-24-unattended-merge.md#A3, docs/decision/records/2026-11-24-unattended-merge.md#A12, docs/decision/records/2026-11-24-unattended-merge.md#A15, docs/decision/records/2026-11-24-unattended-merge.md#A19, docs/decision/records/2026-11-24-unattended-merge.md#A21, docs/decision/records/2026-11-24-unattended-merge.md#A26, docs/decision/records/2026-11-24-unattended-merge.md#A31, docs/decision/records/2026-11-24-unattended-merge.md#A32, docs/decision/records/2026-11-24-unattended-merge.md#A39, docs/decision/records/2026-11-24-unattended-merge.md#A44, docs/decision/records/2026-11-24-unattended-merge.md#A46
- verification: review
- how_to_verify: "agent/skills/kotowari-cycle/" を読み、次のすべてが書いてあることを確かめる。収束した回のうち、人がマージを確認するのは次の3つの場合だけであること。REQ-core-365 の線に当たった。人の判断を待つ指摘が、出どころ（整合のフェーズ、レビュー、人が残すと決めた指摘）を問わず、開いたまま残っている。整合のフェーズが、承認済みの要求を、矛盾か再現で誤りだと示したうえで書き改めた。どの場合も、cycle はプルリクエストを作り、自動のマージは頼まず、開いたままにして、プルリクエストの場所と、止まった原因（当たったパス、開いている指摘、または書き改めた要求）を人に見せること。変更の要約は付けないこと。プルリクエストを作れないホスティングでは、枝を push したところで止まり、原因を最後の報告で見せること。3つ目の場合、整合のフェーズは作業を止めず、控えるのはマージの頼みだけであること。実装役が、承認済みの要求を変えるか消す必要があると返したときは、工程のいまある止まる条件に従って人が出て、何もマージしないこと。戻せない操作（リリース、公開、タグ、手元の外にあるものを消すこと）は、いまある止まる決まりのもとで人に残ること。承認のあとに整合のフェーズが仕様や判断の記録に書き足した具体化は、止まる理由にしないこと

kotowari-cycle は常に、線に当たったとき、人の判断を待つ`指摘`が残ったとき、または`整合のフェーズ`が承認済みの`要求`を書き改めたとき、マージを頼まずにプルリクエストを開いたままにして、プルリクエストと止まった原因を人に見せ、承認済みの`要求`を変えるか消す必要があると実装役が返したときは、マージせずに、工程のいまある止まる条件で止まる。

### REQ-core-368: 人を待たないもの

- kind: ubiquitous
- source: docs/decision/records/2026-11-24-unattended-merge.md#A5, docs/decision/records/2026-11-24-unattended-merge.md#A10, docs/decision/records/2026-11-24-unattended-merge.md#A11, docs/decision/records/2026-11-24-unattended-merge.md#A15, docs/decision/records/2026-11-24-unattended-merge.md#A17, docs/decision/records/2026-11-24-unattended-merge.md#A25, docs/decision/records/2026-11-24-unattended-merge.md#A41
- verification: review
- how_to_verify: "agent/skills/kotowari-cycle/"、"agent/skills/kotowari-implement/" と "agent/skills/kotowari-review/" を読み、次のすべてが書いてあることを確かめる。レビューの指摘のうち、回の収束を止めるのは、走らせて示せる間違いだけであること。REQ-core-367 の止まり方は、これとは別であること。テストを消す変更、検査の抑制を足す変更、依存を足す・変える変更を、それを理由に人に回さないこと。読みやすさと構造の選び方は実装役が決め、止まる理由にも、レビューの席を足す理由にもしないこと。実装役が自分で下した判断は、足した依存も含めて、何を決めたか・なぜか・証拠を持つ1行として書き、その行をプルリクエストの本文と、同じものを cycle の最後の報告に載せること。走らせて示せない意図とのずれは、仕様の穴として、壁打ちの側の出戻りに数えること

kotowari-cycle は常に、レビューの指摘のうち走らせて示せる間違いだけを回の収束を止める理由にし、実装役が自分で下した判断を、何を・なぜ・証拠の1行ずつでプルリクエストの本文と最後の報告に残す。

### REQ-core-369: 回の記録が持つもの

- kind: ubiquitous
- source: docs/decision/records/2026-11-24-unattended-merge.md#A6, docs/decision/records/2026-11-24-unattended-merge.md#A13, docs/decision/records/2026-11-24-unattended-merge.md#A14, docs/decision/records/2026-11-24-unattended-merge.md#A30, docs/decision/records/2026-11-24-unattended-merge.md#A31, docs/decision/records/2026-11-24-unattended-merge.md#A40, docs/decision/records/2026-11-24-unattended-merge.md#A43, docs/decision/records/2026-11-24-unattended-merge.md#A45, docs/decision/records/2026-11-24-unattended-merge.md#A47
- verification: review
- how_to_verify: "agent/skills/kotowari-cycle/" と "agent/skills/kotowari-review/" の指摘の形の資料を読み、次のすべてが書いてあることを確かめる。cycle は新しい記録のファイルを作らず、枝ごとにすでに書いている指摘のファイルに3つを足すこと。線の結果と当たったパス。この依頼が手直しする前の依頼の枝の名前（依頼が前の依頼の手直しだと言っているときだけ書き、そうでなければ空のままにする）。どの終わり方で終わったかとその理由（前の工程への差し戻しと、人を呼んで止まったことを含む）。費用は記録しないこと。指摘のファイルはコミットせず、片付ける作業用のディレクトリの外に置き、工程は消さないこと。マージで終わった回の指摘のファイルは、あとで同じ名前の枝で始めた回の再開としては読まないこと。cycle が片付けるのは、自分でマージを見たときだけであること。指摘のファイルが作業用のディレクトリの外にあることを確かめてから、手元の作業用のディレクトリと手元の枝を消し、作業用のディレクトリに実装役の作業メモのほかにコミットしていない変更があれば、消さずに報告し、ホスティングのサービスの側のものは消さないこと。人があとでマージした回は、cycle は片付けないこと

kotowari-cycle は常に、枝ごとにすでに書いている指摘のファイルに、線の結果、手直しの元の依頼、終わり方とその理由を残し、自分で見たマージのあとは手元の作業用のディレクトリと手元の枝だけを消す。
