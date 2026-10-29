#!/usr/bin/env bun
// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// Workflow: spec-chirho/workflows-chirho/product-delivery-chirho/reviewed-product-chirho.md.
// Exercises source-review/promote + a named real browser journey. No build, development bypass or host.
import { chmod, mkdir, mkdtemp, rm } from "node:fs/promises";
import { join, resolve } from "node:path";
import { tmpdir } from "node:os";
import { pathToFileURL } from "node:url";
import type { Browser as BrowserChirho, Page as PageChirho } from "playwright-core";
import { ReviewedDeploymentChirho } from "./reviewed-deployment-chirho";
import { PublicationJourneyChirho } from "../browser-flows-chirho/publication-chirho";
import { EditorValidationJourneyChirho } from "../browser-flows-chirho/editor-validation-chirho";
import { AgentApiJourneyChirho } from "../browser-flows-chirho/agent-api-chirho";
import { LocalPublicationObserverChirho } from "../browser-flows-chirho/publication-observer-chirho";
import { WorkspaceSessionChirho } from "../browser-flows-chirho/workspace-session-chirho";

const product_chirho = resolve(import.meta.dir, "../..");
const engine_chirho = resolve(process.env.CAIRN_REPO_CHIRHO ?? join(product_chirho, "../cairn-opensource-chirho"));
const module_chirho = process.env.PLAYWRIGHT_MODULE_CHIRHO
  ?? join(product_chirho, "tools-chirho/browser-toolchain-chirho/node_modules/playwright-core/index.mjs");
const port_chirho = Number(process.env.REVIEW_QA_PORT_CHIRHO ?? "18960");
if (!Number.isInteger(port_chirho) || port_chirho < 1024 || port_chirho > 65531) throw Error("Invalid REVIEW_QA_PORT_CHIRHO");
const output_chirho = resolve(process.env.REVIEW_QA_OUTPUT_CHIRHO ?? join(product_chirho, "target/reviewed-product-qa-chirho"));
const address_scope_chirho = process.env.REVIEW_QA_ADDRESS_CHIRHO ?? "reserved-and-taken-chirho";
if (!["reserved-and-taken-chirho", "reserved-only-chirho"].includes(address_scope_chirho)) throw Error("Invalid REVIEW_QA_ADDRESS_CHIRHO");
const flow_chirho = process.env.REVIEW_QA_FLOW_CHIRHO ?? "publication-chirho";
if (!["publication-chirho", "agent-api-chirho"].includes(flow_chirho)) throw Error("Invalid REVIEW_QA_FLOW_CHIRHO");
await mkdir(output_chirho, { recursive: true });
const scratch_chirho = await mkdtemp(join(tmpdir(), "perffection-reviewed-product-chirho-"));
await chmod(scratch_chirho, 0o700);
const fixture_chirho = new ReviewedDeploymentChirho(product_chirho, engine_chirho, scratch_chirho);
const origin_chirho = `http://127.0.0.1:${port_chirho}`;
const session_chirho = new WorkspaceSessionChirho(fixture_chirho, origin_chirho, check_chirho);
const workspace_chirho = session_chirho.workspace_chirho;
const status_chirho = `${workspace_chirho}publication-status-chirho`;
const signin_chirho = session_chirho.signin_chirho;
const slug_chirho = "reviewed-bakery-chirho";
const observer_chirho = new LocalPublicationObserverChirho(fixture_chirho);
const publication_chirho = new PublicationJourneyChirho(fixture_chirho, origin_chirho, slug_chirho, observer_chirho);
const address_recovery_chirho = new PublicationJourneyChirho(fixture_chirho, origin_chirho, "recovered-bakery-chirho", observer_chirho);
const editor_validation_chirho = new EditorValidationJourneyChirho(fixture_chirho, origin_chirho, output_chirho);
const agent_api_chirho = new AgentApiJourneyChirho(fixture_chirho, origin_chirho, output_chirho, { kind_chirho: "local-chirho", deployment_chirho: fixture_chirho });
const children_chirho: ReturnType<typeof Bun.spawn>[] = [];
const checks_chirho: string[] = [];
const observations_chirho: Record<string, unknown> = {};
const browser_errors_chirho: string[] = [];
let browser_chirho: BrowserChirho | undefined;
let editor_page_chirho: PageChirho | undefined;
let error_chirho: string | undefined;
let phase_chirho = "source-review-chirho";

