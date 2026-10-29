#!/usr/bin/env bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
#
# Limited keys over her page and her figures (Cairn K1, landed at c0a12269; F6c, gpt #26625), synthetic
# and local. The routes stay HELD out of app-chirho.toml: this gate binds the figures routes and writes
# the key openings (apps-chirho/perffection-workspace-chirho/deploy_chirho/key-openings-chirho.toml) into a
# scratch mirror of the workspace app, deployed three times to one fresh tenant root:
#   none: nothing opened, so an old all-powerful key works as before;
#   page: the page routes open (her app opts in): the old key is retired everywhere and no new one
#         issues; a page key reads her page and nothing else. On Cairn's K4 that first opening is refused while
#         the legacy client exists (a named control, never retirement), and the matrix below runs on a fresh
#         root where the app opens before its client is registered (gpt #26876);
#   page,figures: her figures' read and Save open as two separate actions on business-figures-chirho.
#         The page key minted before never joins them; a read key never saves; a Save key saves, is read
#         back, meets a stale version, survives a lost answer (re-read, then an exact repeat), and two
#         owners stay apart; her page and her figures stay apart; her browser sees what her assistant saved;
#         an owner with nothing saved is shown an illustrative sample that is never saved as hers (F6d).
# Saved state is always read back; nothing is inferred from request or use records. Every refusal's
# exact status, headers and body are kept in refusals-chirho/, as the adapter receives them (Accept:
# application/json) and once as plain text, for the MCP adapter's recognition of K1's fixed answers.
# Needs a K1 engine and tree-build (Cairn's hash-bound handoff, #26634); no host, pin or customer key.
# Env: CAIRN_CLI_CHIRHO + CANDIDATE_CAIRN_COMMIT_CHIRHO (+ CANDIDATE_CAIRN_SHA256_CHIRHO), TREE_BUILD_CHIRHO,
# CAIRN_REPO_CHIRHO, TENANT_PORT_CHIRHO (default 19300; the drop proxy takes the next port).
# Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/her-figures-chirho.md
set -uo pipefail; umask 077
HERE_CHIRHO="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_CHIRHO="$(cd "$HERE_CHIRHO/../../.." && pwd)"
CAIRN_REPO_CHIRHO="${CAIRN_REPO_CHIRHO:-$REPO_CHIRHO/../cairn-opensource-chirho}"
# shellcheck source=../../lib-chirho/cairn-runtime-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/cairn-runtime-chirho.sh"
# shellcheck source=../../lib-chirho/served-root-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/served-root-chirho.sh"
# shellcheck source=../../lib-chirho/emit-manifest-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/emit-manifest-chirho.sh"
# shellcheck source=../../lib-chirho/prebuilt-app-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/prebuilt-app-chirho.sh"
CLI_CHIRHO="$(resolve_cairn_runtime_chirho "$REPO_CHIRHO" "$CAIRN_REPO_CHIRHO")" || exit 2
TREE_BUILD_CHIRHO="${TREE_BUILD_CHIRHO:-$CAIRN_REPO_CHIRHO/target/debug/tree-build-chirho}"
[ -x "$TREE_BUILD_CHIRHO" ] || { printf 'refusing: no tree-build at %s\n' "$TREE_BUILD_CHIRHO" >&2; exit 2; }
LC_ALL=C grep -a -q "the app root cannot be opened to limited keys" "$TREE_BUILD_CHIRHO" \
  || { printf 'refusing: %s is not a K1 tree-build (it cannot lower key openings)\n' "$TREE_BUILD_CHIRHO" >&2; exit 2; }
APP_CHIRHO="$REPO_CHIRHO/apps-chirho/perffection-workspace-chirho"
if grep -q -E 'figures-chirho|key_access_chirho' "$APP_CHIRHO/app-chirho.toml"; then
  printf 'refusing: app-chirho.toml already routes figures or opens a key route; this gate does that only in scratch\n' >&2; exit 2
fi
TENANT_CHIRHO=beta-chirho; TP_CHIRHO="${TENANT_PORT_CHIRHO:-19300}"; PP_CHIRHO=$((TP_CHIRHO + 1))
require_port_free_chirho "$TP_CHIRHO"; require_port_free_chirho "$PP_CHIRHO"
mkdir -p "$REPO_CHIRHO/target/rehearsal-chirho"
T_CHIRHO="$(mktemp -d "$REPO_CHIRHO/target/rehearsal-chirho/key-grants.XXXX")"; ROOT_CHIRHO="$T_CHIRHO/root-chirho"
mkdir -p "$ROOT_CHIRHO" "$T_CHIRHO/refusals-chirho"
TB_CHIRHO="http://127.0.0.1:$TP_CHIRHO"; WS_CHIRHO="$TB_CHIRHO/app-chirho/perffection-workspace"
LOG_CHIRHO="$T_CHIRHO/journey-chirho.log"; : > "$LOG_CHIRHO"
# shellcheck source=../../lib-chirho/gate-helpers-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/gate-helpers-chirho.sh"
# shellcheck source=../../lib-chirho/key-staging-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/key-staging-chirho.sh"
cli_chirho() { "$CLI_CHIRHO" --root "$ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" "$@"; }
say_chirho "rehearsal dir $T_CHIRHO; engine $("$CLI_CHIRHO" --version 2>/dev/null | grep -oE '[0-9a-f]{40}' | head -1 | cut -c1-12); tree-build $(shasum -a 256 "$TREE_BUILD_CHIRHO" | cut -c1-12); product $(git -C "$REPO_CHIRHO" rev-parse --short=12 HEAD 2>/dev/null)"

