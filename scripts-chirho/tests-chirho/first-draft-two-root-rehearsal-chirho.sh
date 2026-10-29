#!/usr/bin/env bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
#
# Landing gate for the AI first draft: the tenant root serves the bound brief and draft routes, and
# the customer ladder runs through the REAL product routes — the brief page at version 0, an empty
# brief handed back as the form around her own words without reaching the card, her words never
# logged and never at rest in the clear, a full
# brief through the blessed llm card, the resource-usage records unchanged by a refusal (quota
# evidence; a money read belongs to the S3 arms and cairn's balance verb). Every `expect_chirho` is an
# enforcing assertion; the exit code is the verdict. Local dev only: nothing here touches a host,
# money, public signup, a real provider or real-user email.
#
# Preconditions: `scripts-chirho/prepare-apps-chirho.sh --app-chirho workspace` ran against the
# pinned cairn checkout with BOTH draft routes bound in app-chirho.toml (they stay held on main
# until cairn's served card dispatches; the gate refuses while held), and that checkout built
# `cairn-chirho`. Deploys go through the prebuilt-app stager: this gate never runs cargo.
#
# The card arm: `cairn serve` never composes the served card (Cairn's door guards, CAIRN #28985), so
# on the shipping serve the engine refuses the effect and this gate records that answer without
# pinning it, asserting only privacy and quota neutrality. FIRST_DRAFT_CARD_SERVED_CHIRHO=1 adds
# section 5: Cairn's dev-only example serves the stub card on a SEPARATE dev root, and
# scripted-caller arms check:
#   - a draft per fresh key, the replay of a used key, a changed brief under a reused key and a
#     refused card;
#   - the money in aggregate: across those five POSTs the payer's net falls by exactly two prices
#     and the settlement owner's rises by the same, with nothing held (which POST paid stays open);
#   - issued keys on the draft's Save form;
#   - her words absent from that root and its log.
# The brief GET with an issued key, and "save as version 1", wait on an elected tenant root (C3/C4).
#
# Env: CAIRN_REPO_CHIRHO (default ../cairn-opensource-chirho), CAIRN_CLI_CHIRHO (default its debug binary),
#      REHEARSAL_DIR_CHIRHO (default a fresh dir under target/), TENANT_PORT_CHIRHO (8855),
#      FIRST_DRAFT_SCAN_FAULT_CHIRHO=1 (negative lever: the run must FAIL; see section 4).
#      With FIRST_DRAFT_CARD_SERVED_CHIRHO=1:
#        DEV_LLM_SERVE_CHIRHO, the built served_llm_dev_serve_chirho example (this gate never runs cargo);
#        DEV_LLM_SERVE_COMMIT_CHIRHO, the 40-hex commit its --version must name;
#        DEV_PORT_CHIRHO (8856).
#      Evidence: <rehearsal dir>/journey-chirho.log and the captured pages.
set -uo pipefail; umask 077
SCRIPT_DIR_CHIRHO="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_CHIRHO="$(cd "$SCRIPT_DIR_CHIRHO/../.." && pwd)"
CAIRN_REPO_CHIRHO="${CAIRN_REPO_CHIRHO:-$REPO_CHIRHO/../cairn-opensource-chirho}"
CLI_CHIRHO="${CAIRN_CLI_CHIRHO:-$CAIRN_REPO_CHIRHO/target/debug/cairn-chirho}"
TENANT_CHIRHO=beta-chirho; TP_CHIRHO="${TENANT_PORT_CHIRHO:-8855}"
# shellcheck source=../lib-chirho/cairn-runtime-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/cairn-runtime-chirho.sh"
# shellcheck source=../lib-chirho/served-root-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/served-root-chirho.sh"
require_port_free_chirho "$TP_CHIRHO"
mkdir -p "$REPO_CHIRHO/target/rehearsal-chirho"
OWN_RUN_DIR_CHIRHO=0; [ -n "${REHEARSAL_DIR_CHIRHO:-}" ] || OWN_RUN_DIR_CHIRHO=1
T_CHIRHO="${REHEARSAL_DIR_CHIRHO:-$(mktemp -d "$REPO_CHIRHO/target/rehearsal-chirho/draft.XXXX")}"; mkdir -p "$T_CHIRHO"
ROOT_CHIRHO="$T_CHIRHO/root-chirho"; mkdir -p "$ROOT_CHIRHO"
TB_CHIRHO="http://127.0.0.1:$TP_CHIRHO"
LOG_CHIRHO="$T_CHIRHO/journey-chirho.log"; : > "$LOG_CHIRHO"
say_chirho() { printf '%s\n' "$*" | tee -a "$LOG_CHIRHO"; }
FAILS_CHIRHO=0
# expect_chirho <label> <actual> <expected>: an enforcing assertion; the exit code is the verdict
expect_chirho() { if [ "$2" = "$3" ]; then say_chirho "ok   $1 = $2"; else say_chirho "FAIL $1 = $2 (expected $3)"; FAILS_CHIRHO=$((FAILS_CHIRHO + 1)); fi; }
# usage_chirho: "<allowed usage records>/<charged units>" from the tenant's signed usage log
usage_chirho() { python3 - "$ROOT_CHIRHO/tenants-chirho/$TENANT_CHIRHO/resource-usage-chirho.jsonl" <<'PY'
import json, sys, os
p = sys.argv[1]; n = 0; units = 0
if os.path.exists(p):
    for line in open(p):
        line = line.strip()
        if not line: continue
        r = json.loads(line)
        if r.get("decision_chirho") == "AllowedChirho":
            n += 1; units += sum(c.get("amount_chirho", 0) for c in r.get("charges_chirho", []))
print(f"{n}/{units}")
PY
}

