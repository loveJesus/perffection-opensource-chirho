#!/usr/bin/env bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
#
# Harness for scripts-chirho/lib-chirho/dev-balance-records-chirho.py, run exactly as the first-draft gate runs it
# (python3 -I on the real file). The owner's absent "before" record is a zero baseline only from a finished run on
# a fresh root; every other missing record is an error; a malformed, glued, duplicate or out-of-order line never
# reads as an absence; and an invalid read leaves every money field `unavailable`, so no gate check can pass on it
# (gpt #29132). Every fixture is a synthetic log; no engine root is touched.

set -uo pipefail

SCRIPT_ROOT_CHIRHO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PARSER_CHIRHO="$SCRIPT_ROOT_CHIRHO/lib-chirho/dev-balance-records-chirho.py"
TEMP_PARENT_CHIRHO="${TMPDIR:-/tmp}"
TEMP_PARENT_CHIRHO="${TEMP_PARENT_CHIRHO%/}"
TEST_ROOT_CHIRHO="$(mktemp -d "$TEMP_PARENT_CHIRHO/perffection-dev-balance-test-chirho.XXXXXX")"
cleanup_chirho() {
  if [[ "$TEST_ROOT_CHIRHO" == "$TEMP_PARENT_CHIRHO"/perffection-dev-balance-test-chirho.* ]]; then rm -rf -- "$TEST_ROOT_CHIRHO"; fi
}
trap cleanup_chirho EXIT

FAILS_CHIRHO=0
say_chirho() { printf '%s\n' "$*"; }
expect_chirho() { if [ "$2" = "$3" ]; then say_chirho "ok   $1 = $2"; else say_chirho "FAIL $1 = $2 (expected $3)"; FAILS_CHIRHO=$((FAILS_CHIRHO + 1)); fi; }

PAYER_CHIRHO=payer-fixture-principal; OWNER_CHIRHO=owner-fixture-principal; KIND_CHIRHO=money-chirho
# bal_chirho <phase> <principal> <net> <held> [kind]: one line exactly as the example prints it
bal_chirho() {
  printf 'served-llm-dev-balance-chirho phase_chirho=%s principal_chirho=%s kind_chirho=%s net_chirho=%s held_chirho=%s\n' \
    "$1" "$2" "${5:-$KIND_CHIRHO}" "$3" "$4"
}
# run_shape_chirho [extra lines among the "before" records] [extra lines after the "after" records]: the shape of run
# #29130's log, where the owner has no "before" line and the example's other lines sit between the snapshots.
run_shape_chirho() {
  printf 'ResidentCurrentHolderWriteNoMirrorRuntimeChirho\n'
  bal_chirho before "$PAYER_CHIRHO" 137 0
  if [ -n "${1:-}" ]; then printf '%s\n' "$1"; fi
  printf 'cairn-chirho serving on http://127.0.0.1:8856\n'
  printf 'CairnAppExecutionRefusalChirho:attempt_ordinal_chirho=unclassified-chirho:identity_chirho=DagExecutionUnclassifiedRefusedChirho\n'
  bal_chirho after "$OWNER_CHIRHO" 14 0
  bal_chirho after "$PAYER_CHIRHO" 123 0
  if [ -n "${2:-}" ]; then printf '%s\n' "$2"; fi
}
OUTPUTS_CHIRHO=""
# parse_chirho <log name> [stop-how] [root-state]: the parser's one line, then " exit=<status>"
parse_chirho() {
  local out_chirho rc_chirho
  out_chirho="$(python3 -I "$PARSER_CHIRHO" "$TEST_ROOT_CHIRHO/$1.log" "$PAYER_CHIRHO" "$OWNER_CHIRHO" "$KIND_CHIRHO" \
    "${2:-exited:0}" "${3:-fresh-chirho}")"; rc_chirho=$?
  printf '%s exit=%s' "$out_chirho" "$rc_chirho"
}
field_chirho() { printf '%s\n' "$2" | tr ' ' '\n' | sed -n "s/^${1}_chirho=//p"; }
# money_chirho <parser line>: the four money fields; an invalid read must leave all four `unavailable`
money_chirho() { printf '%s %s %s %s' "$(field_chirho net "$1")" "$(field_chirho held "$1")" "$(field_chirho payer_net_change "$1")" "$(field_chirho owner_net_change "$1")"; }
NO_MONEY_CHIRHO="unavailable unavailable unavailable unavailable"
# check_invalid_chirho <label> <log name> <expected records> <expected presence> [stop-how] [root-state]
check_invalid_chirho() {
  local out_chirho; out_chirho="$(parse_chirho "$2" "${5:-exited:0}" "${6:-fresh-chirho}")"; OUTPUTS_CHIRHO+="$out_chirho"$'\n'
  expect_chirho "$1" "$(field_chirho records "$out_chirho") / $(field_chirho presence "$out_chirho") / $(money_chirho "$out_chirho") / ${out_chirho##* }" \
    "$3 / $4 / $NO_MONEY_CHIRHO / exit=1"
}