SCRATCH_CHIRHO="$T_CHIRHO/workspace-scratch-chirho"
TPID_CHIRHO=""; PPID_CHIRHO=""
stop_chirho() { [ -n "$TPID_CHIRHO" ] && kill "$TPID_CHIRHO" 2>/dev/null; [ -n "$PPID_CHIRHO" ] && kill "$PPID_CHIRHO" 2>/dev/null; wait 2>/dev/null; TPID_CHIRHO=""; PPID_CHIRHO=""; }
trap 'stop_chirho; rm -rf "$SCRATCH_CHIRHO"; rm -f "$T_CHIRHO"/*-cred-chirho.txt "$T_CHIRHO"/*-key-chirho.txt' EXIT
# start_chirho <stage>: the engine (invoked directly, so $! is its own pid) and the drop proxy
start_chirho() {
  "$CLI_CHIRHO" --root "$ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" serve --port "$TP_CHIRHO" >> "$T_CHIRHO/tenant-serve.log" 2>&1 & TPID_CHIRHO=$!
  bun "$HERE_CHIRHO/drop-answer-proxy-chirho.ts" "$PP_CHIRHO" "$TP_CHIRHO" /app-chirho/perffection-workspace/agent-save-figures-chirho >> "$T_CHIRHO/proxy.log" 2>&1 & PPID_CHIRHO=$!
  for _ in $(seq 1 80); do curl -s -o /dev/null "$TB_CHIRHO/auth-chirho/login-chirho" && break; sleep 0.5; done
  expect_chirho "stage $1: the server on $TP_CHIRHO is this run's" "$(own_server_verdict_chirho "$TPID_CHIRHO" "$TP_CHIRHO")" own
}
save_body_chirho() { python3 "$HERE_CHIRHO/figures-save-body-chirho.py" "$T_CHIRHO/$1" "$2" "$3" "$4"; }
# mcp_chirho <key name> <tool> <arguments file or ""> <out> [origin]: one call through her own agent's adapter
mcp_chirho() { bun "$HERE_CHIRHO/mcp-call-chirho.ts" "${5:-$TB_CHIRHO}" "$T_CHIRHO/$1-key-chirho.txt" "$2" "${3:+$T_CHIRHO/$3}" "$T_CHIRHO/$4" > "$T_CHIRHO/$4.log" 2>&1; }
mcp_field_chirho() { json_chirho "$1" "d[\"structured_chirho\"]$2"; }

# ---- the root: owner, two users, one OAuth client ----
OWNER_CHIRHO="$(cli_chirho identity-register --kind owner | cut -f1)"
USER_A_CHIRHO="$(new_user_chirho a)"; USER_B_CHIRHO="$(new_user_chirho b)"; USER_C_CHIRHO="$(new_user_chirho c)"
KIND_A_CHIRHO="Sourdough$(openssl rand -hex 4)"; KIND_B_CHIRHO="Plumbing$(openssl rand -hex 4)"; KIND_C_CHIRHO="Bookbinding$(openssl rand -hex 4)"

# ---- 1. nothing opened: an old all-powerful key works as before ----
deploy_stage_chirho none; expect_chirho "stage none: the scratch app deployed with no openings" "$?" 0
CLIENT_CHIRHO="$(register_client_chirho client-register.out)"; expect_chirho "OAuth client registered: an id read from its own answer" "$(printf '%s' "$CLIENT_CHIRHO" | grep -cE '^client/[0-9a-f]{16}$')" 1
legacy_key_chirho old-a "$USER_A_CHIRHO" a; expect_chirho "an old all-powerful key issued before any opening" "$?" 0
start_chirho none
expect_chirho "nothing opened: the old key reads her page" "$(call_chirho old-a GET agent-page-chirho stage0-page.json)" 200
expect_chirho "nothing opened: the old key reads her figures" "$(call_chirho old-a GET agent-figures-chirho stage0-figures.json)" 200
stop_chirho
# the control: deployed again with nothing opened, the old key still works, so what retires it next is
# her opt-in, not a redeploy
deploy_stage_chirho none; expect_chirho "stage none again: redeployed with no openings" "$?" 0
start_chirho none-again
expect_chirho "redeployed with nothing opened: the old key still reads her page" "$(call_chirho old-a GET agent-page-chirho stage0b-page.json)" 200
stop_chirho

