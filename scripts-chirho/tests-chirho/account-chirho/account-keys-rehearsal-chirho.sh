#!/usr/bin/env bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
#
# Her AI connections on the real Account page (gpt #26666, #26699), synthetic and local, on Cairn's K2+K3 tools
# (1f45c632, the hash-bound handoff #26703). The Account route stays HELD on main: this gate binds it to the
# own-keys read (deploy_chirho/held-bindings-chirho.toml, stage "account") in a scratch mirror of the workspace
# app, with the page and figures routes opened to limited keys, and deploys it to one fresh tenant root. Then,
# with two owners signed in to real browser sessions:
#   her list on the page: each row names its app and what it may do, never a key; a Disconnect only for a live
#     row; no control that creates a connection; another owner's keys never shown;
#   Disconnect, by the page's own form: answered to its notice, the same again; the ended key refused exactly as
#     a made-up one; her browser, her other key and the other owner's key unchanged; another owner cannot end
#     hers; a bad form and no session answered as the engine's own fragments;
#   every private answer the product makes leaves private, no-store (Cairn K2): the Account page and its
#     Disconnect, her figures page, review and Save, and her assistant's figures read;
#   her real Chrome, script off: the page at 1440 and 390, her click on Disconnect, the notice the engine's
#     fragment reveals, and the key refused;
#   the page bound for creating too (stage account-connect) shows the same list plus one link to the engine's own
#     form for this app, returning here; the list-only page never does;
#   a key record that fails verification makes the list unavailable, never "none", with no link to create.
# Saved state is read back; nothing is inferred from request or use records. On an engine with Cairn's shared
# absence (4c7539df: in 3c7bc083 and ad4bc794, not in the K2+K3 tools 1f45c632) a log the engine cannot examine
# answers unavailable, never an empty list. The residual is a log deleted before the engine first saw it, which still
# reads as absent (record completeness, Cairn's open question). Local only: no host, pin, installed tool, customer key
# or activation; no create control.
# Env: CAIRN_CLI_CHIRHO + CANDIDATE_CAIRN_COMMIT_CHIRHO (+ CANDIDATE_CAIRN_SHA256_CHIRHO), TREE_BUILD_CHIRHO,
# CAIRN_REPO_CHIRHO, TENANT_PORT_CHIRHO (default 19320). Preconditions: prepare-apps-chirho.sh --app-chirho workspace.
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
LC_ALL=C grep -a -q "the owner's keys are read only by a browser session, never by a key" "$CLI_CHIRHO" \
  || { printf 'refusing: %s is not a K3 engine (it has no own-keys read)\n' "$CLI_CHIRHO" >&2; exit 2; }
TREE_BUILD_CHIRHO="${TREE_BUILD_CHIRHO:-$CAIRN_REPO_CHIRHO/target/debug/tree-build-chirho}"
LC_ALL=C grep -a -q "the app root cannot be opened to limited keys" "$TREE_BUILD_CHIRHO" \
  || { printf 'refusing: %s is not a K1 tree-build (it cannot lower key openings)\n' "$TREE_BUILD_CHIRHO" >&2; exit 2; }
APP_CHIRHO="$REPO_CHIRHO/apps-chirho/perffection-workspace-chirho"
if grep -q -E 'figures-chirho|key_access_chirho|account_keys_chirho' "$APP_CHIRHO/app-chirho.toml"; then
  printf 'refusing: app-chirho.toml already routes figures, opens a key route or binds the own-keys read; this gate does that only in scratch\n' >&2; exit 2