# 1. The run's own shape: the one allowed absence is a zero baseline, and the deltas are the aggregate.
VALID_CHIRHO="records_chirho=valid presence_chirho=present/absent/present/present net_chirho=137/0/123/14 held_chirho=0/0/0/0 payer_net_change_chirho=-14 owner_net_change_chirho=14 other_records_chirho=0"
run_shape_chirho > "$TEST_ROOT_CHIRHO/absent.log"
OUT_CHIRHO="$(parse_chirho absent)"; OUTPUTS_CHIRHO+="$OUT_CHIRHO"$'\n'
expect_chirho "the owner's absent before-record is a zero baseline on a fresh root after a clean finish" "$OUT_CHIRHO" "$VALID_CHIRHO exit=0"

# 2. Every other record is required.
while read -r phase_chirho who_chirho name_chirho; do
  run_shape_chirho | grep -v "phase_chirho=$phase_chirho principal_chirho=$who_chirho " > "$TEST_ROOT_CHIRHO/missing-$name_chirho.log"
  PRESENCE_CHIRHO="present/absent/present/present"
  case "$name_chirho" in
    payer-before) PRESENCE_CHIRHO="absent/absent/present/present";;
    payer-after) PRESENCE_CHIRHO="present/absent/absent/present";;
    owner-after) PRESENCE_CHIRHO="present/absent/present/absent";;
  esac
  check_invalid_chirho "a missing $name_chirho record is an error" "missing-$name_chirho" "invalid:missing-$name_chirho" "$PRESENCE_CHIRHO"
done <<EOF
before $PAYER_CHIRHO payer-before
after $PAYER_CHIRHO payer-after
after $OWNER_CHIRHO owner-after
EOF

# 3. A record that does not parse, or is glued behind other output, is an error and never an absence.
run_shape_chirho "served-llm-dev-balance-chirho phase_chirho=before principal_chirho=$OWNER_CHIRHO kind_chirho=$KIND_CHIRHO net_chirho=0 held_chirho=" \
  > "$TEST_ROOT_CHIRHO/malformed.log"
check_invalid_chirho "a malformed owner before-record is an error, not an absence" malformed "invalid:malformed-balance-line" unavailable
run_shape_chirho "CairnPublicErrorChirho:class_chirho=execution-refusal-chirho$(bal_chirho before "$OWNER_CHIRHO" 0 0)" > "$TEST_ROOT_CHIRHO/glued.log"
check_invalid_chirho "an owner before-record glued behind other output is an error, not an absence" glued "invalid:malformed-balance-line" unavailable

# 4. A second record for one role and phase is an error; nothing picks the first one.
run_shape_chirho "$(bal_chirho before "$OWNER_CHIRHO" 0 0; bal_chirho before "$OWNER_CHIRHO" 0 0)" > "$TEST_ROOT_CHIRHO/duplicate-owner.log"
check_invalid_chirho "a duplicate owner before-record is an error" duplicate-owner "invalid:duplicate-owner-before" "present/duplicate/present/present"
run_shape_chirho "" "$(bal_chirho after "$PAYER_CHIRHO" 130 0)" > "$TEST_ROOT_CHIRHO/duplicate-payer.log"
check_invalid_chirho "a duplicate payer after-record is an error" duplicate-payer "invalid:duplicate-payer-after" "present/absent/duplicate/present"

# 5. A "before" line after the "after" snapshot is an error (a log that mixes two runs).
run_shape_chirho "" "$(bal_chirho before "$OWNER_CHIRHO" 0 0)" > "$TEST_ROOT_CHIRHO/out-of-order.log"
check_invalid_chirho "a before-record after the after-snapshot is an error" out-of-order "invalid:before-line-after-an-after-line" unavailable

