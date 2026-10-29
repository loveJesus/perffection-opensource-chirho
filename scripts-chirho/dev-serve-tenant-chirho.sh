#!/usr/bin/env bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
#
# One-origin local product loop: public display runs on a platform root; auth and the private
# workspace run on a tenant root; loopback Caddy composes them for real same-origin sign-in.
# Registration stays closed. An operator may explicitly request one disposable synthetic account.
# Workflow: spec-chirho/workflows-chirho/product-delivery-chirho/product-authoring-chirho.md.

set -euo pipefail

APP_ROOT_CHIRHO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$APP_ROOT_CHIRHO/scripts-chirho/lib-chirho/prebuilt-app-chirho.sh"
source "$APP_ROOT_CHIRHO/scripts-chirho/lib-chirho/cairn-runtime-chirho.sh"
CAIRN_REPO_CHIRHO="${CAIRN_REPO_CHIRHO:-$(cd "$APP_ROOT_CHIRHO/../cairn-opensource-chirho" && pwd)}"
CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$APP_ROOT_CHIRHO/target}"
export CARGO_TARGET_DIR
PORT_CHIRHO="${PORT_CHIRHO:-8840}"
PORT_PUBLIC_CHIRHO="${PORT_PUBLIC_CHIRHO:-8841}"
PORT_TENANT_CHIRHO="${PORT_TENANT_CHIRHO:-8842}"
CADDY_ADMIN_PORT_CHIRHO="${CADDY_ADMIN_PORT_CHIRHO:-8843}"
TENANT_CHIRHO="${TENANT_CHIRHO:-beta-chirho}"
READY_TIMEOUT_CHIRHO="${READY_TIMEOUT_CHIRHO:-240}"
REQUESTED_STATE_DIR_CHIRHO="${STATE_DIR_CHIRHO:-}"
COMMAND_CHIRHO="serve-chirho"
ISSUE_SYNTHETIC_ACCOUNT_CHIRHO=0
DEV_ROOT_CHIRHO=""
STATE_DIR_CHIRHO=""
OWN_STATE_DIR_CHIRHO=0
PUBLIC_PID_CHIRHO=""
TENANT_PID_CHIRHO=""
CADDY_PID_CHIRHO=""

usage_chirho() {
  cat <<'USAGE_CHIRHO'
usage: dev-serve-tenant-chirho.sh [serve-chirho|await-chirho] [--issue-synthetic-account-chirho]

The optional account flag creates one local-only user credential in a mode-600 state file and
prints the file path, never the value. Without it, the ready message prints the exact operator CLI
shape for issuing a credential from another terminal. HTTP registration always remains closed.
USAGE_CHIRHO
}

while (( $# > 0 )); do
  case "$1" in
    serve-chirho|await-chirho)
      COMMAND_CHIRHO="$1"
      shift
      ;;
    --issue-synthetic-account-chirho)
      ISSUE_SYNTHETIC_ACCOUNT_CHIRHO=1
      shift
      ;;
    -h|--help-chirho)
      usage_chirho
      exit 0
      ;;
    *)
      printf 'unknown argument: %s\n' "$1" >&2
      usage_chirho >&2
      exit 2
      ;;
  esac
done

if [[ "$PORT_CHIRHO" == "$PORT_PUBLIC_CHIRHO" \
  || "$PORT_CHIRHO" == "$PORT_TENANT_CHIRHO" \
  || "$PORT_CHIRHO" == "$CADDY_ADMIN_PORT_CHIRHO" \
  || "$PORT_PUBLIC_CHIRHO" == "$PORT_TENANT_CHIRHO" \
  || "$PORT_PUBLIC_CHIRHO" == "$CADDY_ADMIN_PORT_CHIRHO" \
  || "$PORT_TENANT_CHIRHO" == "$CADDY_ADMIN_PORT_CHIRHO" ]]; then
  printf '%s\n' 'edge, public, tenant, and Caddy admin ports must all be distinct' >&2
  exit 2
fi

http_code_chirho() {
  local method_chirho="$1"
  local path_chirho="$2"
  curl -sS --connect-timeout 1 --max-time 1 -o /dev/null -w '%{http_code}' -X "$method_chirho" \
    "http://127.0.0.1:${PORT_CHIRHO}${path_chirho}" 2>/dev/null || true
}

