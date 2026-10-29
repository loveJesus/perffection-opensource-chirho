# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
"""Two tileable starfield layers (far and near) as RGBA PNGs, for the page's night sky.

The engine admits PNG but not SVG and runs no script, so the sky is CSS gradients plus these tiles,
moved at two speeds by a scroll timeline and faded out as morning comes. Seeded, so a re-run gives
byte-identical files and an asset's hash only changes when this script does.

usage: python3 starfield-chirho.py <out dir>
"""
import sys  # Workflow: spec-chirho/workflows-chirho/first-light-authoring-chirho.md

import numpy as np
from PIL import Image, ImageFilter

OUT_CHIRHO = sys.argv[1]
SIZE_CHIRHO = 768


def layer_chirho(seed_chirho, count_chirho, radius_range_chirho, brightness_range_chirho, glow_chirho):
    rng_chirho = np.random.default_rng(seed_chirho)
    alpha_chirho = np.zeros((SIZE_CHIRHO, SIZE_CHIRHO), dtype=np.float32)
    tint_chirho = np.zeros((SIZE_CHIRHO, SIZE_CHIRHO, 3), dtype=np.float32)
    yy_chirho, xx_chirho = np.mgrid[0:SIZE_CHIRHO, 0:SIZE_CHIRHO]
    # Mostly white, some warm, a few cold: real skies are not one colour.
    palette_chirho = np.array([[1.0, 1.0, 1.0], [1.0, 0.92, 0.78], [0.78, 0.88, 1.0]], dtype=np.float32)
    for _ in range(count_chirho):
        x_chirho, y_chirho = rng_chirho.uniform(0, SIZE_CHIRHO, 2)
        radius_chirho = rng_chirho.uniform(*radius_range_chirho)
        brightness_chirho = rng_chirho.uniform(*brightness_range_chirho) ** 1.8
        color_chirho = palette_chirho[rng_chirho.choice(3, p=[0.7, 0.18, 0.12])]
        # Wrapped distance, so a star near an edge continues on the opposite edge and the tile has no seam.
        dx_chirho = np.minimum(np.abs(xx_chirho - x_chirho), SIZE_CHIRHO - np.abs(xx_chirho - x_chirho))
        dy_chirho = np.minimum(np.abs(yy_chirho - y_chirho), SIZE_CHIRHO - np.abs(yy_chirho - y_chirho))
        star_chirho = brightness_chirho * np.exp(-(dx_chirho ** 2 + dy_chirho ** 2) / (2.0 * radius_chirho ** 2))
        tint_chirho += star_chirho[..., None] * color_chirho
        alpha_chirho += star_chirho
    alpha_chirho = np.clip(alpha_chirho, 0.0, 1.0)
    rgb_chirho = np.where(alpha_chirho[..., None] > 0, tint_chirho / np.maximum(alpha_chirho[..., None], 1e-6), 1.0)
    image_chirho = Image.fromarray(np.dstack([np.clip(rgb_chirho, 0, 1) * 255, alpha_chirho * 255]).astype(np.uint8), "RGBA")
    if glow_chirho:
        halo_chirho = image_chirho.filter(ImageFilter.GaussianBlur(glow_chirho))
        image_chirho = Image.alpha_composite(halo_chirho, image_chirho)
    return image_chirho


layer_chirho(7, 420, (0.45, 0.9), (0.25, 0.85), 0).save(f"{OUT_CHIRHO}/stars-far-chirho.png", optimize=True)
layer_chirho(31, 70, (0.8, 1.5), (0.6, 1.0), 2.4).save(f"{OUT_CHIRHO}/stars-near-chirho.png", optimize=True)
print("STARFIELD-DONE-CHIRHO")