# An early refusal happens before anything is deployed; a run directory this gate made for itself
# is removed, so refusing on main (the routes are held) leaves nothing behind.
refuse_early_chirho() {
  printf 'refusing: %s\n' "$1" >&2
  if [ "$OWN_RUN_DIR_CHIRHO" = 1 ]; then case "$T_CHIRHO" in "$REPO_CHIRHO"/target/rehearsal-chirho/draft.*) rm -rf -- "$T_CHIRHO";; esac; fi
  exit 2
}
cd "$REPO_CHIRHO" || exit 2
for required_chirho in "$CLI_CHIRHO" apps-chirho/perffection-workspace-chirho/.emitted-manifest-chirho.json apps-chirho/perffection-workspace-chirho/deploy_chirho/publication-resource-policy-chirho.json; do
  [ -e "$required_chirho" ] || refuse_early_chirho "missing $required_chirho (run prepare-apps-chirho.sh --app-chirho workspace against the pinned cairn, and build cairn-chirho there)"
done
grep -q 'route_chirho = "draft-page-chirho"' apps-chirho/perffection-workspace-chirho/app-chirho.toml || refuse_early_chirho "the draft routes are not bound in app-chirho.toml (the first draft is held on this pin)"
# The node fixtures are a positive control on the prepare: a run over an emptied fixture directory
# reports "clean (0 fixtures)" (measured 2026-09-19 after the temp cleaner took the files), so the
# gate re-runs the fixture runner and refuses a zero count.
TREE_BUILD_CHIRHO="${TREE_BUILD_CHIRHO:-$CAIRN_REPO_CHIRHO/target/debug/tree-build-chirho}"
FIXTURE_REPORT_CHIRHO="$("$TREE_BUILD_CHIRHO" test --json apps-chirho/perffection-workspace-chirho 2>/dev/null)"
FIXTURE_COUNT_CHIRHO="$(printf '%s' "$FIXTURE_REPORT_CHIRHO" | python3 -c 'import json,sys; r=json.load(sys.stdin); print(r["fixture_count_chirho"] if r.get("clean_chirho") else -1)' 2>/dev/null || echo -1)"
[ "${FIXTURE_COUNT_CHIRHO:-0}" -ge 1 ] || refuse_early_chirho "the workspace node fixtures did not run clean (count $FIXTURE_COUNT_CHIRHO); the prepared tree is incomplete"
# A card run (section 5) binds Cairn's dev example the way the CLI is bound: a prebuilt binary whose
# --version names the commit given for it. Refused here, before any root is touched.
CARD_SERVED_CHIRHO="${FIRST_DRAFT_CARD_SERVED_CHIRHO:-0}"
if [ "$CARD_SERVED_CHIRHO" = 1 ]; then
  DEV_SERVE_CHIRHO="${DEV_LLM_SERVE_CHIRHO:-}"; DEV_COMMIT_CHIRHO="${DEV_LLM_SERVE_COMMIT_CHIRHO:-}"
  DP_CHIRHO="${DEV_PORT_CHIRHO:-8856}"
  [ -x "$DEV_SERVE_CHIRHO" ] || refuse_early_chirho "a card run names the built served_llm_dev_serve_chirho in DEV_LLM_SERVE_CHIRHO"
  [[ "$DEV_COMMIT_CHIRHO" =~ ^[0-9a-f]{40}$ ]] || refuse_early_chirho "DEV_LLM_SERVE_COMMIT_CHIRHO must name the example's full commit"
  # The whole output must be ONE clean stamp line at exactly that commit, the same anchored form as the CLI's
  # (check_cairn_runtime_chirho): a "-dirty" build, a second line or any other commit refuses (CAIRN #29095).
  DEV_STAMP_CHIRHO="$("$DEV_SERVE_CHIRHO" --version 2>/dev/null)"
  DEV_STAMP_PATTERN_CHIRHO='^served_llm_dev_serve_chirho [^[:space:]]+ \(([0-9a-f]{40})\)$'
  [[ "$DEV_STAMP_CHIRHO" =~ $DEV_STAMP_PATTERN_CHIRHO && "${BASH_REMATCH[1]}" == "$DEV_COMMIT_CHIRHO" ]] \
    || refuse_early_chirho "the dev example's --version is not one clean stamp at DEV_LLM_SERVE_COMMIT_CHIRHO"
  [ "$DP_CHIRHO" != "$TP_CHIRHO" ] || refuse_early_chirho "the dev root needs its own port"
  # Section 5 reads an absent balance as a zero baseline only on a dev root this run creates (gpt #29132).
  [ ! -e "$T_CHIRHO/dev-root-chirho" ] || refuse_early_chirho "a card run needs a fresh dev root, and $T_CHIRHO/dev-root-chirho already exists"
  require_port_free_chirho "$DP_CHIRHO"
