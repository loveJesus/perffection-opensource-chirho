#!/usr/bin/env bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
#
# Gate for the agent API (L.J. 2026-09-20/21: "a person might use Claude Code on their own machine
# and interact ... via the rest or mcp api"; "why didn't you make a REST endpoint?"; "1) yes").
# Through the REAL product routes on a tenant root, with operator-issued scoped bearer keys for two
# synthetic users: no key and a made-up key are refused and get no JSON; her key reads the surface
# description, her page and its version as JSON; a save taken OUT OF the served sample makes
# version 1 that HER BROWSER then shows; a second save makes version 2; a stale save changes
# nothing and names the head; every body the API never asked for is refused with nothing saved and
# nothing echoed; user B's key never reads or writes user A; her history keeps 16 versions, a distinct
# save past that retiring the oldest and an exact repeat retiring nothing (arm 8d); a spent key is
# refused by the engine;
# the preview answers a key; her own agent does the same through the MCP adapter over stdio
# (tools-chirho/agent-mcp-chirho, arm 8b); on an engine with the owner key roads, her signed-in session
# lists her key and ends it, after which the key and her agent get nothing and her drafts stay (arm
# 10b, local owner-key revocation); and the keys' values rest nowhere in the root, the serve
# log or any adapter's output (fail-closed scan with its controls). The arms read STRUCTURE (JSON
# keys, statuses, headers), never a sentence. `expect_chirho` enforces; the exit code is the verdict.
#
# KNOWN ENGINE GAPS are written as `gap_chirho` arms: they pass while the gap is as measured and
# FAIL the day the engine changes, so the flip is noticed and the arm becomes a hard expectation.
# Local dev only: no host, no money, no provider, no real-user email, no customer key creation.
#
# Preconditions: prepare-apps-chirho.sh --app-chirho workspace against the pinned cairn, that
# cairn's cairn-chirho built, and the agent routes bound in app-chirho.toml.
# Needs bun (arm 8b drives the MCP adapter). Env: CAIRN_REPO_CHIRHO, CAIRN_CLI_CHIRHO, REHEARSAL_DIR_CHIRHO,
# TENANT_PORT_CHIRHO (8854), AGENT_API_REAL_CLIENT_CHIRHO (claude-code: arm 8c, one model call; off by default).
set -uo pipefail; umask 077
SCRIPT_DIR_CHIRHO="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_CHIRHO="$(cd "$SCRIPT_DIR_CHIRHO/../.." && pwd)"
CAIRN_REPO_CHIRHO="${CAIRN_REPO_CHIRHO:-$REPO_CHIRHO/../cairn-opensource-chirho}"
# shellcheck source=../lib-chirho/cairn-runtime-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/cairn-runtime-chirho.sh"
# shellcheck source=../lib-chirho/root-scan-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/root-scan-chirho.sh"
# shellcheck source=../lib-chirho/served-root-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/served-root-chirho.sh"
CLI_CHIRHO="$(resolve_cairn_runtime_chirho "$REPO_CHIRHO" "$CAIRN_REPO_CHIRHO")" || exit 2
TENANT_CHIRHO=beta-chirho; TP_CHIRHO="${TENANT_PORT_CHIRHO:-8854}"
require_port_free_chirho "$TP_CHIRHO"
mkdir -p "$REPO_CHIRHO/target/rehearsal-chirho"
T_CHIRHO="${REHEARSAL_DIR_CHIRHO:-$(mktemp -d "$REPO_CHIRHO/target/rehearsal-chirho/agent-api.XXXX")}"; mkdir -p "$T_CHIRHO"
ROOT_CHIRHO="$T_CHIRHO/root-chirho"; mkdir -p "$ROOT_CHIRHO"
TB_CHIRHO="http://127.0.0.1:$TP_CHIRHO"; WS_CHIRHO="$TB_CHIRHO/app-chirho/perffection-workspace"
LOG_CHIRHO="$T_CHIRHO/journey-chirho.log"; : > "$LOG_CHIRHO"
# shellcheck source=../lib-chirho/gate-helpers-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/gate-helpers-chirho.sh"
cd "$REPO_CHIRHO" || exit 2
for required_chirho in "$CLI_CHIRHO" apps-chirho/perffection-workspace-chirho/.emitted-manifest-chirho.json; do
  [ -e "$required_chirho" ] || { printf 'refusing: missing %s\n' "$required_chirho" >&2; exit 2; }
done
grep -q 'route_chirho = "agent-save-page-chirho"' apps-chirho/perffection-workspace-chirho/app-chirho.toml || { printf 'refusing: the agent routes are not bound in app-chirho.toml\n' >&2; exit 2; }
say_chirho "rehearsal dir $T_CHIRHO"; PIN_CHIRHO="$(resolve_cairn_pin_chirho "$REPO_CHIRHO" "$CAIRN_REPO_CHIRHO")" || exit 2
STAMP_CHIRHO="$("$CLI_CHIRHO" --version 2>/dev/null | grep -oE '[0-9a-f]{40}' | head -1)"
say_chirho "cairn pin ${PIN_CHIRHO:0:12} runtime stamp ${STAMP_CHIRHO:0:12} product $(git -C "$REPO_CHIRHO" rev-parse --short=12 HEAD 2>/dev/null || echo unknown)"
check_cairn_runtime_chirho "$CLI_CHIRHO" "$CAIRN_REPO_CHIRHO" "$PIN_CHIRHO" 2>> "$LOG_CHIRHO"; RUNTIME_RC_CHIRHO=$?
CANDIDATE_CHIRHO="$(candidate_cairn_commit_chirho 2>/dev/null || true)"
if [ -n "$CANDIDATE_CHIRHO" ]; then
  say_chirho "CANDIDATE RUN: engine ${CANDIDATE_CHIRHO:0:12} named by CANDIDATE_CAIRN_COMMIT_CHIRHO; this is NOT the product pin ${PIN_CHIRHO:0:12} and proves nothing about it"
  expect_chirho "the runtime is the named CANDIDATE's build (candidate ${CANDIDATE_CHIRHO:0:12}, stamp ${STAMP_CHIRHO:0:12})" "$RUNTIME_RC_CHIRHO" 0
else
  expect_chirho "the runtime is the product pin's build (pin ${PIN_CHIRHO:0:12}, stamp ${STAMP_CHIRHO:0:12})" "$RUNTIME_RC_CHIRHO" 0
fi
cli_chirho() { "$CLI_CHIRHO" --root "$ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" "$@"; }

# Cleanup on every exit, set before any credential or key exists: a refusal below (the staging) must not leave one.
TPID_CHIRHO=""
trap 'kill "$TPID_CHIRHO" 2>/dev/null; wait "$TPID_CHIRHO" 2>/dev/null; rm -f "$T_CHIRHO"/*-cred-chirho.txt "$T_CHIRHO"/*-key-chirho.txt' EXIT

