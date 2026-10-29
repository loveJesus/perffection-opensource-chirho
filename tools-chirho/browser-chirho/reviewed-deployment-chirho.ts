// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// Local synthetic review proof, not an unattended release approver. Workflow: reviewed-product-chirho.md.
// Clean-source journeys use only product-local loopback ingress, never a host deployment packet.
import { existsSync } from "node:fs";
import { mkdir, readFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import { resolve_reviewed_runtime_chirho, type RuntimeSelectionChirho } from "./cairn-runtime-chirho";
import { write_private_chirho } from "../browser-flows-chirho/private-file-chirho";

interface ManifestFilesChirho {
  nodes_chirho: { source_chirho: { kind_chirho: string; path_chirho?: string; artifact_path_chirho?: string; manifest_path_chirho?: string } }[];
  assets_chirho: { source_chirho: { path_chirho?: string } }[];
}

interface ReviewDiffChirho {
  candidate_chirho: string;
  candidate_cost_token_chirho: number;
  added_capabilities_chirho: string[];
  warnings_chirho: string[];
}

export interface CandidateChirho {
  app_chirho: string;
  owner_chirho: string;
  root_dag_chirho: string;
  request_access_policy_chirho?: { kind_chirho: string };
  routes_chirho: { route_chirho: string; dag_chirho: string }[];
  assets_chirho: { name_chirho: string; asset_chirho: string }[];
}

export interface SourceReviewChirho {
  source_review_report_chirho: string;
  non_authoritative_chirho: boolean;
  candidate_chirho: CandidateChirho;
  review_chirho: ReviewDiffChirho;
}

// One executed engine CLI call, with named placeholders for private values and local paths.
export interface TranscriptStepChirho { argv_chirho: string[]; refused_chirho?: string; defines_chirho?: string }

// perffection-page-template-chirho -> PERFFECTION_PAGE_TEMPLATE, for placeholder names.
function placeholder_name_chirho(app_chirho: string): string {
  return app_chirho.replace(/-chirho$/, "").toUpperCase().replaceAll("-", "_");
}

const SHEET_LEGEND_CHIRHO = [
  "| Placeholder | What the host owner supplies |",
  "| --- | --- |",
  "| `cairn-chirho` | the reviewed binary, the same bytes as the running services |",
  "| `$PLATFORM_ROOT_CHIRHO` | the fresh beta platform root; `--tenant beta-chirho` addresses the tenant under it |",
  "| `$RELEASE_CHIRHO` | the verified extraction of the package archive |",
  "| `$OWNER_<APP>_CHIRHO`, `$OTHER_OWNER_<APP>_CHIRHO` | Owner principals that `identity-register --kind owner` prints; private |",
  "| `$REVIEW_REPORT_<APP>_CHIRHO` | the report hash that `app-source-review` prints; it binds the Owner, so the host's differs |",
  "| `$USER_<N>_CHIRHO`, `$USER_<N>_CREDENTIAL_FILE_CHIRHO` | synthetic users and their owner-only credential files; private |",
  "| `$OAUTH_CLIENT_CHIRHO`, `$USER_<N>_BEARER_CHIRHO` | the operator draft-scope client and its tokens, for the agent-API journey; private |",
];

// The R5 cache fence as Cairn's reviewed public site block carries it (#24952, #24959): a site-wide
// missing-header default that keeps the engine's own asset headers, and the same header on Caddy's
// own error responses, which bypass that default.
export const R5_CACHE_FENCE_CHIRHO = [
  'header ?Cache-Control "private, no-store"',
  "handle_errors {",
  '\theader Cache-Control "private, no-store"',
  '\trespond "" {err.status_code}',
  "}",
];

// CADDY_CLI_CHIRHO names an installed local executable explicitly, otherwise use PATH.
export function caddy_cli_chirho(environment_chirho: NodeJS.ProcessEnv = process.env): string {
  const explicit_chirho = environment_chirho.CADDY_CLI_CHIRHO;
  const cli_chirho = explicit_chirho ? resolve(explicit_chirho) : Bun.which("caddy");
  if (!cli_chirho || !existsSync(cli_chirho)) {
    throw Error(explicit_chirho ? `CADDY_CLI_CHIRHO names no file: ${explicit_chirho}` : "Installed Caddy is required; this check does not install it");
  }
  return cli_chirho;
}

// No access log is enabled in the clean-source fixture. Keep stray-log detection fail closed.
export const HOST_ACCESS_LOG_CHIRHO = "output file unused-access-chirho.log";

// The committed edge sites a local edge can rehearse, a closed table (gpt #27086): where each site's Caddyfile lives in
// Cairn, the site addresses it serves in file order, and the host access log it writes. The apex file serves the apex
// and then www, whose redirect to the apex hostname is rehearsed as written.
export interface EdgeSiteChirho { path_chirho: string; addresses_chirho: readonly string[]; host_access_log_chirho: string }
export const EDGE_SITES_CHIRHO = {
  "beta-chirho": { path_chirho: "tools-chirho/local-runtime-chirho/Caddyfile-chirho", addresses_chirho: ["http://127.0.0.1:8787"], host_access_log_chirho: HOST_ACCESS_LOG_CHIRHO },
} as const satisfies Record<string, EdgeSiteChirho>;
export type EdgeSiteNameChirho = keyof typeof EDGE_SITES_CHIRHO;

// The beta site block as a loopback edge, for the stand-in, the tunnel phase and a candidate site
// block. Matchers, route order and header handling stay as the source has them; only the site
// address and the two upstream ports change, plus a loopback admin, local storage and the access
// log's file. The log's filter and rolling are validated as written.
export interface BetaEdgeChirho {
  source_chirho: string;
  edge_port_chirho: number;
  // The public service (8789 on the host) and the tenant service (8788 on the host).
  public_port_chirho: number;
  tenant_port_chirho: number;
  admin_port_chirho: number;
  storage_chirho: string;
  // Where HOST_ACCESS_LOG_CHIRHO's file goes; a source from before R6 has no log and derives none.
  access_log_chirho: string;
  // Adds R5_CACHE_FENCE_CHIRHO to a source without one.
  cache_fence_chirho?: boolean;
}

// Any live Cache-Control rule or error handler already in the source counts as a fence.
export function carries_cache_fence_chirho(source_chirho: string): boolean {
  return source_chirho.split("\n").map(line_chirho => line_chirho.trim()).filter(line_chirho => !line_chirho.startsWith("#"))
    .some(line_chirho => /cache-control/i.test(line_chirho) || /^handle_errors\b/.test(line_chirho));
}

export function derive_beta_edge_chirho(edge_chirho: BetaEdgeChirho): string {
  return derive_site_edge_chirho({ ...edge_chirho, site_chirho: EDGE_SITES_CHIRHO["beta-chirho"], further_ports_chirho: [] });
}

// Any committed site the same way. Each site address becomes its own loopback listener: the first takes the edge's port
// (and any fence), and each further one (the apex's www) its own port. Nothing else in a block changes.
export interface SiteEdgeChirho extends BetaEdgeChirho {
  site_chirho: EdgeSiteChirho;
  further_ports_chirho: readonly number[];
}

export function derive_site_edge_chirho(edge_chirho: SiteEdgeChirho): string {
  const site_chirho = edge_chirho.site_chirho;
  if (edge_chirho.further_ports_chirho.length !== site_chirho.addresses_chirho.length - 1) throw Error("The edge needs one loopback port for each site address after the first");
  // A candidate site block is rehearsed as written: adding the lines again would double or mask its fence.
  if (edge_chirho.cache_fence_chirho && carries_cache_fence_chirho(edge_chirho.source_chirho)) {
    throw Error("The source site block already carries a cache fence; derive it as written");
  }
  const fence_chirho = edge_chirho.cache_fence_chirho ? `\n\t${R5_CACHE_FENCE_CHIRHO.join("\n\t")}` : "";
  const listeners_chirho = site_chirho.addresses_chirho.map((address_chirho, index_chirho) => [`\n${address_chirho} {`, index_chirho === 0
    ? `\nhttp://127.0.0.1:${edge_chirho.edge_port_chirho} {${fence_chirho}`
    : `\nhttp://127.0.0.1:${edge_chirho.further_ports_chirho[index_chirho - 1]} {\n\tbind 127.0.0.1`]);
  let caddy_chirho = edge_chirho.source_chirho;
  for (const [needle_chirho, replacement_chirho] of [...listeners_chirho,
    ["reverse_proxy 127.0.0.1:8788", `reverse_proxy 127.0.0.1:${edge_chirho.tenant_port_chirho}`],
    ["reverse_proxy 127.0.0.1:8789", `reverse_proxy 127.0.0.1:${edge_chirho.public_port_chirho}`]]) {
    // A changed or repeated line stops the derivation rather than being guessed at.
    const count_chirho = caddy_chirho.split(needle_chirho).length - 1;
    if (count_chirho !== 1) throw Error(`The source site block has ${count_chirho} of "${needle_chirho.trim()}", where the edge needs exactly one`);
    caddy_chirho = caddy_chirho.replace(needle_chirho, replacement_chirho);
  }
  const host_log_chirho = site_chirho.host_access_log_chirho;
  const logs_chirho = caddy_chirho.split(host_log_chirho).length - 1;
  if (logs_chirho > 1) throw Error(`The source site block has ${logs_chirho} of "${host_log_chirho}", where the edge needs at most one`);
  caddy_chirho = caddy_chirho.replace(host_log_chirho, `output file ${edge_chirho.access_log_chirho}`);
  // Any other log would be written on this machine at the host's path.
  const stray_chirho = caddy_chirho.split("\n").map(line_chirho => line_chirho.trim())
    .find(line_chirho => /^output\s+file\s/.test(line_chirho) && line_chirho.split(/\s+/)[2] !== edge_chirho.access_log_chirho);
  if (stray_chirho) throw Error(`The source site block writes a log the edge does not move: "${stray_chirho}"`);
  return `# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{\n admin 127.0.0.1:${edge_chirho.admin_port_chirho}\n auto_https off\n persist_config off\n storage file_system {\n root ${edge_chirho.storage_chirho}\n }\n}\n` + caddy_chirho;
}

// What a derived edge would open, read from Caddy's own adaptation of it before anything starts: every HTTP server's
// listen addresses, the admin endpoint and the apps it runs.
export interface EdgeListenersChirho { http_chirho: string[]; admin_chirho: string | null; apps_chirho: string[] }
export function edge_listeners_chirho(adapted_chirho: unknown): EdgeListenersChirho {
  const config_chirho = adapted_chirho as { admin?: { listen?: string }; apps?: Record<string, unknown> & { http?: { servers?: Record<string, { listen?: string[] }> } } };
  return { http_chirho: Object.values(config_chirho.apps?.http?.servers ?? {}).flatMap(server_chirho => server_chirho.listen ?? []).sort(),
    admin_chirho: config_chirho.admin?.listen ?? null, apps_chirho: Object.keys(config_chirho.apps ?? {}).sort() };
}
// A changed source can carry a site block the derivation never moves, a wider bind, or an address with no bind
// (gpt #27102). Caddy validates each and would listen on every interface, so the edge starts only when it would open
// exactly the selected site's loopback listeners, the loopback admin and the http app. Returns what it found.
export function require_loopback_listeners_chirho(adapted_chirho: unknown, http_chirho: string[], admin_chirho: string): string {
  const measured_chirho = edge_listeners_chirho(adapted_chirho);
  const expected_chirho: EdgeListenersChirho = { http_chirho: [...http_chirho].sort(), admin_chirho, apps_chirho: ["http"] };
  if (JSON.stringify(measured_chirho) !== JSON.stringify(expected_chirho)) {
    throw Error(`The derived edge would open ${JSON.stringify(measured_chirho)}, where only the selected site's loopback listeners may: ${JSON.stringify(expected_chirho)}`);
  }
  return `${measured_chirho.http_chirho.join(", ")}; admin ${admin_chirho}`;
}

export class ReviewedDeploymentChirho {
  readonly tenant_chirho = "beta-chirho";
  readonly root_chirho: string;
  cli_chirho = "";
  runtime_selection_chirho?: RuntimeSelectionChirho;
  // The Caddy the local edge validates and runs on, and its `caddy version` line.
  edge_cli_chirho = "";
  edge_version_chirho = "";
  // The committed site the edge derives (EDGE_SITES_CHIRHO): beta, unless a consumer that reads its own source selects
  // another (connected-release-services-chirho.ts, gpt #27086).
  edge_site_chirho: EdgeSiteNameChirho = "beta-chirho";
  readonly artifacts_chirho: Record<string, string> = {};
  readonly source_files_chirho: Record<string, string> = {};
  readonly checks_chirho: string[] = [];
  readonly private_values_chirho: string[] = [];
  readonly reviews_chirho: {
    app_chirho: string;
    candidate_chirho: string;
    report_chirho: string;
    surfaces_chirho: { route_chirho: string; diff_chirho: ReviewDiffChirho }[];
  }[] = [];
  readonly candidates_chirho: { review_chirho: SourceReviewChirho; tenant_chirho: boolean }[] = [];
  readonly users_chirho: { principal_chirho: string; credential_chirho: string }[] = [];
  // Synthetic credential files are numbered from here; a resumed root numbers its fresh pair after the kept run's.
  credential_offset_chirho = 0;
  // The OAuth client the bearers were issued under, which a kept run leaves for its resume.
  oauth_client_chirho = "";
  // The workspace's prepared source, and the authorities its reviewed surface may add. A release profile composed into
  // scratch (deploy_chirho/release-profiles-chirho.toml) sets its own; every other caller keeps these. These authority
  // labels are intentional; UI wording, node IDs and renderer hashes are not golden fixtures.
  workspace_source_chirho: string;
  readonly workspace_authorities_chirho = new Set([
    "ReadStoreChirho(prefix=viewer-state-chirho/)", "WriteStoreChirho(prefix=viewer-state-chirho/)",
    "ReadStoreChirho(prefix=publication-chirho/)", "TemplateAppDeployChirho(template=perffection-page-template-chirho)",
  ]);
  // The executed CLI sequence, so the host owner's product steps come from what ran, not from prose.
  readonly transcript_chirho: TranscriptStepChirho[] = [];
  private readonly placeholders_chirho = new Map<string, string>();
  constructor(readonly product_chirho: string, readonly engine_chirho: string, readonly scratch_chirho: string) {
    this.root_chirho = join(scratch_chirho, "root-chirho");
    this.workspace_source_chirho = join(product_chirho, "apps-chirho/perffection-workspace-chirho");
  }
  check_chirho(condition_chirho: unknown, label_chirho: string): void {
    if (!condition_chirho) throw Error(label_chirho);
    this.checks_chirho.push(label_chirho);
  }
  // The transcript as a numbered command sheet with its placeholder legend. Every private value
  // must have a name here; an unnamed one stops the sheet instead of printing as a redaction.
  command_sheet_chirho(): string {
    const steps_chirho = this.transcript_chirho.map((step_chirho, index_chirho) => `${index_chirho + 1}. \`${step_chirho.argv_chirho.join(" ")}\``
      + (step_chirho.refused_chirho === undefined ? "" : ` must refuse: "${step_chirho.refused_chirho}"`)
      + (step_chirho.defines_chirho ? ` -> \`${step_chirho.defines_chirho}\`` : ""));
    const sheet_chirho = [...SHEET_LEGEND_CHIRHO, "", ...steps_chirho, ""].join("\n");
    if (sheet_chirho.includes("[REDACTED]") || this.redact_chirho(sheet_chirho) !== sheet_chirho) throw Error("An unnamed private value reached the command sheet");
    return sheet_chirho;
  }
  redact_chirho(value_chirho: string): string {
    return this.private_values_chirho.reduce((text_chirho, secret_chirho) => text_chirho.replaceAll(secret_chirho, "[REDACTED]"), value_chirho);
  }
  // Longest literal first, so a root inside the release directory keeps its own name; anything
  // private that has no name still redacts.
  private placeholder_text_chirho(value_chirho: string): string {
    const named_chirho = [...this.placeholders_chirho].sort(([left_chirho], [right_chirho]) => right_chirho.length - left_chirho.length)
      .reduce((text_chirho, [literal_chirho, name_chirho]) => text_chirho.replaceAll(literal_chirho, name_chirho), value_chirho);
    return this.redact_chirho(named_chirho);
  }
  // Name a value the last CLI call printed; private values also join the redaction list.
  private define_chirho(value_chirho: string, name_chirho: string, private_chirho = true): void {
    if (private_chirho) this.private_values_chirho.push(value_chirho);
    this.placeholders_chirho.set(value_chirho, name_chirho);
    const step_chirho = this.transcript_chirho.at(-1);
    if (step_chirho) step_chirho.defines_chirho = name_chirho;
  }
  command_chirho(args_chirho: string[], label_chirho: string, expected_success_chirho = true, warning_free_chirho = false): string {
    const result_chirho = Bun.spawnSync(args_chirho, { cwd: this.product_chirho, stdout: "pipe", stderr: "pipe" });
    const output_chirho = result_chirho.stdout.toString().trim();
    const error_chirho = this.redact_chirho(result_chirho.stderr.toString());
    if (this.cli_chirho && args_chirho[0] === this.cli_chirho) {
      this.transcript_chirho.push({ argv_chirho: args_chirho.map(arg_chirho => this.placeholder_text_chirho(arg_chirho)),
        ...(expected_success_chirho ? {} : { refused_chirho: this.placeholder_text_chirho(result_chirho.stderr.toString()).split("\n").find(Boolean)?.slice(0, 240) }) });
    }
    if (expected_success_chirho !== (result_chirho.exitCode === 0)) {
      throw Error(`${label_chirho}: exit ${result_chirho.exitCode}: ${error_chirho.slice(0, 2400)}`);
    }
    if (warning_free_chirho && /\bwarn(?:ing)?\b/i.test(error_chirho)) throw Error(`${label_chirho}: ${error_chirho.slice(0, 2400)}`);
    return expected_success_chirho ? output_chirho : error_chirho;
  }
  flags_chirho(tenant_chirho = false): string[] {
    return ["--root", this.root_chirho, ...(tenant_chirho ? ["--tenant", this.tenant_chirho] : [])];
  }
  run_chirho(args_chirho: string[], tenant_chirho = false, expected_success_chirho = true): string {
    return this.command_chirho([this.cli_chirho, ...this.flags_chirho(tenant_chirho), ...args_chirho], args_chirho[0], expected_success_chirho);
  }
  async hash_chirho(path_chirho: string): Promise<string> {
    return new Bun.CryptoHasher("sha256").update(await Bun.file(path_chirho).arrayBuffer()).digest("hex");
  }
  async capture_chirho(path_chirho: string, label_chirho: string): Promise<void> {
    this.artifacts_chirho[label_chirho] = await this.hash_chirho(path_chirho);
    this.source_files_chirho[label_chirho] = path_chirho;
  }
  async verify_inputs_chirho(): Promise<void> {
    for (const [label_chirho, path_chirho] of Object.entries(this.source_files_chirho)) {
      this.check_chirho(await this.hash_chirho(path_chirho) === this.artifacts_chirho[label_chirho], `input unchanged: ${label_chirho}`);
    }
  }
  async stage_app_chirho(source_chirho: string, app_chirho: string): Promise<string> {
    const stage_chirho = join(this.scratch_chirho, app_chirho);
    const emitted_chirho = join(source_chirho, ".emitted-manifest-chirho.json");
    const stager_chirho = join(this.engine_chirho, "tools-chirho/app-deploy-chirho/host_ready_app_manifest_chirho.py");
    await this.capture_chirho(stager_chirho, "host-ready-stager-chirho");
    await this.capture_chirho(emitted_chirho, `${app_chirho}/emitted-chirho`);
    const inputs_chirho: ManifestFilesChirho = await Bun.file(emitted_chirho).json();
    for (const [index_chirho, entry_chirho] of [...inputs_chirho.nodes_chirho, ...inputs_chirho.assets_chirho].entries()) {
      const source_file_chirho = entry_chirho.source_chirho as ManifestFilesChirho["nodes_chirho"][number]["source_chirho"];
      const path_chirho = source_file_chirho.artifact_path_chirho ?? source_file_chirho.path_chirho;
      if (path_chirho) await this.capture_chirho(resolve(source_chirho, path_chirho), `${app_chirho}/source-input-${index_chirho}-chirho`);
      else if (!["builtin-node-template-chirho", "builtin-phi-chirho"].includes(source_file_chirho.kind_chirho)) throw Error("Prepared app input lacks an artifact path");
    }
    this.command_chirho(["python3", stager_chirho, source_chirho, stage_chirho], "stage prebuilt WASM");
    const manifest_chirho = join(stage_chirho, `${app_chirho}-hosted-source-manifest-chirho.json`);
    const staged_hashes_chirho: string[] = [];
    for (const row_chirho of (await readFile(join(stage_chirho, "sha256-chirho.txt"), "utf8")).trim().split("\n")) {
      const match_chirho = /^([a-f0-9]{64})  (.+)$/.exec(row_chirho);
      if (!match_chirho) throw Error("Invalid staged hash row");
      const path_chirho = resolve(stage_chirho, match_chirho[2]);
      if (!path_chirho.startsWith(`${stage_chirho}/`)) throw Error("Staged checksum path escapes its directory");
      this.check_chirho(await this.hash_chirho(path_chirho) === match_chirho[1], `staged bytes match: ${app_chirho}/${match_chirho[2]}`);
      if (path_chirho !== manifest_chirho) staged_hashes_chirho.push(match_chirho[1]);
      await this.capture_chirho(path_chirho, `${app_chirho}/${match_chirho[2]}`);
    }
    const input_hashes_chirho = Object.entries(this.artifacts_chirho).filter(([label_chirho]) => label_chirho.startsWith(`${app_chirho}/source-input-`)).map(([, hash_chirho]) => hash_chirho);
    this.check_chirho(JSON.stringify(staged_hashes_chirho.sort()) === JSON.stringify(input_hashes_chirho.sort()), `${app_chirho}: staged artifact bytes equal the original prepared inputs`);
    return manifest_chirho;
  }
  // The engine binary, frozen into the scratch. A resumed root (keep-and-resume) freezes its own under another name.
  async select_runtime_chirho(destination_name_chirho?: string): Promise<void> {
    this.runtime_selection_chirho = await resolve_reviewed_runtime_chirho(this.product_chirho, this.engine_chirho, this.scratch_chirho, process.env, destination_name_chirho);
    this.cli_chirho = this.runtime_selection_chirho.cli_chirho;
    if (this.runtime_selection_chirho.mode_chirho === "explicit-candidate-chirho") {
      console.error(`CANDIDATE RUN: ${this.runtime_selection_chirho.commit_chirho}, SHA-256 ${this.runtime_selection_chirho.sha256_chirho}; NOT the product pin. No pin or hosted acceptance.`);
    }
    for (const [relative_chirho, hash_chirho] of Object.entries(this.runtime_selection_chirho.helper_hashes_chirho)) {
      // Retain the independently checked commit hash, not a later re-read that could hide drift.
      this.artifacts_chirho[`candidate-helper/${relative_chirho}`] = hash_chirho;
      this.source_files_chirho[`candidate-helper/${relative_chirho}`] = join(this.engine_chirho, relative_chirho);
    }
    await this.capture_chirho(this.cli_chirho, "cairn-cli-chirho");
    for (const [literal_chirho, name_chirho] of [[this.cli_chirho, "cairn-chirho"], [this.root_chirho, "$PLATFORM_ROOT_CHIRHO"], [this.scratch_chirho, "$RELEASE_CHIRHO"]]) {
      this.placeholders_chirho.set(literal_chirho, name_chirho);
    }
  }
  async initialize_chirho(): Promise<void> {
    await this.select_runtime_chirho();
    await mkdir(this.root_chirho, { mode: 0o700 });
    for (const [source_chirho, app_chirho, tenant_chirho] of [
      [this.product_chirho, "perffection", false],
      [join(this.product_chirho, "apps-chirho/perffection-page-template-chirho"), "perffection-page-template-chirho", false],
      [this.workspace_source_chirho, "perffection-workspace", true],
    ] as const) {
      const manifest_chirho = await this.stage_app_chirho(source_chirho, app_chirho);
      const owner_chirho = this.run_chirho(["identity-register", "--kind", "owner"], tenant_chirho).split("\t")[0];
      this.define_chirho(owner_chirho, `$OWNER_${placeholder_name_chirho(app_chirho)}_CHIRHO`);
      const bypass_refusal_chirho = this.run_chirho(["app-source-deploy", manifest_chirho, "--owner", owner_chirho], tenant_chirho, false);
      this.check_chirho(bypass_refusal_chirho.includes("app-source-review + review-promote"), `${app_chirho}: deployment without review refuses at the review boundary`);
      const review_chirho: SourceReviewChirho = JSON.parse(this.run_chirho(["app-source-review", "--manifest", manifest_chirho, "--owner", owner_chirho, "--json"], tenant_chirho));
      const candidate_chirho = review_chirho.candidate_chirho;
      // The report binds this root's Owner, so the host's differs; the candidate DAG is left literal.
      this.define_chirho(review_chirho.source_review_report_chirho, `$REVIEW_REPORT_${placeholder_name_chirho(app_chirho)}_CHIRHO`, false);
      this.check_chirho(review_chirho.non_authoritative_chirho && /^[a-f0-9]{64}$/.test(review_chirho.source_review_report_chirho), `${app_chirho}: non-authoritative source review has an exact report hash`);
      this.check_chirho(candidate_chirho.app_chirho === app_chirho && candidate_chirho.owner_chirho === owner_chirho, `${app_chirho}: report binds the app and local Owner`);
      this.check_chirho(candidate_chirho.request_access_policy_chirho?.kind_chirho === (tenant_chirho ? "authenticated-chirho" : "public-chirho"), `${app_chirho}: reviewed access policy is explicit`);
      // Review the whole executable surface, not just the root DAG's summary.
      const allowed_chirho = tenant_chirho ? this.workspace_authorities_chirho : new Set<string>();
      this.check_chirho(!this.run_chirho(["refs"], tenant_chirho).includes(candidate_chirho.root_dag_chirho), `${app_chirho}: review has not installed the candidate as a ref`);
      const surfaces_chirho: { route_chirho: string; diff_chirho: ReviewDiffChirho }[] = [];
      for (const route_chirho of [{ route_chirho: "", dag_chirho: candidate_chirho.root_dag_chirho }, ...candidate_chirho.routes_chirho]) {
        const diff_chirho: ReviewDiffChirho = JSON.parse(this.run_chirho(["review-diff", "--app", app_chirho, "--candidate", route_chirho.dag_chirho, "--json"], tenant_chirho));
        if (diff_chirho.candidate_cost_token_chirho !== 0 || diff_chirho.added_capabilities_chirho.some(cap_chirho => !allowed_chirho.has(cap_chirho))) throw Error(`Unexpected review surface: ${JSON.stringify(diff_chirho)}`);
        this.check_chirho(diff_chirho.warnings_chirho.length === 0, `${app_chirho}/${route_chirho.route_chirho}: zero review warnings`);
        surfaces_chirho.push({ route_chirho: route_chirho.route_chirho, diff_chirho });
      }
      this.reviews_chirho.push({ app_chirho, candidate_chirho: candidate_chirho.root_dag_chirho, report_chirho: review_chirho.source_review_report_chirho, surfaces_chirho });
      this.candidates_chirho.push({ review_chirho, tenant_chirho });
    }
  }
  promote_chirho(): void {
    for (const { review_chirho, tenant_chirho } of this.candidates_chirho) {
      const candidate_chirho = review_chirho.candidate_chirho;
      const app_chirho = candidate_chirho.app_chirho;
      const refs_before_chirho = this.run_chirho(["refs"], tenant_chirho);
      const promote_chirho = ["review-promote", "--app", app_chirho, "--owner", candidate_chirho.owner_chirho,
        "--candidate", candidate_chirho.root_dag_chirho, "--source-review-report", review_chirho.source_review_report_chirho];
      const mismatch_chirho = [...promote_chirho];
      mismatch_chirho[mismatch_chirho.indexOf("--candidate") + 1] = "0".repeat(64);
      const mismatch_error_chirho = this.run_chirho(mismatch_chirho, tenant_chirho, false);
      this.check_chirho(mismatch_error_chirho.includes("source review report candidate does not match submitted candidate"), `${app_chirho}: mismatched candidate refuses before promotion`);
      const foreign_chirho = [...promote_chirho];
      foreign_chirho[foreign_chirho.indexOf("--app") + 1] = "unapproved-app-chirho";
      const foreign_error_chirho = this.run_chirho(foreign_chirho, tenant_chirho, false);
      this.check_chirho(foreign_error_chirho.includes("source review report app does not match request path"), `${app_chirho}: source review cannot promote a different app`);
      const other_owner_chirho = this.run_chirho(["identity-register", "--kind", "owner"], tenant_chirho).split("\t")[0];
      this.define_chirho(other_owner_chirho, `$OTHER_OWNER_${placeholder_name_chirho(app_chirho)}_CHIRHO`);
      const wrong_owner_chirho = [...promote_chirho];
      wrong_owner_chirho[wrong_owner_chirho.indexOf("--owner") + 1] = other_owner_chirho;
      const wrong_owner_error_chirho = this.run_chirho(wrong_owner_chirho, tenant_chirho, false);
      this.check_chirho(wrong_owner_error_chirho.includes("source review report owner does not match authenticated owner"), `${app_chirho}: another valid Owner cannot reuse this source review`);
      if (tenant_chirho) {
        const escalation_chirho = this.run_chirho(promote_chirho, true, false);
        this.check_chirho(/capabilit/i.test(escalation_chirho), "workspace: first capability grant needs explicit acceptance");
        promote_chirho.push("--accept-capability-escalation");
      }
      this.check_chirho(this.run_chirho(["refs"], tenant_chirho) === refs_before_chirho, `${app_chirho}: refused promotions leave refs unchanged`);
      this.run_chirho(promote_chirho, tenant_chirho);
      this.run_chirho(["review-verify"], tenant_chirho);
      this.run_chirho(["app-verify"], tenant_chirho);
      const refs_after_chirho = this.run_chirho(["refs"], tenant_chirho);
      this.check_chirho(refs_after_chirho !== refs_before_chirho && refs_after_chirho.includes(candidate_chirho.root_dag_chirho), `${app_chirho}: exact reviewed candidate becomes live`);
      const verification_before_chirho = this.run_chirho(["review-verify"], tenant_chirho);
      const retry_chirho = this.run_chirho(promote_chirho, tenant_chirho);
      this.check_chirho(retry_chirho.includes("already promoted") && this.run_chirho(["review-verify"], tenant_chirho) === verification_before_chirho && this.run_chirho(["refs"], tenant_chirho) === refs_after_chirho, `${app_chirho}: promotion retry adds no review or ref change`);
    }
    this.run_chirho(["tenant-app-verify"], true);
  }
  async configure_publication_chirho(): Promise<void> {
    const template_chirho = this.candidates_chirho.find(entry_chirho => entry_chirho.review_chirho.candidate_chirho.app_chirho === "perffection-page-template-chirho");
    const workspace_chirho = this.candidates_chirho.find(entry_chirho => entry_chirho.tenant_chirho);
    if (!template_chirho || !workspace_chirho) throw Error("Missing reviewed product candidates");
    this.run_chirho(["template-registry-set", "--template-id", "perffection-page-template-chirho", "--root-dag", template_chirho.review_chirho.candidate_chirho.root_dag_chirho, "--response-content-type", "text/html"]);
    for (const [slug_chirho, reason_chirho] of [["perffection", "platform-app-chirho"], ["perffection-workspace", "workspace-app-chirho"], ["perffection-page-template-chirho", "template-name-chirho"]]) {
      this.run_chirho(["publication-reserve-slug", "--slug", slug_chirho, "--reason", reason_chirho]);
    }
    const policy_chirho = join(this.product_chirho, "apps-chirho/perffection-workspace-chirho/deploy_chirho/publication-resource-policy-chirho.json");
    await this.capture_chirho(policy_chirho, "publication-resource-policy-chirho");
    // The archive carries these same bytes (verify-handoff checks it), which the host installs.
    this.placeholders_chirho.set(policy_chirho, "$RELEASE_CHIRHO/operations-chirho/publication-resource-policy-chirho.json");
    this.run_chirho(["resource-policy-set", "perffection-workspace", "--owner", workspace_chirho.review_chirho.candidate_chirho.owner_chirho, "--file", policy_chirho], true);
  }
  // Capture the product fixture separately from engine identity; it is not a hosted-source claim.
  async ingress_source_chirho(): Promise<string> {
    const ingress_chirho = join(this.product_chirho, EDGE_SITES_CHIRHO["beta-chirho"].path_chirho);
    await this.capture_chirho(ingress_chirho, "committed-ingress-chirho");
    return readFile(ingress_chirho, "utf8");
  }
  // The loopback ports of the site's addresses after the first (the apex's www), from +4, beside the fixed four.
  further_ports_chirho(port_chirho: number): number[] {
    return EDGE_SITES_CHIRHO[this.edge_site_chirho].addresses_chirho.slice(1).map((_address_chirho, index_chirho) => port_chirho + 4 + index_chirho);
  }
  // Derive the exact committed packet's route order; only hostname/ports, local storage and the access
  // log's file differ. Edge on `port_chirho`, public service +1, tenant service +2, Caddy admin +3, and any further
  // site address from +4 (further_ports_chirho), all on loopback.
  // `cache_fence_chirho` adds the R5 site-block lines verbatim, for rehearsing them on real responses.
  async derive_ingress_chirho(port_chirho: number, cache_fence_chirho = false): Promise<string> {
    const source_chirho = await this.ingress_source_chirho();
    this.edge_cli_chirho = caddy_cli_chirho();
    await this.capture_chirho(this.edge_cli_chirho, "caddy-cli-chirho");
    this.edge_version_chirho = this.command_chirho([this.edge_cli_chirho, "version"], "local ingress version");
    const config_chirho = join(this.scratch_chirho, "Caddyfile-chirho");
    await Bun.write(config_chirho, derive_site_edge_chirho({ source_chirho, site_chirho: EDGE_SITES_CHIRHO[this.edge_site_chirho],
      edge_port_chirho: port_chirho, public_port_chirho: port_chirho + 1, tenant_port_chirho: port_chirho + 2,
      admin_port_chirho: port_chirho + 3, further_ports_chirho: this.further_ports_chirho(port_chirho), storage_chirho: `${this.scratch_chirho}/caddy-storage-chirho`,
      access_log_chirho: join(this.scratch_chirho, "edge-access-chirho.log"), cache_fence_chirho }));
    this.command_chirho([this.edge_cli_chirho, "fmt", "--overwrite", config_chirho], "format generated local ingress");
    await this.capture_chirho(config_chirho, "derived-local-ingress-chirho");
    // before anything starts: exactly the selected site's loopback listeners and the loopback admin (gpt #27102)
    const adapted_chirho: unknown = JSON.parse(this.command_chirho([this.edge_cli_chirho, "adapt", "--adapter", "caddyfile", "--config", config_chirho], "adapt the local ingress"));
    const listening_chirho = require_loopback_listeners_chirho(adapted_chirho, [port_chirho, ...this.further_ports_chirho(port_chirho)].map(listen_chirho => `127.0.0.1:${listen_chirho}`),
      `127.0.0.1:${port_chirho + 3}`);
    this.check_chirho(true, `local ingress opens only its site's loopback listeners: ${listening_chirho}`);
    this.command_chirho([this.edge_cli_chirho, "validate", "--adapter", "caddyfile", "--config", config_chirho], "local ingress validation", true, true);
    return config_chirho;
  }
  // The owned tenant, public and edge processes behind that edge, in listener-verification order.
  service_commands_chirho(port_chirho: number, config_chirho: string): [string, string[]][] {
    if (!this.edge_cli_chirho) throw Error("Derive the ingress before starting its edge");
    const extra_chirho = extra_serve_args_chirho(process.env.HOSTED_REHEARSAL_SERVE_ARGS_CHIRHO);
    return [
      ["tenant-chirho", [this.cli_chirho, ...this.flags_chirho(true), "serve", "--port", String(port_chirho + 2), ...extra_chirho]],
      ["platform-chirho", [this.cli_chirho, ...this.flags_chirho(), "serve", "--port", String(port_chirho + 1), "--allow-platform-root-chirho", ...extra_chirho]],
      ["edge-chirho", [this.edge_cli_chirho, "run", "--adapter", "caddyfile", "--config", config_chirho]],
    ];
  }
  // Operator-issued synthetic bearers for the draft scope, one per issued user, in user order.
  // Transport proof only: not self-service keys, revocation or enforced action scopes.
  // A resumed root passes the kept run's client: its reviewed candidates lived only in that run.
  issue_synthetic_bearers_chirho(known_client_chirho?: string): string[] {
    let client_chirho = known_client_chirho ?? "";
    if (client_chirho) {
      this.private_values_chirho.push(client_chirho);
      this.placeholders_chirho.set(client_chirho, "$OAUTH_CLIENT_CHIRHO");
    } else {
      const workspace_chirho = this.candidates_chirho.find(candidate_chirho => candidate_chirho.tenant_chirho)?.review_chirho.candidate_chirho;
      if (!workspace_chirho) throw Error("No reviewed workspace");
      this.run_chirho(["oauth-client-register", "--app", workspace_chirho.app_chirho, "--owner", workspace_chirho.owner_chirho], true);
      client_chirho = this.run_chirho(["oauth-clients"], true).split("\n")[0].split("\t")[0];
      this.define_chirho(client_chirho, "$OAUTH_CLIENT_CHIRHO");
    }
    this.oauth_client_chirho = client_chirho;
    return this.users_chirho.map((user_chirho, index_chirho) => {
      const number_chirho = this.credential_offset_chirho + index_chirho;
      const issued_chirho = this.run_chirho(["oauth-token", "--principal", user_chirho.principal_chirho,
        "--credential-file", join(this.scratch_chirho, `synthetic-credential-${number_chirho}-chirho.txt`),
        "--client", client_chirho, "--scope", "page-draft-chirho", "--rate-window-ms", "3600000", "--rate-max-invokes", "400"], true);
      const key_chirho = issued_chirho.split("\n")[0].split("\t").find(value_chirho => value_chirho.startsWith("token="))?.slice(6);
      if (!key_chirho) throw Error("Operator token response lacks a token");
      this.define_chirho(key_chirho, `$USER_${number_chirho}_BEARER_CHIRHO`);
      return key_chirho;
    });
  }
  // Only the issued-account regression uses this. Publication provisioning must not create
  // accounts: the signup journey creates its users exclusively through canonical auth forms.
  async issue_synthetic_users_chirho(): Promise<void> {
    for (let index_chirho = 0; index_chirho < 2; index_chirho++) {
      const number_chirho = this.credential_offset_chirho + index_chirho;
      const credential_chirho = crypto.randomUUID() + crypto.randomUUID();
      const credential_path_chirho = join(this.scratch_chirho, `synthetic-credential-${number_chirho}-chirho.txt`);
      await write_private_chirho(credential_path_chirho, credential_chirho);
      this.private_values_chirho.push(credential_chirho);
      this.placeholders_chirho.set(credential_path_chirho, `$USER_${number_chirho}_CREDENTIAL_FILE_CHIRHO`);
      const principal_chirho = this.run_chirho(["identity-register", "--kind", "user", "--credential-file", credential_path_chirho], true).split("\t")[0];
      this.define_chirho(principal_chirho, `$USER_${number_chirho}_CHIRHO`);
      this.users_chirho.push({ principal_chirho, credential_chirho });
    }
  }
}

// Opt-in serve limits for growth arms only (PFE part 10 grows to 6,000 sign-ins; Cairn #25941-#25943 added the
// flags). Unset in the binding's hosted rehearsal, so reviewed runs serve with the engine's defaults. Accepts only the
// sign-in limit flags, each with a whole number, so the variable cannot hand a service any other switch (for example
// the platform-root one, which the tenant service must never take).
const ALLOWED_SERVE_LIMIT_FLAGS_CHIRHO = new Set(["--sign-in-principal-successes-per-hour-chirho", "--sign-in-source-successes-per-hour-chirho"]);
export function extra_serve_args_chirho(value_chirho: string | undefined): string[] {
  const words_chirho = (value_chirho ?? "").trim().split(/\s+/).filter(Boolean);
  if (words_chirho.length % 2 !== 0) throw Error("HOSTED_REHEARSAL_SERVE_ARGS_CHIRHO takes flag and value pairs");
  for (let index_chirho = 0; index_chirho < words_chirho.length; index_chirho += 2) {
    if (!ALLOWED_SERVE_LIMIT_FLAGS_CHIRHO.has(words_chirho[index_chirho]) || !/^[0-9]+$/.test(words_chirho[index_chirho + 1])) {
      throw Error(`HOSTED_REHEARSAL_SERVE_ARGS_CHIRHO: "${words_chirho[index_chirho]} ${words_chirho[index_chirho + 1]}" is not an allowed sign-in limit flag with a whole number`);
    }
  }
  return words_chirho;
}
