// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// The figures gate's file arms (../figures-rehearsal-chirho.sh, arm 10): a real Chrome, signed in as her,
// chooses a CSV file in her figures page's file form and goes through review and a deliberate Save. The
// paste form beside it keeps her saved figures; nothing is emptied. Phase "files":
//   1-2. the file chosen while her saved figures stay in the paste form: the review names it, then her
//        Save is answered.
// Phase "uncertain", once the gate has read the head back:
//   3. through the drop proxy: a second file's Save reaches the engine but she hears nothing. The
//      request Chrome sent is kept, so the gate can repeat exactly that Save.
// It writes STRUCTURE only (counts, codes, versions, the file name the review shows) plus each page.
// usage: bun figures-upload-browser-chirho.ts <repository root> <base url> <proxy base url> <run dir> <files|uncertain>
// Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/her-figures-chirho.md
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
import type { Page as PageChirho } from "playwright-core";

const [repo_chirho, base_chirho, proxy_base_chirho, dir_chirho, phase_chirho] = process.argv.slice(2);
if (phase_chirho !== "files" && phase_chirho !== "uncertain") {
  throw new Error("usage: <repository root> <base url> <proxy base url> <run dir> <files|uncertain>");
}
const playwright_path_chirho = join(repo_chirho, "tools-chirho/browser-toolchain-chirho/node_modules/playwright-core/index.mjs");
const { chromium } = (await import(pathToFileURL(playwright_path_chirho).href)) as typeof import("playwright-core");
const FIGURES_PATH_CHIRHO = "/app-chirho/perffection-workspace/figures-chirho";
const SAVE_BUTTON_CHIRHO = 'form[action$="/save-figures-chirho"] button[type="submit"]';
// the file form's own Review button (the paste form beside it has one too)
const FILE_REVIEW_BUTTON_CHIRHO = 'form[enctype="multipart/form-data"] button[type="submit"]';

const cookie_chirho = readFileSync(join(dir_chirho, "a-cookie-chirho.txt"), "utf8").trim();
const split_chirho = cookie_chirho.indexOf("=");
const browser_chirho = await chromium.launch({ channel: "chrome", headless: true });
const context_chirho = await browser_chirho.newContext();
// the cookie belongs to the host, so the same session reaches the server and the proxy beside it
await context_chirho.addCookies([{ name: cookie_chirho.slice(0, split_chirho), value: cookie_chirho.slice(split_chirho + 1), url: base_chirho }]);
const page_chirho = await context_chirho.newPage();
const summary_chirho: Record<string, unknown> = {};

const count_chirho = (page_in_chirho: PageChirho, selector_chirho: string) => page_in_chirho.locator(selector_chirho).count();
const keep_chirho = async (name_chirho: string) => writeFileSync(join(dir_chirho, name_chirho), await page_chirho.content());
// her figures page, a file chosen in the file form, then its Review: the review page as she sees it
async function review_file_chirho(origin_chirho: string, file_chirho: string): Promise<void> {
  await page_chirho.goto(`${origin_chirho}${FIGURES_PATH_CHIRHO}`);
  await page_chirho.setInputFiles('input[name="csv_file_chirho"]', join(dir_chirho, file_chirho));
  await Promise.all([page_chirho.waitForURL(/review-figures-chirho$/), page_chirho.click(FILE_REVIEW_BUTTON_CHIRHO)]);
}

// 1 and 2: the file chosen while her saved figures stay in the paste form, the review, and her
// deliberate Save
async function files_phase_chirho(): Promise<void> {
  await page_chirho.goto(`${base_chirho}${FIGURES_PATH_CHIRHO}`);
  summary_chirho.box_held_her_figures_chirho = (await page_chirho.inputValue('textarea[name="csv_chirho"]')).includes("month,kind");
  await review_file_chirho(base_chirho, "upload-rye.csv");
  await keep_chirho("upload-review.html");
  summary_chirho.review_chirho = {
    file_name_chirho: (await page_chirho.locator(".workspace-figures-source-chirho strong").textContent()) ?? "",
    save_forms_chirho: await count_chirho(page_chirho, SAVE_BUTTON_CHIRHO),
    refusal_chirho: await count_chirho(page_chirho, ".workspace-errors-chirho"),
  };
  await Promise.all([page_chirho.waitForURL(/save-figures-chirho$/), page_chirho.click(SAVE_BUTTON_CHIRHO)]);
  await keep_chirho("upload-saved.html");
  summary_chirho.saved_chirho = {
    saved_notice_chirho: await count_chirho(page_chirho, ".workspace-figures-notice-saved-chirho"),
    version_chirho: await page_chirho.inputValue('input[name="expected_version_chirho"]'),
  };
}

// 3: through the drop proxy, the Save goes through and she hears nothing; Chrome's request is kept
async function uncertain_phase_chirho(): Promise<void> {
  await review_file_chirho(proxy_base_chirho, "upload-spelt.csv");
  const failed_chirho = page_chirho.waitForEvent("requestfailed", {
    predicate: (request_chirho) => request_chirho.url().endsWith("/save-figures-chirho"),
    timeout: 15_000,
  });
  const [request_chirho] = await Promise.all([failed_chirho, page_chirho.click(SAVE_BUTTON_CHIRHO).catch(() => undefined)]);
  writeFileSync(join(dir_chirho, "uncertain-upload-save-body.txt"), request_chirho.postData() ?? "");
  summary_chirho.uncertain_chirho = {
    failed_chirho: (request_chirho.failure()?.errorText ?? "").length > 0,
    method_chirho: request_chirho.method(),
    posted_bytes_chirho: (request_chirho.postData() ?? "").length,
  };
}

await (phase_chirho === "files" ? files_phase_chirho() : uncertain_phase_chirho());
await browser_chirho.close();
writeFileSync(join(dir_chirho, `upload-browser-${phase_chirho}-chirho.json`), `${JSON.stringify(summary_chirho, null, 2)}\n`);
