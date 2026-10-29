// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/signup-chirho.md.
// The real product guest, public assets and tenant auth behind one owned loopback origin.
import { readFile } from "node:fs/promises";
import { join } from "node:path";
import { SignupFixtureChirho, type AuthRendererChirho } from "./signup-fixture-chirho";
import type { SourceReviewChirho } from "./reviewed-deployment-chirho";

export class SignupBrowserFixtureChirho extends SignupFixtureChirho {
  readonly browser_origin_chirho = `http://127.0.0.1:${this.port_chirho + 2}`;
  readonly browser_workspace_chirho = `${this.browser_origin_chirho}/app-chirho/${this.app_chirho}/`;
  renderer_chirho?: AuthRendererChirho;
  private readonly processes_chirho: ReturnType<typeof Bun.spawn>[] = [];

  async initialize_browser_chirho(guest_chirho: string): Promise<void> {
    if (this.port_chirho > 65531) throw Error("Signup browser check needs four consecutive loopback ports");
    for (let offset_chirho = 1; offset_chirho < 4; offset_chirho++) {
      const socket_chirho = Bun.listen({ hostname: "127.0.0.1", port: this.port_chirho + offset_chirho, socket: { data() {} } });
      socket_chirho.stop(true);
    }
    await this.initialize_signup_chirho();
    await this.capture_chirho(guest_chirho, "auth-display-guest-chirho");
    const digest_chirho = this.run_chirho(["cas-add", guest_chirho], true);
    this.check_chirho(/^[a-f0-9]{64}$/.test(digest_chirho), "auth guest is addressed in the tenant CAS");
    this.renderer_chirho = { digest_chirho, entry_chirho: "render_auth_chirho", fuel_chirho: 1_000_000,
      max_output_bytes_chirho: 256 * 1024, max_memory_bytes_chirho: 16 * 1024 * 1024 };

    await this.promote_public_app_chirho(this.product_chirho, "perffection");
    await this.promote_public_app_chirho(join(this.product_chirho, "apps-chirho/perffection-page-template-chirho"), "perffection-page-template-chirho");
    await this.configure_publication_chirho();
    this.check_chirho(this.users_chirho.length === 0, "publication provisioning creates no operator-issued customer");

    await this.start_ingress_chirho();
  }

  private async promote_public_app_chirho(source_chirho: string, app_chirho: string): Promise<void> {
    const manifest_chirho = await this.stage_app_chirho(source_chirho, app_chirho);
    const owner_chirho = this.run_chirho(["identity-register", "--kind", "owner"]).split("\t")[0];
    this.private_values_chirho.push(owner_chirho);
    const review_chirho: SourceReviewChirho = JSON.parse(this.run_chirho(["app-source-review", "--manifest", manifest_chirho, "--owner", owner_chirho, "--json"]));
    const candidate_chirho = review_chirho.candidate_chirho;
    this.check_chirho(review_chirho.non_authoritative_chirho && /^[a-f0-9]{64}$/.test(review_chirho.source_review_report_chirho)
      && candidate_chirho.app_chirho === app_chirho && candidate_chirho.owner_chirho === owner_chirho
      && candidate_chirho.request_access_policy_chirho?.kind_chirho === "public-chirho", `${app_chirho}: review binds the owner, app and public display policy`);
    for (const route_chirho of [{ route_chirho: "", dag_chirho: candidate_chirho.root_dag_chirho }, ...candidate_chirho.routes_chirho]) {
      const diff_chirho: { candidate_cost_token_chirho: number; added_capabilities_chirho: string[]; warnings_chirho: string[] } = JSON.parse(
        this.run_chirho(["review-diff", "--app", app_chirho, "--candidate", route_chirho.dag_chirho, "--json"]));
      this.check_chirho(diff_chirho.candidate_cost_token_chirho === 0 && diff_chirho.added_capabilities_chirho.length === 0
        && diff_chirho.warnings_chirho.length === 0, `${app_chirho}: public route is effect-free: ${route_chirho.route_chirho}`);
    }
    this.run_chirho(["review-promote", "--app", app_chirho, "--owner", owner_chirho,
      "--candidate", candidate_chirho.root_dag_chirho, "--source-review-report", review_chirho.source_review_report_chirho]);
    this.run_chirho(["app-verify"]);
    this.run_chirho(["tenant-app-verify"], true);
    this.candidates_chirho.push({ review_chirho, tenant_chirho: false });
  }