fi
say_chirho "rehearsal dir $T_CHIRHO"; PIN_CHIRHO="$(resolve_cairn_pin_chirho "$REPO_CHIRHO" "$CAIRN_REPO_CHIRHO")" || exit 2
STAMP_CHIRHO="$("$CLI_CHIRHO" --version 2>/dev/null | grep -oE '[0-9a-f]{40}' | head -1)"
say_chirho "cairn pin ${PIN_CHIRHO:0:12} runtime stamp ${STAMP_CHIRHO:0:12} (engine checkout now at $(git -C "$CAIRN_REPO_CHIRHO" rev-parse --short=12 HEAD 2>/dev/null || echo unknown)) product $(git -C "$REPO_CHIRHO" rev-parse --short HEAD 2>/dev/null || echo unknown)"
# The binary must be the PRODUCT PIN's build, not whatever an earlier build left behind: the CLI
# stamps its source commit into --version, and a masked build failure once ran a stale binary
# under a newer label (2026-09-18). It is checked against CAIRN-PIN-chirho.txt by the resolver's
# own rule (the stamp is the pin, or contains it with documentation-only drift) and NOT against
# the engine checkout's HEAD: that checkout is the Cairn seats' working tree and moves without us
# (2026-09-20: two documentation commits turned this gate red while the runtime was exactly the
# pin). Enforcing, before any root is touched.
check_cairn_runtime_chirho "$CLI_CHIRHO" "$CAIRN_REPO_CHIRHO" "$PIN_CHIRHO" 2>> "$LOG_CHIRHO"; RUNTIME_RC_CHIRHO=$?
CANDIDATE_CHIRHO="$(candidate_cairn_commit_chirho 2>/dev/null || true)"
if [ -n "$CANDIDATE_CHIRHO" ]; then
  say_chirho "CANDIDATE RUN: engine ${CANDIDATE_CHIRHO:0:12} named by CANDIDATE_CAIRN_COMMIT_CHIRHO; this is NOT the product pin ${PIN_CHIRHO:0:12} and proves nothing about it"
  expect_chirho "the runtime is the named CANDIDATE's build (candidate ${CANDIDATE_CHIRHO:0:12}, stamp ${STAMP_CHIRHO:0:12})" "$RUNTIME_RC_CHIRHO" 0
else
  expect_chirho "the runtime is the product pin's build (pin ${PIN_CHIRHO:0:12}, stamp ${STAMP_CHIRHO:0:12})" "$RUNTIME_RC_CHIRHO" 0
fi
# A runtime that is not the bound build refuses HERE, before any staging, identity or root write (gpt #29088).
[ "$RUNTIME_RC_CHIRHO" = 0 ] || refuse_early_chirho "the runtime is not the bound build (see the log); nothing was staged or written"
# stop_run_chirho <reason>: end a run that cannot go on safely. Roots are already written, so it is a failed run
# (exit 1), not a refusal; the EXIT trap stops only this run's own children, and every file is kept as evidence.
stop_run_chirho() { say_chirho "STOPPED: $1"; say_chirho "VERDICT: $FAILS_CHIRHO failed assertion(s); evidence in $T_CHIRHO"; exit 1; }

# app-source-deploy builds cargo-example nodes from source; stage the prepared outputs once so this
# gate never runs cargo (the helper verifies every staged byte and refuses a missing artifact).
# shellcheck source=../lib-chirho/prebuilt-app-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/prebuilt-app-chirho.sh"
WORKSPACE_MANIFEST_CHIRHO="$(stage_prebuilt_app_chirho "$CAIRN_REPO_CHIRHO" "$REPO_CHIRHO/apps-chirho/perffection-workspace-chirho" perffection-workspace "$T_CHIRHO/stage-workspace-chirho")" || { printf 'refusing: could not stage the prepared workspace app without cargo\n' >&2; exit 2; }

# ---- tenant root: owner, user, workspace app with the draft routes, resource policy ----
OWNER_T_CHIRHO="$("$CLI_CHIRHO" --root "$ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" identity-register --kind owner | cut -f1)"
PW_FILE_CHIRHO="$T_CHIRHO/cred-chirho.txt"; openssl rand -base64 18 | tr -d '\n' > "$PW_FILE_CHIRHO"; chmod 600 "$PW_FILE_CHIRHO"
PRINCIPAL_CHIRHO="$("$CLI_CHIRHO" --root "$ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" identity-register --kind user --credential-file "$PW_FILE_CHIRHO" | cut -f1)"
"$CLI_CHIRHO" --root "$ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" app-source-deploy "$WORKSPACE_MANIFEST_CHIRHO" --owner "$OWNER_T_CHIRHO" --allow-unreviewed-dev > "$T_CHIRHO/deploy-workspace.out" 2>&1; expect_chirho "workspace deploy rc (staged, no cargo)" "$?" 0
expect_chirho "both draft routes deployed" "$(grep -c 'route draft-chirho \|route draft-page-chirho ' "$T_CHIRHO/deploy-workspace.out")" 2
"$CLI_CHIRHO" --root "$ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" resource-policy-set perffection-workspace --owner "$OWNER_T_CHIRHO" --file apps-chirho/perffection-workspace-chirho/deploy_chirho/publication-resource-policy-chirho.json > "$T_CHIRHO/policy.out" 2>&1; expect_chirho "resource-policy-set rc" "$?" 0
# the served card prices ONE flat per-call amount from the app's signed price book (cairn
# effect_pricing, key llm_chirho); the number is a SYNTHETIC test value here — the production row
# is L.J.'s M packet. Set before serve so the money policy can resolve at dispatch.
FIRST_DRAFT_PRICE_CHIRHO="${FIRST_DRAFT_PRICE_CHIRHO:-7}"
"$CLI_CHIRHO" --root "$ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" pricebook-set perffection-workspace --owner "$OWNER_T_CHIRHO" --effect-price "llm_chirho=$FIRST_DRAFT_PRICE_CHIRHO" > "$T_CHIRHO/pricebook.out" 2>&1; expect_chirho "pricebook-set rc (flat price for llm_chirho)" "$?" 0
# The dev money bootstrap's asset kind and funding (section 5 only; synthetic). The bootstrap never
# runs on this tenant root: it makes a root test-kind, and the shipping serve then refuses every app
# route of it (measured 2026-09-19 at cairn 1a105c0d).
FIRST_DRAFT_BILLING_KIND_CHIRHO="${FIRST_DRAFT_BILLING_KIND_CHIRHO:-money-chirho}"
FIRST_DRAFT_FUNDING_CHIRHO="${FIRST_DRAFT_FUNDING_CHIRHO:-137}"

