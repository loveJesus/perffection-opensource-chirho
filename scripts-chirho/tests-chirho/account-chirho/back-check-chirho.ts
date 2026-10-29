// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// Back, examined (gpt #26841). After her one-time key is shown, Back must never show it again. The Chrome driver presses
// Back, waits a bounded while for the page to stop changing, then examines the page as it then stands. After a refused
// Back, Chrome takes a moment to commit its own error page. Only two endings are trusted by name: a real move, or Chrome
// refusing to show a no-store answer again (net::ERR_CACHE_MISS). Any other navigation error is unknown. A page that
// never settles, or cannot be read, counts as showing the key. The controls run the same check on stand-in pages:
// a Back whose navigation is aborted while the key's page stays, and a Back with nowhere to go.
// Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/agent-api-chirho.md
import type { BrowserContext as BrowserContextChirho, Page as PageChirho } from "playwright-core";

export interface BackCheckChirho {
  outcome_chirho: string;
  shows_key_chirho: boolean;
  landed_chirho: string;
  landed_password_fields_chirho: number;
}

const SETTLE_TRIES_CHIRHO = 40;
const SETTLE_STEP_MS_CHIRHO = 250;
const CONTROL_ORIGIN_CHIRHO = "http://back-control-chirho.invalid";
const UNREADABLE_CHIRHO: BackCheckChirho = { outcome_chirho: "unknown", shows_key_chirho: true, landed_chirho: "unsettled", landed_password_fields_chirho: -1 };

// How Back ended: Chrome navigated away, refused to show a no-store page again (net::ERR_CACHE_MISS), or did not move.
// Any other error is unknown, and the gate fails it; it is never proof that the key is gone.
export function back_outcome_chirho(error_chirho: unknown, moved_chirho: boolean): string {
  if (error_chirho === undefined) return moved_chirho ? "navigated" : "stayed";
  const message_chirho = error_chirho instanceof Error ? error_chirho.message : String(error_chirho);
  return message_chirho.includes("net::ERR_CACHE_MISS") ? "cache-miss" : "unknown";
}

// The page once it stops changing: the same address and document on two reads a step apart, or nothing within the bound
async function settled_page_chirho(page_in_chirho: PageChirho): Promise<{ url_chirho: string; html_chirho: string } | undefined> {
  let last_chirho: { url_chirho: string; html_chirho: string } | undefined;
  for (let try_chirho = 0; try_chirho < SETTLE_TRIES_CHIRHO; try_chirho++) {
    await page_in_chirho.waitForTimeout(SETTLE_STEP_MS_CHIRHO);
    try {
      const now_chirho = { url_chirho: page_in_chirho.url(), html_chirho: await page_in_chirho.content() };
      if (last_chirho?.url_chirho === now_chirho.url_chirho && last_chirho.html_chirho === now_chirho.html_chirho) return now_chirho;
      last_chirho = now_chirho;
    } catch {
      last_chirho = undefined;
    }
  }
  return undefined;
}

// Back, then the settled page examined for the key. An address on `base_in_chirho` is named by its path, any other in
// full (Chrome's own error page is chrome-error://chromewebdata/).
export async function back_check_chirho(page_in_chirho: PageChirho, key_in_chirho: string, base_in_chirho: string): Promise<BackCheckChirho> {
  const before_chirho = page_in_chirho.url();
  let error_chirho: unknown = undefined;
  try {
    await page_in_chirho.goBack();
  } catch (caught_chirho) {
    error_chirho = caught_chirho ?? new Error("goBack failed without an error");
  }
  const settled_chirho = await settled_page_chirho(page_in_chirho);
  if (!settled_chirho) return UNREADABLE_CHIRHO;
  let landed_password_fields_chirho: number;
  try {
    landed_password_fields_chirho = await page_in_chirho.locator('input[type="password"]').count();
  } catch {
    return UNREADABLE_CHIRHO;
  }
  const url_chirho = settled_chirho.url_chirho;
  return {
    outcome_chirho: back_outcome_chirho(error_chirho, url_chirho !== before_chirho),
    shows_key_chirho: key_in_chirho.length === 0 || settled_chirho.html_chirho.includes(key_in_chirho),
    landed_chirho: url_chirho.startsWith(`${base_in_chirho}/`) ? new URL(url_chirho).pathname : url_chirho,
    landed_password_fields_chirho,
  };
}

// The controls: the classifier names only the endings it trusts, and two Backs that leave a stand-in key's page in
// place are each seen as showing it, under an ending the gate does not trust.
export async function back_controls_chirho(context_chirho: BrowserContextChirho): Promise<{ classifier_chirho: boolean; failing_chirho: BackCheckChirho; still_chirho: BackCheckChirho }> {
  const classifier_chirho = back_outcome_chirho(new Error("page.goBack: net::ERR_CACHE_MISS at x"), false) === "cache-miss"
    && back_outcome_chirho(new Error("page.goBack: net::ERR_ABORTED at x"), false) === "unknown"
    && back_outcome_chirho(undefined, true) === "navigated" && back_outcome_chirho(undefined, false) === "stayed";
  const stand_in_key_chirho = "0".repeat(64);
  const failing_page_chirho = await context_chirho.newPage();
  let refuse_before_chirho = false;
  await failing_page_chirho.route(`${CONTROL_ORIGIN_CHIRHO}/**`, (route_chirho) => {
    const shows_chirho = route_chirho.request().url().endsWith("/key-chirho");
    if (!shows_chirho && refuse_before_chirho) return route_chirho.abort("aborted");
    return route_chirho.fulfill({ status: 200, contentType: "text/html", headers: { "cache-control": "no-store" },
      body: shows_chirho ? `<p class="key"><code>${stand_in_key_chirho}</code></p>` : "<p>before</p>" });
  });
  await failing_page_chirho.goto(`${CONTROL_ORIGIN_CHIRHO}/before-chirho`);
  await failing_page_chirho.goto(`${CONTROL_ORIGIN_CHIRHO}/key-chirho`);
  refuse_before_chirho = true;
  const failing_chirho = await back_check_chirho(failing_page_chirho, stand_in_key_chirho, CONTROL_ORIGIN_CHIRHO);
  await failing_page_chirho.close();
  const still_page_chirho = await context_chirho.newPage();
  await still_page_chirho.setContent(`<p class="key"><code>${stand_in_key_chirho}</code></p>`);
  const still_chirho = await back_check_chirho(still_page_chirho, stand_in_key_chirho, CONTROL_ORIGIN_CHIRHO);
  await still_page_chirho.close();
  return { classifier_chirho, failing_chirho, still_chirho };
}
