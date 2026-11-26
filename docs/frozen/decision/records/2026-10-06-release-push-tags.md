# リリースのブランチの push でタグを一緒に出さない

## Context

`scripts/release.sh` はリリースのコミットに注釈つきタグを作り、まずコミットをリリースのブランチに push して PR のチェックを通し、通ってから main とタグを出す手順を案内している。push する人の git の設定で `push.followTags` が有効だと、ブランチの push でタグも一緒に出てしまい、この順が守られない。設定によらず順を守れるようにする。

## Agreements

- A1 `scripts/release.sh` が案内するリリースのブランチと main の push のコマンドに `--no-follow-tags` を付け、PROJECT.md のリリースの手順も同じにする。タグは最後の `git push origin <tag>` でだけ出す。これはリリースの手順で、利用者から見える製品の振る舞いではないので IR に書かない。
  - why: `push.followTags` が有効な環境では、注釈つきタグを指すコミットの push でタグも出て、チェックの前に release ワークフローが動く
  - rejected: タグを軽量タグにする（release.sh は注釈つきタグを前提にしている）、タグを作るのを push の後に回す（release.sh の「1つのコミットとタグを一度に作る」を変える大きな変更になる）
  - decided_by: 利用者
