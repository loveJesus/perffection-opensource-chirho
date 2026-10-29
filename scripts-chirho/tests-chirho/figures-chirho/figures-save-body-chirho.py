# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
"""The K1 grant gate's figures Save bodies (key-grants-rehearsal-chirho.sh): the JSON her own assistant
sends to agent-save-figures-chirho, the complete figures document the read answers with, against a version.

usage: figures-save-body-chirho.py <out file> <expected version> <kind> <money in for 2026-02, minor units>
Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/her-figures-chirho.md
"""
import json
import sys

out_chirho, version_chirho, kind_chirho, february_chirho = sys.argv[1], int(sys.argv[2]), sys.argv[3], int(sys.argv[4])
body_chirho = {
    "expected_version_chirho": version_chirho,
    "figures_chirho": {
        "schema_chirho": "perffection-figures-v1-chirho",
        "currency_chirho": "USD",
        "note_chirho": "from her assistant",
        "months_chirho": ["2026-01", "2026-02"],
        "kinds_chirho": [kind_chirho],
        "money_in_chirho": [[120050, february_chirho]],
        "costs_chirho": [[40000, None]],
        "jobs_chirho": [[30, None]],
    },
}
with open(out_chirho, "w", encoding="utf-8") as handle_chirho:
    json.dump(body_chirho, handle_chirho)
