#!/usr/bin/env bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
#
# Harness for scripts-chirho/lib-chirho/prebuilt-app-chirho.sh: `stage_prebuilt_app_chirho engine
# app-root expected-app-name stage-dir` must stage ALREADY-BUILT outputs through the real
# host-ready converter (byte-exact copies, a hosted manifest naming the app, a verified SHA-256
# list) and refuse — without ever running cargo — a missing guest wasm, a wrong app identity, a
# non-empty stage directory and a missing converter. Everything here is a temporary synthetic
# fixture; no real product files or engine roots are touched. The converter's bytes are the real
# ones, copied from the cairn checkout into an owned engine directory.
#
# Env: CAIRN_REPO_CHIRHO (default ../cairn-opensource-chirho) supplies the converter to copy.

set -euo pipefail

SCRIPT_ROOT_CHIRHO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PRODUCT_ROOT_CHIRHO="$(cd "$SCRIPT_ROOT_CHIRHO/.." && pwd)"
CAIRN_REPO_CHIRHO="${CAIRN_REPO_CHIRHO:-$PRODUCT_ROOT_CHIRHO/../cairn-opensource-chirho}"
REAL_CONVERTER_CHIRHO="$CAIRN_REPO_CHIRHO/tools-chirho/app-deploy-chirho/host_ready_app_manifest_chirho.py"
TEMP_PARENT_CHIRHO="${TMPDIR:-/tmp}"
TEMP_PARENT_CHIRHO="${TEMP_PARENT_CHIRHO%/}"
TEST_ROOT_CHIRHO="$(mktemp -d "$TEMP_PARENT_CHIRHO/perffection-prebuilt-app-test-chirho.XXXXXX")"
ENGINE_CHIRHO="$TEST_ROOT_CHIRHO/engine-chirho"
APP_CHIRHO="$TEST_ROOT_CHIRHO/app-chirho"
APP_NAME_CHIRHO="tiny-chirho"
WASM_REL_CHIRHO="target/wasm32-unknown-unknown/release/examples/tiny_guest_chirho.wasm"

cleanup_chirho() {
  if [[ "$TEST_ROOT_CHIRHO" == "$TEMP_PARENT_CHIRHO"/perffection-prebuilt-app-test-chirho.* ]]; then
    rm -rf -- "$TEST_ROOT_CHIRHO"
  fi
}
trap cleanup_chirho EXIT

fail_chirho() {
  printf 'FAIL: %s\n' "$1" >&2
  exit 1
}

assert_contains_chirho() {
  grep -F -- "$1" "$2" >/dev/null || fail_chirho "expected $2 to contain: $1"
}

[[ -f "$REAL_CONVERTER_CHIRHO" ]] || fail_chirho "real converter not found at $REAL_CONVERTER_CHIRHO (set CAIRN_REPO_CHIRHO)"
# shellcheck source=../lib-chirho/prebuilt-app-chirho.sh
source "$SCRIPT_ROOT_CHIRHO/lib-chirho/prebuilt-app-chirho.sh"

# ---- an owned engine dir carrying the real converter's bytes, and a tiny synthetic app ----
mkdir -p "$ENGINE_CHIRHO/tools-chirho/app-deploy-chirho"
cp "$REAL_CONVERTER_CHIRHO" "$ENGINE_CHIRHO/tools-chirho/app-deploy-chirho/host_ready_app_manifest_chirho.py"

make_app_chirho() {
  local root_chirho="$1"
  mkdir -p "$root_chirho/$(dirname "$WASM_REL_CHIRHO")" "$root_chirho/assets_chirho"
  printf 'name = "tiny-chirho"\n' > "$root_chirho/Cargo.toml"
  printf '\x00asm\x01\x00\x00\x00tiny-guest-bytes-chirho' > "$root_chirho/$WASM_REL_CHIRHO"
  printf 'body { color: #123456; }\n' > "$root_chirho/assets_chirho/tiny-chirho.css"
  cat > "$root_chirho/.emitted-manifest-chirho.json" <<JSON_CHIRHO
{
  "app_chirho": "$APP_NAME_CHIRHO",
  "schema_chirho": "app-source-manifest-v1-chirho",
  "owner_chirho": "owner/test-only-chirho",
  "assets_chirho": [
    {"content_type_chirho": "text/css", "name_chirho": "tiny-chirho.css",
     "source_chirho": {"kind_chirho": "path-chirho", "path_chirho": "assets_chirho/tiny-chirho.css"}}
  ],
  "dags_chirho": [{"edges_chirho": [], "id_chirho": "tiny_dag_chirho", "nodes_chirho": ["tiny_node_chirho"]}],
  "modules_chirho": [],
  "deploy_chirho": {"target_chirho": {"kind_chirho": "local-deploy-chirho"}},
  "nodes_chirho": [
    {"entry_chirho": "tiny_chirho", "fuel_chirho": 1, "id_chirho": "tiny_node_chirho", "name_chirho": "tiny_chirho",
     "source_chirho": {"kind_chirho": "cargo-example-chirho", "example_chirho": "tiny_guest_chirho",
       "manifest_path_chirho": "Cargo.toml", "package_chirho": "tiny-chirho", "features_chirho": [],
       "artifact_path_chirho": "$WASM_REL_CHIRHO"}}
  ]
}
JSON_CHIRHO
}
make_app_chirho "$APP_CHIRHO"

