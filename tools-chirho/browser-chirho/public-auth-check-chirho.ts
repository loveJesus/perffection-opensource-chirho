#!/usr/bin/env bun
// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/public-signin-chirho.md.
// Real compiled apps + the normal two-root helper. Synthetic accounts only; no build or host call.
import { chmod, mkdir, mkdtemp, readFile, rm } from "node:fs/promises";
import { dirname, join, relative, resolve } from "node:path";
import { tmpdir } from "node:os";
import { pathToFileURL } from "node:url";
import type { Browser as BrowserChirho, BrowserContext as BrowserContextChirho, Page as PageChirho } from "playwright-core";
import { check_public_help_chirho, HELP_PATH_CHIRHO } from "./public-help-check-chirho";
import { check_account_chirho } from "./account-check-chirho";
import { resolve_cairn_runtime_chirho } from "./cairn-runtime-chirho";
import { check_signin_refusals_chirho } from "./signin-refusal-check-chirho";

const product_root_chirho = resolve(import.meta.dir, "../..");
const cairn_repo_chirho = resolve(process.env.CAIRN_REPO_CHIRHO ?? join(product_root_chirho, "../cairn-opensource-chirho"));
let cli_chirho = "";
const module_chirho = process.env.PLAYWRIGHT_MODULE_CHIRHO;
if (!module_chirho) throw Error("Set PLAYWRIGHT_MODULE_CHIRHO to the installed Playwright index.mjs; no install is performed.");
const port_chirho = Number(process.env.AUTH_QA_PORT_CHIRHO ?? "18940");
if (!Number.isInteger(port_chirho) || port_chirho < 1024 || port_chirho > 65531) throw Error("Invalid AUTH_QA_PORT_CHIRHO");
const base_chirho = `http://127.0.0.1:${port_chirho}`;
const signin_chirho = "/app-chirho/perffection/login-chirho";
const engine_login_chirho = "/auth-chirho/login-chirho";
const workspace_chirho = "/app-chirho/perffection-workspace/";
const account_only_chirho = process.env.AUTH_QA_ACCOUNT_CHIRHO === "only-chirho";
const account_mode_chirho = account_only_chirho || process.env.AUTH_QA_ACCOUNT_CHIRHO === "1";
const output_chirho = resolve(process.env.AUTH_QA_OUTPUT_CHIRHO ?? join(product_root_chirho, "target/public-auth-qa-chirho"));
await mkdir(output_chirho, { recursive: true });
const scratch_chirho = await mkdtemp(join(tmpdir(), "perffection-public-auth-chirho-"));
await chmod(scratch_chirho, 0o700);
const state_chirho = join(scratch_chirho, "state-chirho");
const checks_chirho: string[] = [];
const observations_chirho: Record<string, number | string | boolean> = {};
const secrets_chirho: string[] = [];
const page_errors_chirho: string[] = [];
let browser_chirho: BrowserChirho | undefined;
let helper_chirho: ReturnType<typeof Bun.spawn> | undefined;
const started_at_chirho = new Date().toISOString();
let next_navigation_chirho = 0;

async function pace_navigation_chirho(context_chirho: BrowserContextChirho): Promise<void> {
  // This functional/layout suite is not a load test. Leave room under the normal network actor
  // limit for its API controls. Do not alter request bytes, identity, responses or server policy;
  // credential refusals still accumulate against the real engine's 15-minute budget. Pace the
  // initial navigation too: Playwright does not intercept each hop of an HTTP redirect chain.
  await context_chirho.route(`${base_chirho}/**`, async route_chirho => {
    const request_chirho = route_chirho.request();
    if (request_chirho.isNavigationRequest()) {
      const now_chirho = Date.now();
      const reserved_chirho = Math.max(now_chirho, next_navigation_chirho);
      next_navigation_chirho = reserved_chirho + 1500;
      if (reserved_chirho > now_chirho) await Bun.sleep(reserved_chirho - now_chirho);
    }
    await route_chirho.continue();
  });
}

function check_chirho(ok_chirho: boolean, label_chirho: string): void {
  if (!ok_chirho) throw Error(label_chirho);
  checks_chirho.push(label_chirho);
}