# ---- tenant root: owner, two users, the workspace app, one OAuth client, operator-issued keys ----
OWNER_CHIRHO="$(cli_chirho identity-register --kind owner | cut -f1)"
new_user_chirho() { openssl rand -base64 18 | tr -d '\n' > "$T_CHIRHO/$1-cred-chirho.txt"; chmod 600 "$T_CHIRHO/$1-cred-chirho.txt"; cli_chirho identity-register --kind user --credential-file "$T_CHIRHO/$1-cred-chirho.txt" | cut -f1; }
USER_A_CHIRHO="$(new_user_chirho a)"; USER_B_CHIRHO="$(new_user_chirho b)"; USER_C_CHIRHO="$(new_user_chirho c)"
# shellcheck source=../lib-chirho/prebuilt-app-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/prebuilt-app-chirho.sh"
WORKSPACE_MANIFEST_CHIRHO="$(stage_prebuilt_app_chirho "$CAIRN_REPO_CHIRHO" "$REPO_CHIRHO/apps-chirho/perffection-workspace-chirho" perffection-workspace "$T_CHIRHO/stage-workspace-chirho")" || { printf 'refusing: could not stage the workspace app\n' >&2; exit 2; }
cli_chirho app-source-deploy "$WORKSPACE_MANIFEST_CHIRHO" --owner "$OWNER_CHIRHO" --allow-unreviewed-dev > "$T_CHIRHO/deploy-workspace.out" 2>&1; expect_chirho "workspace app deployed" "$?" 0
CLIENT_CHIRHO="$(register_client_chirho oauth-client.out)"; expect_chirho "OAuth client registered for the workspace app: an id read from its own answer" "$(printf '%s' "$CLIENT_CHIRHO" | grep -cE '^client/[0-9a-f]{16}$')" 1
# key_chirho <name> <principal> <cred name> <max invokes>: the key goes to a mode-600 file, never to a log
key_chirho() { cli_chirho oauth-token --principal "$2" --credential-file "$T_CHIRHO/$3-cred-chirho.txt" --client "$CLIENT_CHIRHO" --scope page-draft-chirho --rate-window-ms 3600000 --rate-max-invokes "$4" 2>/dev/null | head -1 | tr '\t' '\n' | sed -n 's/^token=//p' > "$T_CHIRHO/$1-key-chirho.txt"; chmod 600 "$T_CHIRHO/$1-key-chirho.txt"; [ -s "$T_CHIRHO/$1-key-chirho.txt" ]; }
key_chirho a "$USER_A_CHIRHO" a 400; expect_chirho "a key issued for user A" "$?" 0
key_chirho b "$USER_B_CHIRHO" b 400; expect_chirho "a key issued for user B" "$?" 0
# Measured 2026-09-21: a SECOND key for user A was refused 429 on its first call once A's first key
# had made its calls - the engine seems to count calls per user and client, not per key (its code
# for that was not read). So the spent-key arm uses a user of its own.
key_chirho c-small "$USER_C_CHIRHO" c 2; expect_chirho "a two-call key issued for user C" "$?" 0
# the binary itself, never the cli_chirho function: `$!` must be the engine's own pid
"$CLI_CHIRHO" --root "$ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" serve --port "$TP_CHIRHO" > "$T_CHIRHO/tenant-serve.log" 2>&1 & TPID_CHIRHO=$!
for _ in $(seq 1 80); do curl -s -o /dev/null "$TB_CHIRHO/auth-chirho/login-chirho" && break; sleep 0.5; done
expect_chirho "the server answering on port $TP_CHIRHO is the one this run started" "$(own_server_verdict_chirho "$TPID_CHIRHO" "$TP_CHIRHO")" own
ENGINE_PRIVATE_CHIRHO="$(engine_private_answers_chirho)"; say_chirho "note the engine answers an unowned road private, no-store: $ENGINE_PRIVATE_CHIRHO (1 with Cairn K2)"


MARK_A_CHIRHO="Quillfeather$(openssl rand -hex 6)"; MARK_B_CHIRHO="Harrowgate$(openssl rand -hex 6)"; MARK_REFUSED_CHIRHO="Zebrafinch$(openssl rand -hex 8)"

# 1. no key, and a made-up key: refused, and never a JSON page of anyone's
expect_chirho "no key refused" "$(refused_chirho "$(curl -s -o "$T_CHIRHO/anon.out" -w '%{http_code}' "$WS_CHIRHO/agent-page-chirho")")" refused
expect_chirho "no key gets no page JSON" "$(count_chirho 'page_chirho' anon.out)" 0
printf 'not-a-real-key' > "$T_CHIRHO/fake-key-chirho.txt"
expect_chirho "made-up key refused" "$(refused_chirho "$(api_get_chirho fake agent-page-chirho fake.out)")" refused
expect_chirho "made-up key cannot save" "$(printf '{}' > "$T_CHIRHO/empty.json"; refused_chirho "$(api_post_chirho fake agent-save-page-chirho empty.json fake-save.out)")" refused

# 2. her key reads the surface, her (empty) page and the version to save against, as JSON
expect_chirho "first read with her key" "$(api_get_chirho a agent-page-chirho read-0.json)" 200
expect_chirho "the answer is declared JSON" "$(header_chirho read-0.json content-type)" "application/json; charset=utf-8"
expect_chirho "the answer is nosniff" "$(header_chirho read-0.json x-content-type-options)" nosniff
private_answer_arm_chirho "her private JSON answer carries Cache-Control no-store" "$(no_store_chirho read-0.json)"
expect_chirho "nothing saved yet: state/version/page" "$(json_chirho read-0.json 'd["state_chirho"]')/$(json_chirho read-0.json 'd["current_version_chirho"]')/$(json_chirho read-0.json 'd["page_chirho"]')" "nothing-saved-chirho/0/null"
expect_chirho "the surface names both API paths" "$(json_chirho read-0.json 'sorted(r["path_chirho"] for r in d["surface_chirho"]["routes_chirho"] if "agent-" in r["path_chirho"])')" '["/app-chirho/perffection-workspace/agent-page-chirho", "/app-chirho/perffection-workspace/agent-save-page-chirho"]'
expect_chirho "the surface lists nine fields, each with a limit above zero" "$(json_chirho read-0.json 'len([f for f in d["surface_chirho"]["page_document_chirho"]["fields_chirho"] if f["max_characters_chirho"] > 0])')" 9