# ---- 1. success: exact copies, a hosted manifest naming the app, a verified checksum list ----
STAGE_CHIRHO="$TEST_ROOT_CHIRHO/stage-ok-chirho"
STAGED_CHIRHO="$(stage_prebuilt_app_chirho "$ENGINE_CHIRHO" "$APP_CHIRHO" "$APP_NAME_CHIRHO" "$STAGE_CHIRHO" 2>"$TEST_ROOT_CHIRHO/ok-err-chirho.txt")" \
  || fail_chirho "successful staging returned non-zero: $(cat "$TEST_ROOT_CHIRHO/ok-err-chirho.txt")"
[[ "$STAGED_CHIRHO" == "$STAGE_CHIRHO/$APP_NAME_CHIRHO-hosted-source-manifest-chirho.json" ]] || fail_chirho "unexpected staged manifest path: $STAGED_CHIRHO"
[[ -f "$STAGED_CHIRHO" ]] || fail_chirho "hosted manifest missing"
cmp -s "$APP_CHIRHO/$WASM_REL_CHIRHO" "$STAGE_CHIRHO/tiny_guest_chirho.wasm" || fail_chirho "staged wasm is not a byte-exact copy"
cmp -s "$APP_CHIRHO/assets_chirho/tiny-chirho.css" "$STAGE_CHIRHO/assets-chirho/tiny-chirho.css" || fail_chirho "staged asset is not a byte-exact copy"
(cd "$STAGE_CHIRHO" && shasum -a 256 -c sha256-chirho.txt >/dev/null) || fail_chirho "sha256 list does not verify"
[[ "$(wc -l < "$STAGE_CHIRHO/sha256-chirho.txt" | tr -d ' ')" == "3" ]] || fail_chirho "sha256 list should cover the manifest, the wasm and the asset"
python3 - "$STAGED_CHIRHO" <<'PY_CHIRHO' || fail_chirho "hosted manifest is not rewritten to path-chirho sources"
import json, sys
with open(sys.argv[1]) as handle_chirho:
    manifest_chirho = json.load(handle_chirho)
node_chirho = manifest_chirho["nodes_chirho"][0]["source_chirho"]
asset_chirho = manifest_chirho["assets_chirho"][0]["source_chirho"]
assert node_chirho == {"kind_chirho": "path-chirho", "path_chirho": "tiny_guest_chirho.wasm"}, node_chirho
assert asset_chirho == {"kind_chirho": "path-chirho", "path_chirho": "assets-chirho/tiny-chirho.css"}, asset_chirho
assert manifest_chirho["app_chirho"] == "tiny-chirho"
assert "cargo-example-chirho" not in json.dumps(manifest_chirho)
PY_CHIRHO
# the converter's report names the rewritten cargo-example nodes; a BUILD would show as cargo output
if grep -E 'cargo (build|run|check)|Compiling|Finished' "$TEST_ROOT_CHIRHO/ok-err-chirho.txt" >/dev/null; then
  fail_chirho "the helper must never run cargo"
fi

# ---- 2. a missing guest wasm is a refusal, never a build ----
APP_NO_WASM_CHIRHO="$TEST_ROOT_CHIRHO/app-no-wasm-chirho"
make_app_chirho "$APP_NO_WASM_CHIRHO"
rm -f "$APP_NO_WASM_CHIRHO/$WASM_REL_CHIRHO"
STAGE_CHIRHO="$TEST_ROOT_CHIRHO/stage-no-wasm-chirho"
if stage_prebuilt_app_chirho "$ENGINE_CHIRHO" "$APP_NO_WASM_CHIRHO" "$APP_NAME_CHIRHO" "$STAGE_CHIRHO" >"$TEST_ROOT_CHIRHO/no-wasm-out-chirho.txt" 2>"$TEST_ROOT_CHIRHO/no-wasm-err-chirho.txt"; then
  fail_chirho "a missing wasm must refuse"
