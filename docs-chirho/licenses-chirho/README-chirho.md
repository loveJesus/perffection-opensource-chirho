<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Third-party font provenance and licences

This site redistributes six typefaces as app assets. IM FELL French Canon and Cabin were added on
2026-10-01 for the FIRST LIGHT design, replacing Instrument Serif and Geist (L.J. read the first pair
as "boilerplate AI"). The other four remain for the nine design studies on the front door. Inter,
the old pages' text face, retired the same day, when the last page moved to FIRST LIGHT. The SIL Open Font License 1.1 requires that
the licence text and copyright notice accompany the redistributed font software, so both are kept
here, verbatim, one file per family.

These `.txt` files are **documentation, not app assets.** They are deliberately not declared in
`app-chirho.toml`: the engine's asset content-type set is closed to CSS, PNG, JPEG, PDF, WOFF and
WOFF2, and a licence obligation is met by shipping the text with the software, not by serving it
from a public route.

## What is redistributed

| Asset | Family | Version (name ID 5) | Licence | Embedded notice (name ID 0) | Licence text |
| --- | --- | --- | --- | --- | --- |
| `assets_chirho/fraunces-display-chirho.woff2` | Fraunces | `1.000; [b76b70a41]` | SIL OFL 1.1 | Copyright 2020 The Fraunces Project Authors | `fraunces-OFL-chirho.txt` |
| `assets_chirho/jetbrains-mono-chirho.woff2` | JetBrains Mono | `2.211` | SIL OFL 1.1 | Copyright 2020 The JetBrains Mono Project Authors | `jetbrains-mono-OFL-chirho.txt` |
| `assets_chirho/bodoni-display-chirho.woff2` | Bodoni Moda | `2.005` | SIL OFL 1.1 | Copyright 2020 The Bodoni Moda Project Authors | `bodoni-OFL-chirho.txt` |
| `assets_chirho/archivo-grotesk-chirho.woff2` | Archivo | `2.001` | SIL OFL 1.1 | Copyright 2020 The Archivo Project Authors | `archivo-OFL-chirho.txt` |
| `assets_chirho/fell-canon-chirho.woff2` and `fell-canon-italic-chirho.woff2` | IM FELL French Canon (renamed inside to "Perffection Canon Chirho", see below) | `3.00` | SIL OFL 1.1 | © 2007 Igino Marini, With Reserved Font Name IM FELL French Canon Roman / Italic | `im-fell-french-canon-OFL-chirho.txt` |
| `assets_chirho/cabin-text-chirho.woff2` | Cabin | `Version 3.001` | SIL OFL 1.1 | Copyright 2018 The Cabin Project Authors | `cabin-OFL-chirho.txt` |

Upstream source for all seven: `https://github.com/google/fonts/tree/main/ofl/<family>`.
Each `*-OFL-chirho.txt` above is that family's own OFL text. The original three licence files were
also compared byte for byte with their canonical Google Fonts copies.

That is deliberate and it is checked. The upstream files carry trailing whitespace on a wrapped line
(line 21, `...bundled, embedded, `), which trips `git --check`. It is present in the canonical
`google/fonts` copies and was not introduced here — verified by comparing our bytes against upstream.
Rather than edit a legal text to silence a style warning, `.gitattributes` marks this directory
`-whitespace linguist-vendored`. The licence stays verbatim, and "copied byte for byte" stays true.

## Embedded and licence-file copyright notices

An OFL family carries its copyright notice in **two** places. For two of these five families the
places disagree. Both are recorded, separately, rather than one being reported as the other.

**As embedded in the shipped WOFF2 binaries** (`name` table ID 0, read from the exact files in
`assets_chirho/`):

```
Fraunces        Copyright 2020 The Fraunces Project Authors (github.com/undercasetype/Fraunces)
JetBrains Mono  Copyright 2020 The JetBrains Mono Project Authors (https://github.com/JetBrains/JetBrainsMono)
Bodoni Moda     Copyright 2020 The Bodoni Moda Project Authors (https://github.com/indestructible-type/Bodoni)
Archivo         Copyright 2020 The Archivo Project Authors (https://github.com/Omnibus-Type/Archivo)
IM FELL         © 2007 Igino Marini (www.iginomarini.com) With Reserved Font Name IM FELL French Canon Roman
                (the italic: ... IM FELL French Canon Italic)
Cabin           Copyright 2018 The Cabin Project Authors (https://github.com/impallari/Cabin)
```

**As stated in each upstream `OFL.txt`** (the files copied into this directory):

