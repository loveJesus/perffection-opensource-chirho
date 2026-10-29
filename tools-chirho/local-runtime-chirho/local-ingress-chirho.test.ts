// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { derive_beta_edge_chirho, require_loopback_listeners_chirho } from "../browser-chirho/reviewed-deployment-chirho";

// Workflow: docs-chirho/source-chirho/clone-build-journey-chirho.md.
const fixture_chirho = readFileSync(join(import.meta.dir, "Caddyfile-chirho"), "utf8");

test("local fixture preserves tenant credentials and strips them only for public displays", () => {
  const config_chirho = derive_beta_edge_chirho({ source_chirho: fixture_chirho, edge_port_chirho: 18840,
    public_port_chirho: 18841, tenant_port_chirho: 18842, admin_port_chirho: 18843,
    storage_chirho: "/tmp/synthetic-storage-chirho", access_log_chirho: "/tmp/unused-log-chirho" });
  expect(config_chirho).toContain("http://127.0.0.1:18840 {");
  expect(config_chirho).toContain("admin 127.0.0.1:18843");
  expect(config_chirho).toContain("reverse_proxy 127.0.0.1:18842");
  expect(config_chirho).toContain("reverse_proxy 127.0.0.1:18841");
  const tenant_chirho = config_chirho.slice(config_chirho.indexOf("handle @tenant_chirho"), config_chirho.indexOf("handle @public_chirho"));
  expect(tenant_chirho).not.toContain("header_up -");
  expect(config_chirho.slice(config_chirho.indexOf("handle @public_chirho"))).toContain("header_up -Authorization");
  expect(config_chirho).not.toContain("perffection.com");
  expect(config_chirho).not.toContain("output file");
});

test("listener evidence refuses an unintended public bind", () => {
  const adapted_chirho = { admin: { listen: "127.0.0.1:18843" }, apps: { http: { servers: { local_chirho: { listen: [":18840"] } } } } };
  expect(() => require_loopback_listeners_chirho(adapted_chirho, ["127.0.0.1:18840"], "127.0.0.1:18843")).toThrow();
  adapted_chirho.apps.http.servers.local_chirho.listen = ["127.0.0.1:18840"];
  expect(require_loopback_listeners_chirho(adapted_chirho, ["127.0.0.1:18840"], "127.0.0.1:18843")).toContain("127.0.0.1:18840");
});
