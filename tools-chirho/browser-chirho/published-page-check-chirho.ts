#!/usr/bin/env bun
// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// Native-template browser QA, not a Cairn/host/publication proof. Build the native example and
// prepare page-template first. No golden HTML/prose: checks behavior, layout and safe actions.
import { mkdir } from "node:fs/promises";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import type { BrowserType, ChromiumBrowser } from "playwright-core";

const root_chirho = resolve(import.meta.dir, "../..");
const output_chirho = resolve(process.env.PAGE_QA_OUTPUT_CHIRHO ?? `${root_chirho}/target/page-qa-chirho`);
const baseline_chirho = Bun.argv.includes("--baseline-chirho");
const playwright_path_chirho = process.env.PLAYWRIGHT_MODULE_CHIRHO;
if (!playwright_path_chirho) throw new Error("Set PLAYWRIGHT_MODULE_CHIRHO to an installed Playwright index.mjs; no install is performed.");
const { chromium: chromium_chirho } = await import(pathToFileURL(playwright_path_chirho).href) as { chromium: BrowserType<ChromiumBrowser> };
await mkdir(output_chirho, { recursive: true });

const email_chirho = `${"a".repeat(64)}@${"b".repeat(63)}.${"c".repeat(63)}.example`;
const ordinary_chirho = {
  schema_version_chirho: 2,
  business_name_chirho: "Good Earth Bakery",
  headline_chirho: "Good bread. A little slower.",
  summary_chirho: "Naturally leavened bread, pastries made by hand, and a warm welcome. We bake each morning for our neighbors. Come as you are.",
  services_chirho: [
    { title_chirho: "The daily loaf", body_chirho: "A crackling crust and a soft, open crumb. Pick up your sourdough while it is still warm." },
    { title_chirho: "Something sweet", body_chirho: "Seasonal pastries, slowly laminated and baked in small batches. A good reason to take a break." },
    { title_chirho: "Bread for your table", body_chirho: "Planning a family supper or a gathering? Tell us what you need and we will bake it for you." },
  ],
  contact_heading_chirho: "We will save you a loaf.",
  contact_phone_chirho: "+1 (555) 010-0100",
  contact_email_chirho: "hello@good-earth.example",
  contact_details_chirho: "Tuesday–Saturday · 7am–2pm\n12 Market Street, Example Town",
};
const long_chirho = {
  ...ordinary_chirho,
  business_name_chirho: "W".repeat(80),
  headline_chirho: "W".repeat(140),
  summary_chirho: "界".repeat(600),
  services_chirho: Array.from({ length: 3 }, () => ({ title_chirho: "W".repeat(72), body_chirho: "界".repeat(320) })),
  contact_heading_chirho: "W".repeat(80),
  contact_email_chirho: email_chirho,
  contact_details_chirho: "界".repeat(400),
};
// Combined CJK maxima exceed the document's total byte bound. Long individual fields below stay
// inside the real 8 KiB document limit; they must not pass by rendering the unavailable fallback.
long_chirho.summary_chirho = "W".repeat(600);
long_chirho.services_chirho = Array.from({ length: 3 }, () => ({ title_chirho: "W".repeat(72), body_chirho: "W".repeat(320) }));
long_chirho.contact_details_chirho = "W".repeat(400);

async function render_chirho(document_chirho: unknown, flags_chirho: string[] = []): Promise<string> {
  const child_chirho = Bun.spawn([`${root_chirho}/target/debug/examples/render_published_page_native_chirho`, ...flags_chirho], {
    stdin: new Blob([JSON.stringify({ app_params_chirho: document_chirho })]), stdout: "pipe", stderr: "pipe",
  });
  const [html_chirho, error_chirho, code_chirho] = await Promise.all([
    new Response(child_chirho.stdout).text(), new Response(child_chirho.stderr).text(), child_chirho.exited,
  ]);
  if (code_chirho !== 0) throw new Error(`native renderer failed (${code_chirho}): ${error_chirho}`);
  return html_chirho;
}