# 3. a save built OUT OF the served sample (what an agent would do) makes version 1 ...
save_from_chirho() { python3 - "$T_CHIRHO/$1" "$T_CHIRHO/$2" "$3" "$4" "$5" <<'PY'
import json, sys
src, out, version, name, headline = sys.argv[1:6]
d = json.load(open(src, encoding="utf-8"))
page = d.get("page_chirho") or d["surface_chirho"]["sample_page_chirho"]
page["business_name_chirho"] = name
if headline: page["headline_chirho"] = headline
json.dump({"expected_version_chirho": int(version), "page_chirho": page}, open(out, "w", encoding="utf-8"))
PY
}
save_from_chirho read-0.json save-1.json 0 "$MARK_A_CHIRHO Bindery" ""
expect_chirho "save at version 0" "$(api_post_chirho a agent-save-page-chirho save-1.json saved-1.json)" 200
expect_chirho "saved/version/her name kept" "$(json_chirho saved-1.json 'd["saved_chirho"]')/$(json_chirho saved-1.json 'd["current_version_chirho"]')/$(json_chirho saved-1.json 'd["page_chirho"]["business_name_chirho"]')" "true/1/$MARK_A_CHIRHO Bindery"
expect_chirho "the save answer is declared JSON" "$(header_chirho saved-1.json content-type)" "application/json; charset=utf-8"
# ... that HER BROWSER then shows: a real sign-in, the editor at version 1 with her name in the field
expect_chirho "her browser signs in" "$(sign_in_chirho "$USER_A_CHIRHO" a login.hdr)" 303
COOKIE_CHIRHO="$(session_cookie_chirho login.hdr)"
browser_get_chirho() { curl -s -o "$T_CHIRHO/$2" -w '%{http_code}' -H "Cookie: $COOKIE_CHIRHO" "$WS_CHIRHO/$1"; }
editor_version_chirho() { python3 - "$T_CHIRHO/$1" <<'PY'
import re, sys
m = re.search(r'class="workspace-version-chirho">[^<]*<strong>([0-9]+)</strong>', open(sys.argv[1], encoding="utf-8").read())
print(m.group(1) if m else "none")
PY
}
expect_chirho "her browser's editor: status/version/her name in the field" "$(browser_get_chirho '' editor-1.html)/$(editor_version_chirho editor-1.html)/$(count_chirho "value=\"$MARK_A_CHIRHO Bindery\"" editor-1.html)" "200/1/1"

# 4. read -> revise -> save makes version 2; her browser can restore version 1
expect_chirho "second read" "$(api_get_chirho a agent-page-chirho read-1.json)/$(json_chirho read-1.json 'd["state_chirho"]')/$(json_chirho read-1.json 'd["current_version_chirho"]')" "200/saved-chirho/1"
save_from_chirho read-1.json save-2.json 1 "$MARK_A_CHIRHO Bindery" "Books mended and rebound by hand"
expect_chirho "save at version 1" "$(api_post_chirho a agent-save-page-chirho save-2.json saved-2.json)/$(json_chirho saved-2.json 'd["saved_chirho"]')/$(json_chirho saved-2.json 'd["current_version_chirho"]')" "200/true/2"
expect_chirho "her browser offers to restore version 1" "$(browser_get_chirho '' editor-2.html)/$(editor_version_chirho editor-2.html)/$(count_chirho 'name="restore_from_version_chirho" value="1"' editor-2.html)" "200/2/1"

# 4b. an EXACT repeat of an earlier identical save (a retry after a lost answer) is answered as that
# save again - saved, with the version it produced the first time - and stores no second copy: the
# head stays where it was (gpt #24436 measured this first; the description says so).
expect_chirho "exact repeat of the first save: saved/its own version" "$(api_post_chirho a agent-save-page-chirho save-1.json replay-1.json)/$(json_chirho replay-1.json 'd["saved_chirho"]')/$(json_chirho replay-1.json 'd["current_version_chirho"]')" "200/true/1"
expect_chirho "the repeat stored nothing: the head is still version 2 with the second headline" "$(api_get_chirho a agent-page-chirho read-replay.json)/$(json_chirho read-replay.json 'd["current_version_chirho"]')/$(json_chirho read-replay.json 'd["page_chirho"]["headline_chirho"]')" "200/2/Books mended and rebound by hand"

# 5. a STALE save changes nothing and names the head
save_from_chirho read-1.json save-stale.json 1 "$MARK_REFUSED_CHIRHO Overwrite" ""
expect_chirho "stale save: saved/refusal/head" "$(api_post_chirho a agent-save-page-chirho save-stale.json stale.json)/$(json_chirho stale.json 'd["saved_chirho"]')/$(json_chirho stale.json 'd["refusal_chirho"]')/$(json_chirho stale.json 'd["current_version_chirho"]')" "200/false/stale-version-chirho/2"
expect_chirho "stale save echoed nothing" "$(count_chirho "$MARK_REFUSED_CHIRHO" stale.json)" 0

# 6. bodies the API never asked for: refused, nothing saved, nothing echoed
refusal_arm_chirho() { # <label> <body file> <expected refusal code>
  expect_chirho "$1: status/saved/refusal" "$(api_post_chirho a agent-save-page-chirho "$2" "$2.out")/$(json_chirho "$2.out" 'd["saved_chirho"]')/$(json_chirho "$2.out" 'd["refusal_chirho"]')" "200/false/$3"
  expect_chirho "$1: not echoed" "$(count_chirho "$MARK_REFUSED_CHIRHO" "$2.out")" 0
}
python3 - "$T_CHIRHO" "$MARK_REFUSED_CHIRHO" <<'PY'
import json, sys
t, mark = sys.argv[1:3]
base = json.load(open(f"{t}/save-2.json", encoding="utf-8")); base["expected_version_chirho"] = 2
def write(name, value): open(f"{t}/{name}", "w", encoding="utf-8").write(value if isinstance(value, str) else json.dumps(value))
invalid = json.loads(json.dumps(base)); invalid["page_chirho"]["business_name_chirho"] = ""; invalid["page_chirho"]["headline_chirho"] = mark + "x" * 200
write("bad-invalid.json", invalid)
unknown = json.loads(json.dumps(base)); unknown["page_chirho"]["slug_chirho"] = mark
write("bad-unknown-key.json", unknown)
extra = json.loads(json.dumps(base)); extra["publish_chirho"] = mark
write("bad-extra-key.json", extra)
write("bad-not-json.json", f"please save this for me: {mark}")
write("bad-browser-form.json", f"expected_version_chirho=2&business_name_chirho={mark}")
old = json.loads(json.dumps(base)); old["page_chirho"]["schema_version_chirho"] = 1; old["page_chirho"]["business_name_chirho"] = mark
write("bad-old-schema.json", old)
big = json.loads(json.dumps(base)); big["page_chirho"]["summary_chirho"] = mark + "y" * 40000
write("bad-oversize.json", big)
PY
refusal_arm_chirho "an incomplete, over-limit page" bad-invalid.json invalid-page-chirho
expect_chirho "the refusal names BOTH fields" "$(json_chirho bad-invalid.json.out 'sorted(i["field_path_chirho"] for i in d["issues_chirho"])')" '["business_name_chirho", "headline_chirho"]'
refusal_arm_chirho "a key the page does not have" bad-unknown-key.json unreadable-chirho
refusal_arm_chirho "a key beside the two" bad-extra-key.json unreadable-chirho
refusal_arm_chirho "prose instead of JSON" bad-not-json.json unreadable-chirho
refusal_arm_chirho "her browser's form body" bad-browser-form.json unreadable-chirho
refusal_arm_chirho "an old schema" bad-old-schema.json unsupported-schema-chirho
refusal_arm_chirho "a body past the bound" bad-oversize.json oversize-chirho
expect_chirho "after every refusal her page is STILL version 2" "$(api_get_chirho a agent-page-chirho read-2.json)/$(json_chirho read-2.json 'd["current_version_chirho"]')/$(json_chirho read-2.json 'd["page_chirho"]["headline_chirho"]')" "200/2/Books mended and rebound by hand"

