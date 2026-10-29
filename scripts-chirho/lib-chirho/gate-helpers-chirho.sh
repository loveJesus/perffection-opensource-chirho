#!/usr/bin/env bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
#
# The served workspace gates' shared helpers (agent-api-rehearsal-chirho.sh, figures-rehearsal-chirho.sh):
# a logged verdict per arm, known engine gaps that fail the day the engine changes, JSON read as data
# and never as code, keys sent only from mode-600 files, and a real browser sign-in. Each reads the
# caller's T_CHIRHO (the rehearsal dir), LOG_CHIRHO, TB_CHIRHO (the tenant base URL) and WS_CHIRHO (the
# workspace base URL) when it runs. The caller reports FAILS_CHIRHO and GAPS_CHIRHO in its verdict.
say_chirho() { printf '%s\n' "$*" | tee -a "$LOG_CHIRHO"; }
FAILS_CHIRHO=0; GAPS_CHIRHO=0
expect_chirho() { if [ "$2" = "$3" ]; then say_chirho "ok   $1 = $2"; else say_chirho "FAIL $1 = $2 (expected $3)"; FAILS_CHIRHO=$((FAILS_CHIRHO + 1)); fi; }
# gap_chirho <label> <observed> <the value measured while the gap is open>
gap_chirho() { if [ "$2" = "$3" ]; then say_chirho "gap  $1 = $2 (known engine gap, unchanged)"; GAPS_CHIRHO=$((GAPS_CHIRHO + 1)); else say_chirho "FAIL $1 = $2 (the known gap read $3: the engine changed - make this arm a hard expectation)"; FAILS_CHIRHO=$((FAILS_CHIRHO + 1)); fi; }
# api_get_chirho <key name> <route> <out>; api_post_chirho <key name> <route> <body file> <out>: status code; headers beside the body
api_get_chirho() { curl -s -o "$T_CHIRHO/$3" -D "$T_CHIRHO/$3.hdr" -w '%{http_code}' -H "Authorization: Bearer $(cat "$T_CHIRHO/$1-key-chirho.txt")" "$WS_CHIRHO/$2"; }
api_post_chirho() { curl -s -o "$T_CHIRHO/$4" -D "$T_CHIRHO/$4.hdr" -w '%{http_code}' -H "Authorization: Bearer $(cat "$T_CHIRHO/$1-key-chirho.txt")" -H 'Content-Type: application/json' --data-binary "@$T_CHIRHO/$3" "$WS_CHIRHO/$2"; }
header_chirho() { grep -i "^$2:" "$T_CHIRHO/$1.hdr" | head -1 | sed -E 's/^[^:]*:[[:space:]]*//' | tr -d '\r' | tr 'A-Z' 'a-z'; }
# register_client_chirho <out file>: registers the workspace app's OAuth client and prints its id, read from the
# registration's own answer (`client/<16 hex>` on every engine since the pin). From Cairn's K4 Part 3 (3c7bc083) the client
# of an app opened to keys lives in the connection store, which the legacy `oauth-clients` listing never shows, so a
# listing is never the source (measured 2026-10-10 at ad4bc794). Uses the gate's cli_chirho, OWNER_CHIRHO and T_CHIRHO.
register_client_chirho() { cli_chirho oauth-client-register --app perffection-workspace --owner "$OWNER_CHIRHO" > "$T_CHIRHO/$1" 2>&1 || return 1
  grep -oE 'client/[0-9a-f]{16}' "$T_CHIRHO/$1" | head -1; }
# json_chirho <file> <python expression over d>: one value read out of a JSON answer, or "not-json".
# The expression is always a literal written in THIS script (never served or supplied text), and it
# is evaluated without builtins over the parsed answer only; the served bytes are data, not code.
json_chirho() { python3 - "$T_CHIRHO/$1" "$2" <<'PY'
import json, sys
try:
    d = json.load(open(sys.argv[1], encoding="utf-8"))
except Exception:
    print("not-json"); sys.exit(0)
try:
    v = eval(sys.argv[2], {"__builtins__": {}, "d": d, "len": len, "sorted": sorted, "any": any, "all": all})
except Exception as e:
    v = f"unreadable:{type(e).__name__}"
print(json.dumps(v) if isinstance(v, (dict, list)) else ("null" if v is None else str(v).lower() if isinstance(v, bool) else v))
PY
}
refused_chirho() { printf '%s' "$1" | sed -E 's/^(30[0-9]|401|403)$/refused/'; }
count_chirho() { grep -o -- "$1" "$T_CHIRHO/$2" | wc -l | tr -d ' '; }
# sign_in_chirho <principal> <credential name> <header file>: a real browser sign-in, its status code
sign_in_chirho() { curl -s -o /dev/null -D "$T_CHIRHO/$3" -w '%{http_code}' --data-urlencode "principal_chirho=$1" --data-urlencode "credential_chirho=$(cat "$T_CHIRHO/$2-cred-chirho.txt")" --data-urlencode "redirect_chirho=/app-chirho/perffection-workspace/" "$TB_CHIRHO/auth-chirho/login-chirho"; }
session_cookie_chirho() { grep -i '^set-cookie' "$T_CHIRHO/$1" | sed -E 's/^[Ss]et-[Cc]ookie: ([^;]*);.*/\1/' | tr -d '\r' | head -1; }
# The engine's own answer-cache layer (Cairn K2, landed at 1f45c632): every answer not marked public leaves with
# private, no-store. Probed once on a road no app owns, as Cairn's own witness does, so a product arm is a hard
# expectation on an engine with the layer and a known gap on one before it. 1 or 0.
engine_private_answers_chirho() { curl -s -o /dev/null -D - "$TB_CHIRHO/no-such-road-chirho" | tr -d '\r' | grep -i -c '^cache-control: private, no-store$'; }
# no_store_chirho <answer file>: "<1 if its header file holds a 200>/<1 if it is no-store>", so a missing answer fails
no_store_chirho() { echo "$(head -1 "$T_CHIRHO/$1.hdr" 2>/dev/null | grep -c ' 200')/$(header_chirho "$1" cache-control 2>/dev/null | grep -c 'no-store')"; }
# private_answer_arm_chirho <label> <no_store_chirho value>: reads ENGINE_PRIVATE_CHIRHO, set after the server starts
private_answer_arm_chirho() { if [ "${ENGINE_PRIVATE_CHIRHO:-0}" = 1 ]; then expect_chirho "$1" "$2" "1/1"; else gap_chirho "$1" "$2" "1/0"; fi; }