const cases_chirho: { name_chirho: string; html_chirho: string; service_count_chirho: number; private_chirho: boolean; rtl_chirho: boolean; invalid_chirho?: boolean }[] = [];
for (const count_chirho of [1, 2, 3]) {
  cases_chirho.push({ name_chirho: `services-${count_chirho}-chirho`, service_count_chirho: count_chirho, private_chirho: false, rtl_chirho: false,
    html_chirho: await render_chirho({ ...ordinary_chirho, services_chirho: ordinary_chirho.services_chirho.map((service_chirho, index_chirho) => index_chirho < count_chirho ? service_chirho : { title_chirho: "", body_chirho: "" }) }) });
}
cases_chirho.push({ name_chirho: "long-chirho", html_chirho: await render_chirho(long_chirho), service_count_chirho: 3, private_chirho: false, rtl_chirho: false });
if (!baseline_chirho) {
  cases_chirho.push({ name_chirho: "preview-chirho", html_chirho: await render_chirho(ordinary_chirho, ["--preview-chirho"]), service_count_chirho: 3, private_chirho: true, rtl_chirho: false });
  cases_chirho.push({ name_chirho: "rtl-chirho", html_chirho: await render_chirho(ordinary_chirho, ["--rtl-pseudolocale-chirho"]), service_count_chirho: 3, private_chirho: false, rtl_chirho: true });
  cases_chirho.push({ name_chirho: "invalid-chirho", html_chirho: await render_chirho({ ...ordinary_chirho, contact_email_chirho: "javascript:bad", contact_phone_chirho: "" }), service_count_chirho: 0, private_chirho: false, rtl_chirho: false, invalid_chirho: true });
}
const css_path_chirho = `${root_chirho}/apps-chirho/perffection-page-template-chirho/assets_chirho/page-chirho.css`;
const server_chirho = Bun.serve({ hostname: "127.0.0.1", port: 0,
  fetch(request_chirho) {
    const path_chirho = new URL(request_chirho.url).pathname;
    if (path_chirho.endsWith("/page-chirho.css")) return new Response(Bun.file(css_path_chirho), { headers: { "Content-Type": "text/css" } });
    if (path_chirho === "/favicon.ico") return new Response(null, { status: 204 });
    const case_chirho = cases_chirho.find((candidate_chirho) => path_chirho === `/${candidate_chirho.name_chirho}`);
    return case_chirho ? new Response(case_chirho.html_chirho, { headers: { "Content-Type": "text/html; charset=utf-8" } }) : new Response("Not found", { status: 404 });
  },
});
const results_chirho: { check_chirho: string; passed_chirho: boolean; detail_chirho?: unknown }[] = [];
function expect_chirho(check_chirho: string, passed_chirho: boolean, detail_chirho?: unknown): void {
  results_chirho.push({ check_chirho, passed_chirho, ...(!passed_chirho ? { detail_chirho } : {}) });
}
const browser_chirho = await chromium_chirho.launch({ channel: "chrome", headless: true });
const browser_errors_chirho: string[] = [];
try {
  const page_chirho = await browser_chirho.newPage();
  page_chirho.on("pageerror", (error_chirho: Error) => browser_errors_chirho.push(error_chirho.message));
  page_chirho.on("console", (message_chirho: { type(): string; text(): string }) => { if (message_chirho.type() === "error") browser_errors_chirho.push(message_chirho.text()); });
  for (const case_chirho of cases_chirho) {
    for (const width_chirho of [320, 390, 768, 1440]) {
      const label_chirho = `${case_chirho.name_chirho}/${width_chirho}`;
      await page_chirho.setViewportSize({ width: width_chirho, height: 900 });
      await page_chirho.goto(`http://127.0.0.1:${server_chirho.port}/${case_chirho.name_chirho}`);
      await page_chirho.evaluate(() => document.fonts.ready);
      const layout_chirho = await page_chirho.evaluate(() => {
        const viewport_chirho = document.documentElement.clientWidth;
        const overflow_chirho = [...document.querySelectorAll("body *")].filter((element_chirho) => {
          const rect_chirho = element_chirho.getBoundingClientRect();
          return rect_chirho.width > 0 && (rect_chirho.right > viewport_chirho + 1 || rect_chirho.left < -1) && getComputedStyle(element_chirho).position !== "fixed";
        }).map((element_chirho) => element_chirho.className || element_chirho.tagName);
        const grid_chirho = document.querySelector(".page-service-grid-chirho")?.getBoundingClientRect();
        const cards_chirho = [...document.querySelectorAll(".page-service-chirho")].map((element_chirho) => element_chirho.getBoundingClientRect());
        const used_width_chirho = cards_chirho.length ? Math.max(...cards_chirho.map((rect_chirho) => rect_chirho.right)) - Math.min(...cards_chirho.map((rect_chirho) => rect_chirho.left)) : 0;
        const contacts_chirho = [...document.querySelectorAll<HTMLAnchorElement>('a[href^="tel:"], a[href^="mailto:"]')];
        return { viewport_chirho, scroll_chirho: document.documentElement.scrollWidth, overflow_chirho,
          cards_chirho: cards_chirho.length, grid_width_chirho: grid_chirho?.width ?? 0, used_width_chirho,
          contacts_chirho: contacts_chirho.map((element_chirho) => ({ href_chirho: element_chirho.getAttribute("href"), height_chirho: element_chirho.getBoundingClientRect().height })),
          forms_chirho: document.querySelectorAll("form,script").length,
          heading_chirho: document.querySelectorAll("h1").length,
          direction_chirho: document.documentElement.dir, language_chirho: document.documentElement.lang,
          preview_chirho: !!document.querySelector(".page-preview-rail-chirho"),
          customer_text_chirho: document.body.textContent?.includes("Good Earth Bakery") ?? false,
          preview_return_chirho: document.querySelector(".page-preview-rail-chirho a")?.getAttribute("href") ?? null,
        };
      });
      expect_chirho(`${label_chirho}: no horizontal scroll`, layout_chirho.scroll_chirho === layout_chirho.viewport_chirho && layout_chirho.overflow_chirho.length === 0, layout_chirho);
      expect_chirho(`${label_chirho}: actual expected document`, layout_chirho.cards_chirho === case_chirho.service_count_chirho && layout_chirho.heading_chirho === 1);
      expect_chirho(`${label_chirho}: no browser form or script`, layout_chirho.forms_chirho === 0);
      expect_chirho(`${label_chirho}: preview rail only when private`, layout_chirho.preview_chirho === case_chirho.private_chirho);
      if (case_chirho.private_chirho) expect_chirho(`${label_chirho}: preview keeps the fixed workspace return`, layout_chirho.preview_return_chirho === "/app-chirho/perffection-workspace/");
      if (case_chirho.invalid_chirho) {
        expect_chirho(`${label_chirho}: invalid data never becomes a contact or customer disclosure`, layout_chirho.contacts_chirho.length === 0 && !layout_chirho.customer_text_chirho);
        continue;
      }
      expect_chirho(`${label_chirho}: services use available width`, Math.abs(layout_chirho.grid_width_chirho - layout_chirho.used_width_chirho) <= 3, layout_chirho);
      expect_chirho(`${label_chirho}: both contact targets usable`, layout_chirho.contacts_chirho.length === 2 && layout_chirho.contacts_chirho.every((contact_chirho: { height_chirho: number }) => contact_chirho.height_chirho >= 44));
      await page_chirho.keyboard.press("Tab");
      const first_link_chirho = await page_chirho.evaluate(() => document.activeElement?.getAttribute("href"));
      expect_chirho(`${label_chirho}: first keyboard stop skips navigation`, first_link_chirho === "#top-chirho", first_link_chirho);
      if (!baseline_chirho) {
        expect_chirho(`${label_chirho}: focused skip link is visible`, await page_chirho.evaluate(() => {
          const link_chirho = document.activeElement;
          if (!(link_chirho instanceof HTMLElement)) return false;
          const rect_chirho = link_chirho.getBoundingClientRect();
          return rect_chirho.top >= 0 && rect_chirho.height >= 44 && getComputedStyle(link_chirho).clipPath === "none";
        }));
        await page_chirho.keyboard.press("Enter");
        expect_chirho(`${label_chirho}: skip reaches main`, await page_chirho.evaluate(() => document.activeElement?.tagName === "MAIN"));
        expect_chirho(`${label_chirho}: catalog language and direction`, layout_chirho.language_chirho === (case_chirho.rtl_chirho ? "ar-XB" : "en") && layout_chirho.direction_chirho === (case_chirho.rtl_chirho ? "rtl" : "ltr"));
        await page_chirho.locator('nav a[href="#contact-chirho"]').click();
        expect_chirho(`${label_chirho}: section navigation reaches contact`, new URL(page_chirho.url()).hash === "#contact-chirho");
        if (width_chirho === 320) {
          await page_chirho.evaluate(() => { document.documentElement.style.fontSize = "200%"; });
          expect_chirho(`${label_chirho}: doubled text still reflows`, await page_chirho.evaluate(() => document.documentElement.scrollWidth === document.documentElement.clientWidth));
          await page_chirho.evaluate(() => { document.documentElement.style.removeProperty("font-size"); });
        }
      }
      if (["services-3-chirho", "preview-chirho", "rtl-chirho"].includes(case_chirho.name_chirho) && [390, 1440].includes(width_chirho)) {
        await page_chirho.evaluate(() => window.scrollTo(0, 0));
        await page_chirho.screenshot({ path: `${output_chirho}/${case_chirho.name_chirho}-${width_chirho}.png`, fullPage: true });
      }
    }
  }
  expect_chirho("no browser errors", browser_errors_chirho.length === 0, browser_errors_chirho);
} finally {
  await browser_chirho.close();
  await server_chirho.stop(true);
}
const failed_chirho = results_chirho.filter((result_chirho) => !result_chirho.passed_chirho);
const report_chirho = [
  "{", '  "scope_chirho": "native-template-chrome-local-chirho",', `  "baseline_chirho": ${baseline_chirho},`,
  '  "results_chirho": [', results_chirho.map((result_chirho) => `    ${JSON.stringify(result_chirho)}`).join(",\n"), "  ]", "}", "",
].join("\n");
await Bun.write(`${output_chirho}/result-chirho.json`, report_chirho);
console.log(JSON.stringify({ checks_chirho: results_chirho.length, failed_chirho, output_chirho }, null, 2));
if (failed_chirho.length) process.exitCode = 1;