# ---- 2. the page routes open: her app opts in ----
# From Cairn's K4 (ad4bc794) an app's FIRST opening is refused while it has a client in the legacy store, which an
# older engine would still mint unrestricted keys for (refuse_initial_opening_with_legacy_client_under_append_lock_chirho,
# gpt #26876). There the K1 retirement story cannot run: the refusal is a named control and never proof of retirement,
# and the grant matrix runs on a fresh root where the app opens first and its client is a connection client. The
# retirement of an old key at opt-in stays proved on the K1-era tools it belongs to.
if LC_ALL=C grep -a -q "OpeningWithLegacyClientChirho" "$CLI_CHIRHO"; then
  deploy_stage_chirho page; OPENED_RC_CHIRHO=$?
  expect_chirho "K4: her first opening, with a legacy client, is refused: refused/the reason" "$([ "$OPENED_RC_CHIRHO" -ne 0 ] && echo 1 || echo 0)/$(grep -c 'OpeningWithLegacyClientChirho' "$T_CHIRHO/deploy-$DEPLOYS_CHIRHO-page.out")" "1/1"
  start_chirho page-refused
  expect_chirho "K4: the refused opening retired nothing (it is not retirement): the old key still reads her page" "$(call_chirho old-a GET agent-page-chirho stage1-refused-page.json)" 200
  stop_chirho
  ROOT_CHIRHO="$T_CHIRHO/root-opened-first-chirho"; mkdir -p "$ROOT_CHIRHO"
  OWNER_CHIRHO="$(cli_chirho identity-register --kind owner | cut -f1)"
  USER_A_CHIRHO="$(new_user_chirho a)"; USER_B_CHIRHO="$(new_user_chirho b)"; USER_C_CHIRHO="$(new_user_chirho c)"
  deploy_stage_chirho page; expect_chirho "K4, a fresh root: the page routes opened before any client" "$?" 0
  CLIENT_CHIRHO="$(register_client_chirho client-register-opened-chirho.out)"; expect_chirho "K4, a fresh root: the app's client, registered after the opening: an id read from its own answer" "$(printf '%s' "$CLIENT_CHIRHO" | grep -cE '^client/[0-9a-f]{16}$')" 1
  legacy_key_chirho old-a-again "$USER_A_CHIRHO" a
  expect_chirho "K4: no all-powerful key issues for the opened app: refused/the reason" "$?/$(grep -c 'this app accepts only limited keys' "$T_CHIRHO/issue-old-a-again.out")" "1/1"
  say_chirho "note K4: no old key exists on the fresh root; retirement at opt-in is proved on K1-era tools only"
  RETIREMENT_CHIRHO=k1-era-only
else
  deploy_stage_chirho page; expect_chirho "stage page: the page routes opened" "$?" 0
  legacy_key_chirho old-a-again "$USER_A_CHIRHO" a
  expect_chirho "no new all-powerful key issues once she opted in: refused/the reason" "$?/$(grep -c 'this app accepts only limited keys' "$T_CHIRHO/issue-old-a-again.out")" "1/1"
  RETIREMENT_CHIRHO=here
fi
limited_key_chirho page-a "$USER_A_CHIRHO" a page-read-chirho page-save-chirho page-preview-chirho; expect_chirho "a page key issued (page actions only)" "$?" 0
start_chirho page
if [ "$RETIREMENT_CHIRHO" = here ]; then
  # a retired key is no key at all: the general refusal, not a route's
  expect_chirho "the old key, retired at opt-in, on her page" "$(refusal_chirho old-a GET agent-page-chirho legacy-retired-page)" "$(k1_json_chirho 403 request-not-authorized-chirho)"
  expect_chirho "the old key, retired at opt-in, on her figures" "$(refusal_chirho old-a GET agent-figures-chirho legacy-retired-figures)" "$(k1_json_chirho 403 request-not-authorized-chirho)"
fi
expect_chirho "the page key reads her page: status/state" "$(call_chirho page-a GET agent-page-chirho stage1-page.json)/$(json_chirho stage1-page.json 'd["state_chirho"]')" "200/nothing-saved-chirho"
expect_chirho "the page key on her figures read, not opened yet" "$(refusal_chirho page-a GET agent-figures-chirho page-key-figures-not-open)" "$(k1_json_chirho 403 key-route-not-open-chirho)"
stop_chirho