# ---- serve ----
"$CLI_CHIRHO" --root "$ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" serve --port "$TP_CHIRHO" > "$T_CHIRHO/tenant-serve.log" 2>&1 & TPID_CHIRHO=$!
cleanup_chirho() {
  # Bounded, and only this run's own children by pid (stop_own_child_chirho): never a name, never a wait forever.
  stop_own_child_chirho "$TPID_CHIRHO" 0
  # Section 5's dev example normally stops on its stop file; this only ends one the run left running.
  if [ -n "${DPID_CHIRHO:-}" ]; then touch "$DSTOP_CHIRHO"; stop_own_child_chirho "$DPID_CHIRHO" 10; fi
  rm -f "$PW_FILE_CHIRHO" "${DPW_FILE_CHIRHO:-}"
}
trap cleanup_chirho EXIT
for _ in $(seq 1 80); do CURL_DEADLINE_CHIRHO=2 local_curl_chirho -s -o /dev/null "$TB_CHIRHO/auth-chirho/login-chirho" && break; sleep 0.5; done
OWN_TENANT_CHIRHO="$(own_server_verdict_chirho "$TPID_CHIRHO" "$TP_CHIRHO")"
expect_chirho "the server answering is the one this run started" "$OWN_TENANT_CHIRHO" own
# No credential or cookie is ever sent to a listener this run did not start (gpt #29088).
[ "$OWN_TENANT_CHIRHO" = own ] || stop_run_chirho "the listener on $TP_CHIRHO is $OWN_TENANT_CHIRHO, so no credential is sent to it"

# ---- login ----
code_chirho=$(local_curl_chirho -s -o /dev/null -D "$T_CHIRHO/login.hdr" -w '%{http_code}' --data-urlencode "principal_chirho=$PRINCIPAL_CHIRHO" --data-urlencode "credential_chirho=$(cat "$PW_FILE_CHIRHO")" --data-urlencode "redirect_chirho=/app-chirho/perffection-workspace/" "$TB_CHIRHO/auth-chirho/login-chirho"); expect_chirho "login" "$code_chirho" 303
COOKIE_CHIRHO="$(grep -i '^set-cookie' "$T_CHIRHO/login.hdr" | sed -E 's/^[Ss]et-[Cc]ookie: ([^;]*);.*/\1/' | tr -d '\r')"
WS_CHIRHO="$TB_CHIRHO/app-chirho/perffection-workspace"
get_chirho() { local_curl_chirho -s -o "$T_CHIRHO/$2" -w '%{http_code}' -H "Cookie: $COOKIE_CHIRHO" "$WS_CHIRHO/$1"; }
post_chirho() { local_curl_chirho -s -o "$T_CHIRHO/$3" -w '%{http_code}' -H "Cookie: $COOKIE_CHIRHO" --data "$2" "$WS_CHIRHO/$1"; }
MARKER_CHIRHO="Zebrafish-Quill-$(openssl rand -hex 4)"   # a word of hers that must never come back or be logged
brief_chirho() { # <business name> <phone> -> the full brief body carrying the marker in the about line
  printf 'business_name_chirho=%s&about_chirho=%s+sourdough+and+rye+baked+each+morning+in+Mesa&audience_chirho=Families+and+cafes+around+Mesa&tone_chirho=warm-chirho&service_1_chirho=Sourdough+loaves&service_2_chirho=Wedding+cakes&service_3_chirho=&contact_phone_chirho=%s&contact_email_chirho=&contact_details_chirho=Tuesday+to+Saturday+7+to+2' "$1" "$MARKER_CHIRHO" "$2"
}

