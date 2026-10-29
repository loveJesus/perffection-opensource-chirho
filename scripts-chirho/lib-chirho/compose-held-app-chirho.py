# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
"""A gate's scratch app-chirho.toml: the workspace's own stanzas, then everything held out of the real app,
for the chosen stages only. Nothing here touches the real app-chirho.toml.

- `deploy_chirho/figures-routes-chirho.toml`: the held figures routes, appended unless a release profile leaves them out
  (the gates bind them).
- `deploy_chirho/key-openings-chirho.toml`: `[[opening_chirho]]` entries; a chosen stage's openings are written
  into their route stanzas as `key_access_chirho`, `key_action_chirho` and `key_resources_chirho`.
- `deploy_chirho/held-bindings-chirho.toml`: `[[binding_chirho]]` entries; a chosen stage's binding moves its
  route onto the named DAG (the Account route onto the own-keys read, Cairn K3).
- `deploy_chirho/release-profiles-chirho.toml`: `[[profile_chirho]]` entries; "profile:<name>" chooses that profile's
  stages and whether her held figures routes come with them (a release candidate, gpt #26920).

usage: compose-held-app-chirho.py <app dir> <stages, comma separated, "none", or "profile:<name>"> <output file>
Used by key-grants-rehearsal-chirho.sh and account-keys-rehearsal-chirho.sh (lib-chirho/key-staging-chirho.sh), and by
the connected-tools release candidate (tests-chirho/account-chirho/connected-release-driver-chirho.sh).
Workflows: spec-chirho/workflows-chirho/invite-beta-chirho/her-figures-chirho.md and agent-api-chirho.md
"""
import re
import sys
import tomllib
from pathlib import Path

app_dir_chirho, stages_chirho, out_chirho = sys.argv[1:4]
deploy_chirho = Path(app_dir_chirho) / "deploy_chirho"
figures_routes_chirho = True
if stages_chirho.startswith("profile:"):
    profiles_chirho = tomllib.loads((deploy_chirho / "release-profiles-chirho.toml").read_text(encoding="utf-8"))["profile_chirho"]
    named_chirho = [entry_chirho for entry_chirho in profiles_chirho if entry_chirho["profile_chirho"] == stages_chirho.removeprefix("profile:")]
    if len(named_chirho) != 1:
        sys.exit(f"refusing: no single release profile named {stages_chirho.removeprefix('profile:')!r}")
    chosen_chirho = set(named_chirho[0]["stages_chirho"])
    figures_routes_chirho = named_chirho[0]["figures_routes_chirho"]
else:
    chosen_chirho = set() if stages_chirho == "none" else set(stages_chirho.split(","))
text_chirho = (Path(app_dir_chirho) / "app-chirho.toml").read_text(encoding="utf-8").rstrip("\n") + "\n\n"
if figures_routes_chirho:
    text_chirho += (deploy_chirho / "figures-routes-chirho.toml").read_text(encoding="utf-8")
openings_chirho = tomllib.loads((deploy_chirho / "key-openings-chirho.toml").read_text(encoding="utf-8"))["opening_chirho"]
bindings_chirho = tomllib.loads((deploy_chirho / "held-bindings-chirho.toml").read_text(encoding="utf-8"))["binding_chirho"]
known_chirho = {entry_chirho["stage_chirho"] for entry_chirho in openings_chirho + bindings_chirho}
if chosen_chirho - known_chirho:
    sys.exit(f"refusing: no held entries for stage(s) {sorted(chosen_chirho - known_chirho)}")
# one binding per route: two chosen stages binding the same route (the list-only and the creating Account page) refuse
# before anything is written, in either order (gpt #26829)
chosen_routes_chirho = [binding_chirho["route_chirho"] for binding_chirho in bindings_chirho if binding_chirho["stage_chirho"] in chosen_chirho]
doubled_chirho = sorted({route_chirho for route_chirho in chosen_routes_chirho if chosen_routes_chirho.count(route_chirho) > 1})
if doubled_chirho:
    sys.exit(f"refusing: more than one chosen binding for route(s) {doubled_chirho}; choose one")


def one_stanza_chirho(route_chirho: str) -> str:
    route_line_chirho = f'route_chirho = "{route_chirho}"\n'
    if text_chirho.count(route_line_chirho) != 1:
        sys.exit(f"refusing: route {route_chirho} is not exactly one stanza")
    return route_line_chirho


opened_chirho = {}
for opening_chirho in openings_chirho:
    if opening_chirho["stage_chirho"] not in chosen_chirho:
        continue
    route_line_chirho = one_stanza_chirho(opening_chirho["route_chirho"])
    resources_chirho = ", ".join(f'"{resource_chirho}"' for resource_chirho in opening_chirho["resources_chirho"])
    text_chirho = text_chirho.replace(route_line_chirho, route_line_chirho + (
        'key_access_chirho = "open"\n'
        f'key_action_chirho = "{opening_chirho["action_chirho"]}"\n'
        f"key_resources_chirho = [{resources_chirho}]\n"
    ))
    opened_chirho[opening_chirho["route_chirho"]] = opening_chirho
bound_chirho = {}
for binding_chirho in bindings_chirho:
    if binding_chirho["stage_chirho"] not in chosen_chirho:
        continue
    route_line_chirho = one_stanza_chirho(binding_chirho["route_chirho"])
    pattern_chirho = re.escape(route_line_chirho) + r'((?:key_[a-z_]+ = [^\n]*\n)*)dag_chirho = "[^"]*"'
    text_chirho, count_chirho = re.subn(pattern_chirho, lambda match_chirho: route_line_chirho + match_chirho.group(1) + f'dag_chirho = "{binding_chirho["dag_chirho"]}"', text_chirho)
    if count_chirho != 1:
        sys.exit(f"refusing: route {binding_chirho['route_chirho']} has no dag line to bind")
    bound_chirho[binding_chirho["route_chirho"]] = binding_chirho["dag_chirho"]
# what was written must read back exactly: each chosen route opened or bound once, and nothing else
for stanza_chirho in tomllib.loads(text_chirho).get("route_chirho", []):
    name_chirho = stanza_chirho["route_chirho"]
    if name_chirho in opened_chirho:
        expected_chirho = opened_chirho.pop(name_chirho)
        assert (stanza_chirho["key_access_chirho"], stanza_chirho["key_action_chirho"], stanza_chirho["key_resources_chirho"]) == ("open", expected_chirho["action_chirho"], expected_chirho["resources_chirho"]), name_chirho
    else:
        assert "key_access_chirho" not in stanza_chirho, name_chirho
    if name_chirho in bound_chirho:
        assert stanza_chirho["dag_chirho"] == bound_chirho.pop(name_chirho), name_chirho
assert not opened_chirho and not bound_chirho, f"never written: {sorted(opened_chirho) + sorted(bound_chirho)}"
Path(out_chirho).write_text(text_chirho, encoding="utf-8")
