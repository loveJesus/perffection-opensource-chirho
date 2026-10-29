#!/usr/bin/env python3
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
"""Pass a route's DECLARED response type from app-chirho.toml through to the emitted manifest.

Why this exists: the engine runtime lets an app route answer text/html, text/plain or
application/json (cairn-model-app-chirho normalizer), but tree-build's emit writes "text/html" for
every route and reads no such key from the source descriptor (measured at cairn d60037f3, 1ebb16f9
and HEAD d108277a on 2026-09-21). The SOURCE stays the one place a route's type is declared
(`response_content_type_chirho` in its [[route_chirho]] stanza); this step only copies what the
source says into the build output. Delete it when tree-build passes the key through itself - the
step refuses loudly if the two ever disagree in a way it does not understand.

usage: route-response-types-chirho.py <app-root> <emitted-manifest.json>
Rewrites the manifest in place only when a route declares a type. Exit 0 ok, 2 refusal.
"""
import json
import sys
import tomllib
from pathlib import Path

ALLOWED_CHIRHO = ("text/html", "text/plain", "application/json")
KEY_CHIRHO = "response_content_type_chirho"


def refuse_chirho(message_chirho: str) -> None:
    print(f"refusing: {message_chirho}", file=sys.stderr)
    raise SystemExit(2)


def main_chirho() -> None:
    if len(sys.argv) != 3:
        refuse_chirho("usage: route-response-types-chirho.py <app-root> <emitted-manifest.json>")
    app_root_chirho, manifest_path_chirho = Path(sys.argv[1]), Path(sys.argv[2])
    with (app_root_chirho / "app-chirho.toml").open("rb") as source_chirho:
        descriptor_chirho = tomllib.load(source_chirho)
    declared_chirho = {}
    for route_chirho in descriptor_chirho.get("route_chirho", []):
        if KEY_CHIRHO in route_chirho:
            if route_chirho[KEY_CHIRHO] not in ALLOWED_CHIRHO:
                refuse_chirho(f"route {route_chirho.get('route_chirho')} declares an unsupported type")
            declared_chirho[route_chirho["route_chirho"]] = route_chirho[KEY_CHIRHO]
    if not declared_chirho:
        return
    manifest_chirho = json.loads(manifest_path_chirho.read_text())
    emitted_chirho = {
        route_chirho["route_chirho"]: route_chirho
        for route_chirho in manifest_chirho["deploy_chirho"]["routes_chirho"]
    }
    for name_chirho, type_chirho in declared_chirho.items():
        if name_chirho not in emitted_chirho:
            refuse_chirho(f"declared route {name_chirho} is not in the emitted manifest")
        emitted_chirho[name_chirho][KEY_CHIRHO] = type_chirho
    manifest_path_chirho.write_text(json.dumps(manifest_chirho, indent=2) + "\n")
    for name_chirho in sorted(declared_chirho):
        print(f"  route-type  {name_chirho} = {declared_chirho[name_chirho]}")


if __name__ == "__main__":
    main_chirho()