# ---- 3. her figures open: read and Save, two actions on business-figures-chirho ----
deploy_stage_chirho page,figures; expect_chirho "stage page,figures: her figures' read and Save opened" "$?" 0
limited_key_chirho read-a "$USER_A_CHIRHO" a figures-read-chirho; expect_chirho "a figures read key issued" "$?" 0
limited_key_chirho save-a "$USER_A_CHIRHO" a figures-read-chirho figures-save-chirho; expect_chirho "a figures read+Save key issued for A" "$?" 0
limited_key_chirho save-b "$USER_B_CHIRHO" b figures-read-chirho figures-save-chirho; expect_chirho "a figures read+Save key issued for B" "$?" 0
limited_key_chirho save-c "$USER_C_CHIRHO" c figures-read-chirho figures-save-chirho; expect_chirho "a figures read+Save key issued for C" "$?" 0
start_chirho page,figures
save_body_chirho body-a1.json 0 "$KIND_A_CHIRHO" 130000
# the page key minted before her figures opened never joins them, and keeps her page
expect_chirho "the earlier page key on her figures read" "$(refusal_chirho page-a GET agent-figures-chirho page-key-figures-read)" "$(k1_json_chirho 403 key-action-not-granted-chirho)"
expect_chirho "the earlier page key on her figures Save" "$(refusal_chirho page-a POST agent-save-figures-chirho page-key-figures-save body-a1.json)" "$(k1_json_chirho 403 key-action-not-granted-chirho)"
expect_chirho "the earlier page key still reads her page" "$(call_chirho page-a GET agent-page-chirho stage2-page.json)" 200
# a read key reads, and never saves or reaches her page
expect_chirho "the read key reads her figures: status/state/version" "$(call_chirho read-a GET agent-figures-chirho read-0.json)/$(json_chirho read-0.json 'd["state_chirho"]')/$(json_chirho read-0.json 'd["current_version_chirho"]')" "200/nothing-saved-chirho/0"
expect_chirho "the read key on her figures Save" "$(refusal_chirho read-a POST agent-save-figures-chirho read-key-save body-a1.json)" "$(k1_json_chirho 403 key-action-not-granted-chirho)"
expect_chirho "the read key on her figures Save, asked as plain text" "$(refusal_chirho read-a POST agent-save-figures-chirho read-key-save-plain body-a1.json '*/*')" "403|text/plain; charset=utf-8|error: key-action-not-granted-chirho~"
expect_chirho "the read key on her page" "$(refusal_chirho read-a GET agent-page-chirho read-key-page)" "$(k1_json_chirho 403 key-action-not-granted-chirho)"
expect_chirho "after the refused Saves nothing is saved: version" "$(call_chirho read-a GET agent-figures-chirho read-0b.json > /dev/null; json_chirho read-0b.json 'd["current_version_chirho"]')" 0
# the Save key saves, and the read key reads it back
expect_chirho "the Save key saves her figures: status/schema/saved/version/her kind" "$(call_chirho save-a POST agent-save-figures-chirho save-a1.json body-a1.json)/$(json_chirho save-a1.json 'd["schema_chirho"]')/$(json_chirho save-a1.json 'd["saved_chirho"]')/$(json_chirho save-a1.json 'd["current_version_chirho"]')/$(json_chirho save-a1.json 'd["figures_chirho"]["kinds_chirho"][0]')" "200/perffection-agent-figures-save-v1-chirho/true/1/$KIND_A_CHIRHO"
expect_chirho "read back with the read key: version/her kind/February" "$(call_chirho read-a GET agent-figures-chirho read-1.json > /dev/null; json_chirho read-1.json 'd["current_version_chirho"]')/$(json_chirho read-1.json 'd["figures_chirho"]["kinds_chirho"][0]')/$(json_chirho read-1.json 'd["figures_chirho"]["money_in_chirho"][0][1]')" "1/$KIND_A_CHIRHO/130000"
# the very same Save again (against 0, the same figures) is answered as saved again, with no second version
expect_chirho "the same Save sent again: status/saved/version" "$(call_chirho save-a POST agent-save-figures-chirho again-a1.json body-a1.json)/$(json_chirho again-a1.json 'd["saved_chirho"]')/$(json_chirho again-a1.json 'd["current_version_chirho"]')" "200/true/1"
# other figures against the version before hers: refused as stale, the head unchanged
save_body_chirho body-a1-other.json 0 "$KIND_A_CHIRHO" 140000
expect_chirho "a stale Save: status/saved/refusal/head" "$(call_chirho save-a POST agent-save-figures-chirho stale-a.json body-a1-other.json)/$(json_chirho stale-a.json 'd["saved_chirho"]')/$(json_chirho stale-a.json 'd["refusal_chirho"]')/$(json_chirho stale-a.json 'd["current_version_chirho"]')" "200/false/stale-version-chirho/1"
expect_chirho "after both, one version: head/February" "$(call_chirho read-a GET agent-figures-chirho read-1s.json > /dev/null; json_chirho read-1s.json 'd["current_version_chirho"]')/$(json_chirho read-1s.json 'd["figures_chirho"]["money_in_chirho"][0][1]')" "1/130000"
# two owners apart: B's own figures, never A's
save_body_chirho body-b1.json 0 "$KIND_B_CHIRHO" 99900
expect_chirho "B reads nothing of A's: state/A's kind" "$(call_chirho save-b GET agent-figures-chirho b-read-0.json > /dev/null; json_chirho b-read-0.json 'd["state_chirho"]')/$(count_chirho "$KIND_A_CHIRHO" b-read-0.json)" "nothing-saved-chirho/0"
expect_chirho "B saves B's own figures: saved/version" "$(call_chirho save-b POST agent-save-figures-chirho b-save-1.json body-b1.json > /dev/null; json_chirho b-save-1.json 'd["saved_chirho"]')/$(json_chirho b-save-1.json 'd["current_version_chirho"]')" "true/1"
expect_chirho "A's head is still A's: version/kinds/B's kind anywhere" "$(call_chirho read-a GET agent-figures-chirho read-1b.json > /dev/null; json_chirho read-1b.json 'd["current_version_chirho"]')/$(json_chirho read-1b.json 'd["figures_chirho"]["kinds_chirho"]')/$(count_chirho "$KIND_B_CHIRHO" read-1b.json)" "1/[\"$KIND_A_CHIRHO\"]/0"
# her page and her figures stay apart, both ways
expect_chirho "her page after her figures were saved: status/state" "$(call_chirho page-a GET agent-page-chirho stage2-page-b.json)/$(json_chirho stage2-page-b.json 'd["state_chirho"]')" "200/nothing-saved-chirho"
python3 - "$T_CHIRHO/stage2-page-b.json" "$T_CHIRHO/page-body-a1.json" <<'PY'
import json, sys
read = json.load(open(sys.argv[1], encoding="utf-8"))
json.dump({"expected_version_chirho": 0, "page_chirho": read["surface_chirho"]["sample_page_chirho"]}, open(sys.argv[2], "w", encoding="utf-8"))
PY
expect_chirho "the page key saves her page: status/saved/version" "$(call_chirho page-a POST agent-save-page-chirho page-save-a1.json page-body-a1.json)/$(json_chirho page-save-a1.json 'd["saved_chirho"]')/$(json_chirho page-save-a1.json 'd["current_version_chirho"]')" "200/true/1"
expect_chirho "her figures after her page was saved: version/her kind" "$(call_chirho read-a GET agent-figures-chirho read-1c.json > /dev/null; json_chirho read-1c.json 'd["current_version_chirho"]')/$(json_chirho read-1c.json 'd["figures_chirho"]["kinds_chirho"][0]')" "1/$KIND_A_CHIRHO"
# a lost answer: the Save reaches the engine, she never hears; the re-read tells the truth; an exact repeat
save_body_chirho body-a2.json 1 "$KIND_A_CHIRHO" 150000
expect_chirho "a Save through the drop proxy got no answer: http code" "$(call_chirho save-a POST agent-save-figures-chirho lost-a2.json body-a2.json "http://127.0.0.1:$PP_CHIRHO/app-chirho/perffection-workspace")" 000
expect_chirho "the proxy dropped it after the engine's whole answer: after/before" "$(grep -c '^dropped-after-whole-answer-chirho$' "$T_CHIRHO/proxy.log")/$(grep -c '^dropped-before-whole-answer-chirho$' "$T_CHIRHO/proxy.log")" "1/0"
expect_chirho "the re-read tells the truth: version/February" "$(call_chirho read-a GET agent-figures-chirho read-2.json > /dev/null; json_chirho read-2.json 'd["current_version_chirho"]')/$(json_chirho read-2.json 'd["figures_chirho"]["money_in_chirho"][0][1]')" "2/150000"
expect_chirho "the exact repeat: status/saved/refusal/version" "$(call_chirho save-a POST agent-save-figures-chirho repeat-a2.json body-a2.json)/$(json_chirho repeat-a2.json 'd["saved_chirho"]')/$(json_chirho repeat-a2.json 'd["refusal_chirho"]')/$(json_chirho repeat-a2.json 'd["current_version_chirho"]')" "200/true/null/2"
expect_chirho "after the repeat, still one new version: head" "$(call_chirho read-a GET agent-figures-chirho read-2b.json > /dev/null; json_chirho read-2b.json 'd["current_version_chirho"]')" 2
# her own agent, through the MCP adapter with the same keys: a read; Saves its key was never granted, told as
# refused before they ran; a Save; a stale one; one whose answer is lost, then the truth and a deliberate repeat
save_body_chirho body-m3.json 2 "$KIND_A_CHIRHO" 160000; save_body_chirho body-m-stale.json 2 "$KIND_A_CHIRHO" 165000; save_body_chirho body-m4.json 3 "$KIND_A_CHIRHO" 180000
mcp_chirho read-a read_figures_chirho "" mcp-read-2.json
expect_chirho "MCP: the read key reads her figures: offered/error/state/version" "$(json_chirho mcp-read-2.json 'd["offered_chirho"]')/$(json_chirho mcp-read-2.json 'd["is_error_chirho"]')/$(mcp_field_chirho mcp-read-2.json '["state_chirho"]')/$(mcp_field_chirho mcp-read-2.json '["current_version_chirho"]')" "true/false/saved-chirho/2"
mcp_chirho read-a save_figures_chirho body-m3.json mcp-read-key-save.json
expect_chirho "MCP: the read key's Save, refused before it ran: result/identity/status" "$(mcp_field_chirho mcp-read-key-save.json '["save_result_chirho"]')/$(mcp_field_chirho mcp-read-key-save.json '["refusal_identity_chirho"]')/$(mcp_field_chirho mcp-read-key-save.json '["http_status_chirho"]')" "refused-before-run-chirho/key-action-not-granted-chirho/403"
mcp_chirho page-a save_figures_chirho body-m3.json mcp-page-key-save.json
expect_chirho "MCP: the page key's figures Save, refused before it ran: result/identity" "$(mcp_field_chirho mcp-page-key-save.json '["save_result_chirho"]')/$(mcp_field_chirho mcp-page-key-save.json '["refusal_identity_chirho"]')" "refused-before-run-chirho/key-action-not-granted-chirho"
expect_chirho "MCP: after both refusals, the head: version" "$(call_chirho read-a GET agent-figures-chirho read-2c.json > /dev/null; json_chirho read-2c.json 'd["current_version_chirho"]')" 2
mcp_chirho save-a save_figures_chirho body-m3.json mcp-save-3.json
expect_chirho "MCP: the Save key saves her figures: error/saved/version/February" "$(json_chirho mcp-save-3.json 'd["is_error_chirho"]')/$(mcp_field_chirho mcp-save-3.json '["saved_chirho"]')/$(mcp_field_chirho mcp-save-3.json '["current_version_chirho"]')/$(mcp_field_chirho mcp-save-3.json '["figures_chirho"]["money_in_chirho"][0][1]')" "false/true/3/160000"
mcp_chirho save-a save_figures_chirho body-m-stale.json mcp-stale.json
expect_chirho "MCP: a stale Save: error/refusal/head" "$(json_chirho mcp-stale.json 'd["is_error_chirho"]')/$(mcp_field_chirho mcp-stale.json '["refusal_chirho"]')/$(mcp_field_chirho mcp-stale.json '["current_version_chirho"]')" "true/stale-version-chirho/3"
mcp_chirho save-a save_figures_chirho body-m4.json mcp-lost-4.json "http://127.0.0.1:$PP_CHIRHO"
expect_chirho "MCP: a Save whose answer is lost is unknown: result/status" "$(mcp_field_chirho mcp-lost-4.json '["save_result_chirho"]')/$(mcp_field_chirho mcp-lost-4.json '["http_status_chirho"]')" "unknown-chirho/null"
expect_chirho "the proxy dropped both lost Saves, once each, after the engine's whole answer: after/before" "$(grep -c '^dropped-after-whole-answer-chirho$' "$T_CHIRHO/proxy.log")/$(grep -c '^dropped-before-whole-answer-chirho$' "$T_CHIRHO/proxy.log")" "2/0"
mcp_chirho read-a read_figures_chirho "" mcp-read-4.json
expect_chirho "MCP: the re-read tells the truth: version/February" "$(mcp_field_chirho mcp-read-4.json '["current_version_chirho"]')/$(mcp_field_chirho mcp-read-4.json '["figures_chirho"]["money_in_chirho"][0][1]')" "4/180000"
mcp_chirho save-a save_figures_chirho body-m4.json mcp-repeat-4.json
expect_chirho "MCP: the deliberate exact repeat: saved/version/head" "$(mcp_field_chirho mcp-repeat-4.json '["saved_chirho"]')/$(mcp_field_chirho mcp-repeat-4.json '["current_version_chirho"]')/$(call_chirho read-a GET agent-figures-chirho read-4b.json > /dev/null; json_chirho read-4b.json 'd["current_version_chirho"]')" "true/4/4"
# C's first figures (F6d): nothing saved, so her assistant is shown an illustration of the shape; the
# illustration itself is never saved as hers; C's own figures, put in that shape, are her first save
mcp_chirho save-c read_figures_chirho "" mcp-c-read-0.json
expect_chirho "MCP: C has nothing saved: state/version/first save at/the sample is a figures document" "$(mcp_field_chirho mcp-c-read-0.json '["state_chirho"]')/$(mcp_field_chirho mcp-c-read-0.json '["current_version_chirho"]')/$(mcp_field_chirho mcp-c-read-0.json '["start_chirho"]["first_save_expected_version_chirho"]')/$(mcp_field_chirho mcp-c-read-0.json '["start_chirho"]["sample_figures_chirho"]["schema_chirho"]')" "nothing-saved-chirho/0/0/perffection-figures-v1-chirho"
python3 - "$T_CHIRHO/mcp-c-read-0.json" "$T_CHIRHO/body-c-sample.json" "$T_CHIRHO/body-c1.json" "$KIND_C_CHIRHO" <<'PY'
import copy, json, sys
read, sample_out, own_out, kind = sys.argv[1:5]
start = json.load(open(read, encoding="utf-8"))["structured_chirho"]["start_chirho"]
sample = start["sample_figures_chirho"]
json.dump({"expected_version_chirho": start["first_save_expected_version_chirho"], "figures_chirho": sample}, open(sample_out, "w", encoding="utf-8"))
own = copy.deepcopy(sample)  # the sample's shape, with C's own synthetic values in every place
own.update({"note_chirho": "C's own synthetic figures", "kinds_chirho": [kind, "Repairs"], "money_in_chirho": [[210000, 225000], [61000, 64000]], "costs_chirho": [[90000, 95000], [None, None]], "jobs_chirho": [[30, 33], [None, None]]})
json.dump({"expected_version_chirho": start["first_save_expected_version_chirho"], "figures_chirho": own}, open(own_out, "w", encoding="utf-8"))
PY
mcp_chirho save-c save_figures_chirho body-c-sample.json mcp-c-sample.json
expect_chirho "MCP: the sample saved as C's is refused, and nothing is written: refusal/head" "$(mcp_field_chirho mcp-c-sample.json '["refusal_chirho"]')/$(call_chirho save-c GET agent-figures-chirho c-read-0b.json > /dev/null; json_chirho c-read-0b.json 'd["current_version_chirho"]')" "sample-not-her-figures-chirho/0"
mcp_chirho save-c save_figures_chirho body-c1.json mcp-c-save-1.json
expect_chirho "MCP: C's own first figures saved: error/saved/version" "$(json_chirho mcp-c-save-1.json 'd["is_error_chirho"]')/$(mcp_field_chirho mcp-c-save-1.json '["saved_chirho"]')/$(mcp_field_chirho mcp-c-save-1.json '["current_version_chirho"]')" "false/true/1"
mcp_chirho save-c read_figures_chirho "" mcp-c-read-1.json
expect_chirho "MCP: C's read-back: state/version/C's kind/February/the start gone" "$(mcp_field_chirho mcp-c-read-1.json '["state_chirho"]')/$(mcp_field_chirho mcp-c-read-1.json '["current_version_chirho"]')/$(mcp_field_chirho mcp-c-read-1.json '["figures_chirho"]["kinds_chirho"][0]')/$(mcp_field_chirho mcp-c-read-1.json '["figures_chirho"]["money_in_chirho"][0][1]')/$(json_chirho mcp-c-read-1.json '"start_chirho" not in d["structured_chirho"]')" "saved-chirho/1/$KIND_C_CHIRHO/225000/true"
expect_chirho "A's head is untouched by C: version" "$(call_chirho read-a GET agent-figures-chirho read-4c.json > /dev/null; json_chirho read-4c.json 'd["current_version_chirho"]')" 4
# her browser, signed in, sees what her assistant saved: one store, keys or no keys
expect_chirho "her browser signs in" "$(sign_in_chirho "$USER_A_CHIRHO" a login-a.hdr)" 303
expect_chirho "her figures page: status/version/her kind" "$(curl -s -o "$T_CHIRHO/browser-figures.html" -w '%{http_code}' -H "Cookie: $(session_cookie_chirho login-a.hdr)" "$WS_CHIRHO/figures-chirho")/$(grep -oE 'name="expected_version_chirho" value="[0-9]+"' "$T_CHIRHO/browser-figures.html" | head -1 | grep -oE '[0-9]+')/$(count_chirho "<th scope=\"row\">$KIND_A_CHIRHO</th>" browser-figures.html)" "200/4/1"
stop_chirho

