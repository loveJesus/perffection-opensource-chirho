#!/usr/bin/env python3
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
"""Read the dev example's balance lines as one validated record per role and phase, for the first-draft gate.

Why this exists: Cairn's served_llm_dev_serve_chirho prints, before it takes the ledger and after it stops, one
line per account IN its verified substrate snapshot (the union of the snapshot's net and held keys;
examples/served_llm_dev_serve_chirho.rs at 61660c6c):
    served-llm-dev-balance-chirho phase_chirho=<before|after> principal_chirho=<p> kind_chirho=<k> net_chirho=<n> held_chirho=<n>
An account with no postings has NO line, so on a fresh dev root the settlement owner has no "before" line until its
first charge settles (PFE #29130). That one absence, and no other, is a zero baseline, and only from a complete
snapshot: the example exited 0 on its stop file, on a root the gate created in this run. The payer's two records
and the owner's "after" record are required. A line carrying the balance prefix that does not parse whole, a second
record for one role and phase, or a "before" line after an "after" line is an error, never an absence (gpt #29132).

usage: dev-balance-records-chirho.py <log> <payer> <owner> <kind> <stop-how> <root-state>
    <stop-how> is the gate's STOP_HOW_CHIRHO; <root-state> is fresh-chirho when the gate created the root itself.
Prints ONE line of fields and never a principal. The slots run payer before / owner before / payer after / owner
after, and every money value is `unavailable` unless the records are valid:
    records_chirho=valid|invalid:<reason> presence_chirho=<present|absent|duplicate>/... net_chirho=<n>/<n>/<n>/<n>
    held_chirho=<n>/<n>/<n>/<n> payer_net_change_chirho=<n> owner_net_change_chirho=<n> other_records_chirho=<n>
Exit 0 valid, 1 invalid, 2 refusal. Harness: scripts-chirho/lib-chirho/tests-chirho/dev-balance-records-chirho-test.sh.
Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/first-draft-chirho.md (the gate's card phase).
"""
import re
import sys

PREFIX_CHIRHO = "served-llm-dev-balance-chirho"
NUMBER_CHIRHO = r"(0|-?[1-9][0-9]*)"
LINE_RE_CHIRHO = re.compile(
    rf"{PREFIX_CHIRHO} phase_chirho=(before|after) principal_chirho=(\S+) kind_chirho=(\S+) "
    rf"net_chirho={NUMBER_CHIRHO} held_chirho={NUMBER_CHIRHO}")
SLOTS_CHIRHO = (("payer", "before"), ("owner", "before"), ("payer", "after"), ("owner", "after"))
# The only record allowed to be absent: the settlement owner has no account until its first charge settles.
ZERO_BASELINE_SLOT_CHIRHO = ("owner", "before")
FIELDS_CHIRHO = ("records_chirho", "presence_chirho", "net_chirho", "held_chirho", "payer_net_change_chirho",
                 "owner_net_change_chirho", "other_records_chirho")
UNAVAILABLE_CHIRHO = "unavailable"


def refuse_chirho(message_chirho: str) -> None:
    print(f"refusing: {message_chirho}", file=sys.stderr)
    raise SystemExit(2)


def read_records_chirho(log_path_chirho, roles_chirho, kind_chirho):
    """Returns (read error or None, {slot: [(net, held), ...]}, count of other accounts' or kinds' records)."""
    records_chirho = {slot_chirho: [] for slot_chirho in SLOTS_CHIRHO}
    other_chirho = 0
    try:
        with open(log_path_chirho, encoding="utf-8") as log_chirho:
            text_chirho = log_chirho.read()
    except (OSError, UnicodeDecodeError):
        return "log-unreadable", records_chirho, other_chirho
    after_seen_chirho = False
    for line_chirho in text_chirho.split("\n"):
        # Anywhere in the line, not only at its start: a record glued behind other output must not read as absent.
        if PREFIX_CHIRHO not in line_chirho:
            continue
        match_chirho = LINE_RE_CHIRHO.fullmatch(line_chirho)
        if match_chirho is None:
            return "malformed-balance-line", records_chirho, other_chirho
        phase_chirho, principal_chirho, line_kind_chirho, net_chirho, held_chirho = match_chirho.groups()
        if phase_chirho == "after":
            after_seen_chirho = True
        elif after_seen_chirho:
            return "before-line-after-an-after-line", records_chirho, other_chirho
        role_chirho = roles_chirho.get(principal_chirho)
        if role_chirho is None or line_kind_chirho != kind_chirho:
            other_chirho += 1
            continue
        records_chirho[(role_chirho, phase_chirho)].append((int(net_chirho), int(held_chirho)))
    return None, records_chirho, other_chirho