await_ready_chirho() {
  local start_seconds_chirho=$SECONDS
  local waited_chirho=0
  local public_code_chirho="000"
  local tenant_code_chirho="000"
  local health_code_chirho="000"
  local human_code_chirho="000"
  local register_get_code_chirho="000"
  local register_post_code_chirho="000"
  while (( SECONDS - start_seconds_chirho < READY_TIMEOUT_CHIRHO )); do
    public_code_chirho="$(http_code_chirho GET /app-chirho/perffection/login-chirho)"
    tenant_code_chirho="$(http_code_chirho GET /app-chirho/perffection-workspace/)"
    health_code_chirho="$(http_code_chirho GET /healthz-chirho)"
    human_code_chirho="$(http_code_chirho GET /auth-chirho/human-check-chirho)"
    register_get_code_chirho="$(http_code_chirho GET /auth-chirho/register-chirho)"
    register_post_code_chirho="$(http_code_chirho POST /auth-chirho/register-chirho)"
    if [[ "$public_code_chirho" == "200" \
      && "$tenant_code_chirho" == "401" \
      && "$health_code_chirho" == "200" \
      && "$human_code_chirho" == "503" \
      && "$register_get_code_chirho" == "503" \
      && "$register_post_code_chirho" == "503" ]]; then
      printf 'ready on http://127.0.0.1:%s/ after %ss (public=200 tenant-anon=401 health=200 registration=503/503/503)\n' \
        "$PORT_CHIRHO" "$waited_chirho"
      return 0
    fi
    if [[ -n "$PUBLIC_PID_CHIRHO" ]] && ! kill -0 "$PUBLIC_PID_CHIRHO" 2>/dev/null; then
      break
    fi
    if [[ -n "$TENANT_PID_CHIRHO" ]] && ! kill -0 "$TENANT_PID_CHIRHO" 2>/dev/null; then
      break
    fi
    if [[ -n "$CADDY_PID_CHIRHO" ]] && ! kill -0 "$CADDY_PID_CHIRHO" 2>/dev/null; then
      break
    fi
    sleep 1
    waited_chirho=$((SECONDS - start_seconds_chirho))
  done

  printf 'local product loop failed readiness within %ss (public=%s tenant-anon=%s health=%s registration=%s/%s/%s)\n' \
    "$READY_TIMEOUT_CHIRHO" "$public_code_chirho" "$tenant_code_chirho" "$health_code_chirho" \
    "$human_code_chirho" "$register_get_code_chirho" "$register_post_code_chirho" >&2
  if [[ -n "$STATE_DIR_CHIRHO" ]]; then
    local log_chirho
    for log_chirho in "$STATE_DIR_CHIRHO"/*-log-chirho.txt; do
      [[ -f "$log_chirho" ]] || continue
      printf '%s\n' "--- ${log_chirho} (tail) ---" >&2
      tail -20 "$log_chirho" >&2
    done
  fi
  return 1
}

if [[ "$COMMAND_CHIRHO" == "await-chirho" ]]; then
  await_ready_chirho
  exit $?
fi

cleanup_chirho() {
  local pid_chirho
  for pid_chirho in "$CADDY_PID_CHIRHO" "$TENANT_PID_CHIRHO" "$PUBLIC_PID_CHIRHO"; do
    if [[ -n "$pid_chirho" ]] && kill -0 "$pid_chirho" 2>/dev/null; then
      kill "$pid_chirho" 2>/dev/null || true
      wait "$pid_chirho" 2>/dev/null || true
    fi
  done
  if [[ -n "$DEV_ROOT_CHIRHO" && "$DEV_ROOT_CHIRHO" == "$CARGO_TARGET_DIR"/perffection-tenant-root-chirho.* ]]; then
    rm -rf -- "$DEV_ROOT_CHIRHO"
  fi
  if (( OWN_STATE_DIR_CHIRHO == 1 )) \
    && [[ -n "$STATE_DIR_CHIRHO" && "$STATE_DIR_CHIRHO" == "$CARGO_TARGET_DIR"/perffection-dev-state-chirho.* ]]; then
    rm -rf -- "$STATE_DIR_CHIRHO"
  fi
}
trap cleanup_chirho EXIT INT TERM

CAIRN_CLI_CHIRHO="$(resolve_cairn_runtime_chirho "$APP_ROOT_CHIRHO" "$CAIRN_REPO_CHIRHO")"
if ! command -v caddy >/dev/null 2>&1; then
  printf '%s\n' 'missing caddy executable; install Caddy before starting the one-origin loop' >&2
  exit 1
fi
if (( ISSUE_SYNTHETIC_ACCOUNT_CHIRHO == 1 )) && ! command -v openssl >/dev/null 2>&1; then
  printf '%s\n' 'missing openssl executable needed for the explicit synthetic credential' >&2
  exit 1
fi

cd "$APP_ROOT_CHIRHO"
if [[ "${SKIP_PREPARE_CHIRHO:-0}" != "1" ]]; then
  "$APP_ROOT_CHIRHO/scripts-chirho/prepare-apps-chirho.sh" --app-chirho all
fi

mkdir -p "$CARGO_TARGET_DIR"
DEV_ROOT_CHIRHO="$(mktemp -d "$CARGO_TARGET_DIR/perffection-tenant-root-chirho.XXXXXX")"
if [[ -n "$REQUESTED_STATE_DIR_CHIRHO" ]]; then
  STATE_DIR_CHIRHO="$REQUESTED_STATE_DIR_CHIRHO"
  if [[ -e "$STATE_DIR_CHIRHO" ]]; then
    printf 'refusing existing STATE_DIR_CHIRHO: %s\n' "$STATE_DIR_CHIRHO" >&2
    exit 1
  fi
  mkdir -m 700 "$STATE_DIR_CHIRHO"
else
  STATE_DIR_CHIRHO="$(mktemp -d "$CARGO_TARGET_DIR/perffection-dev-state-chirho.XXXXXX")"
  OWN_STATE_DIR_CHIRHO=1
fi
chmod 700 "$STATE_DIR_CHIRHO"

printf '%s\n' "$DEV_ROOT_CHIRHO" >"$STATE_DIR_CHIRHO/root-path-chirho.txt"
printf '%s\n' "$TENANT_CHIRHO" >"$STATE_DIR_CHIRHO/tenant-chirho.txt"
chmod 600 "$STATE_DIR_CHIRHO/root-path-chirho.txt" "$STATE_DIR_CHIRHO/tenant-chirho.txt"

# app-source-deploy builds cargo-example nodes even when SKIP_PREPARE is set. Convert the prepared
# outputs once, before creating an identity or deploying anything, so this loop cannot rebuild them.
SITE_MANIFEST_CHIRHO="$(stage_prebuilt_app_chirho "$CAIRN_REPO_CHIRHO" "$APP_ROOT_CHIRHO" \
  perffection "$STATE_DIR_CHIRHO/apps-chirho/site-chirho")"
PAGE_MANIFEST_CHIRHO="$(stage_prebuilt_app_chirho "$CAIRN_REPO_CHIRHO" \
  "$APP_ROOT_CHIRHO/apps-chirho/perffection-page-template-chirho" perffection-page-template-chirho \
  "$STATE_DIR_CHIRHO/apps-chirho/page-template-chirho")"
WORKSPACE_MANIFEST_CHIRHO="$(stage_prebuilt_app_chirho "$CAIRN_REPO_CHIRHO" \
  "$APP_ROOT_CHIRHO/apps-chirho/perffection-workspace-chirho" perffection-workspace \
  "$STATE_DIR_CHIRHO/apps-chirho/workspace-chirho")"

owner_of_chirho() {
  local output_chirho
  output_chirho="$("$CAIRN_CLI_CHIRHO" --root "$DEV_ROOT_CHIRHO" "$@" identity-register --kind owner)"
  printf '%s' "${output_chirho%%$'\t'*}"
}

printf '%s\n' '  platform root: deploy site (public display)'
PLATFORM_OWNER_CHIRHO="$(owner_of_chirho)"
"$CAIRN_CLI_CHIRHO" --root "$DEV_ROOT_CHIRHO" \
  app-source-deploy "$SITE_MANIFEST_CHIRHO" \
  --owner "$PLATFORM_OWNER_CHIRHO" --allow-unreviewed-dev >/dev/null

printf '%s\n' '  platform root: deploy customer page template (public display)'
"$CAIRN_CLI_CHIRHO" --root "$DEV_ROOT_CHIRHO" \
  app-source-deploy "$PAGE_MANIFEST_CHIRHO" \
  --owner "$PLATFORM_OWNER_CHIRHO" --allow-unreviewed-dev >/dev/null

printf '  tenant %s: deploy workspace (authenticated)\n' "$TENANT_CHIRHO"
TENANT_OWNER_CHIRHO="$(owner_of_chirho --tenant "$TENANT_CHIRHO")"
"$CAIRN_CLI_CHIRHO" --root "$DEV_ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" \
  app-source-deploy "$WORKSPACE_MANIFEST_CHIRHO" \
  --owner "$TENANT_OWNER_CHIRHO" --allow-unreviewed-dev >/dev/null
"$CAIRN_CLI_CHIRHO" --root "$DEV_ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" tenant-app-verify >/dev/null

if (( ISSUE_SYNTHETIC_ACCOUNT_CHIRHO == 1 )); then
  SYNTHETIC_CREDENTIAL_FILE_CHIRHO="$STATE_DIR_CHIRHO/synthetic-user-credential-chirho.txt"
  SYNTHETIC_PRINCIPAL_FILE_CHIRHO="$STATE_DIR_CHIRHO/synthetic-user-principal-chirho.txt"
  ( umask 077 && openssl rand -hex 24 >"$SYNTHETIC_CREDENTIAL_FILE_CHIRHO" )
  SYNTHETIC_OUTPUT_CHIRHO="$("$CAIRN_CLI_CHIRHO" --root "$DEV_ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" \
    identity-register --kind user --credential-file "$SYNTHETIC_CREDENTIAL_FILE_CHIRHO")"
  printf '%s\n' "${SYNTHETIC_OUTPUT_CHIRHO%%$'\t'*}" >"$SYNTHETIC_PRINCIPAL_FILE_CHIRHO"
  chmod 600 "$SYNTHETIC_PRINCIPAL_FILE_CHIRHO"
fi

CADDYFILE_CHIRHO="$STATE_DIR_CHIRHO/Caddyfile-chirho"
cat >"$CADDYFILE_CHIRHO" <<CADDYFILE_CHIRHO
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
# Generated local-only composition. Tenant/auth matchers must remain before public residual paths.
{
	admin 127.0.0.1:${CADDY_ADMIN_PORT_CHIRHO}
	auto_https off
	persist_config off
	servers {
		protocols h1
	}
	storage file_system {
		root ${STATE_DIR_CHIRHO}/caddy-storage-chirho
	}
}

http://127.0.0.1:${PORT_CHIRHO} {
	bind 127.0.0.1
	encode zstd gzip

	@bare_chirho path /
	@login_display_chirho {
		method GET HEAD
		path /auth-chirho/login-chirho
	}
	@tenant_chirho path /auth-chirho/* /app-chirho/perffection-workspace /app-chirho/perffection-workspace/* /app-assets-chirho/perffection-workspace /app-assets-chirho/perffection-workspace/* /healthz-chirho /healthz
	@public_chirho path /app-chirho/* /app-assets-chirho/*

	route {
		redir @bare_chirho /app-chirho/perffection/login-chirho 302
		# Display only: never redirect a credential POST or intercept an auth refusal response.
		# A Location with no fragment preserves the browser's fixed R1 no-session fragment.
		redir @login_display_chirho /app-chirho/perffection/login-chirho 302
		handle @tenant_chirho {
			reverse_proxy 127.0.0.1:${PORT_TENANT_CHIRHO}
		}
		handle @public_chirho {
			reverse_proxy 127.0.0.1:${PORT_PUBLIC_CHIRHO} {
				header_up -Cookie
				header_up -Authorization
			}
		}
		respond 404
	}
}
CADDYFILE_CHIRHO
chmod 600 "$CADDYFILE_CHIRHO"
caddy validate --config "$CADDYFILE_CHIRHO" --adapter caddyfile >/dev/null

"$CAIRN_CLI_CHIRHO" --root "$DEV_ROOT_CHIRHO" serve \
  --port "$PORT_PUBLIC_CHIRHO" --allow-platform-root-chirho \
  >"$STATE_DIR_CHIRHO/public-log-chirho.txt" 2>&1 &
PUBLIC_PID_CHIRHO=$!
"$CAIRN_CLI_CHIRHO" --root "$DEV_ROOT_CHIRHO" --tenant "$TENANT_CHIRHO" serve \
  --port "$PORT_TENANT_CHIRHO" \
  >"$STATE_DIR_CHIRHO/tenant-log-chirho.txt" 2>&1 &
TENANT_PID_CHIRHO=$!
caddy run --config "$CADDYFILE_CHIRHO" --adapter caddyfile \
  >"$STATE_DIR_CHIRHO/caddy-log-chirho.txt" 2>&1 &
CADDY_PID_CHIRHO=$!

await_ready_chirho
printf '\n  product    http://127.0.0.1:%s/\n' "$PORT_CHIRHO"
printf '  state      %s\n' "$STATE_DIR_CHIRHO"
printf '  root       %s\n' "$DEV_ROOT_CHIRHO"
if (( ISSUE_SYNTHETIC_ACCOUNT_CHIRHO == 1 )); then
  printf '  principal  %s\n' "$SYNTHETIC_PRINCIPAL_FILE_CHIRHO"
  printf '  credential %s  (mode 600; local synthetic value is never printed)\n' "$SYNTHETIC_CREDENTIAL_FILE_CHIRHO"
else
  printf '%s\n' '  account    registration is closed; from another terminal create a mode-600 synthetic credential, then run:'
  printf '             %q --root %q --tenant %q identity-register --kind user --credential-file /absolute/mode-600-synthetic-credential-chirho.txt\n' \
    "$CAIRN_CLI_CHIRHO" "$DEV_ROOT_CHIRHO" "$TENANT_CHIRHO"
fi
printf '%s\n\n' '  Ctrl-C stops all three loopback processes and removes default scratch state'

wait "$CADDY_PID_CHIRHO"
