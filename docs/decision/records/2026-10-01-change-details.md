# 変更照合の具体的な契約

## Context

変更照合の方針は承認されたが、形式・入力・合否とスキルの責任境界が未決だった。
実装役が未決を補ってから記録する状況を避けるため、6領域を実装前の仕様として具体化する。
この記録の細部は文書作成者の推奨案であり、利用者による採用は今回のレビューで確認する。
既存の方針は [変更照合の記録](./2026-10-01-change-conformance.md#Agreements) を継続する。

Position: A1〜A14 は利用者がレビューで採用し、IR とともに e2159ac にコミットした。A15 は一覧表記の補足で、計画のレビューで採用を確認する。

## Agreements

- A1 changes は "--base REV"、"--head REV" と "--staged" のどちらか1つ、"--phase implementation" または "--phase review" を必須とする。既定の比較対象・段階は置かない。--staged は --base HEAD と implementation の組み合わせだけを受ける。既存の --config・--format・--help・--version は維持し、専用オプションを他のコマンドに渡すと引数の誤りで停止する。--help と --version の優先規則は既存と同じとする。
  - why: 手元の検査と取り込み前の検査で、暗黙の比較元や段階によって保証が弱まる経路を減らす。
  - decided_by: 文書作成者（推奨案、利用者の採用はレビューで確認）
- A2 設定に省略可能な changes を加え、その鍵は files、exclude、records とする。各値は基準のディレクトリからの相対 glob の文字列一覧で、exclude の既定は空。changes を書く場合は files と records を明示し、どちらも空にしない。未知の鍵、null、空文字、不正な glob は設定の誤りで停止する。changes の省略時は check と status の照合記録検査を行わず、changes コマンドは設定の誤りで停止する。glob の構文は既存の tests.files と同じとする。
  - why: 既存プロジェクトに段階的に導入できるようにしつつ、専用コマンドが未設定を成功扱いしないようにする。
  - decided_by: 文書作成者（推奨案、利用者の採用はレビューで確認）
- A3 changes は従来の設定探索の代わりに、起動位置から Git の作業ツリーのルートを求め、そのルートを基準のディレクトリとする。比較元・対象の REV は commit に解決し、照合記録の base は解決した比較元の完全な object ID と一致させる。--head の対象はその commit の tree、--staged は index の内容とし、対象ファイル・関連 IR・判断の記録・照合記録・設定ファイルをすべて同じ対象から読む。changes の --config は Git のルートからの相対パスとし、対象に存在する設定ファイルだけを受ける。省略時は対象の ".kotowari/config.yaml" を読む。changes に限り、設定のパスをカレントディレクトリから解釈する既存規則と作業ツリーでの存在確認をこの規則で置き換える。起動位置は Git のルートまたはその下とし、設定に書く全パスの基準は Git のルートとする。作業ツリーの未コミット内容で補わない。Git が無い、履歴・対象・設定を読めない、競合した index は停止する。check と status は従来の作業ツリーを読む。
  - why: コードだけステージ済みでも根拠が手元にしか存在しない状態を通さない。CI でも検査された内容を再現する。
  - decided_by: 文書作成者（推奨案、利用者の採用はレビューで確認）
- A4 Git の比較元と対象の tree の差を、rename 検出なしで追加・変更・削除として列挙する。移動は旧パスの削除と新パスの追加で、種類変更も差として扱う。files に当たり exclude に当たらないパスを検査対象とする。Git に含まれる隠しディレクトリのパスも glob が当たれば含める。未追跡・ステージされていない変更は対象外。対象に選ばれた symlink、submodule、UTF-8 でないパスは対応しない入力として停止する。ファイルの実行権の変更も対象変更に含め、識別値には Git の mode と blob の全バイトを含める。
  - why: rename の推測に左右されない対応と、内容が同じでも実行権が変わる変更の検出を得る。初版の入力境界を狭く明示する。
  - decided_by: 文書作成者（推奨案、利用者の採用はレビューで確認）
- A5 records に当たる照合記録は対象変更の列挙から除く。IR の置き場、decisions.records と decisions.adr の置き場、使用する設定ファイルも照合対象の列挙から除く。関連 IR と判断の記録は参照の検査で扱い、IR は鮮度も検査する。files と exclude の変更による対象範囲の縮小は review が確認する。対象変更が0件なら changes は成功できるが、設定された照合記録の形式・参照の検査は行う。
  - why: 照合記録の追加が新たな照合対象になる循環を避ける。設定範囲自体の妥当性を機械が保証したと誤認しない。
  - decided_by: 文書作成者（推奨案、利用者の採用はレビューで確認）
- A6 照合記録は UTF-8 の YAML とし、1ファイルは version: 1 と entries の一覧だけを持つ。entries は空でもよい。未知の版、未知の鍵、重複した YAML の鍵、誤った型は change_record_invalid の誤りとする。各件の鍵は id、base、role、state、files、ir、conclusion、reason、requirements、decisions、handoff、gaps の12個だけとし、すべて必須とする。id は "[A-Za-z0-9][A-Za-z0-9._-]*" に合う文字列、reason は空白だけでない文字列、base は Git の完全な object ID（40桁または64桁の小文字16進）、role は implementer または reviewer、state は active または archived、conclusion は existing、new または deferred とする。id は読む照合記録全体で一意とする。files は1件以上、ir・requirements・decisions・gaps は0件以上の一覧、handoff は null または判断の記録への参照文字列とする。保存場所は changes.records が指定し、版の異なる記録を自動変換しない。
  - why: 既存の YAML 読み取り基盤を再利用し、手書きと LLM の両方で読める保存契約を作る。欠落と意図的な空の一覧を区別する。
  - decided_by: 文書作成者（推奨案、利用者の採用はレビューで確認）
- A7 files の各件は path、before、after の3鍵だけを持つ。path は基準からの正規化した相対パス、before と after は null または "sha256:" と64桁の小文字16進で、両方 null にしない。追加は before を null、削除は after を null とし、変更は両方に識別値を持つ。識別値は Git の6文字の mode、NUL 1バイト、blob の全バイトを順に連結した値の SHA-256 とする。ir の各件は path と sha256 の2鍵で、sha256 は IR ファイルの全バイトだけを SHA-256 で計算した同じ表記とする。path は空、絶対パス、..、.、バックスラッシュを含む成分を認めない。files と ir の各一覧内の重複パスは誤りとする。
  - why: 既存のガイド用の短い指紋とは別に、Git の対象内容を改行や出典も含めて固定する。before の検査で過去の別の変更を流用できないようにする。
  - decided_by: 文書作成者（推奨案、利用者の採用はレビューで確認）
- A8 requirements は既存の要求 ID の文字列一覧、decisions と handoff は既存の出典と同じリポジトリ内の "パス#決定番号" の表記とする。existing は requirements を1件以上、ir を1件以上持ち、参照要求を定義する IR のすべてを ir に含める。new は decisions を1件以上持ち、仕様を変更した場合の関連 IR への反映を review が確認する。deferred は decisions を1件以上と null でない handoff を持つ。判断の記録・要求・IR の存在と参照の整合を check で検査する。根拠と保留の意味は review が確認する。
  - why: 既存仕様の参照と新判断の根拠を追跡可能にし、機械が文章の意味を判定したという契約を作らない。
  - decided_by: 文書作成者（推奨案、利用者の採用はレビューで確認）
- A9 gaps の各件は category、disposition、refs の3鍵だけを持つ。category は missing_spec、spec_conflict、premise_conflict のいずれかで、未分類の値は誤りとする。disposition は recorded、fixed、deferred のいずれかで、refs は1件以上の判断の記録への参照を持つ。recorded は反映した選択と根拠を参照し、fixed は修正の判断と既存要求への対応を記録し、deferred は明示的な保留を参照する。gaps がある件の conclusion は、deferred が1件でもあれば deferred、それがなく recorded が1件でもあれば new、それ以外は existing とする。fixed を含む件は conclusion が new でも対応する要求とその定義 IR を持つ。recorded と fixed は同じ件に共存できる。各処理先が選択を支えることと、recorded の関連 IR への反映は review が確認する。
  - why: info や record_only のまま完了になる経路を、結論と処理先の構造からも検出する。
  - decided_by: 文書作成者（推奨案、利用者の採用はレビューで確認）
- A10 implementation 段階では各ファイルの同じ比較元と before・after に対応する active な implementer の件を、review 段階では active な implementer と reviewer の両方の件を要求する。coverage の単位は各ファイルで、両役のファイルのまとめ方と件の id は一致しなくてもよい。reviewer の関連 IR の集合は対応する implementer の件の関連 IR をすべて含め、追加の IR も含められる。必要な関連 IR を含まない reviewer の件はそのファイルの review の coverage を満たさず、change_uncovered を出す。結論 deferred は implementation 段階で形式・参照が揃えば通せるが、review 段階では change_deferred の誤りとして通さない。同じ変更が複数件に現れること自体は許すが、同じ base とファイルの before・after の組に対応する active な件に異なる結論があれば change_conclusion_conflict の誤りとする。照合漏れと covered の集計はファイル単位、鮮度の検査は件単位とし、対象変更を含む active な件の対象内のファイルか関連 IR が1つでも不一致ならその件の全ファイルを covered に数えない。保留を正式に採用する場合は判断と IR にその許容を明示して既存仕様または新判断として再照合する。記録の role は独立性の証明ではなく、別の review の実行はスキルが保証する。
  - why: 途中のコミットは進められるが、未決の仕様を残したまま取り込む条件は初版に持ち込まない。
  - decided_by: 文書作成者（推奨案、利用者の採用はレビューで確認）
- A11 changes は形式の検査を全照合記録に行い、参照の存在・整合の検査は active な件だけに行う。coverage と鮮度は active で指定された base と一致する件についてだけ検査する。check と status も形式は全件、参照の存在・整合は active な件だけを検査する。archived な件は現在の参照切れを誤りにせず、coverage の根拠にも使わない。完了した比較の記録は呼び出し側が archived に更新してリポジトリに保持する。base が異なる履歴の件を古さの誤りとしては出さない。対象変更に対応する件が無いときは change_uncovered、before・after の不一致は change_stale、IR の不一致は change_ir_stale とする。比較元に一致する記録が対象外になったファイルを持つ場合、そのファイルの coverage と鮮度は検査しない。
  - why: 完了した過去のレビューの保存が、次のブランチを永続的に止めることを避ける。
  - decided_by: 文書作成者（推奨案、利用者の採用はレビューで確認）
- A12 changes の JSON は base、target、phase、files、covered、findings の6鍵を持つ。base は解決した完全な object ID、target は commit の object ID または "index"、phase は指定値、files は列挙した対象変更数、covered はその段階の記録と鮮度が揃い結論が適合する変更数。findings の形と text の1件の表記は check と同じとする。形式・結論・必須情報・参照の静的な不整合は change_record_invalid とする。今回の新しい指摘はすべて severity error、path は照合記録の相対パス、line は null、detail は関連する件の id と対象パスを示す。change_record_invalid では取得できない id やパスを要求せず、ファイル全体の不正は "file: " と説明、件の不正は "entry " と entries 内の0始まりの位置と ": " と説明を出す。id を取得できるときだけ説明に含める。ただし change_uncovered は対象ファイルのパス、line null、detail は必要な role とする。指摘は path、kind、detail のバイト順で並べる。誤り0件で終了0、誤りありで終了1、実行や入力の停止で終了2。Git の読み取りの停止理由は "git error" とする。check と status は記録の静的検査の誤りを既存の findings に算入し、新しい最上位集計鍵を加えない。status の complete は差分の最終照合を保証しない。
  - why: 既存の指摘の形を使い、CI は終了コードで停止できるようにする。専用コマンドの対象と段階は出力でも確認できる。
  - decided_by: 文書作成者（推奨案、利用者の採用はレビューで確認）
- A13 implement と fixer は、承認済み要求の制約を変えない委譲範囲内の具体化に限り、根拠と判断者を記録して IR に追加できる。承認済み要求を変更・削除する判断、既存の選択と矛盾する追加、根拠から決められない判断は人へ戻す。cycle 自身は実装や意味の判断を行わず、実装側へ記録を委譲し別の review で確認する。実装側の IR 追加後は check と、その追加を含む仕様・実装の照合を必ず再実行する。review の findings JSON は継続して内部用とし、実装側と review 側は共通の YAML 照合記録を別々に作成する。
  - why: 承認済みの要求を勝手に変えることと、根拠のある具体化を区別する。既存の cycle の IR-side findings の扱いはこの条件の範囲だけ改める。
  - decided_by: 文書作成者（推奨案、利用者の採用はレビューで確認）
- A14 導入例では changes.files に製品コード・テスト・配布スキル・ビルドとフックと CI の設定を列挙し、changes.records は "docs/changes/**/*.yaml" とする。生成物は明示した exclude だけで外す。pre-commit は整形後の index に対して "changes --base HEAD --staged --phase implementation" を行い、check も別に行う。CI は pull_request の比較元を base SHA と head SHA の merge-base、対象を head SHA とし、両履歴を取得して "changes --base <比較元> --head <対象> --phase review" と check を行う。merge 用の SHA は対象に使わない。push の導入例はイベントの before と after の比較とし、before が全0の新規ブランチは停止して比較元を明示する。途中のコミット用には HEAD を base とした実装側の件を作り、最終検査用にはブランチの比較元を base とした実装側の件と review 側の件を作り直す。再照合は変わったファイルまたは IR を含む件ごとに行い、同じ件の全ファイルと関連 IR を再確認する。
  - why: 既存の lefthook を再利用し、GitHub のイベントの比較元を呼び出し側が指定する。配布スキルの変更にも記録を求める。
  - decided_by: 文書作成者（推奨案、利用者の採用はレビューで確認）

- A15 コマンドが無い場合の停止詳細は "expected command: check, changes, list, mutants, plan, query or status" とする。既存の一覧表記を維持し、changes をコマンド名のバイト順の位置に加える。
  - why: コマンド名の一覧は決まったが、細かな表記に旧 FLAG の未決参照が残っていた。実装役が表記を判断する余地を残さない
  - decided_by: 文書作成者（補足の推奨案、利用者の採用は計画のレビューで確認）

## Reuse

| 層 | 採用するものと理由 |
|---|---|
| Git の比較と snapshot | Git の plumbing コマンド。既存の diff/tree/index を使い、独自の履歴解析を避ける。具体的な呼び出しとバイト境界は実装計画で整理する |
| YAML・JSON | 既存依存 serde-saphyr・serde_json。記録の version と未知の鍵の拒否を契約にする |
| glob と SHA-256 | 既存依存 globset・sha2。ガイド用の fingerprint の切り詰め処理は使わない |
| 参照・指摘・集計 | 既存の sources、finding、status の処理。変更照合の純粋な判定だけ追加する |
| フックと CI | 既存 lefthook と GitHub Actions の導入例。LLM の判断や文書生成はフックへ埋め込まない |

## Revisions

- 利用者の「gogo」を受けて未決の具体案を作成した。各細部の採用を先取りせず、推奨案として記録した。