# 7. user B's key is user B's page only
expect_chirho "B reads nothing of A" "$(api_get_chirho b agent-page-chirho b-read-0.json)/$(json_chirho b-read-0.json 'd["state_chirho"]')/$(count_chirho "$MARK_A_CHIRHO" b-read-0.json)" "200/nothing-saved-chirho/0"
save_from_chirho b-read-0.json b-save-1.json 0 "$MARK_B_CHIRHO Forge" ""
expect_chirho "B saves B's own version 1" "$(api_post_chirho b agent-save-page-chirho b-save-1.json b-saved-1.json)/$(json_chirho b-saved-1.json 'd["current_version_chirho"]')" "200/1"
expect_chirho "A is untouched by B: version/name, and B's name nowhere in A's answer" "$(api_get_chirho a agent-page-chirho read-3.json)/$(json_chirho read-3.json 'd["current_version_chirho"]')/$(count_chirho "$MARK_A_CHIRHO Bindery" read-3.json)/$(count_chirho "$MARK_B_CHIRHO" read-3.json)" "200/2/1/0"

# 8. the preview answers a key as it answers her browser
expect_chirho "preview with her key: status/type/her name" "$(api_get_chirho a preview-chirho preview.html)/$(header_chirho preview.html content-type | cut -d';' -f1)/$([ "$(count_chirho "$MARK_A_CHIRHO Bindery" preview.html)" -ge 1 ] && echo shown || echo missing)" "200/text/html/shown"

# 8b. her own agent through the MCP adapter (tools-chirho/agent-mcp-chirho, over stdio) on these
# same routes and keys: both protocol eras; a save that her next read, her REST read and the
# preview all show; a stale save refused; B's key sees B's page only; a made-up key gets no answer,
# so a save's result is UNKNOWN, never "nothing stored". Structure only; every adapter's stdout and
# stderr are kept for arm 11's key scan.
MCP_MARK_CHIRHO="Inkwell$(openssl rand -hex 6)"; MCP_OUT_CHIRHO="$T_CHIRHO/mcp-transcripts-chirho"
bun "$REPO_CHIRHO/scripts-chirho/tests-chirho/agent-mcp-journey-chirho.ts" "$TB_CHIRHO" "$T_CHIRHO/a-key-chirho.txt" "$T_CHIRHO/b-key-chirho.txt" "$T_CHIRHO/fake-key-chirho.txt" "$MCP_OUT_CHIRHO" "$MCP_MARK_CHIRHO" "$MARK_REFUSED_CHIRHO" >> "$LOG_CHIRHO" 2>&1
expect_chirho "the MCP journey ran to its end" "$?" 0
MS_CHIRHO=mcp-transcripts-chirho/mcp-journey-summary-chirho.json
expect_chirho "MCP discovery: result/2026-07-28 offered/tools" "$(json_chirho $MS_CHIRHO 'd["discover_result_type_chirho"]')/$(json_chirho $MS_CHIRHO 'd["discover_offers_2026_chirho"]')/$(json_chirho $MS_CHIRHO 'd["tools_chirho"]')" 'complete/true/["preview_page_chirho", "read_figures_chirho", "read_page_chirho", "save_figures_chirho", "save_page_chirho"]'
expect_chirho "the page surface lists exactly the three page routes the adapter calls" "$(json_chirho $MS_CHIRHO 'd["surface_routes_chirho"]')" '["GET /app-chirho/perffection-workspace/agent-page-chirho", "GET /app-chirho/perffection-workspace/preview-chirho", "POST /app-chirho/perffection-workspace/agent-save-page-chirho"]'
expect_chirho "MCP read: flagged/state/version" "$(json_chirho $MS_CHIRHO 'd["read_chirho"]["is_error_chirho"]')/$(json_chirho $MS_CHIRHO 'd["read_chirho"]["state_chirho"]')/$(json_chirho $MS_CHIRHO 'd["read_chirho"]["version_chirho"]')" "false/saved-chirho/2"
expect_chirho "MCP save at version 2: flagged/saved/version/name kept" "$(json_chirho $MS_CHIRHO 'd["save_chirho"]["is_error_chirho"]')/$(json_chirho $MS_CHIRHO 'd["save_chirho"]["saved_chirho"]')/$(json_chirho $MS_CHIRHO 'd["save_chirho"]["version_chirho"]')/$(json_chirho $MS_CHIRHO 'd["save_chirho"]["name_kept_chirho"]')" "false/true/3/true"
expect_chirho "MCP stale save at version 2: flagged/refusal/head" "$(json_chirho $MS_CHIRHO 'd["stale_chirho"]["is_error_chirho"]')/$(json_chirho $MS_CHIRHO 'd["stale_chirho"]["refusal_chirho"]')/$(json_chirho $MS_CHIRHO 'd["stale_chirho"]["head_chirho"]')" "true/stale-version-chirho/3"
expect_chirho "after the MCP save: read version/its name/preview flagged/preview shows it" "$(json_chirho $MS_CHIRHO 'd["after_chirho"]["version_chirho"]')/$(json_chirho $MS_CHIRHO 'd["after_chirho"]["name_is_the_mcp_save_chirho"]')/$(json_chirho $MS_CHIRHO 'd["after_chirho"]["preview_is_error_chirho"]')/$(json_chirho $MS_CHIRHO 'd["after_chirho"]["preview_shows_the_mcp_save_chirho"]')" "3/true/false/true"
expect_chirho "her REST read shows the MCP save: status/version/name" "$(api_get_chirho a agent-page-chirho read-mcp.json)/$(json_chirho read-mcp.json 'd["current_version_chirho"]')/$(json_chirho read-mcp.json 'd["page_chirho"]["business_name_chirho"]')" "200/3/$MCP_MARK_CHIRHO Press"
expect_chirho "MCP legacy era: negotiated version/read version" "$(json_chirho $MS_CHIRHO 'd["legacy_chirho"]["version_chirho"]')/$(json_chirho $MS_CHIRHO 'd["legacy_chirho"]["read_version_chirho"]')" "2025-06-18/3"
expect_chirho "MCP with B's key: flagged/sees A" "$(json_chirho $MS_CHIRHO 'd["b_chirho"]["is_error_chirho"]')/$(json_chirho $MS_CHIRHO 'd["b_chirho"]["sees_a_chirho"]')" "false/false"
expect_chirho "MCP with a made-up key: no answer, refused, a save's result unknown" "$(json_chirho $MS_CHIRHO 'd["made_up_chirho"]["read_outcome_chirho"]')/$(refused_chirho "$(json_chirho $MS_CHIRHO 'd["made_up_chirho"]["read_status_chirho"]')")/$(json_chirho $MS_CHIRHO 'd["made_up_chirho"]["save_result_chirho"]')" "no-answer-chirho/refused/unknown-chirho"
expect_chirho "every adapter ended by itself when its stdin closed: adapters/all exit 0" "$(json_chirho $MS_CHIRHO 'len(d["exits_chirho"])')/$(json_chirho $MS_CHIRHO 'all(v == 0 for v in d["exits_chirho"].values())')" "4/true"

