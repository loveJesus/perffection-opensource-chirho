#!/usr/bin/env bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
#
# One emit pipeline for every caller (prepare-apps-chirho.sh, and the figures gate's scratch binding):
# tree-build's emit, then the product's two passes over its output until tree-build learns them (the
# route's DECLARED response type; the release guest paths), written atomically so a failed pass never
# leaves a half manifest behind.
# usage: emit_app_manifest_chirho <tree-build> <product root> <app root> <output file>
emit_app_manifest_chirho() {
  local tree_build_chirho="$1" product_root_chirho="$2" app_root_chirho="$3" output_chirho="$4"
  local temporary_output_chirho="${output_chirho}.tmp-${$}-chirho"
  if "$tree_build_chirho" emit "$app_root_chirho" >"$temporary_output_chirho" \
    && python3 "$product_root_chirho/scripts-chirho/lib-chirho/route-response-types-chirho.py" \
      "$app_root_chirho" "$temporary_output_chirho" \
    && python3 "$product_root_chirho/scripts-chirho/lib-chirho/release-guest-paths-chirho.py" \
      "$app_root_chirho" "$temporary_output_chirho"; then
    mv "$temporary_output_chirho" "$output_chirho"
  else
    local emit_status_chirho=$?
    rm -f "$temporary_output_chirho"
    return "$emit_status_chirho"
  fi
}
