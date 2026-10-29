#!/usr/bin/env bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
#
# Gate for R1 self-service credential change through the REAL product route and cairn's served road
# (CAIRN #21925): a tenant root, the workspace app, one user with two live sessions; the Account
# page's real form; refusals (wrong current password, too-short new one, foreign redirect target);
# the accepted change kills both sessions, the old password refuses at login, the new one signs in;
# five wrong guesses from one session revoke that session (the owner's login budget is never spent)
# and, since cairn 07871db0, the fifth answer signs her out to the login page with the
# credential-guess-exhausted fragment (cookie cleared), followed here as a browser would;
# neither password is written anywhere in the root. A refusal
# comes back as a URL fragment `#refusal-<token>` the page reveals with CSS :target (CAIRN #21934).
# Every `expect_chirho` enforces; the exit code is the verdict. Local dev only.
#
# Preconditions: prepare-apps-chirho.sh --app-chirho workspace against the pinned cairn, that cairn's
# cairn-chirho built, and the account route bound in app-chirho.toml.
# Env: CAIRN_REPO_CHIRHO, CAIRN_CLI_CHIRHO, REHEARSAL_DIR_CHIRHO, TENANT_PORT_CHIRHO (8853).
set -uo pipefail; umask 077
SCRIPT_DIR_CHIRHO="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_CHIRHO="$(cd "$SCRIPT_DIR_CHIRHO/../.." && pwd)"
CAIRN_REPO_CHIRHO="${CAIRN_REPO_CHIRHO:-$REPO_CHIRHO/../cairn-opensource-chirho}"
# shellcheck source=../lib-chirho/cairn-runtime-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/cairn-runtime-chirho.sh"
CLI_CHIRHO="$(resolve_cairn_runtime_chirho "$REPO_CHIRHO" "$CAIRN_REPO_CHIRHO")" || exit 2
TENANT_CHIRHO=beta-chirho; TP_CHIRHO="${TENANT_PORT_CHIRHO:-8853}"
# shellcheck source=../lib-chirho/served-root-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/served-root-chirho.sh"
require_port_free_chirho "$TP_CHIRHO"
mkdir -p "$REPO_CHIRHO/target/rehearsal-chirho"
T_CHIRHO="${REHEARSAL_DIR_CHIRHO:-$(mktemp -d "$REPO_CHIRHO/target/rehearsal-chirho/account.XXXX")}"; mkdir -p "$T_CHIRHO"
ROOT_CHIRHO="$T_CHIRHO/root-chirho"; mkdir -p "$ROOT_CHIRHO"
TB_CHIRHO="http://127.0.0.1:$TP_CHIRHO"
LOG_CHIRHO="$T_CHIRHO/journey-chirho.log"; : > "$LOG_CHIRHO"
say_chirho() { printf '%s\n' "$*" | tee -a "$LOG_CHIRHO"; }
FAILS_CHIRHO=0
expect_chirho() { if [ "$2" = "$3" ]; then say_chirho "ok   $1 = $2"; else say_chirho "FAIL $1 = $2 (expected $3)"; FAILS_CHIRHO=$((FAILS_CHIRHO + 1)); fi; }
cd "$REPO_CHIRHO" || exit 2
for required_chirho in "$CLI_CHIRHO" apps-chirho/perffection-workspace-chirho/.emitted-manifest-chirho.json; do
  [ -e "$required_chirho" ] || { printf 'refusing: missing %s\n' "$required_chirho" >&2; exit 2; }
done
grep -q 'route_chirho = "account-chirho"' apps-chirho/perffection-workspace-chirho/app-chirho.toml || { printf 'refusing: the account route is not bound in app-chirho.toml (held on this pin)\n' >&2; exit 2; }
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

