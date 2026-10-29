#!/usr/bin/env bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
#
# Harness for scripts-chirho/lib-chirho/root-scan-chirho.sh: the privacy arms' instrument must
# count what it can see, answer 0 only for a COMPLETED scan with no match, and read every way of
# not being able to look (missing root, unreadable file, unreadable directory, unreadable single
# file) as a scan error. A needle is a fixed string, never a regex. Everything is a temporary
# synthetic tree; no engine root or product file is touched.

set -uo pipefail

SCRIPT_ROOT_CHIRHO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib-chirho/root-scan-chirho.sh
source "$SCRIPT_ROOT_CHIRHO/lib-chirho/root-scan-chirho.sh"
TEMP_PARENT_CHIRHO="${TMPDIR:-/tmp}"
TEMP_PARENT_CHIRHO="${TEMP_PARENT_CHIRHO%/}"
TEST_ROOT_CHIRHO="$(mktemp -d "$TEMP_PARENT_CHIRHO/perffection-root-scan-test-chirho.XXXXXX")"
cleanup_chirho() {
  if [[ "$TEST_ROOT_CHIRHO" == "$TEMP_PARENT_CHIRHO"/perffection-root-scan-test-chirho.* ]]; then
    chmod -R u+rwx "$TEST_ROOT_CHIRHO" 2>/dev/null; rm -rf -- "$TEST_ROOT_CHIRHO"
  fi
}
trap cleanup_chirho EXIT

FAILS_CHIRHO=0
say_chirho() { printf '%s\n' "$*"; }
expect_chirho() { if [ "$2" = "$3" ]; then say_chirho "ok   $1 = $2"; else say_chirho "FAIL $1 = $2 (expected $3)"; FAILS_CHIRHO=$((FAILS_CHIRHO + 1)); fi; }

ROOT_CHIRHO="$TEST_ROOT_CHIRHO/root-chirho"; mkdir -p "$ROOT_CHIRHO/sub-chirho"
printf 'a line holding NEEDLE-one here\n' > "$ROOT_CHIRHO/sub-chirho/hit-chirho.txt"
printf 'another NEEDLE-one\n' > "$ROOT_CHIRHO/second-hit-chirho.txt"
printf 'nothing to see\n' > "$ROOT_CHIRHO/clean-chirho.txt"

expect_chirho "two files hold the needle" "$(scan_root_chirho NEEDLE-one "$ROOT_CHIRHO")" 2
expect_chirho "a completed scan with no match is 0" "$(scan_root_chirho NEEDLE-absent "$ROOT_CHIRHO")" 0
expect_chirho "a needle is a fixed string, not a regex" "$(scan_root_chirho '.*' "$ROOT_CHIRHO")" 0
expect_chirho "a needle that starts with a dash is still a needle" "$(scan_root_chirho '-rf' "$ROOT_CHIRHO")" 0
expect_chirho "a missing root is a scan error" "$(scan_root_chirho NEEDLE-one "$TEST_ROOT_CHIRHO/nowhere-chirho")" "scan-error:missing-root"
expect_chirho "sees / blind readings" "$(seen_verdict_chirho 3)/$(seen_verdict_chirho 0)/$(seen_verdict_chirho scan-error:rc=2)" "sees/blind(0)/blind(scan-error:rc=2)"

printf 'locked away: NEEDLE-two\n' > "$ROOT_CHIRHO/locked-chirho.txt"; chmod 000 "$ROOT_CHIRHO/locked-chirho.txt"
if [ -r "$ROOT_CHIRHO/locked-chirho.txt" ]; then
  say_chirho "FAIL this run can read a mode-000 file (privileged): the unreadable arms below prove nothing"; FAILS_CHIRHO=$((FAILS_CHIRHO + 1))
else
  expect_chirho "an unreadable file HOLDING the needle is a scan error, never 0" "$(scan_root_chirho NEEDLE-two "$ROOT_CHIRHO")" "scan-error:rc=2"
  expect_chirho "an unreadable file beside a real hit is still not a number" "$(scan_root_chirho NEEDLE-one "$ROOT_CHIRHO")" "scan-error:rc=2"
  expect_chirho "an unreadable single file is a scan error" "$(file_hits_chirho NEEDLE-two "$ROOT_CHIRHO/locked-chirho.txt")" "scan-error:unreadable"
fi
chmod 600 "$ROOT_CHIRHO/locked-chirho.txt"; rm -f "$ROOT_CHIRHO/locked-chirho.txt"

mkdir "$ROOT_CHIRHO/locked-dir-chirho"; printf 'NEEDLE-three\n' > "$ROOT_CHIRHO/locked-dir-chirho/inside-chirho.txt"; chmod 000 "$ROOT_CHIRHO/locked-dir-chirho"
expect_chirho "an unreadable directory is a scan error" "$(scan_root_chirho NEEDLE-three "$ROOT_CHIRHO")" "scan-error:rc=2"
chmod 700 "$ROOT_CHIRHO/locked-dir-chirho"; rm -rf "$ROOT_CHIRHO/locked-dir-chirho"

expect_chirho "after the faults are removed the scan answers again" "$(scan_root_chirho NEEDLE-one "$ROOT_CHIRHO")" 2
expect_chirho "lines in one file / a missing file" "$(file_hits_chirho NEEDLE-one "$ROOT_CHIRHO/second-hit-chirho.txt")/$(file_hits_chirho NEEDLE-one "$ROOT_CHIRHO/no-such-file-chirho")" "1/scan-error:unreadable"

# the two controls the gates run, through this harness's own expect
scan_controls_chirho NEEDLE-one "$TEST_ROOT_CHIRHO"

if [ "$FAILS_CHIRHO" -gt 0 ]; then say_chirho "root-scan-chirho tests FAILED: $FAILS_CHIRHO"; exit 1; fi
say_chirho "root-scan-chirho tests passed"
