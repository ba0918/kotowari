#!/usr/bin/env bash
set -euo pipefail
if [ "$#" -ne 2 ]; then
  printf '%s\n' 'event error: expected event kind and JSON file' >&2
  exit 2
fi
kind=$1
event=$2
case "$kind" in
  pull_request)
    event_base=$(jq -er '.pull_request.base.sha' "$event") || exit 2
    event_head=$(jq -er '.pull_request.head.sha' "$event") || exit 2
    ;;
  push)
    event_base=$(jq -er '.before' "$event") || exit 2
    event_head=$(jq -er '.after' "$event") || exit 2
    if [[ "$event_base" =~ ^0+$ ]]; then
      printf '%s\n' 'event error: new branch needs an explicit comparison base' >&2
      exit 2
    fi
    ;;
  *) printf '%s\n' 'event error: unsupported event kind' >&2; exit 2 ;;
esac
for object in "$event_base" "$event_head"; do
  if ! [[ "$object" =~ ^([0-9a-f]{40}|[0-9a-f]{64})$ ]]; then
    printf '%s\n' 'event error: expected full lowercase commit IDs' >&2
    exit 2
  fi
  git rev-parse --verify --end-of-options "$object^{commit}" >/dev/null || exit 2
done
if [ "$kind" = pull_request ]; then
  comparison_base=$(git merge-base "$event_base" "$event_head") || exit 2
else
  comparison_base=$event_base
fi
printf 'base=%s\nhead=%s\n' "$comparison_base" "$event_head"
