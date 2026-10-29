#!/usr/bin/env bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
#
# Fast product-only preparation. HTML remains authored as Askama templates; this command compiles
# selected SCSS, checks the selected Cairn app tree, builds its WASM guests into one workspace
# target, and emits its source manifest. Cairn is reused by default and rebuilt only when the
# caller explicitly asks for it.

set -euo pipefail

SCRIPT_ROOT_CHIRHO="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APP_ROOT_CHIRHO="${PERFFECTION_APP_ROOT_CHIRHO:-$(cd "$SCRIPT_ROOT_CHIRHO/.." && pwd)}"
CAIRN_REPO_CHIRHO="${CAIRN_REPO_CHIRHO:-$APP_ROOT_CHIRHO/../cairn-opensource-chirho}"
TREE_BUILD_CHIRHO="${TREE_BUILD_CHIRHO:-$CAIRN_REPO_CHIRHO/target/debug/tree-build-chirho}"
PRODUCT_TARGET_DIR_CHIRHO="${CARGO_TARGET_DIR:-$APP_ROOT_CHIRHO/target}"
# One workspace target is the convention every consumer assumes (the root app's emitted manifest,
# the child apps' linked targets, the node fixtures). A custom CARGO_TARGET_DIR is refused up front
# for this fast product command rather than half-supported; plain cargo keeps honouring it.
CANONICAL_TARGET_CHIRHO="$(cd "$APP_ROOT_CHIRHO" && mkdir -p target && cd target && pwd -P)"
if [[ -n "${CARGO_TARGET_DIR:-}" ]]; then
  REQUESTED_TARGET_CHIRHO="$(mkdir -p "$CARGO_TARGET_DIR" && cd "$CARGO_TARGET_DIR" && pwd -P)"
  if [[ "$REQUESTED_TARGET_CHIRHO" != "$CANONICAL_TARGET_CHIRHO" ]]; then
    printf 'refusing: CARGO_TARGET_DIR=%s is not the workspace target %s; this command supports one build target\n' \
      "$CARGO_TARGET_DIR" "$CANONICAL_TARGET_CHIRHO" >&2
    exit 1
  fi
fi
# Always the resolved absolute path: a relative but equivalent CARGO_TARGET_DIR (e.g. `target`)
# would otherwise create the child-app links relative to the wrong directory.
PRODUCT_TARGET_DIR_CHIRHO="$CANONICAL_TARGET_CHIRHO"
PIN_FILE_CHIRHO="$APP_ROOT_CHIRHO/CAIRN-PIN-chirho.txt"
WORKSPACE_APP_ROOT_CHIRHO="$APP_ROOT_CHIRHO/apps-chirho/perffection-workspace-chirho"
PAGE_TEMPLATE_APP_ROOT_CHIRHO="$APP_ROOT_CHIRHO/apps-chirho/perffection-page-template-chirho"
SHARED_STYLE_ROOT_CHIRHO="crates-chirho/perffection-ui-chirho/styles_chirho"
APP_SELECTION_CHIRHO="all"
BUILD_CAIRN_CHIRHO=0
STYLES_ONLY_CHIRHO=0
PINNED_CAIRN_CHIRHO=""
ACTUAL_CAIRN_CHIRHO=""

usage_chirho() {
  cat <<'USAGE_CHIRHO'
usage: prepare-apps-chirho.sh [--app-chirho site|workspace|page-template|all] [--build-cairn-chirho]

  --app-chirho             Select one product app, or all three (default: all).
  --build-cairn-chirho     Explicitly rebuild Cairn's tree-build and CLI binaries first.
  -h, --help-chirho        Show this help.
USAGE_CHIRHO
}