# ---- tenant root: owner, user, the workspace app ----
OWNER_CHIRHO="$("$CLI_CHIRHO" --root "$ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" identity-register --kind owner | cut -f1)"
OLD_PW_CHIRHO="$(openssl rand -base64 18 | tr -d '\n/+=')"; NEW_PW_CHIRHO="$(openssl rand -base64 18 | tr -d '\n/+=')"
PW_FILE_CHIRHO="$T_CHIRHO/cred-chirho.txt"; printf '%s' "$OLD_PW_CHIRHO" > "$PW_FILE_CHIRHO"; chmod 600 "$PW_FILE_CHIRHO"
PRINCIPAL_CHIRHO="$("$CLI_CHIRHO" --root "$ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" identity-register --kind user --credential-file "$PW_FILE_CHIRHO" | cut -f1)"
rm -f "$PW_FILE_CHIRHO"
# app-source-deploy builds cargo-example nodes from source; stage the prepared outputs once so this
# gate never runs cargo (the helper verifies every staged byte and refuses a missing artifact).
# shellcheck source=../lib-chirho/prebuilt-app-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/prebuilt-app-chirho.sh"
WORKSPACE_MANIFEST_CHIRHO="$(stage_prebuilt_app_chirho "$CAIRN_REPO_CHIRHO" "$REPO_CHIRHO/apps-chirho/perffection-workspace-chirho" perffection-workspace "$T_CHIRHO/stage-workspace-chirho")" || { printf 'refusing: could not stage the prepared workspace app without cargo\n' >&2; exit 2; }
"$CLI_CHIRHO" --root "$ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" app-source-deploy "$WORKSPACE_MANIFEST_CHIRHO" --owner "$OWNER_CHIRHO" --allow-unreviewed-dev > "$T_CHIRHO/deploy-workspace.out" 2>&1; expect_chirho "workspace deploy rc (staged, no cargo)" "$?" 0
"$CLI_CHIRHO" --root "$ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" serve --port "$TP_CHIRHO" > "$T_CHIRHO/tenant-serve.log" 2>&1 & TPID_CHIRHO=$!
trap 'kill "$TPID_CHIRHO" 2>/dev/null; wait "$TPID_CHIRHO" 2>/dev/null' EXIT
for _ in $(seq 1 80); do curl -s -o /dev/null "$TB_CHIRHO/auth-chirho/login-chirho" && break; sleep 0.5; done
expect_chirho "the server answering is the one this run started" "$(own_server_verdict_chirho "$TPID_CHIRHO" "$TP_CHIRHO")" own

WS_CHIRHO="$TB_CHIRHO/app-chirho/perffection-workspace"
ACCOUNT_CHIRHO="/app-chirho/perffection-workspace/account-chirho"
login_chirho() { # <password> <out-header-file> -> http code
  curl -s -o /dev/null -D "$2" -w '%{http_code}' --data-urlencode "principal_chirho=$PRINCIPAL_CHIRHO" --data-urlencode "credential_chirho=$1" --data-urlencode "redirect_chirho=/app-chirho/perffection-workspace/" "$TB_CHIRHO/auth-chirho/login-chirho"
}
cookie_of_chirho() { grep -i '^set-cookie' "$1" | sed -E 's/^[Ss]et-[Cc]ookie: ([^;]*);.*/\1/' | tr -d '\r' | head -1; }
change_chirho() { # <cookie> <current> <new> <redirect> <out-header-file> -> http code
  curl -s -o /dev/null -D "$5" -w '%{http_code}' -H "Cookie: $1" --data-urlencode "credential_chirho=$2" --data-urlencode "new_credential_chirho=$3" --data-urlencode "redirect_chirho=$4" "$TB_CHIRHO/auth-chirho/credential-change-chirho"
}
location_chirho() { grep -i '^location:' "$1" | sed -E 's/^[Ll]ocation: //' | tr -d '\r' | head -1; }

