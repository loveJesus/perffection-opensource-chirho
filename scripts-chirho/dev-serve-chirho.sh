#!/usr/bin/env bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
#
# Fast visual-iteration loop: regenerate, emit, deploy to a disposable root, serve.
#
# This is the DEV path and it says so: it deploys with --allow-unreviewed-dev, which skips the
# exact-hash review promotion. It is for looking at the page in a browser while designing.
# The maintained reviewed product journey is tools-chirho/browser-chirho/reviewed-product-check-chirho.ts.
# Neither this visual loop nor that local review check is hosted release acceptance.
#
# Workflow: spec-chirho/workflows-chirho/product-delivery-chirho/product-authoring-chirho.md

set -euo pipefail

APP_ROOT_CHIRHO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$APP_ROOT_CHIRHO/scripts-chirho/lib-chirho/prebuilt-app-chirho.sh"
source "$APP_ROOT_CHIRHO/scripts-chirho/lib-chirho/cairn-runtime-chirho.sh"
CAIRN_REPO_CHIRHO="${CAIRN_REPO_CHIRHO:-$(cd "$APP_ROOT_CHIRHO/../cairn-opensource-chirho" && pwd)}"
CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$APP_ROOT_CHIRHO/target}"
export CARGO_TARGET_DIR
PORT_CHIRHO="${PORT_CHIRHO:-8841}"
COMMAND_CHIRHO="${1:-serve-chirho}"
READY_TIMEOUT_CHIRHO="${READY_TIMEOUT_CHIRHO:-240}"
DEV_ROOT_CHIRHO=""
SERVE_PID_CHIRHO=""

# A BOUNDED readiness probe, for callers that start this script in the background.
#
# An unbounded `until curl ...; do sleep; done` around a backgrounded serve is a trap: if the child
# dies at build or emit, the waiter spins forever and reads as healthy progress. This exits non-zero
# on timeout and prints the tail of the log so the actual failure is the thing you see.
await_ready_chirho() {
  local log_chirho="${DEV_LOG_CHIRHO:-}"
  local waited_chirho=0
  while (( waited_chirho < READY_TIMEOUT_CHIRHO )); do
    if curl -fsS -o /dev/null "http://127.0.0.1:${PORT_CHIRHO}/app-chirho/perffection/" 2>/dev/null; then
      printf 'ready on port %s after %ss\n' "$PORT_CHIRHO" "$waited_chirho"
      return 0
    fi
    sleep 2
    waited_chirho=$((waited_chirho + 2))
  done
  printf 'dev serve did not become ready on port %s within %ss\n' "$PORT_CHIRHO" "$READY_TIMEOUT_CHIRHO" >&2
  if [[ -n "$log_chirho" && -f "$log_chirho" ]]; then
    printf -- '--- last 20 lines of %s ---\n' "$log_chirho" >&2
    tail -20 "$log_chirho" >&2
  fi
  return 1
}

if [[ "$COMMAND_CHIRHO" == "await-chirho" ]]; then
  await_ready_chirho
  exit $?
fi

CAIRN_CLI_CHIRHO="$(resolve_cairn_runtime_chirho "$APP_ROOT_CHIRHO" "$CAIRN_REPO_CHIRHO")"

cleanup_chirho() {
  # Own the serve child, so signalling this script never leaves a server holding the port after its
  # root has been deleted. See the same note in score-show-hn-demo-chirho.sh.
  if [[ -n "$SERVE_PID_CHIRHO" ]] && kill -0 "$SERVE_PID_CHIRHO" 2>/dev/null; then
    kill "$SERVE_PID_CHIRHO" 2>/dev/null || true
    wait "$SERVE_PID_CHIRHO" 2>/dev/null || true
  fi
  if [[ -n "$DEV_ROOT_CHIRHO" && "$DEV_ROOT_CHIRHO" == "$CARGO_TARGET_DIR"/perffection-dev-root-chirho.* ]]; then
    rm -rf -- "$DEV_ROOT_CHIRHO"
  fi
}
trap cleanup_chirho EXIT INT TERM

cd "$APP_ROOT_CHIRHO"

"$APP_ROOT_CHIRHO/scripts-chirho/prepare-apps-chirho.sh" --app-chirho site

mkdir -p "$CARGO_TARGET_DIR"
DEV_ROOT_CHIRHO="$(mktemp -d "$CARGO_TARGET_DIR/perffection-dev-root-chirho.XXXXXX")"
STAGED_MANIFEST_CHIRHO="$(stage_prebuilt_app_chirho "$CAIRN_REPO_CHIRHO" "$APP_ROOT_CHIRHO" \
  perffection "$DEV_ROOT_CHIRHO/prebuilt-app-chirho")"
OWNER_OUTPUT_CHIRHO="$("$CAIRN_CLI_CHIRHO" --root "$DEV_ROOT_CHIRHO" identity-register --kind owner)"
OWNER_CHIRHO="${OWNER_OUTPUT_CHIRHO%%$'\t'*}"

printf '  deploy (unreviewed dev)\n'
"$CAIRN_CLI_CHIRHO" --root "$DEV_ROOT_CHIRHO" \
  app-source-deploy "$STAGED_MANIFEST_CHIRHO" \
  --owner "$OWNER_CHIRHO" --allow-unreviewed-dev > /dev/null

printf '\n  http://127.0.0.1:%s/app-chirho/perffection/\n\n' "$PORT_CHIRHO"
"$CAIRN_CLI_CHIRHO" --root "$DEV_ROOT_CHIRHO" serve --port "$PORT_CHIRHO" --allow-platform-root-chirho &
SERVE_PID_CHIRHO=$!
wait "$SERVE_PID_CHIRHO"
