// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// The Create journey's browser arm: a real Chrome, signed in as her, with no script, on the ENGINE's own form (Cairn
// K4). It opens the form, reads which actions it offers, ticks the ones asked for, enters her password again and
// creates the connection. The engine's answer shows the key once: the driver writes it ONLY into the caller's mode-700
// handoff directory, as a mode-600 file the caller removes on every exit. That is temporary private persistence, not
// none (gpt #26825). Nothing else holds it: no summary, screenshot, stdout or log. The summary is STRUCTURE only
// (statuses, the offered actions, counts, booleans, opaque key ids), never the key or the password.
// <start> is the form's path, opened directly (preparation by an explicit client id), or "link": open her Account page
// and follow its own link to the engine's form, then come back by the created page's return link (gpt #26825, #26841).
// Two callers share `create_connection_chirho`: the direct-engine gate (account-create-rehearsal-chirho.sh, through
// this file's command line, signed in by its cookie) and the release candidate through the two-root edge
// (connected-release-chirho.ts, signed in by the branded page in the same Chrome context).
// usage: bun account-create-browser-chirho.ts <repository root> <base url> <run dir> <start> <handoff dir> <actions>
// Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/agent-api-chirho.md
import { chmodSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
import type { BrowserContext as BrowserContextChirho } from "playwright-core";
import { back_check_chirho, back_controls_chirho } from "./back-check-chirho";

const KEY_SHAPE_CHIRHO = /^[0-9a-f]{64}$/;
export const ACCOUNT_PATH_CHIRHO = "/app-chirho/perffection-workspace/account-chirho";
const ENTRY_LINK_CHIRHO = 'a[href^="/auth-chirho/connection-new-chirho"]';
const DISCONNECT_FORMS_CHIRHO = 'form[action="/auth-chirho/key-revoke-chirho"]';
// The five guards an engine page carries, as one line (values only; none is secret): its cache control, whether its
// policy forbids framing and script (1 each), nosniff, the referrer policy and the frame option.
export const GUARDED_CHIRHO = "private, no-store|11|nosniff|no-referrer|deny";
export function guard_summary_chirho(headers_chirho: Record<string, string>): string {
  const csp_chirho = headers_chirho["content-security-policy"] ?? "";
  return [
    headers_chirho["cache-control"] ?? "",
    `${Number(csp_chirho.includes("frame-ancestors 'none'"))}${Number(csp_chirho.includes("script-src 'none'"))}`,
    headers_chirho["x-content-type-options"] ?? "",
    headers_chirho["referrer-policy"] ?? "",
    (headers_chirho["x-frame-options"] ?? "").toLowerCase(),
  ].join("|");
}

export interface CreateOptionsChirho {
  base_chirho: string;
  password_chirho: string;
  actions_chirho: string[];
  start_chirho: string;
  // where the key goes: a mode-600 file inside the caller's mode-700 handoff directory
  key_file_chirho: string;
  // the Back witness's controls run in the same Chrome first (gpt #26841); once per browser is enough
  controls_chirho: boolean;
}

// One connection created from her own signed-in context. The page it opens is closed at the end; her session stays.
export async function create_connection_chirho(context_chirho: BrowserContextChirho, options_chirho: CreateOptionsChirho): Promise<Record<string, unknown>> {
  const { base_chirho, password_chirho, start_chirho } = options_chirho;
  const summary_chirho: Record<string, unknown> = {};
  if (options_chirho.controls_chirho) {
    const controls_chirho = await back_controls_chirho(context_chirho);
    summary_chirho.back_classifier_control_chirho = controls_chirho.classifier_chirho;
    summary_chirho.failing_back_control_chirho = controls_chirho.failing_chirho;
    summary_chirho.still_back_control_chirho = controls_chirho.still_chirho;
  }
  const page_chirho = await context_chirho.newPage();
  try {
    let form_answer_chirho;
    if (start_chirho === "link") {
      await page_chirho.goto(`${base_chirho}${ACCOUNT_PATH_CHIRHO}`);
      summary_chirho.entry_links_chirho = await page_chirho.locator(ENTRY_LINK_CHIRHO).count();
      [form_answer_chirho] = await Promise.all([page_chirho.waitForNavigation(), page_chirho.click(ENTRY_LINK_CHIRHO)]);
    } else {
      form_answer_chirho = await page_chirho.goto(`${base_chirho}${start_chirho}`);
    }
    summary_chirho.form_status_chirho = form_answer_chirho?.status() ?? null;
    summary_chirho.form_path_chirho = new URL(page_chirho.url()).pathname;
    summary_chirho.offered_actions_chirho = (await page_chirho.locator('input[type="checkbox"][name="action_chirho"]').evaluateAll(
      (inputs_chirho: HTMLInputElement[]) => inputs_chirho.map(input_chirho => input_chirho.value),
    )).sort();
    summary_chirho.password_fields_chirho = await page_chirho.locator('input[type="password"][name="credential_chirho"]').count();
    // no form, no attempt: the caller's expectations then name what was measured (a closed road's 503, say), not a timeout
    if (summary_chirho.form_status_chirho !== 200 || summary_chirho.password_fields_chirho !== 1) return summary_chirho;
    for (const action_chirho of options_chirho.actions_chirho) await page_chirho.check(`input[type="checkbox"][name="action_chirho"][value="${action_chirho}"]`);
    await page_chirho.fill('input[type="password"][name="credential_chirho"]', password_chirho);
    const [created_answer_chirho] = await Promise.all([
      page_chirho.waitForNavigation(),
      page_chirho.click('form[action="/auth-chirho/connection-new-chirho"] button[type="submit"]'),
    ]);
    summary_chirho.created_status_chirho = created_answer_chirho?.status() ?? null;
    // the one-time answer's own guard headers, as Chrome received them
    summary_chirho.created_guards_chirho = guard_summary_chirho((await created_answer_chirho?.allHeaders()) ?? {});
    const key_chirho = ((await page_chirho.locator("p.key code").textContent()) ?? "").trim();
    const created_html_chirho = await page_chirho.content();
    summary_chirho.key_shape_ok_chirho = KEY_SHAPE_CHIRHO.test(key_chirho);
    summary_chirho.key_times_on_page_chirho = key_chirho ? created_html_chirho.split(key_chirho).length - 1 : 0;
    summary_chirho.password_on_page_chirho = created_html_chirho.includes(password_chirho);
    if (summary_chirho.key_shape_ok_chirho) {
      writeFileSync(options_chirho.key_file_chirho, `${key_chirho}\n`, { mode: 0o600 });
      chmodSync(options_chirho.key_file_chirho, 0o600);
    }
    // the way back the engine's page offers (app selection's successor): an ordinary link to her Account page, followed
    // by a click as she would; Back from there must not bring the key back (gpt #26841)
    const return_links_chirho = page_chirho.locator(`a[href="${ACCOUNT_PATH_CHIRHO}"]`);
    summary_chirho.return_links_chirho = await return_links_chirho.count();
    if (start_chirho === "link" && summary_chirho.return_links_chirho === 1) {
      await Promise.all([page_chirho.waitForNavigation(), return_links_chirho.click()]);
      summary_chirho.returned_path_chirho = new URL(page_chirho.url()).pathname;
      summary_chirho.returned_disconnects_chirho = await page_chirho.locator(DISCONNECT_FORMS_CHIRHO).count();
      // whose connections that list renders: each live row's own Disconnect form names its opaque key id (not a secret)
      summary_chirho.returned_key_ids_chirho = await page_chirho.locator(`${DISCONNECT_FORMS_CHIRHO} input[name="key_id_chirho"]`).evaluateAll(
        (inputs_chirho: HTMLInputElement[]) => inputs_chirho.map(input_chirho => input_chirho.value),
      );
    }
    // Back must not show the key again: where it ends is examined once the page stops changing, never assumed
    const back_chirho = await back_check_chirho(page_chirho, key_chirho, base_chirho);
    summary_chirho.back_outcome_chirho = back_chirho.outcome_chirho;
    summary_chirho.back_shows_key_chirho = back_chirho.shows_key_chirho;
    summary_chirho.back_landed_chirho = back_chirho.landed_chirho;
    summary_chirho.back_landed_password_fields_chirho = back_chirho.landed_password_fields_chirho;
  } finally {
    await page_chirho.close();
  }
  return summary_chirho;
}

// The direct-engine gate's command line: her session from the gate's cookie file, the key into its handoff directory.
if (import.meta.main) {
  const [repo_chirho, base_chirho, dir_chirho, start_chirho, handoff_chirho, actions_text_chirho] = process.argv.slice(2);
  if (!actions_text_chirho) throw new Error("usage: <repository root> <base url> <run dir> <start> <handoff dir> <actions>");
  const playwright_path_chirho = join(repo_chirho, "tools-chirho/browser-toolchain-chirho/node_modules/playwright-core/index.mjs");
  const { chromium } = (await import(pathToFileURL(playwright_path_chirho).href)) as typeof import("playwright-core");
  const cookie_chirho = readFileSync(join(dir_chirho, "a-cookie-chirho.txt"), "utf8").trim();
  const split_chirho = cookie_chirho.indexOf("=");
  const browser_chirho = await chromium.launch({ channel: "chrome", headless: true });
  let summary_chirho: Record<string, unknown> = {};
  try {
    const context_chirho = await browser_chirho.newContext({ javaScriptEnabled: false });
    await context_chirho.addCookies([{ name: cookie_chirho.slice(0, split_chirho), value: cookie_chirho.slice(split_chirho + 1), url: base_chirho }]);
    summary_chirho = await create_connection_chirho(context_chirho, { base_chirho, start_chirho, controls_chirho: true,
      password_chirho: readFileSync(join(dir_chirho, "a-cred-chirho.txt"), "utf8").trim(),
      actions_chirho: actions_text_chirho.split(","), key_file_chirho: join(handoff_chirho, "connection-key-chirho.txt") });
  } finally {
    // contexts first: Chrome takes seconds longer to close with a page still open
    for (const open_context_chirho of browser_chirho.contexts()) await open_context_chirho.close();
    await browser_chirho.close();
  }
  writeFileSync(join(dir_chirho, "account-create-browser-summary-chirho.json"), `${JSON.stringify(summary_chirho)}\n`);
}
