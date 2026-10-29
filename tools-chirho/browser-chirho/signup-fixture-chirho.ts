// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// Local synthetic auth consumer; workflow: signup-chirho.md. Never issues a User through the CLI.
import { mkdir, readFile, readdir, stat } from "node:fs/promises";
import { join } from "node:path";
import { freeze_cairn_candidate_chirho, resolve_cairn_runtime_chirho } from "./cairn-runtime-chirho";
import { ReviewedDeploymentChirho, type SourceReviewChirho } from "./reviewed-deployment-chirho";
import { write_private_chirho } from "../browser-flows-chirho/private-file-chirho";

export interface AuthRendererChirho {
  digest_chirho: string;
  entry_chirho: string;
  fuel_chirho: number;
  max_output_bytes_chirho: number;
  max_memory_bytes_chirho: number;
}

export class SignupFixtureChirho extends ReviewedDeploymentChirho {
  readonly app_chirho = "perffection-workspace";
  readonly origin_chirho: string;
  readonly workspace_chirho: string;
  private child_chirho?: ReturnType<typeof Bun.spawn>;
  private config_sequence_chirho = 0;
  last_scan_chirho = { files_chirho: 0, bytes_chirho: 0 };

  constructor(product_chirho: string, engine_chirho: string, scratch_chirho: string, readonly port_chirho: number) {
    super(product_chirho, engine_chirho, scratch_chirho);
    if (!Number.isInteger(port_chirho) || port_chirho < 1024 || port_chirho > 65535) throw Error("Invalid SIGNUP_QA_PORT_CHIRHO");
    this.origin_chirho = `http://127.0.0.1:${port_chirho}`;
    this.workspace_chirho = `${this.origin_chirho}/app-chirho/${this.app_chirho}/`;
  }

