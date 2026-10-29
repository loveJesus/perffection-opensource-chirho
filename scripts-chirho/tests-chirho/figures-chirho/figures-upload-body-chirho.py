# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
"""The figures gate's crafted uploads (../figures-rehearsal-chirho.sh, arm 10): a figures form framed as a
browser frames multipart/form-data, for the refusals Chrome itself would not send.

usage: figures-upload-body-chirho.py <out file> <good|file-and-paste|empty-file|not-text|broken-framing> <kind>
Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/her-figures-chirho.md
"""
import sys

out_chirho, case_chirho, kind_chirho = sys.argv[1], sys.argv[2], sys.argv[3]
BOUNDARY_CHIRHO = "----WebKitFormBoundaryGateChirho0123"
csv_chirho = f"month,kind,money_in,costs,jobs\r\n2026-03,{kind_chirho},10,,\r\n".encode()
file_chirho = {"empty-file": b"", "not-text": b"month,\xff\xfe\r\n"}.get(case_chirho, csv_chirho)


def part_chirho(name_chirho, value_chirho, filename_chirho=None):
    head_chirho = f'--{BOUNDARY_CHIRHO}\r\nContent-Disposition: form-data; name="{name_chirho}"'
    if filename_chirho is not None:
        head_chirho += f'; filename="{filename_chirho}"\r\nContent-Type: text/csv'
    return head_chirho.encode() + b"\r\n\r\n" + value_chirho + b"\r\n"


close_chirho = f"--{BOUNDARY_CHIRHO}--\r\n".encode()
body_chirho = (
    part_chirho("expected_version_chirho", b"3")
    + part_chirho("csv_chirho", csv_chirho if case_chirho == "file-and-paste" else b"")
    + part_chirho("csv_file_chirho", file_chirho, "gate.csv")
    + part_chirho("currency_chirho", b"USD")
    + part_chirho("note_chirho", b"gate")
    + close_chirho
)
if case_chirho == "broken-framing":
    body_chirho = body_chirho[: -len(close_chirho)]  # the close never comes
with open(out_chirho, "wb") as handle_chirho:
    handle_chirho.write(body_chirho)
