<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Public site catalogs Chirho

Edit public display content here; keep page structure in `templates_chirho/pages_chirho/` and typed
shapes in `src/types_chirho/`.

- `en-chirho.json` is the shared English brand, navigation and footer copy.
- `home-en-chirho.json`, `craft-en-chirho.json` and `trust-en-chirho.json` contain the page copy.
- Fields ending in `_href_chirho`, `_image_chirho` or `_key_chirho`, plus showcase URLs and the
  `{declared_asset_count_chirho}` token, are public content configuration. Copy them unchanged when
  starting a translation; they identify reviewed routes, assets, previews or checked facts.

English is the only reviewed release catalog. The RTL pseudolocale exercises layout metadata and
escaping in tests; no request-selected locale transport or translated release is claimed. Login
actions, credentials, sessions and other effect authority never come from these files.