function check_chirho(value_chirho: unknown, label_chirho: string): void {
  if (!value_chirho) throw Error(label_chirho);
  checks_chirho.push(label_chirho);
}
async function evidence_chirho(name_chirho: string, value_chirho: unknown): Promise<void> {
  const text_chirho = JSON.stringify(value_chirho, null, 2) + "\n";
  if (fixture_chirho.redact_chirho(text_chirho) !== text_chirho) throw Error("Private fixture bytes in evidence");
  await Bun.write(join(output_chirho, name_chirho), text_chirho);
}
function start_chirho(name_chirho: string, args_chirho: string[]): void {
  children_chirho.push(Bun.spawn(args_chirho, { cwd: product_chirho,
    stdout: Bun.file(join(scratch_chirho, `${name_chirho}-stdout-chirho.log`)), stderr: Bun.file(join(scratch_chirho, `${name_chirho}-stderr-chirho.log`)) }));
}
async function status_code_chirho(url_chirho: string): Promise<number> {
  return (await fetch(url_chirho, { signal: AbortSignal.timeout(2000) })).status;
}
async function wait_ready_chirho(): Promise<void> {
  const deadline_chirho = Date.now() + 30_000;
  while (Date.now() < deadline_chirho) {
    if (children_chirho.some(child_chirho => child_chirho.exitCode !== null)) throw Error("Owned service exited during startup");
    try {
      if (await status_code_chirho(`${origin_chirho}/healthz-chirho`) === 200
        && await status_code_chirho(`http://127.0.0.1:${port_chirho + 1}/healthz-chirho`) === 200) return;
    } catch { /* Bounded wait on our own live producers. */ }
    await Bun.sleep(250);
  }
  throw Error("Owned services did not become ready");
}
try {
  await evidence_chirho("reviewed-result-chirho.json", { result_chirho: "running", started_at_chirho: new Date().toISOString() });
  if (!await Bun.file(module_chirho).exists()) throw Error("Browser tools missing: run bun install --frozen-lockfile --ignore-scripts in tools-chirho/browser-toolchain-chirho, or set PLAYWRIGHT_MODULE_CHIRHO");
  observations_chirho.product_head_chirho = fixture_chirho.command_chirho(["git", "rev-parse", "HEAD"], "product coordinate");
  for (const source_chirho of ["reviewed-product-check-chirho.ts", "reviewed-deployment-chirho.ts", "cairn-runtime-chirho.ts", "../browser-flows-chirho/publication-chirho.ts", "../browser-flows-chirho/editor-validation-chirho.ts", "../browser-flows-chirho/form-keyboard-chirho.ts", "../browser-flows-chirho/agent-api-chirho.ts",
    "../browser-flows-chirho/publication-observer-chirho.ts", "../browser-flows-chirho/workspace-session-chirho.ts",
    "../browser-flows-chirho/private-file-chirho.ts"]) {
    await fixture_chirho.capture_chirho(resolve(import.meta.dir, source_chirho), `consumer/${source_chirho}`);
  }
  await fixture_chirho.initialize_chirho();
  observations_chirho.engine_version_chirho = fixture_chirho.command_chirho([fixture_chirho.cli_chirho, "--version"], "binary identity");
  await evidence_chirho("review-surfaces-chirho.json", fixture_chirho.reviews_chirho);
  for (let offset_chirho = 0; offset_chirho < 4; offset_chirho++) {
    const socket_chirho = Bun.listen({ hostname: "127.0.0.1", port: port_chirho + offset_chirho, socket: { data() {} } });
    socket_chirho.stop(true);
  }
  const config_chirho = await fixture_chirho.derive_ingress_chirho(port_chirho);
  for (const [name_chirho, args_chirho] of fixture_chirho.service_commands_chirho(port_chirho, config_chirho)) start_chirho(name_chirho, args_chirho);
  await wait_ready_chirho();
  const server_helper_chirho = join(product_chirho, "scripts-chirho/lib-chirho/served-root-chirho.sh");
  await fixture_chirho.capture_chirho(server_helper_chirho, "owned-listener-helper-chirho");
  for (const [child_index_chirho, port_offset_chirho] of [[0, 2], [1, 1], [2, 0], [2, 3]]) {
    check_chirho(fixture_chirho.command_chirho(["bash", "-c", 'source "$1"; own_server_verdict_chirho "$2" "$3"',
      "reviewed-listener-chirho", server_helper_chirho, String(children_chirho[child_index_chirho].pid), String(port_chirho + port_offset_chirho)],
    "verify owned listener") === "own", `port ${port_chirho + port_offset_chirho} belongs to the process this run started`);
  }
  check_chirho(fixture_chirho.command_chirho(["bash", "-c", 'source "$1"; own_server_verdict_chirho "$2" "$3"',
    "reviewed-listener-control-chirho", server_helper_chirho, String(children_chirho[0].pid), String(port_chirho + 1)],
  "foreign listener control").startsWith("foreign:"), "listener control refuses a healthy service owned by another PID");
  const pre_promotion_chirho = await Promise.all([signin_chirho, workspace_chirho].map(async url_chirho => {
    const response_chirho = await fetch(url_chirho);
    return { status_chirho: response_chirho.status, body_chirho: fixture_chirho.redact_chirho(await response_chirho.text()) };
  }));
  observations_chirho.pre_promotion_chirho = pre_promotion_chirho;
  // initialize_chirho independently proves review installed no candidate ref. Accept the
  // detail-safe refusal identities as well as older pins' internal-detail bodies.
  check_chirho(pre_promotion_chirho.every(response_chirho =>
    (response_chirho.status_chirho === 400
      && (response_chirho.body_chirho.trim() === "error: app-selection-refused-chirho"
        || /no signed owner record/.test(response_chirho.body_chirho)))
    || (response_chirho.status_chirho === 404
      && (response_chirho.body_chirho.trim() === "error: app-not-found-chirho"
        || /no deployed app/.test(response_chirho.body_chirho)))), "source review alone refuses both undeployed apps at their owner/deployment boundaries");
  phase_chirho = "promotion-chirho";
  fixture_chirho.promote_chirho();
  if (flow_chirho === "publication-chirho") await fixture_chirho.configure_publication_chirho();
  await fixture_chirho.issue_synthetic_users_chirho();
  check_chirho(await status_code_chirho(signin_chirho) === 200 && await status_code_chirho(workspace_chirho) === 401, "promotion activates public display and preserves private access policy");
  const { chromium: chromium_chirho } = await import(pathToFileURL(module_chirho).href);
  const active_browser_chirho: BrowserChirho = await chromium_chirho.launch({ channel: "chrome", headless: true });
  browser_chirho = active_browser_chirho;
  observations_chirho.browser_chirho = active_browser_chirho.version();
  const context_chirho = await active_browser_chirho.newContext({ viewport: { width: 1365, height: 950 } });
  context_chirho.on("page", page_chirho => page_chirho.on("pageerror", error_chirho => browser_errors_chirho.push(fixture_chirho.redact_chirho(error_chirho.message))));
  const page_chirho = await context_chirho.newPage();
  const failed_assets_chirho: string[] = [];
  page_chirho.on("response", response_chirho => { if (response_chirho.status() >= 400 && new URL(response_chirho.url()).pathname.startsWith("/app-assets-chirho/")) failed_assets_chirho.push(new URL(response_chirho.url()).pathname); });
  if (flow_chirho === "agent-api-chirho") {
    phase_chirho = "agent-api-transport-chirho";
    editor_page_chirho = page_chirho;
    observations_chirho.agent_api_chirho = agent_api_chirho.observations_chirho;
    await agent_api_chirho.start_chirho(page_chirho);
    await session_chirho.login_chirho(page_chirho, 0);
    phase_chirho = "agent-api-browser-chirho";
    await agent_api_chirho.browser_chirho(page_chirho);
    await agent_api_chirho.prepare_other_chirho();
    const other_context_chirho = await active_browser_chirho.newContext();
    other_context_chirho.on("page", page_chirho => page_chirho.on("pageerror", error_chirho => browser_errors_chirho.push(fixture_chirho.redact_chirho(error_chirho.message))));
    const other_page_chirho = await other_context_chirho.newPage();
    await session_chirho.login_chirho(other_page_chirho, 1);
    await agent_api_chirho.other_browser_chirho(other_page_chirho);
    await other_context_chirho.close();
  } else {
    phase_chirho = "publish-first-saved-version-chirho";
    await session_chirho.login_chirho(page_chirho, 0);
    await session_chirho.save_chirho(page_chirho, "Fresh bread version one", phase_chirho);
    observations_chirho.publication_chirho = await publication_chirho.run_chirho(page_chirho, "Fresh bread version one", "Fresh bread version two");
    phase_chirho = "isolation-and-assets-chirho";
    const other_context_chirho = await active_browser_chirho.newContext();
    other_context_chirho.on("page", page_chirho => page_chirho.on("pageerror", error_chirho => browser_errors_chirho.push(fixture_chirho.redact_chirho(error_chirho.message))));
    const other_page_chirho = await other_context_chirho.newPage();
    await session_chirho.login_chirho(other_page_chirho, 1);
    await publication_chirho.verify_other_viewer_chirho(other_page_chirho);
    phase_chirho = "editor-validation-chirho";
    editor_page_chirho = other_page_chirho;
    observations_chirho.editor_validation_chirho = editor_validation_chirho.observations_chirho;
    observations_chirho.editor_validation_chirho = await editor_validation_chirho.run_chirho(other_page_chirho);
    phase_chirho = "publication-address-recovery-chirho";
    observations_chirho.address_recovery_chirho = address_recovery_chirho.address_observations_chirho;
    await address_recovery_chirho.run_address_recovery_chirho(other_page_chirho, "perffection-page-template-chirho", slug_chirho,
      output_chirho, address_scope_chirho === "reserved-and-taken-chirho");
    phase_chirho = "isolation-and-assets-chirho";
    check_chirho(await status_code_chirho(`http://127.0.0.1:${port_chirho + 2}/app-chirho/${slug_chirho}/`) === 404, "published app is not deployed into the private tenant");
    for (const [method_chirho, path_chirho] of [["GET", "human-check-chirho"], ["GET", "register-chirho"], ["POST", "register-chirho"]]) {
      check_chirho((await fetch(`${origin_chirho}/auth-chirho/${path_chirho}`, { method: method_chirho })).status === 503, `${method_chirho} registration remains closed`);
    }
    observations_chirho.served_css_sha256_chirho = await publication_chirho.verify_assets_chirho(page_chirho);
    check_chirho(failed_assets_chirho.length === 0, "published renderer loads without failed asset responses");
    await page_chirho.screenshot({ path: join(output_chirho, "reviewed-live-page-chirho.png"), fullPage: true });
    await page_chirho.goto(status_chirho);
    await page_chirho.setViewportSize({ width: 390, height: 844 });
    await publication_chirho.check_status_layout_chirho(page_chirho, join(output_chirho, "reviewed-publishing-mobile-chirho.png"));
    fixture_chirho.run_chirho(["publication-verify"]);
  }
  check_chirho(failed_assets_chirho.length === 0, "selected reviewed journey loads all requested app assets");
  fixture_chirho.run_chirho(["review-verify"]);
  fixture_chirho.run_chirho(["review-verify"], true);
  fixture_chirho.run_chirho(["tenant-app-verify"], true);
  check_chirho(browser_errors_chirho.length === 0, "reviewed customer journeys raise no browser exceptions");
} catch (caught_chirho) {
  error_chirho = fixture_chirho.redact_chirho(caught_chirho instanceof Error ? caught_chirho.message : "Unknown failure");
  if (["editor-validation-chirho", "publication-address-recovery-chirho", "agent-api-transport-chirho", "agent-api-browser-chirho"].includes(phase_chirho) && editor_page_chirho) {
    const failure_name_chirho = `${phase_chirho}-failure-chirho.png`;
    await editor_page_chirho.screenshot({ path: join(output_chirho, failure_name_chirho), fullPage: true })
      .catch(capture_error_chirho => { observations_chirho.failure_capture_error_chirho = fixture_chirho.redact_chirho(String(capture_error_chirho)); });
  }
} finally {
  observations_chirho.runtime_selection_chirho = fixture_chirho.runtime_selection_chirho;
  observations_chirho.edge_caddy_chirho = fixture_chirho.edge_version_chirho;
  // A red product arm must retain the same input-drift evidence as a successful journey.
  try { await fixture_chirho.verify_inputs_chirho(); }
  catch (input_error_chirho) {
    const message_chirho = fixture_chirho.redact_chirho(String(input_error_chirho));
    observations_chirho.input_verification_error_chirho = message_chirho;
    error_chirho ??= message_chirho;
  }
  await browser_chirho?.close();
  for (const child_chirho of children_chirho) if (child_chirho.exitCode === null) child_chirho.kill();
  const exits_chirho = await Promise.all(children_chirho.map(child_chirho => child_chirho.exited));
  if (flow_chirho === "agent-api-chirho" && !error_chirho) {
    try { await agent_api_chirho.privacy_chirho(); }
    catch (privacy_error_chirho) { error_chirho = fixture_chirho.redact_chirho(String(privacy_error_chirho)); }
  }
  const result_chirho = error_chirho ? "failed" : flow_chirho === "agent-api-chirho" && agent_api_chirho.known_gaps_chirho.length
    ? "transport-passed-with-open-gaps" : "passed";
  await evidence_chirho("reviewed-result-chirho.json", {
    result_chirho, measured_at_chirho: new Date().toISOString(), phase_chirho, error_chirho,
    scope_chirho: "Local source-review + exact-candidate promotion, prebuilt WASM, real Chrome and synthetic accounts. Only the named flow is exercised (publication uses a test-driven applier). No development deployment bypass; no host/TLS, human release approval, signup, password-change acceptance or provider.",
    flow_chirho, address_scope_chirho: flow_chirho === "publication-chirho" ? address_scope_chirho : "not-exercised-chirho",
    checks_chirho, review_checks_chirho: fixture_chirho.checks_chirho, observations_chirho, browser_errors_chirho, artifacts_chirho: fixture_chirho.artifacts_chirho,
    owned_services_stopped_chirho: exits_chirho.length === children_chirho.length && exits_chirho.every(Number.isInteger),
  });
  await rm(scratch_chirho, { recursive: true, force: true });
  console.log(JSON.stringify({ result_chirho, phase_chirho, browser_checks_chirho: checks_chirho.length, error_chirho, output_chirho }));
  if (error_chirho) process.exitCode = 1;
}