  async initialize_signup_chirho(): Promise<void> {
    // A candidate is a deliberately named integration input, not a change to the product pin.
    // Freeze the handed-off executable. Unconsumed engine source may keep moving independently;
    // only the converter we execute must match that commit. There is no compilation from this tree.
    const expected_commit_chirho = process.env.SIGNUP_QA_ENGINE_COMMIT_CHIRHO;
    if (expected_commit_chirho) {
      if (!/^[a-f0-9]{40}$/.test(expected_commit_chirho) || !process.env.CAIRN_CLI_CHIRHO) throw Error("A candidate needs a full commit and explicit CAIRN_CLI_CHIRHO");
      const relative_stager_chirho = "tools-chirho/app-deploy-chirho/host_ready_app_manifest_chirho.py";
      this.runtime_selection_chirho = await freeze_cairn_candidate_chirho({ source_cli_chirho: process.env.CAIRN_CLI_CHIRHO,
        engine_chirho: this.engine_chirho, commit_chirho: expected_commit_chirho,
        destination_chirho: join(this.scratch_chirho, "cairn-signup-candidate-chirho"), helpers_chirho: [relative_stager_chirho] });
      this.cli_chirho = this.runtime_selection_chirho.cli_chirho;
      this.check_chirho(await this.hash_chirho(this.cli_chirho) === this.runtime_selection_chirho.sha256_chirho, "frozen candidate bytes equal the handed-off executable");
      this.check_chirho(this.runtime_selection_chirho.commit_chirho === expected_commit_chirho, "candidate executable names the requested commit");
      this.check_chirho(await this.hash_chirho(join(this.engine_chirho, relative_stager_chirho)) === this.runtime_selection_chirho.helper_hashes_chirho[relative_stager_chirho], "executed stager matches the candidate commit exactly");
      this.artifacts_chirho[`candidate-helper/${relative_stager_chirho}`] = this.runtime_selection_chirho.helper_hashes_chirho[relative_stager_chirho];
      this.source_files_chirho[`candidate-helper/${relative_stager_chirho}`] = join(this.engine_chirho, relative_stager_chirho);
    } else {
      this.cli_chirho = resolve_cairn_runtime_chirho(this.product_chirho, this.engine_chirho);
    }
    await mkdir(this.root_chirho, { mode: 0o700 });
    await this.capture_chirho(this.cli_chirho, "cairn-cli-chirho");
    const socket_chirho = Bun.listen({ hostname: "127.0.0.1", port: this.port_chirho, socket: { data() {} } });
    socket_chirho.stop(true);

    const source_chirho = join(this.product_chirho, "apps-chirho/perffection-workspace-chirho");
    const manifest_chirho = await this.stage_app_chirho(source_chirho, this.app_chirho);
    const owner_chirho = this.run_chirho(["identity-register", "--kind", "owner"], true).split("\t")[0];
    this.private_values_chirho.push(owner_chirho);
    const review_chirho: SourceReviewChirho = JSON.parse(this.run_chirho(["app-source-review", "--manifest", manifest_chirho, "--owner", owner_chirho, "--json"], true));
    const candidate_chirho = review_chirho.candidate_chirho;
    this.check_chirho(review_chirho.non_authoritative_chirho && /^[a-f0-9]{64}$/.test(review_chirho.source_review_report_chirho), "workspace review names a non-authoritative report");
    this.check_chirho(candidate_chirho.app_chirho === this.app_chirho && candidate_chirho.owner_chirho === owner_chirho
      && candidate_chirho.request_access_policy_chirho?.kind_chirho === "authenticated-chirho", "workspace review binds owner, app and authenticated policy");
    const allowed_chirho = new Set([
      "ReadStoreChirho(prefix=viewer-state-chirho/)", "WriteStoreChirho(prefix=viewer-state-chirho/)",
      "ReadStoreChirho(prefix=publication-chirho/)", "TemplateAppDeployChirho(template=perffection-page-template-chirho)",
    ]);
    for (const route_chirho of [{ route_chirho: "", dag_chirho: candidate_chirho.root_dag_chirho }, ...candidate_chirho.routes_chirho]) {
      const diff_chirho: { candidate_cost_token_chirho: number; added_capabilities_chirho: string[]; warnings_chirho: string[] } = JSON.parse(this.run_chirho(["review-diff", "--app", this.app_chirho, "--candidate", route_chirho.dag_chirho, "--json"], true));
      this.check_chirho(diff_chirho.candidate_cost_token_chirho === 0 && diff_chirho.warnings_chirho.length === 0
        && diff_chirho.added_capabilities_chirho.every(capability_chirho => allowed_chirho.has(capability_chirho)), `reviewed workspace route has only expected capabilities: ${route_chirho.route_chirho}`);
    }
    this.run_chirho(["review-promote", "--app", this.app_chirho, "--owner", owner_chirho, "--candidate", candidate_chirho.root_dag_chirho,
      "--source-review-report", review_chirho.source_review_report_chirho, "--accept-capability-escalation"], true);
    this.run_chirho(["tenant-app-verify"], true);
    this.run_chirho(["app-verify"], true);
    this.candidates_chirho.push({ review_chirho, tenant_chirho: true });
  }

