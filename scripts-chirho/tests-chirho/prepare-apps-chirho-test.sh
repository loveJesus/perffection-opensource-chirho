#!/usr/bin/env bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

set -euo pipefail

SCRIPT_ROOT_CHIRHO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PREPARE_CHIRHO="$SCRIPT_ROOT_CHIRHO/prepare-apps-chirho.sh"
TEMP_PARENT_CHIRHO="${TMPDIR:-/tmp}"
TEMP_PARENT_CHIRHO="${TEMP_PARENT_CHIRHO%/}"
TEST_ROOT_CHIRHO="$(mktemp -d "$TEMP_PARENT_CHIRHO/perffection-prepare-test-chirho.XXXXXX")"
PRODUCT_ROOT_CHIRHO="$TEST_ROOT_CHIRHO/product-chirho"
CAIRN_ROOT_CHIRHO="$TEST_ROOT_CHIRHO/cairn-chirho"
FAKE_BIN_CHIRHO="$TEST_ROOT_CHIRHO/bin-chirho"
COMMAND_LOG_CHIRHO="$TEST_ROOT_CHIRHO/commands-chirho.log"
# The fast command supports exactly one build target: the product workspace target.
TARGET_ROOT_CHIRHO="$PRODUCT_ROOT_CHIRHO/target"

cleanup_chirho() {
  if [[ "$TEST_ROOT_CHIRHO" == "$TEMP_PARENT_CHIRHO"/perffection-prepare-test-chirho.* ]]; then
    rm -rf -- "$TEST_ROOT_CHIRHO"
  fi
}
trap cleanup_chirho EXIT

fail_chirho() {
  printf 'FAIL: %s\n' "$1" >&2
  exit 1
}

assert_contains_chirho() {
  local needle_chirho="$1"
  local path_chirho="$2"
  grep -F -- "$needle_chirho" "$path_chirho" >/dev/null \
    || fail_chirho "expected $path_chirho to contain: $needle_chirho"
}

assert_not_contains_chirho() {
  local needle_chirho="$1"
  local path_chirho="$2"
  if grep -F -- "$needle_chirho" "$path_chirho" >/dev/null; then
    fail_chirho "expected $path_chirho not to contain: $needle_chirho"
  fi
}

assert_output_chirho() {
  local path_chirho="$1"
  [[ -s "$path_chirho" ]] || fail_chirho "expected nonempty output: $path_chirho"
}

mkdir -p \
  "$FAKE_BIN_CHIRHO" \
  "$PRODUCT_ROOT_CHIRHO/scripts-chirho" \
  "$PRODUCT_ROOT_CHIRHO/crates-chirho/perffection-ui-chirho/styles_chirho" \
  "$PRODUCT_ROOT_CHIRHO/styles_chirho/system-chirho" \
  "$PRODUCT_ROOT_CHIRHO/styles_chirho/first-light-chirho" \
  "$PRODUCT_ROOT_CHIRHO/styles_chirho/pages-chirho" \
  "$PRODUCT_ROOT_CHIRHO/apps-chirho/perffection-workspace-chirho/styles_chirho" \
  "$PRODUCT_ROOT_CHIRHO/apps-chirho/perffection-page-template-chirho/styles_chirho" \
  "$CAIRN_ROOT_CHIRHO/crates/cairn-node-sdk-chirho" \
  "$CAIRN_ROOT_CHIRHO/crates/authoring-chirho/cairn-node-templating-chirho"

