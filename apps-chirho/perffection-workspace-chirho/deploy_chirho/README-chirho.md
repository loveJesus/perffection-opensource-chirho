<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV) -->
# Workspace Local Policy Inputs Chirho

These are the existing product's declarative authority inputs, not a host deployment runbook. The manifest alone cannot provision the platform's template registry, publication resource policy or operator-controlled publication applier.

The local publication rehearsal stages the prepared page-template/workspace apps, registers the page renderer on the platform root, reserves product slugs, applies `publication-resource-policy-chirho.json` on the tenant and explicitly drives `publication-apply` on synthetic local state. Run only after the parent build GO and app preparation:

```sh
./scripts-chirho/tests-chirho/publication-two-root-rehearsal-chirho.sh
```

The policy meters publication intents, not successful publications or credits. Its finite window and limit are provisional source configuration, not measured customer demand. The historical source documented refusal/retry UX gaps around exhausted publication quotas; this clean-source import does not claim those gaps are resolved. Recheck the actual selected engine before a release.

`figures-routes-chirho.toml`, `held-bindings-chirho.toml`, `key-openings-chirho.toml` and `release-profiles-chirho.toml` retain source route/contract and held-feature boundaries. They are not permission to expose a route, grant a key, invoke a paid provider or enable production features. Synthetic rehearsals may compose held routes only into owned scratch as documented in their scripts. The default app manifest's release holds remain unchanged.

See `docs-chirho/source-chirho/clone-build-journey-chirho.md` at the product root for the source identity contract and current local gate. No host files, timers, endpoints, credentials or private evidence are supplied here.
