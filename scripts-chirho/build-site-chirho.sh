#!/usr/bin/env bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
#
# Compatibility entry point for the historical release rail: compile the public site's generated
# CSS and check its Cairn tree. The fast product command owns WASM builds and manifest emission.

set -euo pipefail

SCRIPT_ROOT_CHIRHO="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec "$SCRIPT_ROOT_CHIRHO/prepare-apps-chirho.sh" --app-chirho site --styles-only-chirho