# ---- 4. her read route can suddenly do more (scratch only): the key granted to read it refuses ----
# The read route keeps its name and its figures-read opening, but runs the Save's module, so it can write;
# a figures-read key consented to reading only.
deploy_stage_chirho page,figures agent-figures-chirho=agent-save-figures-chirho; expect_chirho "stage escalated: her read route moved onto the Save's module" "$?" 0
start_chirho escalated
expect_chirho "the read key on her read route that can now write" "$(refusal_chirho read-a GET agent-figures-chirho read-key-escalated)" "$(k1_json_chirho 403 key-action-authority-changed-chirho)"
expect_chirho "the same, asked as plain text" "$(refusal_chirho read-a GET agent-figures-chirho read-key-escalated-plain '' '*/*')" "403|text/plain; charset=utf-8|error: key-action-authority-changed-chirho~"

# ---- 5. a Save admitted, then her app redeployed before its body arrived (scratch only) ----
# K1 admits a key on a signed-in-only app before reading the body, without recording a use. This Save's
# body is held open while the Save route moves onto the read module. Measured: the engine checks the key
# again when it runs the request, against the route as it now is, and refuses it there (403
# authority-changed: the read module needs what figures-save never consented to). K1's 409
# key-route-changed needs a redeploy between that check and the route's selection, a window only Cairn's
# hold point reaches (a_redeploy_between_the_gate_and_selection_refuses_with_409_chirho), not this gate.
# against the head, so it would have saved had it run
save_body_chirho body-a3.json 4 "$KIND_A_CHIRHO" 170000
HELD_ANSWER_CHIRHO="$T_CHIRHO/refusals-chirho/held-save-redeployed"; rm -f "$T_CHIRHO/release-chirho"
bun "$HERE_CHIRHO/held-body-client-chirho.ts" "$TP_CHIRHO" /app-chirho/perffection-workspace/agent-save-figures-chirho "$T_CHIRHO/save-a-key-chirho.txt" \
  "$T_CHIRHO/body-a3.json" "$T_CHIRHO/release-chirho" "$HELD_ANSWER_CHIRHO" > "$T_CHIRHO/held-client.log" 2>&1 & HELD_CHIRHO=$!
