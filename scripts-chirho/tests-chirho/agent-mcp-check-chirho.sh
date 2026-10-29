#!/usr/bin/env bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
#
# Quick check of the MCP adapter a customer runs (tools-chirho/agent-mcp-chirho): strict TypeScript
# over the adapter with Node's own module rules and types (it must run under node as well as bun),
# strict TypeScript over its test side with Bun's types, then its unit tests against a fake
# workspace. No engine, no network beyond loopback. The served proof is arm 8b of
# agent-api-rehearsal-chirho.sh. The TypeScript compiler comes from the repo's dev toolchain
# (tools-chirho/browser-toolchain-chirho; install there first); the adapter imports nothing from it.
set -uo pipefail
REPO_CHIRHO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$REPO_CHIRHO" || exit 2
MODULES_CHIRHO=tools-chirho/browser-toolchain-chirho/node_modules
[ -f "$MODULES_CHIRHO/typescript/bin/tsc" ] || { printf 'refusing: the dev toolchain is not installed (see README.md)\n' >&2; exit 2; }
DIR_CHIRHO=tools-chirho/agent-mcp-chirho
STRICT_CHIRHO=(bun "$MODULES_CHIRHO/typescript/bin/tsc" --noEmit --strict --noUnusedLocals --noUnusedParameters --noImplicitReturns --target ESNext)
FAILS_CHIRHO=0

"${STRICT_CHIRHO[@]}" --lib ESNext --module NodeNext --moduleResolution NodeNext --types node --typeRoots "$MODULES_CHIRHO/@types" \
  "$DIR_CHIRHO/perffection-mcp-chirho.ts" || FAILS_CHIRHO=$((FAILS_CHIRHO + 1))
printf 'strict types, adapter (node rules): %s\n' "$([ "$FAILS_CHIRHO" -eq 0 ] && echo ok || echo FAIL)"

"${STRICT_CHIRHO[@]}" --lib ESNext,DOM --module Preserve --moduleResolution Bundler --types bun-types,node --typeRoots "$MODULES_CHIRHO/@types,$MODULES_CHIRHO" \
  "$DIR_CHIRHO/perffection-mcp-chirho.test.ts" "$DIR_CHIRHO/perffection-mcp-saves-chirho.test.ts" "$DIR_CHIRHO/perffection-mcp-rate-limits-chirho.test.ts" \
  "$DIR_CHIRHO/mcp-stdio-client-chirho.ts" scripts-chirho/tests-chirho/agent-mcp-journey-chirho.ts \
  scripts-chirho/tests-chirho/agent-mcp-ended-key-chirho.ts scripts-chirho/tests-chirho/figures-chirho/drop-answer-proxy-chirho.ts \
  scripts-chirho/tests-chirho/figures-chirho/figures-mcp-read-chirho.ts scripts-chirho/tests-chirho/figures-chirho/figures-upload-browser-chirho.ts \
  scripts-chirho/tests-chirho/figures-chirho/held-body-client-chirho.ts scripts-chirho/tests-chirho/figures-chirho/mcp-call-chirho.ts \
  scripts-chirho/tests-chirho/account-chirho/account-browser-chirho.ts scripts-chirho/tests-chirho/account-chirho/account-create-browser-chirho.ts \
  scripts-chirho/tests-chirho/account-chirho/back-check-chirho.ts scripts-chirho/tests-chirho/account-chirho/back-check-chirho.test.ts
TEST_TYPES_RC_CHIRHO=$?; [ "$TEST_TYPES_RC_CHIRHO" -eq 0 ] || FAILS_CHIRHO=$((FAILS_CHIRHO + 1))
printf 'strict types, tests, client and gate drivers (bun rules): %s\n' "$([ "$TEST_TYPES_RC_CHIRHO" -eq 0 ] && echo ok || echo FAIL)"

# A path (./...), never a name filter: a filter makes bun walk the whole tree for test files.
bun test "./$DIR_CHIRHO/"; TESTS_RC_CHIRHO=$?
[ "$TESTS_RC_CHIRHO" -eq 0 ] || FAILS_CHIRHO=$((FAILS_CHIRHO + 1))
printf 'VERDICT: %s failed step(s)\n' "$FAILS_CHIRHO"
[ "$FAILS_CHIRHO" -eq 0 ]