# 6. The absence counts only from a clean finish on a root this run created.
check_invalid_chirho "an absent owner before-record from an unfinished run is an error" absent "invalid:example-not-finished" "present/absent/present/present" "terminated:143"
check_invalid_chirho "an absent owner before-record on a root this run did not create is an error" absent "invalid:owner-before-absent-on-an-unproven-root" "present/absent/present/present" exited:0 reused-chirho

# 7. Held is reported as read, so the gate's 0/0/0/0 check fails on any hold.
{ printf 'cairn-chirho serving on http://127.0.0.1:8856\n'; bal_chirho before "$PAYER_CHIRHO" 137 0
  bal_chirho after "$OWNER_CHIRHO" 14 5; bal_chirho after "$PAYER_CHIRHO" 123 0; } > "$TEST_ROOT_CHIRHO/held.log"
OUT_CHIRHO="$(parse_chirho held)"; OUTPUTS_CHIRHO+="$OUT_CHIRHO"$'\n'
expect_chirho "a nonzero hold is reported as read" "$(field_chirho records "$OUT_CHIRHO")/$(field_chirho held "$OUT_CHIRHO")" "valid/0/0/0/5"

# 8. An explicit owner before-record is read as present. PFE does not accept it as the baseline form: the gate
#    expects present/absent/present/present, because at 61660c6c the example prints no line for an account with no
#    postings, so a line there is a contract change to review, not a quiet pass (gpt #29132 left it optional).
run_shape_chirho "$(bal_chirho before "$OWNER_CHIRHO" 0 0)" > "$TEST_ROOT_CHIRHO/explicit-zero.log"
OUT_CHIRHO="$(parse_chirho explicit-zero)"; OUTPUTS_CHIRHO+="$OUT_CHIRHO"$'\n'
expect_chirho "an explicit zero owner before-record reads as present, so the gate's presence check refuses it" \
  "$(field_chirho records "$OUT_CHIRHO")/$(field_chirho presence "$OUT_CHIRHO")" "valid/present/present/present/present"

# 9. Other accounts, and the payer's other kinds, are not the gate's records and do not disturb them.
run_shape_chirho "$(bal_chirho before third-fixture-principal 5 0; bal_chirho before "$PAYER_CHIRHO" 9 0 credits-chirho)" \
  "$(bal_chirho after third-fixture-principal 5 0)" > "$TEST_ROOT_CHIRHO/others.log"
OUT_CHIRHO="$(parse_chirho others)"; OUTPUTS_CHIRHO+="$OUT_CHIRHO"$'\n'
expect_chirho "other accounts and kinds are counted and left aside" "$OUT_CHIRHO" "${VALID_CHIRHO% other_records_chirho=0} other_records_chirho=3 exit=0"

# 10. A log that cannot be read, and a refused call, leave nothing that can pass.
check_invalid_chirho "a missing log is an error" no-such-log "invalid:log-unreadable" unavailable
REFUSED_CHIRHO="$(python3 -I "$PARSER_CHIRHO" "$TEST_ROOT_CHIRHO/absent.log" "$PAYER_CHIRHO" "$PAYER_CHIRHO" "$KIND_CHIRHO" exited:0 fresh-chirho 2>/dev/null)"
REFUSED_RC_CHIRHO=$?
expect_chirho "the payer named as the owner is refused with nothing printed" "${REFUSED_CHIRHO:-nothing}/$REFUSED_RC_CHIRHO" "nothing/2"

# 11. No output line ever carries a principal; the controls show the count sees every output, and sees a principal
#     where one is.
expect_chirho "no principal in any parser output" "$(printf '%s' "$OUTPUTS_CHIRHO" | grep -c 'fixture-principal')" 0
expect_chirho "control: every collected output is there to be read" "$(printf '%s' "$OUTPUTS_CHIRHO" | grep -c 'records_chirho=')" 15
expect_chirho "control: the same count finds the principals in a fixture log" "$(grep -c 'fixture-principal' "$TEST_ROOT_CHIRHO/absent.log")" 3

if [ "$FAILS_CHIRHO" -gt 0 ]; then say_chirho "dev-balance-records-chirho tests FAILED: $FAILS_CHIRHO"; exit 1; fi
say_chirho "dev-balance-records-chirho tests passed"