# Every style input the prepare names, at the paths it names them (FIRST LIGHT since 2026-10-01).
# The fake style-compile below refuses a missing input, as the real one does, so this list has to
# stay true: it once held retired VELLUM files and a shared-style path with a hyphen, unnoticed.
for style_chirho in \
  crates-chirho/perffection-ui-chirho/styles_chirho/tokens-chirho.scss \
  crates-chirho/perffection-ui-chirho/styles_chirho/chrome-chirho.scss \
  styles_chirho/system-chirho/bench-chirho.scss \
  styles_chirho/system-chirho/print-chirho.scss \
  styles_chirho/first-light-chirho/tokens-chirho.scss \
  styles_chirho/first-light-chirho/type-chirho.scss \
  styles_chirho/first-light-chirho/sky-chirho.scss \
  styles_chirho/first-light-chirho/components-chirho.scss \
  styles_chirho/first-light-chirho/chrome-chirho.scss \
  styles_chirho/first-light-chirho/bench-frame-chirho.scss \
  styles_chirho/first-light-chirho/motion-chirho.scss \
  styles_chirho/first-light-chirho/inner-pages-chirho.scss \
  styles_chirho/pages-chirho/home-chirho.scss \
  styles_chirho/pages-chirho/craft-chirho.scss \
  styles_chirho/pages-chirho/trust-chirho.scss \
  styles_chirho/pages-chirho/login-chirho.scss \
  styles_chirho/pages-chirho/help-chirho.scss \
  styles_chirho/pages-chirho/agent-guide-chirho.scss \
  apps-chirho/perffection-workspace-chirho/styles_chirho/workspace-chirho.scss \
  apps-chirho/perffection-page-template-chirho/styles_chirho/page-chirho.scss; do
  printf '.fixture-chirho { display: block; }\n' >"$PRODUCT_ROOT_CHIRHO/$style_chirho"
done

# prepare runs the real post-emit passes from the product root; they are code under test, not stubs.
# The fixture descriptors declare no route types, so the route pass leaves the manifest unchanged.
mkdir -p "$PRODUCT_ROOT_CHIRHO/scripts-chirho/lib-chirho"
cp "$SCRIPT_ROOT_CHIRHO/lib-chirho/route-response-types-chirho.py" \
  "$SCRIPT_ROOT_CHIRHO/lib-chirho/release-guest-paths-chirho.py" "$PRODUCT_ROOT_CHIRHO/scripts-chirho/lib-chirho/"
for descriptor_chirho in app-chirho.toml apps-chirho/perffection-workspace-chirho/app-chirho.toml \
  apps-chirho/perffection-page-template-chirho/app-chirho.toml; do
  printf '# fixture descriptor Chirho\n' >"$PRODUCT_ROOT_CHIRHO/$descriptor_chirho"
done

printf '[workspace]\n' >"$PRODUCT_ROOT_CHIRHO/Cargo.toml"
printf '# fixture pin\n' >"$PRODUCT_ROOT_CHIRHO/CAIRN-PIN-chirho.txt"
printf '[package]\nname = "cairn-node-sdk-chirho"\nversion = "0.1.0"\n' \
  >"$CAIRN_ROOT_CHIRHO/crates/cairn-node-sdk-chirho/Cargo.toml"
printf '[package]\nname = "cairn-node-templating-chirho"\nversion = "0.1.0"\n' \
  >"$CAIRN_ROOT_CHIRHO/crates/authoring-chirho/cairn-node-templating-chirho/Cargo.toml"
git -C "$CAIRN_ROOT_CHIRHO" init -q -b main_chirho
git -C "$CAIRN_ROOT_CHIRHO" config user.name test-chirho
git -C "$CAIRN_ROOT_CHIRHO" config user.email test-chirho@example.invalid
git -C "$CAIRN_ROOT_CHIRHO" add \
  crates/cairn-node-sdk-chirho/Cargo.toml \
  crates/authoring-chirho/cairn-node-templating-chirho/Cargo.toml
git -C "$CAIRN_ROOT_CHIRHO" commit -q -m 'fixture-chirho'
git -C "$CAIRN_ROOT_CHIRHO" rev-parse HEAD >>"$PRODUCT_ROOT_CHIRHO/CAIRN-PIN-chirho.txt"

