# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
"""One upstream OFL font -> one self-hosted WOFF2, subset to the Latin the site uses.

The engine serves fonts only as installed assets (font/woff2), so every face is subset here first.
Three licence duties are kept in code rather than in memory:
- name IDs 0, 13 and 14 (copyright, licence, licence URL) travel inside the file unchanged;
- a subset is a Modified Version under the OFL, so a face whose copyright line reserves its name
  (IM FELL does) must be renamed: --rename sets the family, full, unique and PostScript names, and
  the script refuses to write a file that still carries the reserved name as a primary name;
- --instance pins variable axes we do not animate (Cabin's width) so the file stays small;
- --kern-pair sets one legacy kerning pair, by characters and font units. IM FELL's italic kerns W
  then h by +173, a gap that reads as a word break at display sizes ("W hat"); -60 closes it
  without a collision (2026-10-01, compared on rendered text).

usage: python3 subset-font-chirho.py <in.ttf> <out.woff2> [--rename "Family Name"] [--instance wdth=100]
                                     [--kern-pair W:h:-60]
"""
import argparse  # Workflow: spec-chirho/workflows-chirho/first-light-authoring-chirho.md
import io
import os
import re

from fontTools import subset
from fontTools.ttLib import TTFont
from fontTools.varLib import instancer

UNICODES_CHIRHO = ("U+0020-007E,U+00A0-00FF,U+0131,U+0152-0153,U+02BB-02BC,U+02C6,U+02DA,U+02DC,"
                   "U+2000-206F,U+2074,U+20AC,U+2122,U+2191-2193,U+2196-2199,U+2212,U+2215,U+2261,"
                   "U+221E,U+2248,U+2713,U+25A0,U+25CF,U+2605,U+FEFF,U+FFFD")
FEATURES_CHIRHO = ["kern", "liga", "calt", "onum", "lnum", "tnum", "pnum", "case"]
RENAMED_IDS_CHIRHO = (1, 3, 4, 6, 16, 17, 21, 22)

parser_chirho = argparse.ArgumentParser()
parser_chirho.add_argument("source_chirho")
parser_chirho.add_argument("out_chirho")
parser_chirho.add_argument("--rename", dest="rename_chirho")
parser_chirho.add_argument("--instance", dest="instance_chirho", action="append", default=[])
parser_chirho.add_argument("--kern-pair", dest="kern_pairs_chirho", action="append", default=[])
arguments_chirho = parser_chirho.parse_args()

font_chirho = TTFont(arguments_chirho.source_chirho)
copyright_chirho = font_chirho["name"].getDebugName(0) or ""
reserved_chirho = re.search(r"reserved (font )?name\s+(.+)$", copyright_chirho, re.IGNORECASE)
if reserved_chirho and not arguments_chirho.rename_chirho:
    raise SystemExit(f"refusing: '{copyright_chirho}' reserves its name, so a subset needs --rename")

if arguments_chirho.instance_chirho:
    limits_chirho = {}
    for pin_chirho in arguments_chirho.instance_chirho:
        axis_chirho, value_chirho = pin_chirho.split("=")
        low_chirho, _, high_chirho = value_chirho.partition(":")
        limits_chirho[axis_chirho] = (float(low_chirho), float(high_chirho)) if high_chirho else float(low_chirho)
    font_chirho = instancer.instantiateVariableFont(font_chirho, limits_chirho)
    # Round-trip once: the subsetter cannot read an instanced gvar that is still lazily loaded
    # (KeyError 'space', fontTools 4.6x).
    buffer_chirho = io.BytesIO()
    font_chirho.save(buffer_chirho)
    buffer_chirho.seek(0)
    font_chirho = TTFont(buffer_chirho)

options_chirho = subset.Options()
options_chirho.flavor = "woff2"
options_chirho.layout_features = FEATURES_CHIRHO
options_chirho.name_IDs = ["*"]
options_chirho.name_languages = ["*"]
options_chirho.notdef_outline = True
options_chirho.recalc_bounds = True
subsetter_chirho = subset.Subsetter(options=options_chirho)
subsetter_chirho.populate(unicodes=subset.parse_unicodes(UNICODES_CHIRHO))
subsetter_chirho.subset(font_chirho)

if arguments_chirho.rename_chirho:
    family_chirho = arguments_chirho.rename_chirho
    style_chirho = font_chirho["name"].getDebugName(2) or "Regular"
    postscript_chirho = f"{family_chirho.replace(' ', '')}-{style_chirho.replace(' ', '')}"
    new_names_chirho = {1: family_chirho, 3: f"{postscript_chirho};subset", 4: f"{family_chirho} {style_chirho}",
                        6: postscript_chirho, 16: family_chirho, 17: style_chirho,
                        21: family_chirho, 22: style_chirho}
    for record_chirho in font_chirho["name"].names:
        if record_chirho.nameID in RENAMED_IDS_CHIRHO:
            record_chirho.string = new_names_chirho[record_chirho.nameID]
if reserved_chirho:
    # The OFL's restriction is on the primary font name as presented to users; the copyright notice
    # (ID 0) that states the reservation must itself travel unchanged.
    reserved_words_chirho = reserved_chirho.group(2).strip().lower()
    for record_chirho in font_chirho["name"].names:
        if record_chirho.nameID in RENAMED_IDS_CHIRHO and reserved_words_chirho in str(record_chirho).lower():
            raise SystemExit(f"refusing: name ID {record_chirho.nameID} still carries the reserved name")

if arguments_chirho.kern_pairs_chirho:
    if "kern" not in font_chirho:
        raise SystemExit("refusing: --kern-pair edits a legacy kern table, and this face has none")
    cmap_chirho = font_chirho.getBestCmap()
    table_chirho = font_chirho["kern"].kernTables[0].kernTable
    for pair_chirho in arguments_chirho.kern_pairs_chirho:
        left_chirho, right_chirho, value_chirho = pair_chirho.split(":")
        key_chirho = (cmap_chirho[ord(left_chirho)], cmap_chirho[ord(right_chirho)])
        print(f"kern {left_chirho}{right_chirho}: {table_chirho.get(key_chirho)} -> {int(value_chirho)}")
        table_chirho[key_chirho] = int(value_chirho)

for kept_chirho in (0, 13, 14):
    if not font_chirho["name"].getDebugName(kept_chirho):
        raise SystemExit(f"refusing: name ID {kept_chirho} (licence notice) did not survive the subset")
font_chirho.save(arguments_chirho.out_chirho)
print(f"{arguments_chirho.out_chirho}: {os.path.getsize(arguments_chirho.out_chirho)} bytes, "
      f"family '{font_chirho['name'].getDebugName(1)}', copyright kept: {font_chirho['name'].getDebugName(0)[:60]}")
