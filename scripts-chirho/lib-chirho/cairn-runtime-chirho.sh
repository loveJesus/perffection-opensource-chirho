#!/usr/bin/env bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
# Workflow: spec-chirho/workflows-chirho/product-delivery-chirho/product-authoring-chirho.md.
# Read-only installed-runtime selection, not a build or a release attestation. Stdout is the path.

resolve_cairn_pin_chirho() {
  local product_chirho="$1" engine_chirho="$2" pin_chirho
  if [[ ! -f "$product_chirho/CAIRN-PIN-chirho.txt" ]]; then
    printf 'refusing: missing CAIRN-PIN-chirho.txt in %s\n' "$product_chirho" >&2
    return 1
  fi
  pin_chirho="$(sed -e '/^[[:space:]]*#/d' -e '/^[[:space:]]*$/d' \
    -e 's/^[[:space:]]*//' -e 's/[[:space:]]*$//' "$product_chirho/CAIRN-PIN-chirho.txt")" || return 1
  if [[ "$pin_chirho" =~ ^[0-9a-f]{40}$ ]]; then
    : # A full commit, never a moving branch or an abbreviated hash.
  elif [[ "$pin_chirho" =~ ^pin-chirho/[^[:space:]]+$ ]]; then
    pin_chirho="refs/tags/$pin_chirho"
  else
    printf '%s\n' 'refusing: Cairn pin must be one full commit or pin-chirho tag' >&2
    return 1
  fi
  if ! git -C "$engine_chirho" rev-parse --verify --quiet "${pin_chirho}^{commit}"; then
    printf 'refusing: Cairn pin is unavailable in %s\n' "$engine_chirho" >&2
    return 1
  fi
}

# A CANDIDATE run: CANDIDATE_CAIRN_COMMIT_CHIRHO (one full commit) together with CAIRN_CLI_CHIRHO
# names an engine build that is NOT the pin, for re-gating a product against an engine successor
# before the pin moves (an occupied-name fix, for instance). The stamp must equal that commit
# exactly - no descendant tolerance - and every gate says CANDIDATE in its log and evidence. It is
# never inferred from an installed binary: without the variable, a non-pin build is refused.
#
# CANDIDATE_CAIRN_SHA256_CHIRHO (the delivered executable's full SHA-256) is OPTIONAL here and,
# when set, must match the bytes or the run refuses. The shell gates bind a run to a stamp; gpt's
# reviewed-app consumer REQUIRES the delivered hash before it executes its frozen copy, so a
# handoff that names only a commit satisfies these gates and not that consumer. A stamp says which
# source a build claims; the hash says which bytes were delivered.
candidate_cairn_commit_chirho() {
  local candidate_chirho="${CANDIDATE_CAIRN_COMMIT_CHIRHO:-}"
  [[ -z "$candidate_chirho" ]] && return 1
  if [[ ! "$candidate_chirho" =~ ^[0-9a-f]{40}$ ]]; then
    printf 'refusing: CANDIDATE_CAIRN_COMMIT_CHIRHO must be one full commit\n' >&2
    return 2
  fi
  if [[ "${CAIRN_CLI_CHIRHO+x}" != x ]]; then
    printf 'refusing: a candidate run needs CAIRN_CLI_CHIRHO to name the candidate executable\n' >&2
    return 2
  fi
  printf '%s\n' "$candidate_chirho"
}