cat >"$FAKE_BIN_CHIRHO/tree-build-chirho" <<'TREE_BUILD_CHIRHO'
#!/usr/bin/env bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
set -euo pipefail
printf 'tree %s\n' "$*" >>"$COMMAND_LOG_CHIRHO"
command_chirho="$1"
shift
case "$command_chirho" in
  style-compile)
    root_chirho="$1"
    shift
    output_chirho=""
    while (( $# > 0 )); do
      if [[ "$1" == "--style" ]]; then
        # As the real compiler does, refuse an input that is not there.
        [[ -f "$root_chirho/$2" ]] || { printf 'missing style input %s\n' "$2" >&2; exit 1; }
        shift 2
        continue
      fi
      if [[ "$1" == "--output" ]]; then
        output_chirho="$2"
        break
      fi
      shift
    done
    [[ -n "$output_chirho" ]]
    mkdir -p "$(dirname "$root_chirho/$output_chirho")"
    printf '/* compiled fixture Chirho */\n' >"$root_chirho/$output_chirho"
    ;;
  check)
    ;;
  test)
    # The prepare treats a zero fixture count as a refusal (a positive control); the fake reports
    # one fixture unless an arm forces the empty case.
    printf '{"schema_chirho":"tree-build-node-test-report-v1-chirho","clean_chirho":true,"fixture_count_chirho":%s,"failures_chirho":[]}\n' "${FAKE_TREE_FIXTURE_COUNT_CHIRHO:-1}"
    ;;
  emit)
    if [[ "${FAKE_TREE_EMIT_EXIT_CHIRHO:-0}" != "0" ]]; then
      exit "$FAKE_TREE_EMIT_EXIT_CHIRHO"
    fi
    if [[ -n "${FAKE_TREE_EMIT_GUEST_CHIRHO:-}" ]]; then
      # One cargo-example guest at tree-build's debug path, as the real emit writes it.
      printf '{"schema_chirho":"fixture-manifest-chirho","nodes_chirho":[{"source_chirho":{"kind_chirho":"cargo-example-chirho","artifact_path_chirho":"target/wasm32-unknown-unknown/debug/examples/%s.wasm"}}]}\n' "$FAKE_TREE_EMIT_GUEST_CHIRHO"
    else
      printf '{"schema_chirho":"fixture-manifest-chirho"}\n'
    fi
    ;;
  *)
    exit 91
    ;;
esac
TREE_BUILD_CHIRHO

cat >"$FAKE_BIN_CHIRHO/cargo" <<'CARGO_CHIRHO'
#!/usr/bin/env bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
set -euo pipefail
printf 'cargo %s\n' "$*" >>"$COMMAND_LOG_CHIRHO"
if [[ "${FAKE_CARGO_EXIT_CHIRHO:-0}" != "0" ]]; then
  exit "$FAKE_CARGO_EXIT_CHIRHO"
fi
args_chirho=("$@")
profile_chirho=debug
for argument_chirho in "${args_chirho[@]}"; do
  if [[ "$argument_chirho" == "--release" ]]; then
    profile_chirho=release
  fi
