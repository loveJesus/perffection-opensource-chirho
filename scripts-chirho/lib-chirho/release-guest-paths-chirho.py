#!/usr/bin/env python3
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
"""Point an emitted manifest's cargo-example guests at their RELEASE builds.

Why this exists: tree-build's emit writes target/wasm32-unknown-unknown/debug/examples/ for every
cargo-example guest (cairn emit_chirho.rs, measured at 7fd2f37f on 2026-09-24), while prepare now
builds every shipped guest --release. Release guests are about 4% of the debug size and were
byte-identical across two checkouts; debug guests are neither. Cairn takes a profile-aware emit as a
later brick (broker #24856). Delete this step when the emitted paths already name the release profile.

usage: release-guest-paths-chirho.py <app-root> <emitted-manifest.json>
Rewrites every cargo-example artifact path in place. Exit 0 ok; exit 2 refuses an unexpected path
or a release artifact that does not exist, leaving the manifest unwritten.
"""
import json
import sys
from pathlib import Path

DEBUG_CHIRHO = "target/wasm32-unknown-unknown/debug/examples/"
RELEASE_CHIRHO = "target/wasm32-unknown-unknown/release/examples/"


def refuse_chirho(message_chirho: str) -> None:
    print(f"refusing: {message_chirho}", file=sys.stderr)
    sys.exit(2)


def main_chirho(arguments_chirho: list[str]) -> None:
    if len(arguments_chirho) != 3:
        refuse_chirho("usage: release-guest-paths-chirho.py <app-root> <emitted-manifest.json>")
    app_root_chirho = Path(arguments_chirho[1])
    manifest_path_chirho = Path(arguments_chirho[2])
    manifest_chirho = json.loads(manifest_path_chirho.read_text(encoding="utf-8"))
    rewritten_chirho = 0
    for node_chirho in manifest_chirho.get("nodes_chirho", []):
        source_chirho = node_chirho.get("source_chirho", {})
        if source_chirho.get("kind_chirho") != "cargo-example-chirho":
            continue
        artifact_chirho = source_chirho.get("artifact_path_chirho", "")
        if artifact_chirho.startswith(DEBUG_CHIRHO):
            artifact_chirho = RELEASE_CHIRHO + artifact_chirho[len(DEBUG_CHIRHO):]
        elif not artifact_chirho.startswith(RELEASE_CHIRHO):
            refuse_chirho(f"unexpected cargo-example artifact path: {artifact_chirho}")
        if not (app_root_chirho / artifact_chirho).is_file():
            refuse_chirho(f"release guest was not built: {artifact_chirho}")
        source_chirho["artifact_path_chirho"] = artifact_chirho
        rewritten_chirho += 1
    if rewritten_chirho:
        manifest_path_chirho.write_text(json.dumps(manifest_chirho, indent=2) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main_chirho(sys.argv)
