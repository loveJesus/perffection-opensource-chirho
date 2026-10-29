// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// Workflow: spec-chirho/workflows-chirho/product-delivery-chirho/customer-help-chirho.md.
// Called by the real two-root journey; this module never starts a server or builds an app.
import { join, resolve } from "node:path";
import type { Page as PageChirho } from "playwright-core";

export const HELP_PATH_CHIRHO = "/app-chirho/perffection/help-chirho";
type CheckChirho = (ok_chirho: boolean, label_chirho: string) => void;

async function layout_chirho(page_chirho: PageChirho, label_chirho: string, check_chirho: CheckChirho): Promise<void> {
  await page_chirho.evaluate(() => document.fonts.ready);
  const measure_chirho = await page_chirho.evaluate(() => ({
    fits_chirho: document.documentElement.scrollWidth <= innerWidth + 1,
    clipped_chirho: [...document.querySelectorAll("main *, .site-nav-chirho *")].filter(element_chirho => {
      const box_chirho = element_chirho.getBoundingClientRect();
      return box_chirho.width > 0 && box_chirho.height > 0 && (box_chirho.left < -1 || box_chirho.right > innerWidth + 1);
    }).map(element_chirho => element_chirho.className || element_chirho.tagName),
  }));
  check_chirho(measure_chirho.fits_chirho && measure_chirho.clipped_chirho.length === 0,
    `${label_chirho}: guide and navigation fit (${measure_chirho.clipped_chirho.join(", ") || "no clipped elements"})`);
  check_chirho(await page_chirho.locator('.help-topics-chirho a, summary, .help-guide-actions-chirho a').evaluateAll(elements_chirho =>
    elements_chirho.every(element_chirho => element_chirho.getBoundingClientRect().height >= 44)), `${label_chirho}: guide controls have touch-sized targets`);
}

