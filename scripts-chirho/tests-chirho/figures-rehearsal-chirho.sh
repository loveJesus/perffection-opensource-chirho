#!/usr/bin/env bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
#
# Her figures, served, on synthetic figures only (gpt #26548; routing A, #26554). The figures routes are
# HELD out of app-chirho.toml until Cairn's K disposition, so this gate binds them only in its own
# scratch mirror of the workspace app. It appends deploy_chirho/figures-routes-chirho.toml to a scratch
# app-chirho.toml, emits through the same pipeline prepare uses, and deploys that to a fresh tenant
# root. Then, with two synthetic owners:
#   paste -> review -> her Save -> the report;
#   her page untouched;
#   a stale Save in another currency that keeps the head and hands her whole attempt back;
#   refused Saves (unsupported currency, nothing kept, an unreadable form) that write nothing;
#   the other owner sees none of it;
#   her assistant's JSON, and the same answer through the MCP adapter's read_figures_chirho;
#   the stored size measured;
#   an UNCERTAIN Save. A proxy (figures-chirho/) forwards her Save and closes her connection with no
#   answer, printing whether the engine's answer had arrived whole first. Her re-read shows the truth,
#   and an exact repeat stores no second version;
#   her figures as a FILE in a real Chrome (gpt #26599, #26616): chosen in the file form while her saved
#   figures stay in the paste form beside it, reviewed, deliberately saved and read back; crafted uploads
#   (a file and a paste together among them) refused with no write; and an upload Save she never hears
#   about, repeated exactly as Chrome sent it.
# Saved state is read back through her own session or key, never inferred from request counts.
# Local only: no host, no money, no provider, no customer figures, no customer key. The exit code is
# the verdict. Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/her-figures-chirho.md
#
# Preconditions: prepare-apps-chirho.sh --app-chirho workspace (it builds the figures guests).
# Env: CAIRN_REPO_CHIRHO, CAIRN_CLI_CHIRHO, TREE_BUILD_CHIRHO (as for prepare) (+ CANDIDATE_CAIRN_COMMIT_CHIRHO for a candidate engine),
# REHEARSAL_DIR_CHIRHO, TENANT_PORT_CHIRHO (default 19280; the proxy takes the next port).
set -uo pipefail; umask 077
SCRIPT_DIR_CHIRHO="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_CHIRHO="$(cd "$SCRIPT_DIR_CHIRHO/../.." && pwd)"
CAIRN_REPO_CHIRHO="${CAIRN_REPO_CHIRHO:-$REPO_CHIRHO/../cairn-opensource-chirho}"
# shellcheck source=../lib-chirho/cairn-runtime-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/cairn-runtime-chirho.sh"
# shellcheck source=../lib-chirho/served-root-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/served-root-chirho.sh"
# shellcheck source=../lib-chirho/emit-manifest-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/emit-manifest-chirho.sh"
# shellcheck source=../lib-chirho/prebuilt-app-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/prebuilt-app-chirho.sh"
CLI_CHIRHO="$(resolve_cairn_runtime_chirho "$REPO_CHIRHO" "$CAIRN_REPO_CHIRHO")" || exit 2
TENANT_CHIRHO=beta-chirho; TP_CHIRHO="${TENANT_PORT_CHIRHO:-19280}"; PP_CHIRHO=$((TP_CHIRHO + 1))
require_port_free_chirho "$TP_CHIRHO"; require_port_free_chirho "$PP_CHIRHO"
mkdir -p "$REPO_CHIRHO/target/rehearsal-chirho"
T_CHIRHO="${REHEARSAL_DIR_CHIRHO:-$(mktemp -d "$REPO_CHIRHO/target/rehearsal-chirho/figures.XXXX")}"; mkdir -p "$T_CHIRHO"
ROOT_CHIRHO="$T_CHIRHO/root-chirho"; mkdir -p "$ROOT_CHIRHO"
TB_CHIRHO="http://127.0.0.1:$TP_CHIRHO"; WS_CHIRHO="$TB_CHIRHO/app-chirho/perffection-workspace"
LOG_CHIRHO="$T_CHIRHO/journey-chirho.log"; : > "$LOG_CHIRHO"
# shellcheck source=../lib-chirho/gate-helpers-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/gate-helpers-chirho.sh"
APP_CHIRHO="$REPO_CHIRHO/apps-chirho/perffection-workspace-chirho"
for required_chirho in "$CLI_CHIRHO" "$APP_CHIRHO/deploy_chirho/figures-routes-chirho.toml" "$REPO_CHIRHO/target/wasm32-unknown-unknown/release/examples/render_figures_page_guest_chirho.wasm"; do
  [ -e "$required_chirho" ] || { printf 'refusing: missing %s (prepare the workspace first)\n' "$required_chirho" >&2; exit 2; }