for _ in $(seq 1 40); do grep -q '^held$' "$T_CHIRHO/held-client.log" && break; sleep 0.25; done
expect_chirho "her Save's head and half its body were sent" "$(grep -c '^held$' "$T_CHIRHO/held-client.log")" 1
deploy_stage_chirho page,figures agent-save-figures-chirho=agent-figures-chirho; expect_chirho "stage moved, while her Save was held: the Save route moved onto the read module" "$?" 0
touch "$T_CHIRHO/release-chirho"; wait "$HELD_CHIRHO"
expect_chirho "the held Save, after the redeploy" "$(head -1 "$HELD_ANSWER_CHIRHO")|$(tail -n +3 "$HELD_ANSWER_CHIRHO" | tr '\n' '~')" 'HTTP/1.1 403 Forbidden|{"error_chirho":"key-action-authority-changed-chirho","ok_chirho":false}'
# her read route is back on the read module, so the read key reads again: the running server took the redeploy
expect_chirho "the held Save stored nothing: status/head/February" "$(call_chirho read-a GET agent-figures-chirho read-3.json)/$(json_chirho read-3.json 'd["current_version_chirho"]')/$(json_chirho read-3.json 'd["figures_chirho"]["money_in_chirho"][0][1]')" "200/4/180000"
expect_chirho "a fresh Save on the moved route" "$(refusal_chirho save-a POST agent-save-figures-chirho save-key-moved body-a3.json)" "$(k1_json_chirho 403 key-action-authority-changed-chirho)"
mcp_chirho save-a save_figures_chirho body-a3.json mcp-moved-save.json
expect_chirho "MCP: a Save on the moved route, refused before it ran: result/identity/status" "$(mcp_field_chirho mcp-moved-save.json '["save_result_chirho"]')/$(mcp_field_chirho mcp-moved-save.json '["refusal_identity_chirho"]')/$(mcp_field_chirho mcp-moved-save.json '["http_status_chirho"]')" "refused-before-run-chirho/key-action-authority-changed-chirho/403"
expect_chirho "nothing stored by any of them: head" "$(call_chirho read-a GET agent-figures-chirho read-3b.json > /dev/null; json_chirho read-3b.json 'd["current_version_chirho"]')" 4
stop_chirho
expect_chirho "MCP: no run wrote its key, each read one, each scan finds a planted one: runs" "$(cat "$T_CHIRHO"/mcp-*.json | grep -c '"key_read_chirho":true,"key_written_chirho":false,"scan_finds_planted_key_chirho":true')/$(ls "$T_CHIRHO"/mcp-*.json | wc -l | tr -d ' ')" "13/13"
for refusal_file_chirho in "$T_CHIRHO"/refusals-chirho/*; do case "$refusal_file_chirho" in *.hdr) continue ;; esac
  say_chirho "note refusal $(basename "$refusal_file_chirho"): $(head -1 "$refusal_file_chirho.hdr" 2>/dev/null | tr -d '\r') | $(wc -c < "$refusal_file_chirho" | tr -d ' ') bytes | $(head -c 160 "$refusal_file_chirho" | tr '\n' '~')"; done
say_chirho "VERDICT: $FAILS_CHIRHO failed assertion(s), $GAPS_CHIRHO known engine gap(s); evidence in $T_CHIRHO"
[ "$FAILS_CHIRHO" -eq 0 ]
