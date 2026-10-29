#!/usr/bin/env bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
#
# Her assistant connected from her Account page (Cairn K4; gpt #26825, #26835), synthetic and local, on Cairn's hash-bound
# K4 Part 3 tools. Issuance is opened ONLY on this run's own scratch root, through the engine's configured `up` road with a
# scratch TOML (connection_issuance_enabled_chirho = true) and its pinned BLAKE3 digest; the dev `serve` road and main stay
# closed. The scratch app opens her page routes to limited keys and binds the Account route to the page bound for creating
# (deploy_chirho/held-bindings-chirho.toml, stage account-connect). Her figures are not opened, so they are never offered.
#   the engine's own form, in a real Chrome with no script: the page actions it offers, two chosen, her password again;
#   the key shown once, captured only into this run's mode-700 handoff directory and removed on every exit;
#   that key, by REST and through her own MCP adapter: her page read, a deliberate draft Save read back; the action she
#     did not choose, her figures, her Account page and Publishing refused;
#   her Account page lists the connection (app and actions in her words); another owner never sees it;
#   her Account page in Chrome at 1440 and 390, and Disconnect by her click on the page's own form: the key then refused
#     exactly as a made-up key, by REST and MCP; her browser, the other owner and his session unchanged;
#   the form and its one-time answer carry private, no-store, the CSP and the framing guards;
#   no key's or password's text in the driver and response evidence this run kept or in the engine's root, the handoff
#     and the temporary credential, cookie and key files excluded (a canary planted in each proves the reader); not a
#     claim about every file, binary encodings or OS buffers. Cleanup and Cairn's own custody proof are separate.
# The journey's start is chosen by the engine's own stamp, never by its answer (gpt #26841). On the explicit-id
# preparation tools listed in PREP_ENGINES_CHIRHO (3c7bc083, gpt #26835), which select a client by id only, the form is
# opened by the registered id and the Account page's link is a known gap. On ANY other engine the link is a hard
# expectation: the journey starts from the Account page's own link, returns by the created page's link, and wrong,
# missing, mixed or repeated selectors, hostile return paths and an ambiguous app are each refused. A successor that
# still answers the link with 400 fails.
# Env: CAIRN_CLI_CHIRHO + CANDIDATE_CAIRN_COMMIT_CHIRHO (+ CANDIDATE_CAIRN_SHA256_CHIRHO), TREE_BUILD_CHIRHO,
# CAIRN_REPO_CHIRHO, TENANT_PORT_CHIRHO (default 19340). Preconditions: prepare-apps-chirho.sh --app-chirho workspace.
# Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/agent-api-chirho.md
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
LC_ALL=C grep -a -q "connection-issuance-disabled-chirho" "$CLI_CHIRHO" \
  || { printf 'refusing: %s is not a K4 engine (it has no connection road)\n' "$CLI_CHIRHO" >&2; exit 2; }
TREE_BUILD_CHIRHO="${TREE_BUILD_CHIRHO:-$CAIRN_REPO_CHIRHO/target/debug/tree-build-chirho}"
LC_ALL=C grep -a -q "the app root cannot be opened to limited keys" "$TREE_BUILD_CHIRHO" \
  || { printf 'refusing: %s is not a K1 tree-build (it cannot lower key openings)\n' "$TREE_BUILD_CHIRHO" >&2; exit 2; }
command -v b3sum > /dev/null || { printf 'refusing: b3sum is needed to pin the scratch TOML digest\n' >&2; exit 2; }
APP_CHIRHO="$REPO_CHIRHO/apps-chirho/perffection-workspace-chirho"
if grep -q -E 'key_access_chirho|account_keys_chirho|account_connect_chirho' "$APP_CHIRHO/app-chirho.toml"; then
  printf 'refusing: app-chirho.toml already opens a key route or binds an own-keys page; this gate does that only in scratch\n' >&2; exit 2