# 8c. OPTIONAL, off unless AGENT_API_REAL_CLIENT_CHIRHO=claude-code: an independent MCP client, not
# the adapter's own test client - Claude Code, headless, as her agent - asked in plain words to change
# only the headline and save. Her REST read must show exactly that change as the next version. The
# traffic both ways is teed into arm 11's scanned folder. This arm makes a model call; the default
# gate never does.
if [ "${AGENT_API_REAL_CLIENT_CHIRHO:-}" = "claude-code" ]; then
  REAL_HEADLINE_CHIRHO="Fresh bread every morning since 1987"
  api_get_chirho a agent-page-chirho read-before-real.json > /dev/null
  REAL_BEFORE_CHIRHO="$(json_chirho read-before-real.json 'd["current_version_chirho"]')"
  REAL_DIR_CHIRHO="$T_CHIRHO/real-client-cwd-chirho"; mkdir -p "$REAL_DIR_CHIRHO"
  python3 - "$REAL_DIR_CHIRHO/mcp-config-chirho.json" "$REPO_CHIRHO/tools-chirho/agent-mcp-chirho/perffection-mcp-chirho.ts" "$MCP_OUT_CHIRHO" "$TB_CHIRHO" "$T_CHIRHO/a-key-chirho.txt" <<'PY2'
import json, shlex, sys
out, adapter, logs, base, key_file = sys.argv[1:6]
tee = f"tee -a {shlex.quote(logs + '/real-client-in-chirho.jsonl')} | bun {shlex.quote(adapter)} 2>>{shlex.quote(logs + '/real-client-stderr-chirho.txt')} | tee -a {shlex.quote(logs + '/real-client-out-chirho.jsonl')}"
json.dump({"mcpServers": {"perffection-chirho": {"type": "stdio", "command": "bash", "args": ["-c", tee],
  "env": {"PERFFECTION_BASE_URL_CHIRHO": base, "PERFFECTION_KEY_FILE_CHIRHO": key_file}}}}, open(out, "w"))
PY2
  REAL_TOOLS_CHIRHO="mcp__perffection-chirho__read_page_chirho,mcp__perffection-chirho__save_page_chirho,mcp__perffection-chirho__preview_page_chirho"
  # a fresh customer's machine: none of the calling session's CLAUDE* variables reach the client
  clean_env_chirho=(); for name_chirho in $(compgen -e | grep -E '^CLAUDE'); do clean_env_chirho+=(-u "$name_chirho"); done
  (cd "$REAL_DIR_CHIRHO" && timeout 420 env "${clean_env_chirho[@]}" claude -p "You are helping the owner of a small business website. Use the perffection-chirho tools. Read the page, then change ONLY the headline to exactly: $REAL_HEADLINE_CHIRHO. Keep every other field exactly as it is, and save the whole page against the version you read. Reply with just the version number you saved." \
    --mcp-config "$REAL_DIR_CHIRHO/mcp-config-chirho.json" --strict-mcp-config --allowedTools "$REAL_TOOLS_CHIRHO" --permission-mode dontAsk \
    --setting-sources local --no-session-persistence --model sonnet --output-format json < /dev/null > "$MCP_OUT_CHIRHO/real-client-result-chirho.json" 2> "$MCP_OUT_CHIRHO/real-client-cli-stderr-chirho.txt")
  expect_chirho "Claude Code as her agent: exit/reported error" "$?/$(json_chirho mcp-transcripts-chirho/real-client-result-chirho.json 'd["is_error"]')" "0/false"
  expect_chirho "her REST read after Claude Code: version is the next one/headline/name untouched" "$(api_get_chirho a agent-page-chirho read-real.json > /dev/null; [ "$(json_chirho read-real.json 'd["current_version_chirho"]')" = "$((REAL_BEFORE_CHIRHO + 1))" ] && echo next || echo other)/$(json_chirho read-real.json 'd["page_chirho"]["headline_chirho"]')/$(json_chirho read-real.json 'd["page_chirho"]["business_name_chirho"]')" "next/$REAL_HEADLINE_CHIRHO/$MCP_MARK_CHIRHO Press"
  REAL_ERA_CHIRHO="$(python3 - "$MCP_OUT_CHIRHO/real-client-in-chirho.jsonl" <<'PY2'
import json, sys
methods, calls = [], []
for line in open(sys.argv[1], encoding="utf-8"):
    try: m = json.loads(line)
    except Exception: continue
    methods.append(m.get("method")); calls += [m["params"].get("name")] if m.get("method") == "tools/call" else []
era = "modern" if "server/discover" in methods and "initialize" not in methods else "legacy" if "initialize" in methods else "unknown"
print(f"{era}:{'read-then-save' if 'read_page_chirho' in calls and 'save_page_chirho' in calls and calls.index('read_page_chirho') < calls.index('save_page_chirho') else 'other:' + ','.join(map(str, calls))}")
PY2
)"
  say_chirho "note Claude Code $(claude --version 2>/dev/null | head -1) spoke the ${REAL_ERA_CHIRHO%%:*} era"
  expect_chirho "Claude Code read the page before it saved" "${REAL_ERA_CHIRHO#*:}" "read-then-save"