  async start_chirho(open_chirho: boolean, ceiling_chirho: number | undefined, renderer_chirho?: AuthRendererChirho): Promise<void> {
    await this.stop_chirho();
    const sequence_chirho = this.config_sequence_chirho++;
    const config_chirho = join(this.scratch_chirho, `signup-${sequence_chirho}-chirho.toml`);
    // This config exists only inside the owned scratch directory; never a shipped deployment file.
    const bytes_chirho = [
      `root_chirho = ${JSON.stringify(this.root_chirho)}`,
      `tenant_chirho = ${JSON.stringify(this.tenant_chirho)}`,
      `port_chirho = ${this.port_chirho}`,
      'registration_token_chirho = ""', 'publish_token_chirho = ""',
      `public_registration_open_chirho = ${open_chirho}`,
      ...(renderer_chirho ? [
        `auth_page_renderer_digest_chirho = ${JSON.stringify(renderer_chirho.digest_chirho)}`,
        `auth_page_renderer_entry_chirho = ${JSON.stringify(renderer_chirho.entry_chirho)}`,
        `auth_page_renderer_fuel_chirho = ${renderer_chirho.fuel_chirho}`,
        `auth_page_renderer_max_output_bytes_chirho = ${renderer_chirho.max_output_bytes_chirho}`,
        `auth_page_renderer_max_memory_bytes_chirho = ${renderer_chirho.max_memory_bytes_chirho}`,
      ] : []),
      ...(ceiling_chirho === undefined ? [] : [`user_identity_ceiling_chirho = ${ceiling_chirho}`]), "",
    ].join("\n");
    await write_private_chirho(config_chirho, bytes_chirho);
    const digest_chirho = this.run_chirho(["cas-add", config_chirho]);
    if (!/^[a-f0-9]{64}$/.test(digest_chirho)) throw Error("Config CAS command did not return a digest");
    const args_chirho = [this.cli_chirho, ...this.flags_chirho(), "up", "--config", config_chirho, "--expected-config-digest-chirho", digest_chirho];
    this.child_chirho = Bun.spawn(args_chirho, { cwd: this.product_chirho,
      stdout: Bun.file(join(this.scratch_chirho, `service-${sequence_chirho}-stdout-chirho.log`)),
      stderr: Bun.file(join(this.scratch_chirho, `service-${sequence_chirho}-stderr-chirho.log`)) });
    const deadline_chirho = Date.now() + 30_000;
    while (Date.now() < deadline_chirho) {
      if (this.child_chirho.exitCode !== null) throw Error("Owned signup service refused its configuration or exited");
      try {
        const response_chirho = await fetch(`${this.origin_chirho}/healthz-chirho`, { signal: AbortSignal.timeout(1000) });
        if (response_chirho.status === 200 && this.child_chirho.exitCode === null) return;
      } catch { /* Bounded startup wait on our own producer. */ }
      await Bun.sleep(100);
    }
    throw Error("Owned signup service did not become ready");
  }

  async stop_chirho(): Promise<void> {
    const child_chirho = this.child_chirho;
    this.child_chirho = undefined;
    if (!child_chirho || child_chirho.exitCode !== null) return;
    child_chirho.kill("SIGTERM");
    await Promise.race([child_chirho.exited, Bun.sleep(2000)]);
    if (child_chirho.exitCode === null) {
      child_chirho.kill("SIGKILL");
      await child_chirho.exited;
    }
  }

  async leaked_markers_chirho(markers_chirho: string[]): Promise<boolean> {
    let files_chirho = 0;
    let bytes_chirho = 0;
    const visit_chirho = async (directory_chirho: string): Promise<boolean> => {
      for (const entry_chirho of await readdir(directory_chirho, { withFileTypes: true })) {
        const path_chirho = join(directory_chirho, entry_chirho.name);
        if (entry_chirho.isSymbolicLink()) throw Error("Unexpected symlink in synthetic identity root");
        if (entry_chirho.isDirectory()) {
          if (await visit_chirho(path_chirho)) return true;
        } else if (entry_chirho.isFile()) {
          files_chirho++;
          const size_chirho = (await stat(path_chirho)).size;
          bytes_chirho += size_chirho;
          this.last_scan_chirho = { files_chirho, bytes_chirho };
          // Bound the entire fresh fixture and the largest per-file allocation; this is an offline
          // test scan, never a production hot path. The bound was sized when prepared debug WASM was
          // about 145 MiB; release guests are about 14 MB (2026-09-24). It is a ceiling, not a target,
          // so do not tighten it to the new size.
          if (files_chirho > 5000 || bytes_chirho > 512 * 1024 * 1024 || size_chirho > 64 * 1024 * 1024) throw Error("Synthetic privacy scan exceeded its bounded fixture size");
          const value_chirho = await readFile(path_chirho);
          if (markers_chirho.some(marker_chirho => value_chirho.includes(Buffer.from(marker_chirho)))) return true;
        }
      }
      return false;
    };
    return visit_chirho(this.root_chirho);
  }
}
