#!/usr/bin/env bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
#
# Staging a scratch workspace app with its held routes, openings and bindings, and issuing keys, for the served
# gates that need them (tests-chirho/figures-chirho/key-grants-rehearsal-chirho.sh,
# tests-chirho/account-chirho/account-keys-rehearsal-chirho.sh). Sourced after gate-helpers-chirho.sh, the runtime,
# emit and prebuilt libs. Each function reads the caller's T_CHIRHO, SCRATCH_CHIRHO, APP_CHIRHO, REPO_CHIRHO,
# CAIRN_REPO_CHIRHO, TREE_BUILD_CHIRHO, OWNER_CHIRHO, CLIENT_CHIRHO and WS_CHIRHO when it runs, and the caller's
# cli_chirho (its engine on its own root). Keys and credentials live only in mode-600 files the caller removes at exit.
# Workflows: spec-chirho/workflows-chirho/invite-beta-chirho/her-figures-chirho.md and agent-api-chirho.md

# new_user_chirho <cred name>: a synthetic user with a random credential; prints the principal
new_user_chirho() { openssl rand -base64 18 | tr -d '\n' > "$T_CHIRHO/$1-cred-chirho.txt"; chmod 600 "$T_CHIRHO/$1-cred-chirho.txt"; cli_chirho identity-register --kind user --credential-file "$T_CHIRHO/$1-cred-chirho.txt" | cut -f1; }

# compose_scratch_app_chirho <stages | profile:<name>> <label>: the scratch mirror of the real app (every entry linked
# but its app-chirho.toml and emitted manifest) with those stages' held openings and bindings (compose-held-app-chirho.py),
# checked and emitted by the caller's tree-build. The composed TOML and the check are kept under <label>. Also used by
# the release candidate's driver (tests-chirho/account-chirho/connected-release-driver-chirho.sh).
compose_scratch_app_chirho() {
  rm -rf "$SCRATCH_CHIRHO"; mkdir -p "$SCRATCH_CHIRHO"
  for entry_chirho in "$APP_CHIRHO"/* "$APP_CHIRHO"/.[!.]*; do
    name_chirho="$(basename "$entry_chirho")"
    case "$name_chirho" in app-chirho.toml|.emitted-manifest-chirho.json|.[!.]*\*) continue ;; esac
    ln -s "$entry_chirho" "$SCRATCH_CHIRHO/$name_chirho"
  done
  python3 "$REPO_CHIRHO/scripts-chirho/lib-chirho/compose-held-app-chirho.py" "$APP_CHIRHO" "$1" "$SCRATCH_CHIRHO/app-chirho.toml" || return 1
  cp "$SCRATCH_CHIRHO/app-chirho.toml" "$T_CHIRHO/app-$2.toml"
  "$TREE_BUILD_CHIRHO" check --json "$SCRATCH_CHIRHO" > "$T_CHIRHO/check-$2.json" 2>&1 || return 1
  emit_app_manifest_chirho "$TREE_BUILD_CHIRHO" "$REPO_CHIRHO" "$SCRATCH_CHIRHO" "$SCRATCH_CHIRHO/.emitted-manifest-chirho.json"
}

# deploy_stage_chirho <stages> [<route>=<other route>]: the scratch composition (compose_scratch_app_chirho), staged and
# deployed; each deploy's evidence under its own number (the stager refuses a stage dir that is not empty). A
# <route>=<other route> makes that route, keeping its name and opening, run the other route's module, as Cairn's own K1
# tests do: the gate's deliberate change of what an opened route does, made in the scratch emitted manifest only (two
# routes cannot name one DAG in app-chirho.toml).
DEPLOYS_CHIRHO=0
deploy_stage_chirho() {
  DEPLOYS_CHIRHO=$((DEPLOYS_CHIRHO + 1)); local label_chirho="$DEPLOYS_CHIRHO-$1${2:+-moved}"
  compose_scratch_app_chirho "$1" "$label_chirho" || return 1
  if [ -n "${2:-}" ]; then python3 - "$SCRATCH_CHIRHO/.emitted-manifest-chirho.json" "${2%%=*}" "${2#*=}" <<'PY' || return 1
import json, sys
path, route, donor = sys.argv[1:4]
manifest = json.load(open(path, encoding="utf-8"))
routes = {entry["route_chirho"]: entry for entry in manifest["deploy_chirho"]["routes_chirho"]}
assert routes[route]["source_chirho"] != routes[donor]["source_chirho"], "already the same module"
routes[route]["source_chirho"] = dict(routes[donor]["source_chirho"])
json.dump(manifest, open(path, "w", encoding="utf-8"), indent=2)
PY
  fi
  cp "$SCRATCH_CHIRHO/.emitted-manifest-chirho.json" "$T_CHIRHO/manifest-$label_chirho.json"
  local manifest_chirho
  manifest_chirho="$(stage_prebuilt_app_chirho "$CAIRN_REPO_CHIRHO" "$SCRATCH_CHIRHO" perffection-workspace "$T_CHIRHO/stage-$label_chirho")" || return 1
  cli_chirho app-source-deploy "$manifest_chirho" --owner "$OWNER_CHIRHO" --allow-unreviewed-dev > "$T_CHIRHO/deploy-$label_chirho.out" 2>&1
}
# The issue output carries the key: the key goes to a mode-600 file removed at exit, and only a copy
# with the key blanked is kept as evidence.
keep_issue_chirho() {
  tr '\t' '\n' < "$T_CHIRHO/issue-$1-key-chirho.txt" | sed -n 's/^token=//p' | head -1 > "$T_CHIRHO/$1-key-chirho.txt"
  sed -E 's/token=[^[:space:]]*/token=[blanked]/' "$T_CHIRHO/issue-$1-key-chirho.txt" > "$T_CHIRHO/issue-$1.out"
  rm -f "$T_CHIRHO/issue-$1-key-chirho.txt"; chmod 600 "$T_CHIRHO/$1-key-chirho.txt"; [ -s "$T_CHIRHO/$1-key-chirho.txt" ]
}
# limited_key_chirho <name> <user> <cred name> <action...>; legacy_key_chirho <name> <user> <cred name>
limited_key_chirho() { local name_chirho="$1" user_chirho="$2" cred_chirho="$3"; shift 3; local args_chirho=()
  for action_chirho in "$@"; do args_chirho+=(--action "$action_chirho"); done
  cli_chirho oauth-limited-key --principal "$user_chirho" --credential-file "$T_CHIRHO/$cred_chirho-cred-chirho.txt" --client "$CLIENT_CHIRHO" "${args_chirho[@]}" --rate-window-ms 3600000 --rate-max-invokes 200 > "$T_CHIRHO/issue-$name_chirho-key-chirho.txt" 2>&1
  keep_issue_chirho "$name_chirho"; }