fi
assert_contains_chirho "REFUSING" "$TEST_ROOT_CHIRHO/no-wasm-err-chirho.txt"
assert_contains_chirho "not a regular file" "$TEST_ROOT_CHIRHO/no-wasm-err-chirho.txt"
[[ ! -s "$TEST_ROOT_CHIRHO/no-wasm-out-chirho.txt" ]] || fail_chirho "a refusal must print no manifest path"
[[ ! -f "$STAGE_CHIRHO/$APP_NAME_CHIRHO-hosted-source-manifest-chirho.json" ]] || fail_chirho "a refused stage must hold no hosted manifest"
[[ ! -f "$STAGE_CHIRHO/sha256-chirho.txt" ]] || fail_chirho "a refused stage must hold no checksum list"
[[ ! -f "$APP_NO_WASM_CHIRHO/$WASM_REL_CHIRHO" ]] || fail_chirho "the helper must not build the missing wasm"

# ---- 3. a wrong app identity is a refusal after the converter ran ----
STAGE_CHIRHO="$TEST_ROOT_CHIRHO/stage-wrong-name-chirho"
if stage_prebuilt_app_chirho "$ENGINE_CHIRHO" "$APP_CHIRHO" "other-chirho" "$STAGE_CHIRHO" >"$TEST_ROOT_CHIRHO/wrong-out-chirho.txt" 2>"$TEST_ROOT_CHIRHO/wrong-err-chirho.txt"; then
  fail_chirho "a wrong expected app name must refuse"
fi
assert_contains_chirho "does not name the expected app: other-chirho" "$TEST_ROOT_CHIRHO/wrong-err-chirho.txt"
[[ ! -s "$TEST_ROOT_CHIRHO/wrong-out-chirho.txt" ]] || fail_chirho "a wrong identity must print no manifest path"

# ---- 4. a non-empty stage directory is a refusal ----
STAGE_CHIRHO="$TEST_ROOT_CHIRHO/stage-dirty-chirho"
mkdir -p "$STAGE_CHIRHO"
printf 'leftover\n' > "$STAGE_CHIRHO/leftover-chirho.txt"
if stage_prebuilt_app_chirho "$ENGINE_CHIRHO" "$APP_CHIRHO" "$APP_NAME_CHIRHO" "$STAGE_CHIRHO" >"$TEST_ROOT_CHIRHO/dirty-out-chirho.txt" 2>"$TEST_ROOT_CHIRHO/dirty-err-chirho.txt"; then
  fail_chirho "a non-empty stage must refuse"
fi
assert_contains_chirho "stage dir is not empty" "$TEST_ROOT_CHIRHO/dirty-err-chirho.txt"
shopt -s dotglob nullglob
STAGED_ENTRIES_CHIRHO=("$STAGE_CHIRHO"/*)
[[ "${#STAGED_ENTRIES_CHIRHO[@]}" == "1" && "${STAGED_ENTRIES_CHIRHO[0]}" == "$STAGE_CHIRHO/leftover-chirho.txt" ]] \
  || fail_chirho "a refused non-empty stage must be left as it was"
[[ "$(<"$STAGE_CHIRHO/leftover-chirho.txt")" == "leftover" ]] || fail_chirho "refusal changed the pre-existing content"

# ---- 5. a missing converter is a refusal before anything runs ----
STAGE_CHIRHO="$TEST_ROOT_CHIRHO/stage-no-converter-chirho"
if stage_prebuilt_app_chirho "$TEST_ROOT_CHIRHO/engine-without-converter-chirho" "$APP_CHIRHO" "$APP_NAME_CHIRHO" "$STAGE_CHIRHO" >"$TEST_ROOT_CHIRHO/no-converter-out-chirho.txt" 2>"$TEST_ROOT_CHIRHO/no-converter-err-chirho.txt"; then
  fail_chirho "a missing converter must refuse"
fi
assert_contains_chirho "missing prebuilt app stager" "$TEST_ROOT_CHIRHO/no-converter-err-chirho.txt"
[[ ! -e "$STAGE_CHIRHO" ]] || fail_chirho "a missing converter must create no stage"

printf 'prebuilt-app-chirho-test: ok (exact copies, verified checksums, refusals for missing wasm / wrong identity / non-empty stage / missing converter)\n'
