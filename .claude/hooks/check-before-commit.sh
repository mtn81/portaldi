#!/usr/bin/env bash
# PreToolUse hook: block `git commit` unless `make lint` and `make test-all` pass.
set -uo pipefail

cmd=$(jq -r '.tool_input.command // empty')
grep -qE '(^|[;&|(]|[[:space:]])git[[:space:]]+commit([[:space:]]|$)' <<<"$cmd" || exit 0

cd "${CLAUDE_PROJECT_DIR:-.}" || exit 2

log=$(mktemp)
trap 'rm -f "$log"' EXIT

for target in lint test-all; do
  if ! make "$target" >"$log" 2>&1; then
    echo "make $target failed, so the commit was blocked. Last lines of the output:" >&2
    tail -n 40 "$log" >&2
    exit 2
  fi
done