async function command_chirho(args_chirho: string[]): Promise<string> {
  const result_chirho = Bun.spawnSync(args_chirho, { cwd: product_root_chirho, stdout: "pipe", stderr: "pipe" });
  if (result_chirho.exitCode !== 0) throw Error(`Read-only metadata command failed: ${args_chirho[0]}`);
  return result_chirho.stdout.toString().trim();
}

async function hash_chirho(path_chirho: string): Promise<string> {
  return new Bun.CryptoHasher("sha256").update(await Bun.file(path_chirho).arrayBuffer()).digest("hex");
}

async function fingerprints_chirho(): Promise<Record<string, string>> {
  const result_chirho: Record<string, string> = { cairn_cli_chirho: await hash_chirho(cli_chirho) };
  // Follow the manifests the helper actually deploys, not stale/held files left in target/.
  for (const manifest_path_chirho of [
    ".emitted-manifest-chirho.json",
    "apps-chirho/perffection-workspace-chirho/.emitted-manifest-chirho.json",
    "apps-chirho/perffection-page-template-chirho/.emitted-manifest-chirho.json",
  ]) {
    const absolute_chirho = join(product_root_chirho, manifest_path_chirho);
    result_chirho[manifest_path_chirho] = await hash_chirho(absolute_chirho);
    const manifest_chirho: { nodes_chirho: { source_chirho?: { artifact_path_chirho?: string; path_chirho?: string } }[];
      assets_chirho: { source_chirho?: { path_chirho?: string } }[] } = await Bun.file(absolute_chirho).json();
    for (const entry_chirho of [...manifest_chirho.nodes_chirho, ...manifest_chirho.assets_chirho]) {
      const source_chirho = entry_chirho.source_chirho as { artifact_path_chirho?: string; path_chirho?: string } | undefined;
      const path_chirho = source_chirho?.artifact_path_chirho ?? source_chirho?.path_chirho;
      if (!path_chirho) continue; // Builtin cards have no app-local file.
      const input_chirho = resolve(dirname(absolute_chirho), path_chirho);
      const label_chirho = relative(product_root_chirho, input_chirho);
      if (label_chirho.startsWith("..")) throw Error("Manifest input leaves the product tree");
      result_chirho[label_chirho] = await hash_chirho(input_chirho);
    }
  }
  return Object.fromEntries(Object.entries(result_chirho).sort(([a_chirho], [b_chirho]) => a_chirho.localeCompare(b_chirho)));
}

async function safe_evidence_chirho(name_chirho: string, value_chirho: unknown): Promise<void> {
  const text_chirho = JSON.stringify(value_chirho, null, 2) + "\n";
  if (secrets_chirho.some(secret_chirho => text_chirho.includes(secret_chirho))) throw Error("Private fixture bytes in evidence");
  await Bun.write(join(output_chirho, name_chirho), text_chirho);
}

async function navigate_chirho(page_chirho: PageChirho, path_chirho: string): Promise<void> {
  const response_chirho = await page_chirho.goto(base_chirho + path_chirho);
  check_chirho(response_chirho?.status() === 200, `display served: ${path_chirho}`);
  await page_chirho.evaluate(() => document.fonts.ready);
}

async function no_private_response_chirho(page_chirho: PageChirho, label_chirho: string): Promise<void> {
  const html_chirho = await page_chirho.content();
  check_chirho(!secrets_chirho.some(secret_chirho => html_chirho.includes(secret_chirho) || page_chirho.url().includes(secret_chirho)), label_chirho);
}

async function wait_for_helper_chirho(): Promise<void> {
  const deadline_chirho = Date.now() + 120_000;
  while (Date.now() < deadline_chirho) {
    if (helper_chirho?.exitCode !== null) throw Error("Local helper exited before readiness; inspect its owner-private scratch log");
    try {
      const response_chirho = await fetch(base_chirho + signin_chirho, { signal: AbortSignal.timeout(1000) });
      if (response_chirho.status === 200 && await Bun.file(join(state_chirho, "synthetic-user-principal-chirho.txt")).exists()) return;
    } catch { /* Bounded startup of our own local producer, not a verdict. */ }
    await Bun.sleep(250);
  }
  throw Error("Local helper readiness timed out");
}

