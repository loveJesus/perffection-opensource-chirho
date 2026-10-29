#!/usr/bin/env bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
#
# Landing gate for self-service publication: two roots (platform + tenant beta-chirho), two serves,
# the applier as the ticker, and the customer ladder through the REAL product routes —
# publish v1 -> republish v2 -> rollback v1 — with replays before and after the tick and after the
# address moved, a fresh stale-base attempt, double submit, a reserved slug, tenant-root isolation
# (G1), quota neutrality from the signed usage log, and public-page privacy. Every `expect_chirho`
# is an enforcing assertion; the exit code is the verdict. Local dev only: nothing here touches a
# host, money, public signup or real-user email.
#
# Preconditions: `scripts-chirho/prepare-apps-chirho.sh --app-chirho page-template` and
# `--app-chirho workspace` ran against the pinned cairn checkout (emitted manifests + wasm), and that
# checkout built `cairn-chirho`. The publication routes must be bound in app-chirho.toml.
#
# Env: CAIRN_REPO_CHIRHO (default ../cairn-opensource-chirho), CAIRN_CLI_CHIRHO (otherwise select an installed matching runtime),
#      REHEARSAL_DIR_CHIRHO (default a fresh dir under target/), TENANT_PORT_CHIRHO (8851),
#      PUBLIC_PORT_CHIRHO (8852). Evidence: <rehearsal dir>/journey-chirho.log and the captured pages.
set -uo pipefail; umask 077
SCRIPT_DIR_CHIRHO="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_CHIRHO="$(cd "$SCRIPT_DIR_CHIRHO/../.." && pwd)"
CAIRN_REPO_CHIRHO="${CAIRN_REPO_CHIRHO:-$REPO_CHIRHO/../cairn-opensource-chirho}"
# shellcheck source=../lib-chirho/cairn-runtime-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/cairn-runtime-chirho.sh"
CLI_CHIRHO="$(resolve_cairn_runtime_chirho "$REPO_CHIRHO" "$CAIRN_REPO_CHIRHO")" || exit 2
TENANT_CHIRHO=beta-chirho; SLUG_CHIRHO=good-earth-bakery
TP_CHIRHO="${TENANT_PORT_CHIRHO:-8851}"; PP_CHIRHO="${PUBLIC_PORT_CHIRHO:-8852}"
# shellcheck source=../lib-chirho/served-root-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/served-root-chirho.sh"
require_port_free_chirho "$TP_CHIRHO"; require_port_free_chirho "$PP_CHIRHO"
mkdir -p "$REPO_CHIRHO/target/rehearsal-chirho"
T_CHIRHO="${REHEARSAL_DIR_CHIRHO:-$(mktemp -d "$REPO_CHIRHO/target/rehearsal-chirho/run.XXXX")}"
mkdir -p "$T_CHIRHO"
ROOT_CHIRHO="$T_CHIRHO/root-chirho"; mkdir -p "$ROOT_CHIRHO"
TB_CHIRHO="http://127.0.0.1:$TP_CHIRHO"; PB_CHIRHO="http://127.0.0.1:$PP_CHIRHO"
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
last_report_chirho() { tail -1 "$T_CHIRHO/apply.log" | grep -oE "\"$1\":[0-9]+" | cut -d: -f2; }

cd "$REPO_CHIRHO" || exit 2
for required_chirho in "$CLI_CHIRHO" apps-chirho/perffection-page-template-chirho/.emitted-manifest-chirho.json apps-chirho/perffection-workspace-chirho/.emitted-manifest-chirho.json apps-chirho/perffection-workspace-chirho/deploy_chirho/publication-resource-policy-chirho.json; do
  [ -e "$required_chirho" ] || { printf 'refusing: missing %s (run prepare-apps-chirho.sh for both apps against the pinned cairn, and build cairn-chirho there)\n' "$required_chirho" >&2; exit 2; }
done
grep -q 'route_chirho = "publish-chirho"' apps-chirho/perffection-workspace-chirho/app-chirho.toml || { printf 'refusing: the publish route is not bound in app-chirho.toml (publication is held on this pin)\n' >&2; exit 2; }
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

# app-source-deploy builds cargo-example nodes from source; stage both prepared apps once so this
# gate never runs cargo (the helper verifies every staged byte and refuses a missing artifact).
# shellcheck source=../lib-chirho/prebuilt-app-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/prebuilt-app-chirho.sh"
TEMPLATE_MANIFEST_CHIRHO="$(stage_prebuilt_app_chirho "$CAIRN_REPO_CHIRHO" "$REPO_CHIRHO/apps-chirho/perffection-page-template-chirho" perffection-page-template-chirho "$T_CHIRHO/stage-template-chirho")" || { printf 'refusing: could not stage the prepared page-template app without cargo\n' >&2; exit 2; }
WORKSPACE_MANIFEST_CHIRHO="$(stage_prebuilt_app_chirho "$CAIRN_REPO_CHIRHO" "$REPO_CHIRHO/apps-chirho/perffection-workspace-chirho" perffection-workspace "$T_CHIRHO/stage-workspace-chirho")" || { printf 'refusing: could not stage the prepared workspace app without cargo\n' >&2; exit 2; }