fi
TENANT_CHIRHO=beta-chirho; TP_CHIRHO="${TENANT_PORT_CHIRHO:-19340}"; require_port_free_chirho "$TP_CHIRHO"
mkdir -p "$REPO_CHIRHO/target/rehearsal-chirho"
T_CHIRHO="$(mktemp -d "$REPO_CHIRHO/target/rehearsal-chirho/account-create.XXXX")"; ROOT_CHIRHO="$T_CHIRHO/root-chirho"
HANDOFF_CHIRHO="$T_CHIRHO/handoff-chirho"; SCRATCH_CHIRHO="$T_CHIRHO/workspace-scratch-chirho"; TPID_CHIRHO=""
# cleanup on every exit, set before any credential, cookie or key exists: the handoff directory goes whole
trap '[ -n "$TPID_CHIRHO" ] && kill "$TPID_CHIRHO" 2>/dev/null; wait 2>/dev/null; rm -rf "$HANDOFF_CHIRHO" "$SCRATCH_CHIRHO"; rm -f "$T_CHIRHO"/*-cred-chirho.txt "$T_CHIRHO"/*-key-chirho.txt "$T_CHIRHO"/*-cookie-chirho.txt' EXIT
mkdir -p "$ROOT_CHIRHO"; mkdir -m 700 "$HANDOFF_CHIRHO"
TB_CHIRHO="http://127.0.0.1:$TP_CHIRHO"; WS_CHIRHO="$TB_CHIRHO/app-chirho/perffection-workspace"
ACCOUNT_PATH_CHIRHO="/app-chirho/perffection-workspace/account-chirho"; FORM_ROUTE_CHIRHO="/auth-chirho/connection-new-chirho"
LOG_CHIRHO="$T_CHIRHO/journey-chirho.log"; : > "$LOG_CHIRHO"
# shellcheck source=../../lib-chirho/gate-helpers-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/gate-helpers-chirho.sh"
# shellcheck source=../../lib-chirho/key-staging-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/key-staging-chirho.sh"
cli_chirho() { "$CLI_CHIRHO" --root "$ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" "$@"; }
say_chirho "rehearsal dir $T_CHIRHO; engine $("$CLI_CHIRHO" --version 2>/dev/null | grep -oE '[0-9a-f]{40}' | head -1 | cut -c1-12); tree-build $(shasum -a 256 "$TREE_BUILD_CHIRHO" | cut -c1-12); product $(git -C "$REPO_CHIRHO" rev-parse --short=12 HEAD 2>/dev/null)"
# ---- the Back witness's own test first, needing no engine: a local no-store POST flow, both endings this gate trusts
# and the controls (back-check-chirho.test.ts, gpt #26841) ----
(cd "$REPO_CHIRHO" && bun test ./scripts-chirho/tests-chirho/account-chirho/) >> "$LOG_CHIRHO" 2>&1
expect_chirho "the Back witness's own test passes: Chrome's error page toward the POST answer, a fresh form from it, the controls" "$?" 0
account_chirho() { curl -s -o "$T_CHIRHO/$2" -D "$T_CHIRHO/$2.hdr" -w '%{http_code}' -H "Cookie: $(cat "$T_CHIRHO/$1-cookie-chirho.txt")" "$TB_CHIRHO$ACCOUNT_PATH_CHIRHO"; }
mcp_chirho() { bun "$REPO_CHIRHO/scripts-chirho/tests-chirho/figures-chirho/mcp-call-chirho.ts" "$TB_CHIRHO" "$1" "$2" "${3:+$T_CHIRHO/$3}" "$T_CHIRHO/$4" > "$T_CHIRHO/$4.log" 2>&1; }
mcp_field_chirho() { json_chirho "$1" "d[\"structured_chirho\"]$2"; }
# the engine's five guard headers on one kept answer, as name=value pairs
guards_chirho() { echo "$(header_chirho "$1" cache-control)|$(header_chirho "$1" content-security-policy | grep -c "frame-ancestors 'none'")$(header_chirho "$1" content-security-policy | grep -c "script-src 'none'")|$(header_chirho "$1" x-content-type-options)|$(header_chirho "$1" referrer-policy)|$(header_chirho "$1" x-frame-options)"; }
GUARDED_CHIRHO="private, no-store|11|nosniff|no-referrer|deny"

