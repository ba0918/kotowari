# 計画: 入力を読んで結果を出す

## Goal

利用者が入力のファイルを渡すと、結果が標準出力に出る。

## Specification

- `docs/ir/core/a.md#REQ-core-001`

## Approach and why

入力を読む関数を先に作り、出力はその後に足す。

## Scope of change

- `src/lib.rs`

## Step order and prerequisites

S1 だけ。

## Verification map

S1 が REQ-core-001 を確かめる。

## Left to the implementer

関数の名前。

## Stop conditions

- 入力の形が仕様から決まらないとき

## Test command

`cargo test`

## Out of scope

- 出力の書式を変えること

## Steps

### S1: 入力を読む

- Purpose: 入力のファイルを読む
- Specification: `docs/ir/core/a.md#REQ-core-001`
- Prerequisites: なし
- May change: `src/lib.rs`
- Done when: 入力のファイルを読んだ結果が返る
- Shown by: test — REQ-core-001
- Left to the implementer: なし
- Stop and hand back if: 入力の形が仕様から決まらないとき