fi
TENANT_CHIRHO=beta-chirho; TP_CHIRHO="${TENANT_PORT_CHIRHO:-19320}"; require_port_free_chirho "$TP_CHIRHO"
mkdir -p "$REPO_CHIRHO/target/rehearsal-chirho"
T_CHIRHO="$(mktemp -d "$REPO_CHIRHO/target/rehearsal-chirho/account-keys.XXXX")"; ROOT_CHIRHO="$T_CHIRHO/root-chirho"; mkdir -p "$ROOT_CHIRHO"
TB_CHIRHO="http://127.0.0.1:$TP_CHIRHO"; WS_CHIRHO="$TB_CHIRHO/app-chirho/perffection-workspace"
ACCOUNT_PATH_CHIRHO="/app-chirho/perffection-workspace/account-chirho"; REVOKE_CHIRHO="$TB_CHIRHO/auth-chirho/key-revoke-chirho"
LOG_CHIRHO="$T_CHIRHO/journey-chirho.log"; : > "$LOG_CHIRHO"
# shellcheck source=../../lib-chirho/gate-helpers-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/gate-helpers-chirho.sh"
# shellcheck source=../../lib-chirho/key-staging-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/key-staging-chirho.sh"
cli_chirho() { "$CLI_CHIRHO" --root "$ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" "$@"; }
say_chirho "rehearsal dir $T_CHIRHO; engine $("$CLI_CHIRHO" --version 2>/dev/null | grep -oE '[0-9a-f]{40}' | head -1 | cut -c1-12); tree-build $(shasum -a 256 "$TREE_BUILD_CHIRHO" | cut -c1-12); product $(git -C "$REPO_CHIRHO" rev-parse --short=12 HEAD 2>/dev/null)"
SCRATCH_CHIRHO="$T_CHIRHO/workspace-scratch-chirho"; TPID_CHIRHO=""
trap '[ -n "$TPID_CHIRHO" ] && kill "$TPID_CHIRHO" 2>/dev/null; wait 2>/dev/null; rm -rf "$SCRATCH_CHIRHO"; rm -f "$T_CHIRHO"/*-cred-chirho.txt "$T_CHIRHO"/*-key-chirho.txt "$T_CHIRHO"/*-cookie-chirho.txt' EXIT
# label_chirho <key> [<key>...]: the catalog's own words, walked by key as data, so no arm pins a sentence
label_chirho() { python3 - "$REPO_CHIRHO/crates-chirho/perffection-ui-chirho/locales_chirho/workspace-chirho/en-chirho.json" "$@" <<'PY'
import json, sys
value = json.load(open(sys.argv[1], encoding="utf-8"))["connections_chirho"]
for key in sys.argv[2:]:
    value = value[key]
print(value)
PY
}
# account_chirho <cookie name> <out>: the Account page in her browser; status
account_chirho() { curl -s -o "$T_CHIRHO/$2" -D "$T_CHIRHO/$2.hdr" -w '%{http_code}' -H "Cookie: $(cat "$T_CHIRHO/$1-cookie-chirho.txt")" "$TB_CHIRHO$ACCOUNT_PATH_CHIRHO"; }
# disconnect_chirho <cookie name or ""> <form body> <out>: the page's own Disconnect form, posted; "status|location"
disconnect_chirho() {
  local cookie_args_chirho=(); [ -n "$1" ] && cookie_args_chirho=(-H "Cookie: $(cat "$T_CHIRHO/$1-cookie-chirho.txt")")
  local status_chirho
  status_chirho="$(curl -s -o "$T_CHIRHO/$3" -D "$T_CHIRHO/$3.hdr" -w '%{http_code}' -H 'Content-Type: application/x-www-form-urlencoded' \
    ${cookie_args_chirho[@]+"${cookie_args_chirho[@]}"} --data-binary "$2" "$REVOKE_CHIRHO")"
  printf '%s|%s' "$status_chirho" "$(header_chirho "$3" location)"
}
forms_to_chirho() { grep -o "action=\"$1\"" "$T_CHIRHO/$2" | wc -l | tr -d ' '; }
# in_list_chirho <text> <file>: how often <text> appears inside her connections list itself, never elsewhere
in_list_chirho() { python3 - "$T_CHIRHO/$2" "$1" "$(label_chirho list_label_chirho)" <<'PY'
import html, sys
page = open(sys.argv[1], encoding="utf-8").read()
start = page.find(f'aria-label="{html.escape(sys.argv[3])}"')
region = page[start:page.find("</ul>", start)] if start >= 0 else ""
print(region.count(html.escape(sys.argv[2])))
PY
}
key_id_of_chirho() { python3 - "$T_CHIRHO/$1" "$2" <<'PY'
import json, sys
rows = json.load(open(sys.argv[1], encoding="utf-8"))
rows = rows.get("keys_chirho", rows) if isinstance(rows, dict) else rows
ids = [row["key_id_chirho"] for row in rows if ",".join(row["scopes_chirho"]) == sys.argv[2]]
print(ids[0] if len(ids) == 1 else f"not-exactly-one:{len(ids)}")
PY
}