else
  say_chirho "skip 8c: the independent-client arm runs only with AGENT_API_REAL_CLIENT_CHIRHO=claude-code (it makes a model call)"
fi

# 8d. her history is bounded: the engine keeps 16 versions (PAGE_STATE_HISTORY_MAX_VERSIONS_CHIRHO).
# At 16 her browser still offers version 1; one more DISTINCT save retires it, and version 2 stays.
# An EXACT repeat of the latest save stores nothing and retires nothing. This is why the adapter marks
# the save tool destructive and idempotent (claude2 #26531, gpt #26537); history is never unbounded.
HISTORY_KEPT_CHIRHO=16
restore_offers_chirho() { browser_get_chirho '' "$1" > /dev/null; printf '%s/%s' "$(count_chirho 'name="restore_from_version_chirho" value="1"' "$1")" "$(count_chirho 'name="restore_from_version_chirho" value="2"' "$1")"; }
api_get_chirho a agent-page-chirho history-last.json > /dev/null
HEAD_CHIRHO="$(json_chirho history-last.json 'd["current_version_chirho"]')"
while [ "$HEAD_CHIRHO" -lt "$HISTORY_KEPT_CHIRHO" ]; do
  save_from_chirho history-last.json history-body.json "$HEAD_CHIRHO" "$MARK_A_CHIRHO Bindery" "Kept version $((HEAD_CHIRHO + 1))"
  [ "$(api_post_chirho a agent-save-page-chirho history-body.json history-last.json)" = 200 ] || break
  [ "$(json_chirho history-last.json 'd["saved_chirho"]')" = true ] || break
  HEAD_CHIRHO="$(json_chirho history-last.json 'd["current_version_chirho"]')"
done
expect_chirho "distinct saves fill the history: head/her browser offers version 1/version 2" "$HEAD_CHIRHO/$(restore_offers_chirho editor-history-full.html)" "$HISTORY_KEPT_CHIRHO/1/1"
save_from_chirho history-last.json history-body.json "$HEAD_CHIRHO" "$MARK_A_CHIRHO Bindery" "Kept version $((HEAD_CHIRHO + 1))"
expect_chirho "one more distinct save: status/saved/version" "$(api_post_chirho a agent-save-page-chirho history-body.json history-past.json)/$(json_chirho history-past.json 'd["saved_chirho"]')/$(json_chirho history-past.json 'd["current_version_chirho"]')" "200/true/$((HISTORY_KEPT_CHIRHO + 1))"
expect_chirho "the oldest version is retired, the next stays: her browser offers version 1/version 2" "$(restore_offers_chirho editor-history-past.html)" "0/1"
expect_chirho "an exact repeat of that save: status/saved/its own version" "$(api_post_chirho a agent-save-page-chirho history-body.json history-replay.json)/$(json_chirho history-replay.json 'd["saved_chirho"]')/$(json_chirho history-replay.json 'd["current_version_chirho"]')" "200/true/$((HISTORY_KEPT_CHIRHO + 1))"
expect_chirho "the repeat stored and retired nothing: head/her browser offers version 2" "$(api_get_chirho a agent-page-chirho history-after-replay.json > /dev/null; json_chirho history-after-replay.json 'd["current_version_chirho"]')/$(restore_offers_chirho editor-history-replay.html | cut -d/ -f2)" "$((HISTORY_KEPT_CHIRHO + 1))/1"
# A late repeat of the FIRST save, whose version is now retired, is still answered as that save
# again (saved, version 1, the page as it stored then: the replay answer outlives the history, measured
# on both engines 2026-10-08), and it stores and retires nothing.
expect_chirho "a late repeat of the retired version's save: status/saved/its own version" "$(api_post_chirho a agent-save-page-chirho save-1.json history-late.json)/$(json_chirho history-late.json 'd["saved_chirho"]')/$(json_chirho history-late.json 'd["current_version_chirho"]')" "200/true/1"
expect_chirho "the late repeat stored and retired nothing: head/her browser offers version 2" "$(api_get_chirho a agent-page-chirho history-after-late.json > /dev/null; json_chirho history-after-late.json 'd["current_version_chirho"]')/$(restore_offers_chirho editor-history-late.html | cut -d/ -f2)" "$((HISTORY_KEPT_CHIRHO + 1))/1"

# 9. a spent key is refused by the engine (the key's own rate limit: two calls)
expect_chirho "two-call key: call 1" "$(api_get_chirho c-small agent-page-chirho small-1.json)" 200
expect_chirho "two-call key: call 2" "$(api_get_chirho c-small agent-page-chirho small-2.json)" 200
expect_chirho "two-call key: call 3 is refused 429 and carries no surface" "$(api_get_chirho c-small agent-page-chirho small-3.json)/$(count_chirho 'surface_chirho' small-3.json)" "429/0"
USES_CHIRHO="$ROOT_CHIRHO/tenants-chirho/$TENANT_CHIRHO/scoped-token-uses-chirho.jsonl"
expect_chirho "the engine recorded the keys' calls" "$([ -f "$USES_CHIRHO" ] && [ "$(wc -l < "$USES_CHIRHO" | tr -d ' ')" -ge 1 ] && echo recorded || echo none)" recorded

# 10. KNOWN GAP: nothing limits what a key may do. A key scoped page-draft-chirho still reads the
# Publishing page (a route no draft scope names). While this holds there is no customer key creation.
gap_chirho "a draft-scoped key is refused at the Publishing page" "$(refused_chirho "$(api_get_chirho a publication-status-chirho publishing.html)")" 200

# 10b. local owner-key revocation on the owner key roads (Cairn 4b-2, b3da5554). With her signed-in
# session (session-authenticated HTTP, as curl; the Account page in a browser is NOT exercised here) she
# lists the key her agent uses and ends it, and the end holds. Only her session administers keys; a key
# never lists or ends one. What survives the end is read through HER SESSION, never through the ended
# key. An engine from before 4b-2 has no such roads, so the arm is skipped by VERSION, never by probing:
# a road that should answer and does not is a failure, not an absence. Issuance and a key's expiry are
# not exercised here (the operator CLI issues no short-lived key); Cairn's own 4b-2 arms hold expiry.
OWNER_ROADS_COMMIT_CHIRHO=b3da555466e043a1655743df9d6f19422f26d3df
git -C "$CAIRN_REPO_CHIRHO" merge-base --is-ancestor "$OWNER_ROADS_COMMIT_CHIRHO" "$STAMP_CHIRHO" 2>/dev/null; ROADS_RC_CHIRHO=$?
if [ "$ROADS_RC_CHIRHO" -gt 1 ]; then
  expect_chirho "this run can tell whether engine ${STAMP_CHIRHO:0:12} carries the owner key roads" unknown known