done
if grep -q 'figures-chirho' "$APP_CHIRHO/app-chirho.toml"; then printf 'refusing: app-chirho.toml already routes figures; this gate binds them only in scratch\n' >&2; exit 2; fi
STAMP_CHIRHO="$("$CLI_CHIRHO" --version 2>/dev/null | grep -oE '[0-9a-f]{40}' | head -1)"
say_chirho "rehearsal dir $T_CHIRHO; engine ${STAMP_CHIRHO:0:12}; product $(git -C "$REPO_CHIRHO" rev-parse --short=12 HEAD 2>/dev/null || echo unknown)"
cli_chirho() { "$CLI_CHIRHO" --root "$ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" "$@"; }

# ---- the scratch binding: a mirror of the workspace app whose app-chirho.toml also routes figures ----
SCRATCH_APP_CHIRHO="$T_CHIRHO/workspace-figures-bound-chirho"; mkdir -p "$SCRATCH_APP_CHIRHO"
# Cleanup on every exit, set before the mirror is filled and before any credential, key or cookie exists: a refusal
# below (tree-build, the staging) must leave neither. The scratch mirror is build input only: its links point back into
# the app, and left behind they would send any tree walk (a `bun test` name filter, a find) round them. Its emitted
# manifest is kept as evidence.
TPID_CHIRHO=""; PPID_CHIRHO=""
trap 'kill "$TPID_CHIRHO" "$PPID_CHIRHO" 2>/dev/null; wait "$TPID_CHIRHO" "$PPID_CHIRHO" 2>/dev/null; cp "$SCRATCH_APP_CHIRHO/.emitted-manifest-chirho.json" "$T_CHIRHO/figures-bound-manifest-chirho.json" 2>/dev/null; rm -rf "$SCRATCH_APP_CHIRHO"; rm -f "$T_CHIRHO"/*-cred-chirho.txt "$T_CHIRHO"/*-key-chirho.txt "$T_CHIRHO"/*-cookie-chirho.txt' EXIT
for entry_chirho in "$APP_CHIRHO"/* "$APP_CHIRHO"/.[!.]*; do
  name_chirho="$(basename "$entry_chirho")"
  case "$name_chirho" in app-chirho.toml|.emitted-manifest-chirho.json|.[!.]*\*) continue ;; esac
  ln -s "$entry_chirho" "$SCRATCH_APP_CHIRHO/$name_chirho"
done
cat "$APP_CHIRHO/app-chirho.toml" "$APP_CHIRHO/deploy_chirho/figures-routes-chirho.toml" > "$SCRATCH_APP_CHIRHO/app-chirho.toml"
# The tree-build prepare uses (the same default and the same override), so the scratch emit matches it.
TREE_BUILD_CHIRHO="${TREE_BUILD_CHIRHO:-$CAIRN_REPO_CHIRHO/target/debug/tree-build-chirho}"
[ -x "$TREE_BUILD_CHIRHO" ] || { printf 'refusing: no tree-build at %s (set TREE_BUILD_CHIRHO as for prepare)\n' "$TREE_BUILD_CHIRHO" >&2; exit 2; }
"$TREE_BUILD_CHIRHO" check --json "$SCRATCH_APP_CHIRHO" > "$T_CHIRHO/scratch-check.json" 2>&1; expect_chirho "tree-build checks the figures-bound scratch app" "$?" 0
emit_app_manifest_chirho "$TREE_BUILD_CHIRHO" "$REPO_CHIRHO" "$SCRATCH_APP_CHIRHO" "$SCRATCH_APP_CHIRHO/.emitted-manifest-chirho.json"; expect_chirho "the scratch app emits" "$?" 0
expect_chirho "the scratch manifest routes the five figures routes" "$(python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); print(sorted(r["route_chirho"] for r in d["deploy_chirho"]["routes_chirho"] if "figures" in r["route_chirho"]))' "$SCRATCH_APP_CHIRHO/.emitted-manifest-chirho.json")" "['agent-figures-chirho', 'agent-save-figures-chirho', 'figures-chirho', 'review-figures-chirho', 'save-figures-chirho']"

# ---- tenant root: owner, two users, the figures-bound workspace app, operator-issued keys ----
OWNER_CHIRHO="$(cli_chirho identity-register --kind owner | cut -f1)"
new_user_chirho() { openssl rand -base64 18 | tr -d '\n' > "$T_CHIRHO/$1-cred-chirho.txt"; chmod 600 "$T_CHIRHO/$1-cred-chirho.txt"; cli_chirho identity-register --kind user --credential-file "$T_CHIRHO/$1-cred-chirho.txt" | cut -f1; }
USER_A_CHIRHO="$(new_user_chirho a)"; USER_B_CHIRHO="$(new_user_chirho b)"
MANIFEST_CHIRHO="$(stage_prebuilt_app_chirho "$CAIRN_REPO_CHIRHO" "$SCRATCH_APP_CHIRHO" perffection-workspace "$T_CHIRHO/stage-workspace-chirho")" || { printf 'refusing: could not stage the figures-bound app\n' >&2; exit 2; }
cli_chirho app-source-deploy "$MANIFEST_CHIRHO" --owner "$OWNER_CHIRHO" --allow-unreviewed-dev > "$T_CHIRHO/deploy.out" 2>&1; expect_chirho "the figures-bound workspace app deployed" "$?" 0
CLIENT_CHIRHO="$(register_client_chirho client-register.out)"; expect_chirho "OAuth client registered: an id read from its own answer" "$(printf '%s' "$CLIENT_CHIRHO" | grep -cE '^client/[0-9a-f]{16}$')" 1
key_chirho() { cli_chirho oauth-token --principal "$2" --credential-file "$T_CHIRHO/$1-cred-chirho.txt" --client "$CLIENT_CHIRHO" --scope page-draft-chirho --rate-window-ms 3600000 --rate-max-invokes 100 2>/dev/null | head -1 | tr '\t' '\n' | sed -n 's/^token=//p' > "$T_CHIRHO/$1-key-chirho.txt"; chmod 600 "$T_CHIRHO/$1-key-chirho.txt"; [ -s "$T_CHIRHO/$1-key-chirho.txt" ]; }
key_chirho a "$USER_A_CHIRHO"; expect_chirho "a key issued for user A" "$?" 0
key_chirho b "$USER_B_CHIRHO"; expect_chirho "a key issued for user B" "$?" 0
"$CLI_CHIRHO" --root "$ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" serve --port "$TP_CHIRHO" > "$T_CHIRHO/tenant-serve.log" 2>&1 & TPID_CHIRHO=$!
bun "$SCRIPT_DIR_CHIRHO/figures-chirho/drop-answer-proxy-chirho.ts" "$PP_CHIRHO" "$TP_CHIRHO" /app-chirho/perffection-workspace/save-figures-chirho > "$T_CHIRHO/proxy.log" 2>&1 & PPID_CHIRHO=$!
for _ in $(seq 1 80); do curl -s -o /dev/null "$TB_CHIRHO/auth-chirho/login-chirho" && break; sleep 0.5; done
expect_chirho "the server answering on port $TP_CHIRHO is the one this run started" "$(own_server_verdict_chirho "$TPID_CHIRHO" "$TP_CHIRHO")" own
ENGINE_PRIVATE_CHIRHO="$(engine_private_answers_chirho)"; say_chirho "note the engine answers an unowned road private, no-store: $ENGINE_PRIVATE_CHIRHO (1 with Cairn K2)"
expect_chirho "A's browser signs in" "$(sign_in_chirho "$USER_A_CHIRHO" a login-a.hdr)" 303
expect_chirho "B's browser signs in" "$(sign_in_chirho "$USER_B_CHIRHO" b login-b.hdr)" 303
A_COOKIE_CHIRHO="Cookie: $(session_cookie_chirho login-a.hdr)"; B_COOKIE_CHIRHO="Cookie: $(session_cookie_chirho login-b.hdr)"
# her session for the real browser of arm 10, removed with the credentials at exit
session_cookie_chirho login-a.hdr > "$T_CHIRHO/a-cookie-chirho.txt"; chmod 600 "$T_CHIRHO/a-cookie-chirho.txt"
# page <cookie header> <route> <out>; form <cookie header> <route> <form body file> <out> [base]: status codes
page_chirho() { curl -s -o "$T_CHIRHO/$3" -D "$T_CHIRHO/$3.hdr" -w '%{http_code}' -H "$1" "$WS_CHIRHO/$2"; }
form_chirho() { curl -s -o "$T_CHIRHO/$4" -D "$T_CHIRHO/$4.hdr" -w '%{http_code}' -H "$1" -H 'Content-Type: application/x-www-form-urlencoded' --data-binary "@$T_CHIRHO/$3" "${5:-$WS_CHIRHO}/$2"; }
# figures_form_chirho <out> <csv file> <expected version> [currency] [note] [extra field]: the exact urlencoded
# fields both review and save post (USD and "synthetic bakery" unless given); an extra field makes it unreadable
figures_form_chirho() { python3 - "$T_CHIRHO/$2" "$3" "${4:-USD}" "${5:-synthetic bakery}" "${6:-}" > "$T_CHIRHO/$1" <<'PY'
import sys, urllib.parse
csv = open(sys.argv[1], encoding="utf-8").read()
fields = [("csv_chirho", csv), ("currency_chirho", sys.argv[3]), ("note_chirho", sys.argv[4]), ("expected_version_chirho", sys.argv[2])]
if sys.argv[5]: fields.append((sys.argv[5], "1"))
sys.stdout.write(urllib.parse.urlencode(fields))
PY
}
version_of_chirho() { grep -oE 'name="expected_version_chirho" value="[0-9]+"' "$T_CHIRHO/$1" | head -1 | grep -oE '[0-9]+' || echo none; }

MARK_CHIRHO="Sourdough$(openssl rand -hex 5)"
printf 'month,kind,money_in,costs,jobs\n2026-01,%s,1200.50,400,300\n2026-01,Cakes,800,500,20\n2026-02,%s,1300,450,320\n2026-02,Cakes,900,600,25\n2026-02,Cakes,10,,\n2026-13,Pies,5,,\n' "$MARK_CHIRHO" "$MARK_CHIRHO" > "$T_CHIRHO/bakery.csv"

# 1. nothing saved: her page offers the paste form at version 0, and no report
expect_chirho "nothing saved: status/version/report" "$(page_chirho "$A_COOKIE_CHIRHO" figures-chirho figures-0.html)/$(version_of_chirho figures-0.html)/$(count_chirho '<table' figures-0.html)" "200/0/0"

# 2. the review: exactly what will be kept (money, costs, jobs, currency, note), every combined and refused line, and a Save of exactly those fields
figures_form_chirho review-body bakery.csv 0
expect_chirho "review: status/kept tables/combined/refused/one Save form" "$(form_chirho "$A_COOKIE_CHIRHO" review-figures-chirho review-body review.html)/$(count_chirho '<caption' review.html)/$(count_chirho 'Cakes</strong>' review.html)/$(count_chirho 'Line 7' review.html)/$(count_chirho 'action="/app-chirho/perffection-workspace/save-figures-chirho"' review.html)" "200/3/1/1/1"
expect_chirho "the review shows the currency and the note as kept" "$(count_chirho '<dd>USD</dd>' review.html)/$(count_chirho '<dd>synthetic bakery</dd>' review.html)" "1/1"
expect_chirho "the review saved nothing: the page still has no version" "$(page_chirho "$A_COOKIE_CHIRHO" figures-chirho figures-0b.html)/$(version_of_chirho figures-0b.html)" "200/0"

# 3. her Save makes version 1, and the report reads it back
expect_chirho "her Save: status/answered saved/version/the report names her kind" "$(form_chirho "$A_COOKIE_CHIRHO" save-figures-chirho review-body saved-1.html)/$(count_chirho 'workspace-figures-notice-saved-chirho' saved-1.html)/$(version_of_chirho saved-1.html)/$(count_chirho "<th scope=\"row\">$MARK_CHIRHO</th>" saved-1.html)" "200/1/1/1"
expect_chirho "her re-read: version/report" "$(page_chirho "$A_COOKIE_CHIRHO" figures-chirho figures-1.html > /dev/null; version_of_chirho figures-1.html)/$(count_chirho "<th scope=\"row\">$MARK_CHIRHO</th>" figures-1.html)" "1/1"
# Before Cairn's K2 an app route set no Cache-Control (the beta ingress adds private, no-store where none is
# set): a known gap there, a hard expectation on an engine with K2's layer (gate-helpers-chirho.sh).
private_answer_arm_chirho "her private figures page carries Cache-Control no-store" "$(no_store_chirho figures-1.html)"

# 4. her page is untouched: figures live under their own key and never rewrite the page document
expect_chirho "her page after the figures save: status/nothing saved on the page" "$(page_chirho "$A_COOKIE_CHIRHO" '' editor.html)/$(count_chirho 'name="restore_from_version_chirho"' editor.html)/$(count_chirho "$MARK_CHIRHO" editor.html)" "200/0/0"

# 5. a stale Save keeps the head and hands her WHOLE attempt back: its CSV, and its own currency and note,
#    which differ from the head's, so a re-review never quietly takes the head's units or provenance.
#    Her page has two forms (paste and file, gpt #26616), and BOTH carry the currency and note, so
#    each is counted twice: a form that dropped them, or showed the head's, fails.
printf 'month,kind,money_in,costs,jobs\n2026-03,Stalebread,5,,\n' > "$T_CHIRHO/stale.csv"; figures_form_chirho stale-body stale.csv 0 EUR 'stale tab note'
expect_chirho "stale Save: status/answered as a conflict/head version offered/her attempt back: CSV/currency/note" "$(form_chirho "$A_COOKIE_CHIRHO" save-figures-chirho stale-body stale.html)/$(count_chirho 'workspace-figures-notice-conflict-chirho' stale.html)/$(version_of_chirho stale.html)/$(count_chirho '2026-03,Stalebread,5.00,,' stale.html)/$(count_chirho 'name="currency_chirho" value="EUR"' stale.html)/$(count_chirho 'name="note_chirho" value="stale tab note"' stale.html)" "200/1/1/1/2/2"
expect_chirho "after the stale Save the head is unchanged: version/her kind/no Stalebread/its currency/its note" "$(page_chirho "$A_COOKIE_CHIRHO" figures-chirho figures-1b.html > /dev/null; version_of_chirho figures-1b.html)/$(count_chirho "<th scope=\"row\">$MARK_CHIRHO</th>" figures-1b.html)/$(count_chirho 'Stalebread' figures-1b.html)/$(count_chirho 'name="currency_chirho" value="USD"' figures-1b.html)/$(count_chirho 'name="note_chirho" value="synthetic bakery"' figures-1b.html)" "1/1/0/2/2"

# 6. a refused Save writes nothing: figures that would be kept but in an unsupported currency, an import that
#    keeps no line, and a form that cannot be read are each answered with the review again and its reason
refused_save_chirho() { echo "$(form_chirho "$A_COOKIE_CHIRHO" save-figures-chirho "$1" "$2")/$(count_chirho 'class="workspace-errors-chirho"' "$2")/$(count_chirho 'action="/app-chirho/perffection-workspace/save-figures-chirho"' "$2")/$(count_chirho 'workspace-figures-notice-' "$2")"; }
figures_form_chirho xyz-body bakery.csv 1 XYZ
expect_chirho "a Save in an unsupported currency: status/the reason/no Save form/no notice" "$(refused_save_chirho xyz-body refused-xyz.html)" "200/1/0/0"
printf 'month,kind,money_in,costs,jobs\n2026-13,Pies,5,,\n' > "$T_CHIRHO/nothing.csv"; figures_form_chirho nothing-body nothing.csv 1
expect_chirho "a Save that keeps no line: status/the reason/no Save form/no notice" "$(refused_save_chirho nothing-body refused-nothing.html)" "200/1/0/0"
figures_form_chirho unreadable-body bakery.csv 1 USD 'synthetic bakery' extra_chirho
expect_chirho "a Save of an unreadable form: status/the reason/no Save form/no notice" "$(refused_save_chirho unreadable-body refused-unreadable.html)" "200/1/0/0"
expect_chirho "after the refused Saves the head is unchanged: version/her kind/its currency" "$(page_chirho "$A_COOKIE_CHIRHO" figures-chirho figures-1c.html > /dev/null; version_of_chirho figures-1c.html)/$(count_chirho "<th scope=\"row\">$MARK_CHIRHO</th>" figures-1c.html)/$(count_chirho 'name="currency_chirho" value="USD"' figures-1c.html)" "1/1/2"

# 7. the other owner sees none of it, in the browser or through a key
expect_chirho "B's figures page: status/version/A's kind" "$(page_chirho "$B_COOKIE_CHIRHO" figures-chirho b-figures.html)/$(version_of_chirho b-figures.html)/$(count_chirho "$MARK_CHIRHO" b-figures.html)" "200/0/0"
expect_chirho "B's assistant: status/state/A's kind" "$(api_get_chirho b agent-figures-chirho b-agent.json)/$(json_chirho b-agent.json 'd["state_chirho"]')/$(count_chirho "$MARK_CHIRHO" b-agent.json)" "200/nothing-saved-chirho/0"

# 8. her assistant reads the same report, the figures and the version, read-only
expect_chirho "A's assistant: status/state/version/her kind in the report" "$(api_get_chirho a agent-figures-chirho a-agent.json)/$(json_chirho a-agent.json 'd["state_chirho"]')/$(json_chirho a-agent.json 'd["current_version_chirho"]')/$(json_chirho a-agent.json 'd["report_chirho"]["kinds_chirho"][0]["kind_chirho"]')" "200/saved-chirho/1/$MARK_CHIRHO"
expect_chirho "the assistant's money is integer cents with unknown named: the combined Cakes February costs" "$(json_chirho a-agent.json 'd["figures_chirho"]["costs_chirho"][1][1]')/$(json_chirho a-agent.json 'd["report_chirho"]["complete_chirho"]')" "null/true"
STORED_CHIRHO="$(python3 -c 'import json,sys; print(len(json.dumps(json.load(open(sys.argv[1]))["figures_chirho"], separators=(",",":"), ensure_ascii=False).encode()))' "$T_CHIRHO/a-agent.json")"
say_chirho "note her saved figures are $STORED_CHIRHO bytes as compact JSON (the door holds 16384)"
expect_chirho "her saved figures fit the door's value bound" "$([ "$STORED_CHIRHO" -le 16384 ] && echo fits || echo over)" fits
private_answer_arm_chirho "her private figures JSON carries Cache-Control no-store" "$(no_store_chirho a-agent.json)"
# K1 landed (c0a12269): a key is refused a route it holds no grant for once her app opens its routes to
# limited keys, which key-grants-rehearsal-chirho.sh proves. This gate opens none, so the old key still reads.
expect_chirho "nothing opened here: the all-powerful key still reads her figures" "$(refused_chirho "$(api_get_chirho a agent-figures-chirho a-agent-2.json)")" 200

# 8b. her own agent through the MCP adapter's read_figures_chirho (tools-chirho/agent-mcp-chirho, over
#     stdio): offered, and the very answer her REST read gave; B's agent sees nothing saved; no key in
#     anything either adapter wrote. Structure only (figures-chirho/figures-mcp-read-chirho.ts).
bun "$SCRIPT_DIR_CHIRHO/figures-chirho/figures-mcp-read-chirho.ts" "$TB_CHIRHO" "$T_CHIRHO/a-key-chirho.txt" "$T_CHIRHO/b-key-chirho.txt" "$T_CHIRHO/mcp-chirho" >> "$LOG_CHIRHO" 2>&1; expect_chirho "the MCP figures read ran to its end" "$?" 0
FM_CHIRHO=mcp-chirho/mcp-figures-summary-chirho.json
expect_chirho "her agent through MCP: offered/flagged/state/version/the very answer her REST read gave" "$(json_chirho $FM_CHIRHO 'd["offered_chirho"]')/$(json_chirho $FM_CHIRHO 'd["a_chirho"]["is_error_chirho"]')/$(json_chirho $FM_CHIRHO 'd["a_chirho"]["state_chirho"]')/$(json_chirho $FM_CHIRHO 'd["a_chirho"]["version_chirho"]')/$(python3 -c 'import json,sys; print("same" if json.load(open(sys.argv[1])) == json.load(open(sys.argv[2])) else "different")' "$T_CHIRHO/mcp-chirho/mcp-a-figures-chirho.json" "$T_CHIRHO/a-agent.json" 2>/dev/null || echo unreadable)" "true/false/saved-chirho/1/same"
expect_chirho "B's agent through MCP: flagged/state/version/no figures" "$(json_chirho $FM_CHIRHO 'd["b_chirho"]["is_error_chirho"]')/$(json_chirho $FM_CHIRHO 'd["b_chirho"]["state_chirho"]')/$(json_chirho $FM_CHIRHO 'd["b_chirho"]["version_chirho"]')/$(json_chirho $FM_CHIRHO 'd["b_chirho"]["figures_absent_chirho"]')" "false/nothing-saved-chirho/0/true"
expect_chirho "no key in anything either adapter wrote: keys read/planted key found/keys found" "$(json_chirho $FM_CHIRHO 'd["keys_read_chirho"]')/$(json_chirho $FM_CHIRHO 'd["scan_finds_planted_key_chirho"]')/$(json_chirho $FM_CHIRHO 'd["keys_written_chirho"]')" "2/true/0"

# 9. an UNCERTAIN Save: the proxy forwards her Save and closes her connection with no answer, printing
#    whether the engine's answer had arrived whole first. The witness is the forwarded Save, no answer
#    to her, and the head her re-read verifies (gpt #26583); the proxy's line says which drop it was.
printf 'month,kind,money_in,costs,jobs\n2026-01,%s,1200.50,400,300\n2026-01,Cakes,800,500,20\n2026-02,%s,1300,450,320\n2026-02,Cakes,999,600,25\n' "$MARK_CHIRHO" "$MARK_CHIRHO" > "$T_CHIRHO/uncertain.csv"; figures_form_chirho uncertain-body uncertain.csv 1
form_chirho "$A_COOKIE_CHIRHO" save-figures-chirho uncertain-body uncertain.html "http://127.0.0.1:$PP_CHIRHO/app-chirho/perffection-workspace" > "$T_CHIRHO/uncertain.code"; CURL_RC_CHIRHO=$?
expect_chirho "the uncertain Save got no answer: curl status/http code/dropped after the engine's whole answer" "$([ "$CURL_RC_CHIRHO" -ne 0 ] && echo no-answer || echo answered)/$(cat "$T_CHIRHO/uncertain.code")/$(grep -c '^dropped-after-whole-answer-chirho$' "$T_CHIRHO/proxy.log" 2>/dev/null)" "no-answer/000/1"
expect_chirho "her re-read tells the truth: it landed as version 2 with February Cakes 999.00" "$(page_chirho "$A_COOKIE_CHIRHO" figures-chirho figures-2.html > /dev/null; version_of_chirho figures-2.html)/$(count_chirho '2026-02,Cakes,999.00,600.00,25' figures-2.html)" "2/1"
# answered as a SAVE (not a conflict, which would also leave the head at 2), and no second version
expect_chirho "an exact repeat of that Save: status/answered saved/answered conflict/version/head" "$(form_chirho "$A_COOKIE_CHIRHO" save-figures-chirho uncertain-body repeat.html)/$(count_chirho 'workspace-figures-notice-saved-chirho' repeat.html)/$(count_chirho 'workspace-figures-notice-conflict-chirho' repeat.html)/$(version_of_chirho repeat.html)/$(page_chirho "$A_COOKIE_CHIRHO" figures-chirho figures-2b.html > /dev/null; version_of_chirho figures-2b.html)" "200/1/0/2/2"

# 10. her figures as a FILE, in a real Chrome signed in as her (figures-chirho/figures-upload-browser-chirho.ts).
#     Every refusal carries its code (data-refusal-chirho), so each arm tells one refusal from another.
RYE_CHIRHO="Rye$(openssl rand -hex 5)"; SPELT_CHIRHO="Spelt$(openssl rand -hex 5)"
printf 'month,kind,money_in,costs,jobs\r\n2026-01,%s,700,250,40\r\n2026-02,%s,820,260,45\r\n' "$RYE_CHIRHO" "$RYE_CHIRHO" > "$T_CHIRHO/upload-rye.csv"
# the second file starts with a byte-order mark, as spreadsheets write it
printf '\xef\xbb\xbfmonth,kind,money_in,costs,jobs\r\n2026-01,%s,90,,\r\n' "$SPELT_CHIRHO" > "$T_CHIRHO/upload-spelt.csv"
upload_browser_chirho() { bun "$SCRIPT_DIR_CHIRHO/figures-chirho/figures-upload-browser-chirho.ts" "$REPO_CHIRHO" "$TB_CHIRHO" "http://127.0.0.1:$PP_CHIRHO" "$T_CHIRHO" "$1" >> "$LOG_CHIRHO" 2>&1; }
upload_browser_chirho files; expect_chirho "Chrome's file runs ran to their end" "$?" 0
UF_CHIRHO=upload-browser-files-chirho.json
expect_chirho "a file chosen while her saved figures stay in the paste form: they were there/the file named/one Save/no refusal" "$(json_chirho $UF_CHIRHO 'd["box_held_her_figures_chirho"]')/$(json_chirho $UF_CHIRHO 'd["review_chirho"]["file_name_chirho"]')/$(json_chirho $UF_CHIRHO 'd["review_chirho"]["save_forms_chirho"]')/$(json_chirho $UF_CHIRHO 'd["review_chirho"]["refusal_chirho"]')" "true/upload-rye.csv/1/0"
expect_chirho "her Save of the reviewed file: answered saved/version" "$(json_chirho $UF_CHIRHO 'd["saved_chirho"]["saved_notice_chirho"]')/$(json_chirho $UF_CHIRHO 'd["saved_chirho"]["version_chirho"]')" "1/3"
expect_chirho "her re-read: version/the file's kind in the report/its February line" "$(page_chirho "$A_COOKIE_CHIRHO" figures-chirho figures-3.html > /dev/null; version_of_chirho figures-3.html)/$(count_chirho "<th scope=\"row\">$RYE_CHIRHO</th>" figures-3.html)/$(count_chirho "2026-02,$RYE_CHIRHO,820.00,260.00,45" figures-3.html)" "3/1/1"
# crafted uploads Chrome would not send: each refused by its own code, with no Save and no write
upload_body_chirho() { python3 "$SCRIPT_DIR_CHIRHO/figures-chirho/figures-upload-body-chirho.py" "$T_CHIRHO/$1" "$2" "$RYE_CHIRHO"; }
# upload_refused_chirho <route> <case>: status/its refusal code/Save forms/notices
upload_refused_chirho() { upload_body_chirho "upload-$2.body" "$2"; echo "$(curl -s -o "$T_CHIRHO/upload-$1-$2.html" -w '%{http_code}' -H "$A_COOKIE_CHIRHO" -H 'Content-Type: multipart/form-data; boundary=----WebKitFormBoundaryGateChirho0123' --data-binary "@$T_CHIRHO/upload-$2.body" "$WS_CHIRHO/$1")/$(grep -oE 'data-refusal-chirho="[a-z-]+"' "$T_CHIRHO/upload-$1-$2.html" | head -1 | cut -d'"' -f2)/$(count_chirho 'action="/app-chirho/perffection-workspace/save-figures-chirho"' "upload-$1-$2.html")/$(count_chirho 'workspace-figures-notice-' "upload-$1-$2.html")"; }
expect_chirho "a crafted upload carrying a file and a paste together: status/code/Save/notice" "$(upload_refused_chirho review-figures-chirho file-and-paste)" "200/file-and-paste-chirho/0/0"
expect_chirho "an upload of an empty chosen file: status/code/Save/notice" "$(upload_refused_chirho review-figures-chirho empty-file)" "200/empty-file-chirho/0/0"
expect_chirho "an upload that is not text: status/code/Save/notice" "$(upload_refused_chirho review-figures-chirho not-text)" "200/file-not-text-chirho/0/0"
expect_chirho "an upload whose close never comes: status/code/Save/notice" "$(upload_refused_chirho review-figures-chirho broken-framing)" "200/unreadable-form-chirho/0/0"
expect_chirho "a good upload posted straight to the save (it reads only the pasted form): status/code/Save/notice" "$(upload_refused_chirho save-figures-chirho good)" "200/unreadable-form-chirho/0/0"
expect_chirho "after the refused uploads her head is unchanged: version/the file's kind" "$(page_chirho "$A_COOKIE_CHIRHO" figures-chirho figures-3b.html > /dev/null; version_of_chirho figures-3b.html)/$(count_chirho "<th scope=\"row\">$RYE_CHIRHO</th>" figures-3b.html)" "3/1"
expect_chirho "B sees none of her file: page version/the file's kind/assistant" "$(page_chirho "$B_COOKIE_CHIRHO" figures-chirho b-figures-3.html > /dev/null; version_of_chirho b-figures-3.html)/$(count_chirho "$RYE_CHIRHO" b-figures-3.html)/$(api_get_chirho b agent-figures-chirho b-agent-3.json > /dev/null; count_chirho "$RYE_CHIRHO" b-agent-3.json)" "0/0/0"
# an upload Save she never hears about: through the drop proxy, then the truth, then Chrome's own Save again
upload_browser_chirho uncertain; expect_chirho "Chrome's uncertain upload Save ran to its end" "$?" 0
UU_CHIRHO=upload-browser-uncertain-chirho.json
expect_chirho "her upload Save got no answer: the request failed/a POST/its body kept" "$(json_chirho $UU_CHIRHO 'd["uncertain_chirho"]["failed_chirho"]')/$(json_chirho $UU_CHIRHO 'd["uncertain_chirho"]["method_chirho"]')/$([ -s "$T_CHIRHO/uncertain-upload-save-body.txt" ] && echo kept || echo none)" "true/POST/kept"
# Chrome itself may resend a Save whose connection closed without an answer. The proxy drops each send,
# so a resend's own answer is never seen: the count is reported, not pinned, and only the head says what
# happened (an exact replay and an optimistic conflict would each leave one version).
DROPS_CHIRHO="$(grep -c '^dropped-after-whole-answer-chirho$' "$T_CHIRHO/proxy.log")"
expect_chirho "the proxy dropped only after the engine's whole answer, and Chrome's Save at least once: before/Chrome dropped" "$(grep -c '^dropped-before-whole-answer-chirho$' "$T_CHIRHO/proxy.log")/$([ "$((DROPS_CHIRHO - 1))" -ge 1 ] && echo yes || echo no)" "0/yes"
say_chirho "note Chrome sent that one Save $((DROPS_CHIRHO - 1)) time(s) (it resends a POST whose connection closed unanswered); the next arms show the head moved once, not which answer the resend got"
expect_chirho "her re-read tells the truth: version 4 with the second file's kind (its byte-order mark gone)" "$(page_chirho "$A_COOKIE_CHIRHO" figures-chirho figures-4.html > /dev/null; version_of_chirho figures-4.html)/$(count_chirho "<th scope=\"row\">$SPELT_CHIRHO</th>" figures-4.html)" "4/1"
expect_chirho "Chrome's own Save, repeated exactly: status/answered saved/answered conflict/version/head" "$(curl -s -o "$T_CHIRHO/repeat-upload.html" -w '%{http_code}' -H "$A_COOKIE_CHIRHO" -H 'Content-Type: application/x-www-form-urlencoded' --data-binary "@$T_CHIRHO/uncertain-upload-save-body.txt" "$WS_CHIRHO/save-figures-chirho")/$(count_chirho 'workspace-figures-notice-saved-chirho' repeat-upload.html)/$(count_chirho 'workspace-figures-notice-conflict-chirho' repeat-upload.html)/$(version_of_chirho repeat-upload.html)/$(page_chirho "$A_COOKIE_CHIRHO" figures-chirho figures-4b.html > /dev/null; version_of_chirho figures-4b.html)" "200/1/0/4/4"

kill "$TPID_CHIRHO" "$PPID_CHIRHO" 2>/dev/null; wait "$TPID_CHIRHO" "$PPID_CHIRHO" 2>/dev/null
say_chirho "VERDICT: $FAILS_CHIRHO failed assertion(s), $GAPS_CHIRHO known engine gap(s) unchanged; evidence in $T_CHIRHO"
[ "$FAILS_CHIRHO" -eq 0 ]
