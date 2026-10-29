// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/public-signin-chirho.md.
// Real form submissions only: a manually selected fragment cannot satisfy these checks.
import { join } from "node:path";
import type { Page as PageChirho, Request as RequestChirho } from "playwright-core";

interface SigninRefusalFixtureChirho {
  page_chirho: PageChirho;
  base_chirho: string;
  signin_chirho: string;
  engine_login_chirho: string;
  workspace_chirho: string;
  principal_chirho: string;
  output_chirho: string;
  secrets_chirho: string[];
  check_chirho: (ok_chirho: boolean, label_chirho: string) => void;
}

export async function check_signin_refusals_chirho(fixture_chirho: SigninRefusalFixtureChirho): Promise<Record<string, number | boolean>> {
  const { page_chirho, base_chirho, signin_chirho, engine_login_chirho, workspace_chirho,
    principal_chirho, output_chirho, secrets_chirho, check_chirho } = fixture_chirho;
  const context_chirho = page_chirho.context();
  const invalid_fragment_chirho = "#refusal-invalid-credential-chirho";
  const rate_fragment_chirho = "#refusal-auth-rate-limited-chirho";
  const wrong_credential_chirho = `wrong-${crypto.randomUUID()}`;
  const unknown_principal_chirho = `user/${crypto.randomUUID().replaceAll("-", "")}${crypto.randomUUID().replaceAll("-", "")}`;
  secrets_chirho.push(wrong_credential_chirho, unknown_principal_chirho);

  async function check_notice_readable_chirho(label_chirho: string): Promise<void> {
    const visible_chirho = await page_chirho.locator('[role="alert"]:visible h2').evaluate(heading_chirho => {
      const box_chirho = heading_chirho.getBoundingClientRect();
      const hit_chirho = document.elementFromPoint(box_chirho.left + box_chirho.width / 2, box_chirho.top + box_chirho.height / 2);
      return { top_chirho: box_chirho.top, bottom_chirho: box_chirho.bottom, viewport_chirho: innerHeight,
        hit_chirho: hit_chirho !== null && heading_chirho.contains(hit_chirho) };
    });
    if (!visible_chirho.hit_chirho || visible_chirho.top_chirho < 0 || visible_chirho.bottom_chirho > visible_chirho.viewport_chirho) {
      await page_chirho.screenshot({ path: join(output_chirho, "signin-obscured-notice-chirho.png") });
    }
    check_chirho(visible_chirho.hit_chirho && visible_chirho.top_chirho >= 0
      && visible_chirho.bottom_chirho <= visible_chirho.viewport_chirho,
    `${label_chirho}: refusal heading is in view and unobscured (${JSON.stringify(visible_chirho)})`);
  }

  async function submit_refused_chirho(name_chirho: string, steer_chirho = false): Promise<string> {
    await page_chirho.locator('[name="principal_chirho"]').fill(name_chirho);
    await page_chirho.locator('[name="credential_chirho"]').fill(wrong_credential_chirho);
    if (steer_chirho) {
      await page_chirho.locator('[name="redirect_chirho"]').evaluate(input_chirho => {
        (input_chirho as HTMLInputElement).value = "/app-chirho/chosen-by-caller-chirho/";
      });
    }
    const [post_chirho, landing_chirho] = await Promise.all([
      page_chirho.waitForResponse(response_chirho => response_chirho.request().method() === "POST"
        && response_chirho.url() === base_chirho + engine_login_chirho),
      page_chirho.waitForNavigation({ waitUntil: "load" }),
      page_chirho.locator('button[type="submit"]').click(),
    ]);
    const headers_chirho = await post_chirho.allHeaders();
    check_chirho(post_chirho.status() === 303, "actual refused sign-in FORM returns 303 rather than an engine error page");
    const location_chirho = headers_chirho.location;
    check_chirho(location_chirho === engine_login_chirho + invalid_fragment_chirho
      || location_chirho === engine_login_chirho + rate_fragment_chirho,
    "refused sign-in uses a fixed canonical destination, never the caller success redirect");
    check_chirho(headers_chirho["set-cookie"] === undefined, "refused sign-in grants no session cookie");
    check_chirho(landing_chirho?.status() === 200 && page_chirho.url() === base_chirho + signin_chirho
      + location_chirho.slice(engine_login_chirho.length), "browser follows the engine return and display alias to the branded 200 page");
    await page_chirho.evaluate(() => document.fonts.ready);
    const visible_notices_chirho = await page_chirho.locator('[role="alert"]:visible').count();
    check_chirho(visible_notices_chirho === 1,
      `a real failed submission reveals exactly one explanation (observed ${visible_notices_chirho})`);
    check_chirho((await page_chirho.locator('[role="alert"]:visible').innerText()).trim().length > 0,
      "the returned explanation is not empty");
    check_chirho(await page_chirho.locator('[name="principal_chirho"]').inputValue() === ""
      && await page_chirho.locator('[name="credential_chirho"]').inputValue() === "",
    "returned sign-in fields contain neither submitted name nor password");
    check_chirho(await page_chirho.locator('form').getAttribute("action") === engine_login_chirho
      && await page_chirho.locator('[name="redirect_chirho"]').inputValue() === workspace_chirho,
    "returned form retains canonical auth and the fixed workspace success destination");
    const html_chirho = await page_chirho.content();
    check_chirho(!secrets_chirho.some(secret_chirho => html_chirho.includes(secret_chirho)
      || page_chirho.url().includes(secret_chirho)), "refused sign-in HTML and URL disclose no synthetic private values");
    check_chirho(!(await context_chirho.cookies()).some(cookie_chirho => cookie_chirho.name === "cairn_session_chirho"),
      "failed submissions leave the browser unauthenticated");
    await check_notice_readable_chirho("actual failed POST return");
    return location_chirho;
  }

  await page_chirho.setViewportSize({ width: 390, height: 844 });
  check_chirho((await page_chirho.goto(base_chirho + signin_chirho))?.status() === 200, "real refusal journey starts at the branded form");
  const wrong_location_chirho = await submit_refused_chirho(principal_chirho, true);
  check_chirho(wrong_location_chirho === engine_login_chirho + invalid_fragment_chirho, "wrong credential produces the invalid-credential return");
  const wrong_html_chirho = await page_chirho.content();
  await page_chirho.screenshot({ path: join(output_chirho, "signin-actual-refusal-chirho.png"), fullPage: true });
  const unknown_location_chirho = await submit_refused_chirho(unknown_principal_chirho);
  check_chirho(unknown_location_chirho === wrong_location_chirho && await page_chirho.content() === wrong_html_chirho,
    "unknown name and wrong password return identical destinations and product HTML");
  check_chirho((await context_chirho.request.get(base_chirho + workspace_chirho)).status() === 401,
    "refusal display did not open the private workspace");

  const json_chirho = await context_chirho.request.post(base_chirho + engine_login_chirho, {
    data: { principal_chirho, credential_chirho: wrong_credential_chirho }, maxRedirects: 0,
  });
  const json_body_chirho = await json_chirho.json();
  check_chirho(json_chirho.status() === 400 && json_chirho.headers().location === undefined
    && json_chirho.headers()["set-cookie"] === undefined && json_body_chirho.ok_chirho === false
    && typeof json_body_chirho.error_chirho === "string", "JSON credential refusal remains a 400 error envelope without a redirect or session");

  // Exhaust a separate synthetic name, not the account needed by Account/recovery acceptance.
  // Bounded actual submits, no production retry loop and no guessed wait time in product copy.
  let rate_location_chirho = "";
  for (let attempt_chirho = 0; attempt_chirho < 12; attempt_chirho++) {
    rate_location_chirho = await submit_refused_chirho(unknown_principal_chirho);
    if (rate_location_chirho === engine_login_chirho + rate_fragment_chirho) break;
  }
  check_chirho(rate_location_chirho === engine_login_chirho + rate_fragment_chirho, "real limiter exhaustion produces the distinct product rate-refusal notice");
  const rate_json_chirho = await context_chirho.request.post(base_chirho + engine_login_chirho, {
    data: { principal_chirho: unknown_principal_chirho, credential_chirho: wrong_credential_chirho }, maxRedirects: 0,
  });
  check_chirho(rate_json_chirho.status() === 429 && rate_json_chirho.headers().location === undefined
    && rate_json_chirho.headers()["set-cookie"] === undefined, "the same exhausted name is rate-refused on JSON without a session or redirect");
  const refresh_requests_chirho: string[] = [];
  // Record methods only, never request bodies or credentials.
  const observe_refresh_chirho = (request_chirho: RequestChirho): void => {
    if (request_chirho.url() === base_chirho + engine_login_chirho) refresh_requests_chirho.push(request_chirho.method());
  };
  page_chirho.on("request", observe_refresh_chirho);
  try {
    check_chirho((await page_chirho.reload())?.status() === 200
      && page_chirho.url() === base_chirho + signin_chirho + rate_fragment_chirho,
    "refresh after a refused POST reloads the explained sign-in display");
  } finally {
    page_chirho.off("request", observe_refresh_chirho);
  }
  check_chirho(!refresh_requests_chirho.includes("POST"), "refresh does not replay or recharge the failed sign-in POST");
  check_chirho(await page_chirho.locator('[role="alert"]:visible').count() === 1
    && await page_chirho.locator('[name="credential_chirho"]').inputValue() === "", "real rate notice remains readable with a blank retry form");
  await check_notice_readable_chirho("refresh after the rate refusal");
  await page_chirho.screenshot({ path: join(output_chirho, "signin-actual-rate-refusal-chirho.png"), fullPage: true });

  // The caller next signs in FROM THIS RETURNED FORM, without a replacement navigation.
  return { wrong_password_status_chirho: 303, wrong_password_product_template_chirho: true,
    unknown_name_same_public_response_chirho: true, real_rate_refusal_chirho: true };
}