# 1. two live sessions
expect_chirho "login A" "$(login_chirho "$OLD_PW_CHIRHO" "$T_CHIRHO/login-a.hdr")" 303
expect_chirho "login B" "$(login_chirho "$OLD_PW_CHIRHO" "$T_CHIRHO/login-b.hdr")" 303
COOKIE_A_CHIRHO="$(cookie_of_chirho "$T_CHIRHO/login-a.hdr")"; COOKIE_B_CHIRHO="$(cookie_of_chirho "$T_CHIRHO/login-b.hdr")"
expect_chirho "two distinct sessions" "$([ -n "$COOKIE_A_CHIRHO" ] && [ "$COOKIE_A_CHIRHO" != "$COOKIE_B_CHIRHO" ] && echo yes || echo no)" yes
# 2. the Account page: the real change form, posting to the road with the way back
expect_chirho "account page" "$(curl -s -o "$T_CHIRHO/account.html" -w '%{http_code}' -H "Cookie: $COOKIE_A_CHIRHO" "$WS_CHIRHO/account-chirho")" 200
expect_chirho "account form posts to the road" "$(grep -c 'action="/auth-chirho/credential-change-chirho"' "$T_CHIRHO/account.html")/$(grep -c "name=\"redirect_chirho\" value=\"$ACCOUNT_CHIRHO\"" "$T_CHIRHO/account.html")/$(grep -c 'name="new_credential_chirho"' "$T_CHIRHO/account.html")" "1/1/1"
expect_chirho "account page pre-renders the refusals, hidden" "$(grep -c 'id="refusal-credential-mismatch-chirho"' "$T_CHIRHO/account.html")/$(grep -c 'workspace-refusal-chirho' "$T_CHIRHO/account.html")" "1/4"
# 3. refusals: wrong current password, too-short new one, foreign redirect
expect_chirho "mismatch" "$(change_chirho "$COOKIE_A_CHIRHO" "not-the-password-chirho" "$NEW_PW_CHIRHO" "$ACCOUNT_CHIRHO" "$T_CHIRHO/mismatch.hdr")/$(location_chirho "$T_CHIRHO/mismatch.hdr")" "303/${ACCOUNT_CHIRHO}#refusal-credential-mismatch-chirho"
expect_chirho "too weak" "$(change_chirho "$COOKIE_A_CHIRHO" "$OLD_PW_CHIRHO" "short" "$ACCOUNT_CHIRHO" "$T_CHIRHO/weak.hdr")/$(location_chirho "$T_CHIRHO/weak.hdr")" "303/${ACCOUNT_CHIRHO}#refusal-credential-too-weak-chirho"
expect_chirho "foreign redirect falls back to /" "$(change_chirho "$COOKIE_A_CHIRHO" "not-the-password-chirho" "$NEW_PW_CHIRHO" "https://evil.example/steal" "$T_CHIRHO/foreign.hdr")/$(location_chirho "$T_CHIRHO/foreign.hdr")" "303//#refusal-credential-mismatch-chirho"
expect_chirho "sessions still live after refusals" "$(curl -s -o /dev/null -w '%{http_code}' -H "Cookie: $COOKIE_A_CHIRHO" "$WS_CHIRHO/")/$(curl -s -o /dev/null -w '%{http_code}' -H "Cookie: $COOKIE_B_CHIRHO" "$WS_CHIRHO/")" "200/200"
# 4. the accepted change: cookie cleared, redirect to login, BOTH sessions dead
expect_chirho "accepted change" "$(change_chirho "$COOKIE_A_CHIRHO" "$OLD_PW_CHIRHO" "$NEW_PW_CHIRHO" "$ACCOUNT_CHIRHO" "$T_CHIRHO/accepted.hdr")/$(location_chirho "$T_CHIRHO/accepted.hdr")/$(grep -ci 'set-cookie: cairn_session_chirho=;.*max-age=0' "$T_CHIRHO/accepted.hdr")" "303//auth-chirho/login-chirho/1"
# a revoked session presented to the workspace is refused as 403 (an anonymous request is 401)
expect_chirho "both pre-change sessions dead on the workspace" "$(curl -s -o /dev/null -w '%{http_code}' -H "Cookie: $COOKIE_A_CHIRHO" "$WS_CHIRHO/")/$(curl -s -o /dev/null -w '%{http_code}' -H "Cookie: $COOKIE_B_CHIRHO" "$WS_CHIRHO/")" "403/403"
expect_chirho "dead session on the door is no-session" "$(change_chirho "$COOKIE_B_CHIRHO" "$NEW_PW_CHIRHO" "$OLD_PW_CHIRHO" "$ACCOUNT_CHIRHO" "$T_CHIRHO/dead.hdr")/$(location_chirho "$T_CHIRHO/dead.hdr")" "303//auth-chirho/login-chirho#refusal-no-session-chirho"
# 5. the old password refuses at login the canonical way (cairn d60037f3: the browser form road
#    303s back to the login page with the fixed invalid-credential fragment and sets no cookie —
#    asserted against that contract, not against another wrong login), the new one signs in
expect_chirho "old password refused at login, no cookie" "$(login_chirho "$OLD_PW_CHIRHO" "$T_CHIRHO/login-old.hdr")/$(location_chirho "$T_CHIRHO/login-old.hdr")/$(grep -ci '^set-cookie' "$T_CHIRHO/login-old.hdr")" "303//auth-chirho/login-chirho#refusal-invalid-credential-chirho/0"
expect_chirho "a wrong password refused the same way" "$(login_chirho "definitely-wrong-chirho" "$T_CHIRHO/login-wrong.hdr")/$(location_chirho "$T_CHIRHO/login-wrong.hdr")/$(grep -ci '^set-cookie' "$T_CHIRHO/login-wrong.hdr")" "303//auth-chirho/login-chirho#refusal-invalid-credential-chirho/0"
expect_chirho "new password signs in" "$(login_chirho "$NEW_PW_CHIRHO" "$T_CHIRHO/login-new.hdr")" 303
COOKIE_C_CHIRHO="$(cookie_of_chirho "$T_CHIRHO/login-new.hdr")"
expect_chirho "new session reaches the workspace" "$(curl -s -o /dev/null -w '%{http_code}' -H "Cookie: $COOKIE_C_CHIRHO" "$WS_CHIRHO/")" 200
# 6. the guess budget is PER SESSION (landed R1, claude2 F2): five failed proofs from one session
#    revoke that session rather than throttle it; a stolen cookie loses access, the owner's login
#    budget is never spent, and a fresh login starts a fresh budget
for _ in 1 2 3 4; do change_chirho "$COOKIE_C_CHIRHO" "wrong-guess-chirho" "$OLD_PW_CHIRHO" "$ACCOUNT_CHIRHO" "$T_CHIRHO/guess.hdr" > /dev/null; done
expect_chirho "fourth wrong guess: session still live" "$(curl -s -o /dev/null -w '%{http_code}' -H "Cookie: $COOKIE_C_CHIRHO" "$WS_CHIRHO/")" 200
expect_chirho "fifth wrong guess signs her out to the login page, cookie cleared" "$(change_chirho "$COOKIE_C_CHIRHO" "wrong-guess-chirho" "$OLD_PW_CHIRHO" "$ACCOUNT_CHIRHO" "$T_CHIRHO/guess5.hdr")/$(location_chirho "$T_CHIRHO/guess5.hdr")/$(grep -ci 'set-cookie: cairn_session_chirho=;.*max-age=0' "$T_CHIRHO/guess5.hdr")" "303//auth-chirho/login-chirho#refusal-credential-guess-exhausted-chirho/1"
# the browser follows that redirect without the dead cookie and without the fragment: a page she can open
expect_chirho "the exhaustion return lands on a page (followed as a browser would)" "$(curl -s -o "$T_CHIRHO/exhausted-landing.html" -w '%{http_code}' "$TB_CHIRHO$(location_chirho "$T_CHIRHO/guess5.hdr" | cut -d'#' -f1)")/$(curl -s -o /dev/null -w '%{http_code}' "$WS_CHIRHO/")" "200/401"
expect_chirho "the fifth wrong guess revoked that session" "$(curl -s -o /dev/null -w '%{http_code}' -H "Cookie: $COOKIE_C_CHIRHO" "$WS_CHIRHO/")/$(change_chirho "$COOKIE_C_CHIRHO" "wrong-guess-chirho" "$OLD_PW_CHIRHO" "$ACCOUNT_CHIRHO" "$T_CHIRHO/guess6.hdr")/$(location_chirho "$T_CHIRHO/guess6.hdr")" "403/303//auth-chirho/login-chirho#refusal-no-session-chirho"
expect_chirho "revocation is on the record" "$("$CLI_CHIRHO" --root "$ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" identity-sessions-list "$PRINCIPAL_CHIRHO" 2>/dev/null | grep -ci 'revoked')" "$("$CLI_CHIRHO" --root "$ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" identity-sessions-list "$PRINCIPAL_CHIRHO" 2>/dev/null | grep -ci 'revoked' | sed 's/^0$/at-least-one/;s/^[1-9][0-9]*$/&/')"
expect_chirho "the owner's login budget is untouched" "$(login_chirho "$NEW_PW_CHIRHO" "$T_CHIRHO/login-after-lockout.hdr")" 303
COOKIE_D_CHIRHO="$(cookie_of_chirho "$T_CHIRHO/login-after-lockout.hdr")"
expect_chirho "a fresh session starts a fresh budget" "$(change_chirho "$COOKIE_D_CHIRHO" "wrong-guess-chirho" "$OLD_PW_CHIRHO" "$ACCOUNT_CHIRHO" "$T_CHIRHO/guess-d.hdr")/$(location_chirho "$T_CHIRHO/guess-d.hdr")/$(curl -s -o /dev/null -w '%{http_code}' -H "Cookie: $COOKIE_D_CHIRHO" "$WS_CHIRHO/")" "303/${ACCOUNT_CHIRHO}#refusal-credential-mismatch-chirho/200"
# 7. neither password is written anywhere in the root. An absence is only evidence if the scan
#    could both see and fail, so the shared fail-closed instrument reads it (gpt #24135): it sees
#    clear text in this root, and a missing root or an unreadable file is a scan error, not a 0.
# shellcheck source=../lib-chirho/root-scan-chirho.sh
source "$REPO_CHIRHO/scripts-chirho/lib-chirho/root-scan-chirho.sh"
expect_chirho "control: the root scan sees clear text in this root" "$(seen_verdict_chirho "$(scan_root_chirho 'account-chirho' "$ROOT_CHIRHO")")" sees
scan_controls_chirho "$NEW_PW_CHIRHO" "$T_CHIRHO"
expect_chirho "no secret in the root" "$(scan_root_chirho "$OLD_PW_CHIRHO" "$ROOT_CHIRHO")/$(scan_root_chirho "$NEW_PW_CHIRHO" "$ROOT_CHIRHO")" "0/0"
say_chirho "VERDICT: $FAILS_CHIRHO failed assertion(s); evidence in $T_CHIRHO"
exit $(( FAILS_CHIRHO > 0 ))