legacy_key_chirho() {
  cli_chirho oauth-token --principal "$2" --credential-file "$T_CHIRHO/$3-cred-chirho.txt" --client "$CLIENT_CHIRHO" --scope page-draft-chirho --rate-window-ms 3600000 --rate-max-invokes 200 > "$T_CHIRHO/issue-$1-key-chirho.txt" 2>&1
  local issued_chirho=$?; keep_issue_chirho "$1"; return "$issued_chirho"; }
# call_chirho <key name> <GET|POST> <route> <out> [body file] [base] [accept]: the status; headers and
# body kept beside it. JSON is asked for, as the MCP adapter asks, unless another Accept is given.
call_chirho() {
  local body_args_chirho=(); [ -n "${5:-}" ] && body_args_chirho=(-H 'Content-Type: application/json' --data-binary "@$T_CHIRHO/$5")
  curl -s -o "$T_CHIRHO/$4" -D "$T_CHIRHO/$4.hdr" -w '%{http_code}' -X "$2" -H "Accept: ${7:-application/json}" \
    -H "Authorization: Bearer $(cat "$T_CHIRHO/$1-key-chirho.txt")" ${body_args_chirho[@]+"${body_args_chirho[@]}"} "${6:-$WS_CHIRHO}/$3"
}
# refusal_chirho <key> <GET|POST> <route> <label> [body] [accept]: "status|content type|body", the answer kept
refusal_chirho() {
  local status_chirho; status_chirho="$(call_chirho "$1" "$2" "$3" "refusals-chirho/$4" "${5:-}" "" "${6:-}")"
  printf '%s|%s|%s' "$status_chirho" "$(header_chirho "refusals-chirho/$4" content-type)" "$(tr '\n' '~' < "$T_CHIRHO/refusals-chirho/$4")"
}
# K1's fixed refusals, as the adapter receives them (Accept: application/json)
k1_json_chirho() { printf '%s|application/json|{"error_chirho":"%s","ok_chirho":false}' "$1" "$2"; }