```
Fraunces        Copyright 2018 The Fraunces Project Authors (https://github.com/undercasetype/Fraunces)
JetBrains Mono  Copyright 2020 The JetBrains Mono Project Authors (https://github.com/JetBrains/JetBrainsMono)
Bodoni Moda     Copyright 2020 The Bodoni Moda Project Authors (https://github.com/indestructible-type/Bodoni)
Archivo         Copyright 2020 The Archivo Project Authors (https://github.com/Omnibus-Type/Archivo)
IM FELL         Copyright (c) 2010, Igino Marini (mail@iginomarini.com)
Cabin           Copyright 2018 The Cabin Project Authors (https://github.com/impallari/Cabin.git)
```

Fraunces reads 2020 in the binary and 2018 in the licence file (Inter, retired 2026-10-01, read 2016
and 2020). This is an upstream inconsistency in the Google Fonts distribution, not
something introduced here — subsetting preserved name ID 0 byte-for-byte. Both notices ship: the
embedded one travels inside the font software, and the upstream one is in this directory.

## Reserved Font Names — checked in both places; one family has one, and is renamed

The OFL restricts *modified* versions from carrying a Reserved Font Name, and subsetting is a
modification, so this had to be checked rather than assumed.

- **IM FELL reserves its name.** Its embedded `name` ID 0 reads "With Reserved Font Name IM FELL
  French Canon Roman" (and "... Italic"). Our subsets are therefore renamed inside: name IDs 1, 3, 4,
  6, 16 and 17 say "Perffection Canon Chirho" (PostScript `PerffectionCanonChirho-Regular` and
  `-Italic`). Name ID 0, which states the reservation, ships unchanged, as the licence requires.
  `tools-chirho/site-art-chirho/subset-font-chirho.py` refuses to subset a face that reserves its
  name without `--rename`, and refuses to write a file whose primary names still carry it; the first
  refusal was run against the IM FELL source as a control (2026-10-01).
- **No other embedded `name` ID 0 above contains a reservation**, and no other `OFL.txt` copyright
  line does either. Those subsets keep their original internal family names legitimately.

(The CSS family names, such as `"Fell Canon Chirho"`, `"Cabin Chirho"`, `"Fraunces Chirho"`,
`"JetBrains Mono Chirho"`, `"Bodoni Chirho"` and `"Archivo Chirho"`, are our own `@font-face`
aliases in `styles_chirho/first-light-chirho/type-chirho.scss` and `bench-frame-chirho.scss`. They are
stylesheet identifiers, not font-internal names.)

## What was done to them

Each upstream TTF was subset and re-flavoured to WOFF2 with fontTools. The FIRST LIGHT faces go
through `tools-chirho/site-art-chirho/subset-font-chirho.py`, which keeps every name record, renames a
face whose licence reserves its name, and pins axes the design does not animate:

```sh
python3 subset-font-chirho.py IMFeFCrm28P.ttf fell-canon-chirho.woff2 --rename "Perffection Canon Chirho"
python3 subset-font-chirho.py IMFeFCit28P.ttf fell-canon-italic-chirho.woff2 --rename "Perffection Canon Chirho"
python3 subset-font-chirho.py "Cabin[wdth,wght].ttf" cabin-text-chirho.woff2 --instance wdth=100
```

The earlier faces were cut with `pyftsubset` directly; the Fraunces command is representative:

```sh
UNICODES_CHIRHO='U+0020-007E,U+00A0-00FF,U+0131,U+0152-0153,U+02BB-02BC,U+02C6,U+02DA,U+02DC,
U+2000-206F,U+2074,U+20AC,U+2122,U+2191-2193,U+2196-2199,U+2212,U+2215,U+2261,
U+221E,U+2248,U+2713,U+25A0,U+25CF,U+2605,U+FEFF,U+FFFD'

pyftsubset "Fraunces[SOFT,WONK,opsz,wght].ttf" \
  --output-file=fraunces-display-chirho.woff2 --flavor=woff2 \
  --unicodes="$UNICODES_CHIRHO" --layout-features='kern,liga,calt,ss01,onum,lnum,tnum' \
  --name-IDs='*' --notdef-outline --recalc-bounds
```

`--name-IDs='*'` is not incidental: it is what preserves name IDs 0 (copyright), 13 (licence
description) and 14 (licence URL) inside the shipped files, so the notice travels with the software
as well as sitting in this directory.

Axes retained after subsetting:

| Family | Axes |
| --- | --- |
| Fraunces | `opsz 9–144`, `wght 100–900`, `SOFT 0–100`, `WONK 0–1` |
| JetBrains Mono | `wght 100–800` |
| Bodoni Moda | `opsz 6–96`, `wght 400–900` |
| Archivo | `wdth 62–125`, `wght 100–900` |
| Cabin | `wght 400–700` (width pinned at 100) |
| IM FELL French Canon | static (roman and italic files) |

## Other third-party material

The photographs and the grain tile are generated for this project, not licensed from anyone. See
`perffection-website-guide-chirho.md` for how they were produced.
