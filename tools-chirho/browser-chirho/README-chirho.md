<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV) -->
# Local Browser Consumers Chirho

These are the existing product's local test consumers, imported from pinned source. They drive real Cairn routes and compiled Askama/WASM guests with synthetic local identities. They are not a second application stack or hosted release acceptance.

The maintained entry point is `reviewed-product-check-chirho.ts`. Run it through the pinned `../browser-toolchain-chirho/` package after the clean engine pin is finalized, the parent gives build GO and all three apps are prepared. It stages and reviews actual app sources, promotes reviewed candidates and exercises the browser editor/publication journey. `public-auth-check-chirho.ts`, `account-check-chirho.ts`, `public-help-check-chirho.ts` and `published-page-check-chirho.ts` provide focused local consumers; their script headers and argument validation describe their inputs.

`cairn-runtime-chirho.ts` selects a matching installed CLI or freezes an explicitly delivered candidate with its real clean Git commit and executable SHA-256. The engine helper it binds is the generic prebuilt-manifest converter; ingress is separately captured from `../local-runtime-chirho/Caddyfile-chirho` in this product repository. No old private commit, host file, source-tree mutation or automatic rebuild is substituted for missing compatibility.

The local Caddy fixture contains no hosted addresses, secrets, host log paths or host operational instructions. Tenant/auth routes retain credentials; public display/asset routes strip Cookie and Authorization. The reviewed harness adapts the derived config and checks actual loopback listeners before launching it. These checks need execution in the final clean-clone gates before they establish behavior.

Signup test files remain because the product already has a pure auth-display guest and a local signup-to-publication consumer. They are not part of the initial closed-registration gate and must not activate a public endpoint. Their synthetic fixtures can explicitly open local test registration. No source import proves current signup acceptance, action-scope enforcement, provider delivery, money movement, real customer recovery or hosted HTTPS behavior.

Prior private browser captures, receipts, host packets and historical check totals are not copied and are not acceptance evidence for this repository. Current instructions and proof scope live in [the clean source guide](../../docs-chirho/source-chirho/clone-build-journey-chirho.md).