# ---- the root: owner, two users, the scratch app (page opened, Account bound for creating), a connection client ----
OWNER_CHIRHO="$(cli_chirho identity-register --kind owner | cut -f1)"
USER_A_CHIRHO="$(new_user_chirho a)"; USER_B_CHIRHO="$(new_user_chirho b)"
deploy_stage_chirho page,account-connect; expect_chirho "the engine accepts the page opened to keys and the Account route bound for creating" "$?" 0
CLIENT_CHIRHO="$(register_client_chirho client-register.out)"; expect_chirho "the app's client registered: an id read from its own answer" "$(printf '%s' "$CLIENT_CHIRHO" | grep -cE '^client/[0-9a-f]{16}$')" 1
limited_key_chirho b-page "$USER_B_CHIRHO" b page-read-chirho; expect_chirho "B's own page key issued (the operator road)" "$?" 0

# ---- issuance opened on this scratch root only, through `up` with its pinned TOML ----
printf 'root_chirho = "%s"\nport_chirho = %s\nconnection_issuance_enabled_chirho = true\n' "$ROOT_CHIRHO" "$TP_CHIRHO" > "$T_CHIRHO/cairn-chirho.toml"
DIGEST_CHIRHO="$(b3sum --no-names "$T_CHIRHO/cairn-chirho.toml")"
"$CLI_CHIRHO" up --config "$T_CHIRHO/cairn-chirho.toml" --expected-config-digest-chirho "$DIGEST_CHIRHO" --tenant "$TENANT_CHIRHO" > "$T_CHIRHO/up.log" 2>&1 & TPID_CHIRHO=$!
for _ in $(seq 1 120); do curl -s -o /dev/null "$TB_CHIRHO/healthz-chirho" && break; sleep 0.5; done
expect_chirho "the up road on $TP_CHIRHO is this run's" "$(own_server_verdict_chirho "$TPID_CHIRHO" "$TP_CHIRHO")" own
ENGINE_PRIVATE_CHIRHO="$(engine_private_answers_chirho)"; expect_chirho "the engine answers an unowned road private, no-store (K2)" "$ENGINE_PRIVATE_CHIRHO" 1
expect_chirho "A's browser signs in" "$(sign_in_chirho "$USER_A_CHIRHO" a login-a.hdr)" 303
expect_chirho "B's browser signs in" "$(sign_in_chirho "$USER_B_CHIRHO" b login-b.hdr)" 303
session_cookie_chirho login-a.hdr > "$T_CHIRHO/a-cookie-chirho.txt"; session_cookie_chirho login-b.hdr > "$T_CHIRHO/b-cookie-chirho.txt"; chmod 600 "$T_CHIRHO"/*-cookie-chirho.txt

# ---- her Account page offers the entry; the engine's stamp decides whether following it is required yet ----
PREP_ENGINES_CHIRHO="3c7bc083ea9ffe6c7fb3e89c35f4c539e5534da6"
STAMP_CHIRHO="$("$CLI_CHIRHO" --version 2>/dev/null | grep -oE '[0-9a-f]{40}' | head -1)"
case " $PREP_ENGINES_CHIRHO " in *" $STAMP_CHIRHO "*) LINK_MODE_CHIRHO=prepared ;; *) LINK_MODE_CHIRHO=link ;; esac
say_chirho "note the journey's start: $LINK_MODE_CHIRHO (engine ${STAMP_CHIRHO:0:12})"
expect_chirho "her Account page before: status/connections listed/the entry to the engine's form" "$(account_chirho a account-a-0.html)/$(count_chirho 'workspace-history-item-chirho' account-a-0.html)/$(count_chirho "$FORM_ROUTE_CHIRHO?app_chirho=perffection-workspace" account-a-0.html)" "200/0/1"
ENTRY_CHIRHO="$(python3 - "$T_CHIRHO/account-a-0.html" <<'PY'
import html, re, sys
page = open(sys.argv[1], encoding="utf-8").read()
links = [html.unescape(h) for h in re.findall(r'href="([^"]*)"', page) if html.unescape(h).startswith("/auth-chirho/connection-new-chirho")]
print(links[0] if links else "")
PY
)"
ENTRY_STATUS_CHIRHO="$(curl -s -o "$T_CHIRHO/entry-follow.html" -D "$T_CHIRHO/entry-follow.html.hdr" -w '%{http_code}' -H "Cookie: $(cat "$T_CHIRHO/a-cookie-chirho.txt")" "$TB_CHIRHO$ENTRY_CHIRHO")"
if [ "$LINK_MODE_CHIRHO" = link ]; then
  expect_chirho "her Account page's own link opens the engine's form: status/password fields" "$ENTRY_STATUS_CHIRHO/$(count_chirho 'name="credential_chirho"' entry-follow.html)" "200/1"
else
  gap_chirho "her Account page's own link opens the engine's form (Cairn's app selection, #26824)" "$ENTRY_STATUS_CHIRHO" 400
fi

# ---- the engine's own form, in a real Chrome with no script: from her Account page's link, or (preparation) by the
# registered client id; its own answer is read on the same road ----
ENC_CLIENT_CHIRHO="$(python3 -c 'import sys, urllib.parse; print(urllib.parse.quote(sys.argv[1], safe=""))' "$CLIENT_CHIRHO")"
FORM_PATH_CHIRHO="$FORM_ROUTE_CHIRHO?client_id_chirho=$ENC_CLIENT_CHIRHO"; START_CHIRHO="$FORM_PATH_CHIRHO"
[ "$LINK_MODE_CHIRHO" = link ] && { FORM_PATH_CHIRHO="$ENTRY_CHIRHO"; START_CHIRHO=link; }
expect_chirho "the form's own answer: status|guards" "$(curl -s -o "$T_CHIRHO/form.html" -D "$T_CHIRHO/form.html.hdr" -w '%{http_code}' -H "Cookie: $(cat "$T_CHIRHO/a-cookie-chirho.txt")" "$TB_CHIRHO$FORM_PATH_CHIRHO")|$(guards_chirho form.html)" "200|$GUARDED_CHIRHO"
bun "$HERE_CHIRHO/account-create-browser-chirho.ts" "$REPO_CHIRHO" "$TB_CHIRHO" "$T_CHIRHO" "$START_CHIRHO" "$HANDOFF_CHIRHO" page-read-chirho,page-save-chirho >> "$LOG_CHIRHO" 2>&1
expect_chirho "her Chrome, script off, ran the engine's form" "$?" 0
BS_CHIRHO=account-create-browser-summary-chirho.json
expect_chirho "the form offered her page actions only (no figures): actions/password fields" "$(json_chirho $BS_CHIRHO 'd["offered_actions_chirho"]')/$(json_chirho $BS_CHIRHO 'd["password_fields_chirho"]')" '["page-preview-chirho", "page-read-chirho", "page-save-chirho"]/1'
expect_chirho "created: status/a key's shape/once on the page/her password on it" "$(json_chirho $BS_CHIRHO 'd["created_status_chirho"]')/$(json_chirho $BS_CHIRHO 'd["key_shape_ok_chirho"]')/$(json_chirho $BS_CHIRHO 'd["key_times_on_page_chirho"]')/$(json_chirho $BS_CHIRHO 'd["password_on_page_chirho"]')" "200/true/1/false"
# Back: the state it leaves is examined, and only the ending measured for this start is trusted; any other ending,
# an unknown navigation error above all, fails (gpt #26841)
BACK_CHIRHO="$(json_chirho $BS_CHIRHO 'd["back_outcome_chirho"]')/$(json_chirho $BS_CHIRHO 'd["back_shows_key_chirho"]')"
if [ "$LINK_MODE_CHIRHO" = prepared ]; then
  expect_chirho "Back from the one-time answer, to the page before it: how it ended/the key shown/where/a fresh form's password fields" "$BACK_CHIRHO/$(json_chirho $BS_CHIRHO 'd["back_landed_chirho"]')/$(json_chirho $BS_CHIRHO 'd["back_landed_password_fields_chirho"]')" "navigated/false/$FORM_ROUTE_CHIRHO/1"
else
  expect_chirho "Back from her Account page toward the one-time answer (a no-store POST answer Chrome must not show again): how it ended/the key shown/where" "$BACK_CHIRHO/$(json_chirho $BS_CHIRHO 'd["back_landed_chirho"]')" "cache-miss/false/chrome-error://chromewebdata/"
fi
expect_chirho "the controls: the classifier/a Back that fails with the key's page in place/a Back with nowhere to go" "$(json_chirho $BS_CHIRHO 'd["back_classifier_control_chirho"]')/$(json_chirho $BS_CHIRHO 'd["failing_back_control_chirho"]["outcome_chirho"]'),$(json_chirho $BS_CHIRHO 'd["failing_back_control_chirho"]["shows_key_chirho"]')/$(json_chirho $BS_CHIRHO 'd["still_back_control_chirho"]["outcome_chirho"]'),$(json_chirho $BS_CHIRHO 'd["still_back_control_chirho"]["shows_key_chirho"]')" "true/unknown,true/stayed,true"
if [ "$LINK_MODE_CHIRHO" = prepared ]; then
  gap_chirho "the created page links back to her Account page (Cairn's app selection, #26824)" "$(json_chirho $BS_CHIRHO 'd["return_links_chirho"]')" 0
else
  expect_chirho "from the Account page's link and back by the created page's: entry links/form status/return links/where it returned/her live Disconnects there" "$(json_chirho $BS_CHIRHO 'd["entry_links_chirho"]')/$(json_chirho $BS_CHIRHO 'd["form_status_chirho"]')/$(json_chirho $BS_CHIRHO 'd["return_links_chirho"]')/$(json_chirho $BS_CHIRHO 'd["returned_path_chirho"]')/$(json_chirho $BS_CHIRHO 'd["returned_disconnects_chirho"]')" "1/200/1/$ACCOUNT_PATH_CHIRHO/1"
fi
KEY_FILE_CHIRHO="$HANDOFF_CHIRHO/connection-key-chirho.txt"
expect_chirho "the one-time answer's own guards, as Chrome received them" "$(json_chirho $BS_CHIRHO 'd["created_guards_chirho"]')" "$GUARDED_CHIRHO"
expect_chirho "the key reached only the handoff: directory/file modes" "$(stat -f '%Lp' "$HANDOFF_CHIRHO")/$(stat -f '%Lp' "$KEY_FILE_CHIRHO" 2>/dev/null)" "700/600"
cp "$KEY_FILE_CHIRHO" "$HANDOFF_CHIRHO/conn-key-chirho.txt"; ln -s "$HANDOFF_CHIRHO/conn-key-chirho.txt" "$T_CHIRHO/conn-key-chirho.txt"

# ---- that key: her page by REST and through her own MCP adapter; everything she did not choose refused ----
expect_chirho "the connection reads her page (REST): status/state" "$(call_chirho conn GET agent-page-chirho conn-read-0.json)/$(json_chirho conn-read-0.json 'd["state_chirho"]')" "200/nothing-saved-chirho"
expect_chirho "the action she did not choose (preview)" "$(call_chirho conn GET preview-chirho conn-preview.json)|$(cat "$T_CHIRHO/conn-preview.json")" '403|{"error_chirho":"key-action-not-granted-chirho","ok_chirho":false}'
expect_chirho "her figures, never opened to keys" "$(call_chirho conn GET agent-figures-chirho conn-figures.json)|$(cat "$T_CHIRHO/conn-figures.json")" '403|{"error_chirho":"key-route-not-open-chirho","ok_chirho":false}'
expect_chirho "her Account page and Publishing, never opened to keys" "$(call_chirho conn GET account-chirho conn-account.json)/$(call_chirho conn GET publication-status-chirho conn-publication.json)" "403/403"
mcp_chirho "$KEY_FILE_CHIRHO" read_page_chirho "" mcp-read-0.json
expect_chirho "MCP with the same key: offered/error/state/version" "$(json_chirho mcp-read-0.json 'd["offered_chirho"]')/$(json_chirho mcp-read-0.json 'd["is_error_chirho"]')/$(mcp_field_chirho mcp-read-0.json '["state_chirho"]')/$(mcp_field_chirho mcp-read-0.json '["current_version_chirho"]')" "true/false/nothing-saved-chirho/0"
python3 - "$T_CHIRHO/mcp-read-0.json" "$T_CHIRHO/save-body.json" <<'PY'
import json, sys
read = json.load(open(sys.argv[1], encoding="utf-8"))["structured_chirho"]
page = read["surface_chirho"]["sample_page_chirho"]
json.dump({"expected_version_chirho": read["current_version_chirho"], "page_chirho": page}, open(sys.argv[2], "w", encoding="utf-8"))
PY
mcp_chirho "$KEY_FILE_CHIRHO" save_page_chirho save-body.json mcp-save-1.json
expect_chirho "MCP: her deliberate draft Save: error/saved/version" "$(json_chirho mcp-save-1.json 'd["is_error_chirho"]')/$(mcp_field_chirho mcp-save-1.json '["saved_chirho"]')/$(mcp_field_chirho mcp-save-1.json '["current_version_chirho"]')" "false/true/1"
expect_chirho "read back with the connection's key: the page's version" "$(call_chirho conn GET agent-page-chirho conn-read-1.json > /dev/null; json_chirho conn-read-1.json 'd["current_version_chirho"]')" 1

# ---- her Account list: the connection in her words; the other owner never sees it ----
curl -s -o "$T_CHIRHO/keys-a.json" -H "Cookie: $(cat "$T_CHIRHO/a-cookie-chirho.txt")" "$TB_CHIRHO/auth-chirho/keys-chirho"
CONN_ID_CHIRHO="$(python3 - "$T_CHIRHO/keys-a.json" <<'PY'
import json, sys
rows = json.load(open(sys.argv[1], encoding="utf-8"))
rows = rows.get("keys_chirho", rows) if isinstance(rows, dict) else rows
ids = [row["key_id_chirho"] for row in rows if row["scopes_chirho"] == ["page-read-chirho", "page-save-chirho"]]
print(ids[0] if len(ids) == 1 else f"not-exactly-one:{len(ids)}")
PY
)"
expect_chirho "her Account page after: status/live Disconnects/her connection's form/the entry still offered" "$(account_chirho a account-a-1.html)/$(grep -o 'action="/auth-chirho/key-revoke-chirho"' "$T_CHIRHO/account-a-1.html" | wc -l | tr -d ' ')/$(count_chirho "value=\"$CONN_ID_CHIRHO\"" account-a-1.html)/$(count_chirho "$FORM_ROUTE_CHIRHO?app_chirho" account-a-1.html)" "200/1/1/1"
expect_chirho "B's Account page: status/A's connection" "$(account_chirho b account-b-1.html)/$(count_chirho "$CONN_ID_CHIRHO" account-b-1.html)" "200/0"
if [ "$LINK_MODE_CHIRHO" = link ]; then
  expect_chirho "the list the created page's link returned her to renders her new connection's own Disconnect, and only it" "$([ "$(json_chirho $BS_CHIRHO 'd["returned_key_ids_chirho"]')" = "[\"$CONN_ID_CHIRHO\"]" ] && echo her-new-connection-only || echo "other: $(json_chirho $BS_CHIRHO 'len(d["returned_key_ids_chirho"])') forms")" her-new-connection-only
fi

# ---- Disconnect by her click on the page's own form, in Chrome with no script (account-browser-chirho.ts): her Account
# page with the entry at 1440 and 390 first; then the key is refused exactly as a made-up key ----
bun "$HERE_CHIRHO/account-browser-chirho.ts" "$REPO_CHIRHO" "$TB_CHIRHO" "$T_CHIRHO" >> "$LOG_CHIRHO" 2>&1
expect_chirho "her Chrome, script off, opened her Account page and pressed Disconnect" "$?" 0
AB_CHIRHO=account-browser-summary-chirho.json
expect_chirho "her Account page in Chrome: sideways overflow at 1440/at 390/live Disconnects" "$(json_chirho $AB_CHIRHO 'd["overflow_1440_chirho"]')/$(json_chirho $AB_CHIRHO 'd["overflow_390_chirho"]')/$(json_chirho $AB_CHIRHO 'd["before_chirho"]["disconnect_buttons_chirho"]')" "false/false/1"
expect_chirho "her click on Disconnect: whose form it was/where she landed/the notice shown/live Disconnects/ended rows" "$([ "$(json_chirho $AB_CHIRHO 'd["clicked_key_id_chirho"]')" = "$CONN_ID_CHIRHO" ] && echo her-new-connection || echo another)/$(json_chirho $AB_CHIRHO 'd["after_chirho"]["path_chirho"]')$(json_chirho $AB_CHIRHO 'd["after_chirho"]["fragment_chirho"]')/$(json_chirho $AB_CHIRHO 'd["after_chirho"]["notice_visible_chirho"]')/$(json_chirho $AB_CHIRHO 'd["after_chirho"]["disconnect_buttons_chirho"]')/$(json_chirho $AB_CHIRHO 'd["after_chirho"]["ended_rows_chirho"]')" "her-new-connection/$ACCOUNT_PATH_CHIRHO#notice-key-ended-chirho/true/0/1"
openssl rand -hex 32 > "$T_CHIRHO/made-up-key-chirho.txt"
expect_chirho "the ended connection, refused exactly as a made-up key: status and answer" "$(call_chirho conn GET agent-page-chirho conn-ended.json)|$(shasum -a 256 < "$T_CHIRHO/conn-ended.json" | cut -c1-16)" "$(call_chirho made-up GET agent-page-chirho made-up.json)|$(shasum -a 256 < "$T_CHIRHO/made-up.json" | cut -c1-16)"
mcp_chirho "$KEY_FILE_CHIRHO" read_page_chirho "" mcp-read-ended.json
expect_chirho "MCP with the ended key: error/outcome" "$(json_chirho mcp-read-ended.json 'd["is_error_chirho"]')/$(mcp_field_chirho mcp-read-ended.json '["adapter_outcome_chirho"]')" "true/no-answer-chirho"
expect_chirho "unchanged: A's browser/B's browser/B's own key" "$(account_chirho a account-a-2.html)/$(account_chirho b account-b-2.html)/$(call_chirho b-page GET agent-page-chirho b-read.json)" "200/200/200"
expect_chirho "her Account page after the end: live Disconnects/an ended row" "$(grep -o 'action="/auth-chirho/key-revoke-chirho"' "$T_CHIRHO/account-a-2.html" | wc -l | tr -d ' ')/$(count_chirho 'workspace-connections-ended-row-chirho' account-a-2.html)" "0/1"
# ---- with app selection: what the engine must refuse before any form is shown (gpt #26824) ----
if [ "$LINK_MODE_CHIRHO" = link ]; then
  APP_SEL_CHIRHO="app_chirho=perffection-workspace"
  # selector_answer_chirho <label> <query>, signed in: "<status>/<the engine's request-invalid answer>/<password fields>".
  # The reason is asserted: a refusal is the engine's own 400 request-invalid page with no form, never any non-200.
  selector_answer_chirho() { local status_chirho; status_chirho="$(curl -s -o "$T_CHIRHO/selector-$1.html" -w '%{http_code}' -H "Cookie: $(cat "$T_CHIRHO/a-cookie-chirho.txt")" "$TB_CHIRHO$FORM_ROUTE_CHIRHO?$2")"
    printf '%s' "$status_chirho/$(count_chirho 'id="request-invalid-chirho"' "selector-$1.html")/$(count_chirho 'name="credential_chirho"' "selector-$1.html")"; }
  # the selectors and returns Cairn's app selection declares refused (#26836, #26837): exactly one well-formed selector,
  # at most one return, and a return exactly /app-chirho/<her app>/<route>, unreserved characters only, untrimmed
  RET_CHIRHO="$APP_SEL_CHIRHO&return_chirho="
  # the controls: the same reader sees her form for the app alone and for the app with her Account page as the return
  expect_chirho "the controls, answered with her form: the app alone/with her Account page as the return" "$(selector_answer_chirho app-alone "$APP_SEL_CHIRHO"),$(selector_answer_chirho app-return "${RET_CHIRHO}%2Fapp-chirho%2Fperffection-workspace%2Faccount-chirho")" "200/0/1,200/0/1"
  for case_chirho in "wrong-app|app_chirho=no-such-app-chirho" "no-selector|" "mixed-selectors|$APP_SEL_CHIRHO&client_id_chirho=$ENC_CLIENT_CHIRHO" \
    "repeated-app|$APP_SEL_CHIRHO&$APP_SEL_CHIRHO" "unknown-field|$APP_SEL_CHIRHO&next_chirho=x" \
    "repeated-return|${RET_CHIRHO}%2Fapp-chirho%2Fperffection-workspace%2Faccount-chirho&return_chirho=%2Fapp-chirho%2Fperffection-workspace%2Faccount-chirho" \
    "foreign-return|${RET_CHIRHO}https%3A%2F%2Fevil.example%2F" "protocol-relative-return|${RET_CHIRHO}%2F%2Fevil.example%2F" \
    "backslash-return|${RET_CHIRHO}%2F%5Cevil.example%2F" "script-return|${RET_CHIRHO}javascript%3Aalert(1)" \
    "other-app-return|${RET_CHIRHO}%2Fapp-chirho%2Fother-app-chirho%2Faccount-chirho" "dot-segment-return|${RET_CHIRHO}%2Fapp-chirho%2Fperffection-workspace%2F.." \
    "query-return|${RET_CHIRHO}%2Fapp-chirho%2Fperffection-workspace%2Faccount-chirho%3Fx%3D1" "padded-return|${RET_CHIRHO}%20%2Fapp-chirho%2Fperffection-workspace%2Faccount-chirho"; do
    expect_chirho "refused before any form: ${case_chirho%%|*}" "$(selector_answer_chirho "${case_chirho%%|*}" "${case_chirho#*|}")" "400/1/0"
  done
  # an ambiguous app: a second connection client for it, and app selection must not choose between them
  cli_chirho oauth-client-register --app perffection-workspace --owner "$OWNER_CHIRHO" > "$T_CHIRHO/client-register-2.out" 2>&1
  expect_chirho "a second client registered for the app: status/the connection store's verified clients" "$?/$(cli_chirho oauth-connection-verify 2>&1 | grep -oE 'verified [0-9]+ connection client' | grep -oE '[0-9]+')" 0/2
  expect_chirho "refused before any form: an app with two connection clients" "$(selector_answer_chirho ambiguous "$APP_SEL_CHIRHO")" "400/1/0"
fi
kill "$TPID_CHIRHO" 2>/dev/null; wait 2>/dev/null; TPID_CHIRHO=""

# ---- no key's or password's text in the run's kept evidence or the engine's root (the temporary secret files excluded) ----
# Positive controls: a random canary planted in the kept evidence and one in the engine's root, each found by the same reader.
CANARY_E_CHIRHO="$(openssl rand -hex 32)"; CANARY_R_CHIRHO="$(openssl rand -hex 32)"
printf '<p>%s</p>\n' "$CANARY_E_CHIRHO" > "$T_CHIRHO/canary-evidence-chirho.html"; printf '%s\n' "$CANARY_R_CHIRHO" > "$ROOT_CHIRHO/canary-root-chirho.txt"
SCAN_CHIRHO="$(python3 - "$T_CHIRHO" "$HANDOFF_CHIRHO" "$ROOT_CHIRHO" "$CANARY_E_CHIRHO" "$CANARY_R_CHIRHO" <<'PY'
import pathlib, sys
root, handoff, engine_root = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2]), pathlib.Path(sys.argv[3])
secrets = [p.read_text().strip() for p in handoff.glob("*-chirho.txt")]
secrets += [p.read_text().strip() for p in root.glob("*-cred-chirho.txt")]
secrets += [p.read_text().strip() for p in root.glob("*-key-chirho.txt") if not p.is_symlink() and p.name != "made-up-key-chirho.txt"]
secrets = [s for s in secrets if s]
kept = [p for p in root.rglob("*") if p.is_file() and not p.is_symlink() and handoff not in p.parents
        and not p.name.endswith(("-cred-chirho.txt", "-cookie-chirho.txt", "-key-chirho.txt"))]
texts = {p: p.read_text(errors="replace") for p in kept}
evidence = [p for p in kept if engine_root not in p.parents]
engine = [p for p in kept if engine_root in p.parents]
found = sum(t.count(s) for t in texts.values() for s in secrets)
canary_e = sum(texts[p].count(sys.argv[4]) for p in evidence)
canary_r = sum(texts[p].count(sys.argv[5]) for p in engine)
print(f"{len(secrets)}/{len(evidence) > 10}/{len(engine) > 0}/{found}/{canary_e}/{canary_r}")
PY
)"
expect_chirho "no key's or password's text in the kept driver and response evidence or the engine's root (the handoff and temporary secret files excluded): secrets/evidence files/root files/found/canaries found in evidence/in the root" "$SCAN_CHIRHO" "5/True/True/0/1/1"
rm -rf "$HANDOFF_CHIRHO"; rm -f "$T_CHIRHO/conn-key-chirho.txt"
expect_chirho "the handoff directory is gone" "$([ -e "$HANDOFF_CHIRHO" ] && echo present || echo gone)" gone
say_chirho "VERDICT: $FAILS_CHIRHO failed assertion(s), $GAPS_CHIRHO known engine gap(s); evidence in $T_CHIRHO"
[ "$FAILS_CHIRHO" -eq 0 ]