# 1. at version 0 the editor carries the First draft link and the brief page offers its one form
expect_chirho "editor v0" "$(get_chirho '' editor-0.html)" 200
expect_chirho "editor v0 first-draft link" "$(grep -c 'perffection-workspace/draft-chirho' "$T_CHIRHO/editor-0.html" | sed 's/^[1-9][0-9]*$/some/')" some
expect_chirho "brief page" "$(get_chirho draft-chirho brief-0.html)" 200
expect_chirho "brief page: one form to the draft door / tone select" "$(grep -c '<form' "$T_CHIRHO/brief-0.html")/$(grep -c 'action="/app-chirho/perffection-workspace/draft-page-chirho"' "$T_CHIRHO/brief-0.html")/$(grep -c '<select' "$T_CHIRHO/brief-0.html")" "1/1/1"
expect_chirho "anonymous brief page and door refused" "$(local_curl_chirho -s -o /dev/null -w '%{http_code}' "$WS_CHIRHO/draft-chirho")/$(local_curl_chirho -s -o /dev/null -w '%{http_code}' --data "$(brief_chirho Anon+Bakery 555-0100)" "$WS_CHIRHO/draft-page-chirho")" "401/401"
# 2. the rejection branch never reaches the card. A readable brief comes back as the form around
#    her own words with every problem linked to its control (ids and links, never sentences); her
#    words go back only in that response: section 4 proves they are not logged and do not rest
#    in the root store in the clear, which is what "private at the sink" has to mean here.
count_chirho() { grep -o -- "$1" "$T_CHIRHO/$2" | wc -l | tr -d ' '; }
expect_chirho "empty brief comes back as the form, three problems linked" "$(post_chirho draft-page-chirho "business_name_chirho=&about_chirho=&tone_chirho=warm-chirho" reject-0.html)/$(count_chirho 'action="/app-chirho/perffection-workspace/draft-page-chirho"' reject-0.html)/$(count_chirho 'href="#draft-business-name-chirho"' reject-0.html)/$(count_chirho 'href="#draft-about-chirho"' reject-0.html)/$(count_chirho 'href="#draft-contact-phone-chirho"' reject-0.html)" "200/1/1/1/1"
expect_chirho "brief without a contact keeps her words in the form; the rule sits on the pair" "$(post_chirho draft-page-chirho "business_name_chirho=Good+Earth+Bakery&about_chirho=$MARKER_CHIRHO+bakes+bread&tone_chirho=plain-chirho" reject-1.html)/$(count_chirho "$MARKER_CHIRHO bakes bread</textarea>" reject-1.html)/$(count_chirho ' id="draft-contact-method-error-chirho"' reject-1.html)/$(count_chirho 'aria-invalid="true"' reject-1.html)/$(count_chirho 'value="plain-chirho" selected' reject-1.html)" "200/1/1/2/1"
# a2b will have the engine issue a hidden idempotency_key_chirho into the brief form. Whether or
# not the route strips it, the product must not choke on it: a contact-less brief CARRYING that
# field is still handed back as the form (not the "could not be read" sentence), and the submitted
# key is not copied into the page the product renders.
KEY_MARKER_CHIRHO="KEYMARK-$(openssl rand -hex 4)"
expect_chirho "a brief carrying the engine's hidden key field is still handed back as the form, key not echoed" "$(post_chirho draft-page-chirho "business_name_chirho=Good+Earth+Bakery&about_chirho=We+bake+bread&tone_chirho=warm-chirho&idempotency_key_chirho=$KEY_MARKER_CHIRHO" reject-key.html)/$(count_chirho 'action="/app-chirho/perffection-workspace/draft-page-chirho"' reject-key.html)/$(count_chirho ' id="draft-contact-method-error-chirho"' reject-key.html)/$(count_chirho "$KEY_MARKER_CHIRHO" reject-key.html)" "200/1/1/0"
USAGE_BEFORE_CARD_CHIRHO="$(usage_chirho)"
# usage_chirho reads the tenant's signed RESOURCE-USAGE log (records/units), not the money ledger:
# 0/0 says those records and units are unchanged. It is not a funded money proof, and it does not
# by itself say which DAG nodes ran (gpt #24138); that the request builder skips such a body is the
# native test's claim (`exactly_one_branch_is_live...`), not an engine trace.
expect_chirho "rejections leave the resource-usage records and units unchanged" "$USAGE_BEFORE_CARD_CHIRHO" "0/0"
# 3. the card branch: a full brief reaches the blessed llm card
CARD_CODE_CHIRHO="$(post_chirho draft-page-chirho "$(brief_chirho Good+Earth+Bakery 555-0142)" draft-1.html)"
say_chirho "card arm status $CARD_CODE_CHIRHO (body head: $(head -c 120 "$T_CHIRHO/draft-1.html" | tr '\n' ' '))"
expect_chirho "card arm: nothing of hers in the answer" "$(grep -c "$PRINCIPAL_CHIRHO" "$T_CHIRHO/draft-1.html")/$(grep -c "${COOKIE_CHIRHO#*=}" "$T_CHIRHO/draft-1.html")" "0/0"
# The shipping serve never composes the card (CAIRN #28985), so here it answers the door's refusal;
# that answer is recorded, not pinned. The served card itself is section 5's, on its own dev root.
say_chirho "card arm on the shipping serve: recorded status $CARD_CODE_CHIRHO (the served card runs in section 5)"
expect_chirho "an uncomposed card refusal leaves the resource-usage records and units unchanged" "$(usage_chirho)" "$USAGE_BEFORE_CARD_CHIRHO"
# 4. privacy: her words never appear in the engine's serve log or in any error body
# The privacy arms read through the shared fail-closed instrument (gpt #24135): a missing root, an
# unreadable file or directory, or any grep error reads as `scan-error:...`, never as a quiet 0.
# shellcheck source=../lib-chirho/root-scan-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/root-scan-chirho.sh"
# Negative lever, a rehearsal of the gate itself: FIRST_DRAFT_SCAN_FAULT_CHIRHO=1 plants an
# unreadable file INSIDE the root before the scans. That run must end FAILED on the scan errors;
# if it passes, the privacy arms below are not fail-closed and prove nothing.
if [ "${FIRST_DRAFT_SCAN_FAULT_CHIRHO:-0}" = "1" ]; then
  printf 'unreadable on purpose\n' > "$ROOT_CHIRHO/zz-scan-fault-chirho"; chmod 000 "$ROOT_CHIRHO/zz-scan-fault-chirho"
  say_chirho "scan fault planted inside the root: this run MUST fail"
fi
expect_chirho "her words never logged or echoed" "$(file_hits_chirho "$MARKER_CHIRHO" "$T_CHIRHO/tenant-serve.log")/$(file_hits_chirho "$MARKER_CHIRHO" "$T_CHIRHO/draft-1.html")" "0/0"
# CAIRN's S3 finding (cairn 0bb7f151): the served prompt rested in PUBLIC CAS as the dispatcher's
# request copy. The store keeps public bytes in the clear, so her marker word must be absent from
# every file under the root; a sealed copy is unreadable and does not count. Three controls first,
# because an absent marker is only privacy if the instrument can both SEE and FAIL: it sees clear
# text in this root (the deployed manifest names the routes); a missing root is a scan error; an
# unreadable file holding the marker is a scan error, never a quiet 0.
expect_chirho "control: the store scan sees clear text in this root" "$(seen_verdict_chirho "$(scan_root_chirho 'draft-page-chirho' "$ROOT_CHIRHO")")" sees
scan_controls_chirho "$MARKER_CHIRHO" "$T_CHIRHO"
expect_chirho "her words never rest in the root store in the clear" "$(scan_root_chirho "$MARKER_CHIRHO" "$ROOT_CHIRHO")" 0
expect_chirho "no secret in the root" "$(scan_root_chirho "$(cat "$PW_FILE_CHIRHO")" "$ROOT_CHIRHO")" 0
# 5. S3, the served card itself (FIRST_DRAFT_CARD_SERVED_CHIRHO=1).
#    - Where: `cairn serve` can never compose it, so Cairn's dev-only example served_llm_dev_serve_chirho serves
#      it (CAIRN #29056) on its OWN platform root.
#    - Provisioning: identities, the app, its policy and a synthetic price first, then the dev money bootstrap
#      (CAIRN #29004, PFE #29022), as the example requires.
#    - What it can serve: only the app root GET and the paid POST. PFE's root and brief pages read her state
#      through a read-store card, so neither renders there (PFE #29008).
#    - So the arms act as a scripted caller: the cairn-idempotency-key-chirho header and no form field (the
#      header-only rule, CAIRN #28977, PFE #28980), straight to the draft door.
#    - The money read is the example's own: the verified substrate balances it prints before it takes the ledger
#      and after it stops, read as one validated record per role and phase (lib-chirho/dev-balance-records-chirho.py).
#    - Still waiting on an elected tenant root once C3/C4 convert them: the browser-shaped brief GET with an
#      issued key, and "save as version 1".
if [ "$CARD_SERVED_CHIRHO" = 1 ]; then
  DEV_ROOT_CHIRHO="$T_CHIRHO/dev-root-chirho"
  # Created here, never reused: the owner's absent "before" balance is a zero baseline only on this run's own root.
  mkdir "$DEV_ROOT_CHIRHO" || stop_run_chirho "the dev root $DEV_ROOT_CHIRHO already exists, so its balances are not this run's"
  DEV_ROOT_STATE_CHIRHO=fresh-chirho
  DSTOP_CHIRHO="$T_CHIRHO/dev-stop-chirho"; DB_CHIRHO="http://127.0.0.1:$DP_CHIRHO"
  REFUSAL_MARKER_CHIRHO="REFUSE-ME-$(openssl rand -hex 4)-CHIRHO"
  dev_cli_chirho() { "$CLI_CHIRHO" --root "$DEV_ROOT_CHIRHO" "$@"; }
  DOWNER_CHIRHO="$(dev_cli_chirho identity-register --kind owner | cut -f1)"
  DPW_FILE_CHIRHO="$T_CHIRHO/dev-cred-chirho.txt"; openssl rand -base64 18 | tr -d '\n' > "$DPW_FILE_CHIRHO"; chmod 600 "$DPW_FILE_CHIRHO"
  DPAYER_CHIRHO="$(dev_cli_chirho identity-register --kind user --credential-file "$DPW_FILE_CHIRHO" | cut -f1)"
  dev_cli_chirho app-source-deploy "$WORKSPACE_MANIFEST_CHIRHO" --owner "$DOWNER_CHIRHO" --allow-unreviewed-dev > "$T_CHIRHO/dev-deploy.out" 2>&1
  expect_chirho "S3: the workspace app deploys on the dev root, before its bootstrap" "$?" 0
  dev_cli_chirho resource-policy-set perffection-workspace --owner "$DOWNER_CHIRHO" --file apps-chirho/perffection-workspace-chirho/deploy_chirho/publication-resource-policy-chirho.json > "$T_CHIRHO/dev-policy.out" 2>&1
  expect_chirho "S3: the resource policy on the dev root" "$?" 0
  dev_cli_chirho pricebook-set perffection-workspace --owner "$DOWNER_CHIRHO" --effect-price "llm_chirho=$FIRST_DRAFT_PRICE_CHIRHO" > "$T_CHIRHO/dev-pricebook.out" 2>&1
  expect_chirho "S3: the synthetic flat price on the dev root" "$?" 0
  dev_cli_chirho served-llm-dev-bootstrap-chirho --payer-principal "$DPAYER_CHIRHO" --owner-principal "$DOWNER_CHIRHO" --amount "$FIRST_DRAFT_FUNDING_CHIRHO" --billing-kind "$FIRST_DRAFT_BILLING_KIND_CHIRHO" > "$T_CHIRHO/dev-bootstrap.out" 2>&1
  expect_chirho "S3: the dev money bootstrap funds the payer" "$?" 0
  say_chirho "S3: dev example $DEV_STAMP_CHIRHO on 127.0.0.1:$DP_CHIRHO"
  CAIRN_DEV_LLM_SERVED_CHIRHO=1 "$DEV_SERVE_CHIRHO" --root-chirho "$DEV_ROOT_CHIRHO" --port-chirho "$DP_CHIRHO" \
    --app-chirho perffection-workspace --paid-route-chirho draft-page-chirho --billing-kind-chirho "$FIRST_DRAFT_BILLING_KIND_CHIRHO" \
    --stop-file-chirho "$DSTOP_CHIRHO" --refusal-marker-chirho "$REFUSAL_MARKER_CHIRHO" > "$T_CHIRHO/dev-serve.log" 2>&1 & DPID_CHIRHO=$!
  for _ in $(seq 1 80); do CURL_DEADLINE_CHIRHO=2 local_curl_chirho -s -o /dev/null "$DB_CHIRHO/auth-chirho/login-chirho" && break; sleep 0.5; done
  OWN_DEV_CHIRHO="$(own_server_verdict_chirho "$DPID_CHIRHO" "$DP_CHIRHO")"
  expect_chirho "S3: the dev server answering is the one this run started" "$OWN_DEV_CHIRHO" own
  [ "$OWN_DEV_CHIRHO" = own ] || stop_run_chirho "the listener on $DP_CHIRHO is $OWN_DEV_CHIRHO, so no credential is sent to it"
  code_chirho=$(local_curl_chirho -s -o /dev/null -D "$T_CHIRHO/dev-login.hdr" -w '%{http_code}' --data-urlencode "principal_chirho=$DPAYER_CHIRHO" --data-urlencode "credential_chirho=$(cat "$DPW_FILE_CHIRHO")" --data-urlencode "redirect_chirho=/app-chirho/perffection-workspace/" "$DB_CHIRHO/auth-chirho/login-chirho")
  expect_chirho "S3: the payer signs in on the dev root" "$code_chirho" 303
  DCOOKIE_CHIRHO="$(grep -i '^set-cookie' "$T_CHIRHO/dev-login.hdr" | sed -E 's/^[Ss]et-[Cc]ookie: ([^;]*);.*/\1/' | tr -d '\r')"
  key_chirho() { openssl rand -base64 32 | tr '+/' '-_' | tr -d '=\n'; }
  dev_post_chirho() { local_curl_chirho -s -o "$T_CHIRHO/$3" -w '%{http_code}' -H "Cookie: $DCOOKIE_CHIRHO" -H "cairn-idempotency-key-chirho: $1" --data "$2" "$DB_CHIRHO/app-chirho/perffection-workspace/draft-page-chirho"; }
  # The page as the product rendered it, with the engine's issued key values masked so a replay can be compared.
  unkeyed_chirho() { sed -E 's/(name="idempotency_key_chirho" value=")[A-Za-z0-9_-]{43}"/\1ISSUED"/g' "$T_CHIRHO/$1"; }
  K1_CHIRHO="$(key_chirho)"; K2_CHIRHO="$(key_chirho)"; K3_CHIRHO="$(key_chirho)"
  expect_chirho "S3: a draft at version 0 from the stub" "$(dev_post_chirho "$K1_CHIRHO" "$(brief_chirho Good+Earth+Bakery 555-0142)" dev-draft-1.html)/$(count_chirho ' id="workspace-notice-heading-chirho"' dev-draft-1.html)/$(count_chirho 'name="expected_version_chirho" value="0"' dev-draft-1.html)/$(count_chirho 'value="Good Earth Bakery"' dev-draft-1.html)/$(count_chirho 'value="555-0142"' dev-draft-1.html)" "200/1/1/1/1"
  ISSUED_CHIRHO="$(grep -oE 'name="idempotency_key_chirho" value="[A-Za-z0-9_-]{43}"' "$T_CHIRHO/dev-draft-1.html" | wc -l | tr -d ' ')"
  expect_chirho "S3: the draft's forms carry issued keys, none left empty" "$([ "$ISSUED_CHIRHO" -ge 1 ] && echo some || echo none)/$(count_chirho 'name="idempotency_key_chirho" value=""' dev-draft-1.html)" "some/0"
  expect_chirho "S3: the same key and brief answer the recorded draft" "$(dev_post_chirho "$K1_CHIRHO" "$(brief_chirho Good+Earth+Bakery 555-0142)" dev-draft-1b.html)/$(cmp -s <(unkeyed_chirho dev-draft-1.html) <(unkeyed_chirho dev-draft-1b.html) && echo same || echo different)" "200/same"
  # Cairn's named follow-up (CAIRN #28973): a changed brief under a reused key answers the run-path refusal,
  # not yet #refusal-caller-key-conflict-chirho. Its status is recorded; the money checks below see all five POSTs
  # together, so they cannot say on their own that this one cost nothing.
  say_chirho "S3 open: the same key with a changed brief answered $(dev_post_chirho "$K1_CHIRHO" "$(brief_chirho Changed+Bakery 555-0177)" dev-draft-1c.html)"
  expect_chirho "S3: a fresh key with a distinct brief makes a distinct draft" "$(dev_post_chirho "$K2_CHIRHO" "$(brief_chirho North+Street+Repairs 555-0124)" dev-draft-2.html)/$(count_chirho 'value="North Street Repairs"' dev-draft-2.html)" "200/1"
  # The refused brief carries words of her own beside the stub's marker, and the privacy arms below cover both.
  REFUSED_WORD_CHIRHO="Kestrel-Lantern-$(openssl rand -hex 4)"
  REFUSED_BRIEF_CHIRHO="business_name_chirho=Refused+Bakery&about_chirho=$REFUSAL_MARKER_CHIRHO+$REFUSED_WORD_CHIRHO+bakes+bread&audience_chirho=&tone_chirho=warm-chirho&service_1_chirho=&service_2_chirho=&service_3_chirho=&contact_phone_chirho=555-0199&contact_email_chirho=&contact_details_chirho="
  expect_chirho "S3: a refused card renders a sentence" "$(dev_post_chirho "$K3_CHIRHO" "$REFUSED_BRIEF_CHIRHO" dev-refused.html)/$(count_chirho ' id="draft-refused-heading-chirho"' dev-refused.html)" "200/1"
  # Bounded: 30 s for the stop file, then TERM and KILL of this run's own child only. The after-balances exist only
  # when it stops by itself, so any other ending fails the money checks below with the evidence kept.
  touch "$DSTOP_CHIRHO"
  stop_own_child_chirho "$DPID_CHIRHO" 30; DPID_CHIRHO=""
  expect_chirho "S3: the dev example stops on its stop file within 30 s" "$STOP_HOW_CHIRHO" "exited:0"
  # The balances, as one validated record per role and phase (gpt #29132). The example prints a line only for an
  # account IN its verified snapshot (CAIRN #29092), so on this fresh root the settlement owner has no "before" line
  # until its first charge settles (PFE #29130). The parser takes that one absence as a zero baseline, and only from
  # a run the example finished cleanly on a root this run created; it requires every other record, and reads a
  # malformed, glued, duplicate or out-of-order line as an error, never as an absence. Each money check reads the
  # parser's fields, which stay `unavailable` unless the records are valid, so no missing value passes as a number.
  BALANCES_CHIRHO="$(python3 -I "$REPO_CHIRHO/scripts-chirho/lib-chirho/dev-balance-records-chirho.py" "$T_CHIRHO/dev-serve.log" \
    "$DPAYER_CHIRHO" "$DOWNER_CHIRHO" "$FIRST_DRAFT_BILLING_KIND_CHIRHO" "$STOP_HOW_CHIRHO" "$DEV_ROOT_STATE_CHIRHO")"
  balance_field_chirho() { # <field>: one field of the parser's line, or `unavailable` when it printed none
    local value_chirho; value_chirho="$(printf '%s\n' "$BALANCES_CHIRHO" | tr ' ' '\n' | sed -n "s/^${1}_chirho=//p")"
    printf '%s' "${value_chirho:-unavailable}"
  }
  say_chirho "S3: balance records ${BALANCES_CHIRHO:-(none printed)}"
  expect_chirho "S3: the balance lines read as one valid record per role and phase" "$(balance_field_chirho records)" valid
  expect_chirho "S3: records by role and phase (payer before, owner before, payer after, owner after); the owner has no account before its first charge" \
    "$(balance_field_chirho presence)" "present/absent/present/present"
  expect_chirho "S3: the payer starts funded" "$(balance_field_chirho net | cut -d/ -f1)" "$FIRST_DRAFT_FUNDING_CHIRHO"
  # Before and after totals prove the aggregate across all five POSTs, not which POST paid (gpt #29132).
  expect_chirho "S3: across the five POSTs the payer's net fell by exactly two flat prices in aggregate" "$(balance_field_chirho payer_net_change)" "-$(( 2 * FIRST_DRAFT_PRICE_CHIRHO ))"
  expect_chirho "S3: across the five POSTs the settlement owner's net rose by the same aggregate" "$(balance_field_chirho owner_net_change)" "$(( 2 * FIRST_DRAFT_PRICE_CHIRHO ))"
  # A Hold moves held without moving net, so net alone cannot show that nothing was left in escrow (gpt #29088).
  expect_chirho "S3: nothing held for the payer or the owner, before and after" "$(balance_field_chirho held)" "0/0/0/0"
  say_chirho "S3 open: which of the five POSTs paid is not witnessed here; per-invocation charge attribution stays open"
  expect_chirho "S3: her words, and her refused brief's, never logged by the dev server" "$(file_hits_chirho "$MARKER_CHIRHO" "$T_CHIRHO/dev-serve.log")/$(file_hits_chirho "$REFUSED_WORD_CHIRHO" "$T_CHIRHO/dev-serve.log")/$(file_hits_chirho "$REFUSAL_MARKER_CHIRHO" "$T_CHIRHO/dev-serve.log")" "0/0/0"
  expect_chirho "S3 control: the store scan sees clear text in the dev root" "$(seen_verdict_chirho "$(scan_root_chirho 'draft-page-chirho' "$DEV_ROOT_CHIRHO")")" sees
  scan_controls_chirho "$REFUSED_WORD_CHIRHO" "$T_CHIRHO"
  expect_chirho "S3: her words never rest in the dev root store in the clear" "$(scan_root_chirho "$MARKER_CHIRHO" "$DEV_ROOT_CHIRHO")" 0
  expect_chirho "S3: her refused brief's words never rest in the dev root store in the clear" "$(scan_root_chirho "$REFUSED_WORD_CHIRHO" "$DEV_ROOT_CHIRHO")/$(scan_root_chirho "$REFUSAL_MARKER_CHIRHO" "$DEV_ROOT_CHIRHO")" "0/0"
  expect_chirho "S3: no secret in the dev root" "$(scan_root_chirho "$(cat "$DPW_FILE_CHIRHO")" "$DEV_ROOT_CHIRHO")" 0
fi
say_chirho "VERDICT: $FAILS_CHIRHO failed assertion(s); evidence in $T_CHIRHO"
exit $(( FAILS_CHIRHO > 0 ))