def presence_chirho(records_chirho) -> str:
    return "/".join(
        "absent" if not found_chirho else "present" if len(found_chirho) == 1 else "duplicate"
        for found_chirho in (records_chirho[slot_chirho] for slot_chirho in SLOTS_CHIRHO))


def invalid_reason_chirho(records_chirho, stop_how_chirho, root_state_chirho):
    """The first reason these records cannot stand, or None."""
    if stop_how_chirho != "exited:0":
        return "example-not-finished"
    for slot_chirho in SLOTS_CHIRHO:
        name_chirho = "-".join(slot_chirho)
        if len(records_chirho[slot_chirho]) > 1:
            return f"duplicate-{name_chirho}"
        if not records_chirho[slot_chirho] and slot_chirho != ZERO_BASELINE_SLOT_CHIRHO:
            return f"missing-{name_chirho}"
    if not records_chirho[ZERO_BASELINE_SLOT_CHIRHO] and root_state_chirho != "fresh-chirho":
        return "owner-before-absent-on-an-unproven-root"
    return None


def main_chirho() -> int:
    if len(sys.argv) != 7:
        refuse_chirho("usage: dev-balance-records-chirho.py <log> <payer> <owner> <kind> <stop-how> <root-state>")
    log_path_chirho, payer_chirho, owner_chirho, kind_chirho, stop_how_chirho, root_state_chirho = sys.argv[1:]
    if not payer_chirho or not owner_chirho or not kind_chirho:
        refuse_chirho("the payer, the owner and the kind must each be named")
    if payer_chirho == owner_chirho:
        refuse_chirho("the payer and the owner must be different principals")
    fields_chirho = dict.fromkeys(FIELDS_CHIRHO, UNAVAILABLE_CHIRHO)
    read_error_chirho, records_chirho, other_chirho = read_records_chirho(
        log_path_chirho, {payer_chirho: "payer", owner_chirho: "owner"}, kind_chirho)
    if read_error_chirho:
        fields_chirho["records_chirho"] = f"invalid:{read_error_chirho}"
    else:
        fields_chirho["presence_chirho"] = presence_chirho(records_chirho)
        fields_chirho["other_records_chirho"] = str(other_chirho)
        reason_chirho = invalid_reason_chirho(records_chirho, stop_how_chirho, root_state_chirho)
        fields_chirho["records_chirho"] = f"invalid:{reason_chirho}" if reason_chirho else "valid"
        if reason_chirho is None:
            # Exactly one record per slot now, except the owner's allowed absence, which is its zero baseline.
            values_chirho = [found_chirho[0] if found_chirho else (0, 0)
                             for found_chirho in (records_chirho[slot_chirho] for slot_chirho in SLOTS_CHIRHO)]
            nets_chirho = [net_chirho for net_chirho, held_chirho in values_chirho]
            helds_chirho = [held_chirho for net_chirho, held_chirho in values_chirho]
            payer_before_chirho, owner_before_chirho, payer_after_chirho, owner_after_chirho = nets_chirho
            fields_chirho["net_chirho"] = "/".join(map(str, nets_chirho))
            fields_chirho["held_chirho"] = "/".join(map(str, helds_chirho))
            fields_chirho["payer_net_change_chirho"] = str(payer_after_chirho - payer_before_chirho)
            fields_chirho["owner_net_change_chirho"] = str(owner_after_chirho - owner_before_chirho)
    print(" ".join(f"{name_chirho}={value_chirho}" for name_chirho, value_chirho in fields_chirho.items()))
    return 0 if fields_chirho["records_chirho"] == "valid" else 1


if __name__ == "__main__":
    sys.exit(main_chirho())
