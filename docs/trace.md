# 検証が review の要求の確かめ方

この表は、`docs/ir` の要求のうち `- 検証: review` のものについて、人が確かめる手順を記す。unit と property の要求は、テストの印（`tests/*.rs` の `// @kotowari[ID]`）と `kotowari check` の requirement_without_test が対応を持つので、この表には載せない。テスト名は `rg 'req_085' tests/` のように ID で引ける。

## 要求の確かめ方

| REQ | 種類 | 確かめ方 |
|---|---|---|
| REQ-041 | review | `src/ir.rs` で scope_lines の中身を検査せず存在だけ確認。`check_documents` に範囲の内容検査がないことを確認 |
| REQ-084 | review | `src/tests_discovery.rs` に正規表現によるテスト検出がないことを確認。tree-sitter のみ使用 |
| REQ-089 | review | `src/ir.rs` にコードで固定した形で IR を読むことを確認。parse_document 関数 |
| REQ-092 | review | 判断の記録と ADR の両方のディレクトリを読むことを `src/sources.rs` で確認 |
| REQ-093 | review | 判断の記録の検査は出典の存在確認のみで、ファイルの上書きはしない |
| REQ-094 | review | ADR の節の検査は出典のための見出し照合のみ。5節の構造検査はしない |
| REQ-095 | review | ADR の決定の節の検査は出典のための見出し照合のみ |
| REQ-096 | review | kotowari は ADR だけの運用を禁止する検査をしない（設定に両方のパスが必要） |
| REQ-097 | review | `- 出典:` の行、`@source=` のタグ、用語集の出典の列に現れる `docs/decision/adr/` のファイルを列挙し（`rg -o 'docs/decision/adr/[^ ,|]+' docs/ir` の出典の行だけ）、そのファイルがすべて存在することを確認。2026-09-17 時点で 0002 と 0003 |
| REQ-101 | review | CLI の出力は JSON/text で LLM が読みやすい形。`src/main.rs` を確認 |
| REQ-103 | review | ADR のファイル名形式は出典の検査時に見るが、形式自体は検査しない |
| REQ-105 | review | `src/` にライブラリとバイナリの2ターゲット。モジュールは config, ir, sources, terms, tests_discovery |
| REQ-108 | review | Linux と macOS で `cargo test` が通ることを確認。Windows は動作を約束しない（パスの区切りの正規化 REQ-110 だけ） |
| REQ-109 | review | `src/lib.rs` の `run_check` で置き場の存在を検査し、`StopReason` で停止していることを確認。黙って飛ばす経路が無いことを `rg 'filter_map|if let Ok' src/` で確認 |
| REQ-120 | review | `src/ir.rs` と `src/lib.rs` で、仕様に列挙されていない振る舞いを黙って決めていないことを確認。`parse_document` の gherkin 解析で有効な行の種類以外を invalid_gherkin_line にし、`read_utf8_file` で読めないファイルを停止にし、`check_documents` で形に合わない見出しの下を読まないことを確認 |
| REQ-136 | review | 判断の記録を読む関数が1つで（`src/sources.rs` の parse_records_file、または記録の読み取りのモジュール）、形の検査と出典の判定がその返す構造だけを読むことを確認。`rg "lines\(\)" src/sources.rs` で、記録のファイルの行を直接読む箇所が読み取り関数の外に無いことを確認 |