# ---- platform root: template app, registry, reservations ----
OWNER_P_CHIRHO="$("$CLI_CHIRHO" --root "$ROOT_CHIRHO" identity-register --kind owner | cut -f1)"
"$CLI_CHIRHO" --root "$ROOT_CHIRHO" app-source-deploy "$TEMPLATE_MANIFEST_CHIRHO" --owner "$OWNER_P_CHIRHO" --allow-unreviewed-dev > "$T_CHIRHO/deploy-template.out" 2>&1
expect_chirho "template deploy rc (staged, no cargo)" "$?" 0
TEMPLATE_DAG_CHIRHO="$(grep -E '^app dag ' "$T_CHIRHO/deploy-template.out" | awk '{print $3}' | tail -1)"
expect_chirho "template root dag named" "${#TEMPLATE_DAG_CHIRHO}" 64
"$CLI_CHIRHO" --root "$ROOT_CHIRHO" template-registry-set --template-id perffection-page-template-chirho --root-dag "$TEMPLATE_DAG_CHIRHO" --response-content-type text/html > "$T_CHIRHO/registry.out" 2>&1; expect_chirho "template-registry-set rc" "$?" 0
for pair_chirho in "perffection:platform-app-chirho" "perffection-workspace:workspace-app-chirho" "perffection-page-template-chirho:template-name-chirho"; do
  "$CLI_CHIRHO" --root "$ROOT_CHIRHO" publication-reserve-slug --slug "${pair_chirho%%:*}" --reason "${pair_chirho##*:}" > "$T_CHIRHO/reserve.out" 2>&1; expect_chirho "reserve ${pair_chirho%%:*} rc" "$?" 0
done

# ---- tenant root: owner, user, workspace app, resource policy ----
OWNER_T_CHIRHO="$("$CLI_CHIRHO" --root "$ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" identity-register --kind owner | cut -f1)"
PW_FILE_CHIRHO="$T_CHIRHO/cred-chirho.txt"; openssl rand -base64 18 | tr -d '\n' > "$PW_FILE_CHIRHO"; chmod 600 "$PW_FILE_CHIRHO"
PRINCIPAL_CHIRHO="$("$CLI_CHIRHO" --root "$ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" identity-register --kind user --credential-file "$PW_FILE_CHIRHO" | cut -f1)"
"$CLI_CHIRHO" --root "$ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" app-source-deploy "$WORKSPACE_MANIFEST_CHIRHO" --owner "$OWNER_T_CHIRHO" --allow-unreviewed-dev > "$T_CHIRHO/deploy-workspace.out" 2>&1; expect_chirho "workspace deploy rc (staged, no cargo)" "$?" 0
# The meter the publish door charges must RECOVER. At this pin a meter without window_ms_chirho is
# cumulative for as long as its policy hash stays active: until 2026-09-20 this file refused every
# publish after her sixteenth, as a bare 400 with no page (evidence
# 26-09-20-publish-meter-lifetime-wall-chirho). Scoped to the publication-intent action's own
# meters (gpt #24194): a deliberate finite grant elsewhere may be cumulative, and the numbers are
# plastic, so neither is pinned. publish_meters_chirho <policy file> prints
# "<meters the publish action charges>/<of those, without a finite positive window and limit>".
publish_meters_chirho() { python3 - "$1" <<'PY'
import json, sys
policy = json.load(open(sys.argv[1]))
charged = {c.get("resource_chirho") for a in policy.get("actions_chirho", [])
           if a.get("section_chirho") == "publication-intent-chirho" for c in a.get("charges_chirho", [])}
def finite(value): return isinstance(value, int) and not isinstance(value, bool) and value > 0
walls = [m for m in policy.get("meters_chirho", []) if m.get("resource_chirho") in charged
         and not (finite(m.get("window_ms_chirho")) and finite(m.get("per_user_limit_chirho")))]
print(f"{len(charged)}/{len(walls)}")
PY
}
printf '{"meters_chirho":[{"resource_chirho":"publications-chirho","unit_chirho":"intent-chirho","per_user_limit_chirho":16,"overage_chirho":"FailClosedChirho"}],"actions_chirho":[{"section_chirho":"publication-intent-chirho","action_chirho":"intend-chirho","charges_chirho":[{"resource_chirho":"publications-chirho","amount_chirho":1}]}]}' > "$T_CHIRHO/policy-control-chirho.json"
expect_chirho "control: a windowless publish meter is seen as a wall" "$(publish_meters_chirho "$T_CHIRHO/policy-control-chirho.json")" "1/1"
expect_chirho "the deployed publish meter recovers (charged meters / walls)" "$(publish_meters_chirho apps-chirho/perffection-workspace-chirho/deploy_chirho/publication-resource-policy-chirho.json)" "1/0"
"$CLI_CHIRHO" --root "$ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" resource-policy-set perffection-workspace --owner "$OWNER_T_CHIRHO" --file apps-chirho/perffection-workspace-chirho/deploy_chirho/publication-resource-policy-chirho.json > "$T_CHIRHO/policy.out" 2>&1; expect_chirho "resource-policy-set rc" "$?" 0

