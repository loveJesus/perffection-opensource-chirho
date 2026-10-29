// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// The Back witness's own test (back-check-chirho.ts), in a real Chrome with script off, against a local server that
// answers as the engine's connection pages do: no-store everywhere, a form, a POST answer showing a key with a link away.
// It proves both endings the Create gate trusts on the shapes it meets:
// - the link start: Back from the page the answer's link reached, toward the POST answer, ends on Chrome's own error page;
// - the preparation start: Back straight from the answer reaches a fresh form.
// It also proves the controls. The Create gate runs it first, so each run proves its own witness.
// Run: bun test ./scripts-chirho/tests-chirho/account-chirho/
// Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/agent-api-chirho.md
import { afterAll, beforeAll, expect, test } from "bun:test";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
import type { Browser as BrowserChirho, BrowserContext as BrowserContextChirho, Page as PageChirho } from "playwright-core";
import { back_check_chirho, back_controls_chirho } from "./back-check-chirho";

const REPO_CHIRHO = join(import.meta.dir, "../../..");
const KEY_CHIRHO = "ab".repeat(32);
const GUARDS_CHIRHO = { "content-type": "text/html", "cache-control": "private, no-store", "content-security-policy": "frame-ancestors 'none'; script-src 'none'" };
const FORM_CHIRHO = '<form method="post" action="/answer-chirho"><input type="password" name="credential_chirho"><button type="submit">Create</button></form>';
const ANSWER_CHIRHO = `<p class="key"><code>${KEY_CHIRHO}</code></p><p class="return"><a href="/account-chirho">Back to her account</a></p>`;

const server_chirho = Bun.serve({
  port: 0,
  hostname: "127.0.0.1",
  fetch(request_chirho) {
    const path_chirho = new URL(request_chirho.url).pathname;
    if (path_chirho === "/form-chirho") return new Response(FORM_CHIRHO, { headers: GUARDS_CHIRHO });
    if (path_chirho === "/answer-chirho" && request_chirho.method === "POST") return new Response(ANSWER_CHIRHO, { headers: GUARDS_CHIRHO });
    return new Response("<p>her account</p>", { headers: GUARDS_CHIRHO });
  },
});
const BASE_CHIRHO = `http://127.0.0.1:${server_chirho.port}`;
// launching and closing Chrome can take more than bun's five-second default on a busy machine
const BOUND_MS_CHIRHO = 60_000;
let browser_chirho: BrowserChirho;
let context_chirho: BrowserContextChirho;

beforeAll(async () => {
  const playwright_chirho = join(REPO_CHIRHO, "tools-chirho/browser-toolchain-chirho/node_modules/playwright-core/index.mjs");
  const { chromium } = (await import(pathToFileURL(playwright_chirho).href)) as typeof import("playwright-core");
  browser_chirho = await chromium.launch({ channel: "chrome", headless: true });
  context_chirho = await browser_chirho.newContext({ javaScriptEnabled: false });
}, BOUND_MS_CHIRHO);
// the context first: Chrome takes seconds longer to close with a page still open
afterAll(async () => {
  await context_chirho?.close();
  await browser_chirho?.close();
  server_chirho.stop(true);
}, BOUND_MS_CHIRHO);

// her form, submitted: the page then shows the one-time answer
async function answered_page_chirho(): Promise<PageChirho> {
  const page_chirho = await context_chirho.newPage();
  await page_chirho.goto(`${BASE_CHIRHO}/form-chirho`);
  await page_chirho.fill('input[name="credential_chirho"]', "a password");
  await Promise.all([page_chirho.waitForNavigation(), page_chirho.click('button[type="submit"]')]);
  expect(await page_chirho.content()).toContain(KEY_CHIRHO);
  return page_chirho;
}

test("the link start: Back toward the no-store POST answer ends on Chrome's own error page, without the key", async () => {
  const page_chirho = await answered_page_chirho();
  try {
    await Promise.all([page_chirho.waitForNavigation(), page_chirho.click('a[href="/account-chirho"]')]);
    expect(await back_check_chirho(page_chirho, KEY_CHIRHO, BASE_CHIRHO)).toEqual({
      outcome_chirho: "cache-miss", shows_key_chirho: false, landed_chirho: "chrome-error://chromewebdata/", landed_password_fields_chirho: 0,
    });
  } finally {
    await page_chirho.close();
  }
}, BOUND_MS_CHIRHO);

test("the preparation start: Back from the answer reaches a fresh form, without the key", async () => {
  const page_chirho = await answered_page_chirho();
  try {
    expect(await back_check_chirho(page_chirho, KEY_CHIRHO, BASE_CHIRHO)).toEqual({
      outcome_chirho: "navigated", shows_key_chirho: false, landed_chirho: "/form-chirho", landed_password_fields_chirho: 1,
    });
  } finally {
    await page_chirho.close();
  }
}, BOUND_MS_CHIRHO);

test("the controls: a failing Back and a Back with nowhere to go are each seen showing the stand-in key", async () => {
  const controls_chirho = await back_controls_chirho(context_chirho);
  expect(controls_chirho.classifier_chirho).toBe(true);
  expect([controls_chirho.failing_chirho.outcome_chirho, controls_chirho.failing_chirho.shows_key_chirho]).toEqual(["unknown", true]);
  expect([controls_chirho.still_chirho.outcome_chirho, controls_chirho.still_chirho.shows_key_chirho]).toEqual(["stayed", true]);
}, BOUND_MS_CHIRHO);