elif [ "$ROADS_RC_CHIRHO" -eq 1 ]; then
  say_chirho "skip 10b: engine ${STAMP_CHIRHO:0:12} predates the owner key roads (${OWNER_ROADS_COMMIT_CHIRHO:0:12})"
else
  # keys_chirho <out> [curl args...]: the owner key list; end_key_chirho <out> <key id> [curl args...]: the revoke door
  keys_chirho() { local out_chirho="$1"; shift; curl -s -o "$T_CHIRHO/$out_chirho" -D "$T_CHIRHO/$out_chirho.hdr" -w '%{http_code}' "$@" "$TB_CHIRHO/auth-chirho/keys-chirho"; }
  end_key_chirho() { local out_chirho="$1"; printf '{"key_id_chirho":"%s"}' "$2" > "$T_CHIRHO/$out_chirho.body"; shift 2
    curl -s -o "$T_CHIRHO/$out_chirho" -D "$T_CHIRHO/$out_chirho.hdr" -w '%{http_code}' -H 'Content-Type: application/json' --data-binary "@$T_CHIRHO/$out_chirho.body" "$@" "$TB_CHIRHO/auth-chirho/key-revoke-chirho"; }
  as_a_chirho=(-H "Cookie: $COOKIE_CHIRHO")
  expect_chirho "her key list: status/ok/her keys/live" "$(keys_chirho keys-a-0.json "${as_a_chirho[@]}")/$(json_chirho keys-a-0.json 'd["ok_chirho"]')/$(json_chirho keys-a-0.json 'len(d["keys_chirho"])')/$(json_chirho keys-a-0.json 'd["keys_chirho"][0]["live_chirho"]')" "200/true/1/true"
  expect_chirho "her key list names the workspace and the draft scope" "$(json_chirho keys-a-0.json 'd["keys_chirho"][0]["app_chirho"]')/$(json_chirho keys-a-0.json 'd["keys_chirho"][0]["scopes_chirho"]')" 'perffection-workspace/["page-draft-chirho"]'
  expect_chirho "her key list is private: no-store" "$(header_chirho keys-a-0.json cache-control | grep -c 'no-store')" 1
  expect_chirho "her key list carries no key: hers/B's" "$(file_hits_chirho "$(cat "$T_CHIRHO/a-key-chirho.txt")" "$T_CHIRHO/keys-a-0.json")/$(file_hits_chirho "$(cat "$T_CHIRHO/b-key-chirho.txt")" "$T_CHIRHO/keys-a-0.json")" "0/0"
  KEY_A_ID_CHIRHO="$(json_chirho keys-a-0.json 'd["keys_chirho"][0]["key_id_chirho"]')"
  expect_chirho "B's browser signs in" "$(sign_in_chirho "$USER_B_CHIRHO" b login-b.hdr)" 303
  as_b_chirho=(-H "Cookie: $(session_cookie_chirho login-b.hdr)")
  expect_chirho "B's list: status/B's one key/not hers" "$(keys_chirho keys-b-0.json "${as_b_chirho[@]}")/$(json_chirho keys-b-0.json 'len(d["keys_chirho"])')/$([ "$(json_chirho keys-b-0.json 'd["keys_chirho"][0]["key_id_chirho"]')" != "$KEY_A_ID_CHIRHO" ] && echo own || echo shared)" "200/1/own"
  KEY_B_ID_CHIRHO="$(json_chirho keys-b-0.json 'd["keys_chirho"][0]["key_id_chirho"]')"
  # no session, and a key in place of one: nothing listed, nothing ended
  expect_chirho "no session: list refused/token/keys listed" "$(keys_chirho keys-anon.json)/$(json_chirho keys-anon.json 'd["error_chirho"]')/$(count_chirho 'key_id_chirho' keys-anon.json)" "401/no-session-chirho/0"
  expect_chirho "her key cannot list keys: status/keys listed" "$(keys_chirho keys-bearer.json -H "Authorization: Bearer $(cat "$T_CHIRHO/a-key-chirho.txt")")/$(count_chirho 'key_id_chirho' keys-bearer.json)" "401/0"
  expect_chirho "her key cannot end itself: status/the key still reads" "$(end_key_chirho end-bearer.json "$KEY_A_ID_CHIRHO" -H "Authorization: Bearer $(cat "$T_CHIRHO/a-key-chirho.txt")")/$(api_get_chirho a agent-page-chirho read-after-bearer-end.json)" "401/200"
  # B's key id is, to her, exactly an unknown id; B's key lives on
  end_key_chirho end-unknown.json "$(openssl rand -hex 16)" "${as_a_chirho[@]}" > "$T_CHIRHO/end-unknown.code"
  expect_chirho "she cannot end B's key: status/token/same bytes as an unknown id" "$(end_key_chirho end-cross.json "$KEY_B_ID_CHIRHO" "${as_a_chirho[@]}")/$(json_chirho end-cross.json 'd["error_chirho"]')/$(cmp -s "$T_CHIRHO/end-cross.json" "$T_CHIRHO/end-unknown.json" && [ "$(cat "$T_CHIRHO/end-unknown.code")" = 404 ] && echo same || echo different)" "404/no-such-key-chirho/same"
  expect_chirho "B's key still reads B's page" "$(api_get_chirho b agent-page-chirho b-read-after-cross.json)" 200
  # she ends her own key, twice: the same answer both times
  api_get_chirho a agent-page-chirho read-before-end.json > /dev/null
  HEAD_BEFORE_END_CHIRHO="$(json_chirho read-before-end.json 'd["current_version_chirho"]')"; NAME_BEFORE_END_CHIRHO="$(json_chirho read-before-end.json 'd["page_chirho"]["business_name_chirho"]')"
  expect_chirho "she ends her key: status/ok, then again" "$(end_key_chirho end-own.json "$KEY_A_ID_CHIRHO" "${as_a_chirho[@]}")/$(json_chirho end-own.json 'd["ok_chirho"]')/$(end_key_chirho end-own-again.json "$KEY_A_ID_CHIRHO" "${as_a_chirho[@]}")/$(json_chirho end-own-again.json 'd["ok_chirho"]')" "200/true/200/true"
  expect_chirho "the key-road answers carry no key" "$(cat "$T_CHIRHO"/keys-*.json "$T_CHIRHO"/end-*.json > "$T_CHIRHO/key-roads-all.txt"; file_hits_chirho "$(cat "$T_CHIRHO/a-key-chirho.txt")" "$T_CHIRHO/key-roads-all.txt")/$(file_hits_chirho "$(cat "$T_CHIRHO/b-key-chirho.txt")" "$T_CHIRHO/key-roads-all.txt")" "0/0"
  # the ended key gets nothing on any road, refused exactly as a made-up key is (no oracle)
  MARK_ENDED_CHIRHO="Lanternfish$(openssl rand -hex 6)"
  save_from_chirho read-before-end.json ended-body.json "$HEAD_BEFORE_END_CHIRHO" "$MARK_ENDED_CHIRHO Bindery" ""
  expect_chirho "the ended key: read/save/preview, each as a made-up key" "$(api_get_chirho a agent-page-chirho ended-read.json)/$(api_post_chirho a agent-save-page-chirho ended-body.json ended-save.json)/$(api_get_chirho a preview-chirho ended-preview.html)" "$(api_get_chirho fake agent-page-chirho fake-read.json)/$(api_post_chirho fake agent-save-page-chirho ended-body.json fake-save-2.json)/$(api_get_chirho fake preview-chirho fake-preview.html)"
  # her page under either name: the one saved before the end, or one the ended key's save would have stored
  expect_chirho "the ended key was refused, and got no page and no preview of hers" "$(refused_chirho "$(api_get_chirho a agent-page-chirho ended-read-2.json)")/$(count_chirho 'page_chirho' ended-read.json)/$(count_chirho "$NAME_BEFORE_END_CHIRHO" ended-preview.html)/$(count_chirho "$MARK_ENDED_CHIRHO" ended-preview.html)" "refused/0/0/0"
  # her agent, still holding the ended key: no page, no preview, and a save it cannot know about
  bun "$REPO_CHIRHO/scripts-chirho/tests-chirho/agent-mcp-ended-key-chirho.ts" "$TB_CHIRHO" "$T_CHIRHO/a-key-chirho.txt" "$T_CHIRHO/ended-body.json" "$MCP_OUT_CHIRHO" >> "$LOG_CHIRHO" 2>&1
  expect_chirho "the ended-key MCP run ran to its end" "$?" 0
  ES_CHIRHO=mcp-transcripts-chirho/mcp-ended-key-summary-chirho.json
  expect_chirho "her agent with the ended key: read refused, no page/save unknown/preview refused/adapter exit" "$(refused_chirho "$(json_chirho $ES_CHIRHO 'd["read_chirho"]["status_chirho"]')")/$(json_chirho $ES_CHIRHO 'd["read_chirho"]["has_page_chirho"]')/$(json_chirho $ES_CHIRHO 'd["save_chirho"]["save_result_chirho"]')/$(refused_chirho "$(json_chirho $ES_CHIRHO 'd["preview_chirho"]["status_chirho"]')")/$(json_chirho $ES_CHIRHO 'd["exit_chirho"]')" "refused/false/unknown-chirho/refused/0"
  # the witness is her still-signed-in session: the same version and name, and no ended-key save
  expect_chirho "her session after the end: status/version/her name/the ended key's save" "$(browser_get_chirho '' editor-after-end.html)/$(editor_version_chirho editor-after-end.html)/$(count_chirho "value=\"$NAME_BEFORE_END_CHIRHO\"" editor-after-end.html)/$(count_chirho "$MARK_ENDED_CHIRHO" editor-after-end.html)" "200/$HEAD_BEFORE_END_CHIRHO/1/0"
  # the end is shown, not hidden: her list keeps the key, marked not live; B is untouched
  expect_chirho "her list after the end: status/count/the same key/live" "$(keys_chirho keys-a-1.json "${as_a_chirho[@]}")/$(json_chirho keys-a-1.json 'len(d["keys_chirho"])')/$([ "$(json_chirho keys-a-1.json 'd["keys_chirho"][0]["key_id_chirho"]')" = "$KEY_A_ID_CHIRHO" ] && echo same || echo other)/$(json_chirho keys-a-1.json 'd["keys_chirho"][0]["live_chirho"]')" "200/1/same/false"
  expect_chirho "B after her end: B's key reads/B's list live" "$(api_get_chirho b agent-page-chirho b-read-after-end.json)/$(keys_chirho keys-b-1.json "${as_b_chirho[@]}" > /dev/null; json_chirho keys-b-1.json 'd["keys_chirho"][0]["live_chirho"]')" "200/true"
