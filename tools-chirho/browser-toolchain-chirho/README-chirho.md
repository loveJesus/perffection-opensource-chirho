<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV) -->
# Browser Development Tools Chirho

This pinned development-only Bun package drives installed Chrome against the existing compiled Rust/WASM product. It is not a JavaScript product runtime, server, frontend or dependency of ordinary Rust/WASM compilation.

After the parent build GO and app preparation, from this directory:

```sh
bun install --frozen-lockfile --ignore-scripts
bun run check-chirho
bun run test-chirho
bun run reviewed-chirho
```

`test-chirho` covers local runtime selection and the customer-run MCP adapter. The strict TypeScript project includes the maintained local browser consumers and local-ingress tests, not excluded host/invitation/provider tools. The lockfile and original exact development versions are preserved; no dependency upgrade or browser download was performed during source preparation.

The reviewed journey uses the product-owned loopback Caddy fixture. Build identity uses the actual clean engine commit, not an old private source hash. Runtime consumers never rebuild apps/Cairn or install packages to resolve missing inputs. Explicit runtime/browser overrides must follow each consumer's supported options; do not use Bun's disposable download cache as a dependency path.

Signup test consumers remain source-only and separately gated. No command here authorizes public signup, hosts, providers or money. See [source build and journey guide](../../docs-chirho/source-chirho/clone-build-journey-chirho.md).