done
for (( index_chirho = 0; index_chirho < ${#args_chirho[@]}; index_chirho++ )); do
  if [[ "${args_chirho[$index_chirho]}" == "--example" ]]; then
    index_chirho=$((index_chirho + 1))
    example_chirho="${args_chirho[$index_chirho]}"
    artifact_chirho="$CARGO_TARGET_DIR/wasm32-unknown-unknown/$profile_chirho/examples/${example_chirho}.wasm"
    mkdir -p "$(dirname "$artifact_chirho")"
    printf 'fixture wasm Chirho\n' >"$artifact_chirho"
  fi
done
CARGO_CHIRHO
chmod +x "$FAKE_BIN_CHIRHO/tree-build-chirho" "$FAKE_BIN_CHIRHO/cargo"

run_prepare_chirho() {
  PATH="$FAKE_BIN_CHIRHO:$PATH" \
  COMMAND_LOG_CHIRHO="$COMMAND_LOG_CHIRHO" \
  PERFFECTION_APP_ROOT_CHIRHO="$PRODUCT_ROOT_CHIRHO" \
  CAIRN_REPO_CHIRHO="$CAIRN_ROOT_CHIRHO" \
  TREE_BUILD_CHIRHO="$FAKE_BIN_CHIRHO/tree-build-chirho" \
  CAIRN_CLI_CHIRHO="$FAKE_BIN_CHIRHO/missing-cairn-chirho" \
  CARGO_TARGET_DIR="$TARGET_ROOT_CHIRHO" \
  "$PREPARE_CHIRHO" "$@"
}

: >"$COMMAND_LOG_CHIRHO"
run_prepare_chirho --app-chirho workspace >/dev/null
assert_contains_chirho '-p perffection-workspace-chirho' "$COMMAND_LOG_CHIRHO"
assert_not_contains_chirho '-p perffection-site-chirho' "$COMMAND_LOG_CHIRHO"
assert_not_contains_chirho '-p perffection-page-template-chirho' "$COMMAND_LOG_CHIRHO"
assert_not_contains_chirho "$CAIRN_ROOT_CHIRHO/Cargo.toml" "$COMMAND_LOG_CHIRHO"
assert_output_chirho "$PRODUCT_ROOT_CHIRHO/apps-chirho/perffection-workspace-chirho/assets_chirho/workspace-chirho.css"
assert_output_chirho "$PRODUCT_ROOT_CHIRHO/apps-chirho/perffection-workspace-chirho/.emitted-manifest-chirho.json"
assert_output_chirho "$TARGET_ROOT_CHIRHO/wasm32-unknown-unknown/release/examples/build_page_write_guest_chirho.wasm"

: >"$COMMAND_LOG_CHIRHO"
run_prepare_chirho --app-chirho page-template >/dev/null
assert_contains_chirho '-p perffection-page-template-chirho' "$COMMAND_LOG_CHIRHO"
assert_not_contains_chirho '-p perffection-workspace-chirho' "$COMMAND_LOG_CHIRHO"
assert_output_chirho "$PRODUCT_ROOT_CHIRHO/apps-chirho/perffection-page-template-chirho/assets_chirho/page-chirho.css"
assert_output_chirho "$PRODUCT_ROOT_CHIRHO/apps-chirho/perffection-page-template-chirho/.emitted-manifest-chirho.json"
assert_output_chirho "$TARGET_ROOT_CHIRHO/wasm32-unknown-unknown/release/examples/render_published_page_guest_chirho.wasm"

: >"$COMMAND_LOG_CHIRHO"
run_prepare_chirho --app-chirho site --build-cairn-chirho >/dev/null
assert_contains_chirho "--manifest-path $CAIRN_ROOT_CHIRHO/Cargo.toml" "$COMMAND_LOG_CHIRHO"
assert_contains_chirho '-p cairn-tree-build-chirho' "$COMMAND_LOG_CHIRHO"
assert_contains_chirho '-p perffection-site-chirho' "$COMMAND_LOG_CHIRHO"
assert_output_chirho "$TARGET_ROOT_CHIRHO/wasm32-unknown-unknown/release/examples/render_auth_guest_chirho.wasm"
assert_output_chirho "$TARGET_ROOT_CHIRHO/wasm32-unknown-unknown/release/examples/render_agent_guide_guest_chirho.wasm"
assert_output_chirho "$PRODUCT_ROOT_CHIRHO/assets_chirho/agent-guide-chirho.css"

: >"$COMMAND_LOG_CHIRHO"
run_prepare_chirho --app-chirho page-template --build-cairn-chirho >/dev/null
assert_contains_chirho "--manifest-path $CAIRN_ROOT_CHIRHO/Cargo.toml" "$COMMAND_LOG_CHIRHO"
assert_contains_chirho '-p cairn-tree-build-chirho' "$COMMAND_LOG_CHIRHO"
assert_contains_chirho '-p cairn-cli-chirho' "$COMMAND_LOG_CHIRHO"
assert_contains_chirho '-p perffection-page-template-chirho' "$COMMAND_LOG_CHIRHO"

set +e
run_prepare_chirho --app-chirho not-an-app-chirho >/dev/null 2>&1
selection_status_chirho=$?
set -e
[[ "$selection_status_chirho" == "2" ]] \
  || fail_chirho "expected invalid app selection status 2, got $selection_status_chirho"

set +e
FAKE_CARGO_EXIT_CHIRHO=19 run_prepare_chirho --app-chirho site >/dev/null 2>&1
cargo_status_chirho=$?
set -e
[[ "$cargo_status_chirho" == "19" ]] \
  || fail_chirho "expected cargo failure 19 to propagate, got $cargo_status_chirho"

printf '\n# dirty dependency Chirho\n' >>"$CAIRN_ROOT_CHIRHO/crates/cairn-node-sdk-chirho/Cargo.toml"
set +e
run_prepare_chirho --app-chirho site >/dev/null 2>&1
pin_status_chirho=$?
set -e
[[ "$pin_status_chirho" != "0" ]] || fail_chirho 'expected dirty Cairn dependency to refuse'
git -C "$CAIRN_ROOT_CHIRHO" checkout -q -- crates/cairn-node-sdk-chirho/Cargo.toml

rm -f "$PRODUCT_ROOT_CHIRHO/apps-chirho/perffection-page-template-chirho/.emitted-manifest-chirho.json"
set +e
FAKE_TREE_EMIT_EXIT_CHIRHO=23 run_prepare_chirho --app-chirho page-template >/dev/null 2>&1
emit_status_chirho=$?
set -e
[[ "$emit_status_chirho" == "23" ]] \
  || fail_chirho "expected emit failure 23 to propagate, got $emit_status_chirho"
[[ ! -e "$PRODUCT_ROOT_CHIRHO/apps-chirho/perffection-page-template-chirho/.emitted-manifest-chirho.json" ]] \
  || fail_chirho 'failed emit replaced the prior manifest'
if compgen -G "$PRODUCT_ROOT_CHIRHO/apps-chirho/perffection-page-template-chirho/.emitted-manifest-chirho.json.tmp-*-chirho" >/dev/null; then
  fail_chirho 'failed emit left a temporary manifest'
fi

# Emit names debug guest paths; the shipped manifest must name the release build prepare made,
# and must refuse, not ship a debug path, when that release build does not exist.
TEMPLATE_MANIFEST_CHIRHO="$PRODUCT_ROOT_CHIRHO/apps-chirho/perffection-page-template-chirho/.emitted-manifest-chirho.json"
FAKE_TREE_EMIT_GUEST_CHIRHO=render_published_page_guest_chirho run_prepare_chirho --app-chirho page-template >/dev/null
assert_contains_chirho 'target/wasm32-unknown-unknown/release/examples/render_published_page_guest_chirho.wasm' "$TEMPLATE_MANIFEST_CHIRHO"
assert_not_contains_chirho 'wasm32-unknown-unknown/debug/examples' "$TEMPLATE_MANIFEST_CHIRHO"
cp "$TEMPLATE_MANIFEST_CHIRHO" "$TEST_ROOT_CHIRHO/release-manifest-before-chirho.json"
set +e
FAKE_TREE_EMIT_GUEST_CHIRHO=never_built_guest_chirho run_prepare_chirho --app-chirho page-template >/dev/null 2>"$TEST_ROOT_CHIRHO/release-refusal-chirho.err"
release_status_chirho=$?
set -e
[[ "$release_status_chirho" == "2" ]] || fail_chirho "expected a missing release guest to refuse with 2, got $release_status_chirho"
grep -q 'release guest was not built' "$TEST_ROOT_CHIRHO/release-refusal-chirho.err" \
  || fail_chirho 'the missing-release-guest refusal did not name itself'
cmp -s "$TEMPLATE_MANIFEST_CHIRHO" "$TEST_ROOT_CHIRHO/release-manifest-before-chirho.json" \
  || fail_chirho 'a refused release rewrite replaced the prior manifest'

# A workspace prepare whose fixture runner finds nothing must refuse: green over an emptied
# fixture directory is exactly the false claim the control exists to stop.
set +e
FAKE_TREE_FIXTURE_COUNT_CHIRHO=0 run_prepare_chirho --app-chirho workspace >/dev/null 2>"$TEST_ROOT_CHIRHO/fixture-refusal-chirho.err"
fixture_status_chirho=$?
set -e
[[ "$fixture_status_chirho" != "0" ]] || fail_chirho 'expected a zero fixture count to refuse the workspace prepare'
grep -q 'workspace node fixtures did not run clean' "$TEST_ROOT_CHIRHO/fixture-refusal-chirho.err" \
  || fail_chirho 'the fixture-count refusal did not name itself'

printf '%s\n' 'prepare-apps-chirho tests passed'