# ---- two serves + the applier on demand ----
"$CLI_CHIRHO" --root "$ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" serve --port "$TP_CHIRHO" > "$T_CHIRHO/tenant-serve.log" 2>&1 & TPID_CHIRHO=$!
"$CLI_CHIRHO" --root "$ROOT_CHIRHO" serve --port "$PP_CHIRHO" --allow-platform-root-chirho > "$T_CHIRHO/public-serve.log" 2>&1 & PPID_CHIRHO=$!
trap 'kill "$TPID_CHIRHO" "$PPID_CHIRHO" 2>/dev/null; wait "$TPID_CHIRHO" "$PPID_CHIRHO" 2>/dev/null; rm -f "$PW_FILE_CHIRHO"' EXIT
for _ in $(seq 1 80); do curl -s -o /dev/null "$TB_CHIRHO/auth-chirho/login-chirho" && curl -s -o /dev/null "$PB_CHIRHO/" && break; sleep 0.5; done
expect_chirho "the servers answering are the two this run started" "$(own_server_verdict_chirho "$TPID_CHIRHO" "$TP_CHIRHO")/$(own_server_verdict_chirho "$PPID_CHIRHO" "$PP_CHIRHO")" "own/own"
apply_chirho() { "$CLI_CHIRHO" --root "$ROOT_CHIRHO" publication-apply --for-tenant "$TENANT_CHIRHO" --json 2>&1 | tee -a "$T_CHIRHO/apply.log" | tail -1 | cut -c1-300; }

# ---- login ----
code_chirho=$(curl -s -o /dev/null -D "$T_CHIRHO/login.hdr" -w '%{http_code}' --data-urlencode "principal_chirho=$PRINCIPAL_CHIRHO" --data-urlencode "credential_chirho=$(cat "$PW_FILE_CHIRHO")" --data-urlencode "redirect_chirho=/app-chirho/perffection-workspace/" "$TB_CHIRHO/auth-chirho/login-chirho"); expect_chirho "login" "$code_chirho" 303
COOKIE_CHIRHO="$(grep -i '^set-cookie' "$T_CHIRHO/login.hdr" | sed -E 's/^[Ss]et-[Cc]ookie: ([^;]*);.*/\1/' | tr -d '\r')"
WS_CHIRHO="$TB_CHIRHO/app-chirho/perffection-workspace"
get_chirho() { curl -s -o "$T_CHIRHO/$2" -w '%{http_code}' -H "Cookie: $COOKIE_CHIRHO" "$WS_CHIRHO/$1"; }
post_chirho() { curl -s -o "$T_CHIRHO/$3" -w '%{http_code}' -H "Cookie: $COOKIE_CHIRHO" --data "$2" "$WS_CHIRHO/$1"; }
form_chirho() { printf 'schema_version_chirho=2&expected_version_chirho=%s&business_name_chirho=North+Street+Repairs&headline_chirho=%s&summary_chirho=Straightforward+local+service&service_1_title_chirho=Repairs&service_1_body_chirho=Homes+and+small+shops&service_2_title_chirho=&service_2_body_chirho=&service_3_title_chirho=&service_3_body_chirho=&contact_heading_chirho=Tell+us+what+needs+attention&contact_phone_chirho=555-0124&contact_email_chirho=&contact_details_chirho=During+business+hours' "$1" "$2"; }
public_chirho() { curl -s -o "$T_CHIRHO/$1" -w '%{http_code}' "$PB_CHIRHO/app-chirho/$SLUG_CHIRHO/"; }
same_chirho() { cmp -s "$T_CHIRHO/$1" "$T_CHIRHO/$2" && echo yes || echo no; }
# The arms below read the pages' STRUCTURE (ids, classes, values), never a customer sentence: copy
# is plastic and a catalog edit must not turn the gate red.
count_chirho() { grep -o -- "$1" "$T_CHIRHO/$2" | wc -l | tr -d ' '; }
# door_page_chirho <file>: the publish door's answer page. One intent heading; the alert eyebrow
# is rendered only when the request was NOT queued.
door_page_chirho() {
  if [ "$(count_chirho ' id="intent-heading-chirho"' "$1")" != 1 ]; then echo other
  elif [ "$(count_chirho 'workspace-eyebrow-alert-chirho' "$1")" = 0 ]; then echo queued
  else echo refused; fi
}
# live_version_chirho <file>: the version the Publishing page names as live, or "none".
live_version_chirho() {
  local version_chirho
  version_chirho="$(grep -o 'class="workspace-version-chirho">[^<]*<strong>[0-9]*</strong>' "$T_CHIRHO/$1" | grep -o '<strong>[0-9]*' | grep -o '[0-9]*' | head -1)"
  echo "${version_chirho:-none}"
}
# saved_lines_chirho <file>: prose-class lines on the Publishing page. The saved-version line is
# always there (1); "this saved version is what is live" adds one in place of a republish form (2).
saved_lines_chirho() { count_chirho 'class="workspace-saved-version-chirho"' "$1"; }