# ---- the root: owner, two users, the scratch app with the Account route bound, limited keys ----
OWNER_CHIRHO="$(cli_chirho identity-register --kind owner | cut -f1)"
USER_A_CHIRHO="$(new_user_chirho a)"; USER_B_CHIRHO="$(new_user_chirho b)"
deploy_stage_chirho page,figures,account; expect_chirho "the engine accepts the Account route holding the own-keys read, beside routes opened to keys" "$?" 0
CLIENT_CHIRHO="$(register_client_chirho client-register.out)"; expect_chirho "OAuth client registered: an id read from its own answer" "$(printf '%s' "$CLIENT_CHIRHO" | grep -cE '^client/[0-9a-f]{16}$')" 1
limited_key_chirho a-page "$USER_A_CHIRHO" a page-read-chirho page-save-chirho page-preview-chirho; expect_chirho "A's page key issued" "$?" 0
limited_key_chirho a-figures "$USER_A_CHIRHO" a figures-read-chirho figures-save-chirho; expect_chirho "A's figures key issued" "$?" 0
limited_key_chirho b-figures "$USER_B_CHIRHO" b figures-read-chirho figures-save-chirho; expect_chirho "B's figures key issued" "$?" 0
# start_server_chirho <label>: the engine invoked directly (so $! is its own pid), then confirmed as this run's
start_server_chirho() {
  "$CLI_CHIRHO" --root "$ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" serve --port "$TP_CHIRHO" >> "$T_CHIRHO/tenant-serve.log" 2>&1 & TPID_CHIRHO=$!
  for _ in $(seq 1 80); do curl -s -o /dev/null "$TB_CHIRHO/auth-chirho/login-chirho" && break; sleep 0.5; done
  expect_chirho "$1: the server on $TP_CHIRHO is this run's" "$(own_server_verdict_chirho "$TPID_CHIRHO" "$TP_CHIRHO")" own
}
start_server_chirho "list-bound"
ENGINE_PRIVATE_CHIRHO="$(engine_private_answers_chirho)"
expect_chirho "the engine answers an unowned road private, no-store (Cairn K2's layer)" "$ENGINE_PRIVATE_CHIRHO" 1
expect_chirho "A's browser signs in" "$(sign_in_chirho "$USER_A_CHIRHO" a login-a.hdr)" 303
expect_chirho "B's browser signs in" "$(sign_in_chirho "$USER_B_CHIRHO" b login-b.hdr)" 303
session_cookie_chirho login-a.hdr > "$T_CHIRHO/a-cookie-chirho.txt"; session_cookie_chirho login-b.hdr > "$T_CHIRHO/b-cookie-chirho.txt"; chmod 600 "$T_CHIRHO"/*-cookie-chirho.txt

# ---- her list: the engine's JSON road names the ids; the page shows the same rows, in her words ----
expect_chirho "A's key list (JSON road): status" "$(curl -s -o "$T_CHIRHO/keys-a.json" -w '%{http_code}' -H "Cookie: $(cat "$T_CHIRHO/a-cookie-chirho.txt")" "$TB_CHIRHO/auth-chirho/keys-chirho")" 200
A_PAGE_ID_CHIRHO="$(key_id_of_chirho keys-a.json page-preview-chirho,page-read-chirho,page-save-chirho)"; A_FIGURES_ID_CHIRHO="$(key_id_of_chirho keys-a.json figures-read-chirho,figures-save-chirho)"
IDS_FOUND_CHIRHO=yes; case "$A_PAGE_ID_CHIRHO$A_FIGURES_ID_CHIRHO" in *not-exactly*) IDS_FOUND_CHIRHO=no ;; esac
expect_chirho "a limited key's scopes are its sorted action names: page/figures ids found" "$IDS_FOUND_CHIRHO" yes
curl -s -o "$T_CHIRHO/keys-b.json" -H "Cookie: $(cat "$T_CHIRHO/b-cookie-chirho.txt")" "$TB_CHIRHO/auth-chirho/keys-chirho"; B_FIGURES_ID_CHIRHO="$(key_id_of_chirho keys-b.json figures-read-chirho,figures-save-chirho)"
expect_chirho "A's Account page: status/private, no-store/Disconnect forms/her two ids/B's id" "$(account_chirho a account-a-0.html)/$(no_store_chirho account-a-0.html)/$(forms_to_chirho /auth-chirho/key-revoke-chirho account-a-0.html)/$(count_chirho "value=\"$A_PAGE_ID_CHIRHO\"" account-a-0.html)$(count_chirho "value=\"$A_FIGURES_ID_CHIRHO\"" account-a-0.html)/$(count_chirho "$B_FIGURES_ID_CHIRHO" account-a-0.html)" "200/1/1/2/11/0"
expect_chirho "each row names its app and what it may do, in her list: workspace/read figures/save figures/read page" "$(in_list_chirho "$(label_chirho app_labels_chirho perffection-workspace)" account-a-0.html)/$(in_list_chirho "$(label_chirho action_labels_chirho figures-read-chirho)" account-a-0.html)/$(in_list_chirho "$(label_chirho action_labels_chirho figures-save-chirho)" account-a-0.html)/$(in_list_chirho "$(label_chirho action_labels_chirho page-read-chirho)" account-a-0.html)" "2/1/1/1"
expect_chirho "the list-only page leads nowhere to create one: links to the engine's connection form" "$(count_chirho 'connection-new-chirho' account-a-0.html)" 0
expect_chirho "no control creates a connection: every form posts to her password road or the Disconnect road" "$(grep -o 'action="[^"]*"' "$T_CHIRHO/account-a-0.html" | sort | uniq -c | tr -s ' ' | tr '\n' ';')" " 1 action=\"/auth-chirho/credential-change-chirho\"; 2 action=\"/auth-chirho/key-revoke-chirho\";"
# her key list is a browser session's alone: the route holding it is never opened to keys
expect_chirho "a key of hers cannot open her Account page: status|answer" "$(call_chirho a-page GET account-chirho key-on-account.json)|$(cat "$T_CHIRHO/key-on-account.json")" '403|{"error_chirho":"key-route-not-open-chirho","ok_chirho":false}'
expect_chirho "B's Account page: status/Disconnect forms/B's id/A's ids" "$(account_chirho b account-b-0.html)/$(forms_to_chirho /auth-chirho/key-revoke-chirho account-b-0.html)/$(count_chirho "value=\"$B_FIGURES_ID_CHIRHO\"" account-b-0.html)/$(count_chirho "$A_PAGE_ID_CHIRHO" account-b-0.html)$(count_chirho "$A_FIGURES_ID_CHIRHO" account-b-0.html)" "200/1/1/00"

# ---- K2: every private answer the product makes leaves private, no-store ----
printf 'month,kind,money_in,costs,jobs\n2026-01,Weddings,1200.50,400,3\n' > "$T_CHIRHO/a.csv"
python3 - "$T_CHIRHO/a.csv" > "$T_CHIRHO/a-figures-form.txt" <<'PY'
import sys, urllib.parse
print(urllib.parse.urlencode([("csv_chirho", open(sys.argv[1], encoding="utf-8").read()), ("currency_chirho", "USD"), ("note_chirho", "synthetic"), ("expected_version_chirho", "0")]), end="")
PY
browser_chirho() { curl -s -o "$T_CHIRHO/$3" -D "$T_CHIRHO/$3.hdr" -w '%{http_code}' -H "Cookie: $(cat "$T_CHIRHO/a-cookie-chirho.txt")" ${4:+-H 'Content-Type: application/x-www-form-urlencoded' --data-binary "@$T_CHIRHO/$4"} -X "$1" "$WS_CHIRHO/$2"; }
browser_chirho GET figures-chirho figures-a.html > /dev/null; browser_chirho POST review-figures-chirho review-a.html a-figures-form.txt > /dev/null; browser_chirho POST save-figures-chirho save-a.html a-figures-form.txt > /dev/null
call_chirho b-figures GET agent-figures-chirho agent-b.json > /dev/null
expect_chirho "private, no-store: her figures page/review/Save/her assistant's read" "$(no_store_chirho figures-a.html)|$(no_store_chirho review-a.html)|$(no_store_chirho save-a.html)|$(no_store_chirho agent-b.json)" "1/1|1/1|1/1|1/1"
expect_chirho "her Save stored her figures: the page now shows version" "$(browser_chirho GET figures-chirho figures-a-1.html > /dev/null; grep -oE 'name="expected_version_chirho" value="[0-9]+"' "$T_CHIRHO/figures-a-1.html" | head -1 | grep -oE '[0-9]+')" 1

# ---- Disconnect, by the page's own form ----
ENDED_CHIRHO="$ACCOUNT_PATH_CHIRHO#notice-key-ended-chirho"
expect_chirho "A ends her figures key: status|where it sends her" "$(disconnect_chirho a "key_id_chirho=$A_FIGURES_ID_CHIRHO&redirect_chirho=$ACCOUNT_PATH_CHIRHO" end-1.txt)" "303|$ENDED_CHIRHO"
expect_chirho "the Disconnect answer is private, no-store" "$(header_chirho end-1.txt cache-control)" "private, no-store"
expect_chirho "the same Disconnect again: status|where it sends her" "$(disconnect_chirho a "key_id_chirho=$A_FIGURES_ID_CHIRHO&redirect_chirho=$ACCOUNT_PATH_CHIRHO" end-2.txt)" "303|$ENDED_CHIRHO"
openssl rand -hex 32 > "$T_CHIRHO/made-up-key-chirho.txt"; chmod 600 "$T_CHIRHO/made-up-key-chirho.txt"
expect_chirho "the ended key is refused exactly as a made-up key: status and answer" "$(call_chirho a-figures GET agent-figures-chirho ended-read.json)|$(shasum -a 256 < "$T_CHIRHO/ended-read.json" | cut -c1-16)" "$(call_chirho made-up GET agent-figures-chirho made-up-read.json)|$(shasum -a 256 < "$T_CHIRHO/made-up-read.json" | cut -c1-16)"
expect_chirho "the control: a made-up key is refused" "$(refused_chirho "$(call_chirho made-up GET agent-figures-chirho made-up-read-2.json)")" refused
expect_chirho "unchanged: A's page key/A's browser/B's key" "$(call_chirho a-page GET agent-page-chirho a-page-read.json)/$(account_chirho a account-a-1.html)/$(call_chirho b-figures GET agent-figures-chirho b-read.json)" "200/200/200"
expect_chirho "A's page after: Disconnect forms/the page key's form/the ended key's form/an ended row" "$(forms_to_chirho /auth-chirho/key-revoke-chirho account-a-1.html)/$(count_chirho "value=\"$A_PAGE_ID_CHIRHO\"" account-a-1.html)/$(count_chirho "value=\"$A_FIGURES_ID_CHIRHO\"" account-a-1.html)/$(count_chirho 'workspace-connections-ended-row-chirho' account-a-1.html)" "1/1/0/1"
expect_chirho "B cannot end A's key: status|where it sends B" "$(disconnect_chirho b "key_id_chirho=$A_PAGE_ID_CHIRHO&redirect_chirho=$ACCOUNT_PATH_CHIRHO" end-b-on-a.txt)" "303|$ACCOUNT_PATH_CHIRHO#refusal-no-such-key-chirho"
expect_chirho "A's page key still works after B's try" "$(call_chirho a-page GET agent-page-chirho a-page-read-2.json)" 200
expect_chirho "a form without a key: status|where" "$(disconnect_chirho a "redirect_chirho=$ACCOUNT_PATH_CHIRHO" end-invalid.txt)" "303|$ACCOUNT_PATH_CHIRHO#refusal-request-invalid-chirho"
expect_chirho "no session: status|where" "$(disconnect_chirho "" "key_id_chirho=$A_PAGE_ID_CHIRHO&redirect_chirho=$ACCOUNT_PATH_CHIRHO" end-no-session.txt)" "303|/auth-chirho/login-chirho#refusal-no-session-chirho"

# ---- her real Chrome with script off: the page's own Disconnect, and the notice the engine's fragment reveals ----
bun "$HERE_CHIRHO/account-browser-chirho.ts" "$REPO_CHIRHO" "$TB_CHIRHO" "$T_CHIRHO" >> "$LOG_CHIRHO" 2>&1; expect_chirho "her Chrome ran the Account page, script off" "$?" 0
BS_CHIRHO=account-browser-summary-chirho.json
expect_chirho "Chrome at 1440 and 390: anything overflowing sideways" "$(json_chirho $BS_CHIRHO 'd["overflow_1440_chirho"]')/$(json_chirho $BS_CHIRHO 'd["overflow_390_chirho"]')" "false/false"
expect_chirho "Chrome before her click: Disconnect buttons/ended rows/the notice shown/whose form she pressed" "$(json_chirho $BS_CHIRHO 'd["before_chirho"]["disconnect_buttons_chirho"]')/$(json_chirho $BS_CHIRHO 'd["before_chirho"]["ended_rows_chirho"]')/$(json_chirho $BS_CHIRHO 'd["before_chirho"]["notice_visible_chirho"]')/$([ "$(json_chirho $BS_CHIRHO 'd["clicked_key_id_chirho"]')" = "$A_PAGE_ID_CHIRHO" ] && echo her-page-key || echo another)" "1/1/false/her-page-key"
expect_chirho "Chrome after her click: the page/the fragment/the notice shown/Disconnect buttons/ended rows" "$(json_chirho $BS_CHIRHO 'd["after_chirho"]["path_chirho"]')/$(json_chirho $BS_CHIRHO 'd["after_chirho"]["fragment_chirho"]')/$(json_chirho $BS_CHIRHO 'd["after_chirho"]["notice_visible_chirho"]')/$(json_chirho $BS_CHIRHO 'd["after_chirho"]["disconnect_buttons_chirho"]')/$(json_chirho $BS_CHIRHO 'd["after_chirho"]["ended_rows_chirho"]')" "$ACCOUNT_PATH_CHIRHO/#notice-key-ended-chirho/true/0/2"
expect_chirho "the key she ended in Chrome is refused; her browser session still works" "$(refused_chirho "$(call_chirho a-page GET agent-page-chirho a-page-ended.json)")/$(account_chirho a account-a-2.html)" "refused/200"

# ---- the page bound for creating too (stage account-connect): the same list, plus one link to the engine's form ----
# Redeployed with the other Account binding; her session and keys carry over. This gate runs on engines before and
# after K4's app selection and never follows the link: following it is the Create gate's proof
# (account-create-rehearsal-chirho.sh).
kill "$TPID_CHIRHO" 2>/dev/null; wait 2>/dev/null; TPID_CHIRHO=""
deploy_stage_chirho page,figures,account-connect; expect_chirho "the engine accepts the Account route bound for creating too" "$?" 0
start_server_chirho "create-bound"
CONNECT_HREF_CHIRHO="/auth-chirho/connection-new-chirho?app_chirho=perffection-workspace&return_chirho=%2Fapp-chirho%2Fperffection-workspace%2Faccount-chirho"
connect_links_chirho() { python3 - "$T_CHIRHO/$1" <<'PY'
import html, re, sys
page = open(sys.argv[1], encoding="utf-8").read()
print(" ".join(html.unescape(h) for h in re.findall(r'href="([^"]*)"', page) if html.unescape(h).startswith("/auth-chirho/connection-new-chirho")))
PY
}
expect_chirho "the create-bound page: status/the one link to the engine's form, as a browser reads it/Disconnect forms/forms" "$(account_chirho a account-a-connect.html)/$(connect_links_chirho account-a-connect.html)/$(forms_to_chirho /auth-chirho/key-revoke-chirho account-a-connect.html)/$(count_chirho '<form' account-a-connect.html)" "200/$CONNECT_HREF_CHIRHO/0/1"

# ---- a key record that fails verification: unavailable, never "none" ----
# The flip goes into the store that holds her keys: from Cairn's K4 (3c7bc083, ad4bc794) a connection client's keys
# live in connection-tokens-chirho.jsonl, before it in scoped-tokens-chirho.jsonl. The flip must land, or the arm below
# proves nothing (measured 2026-10-10: on ad4bc794 the old scoped-token path flipped nothing and the arm failed blind).
TAMPERED_CHIRHO="$(python3 - "$ROOT_CHIRHO/tenants-chirho/$TENANT_CHIRHO" <<'PY'
import json, pathlib, sys
tenant = pathlib.Path(sys.argv[1])
for name in ("connection-tokens-chirho.jsonl", "scoped-tokens-chirho.jsonl"):
    path = tenant / name
    if not path.is_file() or not path.read_text(encoding="utf-8").strip():
        continue
    lines = path.read_text(encoding="utf-8").split("\n")
    sig = json.loads(lines[0])["sig_chirho"]
    flipped = ("1" if sig[0] != "1" else "2") + sig[1:]
    lines[0] = lines[0].replace(f'"sig_chirho":"{sig}"', f'"sig_chirho":"{flipped}"', 1)
    assert len(flipped) == len(sig) and flipped in lines[0]
    path.write_text("\n".join(lines), encoding="utf-8")
    # the live flip: read back from the store itself, the record now carries the changed signature and not its own
    first = json.loads(path.read_text(encoding="utf-8").split("\n")[0])
    print(name if first["sig_chirho"] == flipped and first["sig_chirho"] != sig else "read-back-unchanged")
    break
else:
    print("no-key-store")
PY
)"
# the verdict first, outside any $( ): a case pattern's ")" would close the substitution early
case "$TAMPERED_CHIRHO" in *-tokens-chirho.jsonl) FLIP_CHIRHO=landed ;; *) FLIP_CHIRHO="missed: ${TAMPERED_CHIRHO:-nothing}" ;; esac
expect_chirho "the flip landed in a key store, read back from it (one signature changed)" "$FLIP_CHIRHO" landed
say_chirho "note the tampered store: $TAMPERED_CHIRHO"
expect_chirho "a key record that fails verification: page status/unavailable said/none said/Disconnect forms/private, no-store/link to create" "$(account_chirho a account-a-tampered.html)/$(count_chirho "$(label_chirho unavailable_chirho)" account-a-tampered.html)/$(count_chirho "$(label_chirho empty_chirho)" account-a-tampered.html)/$(forms_to_chirho /auth-chirho/key-revoke-chirho account-a-tampered.html)/$(no_store_chirho account-a-tampered.html)/$(count_chirho 'connection-new-chirho' account-a-tampered.html)" "200/1/0/0/1/1/0"
kill "$TPID_CHIRHO" 2>/dev/null; wait 2>/dev/null; TPID_CHIRHO=""

# ---- no key ever appears in anything an owner's browser or this gate received ----
KEYS_SCAN_CHIRHO="$(python3 - "$T_CHIRHO" <<'PY'
import pathlib, sys
root = pathlib.Path(sys.argv[1])
keys = [p.read_text().strip() for p in root.glob("*-key-chirho.txt") if p.read_text().strip()]
answers = [p for p in root.iterdir() if p.is_file() and p.suffix in (".html", ".json", ".txt", ".hdr") and not p.name.endswith("-key-chirho.txt") and not p.name.endswith("-cookie-chirho.txt")]
text = "\n".join(p.read_text(errors="replace") for p in answers)
planted = (text + keys[0]).count(keys[0]) > 0 if keys else False
print(f"{len(keys)}/{len(answers) > 10}/{sum(text.count(k) for k in keys)}/{planted}")
PY
)"
expect_chirho "no key appears in any answer: keys read/answers scanned/keys found/the planted control found" "$KEYS_SCAN_CHIRHO" "4/True/0/True"
say_chirho "VERDICT: $FAILS_CHIRHO failed assertion(s), $GAPS_CHIRHO known engine gap(s); evidence in $T_CHIRHO"
[ "$FAILS_CHIRHO" -eq 0 ]