while (( $# > 0 )); do
  case "$1" in
    --app-chirho)
      if (( $# < 2 )); then
        printf '%s\n' 'missing value after --app-chirho' >&2
        exit 2
      fi
      APP_SELECTION_CHIRHO="$2"
      shift 2
      ;;
    --build-cairn-chirho)
      BUILD_CAIRN_CHIRHO=1
      shift
      ;;
    --styles-only-chirho)
      # Internal compatibility mode used by build-site-chirho.sh and the historical release rail.
      STYLES_ONLY_CHIRHO=1
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

case "$APP_SELECTION_CHIRHO" in
  site|workspace|page-template|all) ;;
  *)
    printf 'invalid --app-chirho value: %s\n' "$APP_SELECTION_CHIRHO" >&2
    usage_chirho >&2
    exit 2
    ;;
esac

if (( STYLES_ONLY_CHIRHO == 1 )) && [[ "$APP_SELECTION_CHIRHO" != "site" ]]; then
  printf '%s\n' '--styles-only-chirho is reserved for the site compatibility helper' >&2
  exit 2
fi

# shellcheck source=lib-chirho/emit-manifest-chirho.sh
source "$SCRIPT_ROOT_CHIRHO/lib-chirho/emit-manifest-chirho.sh"
require_file_chirho() {
  local path_chirho="$1"
  if [[ ! -f "$path_chirho" ]]; then
    printf 'missing required file: %s\n' "$path_chirho" >&2
    return 1
  fi
}