try {
  // Never leave a previous green verdict as the current file while a rerun is unfinished.
  await safe_evidence_chirho("browser-result-chirho.json", { result_chirho: "running", started_at_chirho });
  cli_chirho = resolve_cairn_runtime_chirho(product_root_chirho, cairn_repo_chirho);
  const artifacts_before_chirho = await fingerprints_chirho();
  const product_head_chirho = await command_chirho(["git", "rev-parse", "HEAD"]);
  const product_delta_chirho = await command_chirho(["git", "diff", "--name-only"]);
  const engine_version_chirho = await command_chirho([cli_chirho, "--version"]);
  const { chromium: chromium_chirho } = await import(pathToFileURL(module_chirho).href);

  // A hidden cargo invocation would mutate shared outputs. Arm a subprocess guard, rather than
  // trusting SKIP_PREPARE or passing only because the current source happened to be up to date.
  const no_build_chirho = join(scratch_chirho, "no-build-chirho");
  await mkdir(no_build_chirho, { mode: 0o700 });
  await Bun.write(join(no_build_chirho, "cargo"), "#!/bin/sh\n# Generated no-build test guard.\nexit 87\n");
  await chmod(join(no_build_chirho, "cargo"), 0o700);
  const helper_env_chirho = { ...process.env, PATH: `${no_build_chirho}:${process.env.PATH ?? ""}` };
  check_chirho(Bun.spawnSync(["cargo", "--version"], { env: helper_env_chirho, stdout: "pipe", stderr: "pipe" }).exitCode === 87,
    "no-build guard is armed: invoking cargo refuses");

  // Refuse occupied ports: a green from somebody else's server is not our fixture.
  for (let offset_chirho = 0; offset_chirho < 4; offset_chirho++) {
    const reservation_chirho = Bun.listen({ hostname: "127.0.0.1", port: port_chirho + offset_chirho, socket: { data() {} } });
    reservation_chirho.stop(true);
  }
  helper_chirho = Bun.spawn(["bash", "scripts-chirho/dev-serve-tenant-chirho.sh", "--issue-synthetic-account-chirho"], {
    cwd: product_root_chirho,
    env: {
      ...helper_env_chirho, CAIRN_REPO_CHIRHO: cairn_repo_chirho, CAIRN_CLI_CHIRHO: cli_chirho,
      SKIP_PREPARE_CHIRHO: "1", STATE_DIR_CHIRHO: state_chirho,
      PORT_CHIRHO: String(port_chirho), PORT_PUBLIC_CHIRHO: String(port_chirho + 1),
      PORT_TENANT_CHIRHO: String(port_chirho + 2), CADDY_ADMIN_PORT_CHIRHO: String(port_chirho + 3),
    },
    stdout: Bun.file(join(scratch_chirho, "helper-log-chirho.txt")),
    stderr: Bun.file(join(scratch_chirho, "helper-errors-chirho.txt")),
  });
  await wait_for_helper_chirho();
  const principal_chirho = (await readFile(join(state_chirho, "synthetic-user-principal-chirho.txt"), "utf8")).trim();
  const credential_chirho = (await readFile(join(state_chirho, "synthetic-user-credential-chirho.txt"), "utf8")).trim();
  secrets_chirho.push(principal_chirho, credential_chirho);

  browser_chirho = await chromium_chirho.launch({ channel: "chrome", headless: true });
  if (!browser_chirho) throw Error("Chrome did not start");
  if (!account_only_chirho) {
    const context_chirho = await browser_chirho.newContext({ viewport: { width: 390, height: 844 } });
    await pace_navigation_chirho(context_chirho);
    const page_chirho = await context_chirho.newPage();
    page_chirho.on("pageerror", error_chirho => page_errors_chirho.push(`${new URL(page_chirho.url()).pathname}: ${error_chirho.name}: ${error_chirho.message}`));
    const request_chirho = context_chirho.request;
    const help_html_chirho = await check_public_help_chirho(page_chirho, base_chirho, output_chirho, check_chirho);
    await page_chirho.setViewportSize({ width: 390, height: 844 });

    const get_alias_chirho = await request_chirho.get(base_chirho + engine_login_chirho, { maxRedirects: 0 });
    check_chirho(get_alias_chirho.status() === 302 && get_alias_chirho.headers().location === signin_chirho, "exact login GET redirects to product display");
    const head_alias_chirho = await request_chirho.head(base_chirho + engine_login_chirho, { maxRedirects: 0 });
    check_chirho(head_alias_chirho.status() === 302 && head_alias_chirho.headers().location === signin_chirho, "exact login HEAD redirects to product display");
    const trailing_chirho = await request_chirho.get(base_chirho + engine_login_chirho + "/unrelated-chirho", { maxRedirects: 0 });
    check_chirho(trailing_chirho.status() === 404, "display alias does not swallow other auth paths");

    await navigate_chirho(page_chirho, engine_login_chirho);
    check_chirho(new URL(page_chirho.url()).pathname === signin_chirho, "engine login GET opens actual product template");
    check_chirho(await page_chirho.locator('form').getAttribute("action") === engine_login_chirho, "POST remains the canonical tenant auth road");
    check_chirho(await page_chirho.locator('[name="redirect_chirho"]').inputValue() === workspace_chirho, "success destination remains the fixed private workspace");
    check_chirho(await page_chirho.locator('[name="principal_chirho"]').inputValue() === "" && await page_chirho.locator('[name="credential_chirho"]').inputValue() === "", "sign-in fields start blank");
    const anonymous_html_chirho = await page_chirho.content();
    check_chirho(await page_chirho.locator('[role="alert"]:visible').count() === 0, "no unsolicited error notice");
    await page_chirho.locator('button[type="submit"]').click();
    check_chirho(new URL(page_chirho.url()).pathname === signin_chirho && await page_chirho.locator('input:invalid').count() === 2, "empty form is blocked by native validation");

    await navigate_chirho(page_chirho, signin_chirho);
    await page_chirho.keyboard.press("Tab");
    check_chirho(await page_chirho.locator('a[href="#login-form-chirho"]').evaluate(link_chirho => document.activeElement === link_chirho), "first keyboard stop bypasses navigation to the form");
    await page_chirho.keyboard.press("Enter");
    check_chirho(await page_chirho.locator('form').evaluate(form_chirho => document.activeElement === form_chirho), "skip link focuses real form");
    await page_chirho.keyboard.press("Tab");
    check_chirho(await page_chirho.locator('[name="principal_chirho"]').evaluate(input_chirho => document.activeElement === input_chirho), "keyboard enters sign-in name");
    await page_chirho.keyboard.press("Tab");
    check_chirho(await page_chirho.locator('[name="credential_chirho"]').evaluate(input_chirho => document.activeElement === input_chirho), "keyboard enters password");
    await page_chirho.keyboard.press("Tab");
    check_chirho(await page_chirho.locator('button[type="submit"]').evaluate(button_chirho => document.activeElement === button_chirho), "keyboard reaches submit");

    for (const width_chirho of [320, 390, 768, 1440]) {
      await page_chirho.setViewportSize({ width: width_chirho, height: 1000 });
      await navigate_chirho(page_chirho, signin_chirho);
      check_chirho(await page_chirho.evaluate(() => document.documentElement.scrollWidth <= innerWidth + 1), `${width_chirho}px login fits without clipping`);
      check_chirho(await page_chirho.locator('nav a').evaluateAll(links_chirho => links_chirho.length > 0 && links_chirho.every(link_chirho => {
        const box_chirho = link_chirho.getBoundingClientRect();
        return box_chirho.width > 0 && box_chirho.height >= 24 && box_chirho.left >= -1 && box_chirho.right <= innerWidth + 1;
      })), `${width_chirho}px navigation remains usable`);
      check_chirho(await page_chirho.locator('input:not([type="hidden"]), button').evaluateAll(controls_chirho => controls_chirho.every(control_chirho => {
        const box_chirho = control_chirho.getBoundingClientRect();
        return box_chirho.height >= 44 && parseFloat(getComputedStyle(control_chirho).fontSize) >= 16;
      })), `${width_chirho}px form has touch-size controls and readable text`);
    }
    // Public-page readability is not conditional on scroll-triggered entrance animations. Check
    // normal scrolling separately from reduced-motion full-page captures used for visual review.
    for (const path_chirho of ["/app-chirho/perffection/", "/app-chirho/perffection/craft-chirho", "/app-chirho/perffection/trust-chirho", "/app-chirho/perffection/help-chirho"]) {
      for (const width_chirho of [320, 1440]) {
        await page_chirho.setViewportSize({ width: width_chirho, height: 1000 });
        await navigate_chirho(page_chirho, path_chirho);
        check_chirho(await page_chirho.evaluate(() => document.documentElement.scrollWidth <= innerWidth + 1), `${width_chirho}px public page fits without horizontal scrolling: ${path_chirho}`);
        if (path_chirho === "/app-chirho/perffection/") {
          await page_chirho.screenshot({ path: join(output_chirho, `home-entry-${width_chirho}-chirho.png`) });
        }
        check_chirho(await page_chirho.locator('.site-brand-group-chirho, nav a').evaluateAll(elements_chirho => elements_chirho.every(element_chirho => {
          const box_chirho = element_chirho.getBoundingClientRect();
          return box_chirho.width > 0 && box_chirho.left >= -1 && box_chirho.right <= innerWidth + 1;
        })), `${width_chirho}px shared header fits on ${path_chirho}`);
        for (const heading_chirho of await page_chirho.locator("main h2").all()) {
          if (!await heading_chirho.isVisible()) continue;
          await heading_chirho.evaluate(element_chirho => element_chirho.scrollIntoView({ block: "center", behavior: "instant" }));
          await page_chirho.evaluate(() => new Promise<void>(resolve_chirho => requestAnimationFrame(() => requestAnimationFrame(() => resolve_chirho()))));
          const opacity_chirho = await heading_chirho.evaluate(element_chirho => {
            let effective_chirho = 1;
            for (let ancestor_chirho: Element | null = element_chirho; ancestor_chirho; ancestor_chirho = ancestor_chirho.parentElement) {
              effective_chirho *= Number(getComputedStyle(ancestor_chirho).opacity);
            }
            return effective_chirho;
          });
          check_chirho(opacity_chirho >= 0.95, `${width_chirho}px section heading remains readable when scrolled into view: ${path_chirho}`);
        }
        if (path_chirho === "/app-chirho/perffection/") {
          await page_chirho.emulateMedia({ reducedMotion: "reduce" });
          await page_chirho.screenshot({ path: join(output_chirho, `home-${width_chirho}-chirho.png`), fullPage: true });
          await page_chirho.emulateMedia({ reducedMotion: "no-preference" });
        }
      }
      const [signin_response_chirho] = await Promise.all([
        page_chirho.waitForNavigation({ waitUntil: "networkidle" }),
        page_chirho.locator(`nav a[href="${engine_login_chirho}"]`).click(),
      ]);
      check_chirho(signin_response_chirho?.status() === 200 && page_chirho.url() === base_chirho + signin_chirho,
        `${path_chirho}: public canonical sign-in link follows the issued-account display redirect`);
      check_chirho(await page_chirho.locator(`form[action="${engine_login_chirho}"] [name="principal_chirho"]`).count() === 1
        && await page_chirho.locator('[name="username_chirho"], a[href="/auth-chirho/register-chirho"]').count() === 0,
        `${path_chirho}: installed baseline keeps its issued-account form without offering unbound username signup`);
      check_chirho(await page_chirho.locator('nav a[aria-current="page"]').getAttribute("href") === engine_login_chirho,
        `${path_chirho}: canonical Sign in is current on the redirected public display`);
    }
    for (const fragment_chirho of ["#refusal-no-session-chirho", "#refusal-credential-guess-exhausted-chirho",
      "#refusal-invalid-credential-chirho", "#refusal-auth-rate-limited-chirho"]) {
      for (const width_chirho of [320, 390, 768, 1440]) {
        await page_chirho.setViewportSize({ width: width_chirho, height: 844 });
        await navigate_chirho(page_chirho, engine_login_chirho + fragment_chirho);
        check_chirho(page_chirho.url() === base_chirho + signin_chirho + fragment_chirho,
          `${width_chirho}px login display preserves the fixed refusal fragment`);
        check_chirho(await page_chirho.locator('[role="alert"]:visible').count() === 1, `${width_chirho}px fixed refusal selects one notice`);
        check_chirho(await page_chirho.locator('[role="alert"]:visible h2').evaluate(heading_chirho => {
          const box_chirho = heading_chirho.getBoundingClientRect();
          const hit_chirho = document.elementFromPoint(box_chirho.left + box_chirho.width / 2, box_chirho.top + box_chirho.height / 2);
          return box_chirho.top >= 0 && box_chirho.bottom <= innerHeight && hit_chirho !== null && heading_chirho.contains(hit_chirho);
        }), `${width_chirho}px fixed refusal heading is readable and not covered by navigation`);
        check_chirho(await page_chirho.evaluate(() => document.documentElement.scrollWidth <= innerWidth + 1), `${width_chirho}px refusal fits without horizontal clipping`);
        if (width_chirho === 390 && ["#refusal-invalid-credential-chirho", "#refusal-auth-rate-limited-chirho"].includes(fragment_chirho)) {
          check_chirho(await page_chirho.locator('[name="principal_chirho"]').inputValue() === ""
            && await page_chirho.locator('[name="credential_chirho"]').inputValue() === "", "display-only refusal capture has blank sign-in fields");
          await no_private_response_chirho(page_chirho, "fixed sign-in refusal contains no private fixture values");
          await page_chirho.screenshot({ path: join(output_chirho, `signin-${fragment_chirho.slice(1)}.png`), fullPage: true });
        }
      }
    }
    await page_chirho.setViewportSize({ width: 390, height: 844 });
    await navigate_chirho(page_chirho, engine_login_chirho + "#refusal-no-session-chirho");
    check_chirho(page_chirho.url() === base_chirho + signin_chirho + "#refusal-no-session-chirho", "browser inherits fixed no-session fragment across display redirect");
    check_chirho(await page_chirho.locator('[role="alert"]:visible').count() === 1, "known fragment shows one actionable notice");
    await no_private_response_chirho(page_chirho, "notice contains no synthetic secret or principal");
    await page_chirho.screenshot({ path: join(output_chirho, "signin-return-mobile-chirho.png"), fullPage: true });
    await Promise.all([
      page_chirho.waitForURL(base_chirho + signin_chirho, { waitUntil: "load" }),
      page_chirho.locator('[role="alert"]:visible a').click(),
    ]);
    check_chirho(await page_chirho.locator('[role="alert"]:visible').count() === 0 && new URL(page_chirho.url()).hash === "", "notice dismisses to the clean public sign-in display");
    await navigate_chirho(page_chirho, engine_login_chirho + "#unknown-fragment-chirho");
    check_chirho(await page_chirho.locator('[role="alert"]:visible').count() === 0, "unknown fragment does not claim a known result");
    await page_chirho.locator('a[href="#login-help-chirho"]').click();
    check_chirho(await page_chirho.locator('aside').evaluate(aside_chirho => document.activeElement === aside_chirho), "help link focuses on-page help");

    // Explicit layout stress, not evidence of a translated runtime release.
    await navigate_chirho(page_chirho, signin_chirho);
    await page_chirho.evaluate(() => {
      document.documentElement.dir = "rtl";
      document.documentElement.style.fontSize = "200%";
      for (const element_chirho of document.querySelectorAll('main p, main small, main h1, main h2, main h3, main label > span, main button')) {
        element_chirho.textContent = `${element_chirho.textContent} — ${element_chirho.textContent}`;
      }
    });
    await page_chirho.screenshot({ path: join(output_chirho, "signin-expanded-rtl-chirho.png"), fullPage: true });
    const reflow_chirho = await page_chirho.evaluate(() => ({
      viewport_chirho: innerWidth,
      scroll_chirho: document.documentElement.scrollWidth,
      overflow_chirho: [...document.querySelectorAll("main *")].filter(element_chirho => {
        const box_chirho = element_chirho.getBoundingClientRect();
        return box_chirho.width > 0 && (box_chirho.left < -1 || box_chirho.right > innerWidth + 1);
      }).map(element_chirho => ({ tag_chirho: element_chirho.tagName, class_chirho: element_chirho.className })),
    }));
    await safe_evidence_chirho("reflow-chirho.json", reflow_chirho);
    check_chirho(reflow_chirho.scroll_chirho <= reflow_chirho.viewport_chirho + 1, "390px simulated RTL and doubled text keep the form usable");

    for (const [method_chirho, path_chirho] of [
      ["GET", "/auth-chirho/human-check-chirho"], ["GET", "/auth-chirho/register-chirho"], ["POST", "/auth-chirho/register-chirho"],
    ]) {
      check_chirho((await request_chirho.fetch(base_chirho + path_chirho, { method: method_chirho })).status() === 503, `${method_chirho} registration road stays closed`);
    }
    check_chirho((await request_chirho.get(base_chirho + workspace_chirho)).status() === 401, "anonymous workspace still refuses before app execution");
    Object.assign(observations_chirho, await check_signin_refusals_chirho({
      page_chirho, base_chirho, signin_chirho, engine_login_chirho, workspace_chirho,
      principal_chirho, output_chirho, secrets_chirho, check_chirho,
    }));

    // Use the actual returned form, not a manually navigated substitute.
    await page_chirho.locator('[name="principal_chirho"]').fill(principal_chirho);
    await page_chirho.locator('[name="credential_chirho"]').fill(credential_chirho);
    const login_response_chirho = page_chirho.waitForResponse(response_chirho => response_chirho.request().method() === "POST" && response_chirho.url() === base_chirho + engine_login_chirho);
    await page_chirho.locator('button[type="submit"]').click();
    check_chirho((await login_response_chirho).status() === 303, "actual returned browser form signs in successfully through tenant auth");
    await page_chirho.waitForURL(base_chirho + workspace_chirho);
    check_chirho(await page_chirho.locator('form[action$="/edit-page-chirho"]').count() === 1, "authenticated user reaches real editor");
    const session_chirho = (await context_chirho.cookies()).find(cookie_chirho => cookie_chirho.name === "cairn_session_chirho");
    if (!session_chirho) throw Error("No session cookie after real login");
    secrets_chirho.push(session_chirho.value);
    check_chirho(session_chirho.secure && session_chirho.httpOnly && session_chirho.sameSite === "Lax", "browser stores Secure HttpOnly SameSite=Lax session");
    await no_private_response_chirho(page_chirho, "editor response contains no synthetic principal/password/session");
    await Promise.all([
      page_chirho.waitForURL(base_chirho + HELP_PATH_CHIRHO, { waitUntil: "load" }),
      page_chirho.locator(`nav a[href="${HELP_PATH_CHIRHO}"]`).click(),
    ]);
    check_chirho(new URL(page_chirho.url()).pathname === HELP_PATH_CHIRHO && await page_chirho.content() === help_html_chirho,
      "workspace Help link reaches the same public guide without exposing the valid tenant session");
    await Promise.all([
      page_chirho.waitForURL(base_chirho + workspace_chirho, { waitUntil: "load" }),
      page_chirho.locator(`.help-start-chirho a[href="${workspace_chirho}"]`).click(),
    ]);
    check_chirho(new URL(page_chirho.url()).pathname === workspace_chirho && await page_chirho.locator('form[action$="/edit-page-chirho"]').count() === 1,
      "Help returns to the real authenticated editor without asking for the password again");
    await navigate_chirho(page_chirho, workspace_chirho + "preview-chirho");
    await no_private_response_chirho(page_chirho, "preview response contains no synthetic principal/password/session");
    await navigate_chirho(page_chirho, signin_chirho);
    check_chirho(await page_chirho.content() === anonymous_html_chirho, "public sign-in bytes are independent of a valid tenant session");
    const malformed_chirho = { Authorization: "Bearer invalid-synthetic-chirho" };
    check_chirho((await request_chirho.get(base_chirho + signin_chirho, { headers: malformed_chirho })).status() === 200, "public-only matcher strips invalid Authorization");
    check_chirho((await request_chirho.get(base_chirho + workspace_chirho, { headers: malformed_chirho })).status() === 403, "tenant matcher preserves and refuses invalid Authorization");
    const logout_response_chirho = page_chirho.waitForResponse(response_chirho => response_chirho.url() === base_chirho + "/auth-chirho/logout-chirho");
    await navigate_chirho(page_chirho, "/auth-chirho/logout-chirho");
    check_chirho((await logout_response_chirho).status() === 303, "logout remains a real engine revocation");
    check_chirho(new URL(page_chirho.url()).pathname === signin_chirho && await page_chirho.locator('form').getAttribute("action") === engine_login_chirho, "logout returns to branded sign-in instead of the generic engine form");
    check_chirho(!(await context_chirho.cookies()).some(cookie_chirho => cookie_chirho.name === "cairn_session_chirho"), "logout clears the browser session");
    check_chirho((await request_chirho.get(base_chirho + workspace_chirho)).status() === 401, "workspace remains private after logout");
    const old_cookie_chirho = { Cookie: `cairn_session_chirho=${session_chirho.value}` };
    check_chirho((await request_chirho.get(base_chirho + workspace_chirho, { headers: old_cookie_chirho })).status() === 403, "old session is revoked, not merely removed from browser");
    check_chirho((await request_chirho.get(base_chirho + signin_chirho, { headers: old_cookie_chirho })).status() === 200, "stale tenant session cannot poison public re-entry");
    check_chirho((await request_chirho.get(base_chirho + HELP_PATH_CHIRHO, { headers: old_cookie_chirho })).status() === 200,
      "Help remains reachable with a revoked tenant cookie");
    await page_chirho.setViewportSize({ width: 1440, height: 1000 });
    await navigate_chirho(page_chirho, signin_chirho);
    await page_chirho.screenshot({ path: join(output_chirho, "signin-desktop-chirho.png"), fullPage: true });
    await no_private_response_chirho(page_chirho, "re-entry display and URL contain no prior credentials");
    check_chirho(await page_chirho.locator("script").count() === 0 && page_errors_chirho.length === 0, "production display has no scripts or browser exceptions");
  }
  const account_result_chirho = account_mode_chirho ? await check_account_chirho({
    browser_chirho, base_chirho, principal_chirho, credential_chirho, output_chirho, secrets_chirho, check_chirho,
    prepare_context_chirho: pace_navigation_chirho,
  }) : undefined;
  const artifacts_after_chirho = await fingerprints_chirho();
  const changed_artifacts_chirho = [...new Set([...Object.keys(artifacts_before_chirho), ...Object.keys(artifacts_after_chirho)])]
    .filter(path_chirho => artifacts_before_chirho[path_chirho] !== artifacts_after_chirho[path_chirho]);
  check_chirho(changed_artifacts_chirho.length === 0, changed_artifacts_chirho.length === 0
    ? "consumed product and engine outputs did not change during the run"
    : `Inputs changed during the browser run: ${changed_artifacts_chirho.join(", ")}`);
  const verdict_chirho = account_result_chirho?.findings_chirho.length ? "failed" : "passed";
  await safe_evidence_chirho("browser-result-chirho.json", {
    result_chirho: verdict_chirho, started_at_chirho, measured_at_chirho: new Date().toISOString(), browser_chirho: browser_chirho.version(),
    scope_chirho: `Local unreviewed-development two-root apps, synthetic account, current pin. ${account_only_chirho ? "Focused Account journey; public Help/layout regression suite not run." : "Includes public sign-in and Help regression checks."} ${account_mode_chirho ? "Includes real-browser R1 password-change checks; failures remain explicit." : "No R1 acceptance."} No host/TLS, signup activation, reviewed release, complete recovery or provider.`,
    product_head_chirho, product_dirty_paths_chirho: product_delta_chirho.split("\n"), engine_version_chirho,
    check_count_chirho: checks_chirho.length, checks_chirho, observations_chirho, artifacts_chirho: artifacts_before_chirho,
    ...(account_result_chirho ? { account_chirho: account_result_chirho } : {}),
  });
  console.log(JSON.stringify({ result_chirho: verdict_chirho, check_count_chirho: checks_chirho.length, output_chirho, observations_chirho,
    ...(account_result_chirho ? { account_chirho: account_result_chirho } : {}) }));
  if (verdict_chirho === "failed") process.exitCode = 1;
} catch (error_chirho) {
  const message_chirho = error_chirho instanceof Error ? error_chirho.message : "Unknown failure";
  const safe_message_chirho = secrets_chirho.reduce((text_chirho, secret_chirho) => text_chirho.replaceAll(secret_chirho, "[REDACTED]"), message_chirho);
  const failure_chirho = { result_chirho: "failed", started_at_chirho, measured_at_chirho: new Date().toISOString(),
    error_chirho: safe_message_chirho, completed_checks_chirho: checks_chirho,
    browser_errors_chirho: page_errors_chirho.map(error_chirho => secrets_chirho.reduce(
      (text_chirho, secret_chirho) => text_chirho.replaceAll(secret_chirho, "[REDACTED]"), error_chirho)) };
  await safe_evidence_chirho("browser-result-chirho.json", failure_chirho);
  await safe_evidence_chirho("browser-failure-chirho.json", failure_chirho);
  console.error(safe_message_chirho);
  process.exitCode = 1;
} finally {
  await browser_chirho?.close();
  if (helper_chirho && helper_chirho.exitCode === null) {
    helper_chirho.kill("SIGTERM");
    await helper_chirho.exited;
  }
  // The helper stops its three processes and removes its fresh root; only our synthetic metadata
  // remains in this unique directory. No external state directory is accepted by this checker.
  await rm(scratch_chirho, { recursive: true, force: true });
}
