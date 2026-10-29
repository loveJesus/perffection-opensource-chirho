#!/usr/bin/env bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
# Workflow: spec-chirho/workflows-chirho/product-delivery-chirho/product-authoring-chirho.md.
# Sourced by local serve helpers. Preparation builds once; deployment consumes those exact outputs.

stage_prebuilt_app_chirho() {
  local engine_chirho="$1"
  local app_root_chirho="$2"
  local app_name_chirho="$3"
  local stage_chirho="$4"
  local stager_chirho="$engine_chirho/tools-chirho/app-deploy-chirho/host_ready_app_manifest_chirho.py"
  local manifest_chirho="$stage_chirho/${app_name_chirho}-hosted-source-manifest-chirho.json"

  if [[ ! -f "$stager_chirho" ]]; then
    printf 'missing prebuilt app stager: %s\n' "$stager_chirho" >&2
    return 1
  fi
  if ! python3 "$stager_chirho" "$app_root_chirho" "$stage_chirho" >&2; then
    return 1
  fi
  if [[ ! -f "$manifest_chirho" ]]; then
    printf 'staged manifest does not name the expected app: %s\n' "$app_name_chirho" >&2
    return 1
  fi
  # Verify every generated manifest/module/asset before any engine mutation. This is an automatic
  # local copy, not an approval checkpoint or a new source tree. Missing outputs never trigger cargo.
  if ! (cd "$stage_chirho" && shasum -a 256 -c sha256-chirho.txt >/dev/null); then
    printf 'staged app checksum verification failed: %s\n' "$app_name_chirho" >&2
    return 1
  fi
  printf '%s\n' "$manifest_chirho"
}
