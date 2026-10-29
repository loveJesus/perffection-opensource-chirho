#!/usr/bin/env bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
#
# Fail-closed reads for the rehearsal gates' privacy arms. "Her words are nowhere under the root"
# and "no password rests in the root" are absence claims, and an absence is only evidence if the
# instrument would have said so had it been unable to look. A plain
# `grep -rl ... 2>/dev/null | wc -l` hides grep's stderr and status: measured on BSD grep 2.6.0, an
# unreadable file HOLDING the needle gives exit 2 with zero hits, which that pipe prints as 0
# (gpt #24135). These functions print a number, or `scan-error:...` when the scan could not
# complete, so an `expect ... 0` arm FAILS instead of passing on a partial scan.
#
# Sourced by the gates; harness: scripts-chirho/tests-chirho/root-scan-chirho-test.sh.

# scan_root_chirho <needle> <root>: how many files under the root hold the needle, matched as a
# fixed string (a password is not a regex). grep's exit 1 is "no match" and is an answer; 2, or
# anything on stderr, is an error.
scan_root_chirho() {
  local needle_chirho="$1" root_chirho="$2" out_chirho rc_chirho err_chirho
  [ -d "$root_chirho" ] || { printf 'scan-error:missing-root'; return 0; }
  err_chirho="$(mktemp "${TMPDIR:-/tmp}/root-scan-stderr-chirho.XXXXXX")" || { printf 'scan-error:no-temp'; return 0; }
  out_chirho="$(grep -rlF -- "$needle_chirho" "$root_chirho" 2>"$err_chirho")"; rc_chirho=$?
  if [ "$rc_chirho" -ge 2 ] || [ -s "$err_chirho" ]; then
    rm -f "$err_chirho"; printf 'scan-error:rc=%s' "$rc_chirho"; return 0
  fi
  rm -f "$err_chirho"
  if [ -z "$out_chirho" ]; then printf '0'; else printf '%s\n' "$out_chirho" | wc -l | tr -d ' '; fi
}

# file_hits_chirho <needle> <file>: matching lines in one file, or `scan-error:...` when the file
# cannot be read (a missing serve log is not a clean serve log).
file_hits_chirho() {
  local out_chirho rc_chirho
  [ -r "$2" ] || { printf 'scan-error:unreadable'; return 0; }
  out_chirho="$(grep -cF -- "$1" "$2" 2>/dev/null)"; rc_chirho=$?
  if [ "$rc_chirho" -ge 2 ]; then printf 'scan-error:rc=%s' "$rc_chirho"; else printf '%s' "$out_chirho"; fi
}

# seen_verdict_chirho <reading>: a count above zero is "sees"; zero, empty or a scan error is
# "blind(...)", with the reading shown.
seen_verdict_chirho() {
  case "$1" in scan-error*|0|'') printf 'blind(%s)' "$1";; *) printf 'sees';; esac
}

# scan_controls_chirho <needle> <scratch dir>: the two negative controls every privacy arm owes,
# through the gate's own `expect_chirho`, `say_chirho` and `FAILS_CHIRHO`: a missing root and an
# unreadable file holding the needle must each read as a scan error, never as a quiet 0. If this
# run can read a mode-000 file (a privileged run) the control proves nothing, and says so as a FAIL.
scan_controls_chirho() {
  local needle_chirho="$1" scratch_chirho="$2/scan-control-chirho"
  expect_chirho "control: a missing root is a scan error, not a 0" "$(scan_root_chirho "$needle_chirho" "$2/no-such-root-chirho")" "scan-error:missing-root"
  mkdir -p "$scratch_chirho"
  printf '%s\n' "$needle_chirho" > "$scratch_chirho/locked-chirho.txt"; chmod 000 "$scratch_chirho/locked-chirho.txt"
  if [ -r "$scratch_chirho/locked-chirho.txt" ]; then
    say_chirho "FAIL control: this run can read a mode-000 file (privileged), so the unreadable-file control proves nothing"
    FAILS_CHIRHO=$((FAILS_CHIRHO + 1))
  else
    expect_chirho "control: an unreadable file holding the needle is a scan error, not a 0" "$(scan_root_chirho "$needle_chirho" "$scratch_chirho")" "scan-error:rc=2"
  fi
  chmod 600 "$scratch_chirho/locked-chirho.txt"; rm -rf "$scratch_chirho"
}