# 1. fresh editor: Publishing nav link, no pointer at version 0
expect_chirho "editor v0" "$(get_chirho '' editor-0.html)" 200
expect_chirho "editor v0 publishing link / pointer" "$(grep -c 'publication-status-chirho' "$T_CHIRHO/editor-0.html")/$(count_chirho 'workspace-publish-pointer-chirho' editor-0.html)" "1/0"
# 2. save v1 -> the pointer and still exactly one form (the save form)
expect_chirho "save v1" "$(post_chirho edit-page-chirho "$(form_chirho 0 Clear+help)" save-1.html)" 200
expect_chirho "editor v1" "$(get_chirho '' editor-1.html)" 200
expect_chirho "editor v1: one pointer, one link to Publishing, only the save form" "$(count_chirho 'workspace-publish-pointer-chirho' editor-1.html)/$(count_chirho 'class="workspace-button-link-chirho" href="/app-chirho/perffection-workspace/publication-status-chirho"' editor-1.html)/$(count_chirho '<form' editor-1.html)" "1/1/1"
# 2b. a refused save, served: neither contact method and a half-filled second service. Her words
#     come back in the editor, each problem is linked from the summary to a control and said beside
#     it (ids and links, never sentences), and nothing is written: the editor stays at version 1.
refused_form_chirho() { printf 'schema_version_chirho=2&expected_version_chirho=1&business_name_chirho=North+Street+Repairs&headline_chirho=Clear+help&summary_chirho=Straightforward+local+service&service_1_title_chirho=Repairs&service_1_body_chirho=Homes+and+small+shops&service_2_title_chirho=Callouts&service_2_body_chirho=&service_3_title_chirho=&service_3_body_chirho=&contact_heading_chirho=Tell+us+what+needs+attention&contact_phone_chirho=&contact_email_chirho=&contact_details_chirho=During+business+hours'; }
expect_chirho "refused save answers with the editor" "$(post_chirho edit-page-chirho "$(refused_form_chirho)" save-refused.html)/$(count_chirho '<form' save-refused.html)" "200/1"
expect_chirho "refused save: summary links to the controls" "$(count_chirho 'href="#contact-phone-chirho"' save-refused.html)/$(count_chirho 'href="#service-2-body-chirho"' save-refused.html)" "1/1"
expect_chirho "refused save: one message per home, three controls marked" "$(count_chirho ' id="contact-method-error-chirho"' save-refused.html)/$(count_chirho ' id="service-2-body-error-chirho"' save-refused.html)/$(count_chirho 'aria-invalid="true"' save-refused.html)" "1/1/3"
expect_chirho "refused save keeps her words" "$(count_chirho 'value="Callouts"' save-refused.html)" 1
expect_chirho "refused save wrote nothing: editor still at version 1" "$(get_chirho '' editor-1b.html)/$(count_chirho 'name="expected_version_chirho" value="1"' editor-1b.html)/$(count_chirho 'aria-invalid' editor-1b.html)" "200/1/0"
# 2c. a refused paste on "Recover page content", served: her paste comes back in the field, the
#     reason is linked from the top and said beside the field, and nothing is written.
expect_chirho "refused paste answers with the import form" "$(post_chirho review-page-document-chirho 'expected_version_chirho=1&page_document_chirho=%7B+not+json+at+all' import-refused.html)/$(count_chirho '<form' import-refused.html)" "200/1"
expect_chirho "refused paste: kept, field marked, reason linked and beside it" "$(count_chirho '>{ not json at all</textarea>' import-refused.html)/$(grep -o '<textarea id="page-document-import-chirho"[^>]*>' "$T_CHIRHO/import-refused.html" | grep -c 'aria-invalid="true"')/$(count_chirho 'href="#page-document-import-chirho"' import-refused.html)/$(count_chirho ' id="page-document-import-error-chirho"' import-refused.html)" "1/1/1/1"
expect_chirho "refused paste wrote nothing: editor still at version 1" "$(get_chirho '' editor-1c.html)/$(count_chirho 'name="expected_version_chirho" value="1"' editor-1c.html)" "200/1"
# 3. Publishing page before any publish: the new-address form from the road's head
expect_chirho "status dark" "$(get_chirho publication-status-chirho status-0.html)" 200
expect_chirho "status dark: new-address form only" "$(grep -c '<form' "$T_CHIRHO/status-0.html")/$(grep -c 'value="publish-chirho"' "$T_CHIRHO/status-0.html")/$(grep -c 'value="dark-chirho"' "$T_CHIRHO/status-0.html")" "1/1/1"
# 4. publish v1
expect_chirho "publish v1 queued" "$(post_chirho publish-chirho "kind_chirho=publish-chirho&slug_chirho=$SLUG_CHIRHO&version_chirho=1&receipt_digest_chirho=&base_publication_chirho=dark-chirho" publish-1.html)/$(door_page_chirho publish-1.html)" "200/queued"
expect_chirho "public before apply" "$(public_chirho public-0.html)" 404
say_chirho "apply 1: $(apply_chirho)"; expect_chirho "apply 1 applied" "$(last_report_chirho applied_chirho)" 1
expect_chirho "public v1" "$(public_chirho public-1.html)/$(grep -c 'North Street Repairs' "$T_CHIRHO/public-1.html" | sed 's/^0$/0/;s/^[1-9][0-9]*$/some/')/$(grep -c 'Clear help' "$T_CHIRHO/public-1.html")" "200/some/1"
expect_chirho "tenant root G1" "$(curl -s -o /dev/null -w '%{http_code}' "$TB_CHIRHO/app-chirho/$SLUG_CHIRHO/")" 404
# saved version 1 is what is live: no republish form, the sentence instead (gpt #23724/#23727)
expect_chirho "status live1: saved is live, no form" "$(get_chirho publication-status-chirho status-1.html)/$(live_version_chirho status-1.html)/$(grep -c '<form' "$T_CHIRHO/status-1.html")/$(saved_lines_chirho status-1.html)" "200/1/0/2"
# 5. save v2, then the Publishing page offers version 2 with the live base; republish through it
expect_chirho "save v2" "$(post_chirho edit-page-chirho "$(form_chirho 1 Fixed+right+the+first+time)" save-2.html)" 200
expect_chirho "status after saving v2: republish form only" "$(get_chirho publication-status-chirho status-1b.html)/$(grep -c '<form' "$T_CHIRHO/status-1b.html")/$(grep -c 'value="republish-chirho"' "$T_CHIRHO/status-1b.html")/$(saved_lines_chirho status-1b.html)" "200/1/1/1"
BASE1_CHIRHO="$(grep -oE 'name="base_publication_chirho" value="[0-9a-f]{64}"' "$T_CHIRHO/status-1b.html" | head -1 | grep -oE '[0-9a-f]{64}')"
expect_chirho "base after v1 rendered" "${#BASE1_CHIRHO}" 64
expect_chirho "republish v2 queued" "$(post_chirho publish-chirho "kind_chirho=republish-chirho&slug_chirho=$SLUG_CHIRHO&version_chirho=2&receipt_digest_chirho=&base_publication_chirho=$BASE1_CHIRHO" publish-2.html)/$(door_page_chirho publish-2.html)" "200/queued"
say_chirho "apply 2: $(apply_chirho)"; expect_chirho "apply 2 applied" "$(last_report_chirho applied_chirho)" 1
expect_chirho "public v2 headline / differs from v1" "$(public_chirho public-2.html)/$(grep -c 'Fixed right the first time' "$T_CHIRHO/public-2.html")/$([ "$(same_chirho public-1.html public-2.html)" = no ] && echo yes || echo no)" "200/1/yes"
# 6. Publishing page after v2: republish + exactly one rollback (version 1), never the live record
expect_chirho "status live2: rollback only, saved is live" "$(get_chirho publication-status-chirho status-2.html)/$(live_version_chirho status-2.html)/$(grep -c '<form' "$T_CHIRHO/status-2.html")/$(grep -c 'value="rollback-chirho"' "$T_CHIRHO/status-2.html")/$(grep -c 'value="republish-chirho"' "$T_CHIRHO/status-2.html")/$(saved_lines_chirho status-2.html)" "200/2/1/1/0/2"
PARAMS1_CHIRHO="$(grep -oE 'name="params_hash_chirho" value="[0-9a-f]{64}"' "$T_CHIRHO/status-2.html" | head -1 | grep -oE '[0-9a-f]{64}')"
BASE2_CHIRHO="$(grep -oE 'name="base_publication_chirho" value="[0-9a-f]{64}"' "$T_CHIRHO/status-2.html" | head -1 | grep -oE '[0-9a-f]{64}')"
expect_chirho "rollback target and a new base rendered" "${#PARAMS1_CHIRHO}/$([ "$BASE2_CHIRHO" != "$BASE1_CHIRHO" ] && [ "${#BASE2_CHIRHO}" = 64 ] && echo yes || echo no)" "64/yes"
# 7. rollback to v1 through that form; replay the same bytes before and after the tick
RB_CHIRHO="kind_chirho=rollback-chirho&slug_chirho=$SLUG_CHIRHO&params_hash_chirho=$PARAMS1_CHIRHO&version_chirho=1&base_publication_chirho=$BASE2_CHIRHO"
expect_chirho "rollback v1 queued" "$(post_chirho publish-chirho "$RB_CHIRHO" rollback-1.html)/$(door_page_chirho rollback-1.html)" "200/queued"
expect_chirho "rollback replay before the tick identical" "$(post_chirho publish-chirho "$RB_CHIRHO" rollback-1b.html)/$(same_chirho rollback-1.html rollback-1b.html)" "200/yes"
say_chirho "apply 3: $(apply_chirho)"; expect_chirho "apply 3 applied / seen" "$(last_report_chirho applied_chirho)/$(last_report_chirho intents_seen_chirho)" "1/3"
expect_chirho "public v3 identical to v1" "$(public_chirho public-3.html)/$(same_chirho public-1.html public-3.html)" "200/yes"
say_chirho "apply 3 again: $(apply_chirho)"; expect_chirho "idempotent tick" "$(last_report_chirho applied_chirho)" 0
USAGE_LADDER_CHIRHO="$(usage_chirho)"; expect_chirho "usage after three acts" "$USAGE_LADDER_CHIRHO" "3/3"
expect_chirho "late rollback replay identical" "$(post_chirho publish-chirho "$RB_CHIRHO" rollback-1c.html)/$(same_chirho rollback-1.html rollback-1c.html)" "200/yes"
say_chirho "apply 4: $(apply_chirho)"; expect_chirho "nothing new after the late replay" "$(last_report_chirho applied_chirho)/$(last_report_chirho intents_seen_chirho)/$(usage_chirho)" "0/3/$USAGE_LADDER_CHIRHO"
# 8. the original republish bytes sent late: a replay, however stale base1 is now
expect_chirho "late republish replay identical" "$(post_chirho publish-chirho "kind_chirho=republish-chirho&slug_chirho=$SLUG_CHIRHO&version_chirho=2&receipt_digest_chirho=&base_publication_chirho=$BASE1_CHIRHO" replay-2.html)/$(same_chirho publish-2.html replay-2.html)" "200/yes"
expect_chirho "usage after the republish replay" "$(usage_chirho)" "$USAGE_LADDER_CHIRHO"
# 9. a FRESH approval against a stale base: refused as words, no intent, no charge
STALE_CODE_CHIRHO="$(post_chirho publish-chirho "kind_chirho=republish-chirho&slug_chirho=$SLUG_CHIRHO&version_chirho=2&receipt_digest_chirho=&base_publication_chirho=$BASE2_CHIRHO" stale.html)"
expect_chirho "stale-base fresh attempt: refused as a page of words, token hidden" "$STALE_CODE_CHIRHO/$(door_page_chirho stale.html)/$(count_chirho 'base-moved' stale.html)" "200/refused/0"
say_chirho "apply after stale: $(apply_chirho)"; expect_chirho "stale attempt: no intent, no charge" "$(last_report_chirho intents_seen_chirho)/$(usage_chirho)" "3/$USAGE_LADDER_CHIRHO"
# 10. double submit of a fresh republish with the current base
expect_chirho "status live3 names version 1" "$(get_chirho publication-status-chirho status-3.html)/$(live_version_chirho status-3.html)" "200/1"
BASE3_CHIRHO="$(grep -oE 'name="base_publication_chirho" value="[0-9a-f]{64}"' "$T_CHIRHO/status-3.html" | head -1 | grep -oE '[0-9a-f]{64}')"
RP_CHIRHO="kind_chirho=republish-chirho&slug_chirho=$SLUG_CHIRHO&version_chirho=2&receipt_digest_chirho=&base_publication_chirho=$BASE3_CHIRHO"
expect_chirho "double submit identical" "$(post_chirho publish-chirho "$RP_CHIRHO" double-a.html)/$(post_chirho publish-chirho "$RP_CHIRHO" double-b.html)/$(same_chirho double-a.html double-b.html)" "200/200/yes"
say_chirho "apply 5: $(apply_chirho)"; expect_chirho "double submit: one applied" "$(last_report_chirho applied_chirho)/$(last_report_chirho intents_seen_chirho)" "1/4"
expect_chirho "public v4 identical to v2" "$(public_chirho public-4.html)/$(same_chirho public-2.html public-4.html)" "200/yes"
# 11. the OLD rollback replayed after that later republish applied: original receipt, v2 stays live
USAGE_LATE_CHIRHO="$(usage_chirho)"
expect_chirho "old rollback replay after v2 re-applied identical" "$(post_chirho publish-chirho "$RB_CHIRHO" rollback-1d.html)/$(same_chirho rollback-1.html rollback-1d.html)" "200/yes"
say_chirho "apply 5b: $(apply_chirho)"; expect_chirho "old rollback replay: no intent, no charge" "$(last_report_chirho intents_seen_chirho)/$(usage_chirho)" "4/$USAGE_LATE_CHIRHO"
expect_chirho "v2 still live" "$(public_chirho public-5.html)/$(same_chirho public-2.html public-5.html)" "200/yes"
# 12. reserved slug: accepted at the door, refused by the platform, the refusal reaches her page
expect_chirho "publish reserved queued" "$(post_chirho publish-chirho "kind_chirho=publish-chirho&slug_chirho=perffection&version_chirho=2&receipt_digest_chirho=&base_publication_chirho=dark-chirho" reserved.html)/$(door_page_chirho reserved.html)" "200/queued"
say_chirho "apply 6: $(apply_chirho)"; expect_chirho "reserved refused by the applier" "$(last_report_chirho refused_chirho)" 1
# She has a live address, so the page offers no new-address form: the refusal is a line in that
# name's history (one refused record), the platform's token never shown, and nothing links to an
# address field that is not there.
expect_chirho "reserved refusal recorded, token hidden, no dangling field link" "$(get_chirho publication-status-chirho status-4.html)/$(count_chirho 'slug-reserved-chirho' status-4.html)/$(count_chirho 'href="#publish-slug-chirho"' status-4.html)/$(count_chirho ' id="publish-slug-chirho"' status-4.html)" "200/0/0/0"
expect_chirho "reserved address dark" "$(curl -s -o /dev/null -w '%{http_code}' "$PB_CHIRHO/app-chirho/perffection/")" 404
# 12b. a second customer with NOTHING live tries the same reserved name: the page offers the
#      new-address form again, and the refusal meets her AT the address field with the name she
#      typed handed back (ids and values, never sentences); once at the top as a link to the field.
PW2_FILE_CHIRHO="$T_CHIRHO/cred-2-chirho.txt"; openssl rand -base64 18 | tr -d '\n' > "$PW2_FILE_CHIRHO"; chmod 600 "$PW2_FILE_CHIRHO"
PRINCIPAL2_CHIRHO="$("$CLI_CHIRHO" --root "$ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" identity-register --kind user --credential-file "$PW2_FILE_CHIRHO" | cut -f1)"
code_chirho=$(curl -s -o /dev/null -D "$T_CHIRHO/login-2.hdr" -w '%{http_code}' --data-urlencode "principal_chirho=$PRINCIPAL2_CHIRHO" --data-urlencode "credential_chirho=$(cat "$PW2_FILE_CHIRHO")" --data-urlencode "redirect_chirho=/app-chirho/perffection-workspace/" "$TB_CHIRHO/auth-chirho/login-chirho"); rm -f "$PW2_FILE_CHIRHO"
expect_chirho "second customer login" "$code_chirho" 303
COOKIE_FIRST_CHIRHO="$COOKIE_CHIRHO"
COOKIE_CHIRHO="$(grep -i '^set-cookie' "$T_CHIRHO/login-2.hdr" | sed -E 's/^[Ss]et-[Cc]ookie: ([^;]*);.*/\1/' | tr -d '\r')"
expect_chirho "second customer saves v1" "$(post_chirho edit-page-chirho "$(form_chirho 0 Second+shop)" save-2nd.html)" 200
expect_chirho "second customer: reserved name queued" "$(post_chirho publish-chirho "kind_chirho=publish-chirho&slug_chirho=perffection&version_chirho=1&receipt_digest_chirho=&base_publication_chirho=dark-chirho" reserved-2nd.html)" 200
say_chirho "apply 7: $(apply_chirho)"; expect_chirho "second customer's reserved name refused by the applier" "$(last_report_chirho refused_chirho)" 1
expect_chirho "the refusal meets her at the address field, her name handed back" "$(get_chirho publication-status-chirho status-2nd.html)/$(count_chirho ' id="publish-slug-chirho"' status-2nd.html)/$(count_chirho 'name="slug_chirho" maxlength="48"' status-2nd.html)/$(grep -o '<input id="publish-slug-chirho"[^>]*>' "$T_CHIRHO/status-2nd.html" | grep -c ' value="perffection".* aria-invalid="true"')/$(count_chirho ' id="publish-slug-error-chirho"' status-2nd.html)/$(count_chirho 'href="#publish-slug-chirho"' status-2nd.html)/$(count_chirho 'slug-reserved-chirho' status-2nd.html)" "200/1/1/1/1/1/0"
# a name she only asked for is never shown as her address: its path appears nowhere on her page
# (the Help link lives under the same prefix and is the only thing allowed to)
expect_chirho "the refused name's path is not shown as hers" "$(grep -o '/app-chirho/perffection/[a-z-]*' "$T_CHIRHO/status-2nd.html" | grep -vc 'help-chirho')" 0
expect_chirho "the reserved name stays dark" "$(curl -s -o /dev/null -w '%{http_code}' "$PB_CHIRHO/app-chirho/perffection/")" 404
# 12c. the second customer asks for the FIRST customer's LIVE name. The engine's door refuses it
#      BEFORE admission as slug-taken-chirho with HER validated name and HER verified head, and the
#      product page then offers the new-address form with her name handed back: nothing of the
#      first customer's on it, no intent queued, no usage charged, the first customer's site
#      untouched. Engine fix: cairn 7fd2f37f (occupied-name facet 1, certified by three Cairn
#      seats 2026-09-22), the pin since 2026-09-24. Before that pin the engine answered base-moved
#      and leaked the other owner's digest (the full-journey RED at gpt's check 473); this arm was
#      HELD on the old pin and RUN on the candidate, and it is unconditional now.
USAGE_BEFORE_TAKEN_CHIRHO="$(usage_chirho)"; INTENTS_BEFORE_TAKEN_CHIRHO="$(last_report_chirho intents_seen_chirho)"
TAKEN_CODE_CHIRHO="$(post_chirho publish-chirho "kind_chirho=publish-chirho&slug_chirho=$SLUG_CHIRHO&version_chirho=1&receipt_digest_chirho=&base_publication_chirho=dark-chirho" taken-2nd.html)"
expect_chirho "another customer's live name: refused at the door as a page, not queued" "$TAKEN_CODE_CHIRHO/$(door_page_chirho taken-2nd.html)" "200/refused"
expect_chirho "the door's refusal offers HER the new-address form with her name handed back" "$(count_chirho ' id="publish-slug-chirho"' taken-2nd.html)/$(count_chirho " value=\"$SLUG_CHIRHO\"" taken-2nd.html)/$(count_chirho 'name="version_chirho" value="1"' taken-2nd.html)/$(count_chirho 'name="base_publication_chirho" value="dark-chirho"' taken-2nd.html)" "1/1/1/1"
expect_chirho "nothing of the first customer's on her page: no live digest, no live path, no engine code" "$(count_chirho "$BASE3_CHIRHO" taken-2nd.html)/$(grep -o "/app-chirho/$SLUG_CHIRHO/" "$T_CHIRHO/taken-2nd.html" | wc -l | tr -d ' ')/$(count_chirho 'slug-taken-chirho' taken-2nd.html)" "0/0/0"
say_chirho "apply 8: $(apply_chirho)"; expect_chirho "no intent was admitted and nothing was charged for the refused name" "$(last_report_chirho intents_seen_chirho)/$(usage_chirho)" "$INTENTS_BEFORE_TAKEN_CHIRHO/$USAGE_BEFORE_TAKEN_CHIRHO"
expect_chirho "the first customer's site is untouched" "$(public_chirho public-after-taken.html)/$(same_chirho public-4.html public-after-taken.html)" "200/yes"
COOKIE_CHIRHO="$COOKIE_FIRST_CHIRHO"
# 13. privacy: nothing of hers on the public page
expect_chirho "public page free of principal and session" "$(grep -c "$PRINCIPAL_CHIRHO" "$T_CHIRHO/public-4.html")/$(grep -c "${COOKIE_CHIRHO#*=}" "$T_CHIRHO/public-4.html")" "0/0"
say_chirho "VERDICT: $FAILS_CHIRHO failed assertion(s); evidence in $T_CHIRHO"
exit $(( FAILS_CHIRHO > 0 ))