  private async start_ingress_chirho(): Promise<void> {
    // Derive the product-local fixture; change only ports and the explicit signup entry.
    // In particular there is NO canonical-login GET rewrite to the issued-principal app page.
    const ingress_chirho = join(this.product_chirho, "tools-chirho/local-runtime-chirho/Caddyfile-chirho");
    await this.capture_chirho(ingress_chirho, "base-ingress-chirho");
    let config_chirho = await readFile(ingress_chirho, "utf8");
    for (const [needle_chirho, replacement_chirho] of [
      ["\nhttp://127.0.0.1:8787 {", `\n${this.browser_origin_chirho} {`],
      ["reverse_proxy 127.0.0.1:8788", `reverse_proxy 127.0.0.1:${this.port_chirho}`],
      ["reverse_proxy 127.0.0.1:8789", `reverse_proxy 127.0.0.1:${this.port_chirho + 1}`],
      ["redir @bare_chirho /app-chirho/perffection/login-chirho 302", "redir @bare_chirho /app-chirho/perffection/ 302"],
      ["redir @login_get_chirho /app-chirho/perffection/login-chirho{?query} 302", "# The engine owns signup/login display in this explicit local-only test."],
    ]) {
      if (config_chirho.split(needle_chirho).length !== 2) throw Error("Signup ingress replacement is not unique");
      config_chirho = config_chirho.replace(needle_chirho, replacement_chirho);
    }
    const config_path_chirho = join(this.scratch_chirho, "Caddyfile-signup-chirho");
    await Bun.write(config_path_chirho, `# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{\n admin 127.0.0.1:${this.port_chirho + 3}\n auto_https off\n persist_config off\n storage file_system {\n root ${this.scratch_chirho}/caddy-storage-chirho\n }\n}\n${config_chirho}`);
    this.command_chirho(["caddy", "fmt", "--overwrite", config_path_chirho], "format owned signup ingress");
    this.command_chirho(["caddy", "validate", "--adapter", "caddyfile", "--config", config_path_chirho], "validate owned signup ingress", true, true);
    await this.capture_chirho(config_path_chirho, "local-signup-ingress-chirho");
    const caddy_chirho = Bun.which("caddy");
    if (!caddy_chirho) throw Error("Installed Caddy is required; this check does not install it");
    await this.capture_chirho(caddy_chirho, "caddy-cli-chirho");
    this.spawn_chirho("public-chirho", [this.cli_chirho, ...this.flags_chirho(), "serve", "--port", String(this.port_chirho + 1), "--allow-platform-root-chirho"]);
    this.spawn_chirho("edge-chirho", [caddy_chirho, "run", "--adapter", "caddyfile", "--config", config_path_chirho]);
  }

  async start_browser_chirho(open_chirho: boolean, ceiling_chirho: number): Promise<void> {
    if (!this.renderer_chirho) throw Error("Auth guest has not been configured");
    await this.start_chirho(open_chirho, ceiling_chirho, this.renderer_chirho);
    const deadline_chirho = Date.now() + 30_000;
    while (Date.now() < deadline_chirho) {
      if (this.processes_chirho.some(process_chirho => process_chirho.exitCode !== null)) throw Error("Owned public/edge service exited");
      try {
        const checks_chirho = await Promise.all([
          `${this.browser_origin_chirho}/healthz-chirho`,
          `${this.browser_origin_chirho}/app-assets-chirho/perffection/by-name-chirho/login-chirho.css`,
        ].map(url_chirho => fetch(url_chirho, { signal: AbortSignal.timeout(1000) })));
        if (checks_chirho.every(response_chirho => response_chirho.status === 200)) return;
      } catch { /* Bounded startup wait, not a retry of any customer operation. */ }
      await Bun.sleep(100);
    }
    throw Error("Owned composed signup service did not become ready");
  }

  private spawn_chirho(name_chirho: string, arguments_chirho: string[]): void {
    this.processes_chirho.push(Bun.spawn(arguments_chirho, { cwd: this.product_chirho,
      stdout: Bun.file(join(this.scratch_chirho, `${name_chirho}-stdout-chirho.log`)),
      stderr: Bun.file(join(this.scratch_chirho, `${name_chirho}-stderr-chirho.log`)) }));
  }

  async stop_browser_chirho(): Promise<void> {
    await this.stop_chirho();
    for (const child_chirho of this.processes_chirho.reverse()) {
      if (child_chirho.exitCode !== null) continue;
      child_chirho.kill("SIGTERM");
      await Promise.race([child_chirho.exited, Bun.sleep(2000)]);
      if (child_chirho.exitCode === null) { child_chirho.kill("SIGKILL"); await child_chirho.exited; }
    }
  }
}
