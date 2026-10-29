#!/usr/bin/env bun
// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/signup-chirho.md.
// Real configured guest + reviewed product apps. No mocked auth, CLI User, engine build or host.
import { chmod, mkdir, mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import type { Browser as BrowserChirho, Page as PageChirho, Response as ResponseChirho } from "playwright-core";
import { SignupBrowserFixtureChirho } from "./signup-browser-fixture-chirho";
import { PublicationJourneyChirho } from "../browser-flows-chirho/publication-chirho";
import { LocalPublicationObserverChirho } from "../browser-flows-chirho/publication-observer-chirho";

interface UserChirho { username_chirho: string; credential_chirho: string }

const product_chirho = resolve(import.meta.dir, "../..");
const engine_chirho = resolve(process.env.CAIRN_REPO_CHIRHO ?? join(product_chirho, "../cairn-opensource-chirho"));
const output_chirho = resolve(process.env.SIGNUP_BROWSER_OUTPUT_CHIRHO ?? join(product_chirho, "target/signup-browser-qa-chirho"));
const guest_chirho = resolve(process.env.AUTH_DISPLAY_GUEST_CHIRHO ?? join(product_chirho,
  "target/wasm32-unknown-unknown/release/examples/render_auth_guest_chirho.wasm"));
const port_chirho = Number(process.env.SIGNUP_BROWSER_PORT_CHIRHO ?? "18980");
const module_chirho = process.env.PLAYWRIGHT_MODULE_CHIRHO
  ?? join(product_chirho, "tools-chirho/browser-toolchain-chirho/node_modules/playwright-core/index.mjs");
if (process.argv.length > 2) throw Error("This command has only full-journey mode; it cannot pass as a login-only check");
await mkdir(output_chirho, { recursive: true });
const scratch_chirho = await mkdtemp(join(tmpdir(), "perffection-signup-browser-chirho-"));
await chmod(scratch_chirho, 0o700);
const fixture_chirho = new SignupBrowserFixtureChirho(product_chirho, engine_chirho, scratch_chirho, port_chirho);
const origin_chirho = fixture_chirho.browser_origin_chirho;
const login_chirho = `${origin_chirho}/auth-chirho/login-chirho`;
const registration_chirho = `${origin_chirho}/auth-chirho/register-chirho`;
const workspace_chirho = fixture_chirho.browser_workspace_chirho;
const publication_chirho = new PublicationJourneyChirho(fixture_chirho, origin_chirho, "signup-bakery-chirho",
  new LocalPublicationObserverChirho(fixture_chirho));
const observations_chirho: Record<string, unknown> = {};
const errors_chirho: string[] = [];
let phase_chirho = "review-and-configure-chirho";
let failure_chirho: string | undefined;
let browser_chirho: BrowserChirho | undefined;
const check_chirho = (condition_chirho: unknown, label_chirho: string): void => fixture_chirho.check_chirho(condition_chirho, label_chirho);

async function evidence_chirho(result_chirho: string): Promise<void> {
  const text_chirho = JSON.stringify({ result_chirho, at_chirho: new Date().toISOString(), phase_chirho,
    scope_chirho: "Local Chrome signup/sign-in/save/publish/republish/rollback/re-entry through a CAS-configured guest and reviewed apps; synthetic accounts and test-driven applier only.",
    failure_chirho, observations_chirho, browser_errors_chirho: errors_chirho.map(error_chirho => fixture_chirho.redact_chirho(error_chirho)),
    checks_chirho: fixture_chirho.checks_chirho, artifacts_chirho: fixture_chirho.artifacts_chirho,
    limits_chirho: ["Not hosted TLS, launch, complete recovery or anti-abuse acceptance", "Arithmetic challenge solving proves only the local protocol",
      "No provider, money, production pin or activation change", "Engine audit and its unresolved failures are separate"],
  }, null, 2) + "\n";
  if (fixture_chirho.redact_chirho(text_chirho) !== text_chirho) throw Error("Private fixture bytes in proposed browser evidence");
  await Bun.write(join(output_chirho, "signup-browser-result-chirho.json"), text_chirho);
}

function user_chirho(): UserChirho {
  const value_chirho = { username_chirho: `member_${crypto.randomUUID().replaceAll("-", "")}`, credential_chirho: crypto.randomUUID() + crypto.randomUUID() };
  fixture_chirho.private_values_chirho.push(value_chirho.username_chirho, value_chirho.credential_chirho);
  return value_chirho;
}

async function protected_html_chirho(page_chirho: PageChirho, response_chirho: ResponseChirho): Promise<void> {
  const headers_chirho = await response_chirho.allHeaders();
  check_chirho(headers_chirho["content-type"]?.includes("text/html"), `${phase_chirho}: engine returns HTML`);
  check_chirho(/(?:^|,)\s*no-store\b/i.test(headers_chirho["cache-control"] ?? ""), `${phase_chirho}: auth response prohibits storage`);
  const csp_chirho = headers_chirho["content-security-policy"] ?? "";
  for (const directive_chirho of ["default-src 'none'", "script-src 'none'", "form-action 'self'", "frame-ancestors 'none'", "base-uri 'none'"]) {
    check_chirho(csp_chirho.split(";").some(part_chirho => part_chirho.trim() === directive_chirho), `${phase_chirho}: ${directive_chirho}`);
  }
  await page_chirho.waitForLoadState("networkidle");
  check_chirho(await page_chirho.locator('main h1').count() === 1 && await page_chirho.locator('nav').count() > 0,
    `${phase_chirho}: configured product shell, not the built-in form`);
  check_chirho(await page_chirho.locator('script, [name="principal_chirho"], [name="registration_token_chirho"]').count() === 0,
    `${phase_chirho}: no script, copied principal or invitation field`);
  const html_chirho = await page_chirho.content();
  check_chirho(fixture_chirho.redact_chirho(html_chirho) === html_chirho && !/user\/[a-f0-9]{64}/.test(html_chirho),
    `${phase_chirho}: returned page does not echo account secrets or principal`);
  check_chirho(fixture_chirho.redact_chirho(page_chirho.url()) === page_chirho.url(), `${phase_chirho}: address contains no private fixture marker`);
}

async function visit_auth_chirho(page_chirho: PageChirho, route_chirho: string, status_chirho = 200): Promise<void> {
  const response_chirho = await page_chirho.goto(route_chirho);
  check_chirho(response_chirho?.status() === status_chirho, `${phase_chirho}: GET status ${status_chirho}`);
  await protected_html_chirho(page_chirho, response_chirho!);
}

async function follow_auth_link_chirho(page_chirho: PageChirho, selector_chirho: string, route_chirho: string, status_chirho = 200): Promise<void> {
  const [response_chirho] = await Promise.all([page_chirho.waitForNavigation({ waitUntil: "networkidle" }),
    page_chirho.locator(selector_chirho).click()]);
  check_chirho(page_chirho.url() === route_chirho && response_chirho?.status() === status_chirho,
    `${phase_chirho}: real link reaches the canonical auth display with status ${status_chirho}`);
  await protected_html_chirho(page_chirho, response_chirho!);
}

async function public_entry_chirho(page_chirho: PageChirho): Promise<void> {
  const home_chirho = "/app-chirho/perffection/";
  for (const path_chirho of [home_chirho, `${home_chirho}craft-chirho`, `${home_chirho}trust-chirho`, `${home_chirho}help-chirho`]) {
    const response_chirho = await page_chirho.goto(path_chirho === home_chirho ? origin_chirho : origin_chirho + path_chirho);
    check_chirho(response_chirho?.status() === 200 && new URL(page_chirho.url()).pathname === path_chirho,
      `public entry serves ${path_chirho}, including the bare-origin front door`);
    const current_chirho = page_chirho.locator('nav a[aria-current="page"]');
    check_chirho(await current_chirho.count() === 1 && await current_chirho.getAttribute("href") === path_chirho,
      `${path_chirho}: navigation marks the actual public page`);
    check_chirho(await page_chirho.locator('a[href="/app-chirho/perffection/login-chirho"]').count() === 0,
      `${path_chirho}: no navigation into the legacy principal form`);
    await follow_auth_link_chirho(page_chirho, 'nav a[href="/auth-chirho/login-chirho"]', login_chirho);
    await form_fields_chirho(page_chirho, "/auth-chirho/login-chirho", ["username_chirho", "credential_chirho", "redirect_chirho"]);
    check_chirho(await page_chirho.locator('nav a[aria-current="page"]').getAttribute("href") === "/auth-chirho/login-chirho",
      `${path_chirho}: canonical username screen marks Sign in current`);
    check_chirho(errors_chirho.length === 0, `${path_chirho}: public navigation reaches auth without browser errors`);
  }
}

async function json_refusal_chirho(route_chirho: string, status_chirho: number, code_chirho: string): Promise<void> {
  const response_chirho = await fetch(route_chirho);
  check_chirho(response_chirho.status === status_chirho && response_chirho.headers.get("content-type")?.includes("application/json"),
    `${phase_chirho}: configured renderer preserves JSON refusal status and representation`);
  const body_chirho = await response_chirho.json() as Record<string, unknown>;
  check_chirho(body_chirho.ok_chirho === false && body_chirho.code_chirho === code_chirho
    && typeof body_chirho.error_chirho === "string" && !response_chirho.headers.has("set-cookie"),
    `${phase_chirho}: JSON refusal retains its public code without a session`);
}

async function submit_chirho(page_chirho: PageChirho, action_chirho: string): Promise<ResponseChirho> {
  const response_chirho = page_chirho.waitForResponse(response_chirho =>
    response_chirho.request().method() === "POST" && response_chirho.url() === `${origin_chirho}${action_chirho}`);
  await Promise.all([page_chirho.waitForNavigation({ waitUntil: "networkidle" }),
    page_chirho.locator(`form[action="${action_chirho}"] button[type="submit"]`).click()]);
  return await response_chirho;
}

async function form_fields_chirho(page_chirho: PageChirho, action_chirho: string, names_chirho: string[]): Promise<void> {
  const form_chirho = page_chirho.locator(`form[action="${action_chirho}"]`);
  check_chirho(await form_chirho.count() === 1 && await form_chirho.getAttribute("method") === "post", `${phase_chirho}: direct canonical POST form`);
  const actual_chirho = await form_chirho.locator('[name]').evaluateAll(elements_chirho => elements_chirho.map(element_chirho => element_chirho.getAttribute("name")).sort());
  check_chirho(JSON.stringify(actual_chirho) === JSON.stringify([...names_chirho].sort()), `${phase_chirho}: exact auth protocol field set`);
  check_chirho(await form_chirho.locator('[name="username_chirho"]').inputValue() === ""
    && await form_chirho.locator('[name="credential_chirho"]').inputValue() === "", `${phase_chirho}: credentials are not prefilled or echoed`);
}

async function challenge_answer_chirho(page_chirho: PageChirho): Promise<string> {
  const answer_chirho = page_chirho.locator('[name="human_check_chirho"]');
  const id_chirho = await answer_chirho.getAttribute("id");
  const label_chirho = await page_chirho.locator(`label[for="${id_chirho}"]`).textContent();
  const parts_chirho = /\b(\d+)\s*\+\s*(\d+)\b/.exec(label_chirho ?? "");
  if (!parts_chirho) throw Error("Local runner can only solve the synthetic arithmetic challenge");
  return String(Number(parts_chirho[1]) + Number(parts_chirho[2]));
}

async function fill_signup_chirho(page_chirho: PageChirho, account_chirho: UserChirho): Promise<string> {
  await form_fields_chirho(page_chirho, "/auth-chirho/register-chirho", ["kind_chirho", "username_chirho", "credential_chirho", "human_check_chirho", "human_check_token_chirho"]);
  const token_chirho = await page_chirho.locator('[name="human_check_token_chirho"]').inputValue();
  check_chirho(token_chirho.length > 0, "fresh challenge has an opaque token");
  fixture_chirho.private_values_chirho.push(token_chirho, encodeURIComponent(token_chirho));
  await page_chirho.locator('[name="username_chirho"]').fill(account_chirho.username_chirho);
  await page_chirho.locator('[name="credential_chirho"]').fill(account_chirho.credential_chirho);
  await page_chirho.locator('[name="human_check_chirho"]').fill(await challenge_answer_chirho(page_chirho));
  return token_chirho;
}

async function login_user_chirho(page_chirho: PageChirho, account_chirho: UserChirho, wrong_chirho = false): Promise<string> {
  await visit_auth_chirho(page_chirho, login_chirho);
  await form_fields_chirho(page_chirho, "/auth-chirho/login-chirho", ["username_chirho", "credential_chirho", "redirect_chirho"]);
  check_chirho(await page_chirho.locator('main a[href="/auth-chirho/register-chirho"]').evaluate(link_chirho =>
    getComputedStyle(link_chirho).textDecorationLine.includes("underline")), "account availability is visibly identifiable as a link");
  await page_chirho.locator('[name="username_chirho"]').fill(account_chirho.username_chirho);
  await page_chirho.locator('[name="credential_chirho"]').fill(wrong_chirho ? "wrong-synthetic-password-chirho" : account_chirho.credential_chirho);
  const response_chirho = await submit_chirho(page_chirho, "/auth-chirho/login-chirho");
  check_chirho(response_chirho.status() === 303, `${phase_chirho}: form login returns SEE_OTHER`);
  if (wrong_chirho) {
    check_chirho((await response_chirho.allHeaders())["set-cookie"] === undefined, "wrong sign-in does not set a session");
    check_chirho(page_chirho.url() === `${login_chirho}#refusal-invalid-credential-chirho`, "wrong sign-in returns to canonical username display");
    const alerts_chirho = page_chirho.locator('[role="alert"]:visible');
    check_chirho(await alerts_chirho.count() === 1, "wrong sign-in shows exactly one readable notice");
    check_chirho(await page_chirho.locator('[name="username_chirho"]').inputValue() === ""
      && await page_chirho.locator('[name="credential_chirho"]').inputValue() === "", "refused credentials are not reflected into retry fields");
    return (await alerts_chirho.innerText()).trim();
  }
  check_chirho(page_chirho.url() === workspace_chirho, "username login reaches the private workspace");
  const cookie_chirho = (await page_chirho.context().cookies()).find(cookie_chirho => cookie_chirho.name === "cairn_session_chirho");
  check_chirho(cookie_chirho?.httpOnly && cookie_chirho.secure && cookie_chirho.sameSite === "Lax", "browser stored a Secure HttpOnly SameSite=Lax session");
  fixture_chirho.private_values_chirho.push(cookie_chirho!.value);
  check_chirho(await page_chirho.locator('form[action$="edit-page-chirho"]').count() === 1, "authenticated real editor is usable");
  return cookie_chirho!.value;
}

try {
  await evidence_chirho("running-chirho");
  if (!await Bun.file(module_chirho).exists()) throw Error("Browser tools missing: run bun install --frozen-lockfile --ignore-scripts in tools-chirho/browser-toolchain-chirho, or set PLAYWRIGHT_MODULE_CHIRHO");
  observations_chirho.product_head_chirho = fixture_chirho.command_chirho(["git", "rev-parse", "HEAD"], "product coordinate");
  for (const name_chirho of ["signup-browser-check-chirho.ts", "signup-browser-fixture-chirho.ts", "signup-fixture-chirho.ts", "reviewed-deployment-chirho.ts", "cairn-runtime-chirho.ts", "../browser-flows-chirho/publication-chirho.ts", "../browser-flows-chirho/form-keyboard-chirho.ts"]) {
    await fixture_chirho.capture_chirho(join(import.meta.dir, name_chirho), `consumer/${name_chirho}`);
  }
  await fixture_chirho.initialize_browser_chirho(guest_chirho);
  observations_chirho.engine_version_chirho = fixture_chirho.command_chirho([fixture_chirho.cli_chirho, "--version"], "candidate identity");
  observations_chirho.renderer_chirho = fixture_chirho.renderer_chirho;
  await fixture_chirho.start_browser_chirho(false, 2);
  const { chromium: chromium_chirho } = await import(pathToFileURL(module_chirho).href) as typeof import("playwright-core");
  browser_chirho = await chromium_chirho.launch({ channel: "chrome", headless: true });
  observations_chirho.browser_version_chirho = browser_chirho.version();
  const context_chirho = await browser_chirho.newContext({ viewport: { width: 390, height: 844 }, reducedMotion: "reduce" });
  const page_chirho = await context_chirho.newPage();
  page_chirho.setDefaultTimeout(15_000);
  page_chirho.on("pageerror", error_chirho => errors_chirho.push(`${phase_chirho}: ${new URL(page_chirho.url()).pathname}: ${error_chirho.name}: ${error_chirho.message}`));
  page_chirho.on("console", message_chirho => { if (/Content Security Policy|Refused to (?:load|apply|execute)/i.test(message_chirho.text())) errors_chirho.push(`${phase_chirho}: ${new URL(page_chirho.url()).pathname}: ${message_chirho.text()}`); });

  phase_chirho = "public-account-entry-chirho";
  await public_entry_chirho(page_chirho);
  phase_chirho = "configured-login-chirho";
  await form_fields_chirho(page_chirho, "/auth-chirho/login-chirho", ["username_chirho", "credential_chirho", "redirect_chirho"]);
  check_chirho(await page_chirho.evaluate(() => document.documentElement.scrollWidth <= innerWidth), "served phone sign-in fits the viewport");
  check_chirho(await page_chirho.locator('link[rel="stylesheet"]').evaluateAll(elements_chirho => elements_chirho.length > 0
    && elements_chirho.every(element_chirho => (element_chirho as HTMLLinkElement).sheet !== null)), "same-origin auth stylesheets loaded under the served CSP");
  check_chirho(await page_chirho.evaluate(async () => { await document.fonts.ready; return [...document.fonts].some(face_chirho => face_chirho.status === "loaded"); }),
    "auth page loads a real font face under the served CSP");
  await page_chirho.screenshot({ path: join(output_chirho, "served-login-chirho.png"), fullPage: true });
  for (const width_chirho of [320, 768, 1440]) {
    await page_chirho.setViewportSize({ width: width_chirho, height: 1000 });
    check_chirho(await page_chirho.evaluate(() => document.documentElement.scrollWidth <= innerWidth),
      `${width_chirho}px username entry and availability link fit the viewport`);
  }
  await page_chirho.screenshot({ path: join(output_chirho, "served-login-desktop-chirho.png"), fullPage: true });
  await page_chirho.setViewportSize({ width: 390, height: 844 });

  phase_chirho = "closed-registration-chirho";
  await follow_auth_link_chirho(page_chirho, 'main a[href="/auth-chirho/register-chirho"]', registration_chirho, 503);
  check_chirho(await page_chirho.locator('form').count() === 0, "closed registration displays no signup form");
  await page_chirho.screenshot({ path: join(output_chirho, "served-registration-closed-chirho.png"), fullPage: true });
  await json_refusal_chirho(`${origin_chirho}/auth-chirho/human-check-chirho`, 503, "registration-disabled-chirho");
  check_chirho((await fetch(workspace_chirho)).status === 401, "anonymous workspace remains refused");
  await fixture_chirho.start_browser_chirho(true, 2);

  phase_chirho = "challenge-refusal-chirho";
  const first_chirho = user_chirho();
  await follow_auth_link_chirho(page_chirho, 'nav a[href="/auth-chirho/login-chirho"]', login_chirho);
  await follow_auth_link_chirho(page_chirho, 'main a[href="/auth-chirho/register-chirho"]', registration_chirho);
  // Capture only the untouched blank form; no submitted name, password or visible token.
  await form_fields_chirho(page_chirho, "/auth-chirho/register-chirho", ["kind_chirho", "username_chirho", "credential_chirho", "human_check_chirho", "human_check_token_chirho"]);
  await page_chirho.screenshot({ path: join(output_chirho, "served-signup-chirho.png"), fullPage: true });
  const rejected_token_chirho = await fill_signup_chirho(page_chirho, first_chirho);
  await page_chirho.locator('[name="human_check_chirho"]').fill("999999");
  const refusal_chirho = await submit_chirho(page_chirho, "/auth-chirho/register-chirho");
  check_chirho(refusal_chirho.status() >= 400 && refusal_chirho.status() < 500, "wrong challenge is refused by canonical auth");
  await protected_html_chirho(page_chirho, refusal_chirho);
  check_chirho(await page_chirho.locator('[role="alert"]:visible').count() === 1 && await page_chirho.locator('form').count() === 0,
    "registration refusal is readable and does not silently resubmit credentials");
  await Promise.all([page_chirho.waitForNavigation({ waitUntil: "networkidle" }),
    page_chirho.locator(`main a[href="/auth-chirho/register-chirho"]`).click()]);

  phase_chirho = "browser-signup-chirho";
  const fresh_token_chirho = await fill_signup_chirho(page_chirho, first_chirho);
  check_chirho(fresh_token_chirho !== rejected_token_chirho, "explicit retry obtains a fresh challenge");
  const created_chirho = await submit_chirho(page_chirho, "/auth-chirho/register-chirho");
  check_chirho(created_chirho.status() === 200, "browser signup creates an account through the real engine");
  await protected_html_chirho(page_chirho, created_chirho);
  const replay_body_chirho = created_chirho.request().postData();
  if (!replay_body_chirho) throw Error("Missing actual registration form bytes");
  fixture_chirho.private_values_chirho.push(replay_body_chirho);
  check_chirho(await page_chirho.locator('form').count() === 0 && (await context_chirho.cookies()).length === 0,
    "success explains the explicit sign-in handoff without inventing a session");
  await Promise.all([page_chirho.waitForNavigation({ waitUntil: "networkidle" }), page_chirho.locator('main a[href="/auth-chirho/login-chirho"]').click()]);
  check_chirho(await page_chirho.locator('[name="username_chirho"]').count() === 1, "account handoff reaches the username form");
  const replay_chirho = await context_chirho.request.post(registration_chirho, { data: replay_body_chirho,
    headers: { "Content-Type": "application/x-www-form-urlencoded" }, maxRedirects: 0 });
  check_chirho(replay_chirho.status() >= 400 && !replay_chirho.headers()["set-cookie"], "spent challenge replay refuses without a session");

  phase_chirho = "signin-refusal-and-save-chirho";
  const known_notice_chirho = await login_user_chirho(page_chirho, first_chirho, true);
  const unknown_notice_chirho = await login_user_chirho(page_chirho, user_chirho(), true);
  check_chirho(known_notice_chirho === unknown_notice_chirho, "unknown username and wrong password have the same visible refusal");
  const old_cookie_chirho = await login_user_chirho(page_chirho, first_chirho);
  const first_headline_chirho = `Synthetic saved page ${crypto.randomUUID()}`;
  const headline_chirho = "Fresh bread for your neighbourhood";
  for (const [name_chirho, value_chirho] of Object.entries({ business_name_chirho: "Synthetic Bakery", headline_chirho: first_headline_chirho,
    summary_chirho: "A synthetic business used only for local signup acceptance.", service_1_title_chirho: "Fresh bread",
    service_1_body_chirho: "Baked each morning.", contact_heading_chirho: "Contact the bakery", contact_phone_chirho: "+1 202 555 0147" })) {
    await page_chirho.locator(`[name="${name_chirho}"]`).fill(value_chirho);
  }
  const save_chirho = await submit_chirho(page_chirho, "/app-chirho/perffection-workspace/edit-page-chirho");
  check_chirho(save_chirho.status() === 200 && await page_chirho.locator('[name="headline_chirho"]').inputValue() === first_headline_chirho,
    "self-created customer saves her page through the reviewed write route");

  phase_chirho = "signup-to-publication-chirho";
  observations_chirho.publication_chirho = await publication_chirho.run_chirho(page_chirho, first_headline_chirho, headline_chirho);
  await publication_chirho.check_status_layout_chirho(page_chirho, join(output_chirho, "served-publishing-chirho.png"));
  observations_chirho.served_css_sha256_chirho = await publication_chirho.verify_assets_chirho(page_chirho);
  await page_chirho.screenshot({ path: join(output_chirho, "served-published-page-chirho.png"), fullPage: true });
  check_chirho((await fetch(`${fixture_chirho.origin_chirho}/app-chirho/${publication_chirho.slug_chirho}/`)).status === 404,
    "self-published site exists only on the platform root, not the private tenant");
  fixture_chirho.run_chirho(["tenant-app-verify"], true);

  phase_chirho = "published-account-reentry-chirho";
  const help_chirho = await page_chirho.goto(`${origin_chirho}/app-chirho/perffection/help-chirho`);
  check_chirho(help_chirho?.status() === 200, "tenant session does not poison public Help through the two-root origin");
  await follow_auth_link_chirho(page_chirho, '#access-chirho a[href="/auth-chirho/login-chirho"]', login_chirho);
  await page_chirho.goto(`${origin_chirho}/auth-chirho/logout-chirho`);
  check_chirho(page_chirho.url() === login_chirho && !(await context_chirho.cookies()).some(cookie_chirho => cookie_chirho.name === "cairn_session_chirho"),
    "logout returns to username sign-in and clears the browser session");
  check_chirho((await fetch(workspace_chirho, { headers: { Cookie: `cairn_session_chirho=${old_cookie_chirho}` } })).status === 403,
    "logout revokes the old cookie rather than only deleting it in the browser");
  await login_user_chirho(page_chirho, first_chirho);
  check_chirho(await page_chirho.locator('[name="headline_chirho"]').inputValue() === headline_chirho, "repeat username sign-in retains the saved page");
  await page_chirho.goto(`${origin_chirho}/auth-chirho/logout-chirho`);

  phase_chirho = "capacity-and-isolation-chirho";
  const second_chirho = user_chirho();
  await visit_auth_chirho(page_chirho, registration_chirho);
  await fill_signup_chirho(page_chirho, second_chirho);
  check_chirho((await submit_chirho(page_chirho, "/auth-chirho/register-chirho")).status() === 200, "remaining capacity accepts a second customer after refused and replayed attempts");
  await login_user_chirho(page_chirho, second_chirho);
  check_chirho(await page_chirho.locator('[name="headline_chirho"]').inputValue() === "", "second self-created customer cannot read the first draft");
  await publication_chirho.verify_other_viewer_chirho(page_chirho);
  await page_chirho.goto(`${origin_chirho}/auth-chirho/logout-chirho`);
  await follow_auth_link_chirho(page_chirho, 'main a[href="/auth-chirho/register-chirho"]', registration_chirho, 503);
  check_chirho(await page_chirho.locator('form').count() === 0, "capacity-full page contains no unusable signup form");
  await page_chirho.screenshot({ path: join(output_chirho, "served-registration-full-chirho.png"), fullPage: true });
  await json_refusal_chirho(`${origin_chirho}/auth-chirho/human-check-chirho`, 503, "registration-cohort-full-chirho");
  await login_user_chirho(page_chirho, first_chirho);
  check_chirho(await page_chirho.locator('[name="headline_chirho"]').inputValue() === headline_chirho, "existing customer can sign in when registration is full");

  phase_chirho = "renderer-startup-refusals-chirho";
  const renderer_chirho = fixture_chirho.renderer_chirho!;
  for (const invalid_chirho of [{ ...renderer_chirho, digest_chirho: "0".repeat(64) }, { ...renderer_chirho, entry_chirho: "missing_entry_chirho" }]) {
    let refused_chirho = false;
    try { await fixture_chirho.start_chirho(false, 2, invalid_chirho); }
    catch (error_chirho) { if (!(error_chirho instanceof Error) || error_chirho.message !== "Owned signup service refused its configuration or exited") throw error_chirho; refused_chirho = true; }
    check_chirho(refused_chirho, "invalid configured renderer refuses service startup, never a fallback page");
  }
  await fixture_chirho.start_chirho(false, 2, { ...renderer_chirho, fuel_chirho: 1 });
  const trapped_chirho = await fetch(login_chirho);
  const trapped_html_chirho = await trapped_chirho.text();
  check_chirho(trapped_chirho.status === 500 && !/<form\b|name=["']principal_chirho/.test(trapped_html_chirho),
    "real product guest exhausting its configured fuel refuses the request without a principal-form fallback");
  await fixture_chirho.start_browser_chirho(false, 2);
  await context_chirho.clearCookies();
  await visit_auth_chirho(page_chirho, login_chirho);

  phase_chirho = "admission-refusal-chirho";
  await fixture_chirho.start_browser_chirho(true, 3);
  // Exercise the early gate with a finite burst on this owned loopback fixture only. This tests
  // the representation, not which internal limit fired or production per-client trust/rate tuning.
  let admitted_chirho = 0;
  for (; admitted_chirho < 64; admitted_chirho++) {
    const response_chirho = await fetch(`${origin_chirho}/auth-chirho/human-check-chirho`);
    const body_chirho = await response_chirho.json() as Record<string, unknown>;
    if (response_chirho.status === 429) {
      check_chirho(body_chirho.code_chirho === "too-many-requests-chirho", "early admission keeps the collapsed JSON refusal code");
      break;
    }
    check_chirho(response_chirho.status === 200 && body_chirho.ok_chirho === true
      && typeof body_chirho.human_check_token_chirho === "string", "configured display leaves successful JSON challenge issuance usable");
  }
  check_chirho(admitted_chirho > 0 && admitted_chirho < 64, "bounded owned burst reaches the auth admission refusal");
  await visit_auth_chirho(page_chirho, registration_chirho, 429);
  check_chirho(await page_chirho.locator('[role="alert"]:visible').count() === 1 && await page_chirho.locator('form').count() === 0,
    "early admission renders one branded actionless notice");
  await login_user_chirho(page_chirho, first_chirho);
  check_chirho(await page_chirho.locator('[name="headline_chirho"]').inputValue() === headline_chirho,
    "signup admission refusal does not block existing customer sign-in or lose her saved page");
  await page_chirho.goto(`${origin_chirho}/auth-chirho/logout-chirho`);

  phase_chirho = "registration-failure-window-chirho";
  await fixture_chirho.start_browser_chirho(true, 3);
  let limited_chirho = false;
  for (let attempt_chirho = 0; attempt_chirho < 8; attempt_chirho++) {
    await visit_auth_chirho(page_chirho, registration_chirho);
    await fill_signup_chirho(page_chirho, user_chirho());
    await page_chirho.locator('[name="human_check_chirho"]').fill("999999");
    const response_chirho = await submit_chirho(page_chirho, "/auth-chirho/register-chirho");
    check_chirho(response_chirho.status() === 400 || response_chirho.status() === 429, "repeated invalid challenges are refused");
    observations_chirho.registration_failure_status_chirho = response_chirho.status();
    if (response_chirho.status() === 429) {
      observations_chirho.registration_failure_window_chirho = {
        submission_count_chirho: attempt_chirho + 1,
        cache_control_chirho: (await response_chirho.allHeaders())["cache-control"] ?? null,
        product_shell_chirho: await page_chirho.locator('main h1').count() === 1 && await page_chirho.locator('nav').count() > 0,
        visible_notice_count_chirho: await page_chirho.locator('[role="alert"]:visible').count(),
      };
    }
    await protected_html_chirho(page_chirho, response_chirho);
    if (response_chirho.status() === 429) { limited_chirho = true; break; }
  }
  check_chirho(limited_chirho && await page_chirho.locator('[role="alert"]:visible').count() === 1
    && await page_chirho.locator('form').count() === 0, "registration failure-window refusal remains a readable branded page");
  check_chirho(errors_chirho.length === 0, "served product auth pages have no browser script/CSP errors");
  phase_chirho = "complete-local-browser-journey-chirho";
} catch (error_chirho) {
  failure_chirho = fixture_chirho.redact_chirho(error_chirho instanceof Error ? error_chirho.message : "Unexpected signup browser failure");
} finally {
  await browser_chirho?.close();
  await fixture_chirho.stop_browser_chirho();
  try { await fixture_chirho.verify_inputs_chirho(); }
  catch (error_chirho) { failure_chirho = fixture_chirho.redact_chirho(error_chirho instanceof Error ? error_chirho.message : "Consumed input changed"); }
  await evidence_chirho(failure_chirho ? "failed-chirho" : "passed-chirho");
  await rm(scratch_chirho, { recursive: true, force: true });
}
console.log(JSON.stringify({ result_chirho: failure_chirho ? "failed-chirho" : "passed-chirho", phase_chirho,
  checks_chirho: fixture_chirho.checks_chirho.length, failure_chirho }));
if (failure_chirho) process.exitCode = 1;
