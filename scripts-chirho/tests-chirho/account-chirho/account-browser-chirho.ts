// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// The Account page's browser arm: a real Chrome, signed in as her, with no script of ours on the page. It opens her
// Account page at 1440 and 390 wide (a screenshot each, and whether anything overflows sideways), then presses one live
// connection's Disconnect button. The engine's 303 brings her back to the page at #notice-key-ended-chirho, where CSS
// :target reveals the notice; the row is then ended, with no button. The form it presses is the page's own rendered
// form, and it records that form's own hidden key id, so the caller can say whose connection the click ended (gpt
// #26864). It writes STRUCTURE only (counts, visibility, the fragment, that opaque id) plus each page.
// Two callers share `disconnect_chirho`: the Account and Create gates (account-keys-rehearsal-chirho.sh,
// account-create-rehearsal-chirho.sh, through this file's command line, signed in by a cookie) and the release
// candidate through the two-root edge (connected-release-chirho.ts, in her own signed-in context).
// usage: bun account-browser-chirho.ts <repository root> <base url> <run dir>
// Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/agent-api-chirho.md
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
import type { BrowserContext as BrowserContextChirho, Page as PageChirho } from "playwright-core";

const ACCOUNT_PATH_CHIRHO = "/app-chirho/perffection-workspace/account-chirho";
const DISCONNECT_FORM_CHIRHO = 'form[action="/auth-chirho/key-revoke-chirho"]';
const DISCONNECT_BUTTON_CHIRHO = `${DISCONNECT_FORM_CHIRHO} button[type="submit"]`;
const ENDED_ROW_CHIRHO = ".workspace-connections-ended-row-chirho";
const NOTICE_CHIRHO = "#notice-key-ended-chirho";

const state_chirho = async (page_in_chirho: PageChirho) => ({
  disconnect_buttons_chirho: await page_in_chirho.locator(DISCONNECT_BUTTON_CHIRHO).count(),
  ended_rows_chirho: await page_in_chirho.locator(ENDED_ROW_CHIRHO).count(),
  notice_visible_chirho: await page_in_chirho.locator(NOTICE_CHIRHO).isVisible(),
});

// `key_id_chirho` names the connection to end, by its own form's hidden key id; without one, the first live row's.
// Screenshots go to `out_chirho` with `label_chirho` in their names.
export async function disconnect_chirho(context_chirho: BrowserContextChirho, base_chirho: string, out_chirho: string,
  label_chirho = "account", key_id_chirho?: string): Promise<Record<string, unknown>> {
  const page_chirho = await context_chirho.newPage();
  // whether the page is wider than the window at this width, after the screenshot
  const at_width_chirho = async (width_chirho: number): Promise<boolean> => {
    await page_chirho.setViewportSize({ width: width_chirho, height: 900 });
    await page_chirho.goto(`${base_chirho}${ACCOUNT_PATH_CHIRHO}`);
    await page_chirho.screenshot({ path: join(out_chirho, `${label_chirho}-${width_chirho}-chirho.png`), fullPage: true });
    return page_chirho.evaluate(() => document.documentElement.scrollWidth > window.innerWidth);
  };
  const summary_chirho: Record<string, unknown> = {};
  try {
    summary_chirho.overflow_1440_chirho = await at_width_chirho(1440);
    summary_chirho.overflow_390_chirho = await at_width_chirho(390);
    summary_chirho.before_chirho = await state_chirho(page_chirho);
    // the connection her click ends: the hidden key id of the very form whose button is pressed
    const form_chirho = key_id_chirho === undefined ? page_chirho.locator(DISCONNECT_FORM_CHIRHO).first()
      : page_chirho.locator(DISCONNECT_FORM_CHIRHO).filter({ has: page_chirho.locator(`input[name="key_id_chirho"][value="${key_id_chirho}"]`) });
    summary_chirho.matching_forms_chirho = await form_chirho.count();
    summary_chirho.clicked_key_id_chirho = await form_chirho.locator('input[name="key_id_chirho"]').getAttribute("value");
    await Promise.all([page_chirho.waitForURL(/#notice-key-ended-chirho$/), form_chirho.locator('button[type="submit"]').click()]);
    await page_chirho.screenshot({ path: join(out_chirho, `${label_chirho}-after-390-chirho.png`), fullPage: true });
    writeFileSync(join(out_chirho, `${label_chirho}-browser-after.html`), await page_chirho.content());
    summary_chirho.after_chirho = { fragment_chirho: new URL(page_chirho.url()).hash, path_chirho: new URL(page_chirho.url()).pathname, ...(await state_chirho(page_chirho)) };
  } finally {
    await page_chirho.close();
  }
  return summary_chirho;
}

// The Account and Create gates' command line: her session from the gate's cookie file, the first live row's Disconnect.
if (import.meta.main) {
  const [repo_chirho, base_chirho, dir_chirho] = process.argv.slice(2);
  if (!dir_chirho) throw new Error("usage: <repository root> <base url> <run dir>");
  const playwright_path_chirho = join(repo_chirho, "tools-chirho/browser-toolchain-chirho/node_modules/playwright-core/index.mjs");
  const { chromium } = (await import(pathToFileURL(playwright_path_chirho).href)) as typeof import("playwright-core");
  const cookie_chirho = readFileSync(join(dir_chirho, "a-cookie-chirho.txt"), "utf8").trim();
  const split_chirho = cookie_chirho.indexOf("=");
  const browser_chirho = await chromium.launch({ channel: "chrome", headless: true });
  const context_chirho = await browser_chirho.newContext({ javaScriptEnabled: false });
  let summary_chirho: Record<string, unknown> = {};
  try {
    await context_chirho.addCookies([{ name: cookie_chirho.slice(0, split_chirho), value: cookie_chirho.slice(split_chirho + 1), url: base_chirho }]);
    summary_chirho = await disconnect_chirho(context_chirho, base_chirho, dir_chirho);
  } finally {
    // the context first: Chrome takes seconds longer to close with a page still open
    await context_chirho.close();
    await browser_chirho.close();
  }
  writeFileSync(join(dir_chirho, "account-browser-summary-chirho.json"), `${JSON.stringify(summary_chirho)}\n`);
}