check_cairn_runtime_chirho() {
  local cli_chirho="$1" engine_chirho="$2" pin_chirho="$3"
  local version_chirho stamp_chirho changes_chirho candidate_chirho
  local pattern_chirho='^cairn-chirho [^[:space:]]+ \(([0-9a-f]{40})\)$'
  if [[ ! -f "$cli_chirho" || ! -x "$cli_chirho" ]]; then
    printf 'unavailable Cairn executable: %s\n' "$cli_chirho" >&2
    return 1
  fi
  if ! version_chirho="$("$cli_chirho" --version)" || [[ ! "$version_chirho" =~ $pattern_chirho ]]; then
    printf 'refusing unstamped Cairn executable: %s\n' "$cli_chirho" >&2
    return 1
  fi
  stamp_chirho="${BASH_REMATCH[1]}"
  candidate_chirho="$(candidate_cairn_commit_chirho)"; case $? in 2) return 1 ;; esac
  if [[ -n "$candidate_chirho" ]]; then
    if [[ "$stamp_chirho" != "$candidate_chirho" ]]; then
      printf 'Cairn executable is not the named CANDIDATE %s: %s (%s)\n' \
        "${candidate_chirho:0:12}" "$cli_chirho" "${stamp_chirho:0:12}" >&2
      return 1
    fi
    local sha_note_chirho="delivered SHA-256 not supplied: bound to the stamp only" expected_sha_chirho actual_sha_chirho
    expected_sha_chirho="${CANDIDATE_CAIRN_SHA256_CHIRHO:-}"
    if [[ -n "$expected_sha_chirho" ]]; then
      if [[ ! "$expected_sha_chirho" =~ ^[0-9a-f]{64}$ ]]; then
        printf 'refusing: CANDIDATE_CAIRN_SHA256_CHIRHO must be one full lowercase SHA-256\n' >&2
        return 1
      fi
      actual_sha_chirho="$(shasum -a 256 "$cli_chirho" | awk '{print $1}')" || return 1
      if [[ "$actual_sha_chirho" != "$expected_sha_chirho" ]]; then
        printf 'Cairn executable bytes are not the delivered SHA-256 %s: %s (%s)\n' \
          "${expected_sha_chirho:0:12}" "$cli_chirho" "${actual_sha_chirho:0:12}" >&2
        return 1
      fi
      sha_note_chirho="delivered SHA-256 ${expected_sha_chirho:0:12} verified"
    fi
    printf '  cairn runtime %s (CANDIDATE stamp=%s, NOT the pin %s; %s)\n' "$cli_chirho" "$stamp_chirho" "${pin_chirho:0:12}" "$sha_note_chirho" >&2
    return 0
  fi
  if [[ "$stamp_chirho" != "$pin_chirho" ]]; then
    if ! git -C "$engine_chirho" merge-base --is-ancestor "$pin_chirho" "$stamp_chirho" 2>/dev/null; then
      printf 'Cairn executable does not contain pin %s: %s (%s)\n' \
        "${pin_chirho:0:12}" "$cli_chirho" "${stamp_chirho:0:12}" >&2
      return 1
    fi
    # Compare net source bytes, including deleted/renamed paths; do not trust a commit's subject.
    changes_chirho="$(git -C "$engine_chirho" diff --name-only --no-renames "$pin_chirho" "$stamp_chirho" -- . ':!*.md')" || return 1
    if [[ -n "$changes_chirho" ]]; then
      printf 'Cairn executable has non-documentation drift from pin %s: %s (%s)\n' \
        "${pin_chirho:0:12}" "$cli_chirho" "${stamp_chirho:0:12}" >&2
      return 1
    fi
  fi
  printf '  cairn runtime %s (stamp=%s pin=%s)\n' "$cli_chirho" "$stamp_chirho" "$pin_chirho" >&2
}

resolve_cairn_runtime_chirho() {
  if [[ $# != 2 ]]; then
    printf '%s\n' 'usage: cairn-runtime-chirho.sh <product root> <Cairn checkout>' >&2
    return 2
  fi
  local product_chirho="$1" engine_chirho="$2" pin_chirho cli_chirho
  local candidates_chirho=()
  pin_chirho="$(resolve_cairn_pin_chirho "$product_chirho" "$engine_chirho")" || return 1
  if [[ "${CAIRN_CLI_CHIRHO+x}" == x ]]; then
    # An explicit override is a request, not a hint. Never silently replace a mismatched override.
    # (With CANDIDATE_CAIRN_COMMIT_CHIRHO it is a candidate run; see candidate_cairn_commit_chirho.)
    candidates_chirho=("$CAIRN_CLI_CHIRHO")
  else
    candidates_chirho=("$engine_chirho/target/debug/cairn-chirho" "$engine_chirho/target/release/cairn-chirho")
  fi
  for cli_chirho in "${candidates_chirho[@]}"; do
    if check_cairn_runtime_chirho "$cli_chirho" "$engine_chirho" "$pin_chirho"; then
      printf '%s/%s\n' "$(cd "$(dirname "$cli_chirho")" && pwd -P)" "$(basename "$cli_chirho")"
      return 0
    fi
  done
  printf '%s\n' 'refusing: no installed Cairn runtime matches the product pin; install a matching CLI or set CAIRN_CLI_CHIRHO to one. Nothing was built.' >&2
  return 1
}

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  resolve_cairn_runtime_chirho "$@"
fi