export async function check_public_help_chirho(
  page_chirho: PageChirho,
  base_chirho: string,
  output_chirho: string,
  check_chirho: CheckChirho,
): Promise<string> {
  const url_chirho = base_chirho + HELP_PATH_CHIRHO;
  const response_chirho = await page_chirho.goto(url_chirho);
  check_chirho(response_chirho?.status() === 200, "public Help is a real served route before sign-in");
  const anonymous_html_chirho = await page_chirho.content();
  check_chirho(await page_chirho.locator("form, input, script").count() === 0, "Help neither collects input nor runs client scripts");
  const current_chirho = page_chirho.locator('.site-nav-chirho a[aria-current="page"]');
  check_chirho(await current_chirho.count() === 1 && await current_chirho.getAttribute("href") === HELP_PATH_CHIRHO,
    "shared public navigation marks only Help as current");
  check_chirho(await page_chirho.locator('h1').count() === 1, "guide has one primary heading");
  const request_chirho = page_chirho.context().request;
  const marker_chirho = "help-untrusted-request-probe-chirho";
  const tainted_chirho = await request_chirho.get(url_chirho + `?hint_chirho=${marker_chirho}`, {
    headers: { Authorization: `Bearer ${marker_chirho}`, Cookie: `cairn_session_chirho=${marker_chirho}` },
  });
  check_chirho(tainted_chirho.status() === 200 && await tainted_chirho.text() === await response_chirho!.text(),
    "query and invalid credentials cannot personalize or poison public Help");
  const body_chirho = await request_chirho.post(url_chirho, { data: marker_chirho, maxRedirects: 0 });
  check_chirho(body_chirho.status() >= 400 && body_chirho.status() < 500 && !(await body_chirho.text()).includes(marker_chirho),
    "Help refuses a request body without echoing its contents");

  // The native details control and document anchors must work without any JavaScript support.
  await page_chirho.emulateMedia({ reducedMotion: "reduce" });
  await page_chirho.keyboard.press("Tab");
  check_chirho(await page_chirho.locator('.help-skip-chirho').evaluate(element_chirho => element_chirho === document.activeElement),
    "first Help keyboard stop skips the shared header");
  await page_chirho.keyboard.press("Enter");
  check_chirho(await page_chirho.locator('.help-topics-chirho').evaluate(element_chirho => element_chirho === document.activeElement),
    "Help skip link focuses the topic index");
  const first_topic_chirho = page_chirho.locator('.help-topics-chirho a').first();
  const first_anchor_chirho = await first_topic_chirho.getAttribute("href");
  if (!first_anchor_chirho?.startsWith("#")) throw Error("Help topic is not a document anchor");
  await page_chirho.keyboard.press("Tab");
  await page_chirho.keyboard.press("Enter");
  const first_guide_chirho = page_chirho.locator(first_anchor_chirho);
  check_chirho(await first_guide_chirho.evaluate(element_chirho => element_chirho === document.activeElement),
    "keyboard topic link focuses its actual guide");
  check_chirho(await first_guide_chirho.evaluate(element_chirho => {
    const header_chirho = document.querySelector('.site-nav-chirho')!.getBoundingClientRect();
    return element_chirho.getBoundingClientRect().top >= Math.max(0, header_chirho.bottom) - 1;
  }), "topic heading is not hidden underneath the header");
  const summary_chirho = first_guide_chirho.locator('summary');
  await summary_chirho.focus();
  await page_chirho.keyboard.press("Enter");
  check_chirho(await first_guide_chirho.locator('details').evaluate(element_chirho => (element_chirho as HTMLDetailsElement).open),
    "keyboard opens the native Help answer");
  await page_chirho.keyboard.press("Space");
  check_chirho(!await first_guide_chirho.locator('details').evaluate(element_chirho => (element_chirho as HTMLDetailsElement).open),
    "keyboard closes the native Help answer");
  await first_guide_chirho.locator('a[href="#help-topics-chirho"]').click();
  check_chirho(await page_chirho.locator('.help-topics-chirho').evaluate(element_chirho => element_chirho === document.activeElement),
    "back-to-topics returns focus, not just scroll position");
  await page_chirho.emulateMedia({ reducedMotion: "no-preference" });

  for (const width_chirho of [320, 390, 768, 1440]) {
    await page_chirho.setViewportSize({ width: width_chirho, height: 1000 });
    await page_chirho.goto(url_chirho);
    await page_chirho.locator('summary').first().click();
    await layout_chirho(page_chirho, `${width_chirho}px Help`, check_chirho);
    if (width_chirho === 390 || width_chirho === 1440) {
      await page_chirho.screenshot({ path: join(output_chirho, `help-${width_chirho}-chirho.png`), fullPage: true });
      await page_chirho.evaluate(() => scrollTo(0, 0));
      await page_chirho.screenshot({ path: join(output_chirho, `help-entry-${width_chirho}-chirho.png`) });
    }
  }

  // Render the SAME Askama template through its native authoring binary. This is an explicit
  // locale/layout test, not a browser-selected language or a translated production release.
  const native_chirho = resolve(process.env.HELP_NATIVE_RENDERER_CHIRHO ?? join(import.meta.dir, "../../target/debug/examples/render_help_native_chirho"));
  const rendered_chirho = Bun.spawnSync([native_chirho, "--pseudolocale-chirho"], { stdout: "pipe", stderr: "pipe" });
  if (rendered_chirho.exitCode !== 0) throw Error("Build the product-only render_help_native_chirho example before this browser check");
  const pseudolocale_html_chirho = rendered_chirho.stdout.toString();
  const route_chirho = url_chirho + "?layout-chirho=rtl-chirho";
  await page_chirho.route(route_chirho, request_chirho => request_chirho.fulfill({ contentType: "text/html", body: pseudolocale_html_chirho }));
  try {
    for (const width_chirho of [390, 768, 1440]) {
      await page_chirho.setViewportSize({ width: width_chirho, height: 1000 });
      await page_chirho.goto(route_chirho);
      check_chirho(await page_chirho.locator('html').getAttribute("dir") === "rtl" && await page_chirho.locator('html').getAttribute("lang") === "ar-XB",
        `${width_chirho}px actual template carries pseudolocale metadata`);
      await page_chirho.evaluate(() => { document.documentElement.style.fontSize = "200%"; });
      await page_chirho.locator('summary').first().click();
      await page_chirho.screenshot({ path: join(output_chirho, `help-rtl-${width_chirho}-chirho.png`) });
      await layout_chirho(page_chirho, `${width_chirho}px Help pseudolocale with 200% text`, check_chirho);
      if (width_chirho === 390) await page_chirho.screenshot({ path: join(output_chirho, "help-expanded-rtl-chirho.png"), fullPage: true });
    }
  } finally {
    await page_chirho.unroute(route_chirho);
  }
  await page_chirho.goto(url_chirho);
  return anonymous_html_chirho;
}
