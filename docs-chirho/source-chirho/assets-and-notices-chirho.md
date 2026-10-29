<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV) -->
# Assets And Notices Chirho

All 31 authored files under `assets_chirho/` came from product commit `ee4dd25c8fb95b2f60a5693be02c61d0285636a8`, not Downloads, runtime state or generated build directories. Exact original Git blobs and source/prepared SHA-256 are recorded per file in `provenance-chirho/`. Authored artwork and fonts are intentionally retained so the real existing site is present. Generated CSS, WASM and emitted manifests are excluded and must be built from source.

## Fonts

Seven WOFF2 files cover six families. Original `docs-chirho/licenses-chirho/*-OFL-chirho.txt` notices and the original license README are retained unchanged. `.gitattributes` retains the upstream-license whitespace exception; no legal text was reformatted.

| Files in assets_chirho | Origin family and retained notice | Preparation evidence and hold |
| --- | --- | --- |
| `archivo-grotesk-chirho.woff2` | Archivo; `archivo-OFL-chirho.txt`, copyright 2020 Archivo Project Authors | Source records Google Fonts origin and subsetting; exact font bytes inventoried |
| `bodoni-display-chirho.woff2` | Bodoni Moda; `bodoni-OFL-chirho.txt`, copyright 2020 Bodoni Moda Project Authors | Source records Google Fonts origin and subsetting; exact font bytes inventoried |
| `cabin-text-chirho.woff2` | Cabin; `cabin-OFL-chirho.txt`, copyright 2018 Cabin Project Authors | Source records width pinned to 100 and name/notice preservation |
| `fell-canon-chirho.woff2`, `fell-canon-italic-chirho.woff2` | IM FELL French Canon; `im-fell-french-canon-OFL-chirho.txt`, copyright 2010 Igino Marini | Embedded 2007 notice/RFN retained; primary family/PostScript names independently decoded as Perffection Canon Chirho |
| `fraunces-display-chirho.woff2` | Fraunces; `fraunces-OFL-chirho.txt`, copyright 2018 Fraunces Project Authors | Source records embedded 2020 vs notice 2018 difference; both source notices retained |
| `jetbrains-mono-chirho.woff2` | JetBrains Mono; `jetbrains-mono-OFL-chirho.txt`, copyright 2020 JetBrains Mono Project Authors | Source records Google Fonts origin and subsetting; exact font bytes inventoried |

The imported license README documents SIL OFL 1.1, upstream sources, modifications, reserved-name handling and prior notice comparisons. On 2026-10-29 an offline FontTools inspection independently decoded all seven current WOFF2 name tables and checked all six retained notices against the pinned preparation objects. Embedded copyright and OFL descriptions remain present. The modified IM FELL primary names are Perffection Canon Chirho; the original copyright/reserved-name notices remain intact. Fraunces and IM FELL retain their documented differing embedded/upstream notice years. No font bytes or upstream legal notices were changed during this inspection. The existing font-subsetting script is retained for provenance. This checks the shipped metadata and preserved notices, not an exact upstream-binary-revision comparison or a general legal certification.

## Raster Inventory

| Files in assets_chirho | Available origin evidence | Public-release status |
| --- | --- | --- |
| `bakehouse-chirho.jpg`, `ceramics-chirho.jpg`, `florist-chirho.jpg` | Pinned website guide records GPT Image 2 project key art, resized JPEGs | HOLD: exact generation receipts, input rights and redistribution terms not in clean source |
| `town-dawn-chirho.jpg`, `town-night-chirho.jpg`, `town-morning-chirho.jpg` and their `-small-chirho.jpg` variants | Pinned guide and retained art-direction/town prompts describe generated dawn and matching scene edits | HOLD: provider/model attribution alone is not rights clearance |
| `town-wake-a-chirho.jpg`, `town-wake-b-chirho.jpg` | Pinned guide describes a generated night-to-dawn video workflow; exact frame mapping is not established here | HOLD: per-file source/derivative mapping and input rights need evidence |
| `door-night-chirho.jpg`, `door-night-small-chirho.jpg`, `shop-inside-chirho.jpg`, `shop-inside-small-chirho.jpg` | Retained project prompts and pinned guide describe project image generation/edit pipeline | HOLD: exact selected generation and resize mappings not established here |
| `easy-phone-chirho.jpg`, `easy-question-chirho.jpg`, `easy-result-chirho.jpg` | Pinned guide attributes editor illustrations to an invented bakery rendered locally | HOLD: exact screenshot source, visible content/rights and derivative mapping not independently checked |
| `seal-chirho.png`, `seal-sheet-chirho.jpg`, `seal-sheet-small-chirho.jpg` | Existing `blender-seal-chirho.py` and `sprite-sheet-chirho.sh`; pinned guide describes rendered crystal/sprite pipeline | HOLD: exact render-to-selected-output mapping not rerun or independently established |
| `stars-far-chirho.png`, `stars-near-chirho.png` | Existing seeded `starfield-chirho.py`, guide attributes stars to it | HOLD until exact regeneration/output attribution is checked; no rerender was run |
| `tooth-chirho.png` | Exact committed bytes only; no reliable per-file production/rights record found in bounded source inspection | HOLD: unsupported provenance |

No provider request, image generation, paid API call or asset replacement was performed. The private website guide and spend/evidence directories were inspected only as committed provenance references and were not copied. The source guide's broad statement that generated photographs are "not licensed from anyone" is not adopted as legal clearance. Rights, generated-input provenance, model terms at generation and final public-license/patent compatibility remain deliberate release gates. Keeping these assets in a private preparation is not permission to publish them.

## Code Notices

Project-owned product code uses the proposed AGPL terms. John 3:16 source comments and all third-party font notices remain. No top-level public license/patent grant or historical creator attribution has been invented. The parent separately owns release licensing and security review. Dependency licenses are a separate lockfile/source audit, not established by this asset inventory.
