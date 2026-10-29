#!/usr/bin/env bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
#
# Frames -> one grid JPEG sprite sheet that CSS steps through, by time or by scroll. The engine
# admits only CSS, PNG, JPEG, PDF and WOFF assets and runs no script, so a "video" on the site is
# a sprite sheet: background-position-x stepped once per row, background-position-y stepped once
# over the whole range, both on the same timeline.
#
# usage: sprite-sheet-chirho.sh <frames dir> <frame glob> <out.jpg> <frame px wide> <frame px high> <columns> [quality] [glow]
#   glow: "glow" flattens RGBA frames onto exact black and screens a soft halo of their highlights.
#   PICK_CHIRHO=K (env): use K frames chosen evenly from the matches, first and last included.
#   HOLD_CHIRHO=H (env): repeat the first and the last frame H extra times each. Safari 26.0-26.4
#     misreads scroll progress near 0% and 100%, so a held end frame keeps the ends exact there.
# Budgets (2026-09-30 research): keep each side at or under 8192 px (4096 for phones), and the
# DECODED size (w * h * 4 bytes) under about 64 MB on desktop and 12 MB for phones; past that Chrome
# re-decodes while scrubbing (stutter) or its decoder downsamples (blur). Frame sides in multiples of 16.
# Workflow: spec-chirho/workflows-chirho/first-light-authoring-chirho.md
set -euo pipefail

FRAMES_DIR_CHIRHO="$1"; GLOB_CHIRHO="$2"; OUT_CHIRHO="$3"
WIDTH_CHIRHO="$4"; HEIGHT_CHIRHO="$5"; COLUMNS_CHIRHO="$6"
QUALITY_CHIRHO="${7:-74}"; GLOW_CHIRHO="${8:-}"
PICK_CHIRHO="${PICK_CHIRHO:-0}"; HOLD_CHIRHO="${HOLD_CHIRHO:-0}"

WORK_CHIRHO="$(mktemp -d)"
trap 'rm -rf "$WORK_CHIRHO"' EXIT
shopt -s nullglob
ALL_CHIRHO=("$FRAMES_DIR_CHIRHO"/$GLOB_CHIRHO)
(( ${#ALL_CHIRHO[@]} > 0 )) || { echo "no frames match $FRAMES_DIR_CHIRHO/$GLOB_CHIRHO" >&2; exit 1; }
if (( PICK_CHIRHO > 1 )); then
  PICKED_CHIRHO=()
  for (( i_chirho = 0; i_chirho < PICK_CHIRHO; i_chirho++ )); do
    PICKED_CHIRHO+=("${ALL_CHIRHO[$(( i_chirho * (${#ALL_CHIRHO[@]} - 1) / (PICK_CHIRHO - 1) ))]}")
  done
else
  PICKED_CHIRHO=("${ALL_CHIRHO[@]}")
fi
FRAMES_CHIRHO=()
for (( h_chirho = 0; h_chirho < HOLD_CHIRHO; h_chirho++ )); do FRAMES_CHIRHO+=("${PICKED_CHIRHO[0]}"); done
FRAMES_CHIRHO+=("${PICKED_CHIRHO[@]}")
for (( h_chirho = 0; h_chirho < HOLD_CHIRHO; h_chirho++ )); do FRAMES_CHIRHO+=("${PICKED_CHIRHO[${#PICKED_CHIRHO[@]} - 1]}"); done
COUNT_CHIRHO=${#FRAMES_CHIRHO[@]}
ROWS_CHIRHO=$(( (COUNT_CHIRHO + COLUMNS_CHIRHO - 1) / COLUMNS_CHIRHO ))
SHEET_W_CHIRHO=$(( WIDTH_CHIRHO * COLUMNS_CHIRHO )); SHEET_H_CHIRHO=$(( HEIGHT_CHIRHO * ROWS_CHIRHO ))
if (( SHEET_W_CHIRHO > 8192 || SHEET_H_CHIRHO > 8192 )); then
  echo "refusing: a ${SHEET_W_CHIRHO}x${SHEET_H_CHIRHO} sheet is past 8192 px; use fewer columns, rows or pixels" >&2
  exit 1
fi

INDEX_CHIRHO=0
for FRAME_CHIRHO in "${FRAMES_CHIRHO[@]}"; do
  PREPARED_CHIRHO="$WORK_CHIRHO/$(printf '%04d' "$INDEX_CHIRHO")-chirho.png"
  if [[ "$GLOW_CHIRHO" == glow ]]; then
    magick "$FRAME_CHIRHO" -background black -flatten \
      \( +clone -channel RGB -level 55%,100% -blur 0x14 \) -compose screen -composite \
      -resize "${WIDTH_CHIRHO}x${HEIGHT_CHIRHO}^" -gravity center -extent "${WIDTH_CHIRHO}x${HEIGHT_CHIRHO}" "$PREPARED_CHIRHO"
  else
    magick "$FRAME_CHIRHO" -resize "${WIDTH_CHIRHO}x${HEIGHT_CHIRHO}^" -gravity center \
      -extent "${WIDTH_CHIRHO}x${HEIGHT_CHIRHO}" "$PREPARED_CHIRHO"
  fi
  INDEX_CHIRHO=$(( INDEX_CHIRHO + 1 ))
done

magick montage "$WORK_CHIRHO"/*-chirho.png -tile "${COLUMNS_CHIRHO}x${ROWS_CHIRHO}" -geometry +0+0 -background black \
  "$WORK_CHIRHO/sheet-chirho.png"
magick "$WORK_CHIRHO/sheet-chirho.png" -strip -sampling-factor 4:2:0 -interlace JPEG -quality "$QUALITY_CHIRHO" "$OUT_CHIRHO"
printf 'sheet %s: %d frames, %d columns x %d rows, %dx%d px, %s bytes, %d MB decoded\n' "$OUT_CHIRHO" "$COUNT_CHIRHO" \
  "$COLUMNS_CHIRHO" "$ROWS_CHIRHO" "$SHEET_W_CHIRHO" "$SHEET_H_CHIRHO" "$(stat -f '%z' "$OUT_CHIRHO")" \
  "$(( SHEET_W_CHIRHO * SHEET_H_CHIRHO * 4 / 1048576 ))"
if (( COUNT_CHIRHO != COLUMNS_CHIRHO * ROWS_CHIRHO )); then
  echo "warning: $COUNT_CHIRHO frames leave $(( COLUMNS_CHIRHO * ROWS_CHIRHO - COUNT_CHIRHO )) empty cells; the CSS steps will show black there" >&2
fi