resolve_cairn_source_chirho() {
  require_file_chirho "$PIN_FILE_CHIRHO"
  if ! git -C "$CAIRN_REPO_CHIRHO" rev-parse --git-dir >/dev/null 2>&1; then
    printf 'Cairn source is not a Git checkout: %s\n' "$CAIRN_REPO_CHIRHO" >&2
    return 1
  fi

  local pin_line_chirho
  pin_line_chirho="$(sed '/^#/d; /^[[:space:]]*$/d' "$PIN_FILE_CHIRHO" | tr -d '[:space:]')"
  if [[ "$pin_line_chirho" =~ ^[0-9a-f]{40}$ ]]; then
    PINNED_CAIRN_CHIRHO="$pin_line_chirho"
  elif [[ "$pin_line_chirho" == pin-chirho/* ]]; then
    if ! PINNED_CAIRN_CHIRHO="$(git -C "$CAIRN_REPO_CHIRHO" rev-parse --verify --quiet "refs/tags/${pin_line_chirho}^{commit}")"; then
      printf 'Cairn pin tag is unavailable: %s\n' "$pin_line_chirho" >&2
      return 1
    fi
  else
    printf '%s must contain one full commit or pin-chirho tag\n' "$PIN_FILE_CHIRHO" >&2
    return 1
  fi
  ACTUAL_CAIRN_CHIRHO="$(git -C "$CAIRN_REPO_CHIRHO" rev-parse HEAD)"

  # These are the only Cairn source packages linked into the fast product WASM build. Compare the
  # working bytes, not just HEAD, and also refuse untracked dependency source.
  local dependency_paths_chirho=(
    Cargo.toml
    Cargo.lock
    crates/cairn-node-sdk-chirho
    crates/authoring-chirho/cairn-node-templating-chirho
  )
  if ! git -C "$CAIRN_REPO_CHIRHO" diff --quiet "$PINNED_CAIRN_CHIRHO" -- "${dependency_paths_chirho[@]}"; then
    printf 'refusing: product Cairn dependencies differ from pin %s\n' "${PINNED_CAIRN_CHIRHO:0:8}" >&2
    git -C "$CAIRN_REPO_CHIRHO" diff --name-only "$PINNED_CAIRN_CHIRHO" -- "${dependency_paths_chirho[@]}" >&2
    return 1
  fi
  local untracked_dependencies_chirho
  untracked_dependencies_chirho="$(git -C "$CAIRN_REPO_CHIRHO" ls-files --others --exclude-standard -- "${dependency_paths_chirho[@]}")"
  if [[ -n "$untracked_dependencies_chirho" ]]; then
    printf 'refusing: untracked product Cairn dependency source exists:\n%s\n' "$untracked_dependencies_chirho" >&2
    return 1
  fi
}

build_cairn_tools_chirho() {
  local cairn_source_paths_chirho=(Cargo.toml Cargo.lock src crates)
  if ! git -C "$CAIRN_REPO_CHIRHO" diff --quiet "$PINNED_CAIRN_CHIRHO" -- "${cairn_source_paths_chirho[@]}"; then
    printf 'refusing explicit Cairn build: source differs from pin %s\n' "${PINNED_CAIRN_CHIRHO:0:8}" >&2
    return 1
  fi
  local untracked_source_chirho
  untracked_source_chirho="$(git -C "$CAIRN_REPO_CHIRHO" ls-files --others --exclude-standard -- "${cairn_source_paths_chirho[@]}")"
  if [[ -n "$untracked_source_chirho" ]]; then
    printf '%s\n' 'refusing explicit Cairn build: untracked source exists under Cargo/src/crates' >&2
    return 1
  fi

  printf '  cairn build  tree-build-chirho + cairn-chirho at %s\n' "${PINNED_CAIRN_CHIRHO:0:8}"
  (
    unset CARGO_TARGET_DIR
    cargo build --locked --manifest-path "$CAIRN_REPO_CHIRHO/Cargo.toml" \
      -p cairn-tree-build-chirho --bin tree-build-chirho \
      -p cairn-cli-chirho --bin cairn-chirho
  )
}

compile_site_styles_chirho() {
  local page_chirho
  # The public pages are on FIRST LIGHT: the front door since 2026-09-30, "What's in it" and Trust
  # since 2026-10-01. The shared tokens and chrome come first because the nav markup is shared with
  # the workspace. Motion comes after every component it animates.
  local first_light_styles_chirho=(
    --style "$SHARED_STYLE_ROOT_CHIRHO/tokens-chirho.scss"
    --style "$SHARED_STYLE_ROOT_CHIRHO/chrome-chirho.scss"
    --style styles_chirho/first-light-chirho/tokens-chirho.scss
    --style styles_chirho/first-light-chirho/type-chirho.scss
    --style styles_chirho/first-light-chirho/sky-chirho.scss
    --style styles_chirho/first-light-chirho/components-chirho.scss
    --style styles_chirho/first-light-chirho/chrome-chirho.scss
  )
  # The nine design studies keep their own stylesheet, framed by bench-frame for the new system.
  printf '%s\n' '  style       site/home'
  "$TREE_BUILD_CHIRHO" style-compile "$APP_ROOT_CHIRHO" \
    "${first_light_styles_chirho[@]}" \
    --style styles_chirho/system-chirho/bench-chirho.scss \
    --style styles_chirho/first-light-chirho/bench-frame-chirho.scss \
    --style styles_chirho/first-light-chirho/motion-chirho.scss \
    --style styles_chirho/pages-chirho/home-chirho.scss \
    --style styles_chirho/system-chirho/print-chirho.scss \
    --output assets_chirho/home-chirho.css
  require_file_chirho "$APP_ROOT_CHIRHO/assets_chirho/home-chirho.css"

  for page_chirho in craft trust; do
    printf '  style       site/%s\n' "$page_chirho"
    "$TREE_BUILD_CHIRHO" style-compile "$APP_ROOT_CHIRHO" \
      "${first_light_styles_chirho[@]}" \
      --style styles_chirho/first-light-chirho/inner-pages-chirho.scss \
      --style styles_chirho/first-light-chirho/motion-chirho.scss \
      --style "styles_chirho/pages-chirho/${page_chirho}-chirho.scss" \
      --style styles_chirho/system-chirho/print-chirho.scss \
      --output "assets_chirho/${page_chirho}-chirho.css"
    require_file_chirho "$APP_ROOT_CHIRHO/assets_chirho/${page_chirho}-chirho.css"
  done

  # Sign-in (and the branded auth view), Help and the retired agent guide, on FIRST LIGHT since
  # 2026-10-01. The agent guide extends Help's stylesheet (Cairn's bounded style ABI).
  local page_styles_chirho
  for page_chirho in login help agent-guide; do
    page_styles_chirho=(--style "styles_chirho/pages-chirho/${page_chirho}-chirho.scss")
    if [[ "$page_chirho" == agent-guide ]]; then
      page_styles_chirho=(--style styles_chirho/pages-chirho/help-chirho.scss "${page_styles_chirho[@]}")
    fi
    printf '  style       site/%s\n' "$page_chirho"
    "$TREE_BUILD_CHIRHO" style-compile "$APP_ROOT_CHIRHO" \
      "${first_light_styles_chirho[@]}" \
      "${page_styles_chirho[@]}" \
      --style styles_chirho/system-chirho/print-chirho.scss \
      --output "assets_chirho/${page_chirho}-chirho.css"
    require_file_chirho "$APP_ROOT_CHIRHO/assets_chirho/${page_chirho}-chirho.css"
  done

  printf '%s\n' '  check       site'
  "$TREE_BUILD_CHIRHO" check --json "$APP_ROOT_CHIRHO" >/dev/null
}

compile_workspace_styles_chirho() {
  printf '%s\n' '  style       workspace'
  "$TREE_BUILD_CHIRHO" style-compile "$APP_ROOT_CHIRHO" \
    --style "$SHARED_STYLE_ROOT_CHIRHO/tokens-chirho.scss" \
    --style "$SHARED_STYLE_ROOT_CHIRHO/chrome-chirho.scss" \
    --style apps-chirho/perffection-workspace-chirho/styles_chirho/workspace-chirho.scss \
    --output apps-chirho/perffection-workspace-chirho/assets_chirho/workspace-chirho.css
  require_file_chirho "$WORKSPACE_APP_ROOT_CHIRHO/assets_chirho/workspace-chirho.css"

  printf '%s\n' '  check       workspace'
  "$TREE_BUILD_CHIRHO" check --json "$WORKSPACE_APP_ROOT_CHIRHO" >/dev/null
}

compile_page_template_styles_chirho() {
  printf '%s\n' '  style       page-template'
  "$TREE_BUILD_CHIRHO" style-compile "$APP_ROOT_CHIRHO" \
    --style apps-chirho/perffection-page-template-chirho/styles_chirho/page-chirho.scss \
    --output apps-chirho/perffection-page-template-chirho/assets_chirho/page-chirho.css
  require_file_chirho "$PAGE_TEMPLATE_APP_ROOT_CHIRHO/assets_chirho/page-chirho.css"

  printf '%s\n' '  check       page-template'
  "$TREE_BUILD_CHIRHO" check --json "$PAGE_TEMPLATE_APP_ROOT_CHIRHO" >/dev/null
}

# The engine's app-source-deploy opens a cargo-example artifact relative to the APP directory
# (`<app>/target/wasm32-unknown-unknown/...`), while one Cargo workspace builds into the shared
# target. One symlink keeps a single build honest for deploys and node fixtures alike; a real
# directory there is stale pre-consolidation output and is refused rather than silently used.
link_app_wasm_target_chirho() {
  local app_root_chirho="$1"
  local link_chirho="$app_root_chirho/target/wasm32-unknown-unknown"
  local built_chirho="$PRODUCT_TARGET_DIR_CHIRHO/wasm32-unknown-unknown"
  if [[ -L "$link_chirho" ]]; then
    # A link that points anywhere else (an earlier CARGO_TARGET_DIR, a broken target) would let a
    # deploy read bytes this run did not build; re-point it and say so.
    if [[ "$(readlink "$link_chirho")" != "$built_chirho" ]]; then
      ln -sfn "$built_chirho" "$link_chirho"
      printf '  relink      %s -> shared target (was elsewhere)\n' "${link_chirho#"$APP_ROOT_CHIRHO"/}"
    fi
    return 0
  fi
  if [[ -d "$link_chirho" ]]; then
    # Pre-consolidation app-local output: preserved beside the link, never deleted.
    local stale_chirho
    stale_chirho="$app_root_chirho/target/stale-wasm32-unknown-unknown-$(date '+%Y%m%d%H%M%S')-chirho"
    mv "$link_chirho" "$stale_chirho"
    printf '  moved       stale app-local wasm to %s\n' "${stale_chirho#"$APP_ROOT_CHIRHO"/}"
  fi
  mkdir -p "$app_root_chirho/target"
  ln -s "$built_chirho" "$link_chirho"
  printf '  link        %s -> shared target\n' "${link_chirho#"$APP_ROOT_CHIRHO"/}"
}

build_wasm_chirho() {
  local package_chirho="$1"
  shift
  local build_args_chirho=("$@")
  local profile_chirho=debug
  local argument_chirho
  for argument_chirho in "${build_args_chirho[@]}"; do
    if [[ "$argument_chirho" == "--release" ]]; then
      profile_chirho=release
    fi
  done
  printf '  wasm        %s\n' "$package_chirho"
  CARGO_TARGET_DIR="$PRODUCT_TARGET_DIR_CHIRHO" cargo build --locked \
    --manifest-path "$APP_ROOT_CHIRHO/Cargo.toml" \
    -p "$package_chirho" --target wasm32-unknown-unknown "${build_args_chirho[@]}"

  local index_chirho
  local example_chirho
  for (( index_chirho = 0; index_chirho < ${#build_args_chirho[@]}; index_chirho++ )); do
    if [[ "${build_args_chirho[$index_chirho]}" == "--example" ]]; then
      index_chirho=$((index_chirho + 1))
      example_chirho="${build_args_chirho[$index_chirho]}"
      require_file_chirho "$PRODUCT_TARGET_DIR_CHIRHO/wasm32-unknown-unknown/$profile_chirho/examples/${example_chirho}.wasm"
    fi
  done
}

emit_manifest_chirho() {
  local app_root_chirho="$1"
  local label_chirho="$2"
  local output_chirho="$app_root_chirho/.emitted-manifest-chirho.json"
  printf '  emit        %s\n' "$label_chirho"
  # tree-build's emit writes text/html for every route and names debug guest paths; the shared pipeline
  # passes each route's DECLARED type through and points the manifest at the release guests.
  emit_app_manifest_chirho "$TREE_BUILD_CHIRHO" "$APP_ROOT_CHIRHO" "$app_root_chirho" "$output_chirho" || return
  require_file_chirho "$output_chirho"
}

prepare_site_chirho() {
  compile_site_styles_chirho
  build_wasm_chirho perffection-site-chirho --release \
    --example render_home_guest_chirho \
    --example render_craft_guest_chirho \
    --example render_trust_guest_chirho \
    --example render_login_guest_chirho \
    --example render_setup_guest_chirho \
    --example render_help_guest_chirho \
    --example render_agent_guide_guest_chirho
  # Auth renders are unauthenticated and have a stricter fuel cap than ordinary app renders.
  # The real debug guest exhausts 1M fuel; the optimized guest is measured below that same cap.
  # Keep one explicit output path, so a caller cannot accidentally hand the engine stale debug bytes.
  # Every shipped guest is release now; this one keeps its own line because its fuel cap is the reason.
  build_wasm_chirho perffection-site-chirho --release --example render_auth_guest_chirho
  emit_manifest_chirho "$APP_ROOT_CHIRHO" site
}

prepare_workspace_chirho() {
  compile_workspace_styles_chirho
  build_wasm_chirho perffection-workspace-chirho --release \
    --example build_page_read_guest_chirho \
    --example build_page_write_guest_chirho \
    --example build_page_restore_guest_chirho \
    --example render_workspace_state_guest_chirho \
    --example render_private_preview_guest_chirho \
    --example render_page_rejection_guest_chirho \
    --example render_page_document_export_guest_chirho \
    --example build_publication_status_read_guest_chirho \
    --example render_publication_status_guest_chirho \
    --example build_publication_intent_guest_chirho \
    --example render_publication_form_rejection_guest_chirho \
    --example render_publication_intent_guest_chirho \
    --example render_page_document_import_guest_chirho \
    --example review_page_document_guest_chirho \
    --example render_account_guest_chirho \
    --example build_own_keys_read_guest_chirho \
    --example render_account_keys_guest_chirho \
    --example render_account_connect_guest_chirho \
    --example build_draft_request_guest_chirho \
    --example render_draft_brief_guest_chirho \
    --example render_draft_brief_rejection_guest_chirho \
    --example render_draft_review_guest_chirho \
    --example render_agent_page_guest_chirho \
    --example build_agent_page_write_guest_chirho \
    --example render_agent_save_outcome_guest_chirho \
    --example render_agent_save_refusal_guest_chirho \
    --example build_figures_read_guest_chirho \
    --example render_figures_page_guest_chirho \
    --example review_figures_guest_chirho \
    --example build_figures_write_guest_chirho \
    --example render_figures_saved_guest_chirho \
    --example render_figures_rejection_guest_chirho \
    --example render_agent_figures_guest_chirho \
    --example build_agent_figures_write_guest_chirho \
    --example render_agent_figures_refusal_guest_chirho \
    --example render_agent_figures_saved_guest_chirho
  link_app_wasm_target_chirho "$WORKSPACE_APP_ROOT_CHIRHO"
  emit_manifest_chirho "$WORKSPACE_APP_ROOT_CHIRHO" workspace
  printf '%s\n' '  test        workspace node fixtures'
  # A positive control: the runner reports "clean" over an emptied fixture directory too (measured
  # 2026-09-19 after the temp cleaner took the files), so a zero count refuses the prepare.
  local fixture_report_chirho fixture_count_chirho
  fixture_report_chirho="$("$TREE_BUILD_CHIRHO" test --json "$WORKSPACE_APP_ROOT_CHIRHO")" || return 1
  fixture_count_chirho="$(printf '%s' "$fixture_report_chirho" | python3 -c 'import json,sys; r=json.load(sys.stdin); print(r["fixture_count_chirho"] if r.get("clean_chirho") else -1)')" || return 1
  if [[ "$fixture_count_chirho" -lt 1 ]]; then
    printf 'refusing: workspace node fixtures did not run clean (count %s)\n' "$fixture_count_chirho" >&2
    return 1
  fi
  printf '  fixtures    %s clean\n' "$fixture_count_chirho"
}

prepare_page_template_chirho() {
  compile_page_template_styles_chirho
  build_wasm_chirho perffection-page-template-chirho --release \
    --example render_published_page_guest_chirho
  link_app_wasm_target_chirho "$PAGE_TEMPLATE_APP_ROOT_CHIRHO"
  emit_manifest_chirho "$PAGE_TEMPLATE_APP_ROOT_CHIRHO" page-template
}

if [[ ! -x "$TREE_BUILD_CHIRHO" && "$BUILD_CAIRN_CHIRHO" != "1" ]]; then
  printf 'missing tree-build-chirho at %s\n' "$TREE_BUILD_CHIRHO" >&2
  printf '%s\n' 'rerun with --build-cairn-chirho or set TREE_BUILD_CHIRHO' >&2
  exit 1
fi

if (( STYLES_ONLY_CHIRHO == 1 )); then
  compile_site_styles_chirho
  exit 0
fi

resolve_cairn_source_chirho
if (( BUILD_CAIRN_CHIRHO == 1 )); then
  build_cairn_tools_chirho
fi
if [[ ! -x "$TREE_BUILD_CHIRHO" ]]; then
  printf 'Cairn build did not produce tree-build-chirho at %s\n' "$TREE_BUILD_CHIRHO" >&2
  exit 1
fi

case "$APP_SELECTION_CHIRHO" in
  site)
    prepare_site_chirho
    ;;
  workspace)
    prepare_workspace_chirho
    ;;
  page-template)
    prepare_page_template_chirho
    ;;
  all)
    prepare_site_chirho
    prepare_workspace_chirho
    prepare_page_template_chirho
    ;;
esac

printf '  cargo target %s\n' "$PRODUCT_TARGET_DIR_CHIRHO"
printf '  cairn source head=%s pin=%s\n' "${ACTUAL_CAIRN_CHIRHO:0:12}" "${PINNED_CAIRN_CHIRHO:0:12}"
printf '  tree-build sha256 %s\n' "$(shasum -a 256 "$TREE_BUILD_CHIRHO" | awk '{print $1}')"