fi

# 11. the keys' values rest nowhere (fail-closed scan, with its controls)
kill "$TPID_CHIRHO" 2>/dev/null; wait "$TPID_CHIRHO" 2>/dev/null
expect_chirho "control: the store scan sees clear text in this root" "$(seen_verdict_chirho "$(scan_root_chirho 'agent-save-page-chirho' "$ROOT_CHIRHO")")" sees
scan_controls_chirho "$MARK_REFUSED_CHIRHO" "$T_CHIRHO"
for key_name_chirho in a b c-small; do
  expect_chirho "key $key_name_chirho: value absent from every file under the root" "$(scan_root_chirho "$(cat "$T_CHIRHO/$key_name_chirho-key-chirho.txt")" "$ROOT_CHIRHO")" 0
  expect_chirho "key $key_name_chirho: value absent from the serve log" "$(file_hits_chirho "$(cat "$T_CHIRHO/$key_name_chirho-key-chirho.txt")" "$T_CHIRHO/tenant-serve.log")" 0
done
expect_chirho "refused bodies' marker absent from every file under the root" "$(scan_root_chirho "$MARK_REFUSED_CHIRHO" "$ROOT_CHIRHO")" 0
expect_chirho "refused bodies' marker absent from the serve log" "$(file_hits_chirho "$MARK_REFUSED_CHIRHO" "$T_CHIRHO/tenant-serve.log")" 0
expect_chirho "no password in the root" "$(scan_root_chirho "$(cat "$T_CHIRHO/a-cred-chirho.txt")" "$ROOT_CHIRHO")" 0
expect_chirho "control: the MCP transcript scan sees the MCP save's name" "$(seen_verdict_chirho "$(scan_root_chirho "$MCP_MARK_CHIRHO" "$MCP_OUT_CHIRHO")")" sees
for key_name_chirho in a b; do
  expect_chirho "key $key_name_chirho: value absent from every MCP adapter's stdout and stderr" "$(scan_root_chirho "$(cat "$T_CHIRHO/$key_name_chirho-key-chirho.txt")" "$MCP_OUT_CHIRHO")" 0
done

say_chirho "VERDICT: $FAILS_CHIRHO failed assertion(s), $GAPS_CHIRHO known engine gap(s) unchanged; evidence in $T_CHIRHO"
[ "$FAILS_CHIRHO" -eq 0 ]
