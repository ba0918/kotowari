# 変更照合の入力と対象

変更照合の具体的な入力、記録または合否を定める。具体的な契約と判断の根拠は出典の記録で追う。

## Requirements

### REQ-core-263: 比較対象と段階

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-details.md#A1
- verification: unit

変更照合の検査は常に、次の契約を満たす。changes は "--base REV"、"--head REV" と "--staged" のどちらか1つ、"--phase implementation" または "--phase review" を必須とする。既定の比較対象・段階は置かない。--staged は --base HEAD と implementation の組み合わせだけを受ける。既存の --config・--format・--help・--version は維持し、専用オプションを他のコマンドに渡すと引数の誤りで停止する。--help と --version の優先規則は既存と同じとする。

### REQ-core-264: 設定

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-details.md#A2
- verification: unit

変更照合の検査は常に、次の契約を満たす。設定に省略可能な changes を加え、その鍵は files、exclude、records とする。各値は基準のディレクトリからの相対 glob の文字列一覧で、exclude の既定は空。changes を書く場合は files と records を明示し、どちらも空にしない。未知の鍵、null、空文字、不正な glob は設定の誤りで停止する。changes の省略時は check と status の照合記録検査を行わず、changes コマンドは設定の誤りで停止する。glob の構文は既存の tests.files と同じとする。

### REQ-core-265: Git の読み取り境界

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-details.md#A3
- verification: unit

変更照合の検査は常に、次の契約を満たす。changes は従来の設定探索の代わりに、起動位置から Git の作業ツリーのルートを求め、そのルートを基準のディレクトリとする。比較元・対象の REV は commit に解決し、照合記録の base は解決した比較元の完全な object ID と一致させる。--head の対象はその commit の tree、--staged は index の内容とし、対象ファイル・関連 IR・判断の記録・照合記録・設定ファイルをすべて同じ対象から読む。changes の --config は Git のルートからの相対パスとし、対象に存在する設定ファイルだけを受ける。省略時は対象の ".kotowari/config.yaml" を読む。changes に限り、設定のパスをカレントディレクトリから解釈する既存規則と作業ツリーでの存在確認をこの規則で置き換える。起動位置は Git のルートまたはその下とし、設定に書く全パスの基準は Git のルートとする。作業ツリーの未コミット内容で補わない。Git が無い、履歴・対象・設定を読めない、競合した index は停止する。check と status は従来の作業ツリーを読む。

### REQ-core-266: 対象列挙

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-details.md#A4
- verification: unit

変更照合の検査は常に、次の契約を満たす。Git の比較元と対象の tree の差を、rename 検出なしで追加・変更・削除として列挙する。移動は旧パスの削除と新パスの追加で、種類変更も差として扱う。files に当たり exclude に当たらないパスを検査対象とする。Git に含まれる隠しディレクトリのパスも glob が当たれば含める。未追跡・ステージされていない変更は対象外。対象に選ばれた symlink、submodule、UTF-8 でないパスは対応しない入力として停止する。ファイルの実行権の変更も対象変更に含め、識別値には Git の mode と blob の全バイトを含める。

### REQ-core-267: 記録自身と対象外

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-details.md#A5
- verification: unit

変更照合の検査は常に、次の契約を満たす。records に当たる照合記録は対象変更の列挙から除く。IR の置き場、decisions.records と decisions.adr の置き場、使用する設定ファイルも照合対象の列挙から除く。関連 IR と判断の記録は参照の検査で扱い、IR は鮮度も検査する。files と exclude の変更による対象範囲の縮小は review が確認する。対象変更が0件なら changes は成功できるが、設定された照合記録の形式・参照の検査は行う。

## Examples

```gherkin
@id=EX-core-441 @about=REQ-core-263 @source=docs/decision/records/2026-10-01-change-details.md#A1,docs/decision/records/2026-10-01-change-details.md#A12,docs/decision/records/2026-10-01-change-details.md#A11
Scenario: 段階の指定がない
  Given 比較元と対象だけが指定されている
  When changes を実行する
  Then 引数の誤りで終了2となる

@id=EX-core-442 @about=REQ-core-264 @source=docs/decision/records/2026-10-01-change-details.md#A2,docs/decision/records/2026-10-01-change-details.md#A12,docs/decision/records/2026-10-01-change-details.md#A11
Scenario: 未導入プロジェクト
  Given 設定に changes がない
  When changes を実行する
  Then 設定の誤りで終了2となる

@id=EX-core-443 @about=REQ-core-265 @source=docs/decision/records/2026-10-01-change-details.md#A3,docs/decision/records/2026-10-01-change-details.md#A12,docs/decision/records/2026-10-01-change-details.md#A11
Scenario: 未ステージの記録で補わない
  Given index に対象変更があり、照合記録は作業ツリーにだけ存在する
  When ステージ内容の changes を実行する
  Then 作業ツリーの記録で照合済みとしない

@id=EX-core-444 @about=REQ-core-266 @source=docs/decision/records/2026-10-01-change-details.md#A4,docs/decision/records/2026-10-01-change-details.md#A12,docs/decision/records/2026-10-01-change-details.md#A11
Scenario: 移動は削除と追加
  Given 対象内の通常ファイルが別パスへ移動している
  When changes を実行する
  Then 旧パスの削除と新パスの追加を対象にする

@id=EX-core-445 @about=REQ-core-267 @source=docs/decision/records/2026-10-01-change-details.md#A5,docs/decision/records/2026-10-01-change-details.md#A12,docs/decision/records/2026-10-01-change-details.md#A11
Scenario: 記録自身の循環を避ける
  Given 設定された記録のファイルだけが増え、その形式と参照は正しい
  When changes を実行する
  Then 対象変更0件として成功する
```
