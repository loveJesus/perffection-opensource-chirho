// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/account-recovery-chirho.md.
// Reuse public-auth-check's producer and privacy guard. This module never builds or issues users.
import { join } from "node:path";
import type { Browser as BrowserChirho, BrowserContext as BrowserContextChirho, Page as PageChirho } from "playwright-core";

interface AccountCheckOptionsChirho {
  browser_chirho: BrowserChirho;
  base_chirho: string;
  principal_chirho: string;
  credential_chirho: string;
  output_chirho: string;
  secrets_chirho: string[];
  check_chirho: (ok_chirho: boolean, label_chirho: string) => void;
  prepare_context_chirho: (context_chirho: BrowserContextChirho) => Promise<void>;
}

export interface AccountCheckResultChirho {
  findings_chirho: string[];
  observations_chirho: Record<string, string | number | boolean>;
}

export async function check_account_chirho(options_chirho: AccountCheckOptionsChirho): Promise<AccountCheckResultChirho> {
  const { browser_chirho, base_chirho, principal_chirho, credential_chirho, output_chirho, secrets_chirho, check_chirho } = options_chirho;
  const workspace_chirho = "/app-chirho/perffection-workspace/";
  const account_chirho = workspace_chirho + "account-chirho";
  const signin_chirho = "/app-chirho/perffection/login-chirho";
  const login_chirho = "/auth-chirho/login-chirho";
  const change_chirho = "/auth-chirho/credential-change-chirho";
  const exhausted_fragment_chirho = "#refusal-credential-guess-exhausted-chirho";
  const new_credential_chirho = `New-fixture-${crypto.randomUUID()}-chirho`;
  const wrong_credential_chirho = `Wrong-fixture-${crypto.randomUUID()}-chirho`;
  secrets_chirho.push(new_credential_chirho, wrong_credential_chirho);
  const findings_chirho: string[] = [];
  const observations_chirho: Record<string, string | number | boolean> = {};
  const contexts_chirho: BrowserContextChirho[] = [];
  const errors_chirho: string[] = [];

  async function display_chirho(page_chirho: PageChirho, path_chirho: string): Promise<void> {
    const response_chirho = await page_chirho.goto(base_chirho + path_chirho);
    check_chirho(response_chirho?.status() === 200, `Account journey display: ${path_chirho} (HTTP ${response_chirho?.status() ?? "no response"})`);
    await page_chirho.evaluate(() => document.fonts.ready);
  }

  async function private_bytes_absent_chirho(page_chirho: PageChirho, label_chirho: string): Promise<void> {
    const html_chirho = await page_chirho.content();
    check_chirho(!secrets_chirho.some(secret_chirho => html_chirho.includes(secret_chirho) || page_chirho.url().includes(secret_chirho)), label_chirho);
  }

  async function blank_capture_chirho(page_chirho: PageChirho, name_chirho: string, full_page_chirho = true): Promise<void> {
    check_chirho(await page_chirho.locator('input[type="password"]').evaluateAll(inputs_chirho =>
      inputs_chirho.every(input_chirho => (input_chirho as HTMLInputElement).value === "")), `blank password fields before ${name_chirho}`);
    await private_bytes_absent_chirho(page_chirho, `no auth material before ${name_chirho}`);
    await page_chirho.screenshot({ path: join(output_chirho, name_chirho), fullPage: full_page_chirho });
  }

  async function notice_readable_chirho(page_chirho: PageChirho): Promise<boolean> {
    return page_chirho.locator('[role="alert"]:visible h2').evaluate(heading_chirho => {
      const box_chirho = heading_chirho.getBoundingClientRect();
      const hit_chirho = document.elementFromPoint(box_chirho.left + box_chirho.width / 2, box_chirho.top + box_chirho.height / 2);
      return box_chirho.top >= 0 && box_chirho.bottom <= innerHeight && hit_chirho !== null && heading_chirho.contains(hit_chirho);
    });
  }

  async function login_session_chirho(page_chirho: PageChirho, password_chirho: string, already_at_signin_chirho = false): Promise<string> {
    if (!already_at_signin_chirho) await display_chirho(page_chirho, signin_chirho);
    await page_chirho.locator('[name="principal_chirho"]').fill(principal_chirho);
    await page_chirho.locator('[name="credential_chirho"]').fill(password_chirho);
    const post_chirho = page_chirho.waitForResponse(response_chirho => response_chirho.url() === base_chirho + login_chirho && response_chirho.request().method() === "POST");
    await Promise.all([page_chirho.waitForURL(base_chirho + workspace_chirho), page_chirho.locator('button[type="submit"]').click()]);
    check_chirho((await post_chirho).status() === 303, "Account journey real sign-in succeeds");
    const session_chirho = (await page_chirho.context().cookies()).find(cookie_chirho => cookie_chirho.name === "cairn_session_chirho");
    if (!session_chirho) throw Error("Account journey did not receive a session");
    secrets_chirho.push(session_chirho.value);
    check_chirho(session_chirho.secure && session_chirho.httpOnly && session_chirho.sameSite === "Lax", "Account journey uses a protected session cookie");
    await private_bytes_absent_chirho(page_chirho, "signed-in editor does not echo auth material");
    return session_chirho.value;
  }

  async function submit_change_chirho(page_chirho: PageChirho, current_chirho: string, next_chirho: string) {
    const form_chirho = page_chirho.locator(`form[action="${change_chirho}"]`);
    await form_chirho.locator('[name="credential_chirho"]').fill(current_chirho);
    await form_chirho.locator('[name="new_credential_chirho"]').fill(next_chirho);
    const pending_chirho = page_chirho.waitForResponse(response_chirho => response_chirho.url() === base_chirho + change_chirho && response_chirho.request().method() === "POST");
    const [landing_chirho] = await Promise.all([
      page_chirho.waitForNavigation({ waitUntil: "load" }), form_chirho.locator('button[type="submit"]').click(),
    ]);
    return { post_chirho: await pending_chirho, landing_chirho };
  }

  async function cookie_status_chirho(context_chirho: BrowserContextChirho, value_chirho: string): Promise<number> {
    return (await context_chirho.request.get(base_chirho + account_chirho, {
      headers: { Cookie: `cairn_session_chirho=${value_chirho}` }, maxRedirects: 0,
    })).status();
  }

  try {
    for (let index_chirho = 0; index_chirho < 2; index_chirho++) {
      const context_chirho = await browser_chirho.newContext({ viewport: { width: 390, height: 844 } });
      await options_chirho.prepare_context_chirho(context_chirho);
      context_chirho.on("page", page_chirho => page_chirho.on("pageerror", error_chirho => errors_chirho.push(error_chirho.name)));
      contexts_chirho.push(context_chirho);
    }
    const [first_context_chirho, second_context_chirho] = contexts_chirho;
    const first_chirho = await first_context_chirho.newPage();
    const second_chirho = await second_context_chirho.newPage();
    const first_cookie_chirho = await login_session_chirho(first_chirho, credential_chirho);
    const second_cookie_chirho = await login_session_chirho(second_chirho, credential_chirho);
    check_chirho(first_cookie_chirho !== second_cookie_chirho, "two independent browser contexts hold different live sessions");

    // Save through the actual editor before changing credentials. The retained version, not an
    // identity-shaped success message, proves that a password change did not issue another user.
    const headline_chirho = "Account recovery fixture: keep this saved page";
    const edit_chirho = first_chirho.locator('form[action$="/edit-page-chirho"]');
    for (const [name_chirho, value_chirho] of Object.entries({
      business_name_chirho: "Recovery Test Bakery", headline_chirho,
      summary_chirho: "Synthetic page for a local password-change check.", service_1_title_chirho: "Fresh bread",
      service_1_body_chirho: "Baked each morning.", contact_heading_chirho: "Visit the bakery", contact_phone_chirho: "+1 202 555 0147",
    })) await edit_chirho.locator(`[name="${name_chirho}"]`).fill(value_chirho);
    const [saved_chirho] = await Promise.all([
      first_chirho.waitForNavigation({ waitUntil: "load" }), edit_chirho.locator('button[type="submit"]').click(),
    ]);
    check_chirho(saved_chirho?.status() === 200 && await first_chirho.locator('[name="expected_version_chirho"]').inputValue() === "1",
      "real editor saves a private version before credential change");
    await Promise.all([
      first_chirho.waitForURL(base_chirho + account_chirho, { waitUntil: "load" }),
      first_chirho.locator(`nav a[href="${account_chirho}"]`).click(),
    ]);
    check_chirho(new URL(first_chirho.url()).pathname === account_chirho, "real workspace navigation opens Account");
    const form_chirho = first_chirho.locator(`form[action="${change_chirho}"]`);
    check_chirho(await first_chirho.locator("form").count() === 1 && await form_chirho.count() === 1, "only the implemented password-change form is offered");
    check_chirho(await form_chirho.locator('[name="redirect_chirho"]').inputValue() === account_chirho && await form_chirho.locator('[name="principal_chirho"]').count() === 0,
      "Account posts a fixed return path and no claimed principal");
    check_chirho(await first_chirho.locator('[role="alert"]:visible').count() === 0, "Account begins without a spurious refusal");
    check_chirho(await form_chirho.locator('input[type="password"]').evaluateAll(inputs_chirho => inputs_chirho.every(input_chirho =>
      (input_chirho as HTMLInputElement).labels?.length === 1)), "both password controls have associated visible labels");
    await form_chirho.locator('button[type="submit"]').click();
    check_chirho(await form_chirho.locator("input:invalid").count() === 2 && new URL(first_chirho.url()).pathname === account_chirho,
      "empty Account form is stopped by native validation");
    await form_chirho.locator('[name="credential_chirho"]').fill(credential_chirho);
    await form_chirho.locator('[name="new_credential_chirho"]').fill("short");
    check_chirho(await form_chirho.locator('[name="new_credential_chirho"]').evaluate(input_chirho => !(input_chirho as HTMLInputElement).checkValidity()),
      "Account browser validation rejects a too-short new password");

    for (const width_chirho of [320, 390, 768, 1440]) {
      await first_chirho.setViewportSize({ width: width_chirho, height: 1000 });
      await display_chirho(first_chirho, account_chirho);
      const layout_chirho = await first_chirho.evaluate(() => ({
        scroll_chirho: document.documentElement.scrollWidth,
        overflowing_chirho: [...document.querySelectorAll("header *, main *")].filter(element_chirho => {
          const box_chirho = element_chirho.getBoundingClientRect();
          return box_chirho.width > 0 && (box_chirho.left < -1 || box_chirho.right > innerWidth + 1);
        }).map(element_chirho => `${element_chirho.tagName}.${element_chirho.className}`),
      }));
      if (layout_chirho.scroll_chirho > width_chirho + 1) await blank_capture_chirho(first_chirho, `account-overflow-${width_chirho}-chirho.png`, false);
      check_chirho(layout_chirho.scroll_chirho <= width_chirho + 1,
        `${width_chirho}px Account fits the viewport (measured ${layout_chirho.scroll_chirho}px; overflow ${layout_chirho.overflowing_chirho.join(", ") || "none"})`);
      check_chirho(await form_chirho.locator('input[type="password"], button').evaluateAll(controls_chirho => controls_chirho.every(control_chirho => {
        const box_chirho = control_chirho.getBoundingClientRect();
        return box_chirho.height >= 44 && box_chirho.left >= -1 && box_chirho.right <= innerWidth + 1;
      })), `${width_chirho}px Account controls remain touch-sized and in view`);
      if (width_chirho === 390 || width_chirho === 1440) await blank_capture_chirho(first_chirho, `account-${width_chirho}-chirho.png`);
    }
    await first_chirho.setViewportSize({ width: 390, height: 844 });
    await display_chirho(first_chirho, account_chirho);
    await first_chirho.evaluate(() => {
      document.documentElement.dir = "rtl";
      document.documentElement.style.fontSize = "200%";
      for (const element_chirho of document.querySelectorAll("main p, main small, main h1, main h2, main label > span, main button")) {
        element_chirho.textContent = `${element_chirho.textContent} — ${element_chirho.textContent}`;
      }
    });
    check_chirho(await first_chirho.evaluate(() => document.documentElement.scrollWidth <= innerWidth + 1), "Account expanded-text RTL stress does not introduce horizontal scrolling");
    await blank_capture_chirho(first_chirho, "account-expanded-rtl-chirho.png");
    await display_chirho(first_chirho, account_chirho);

    const wrong_chirho = await submit_change_chirho(first_chirho, wrong_credential_chirho, new_credential_chirho);
    check_chirho(wrong_chirho.post_chirho.status() === 303 && wrong_chirho.landing_chirho?.status() === 200, "wrong current password returns to a served Account page");
    check_chirho(new URL(first_chirho.url()).hash === "#refusal-credential-mismatch-chirho" && await first_chirho.locator('[role="alert"]:visible').count() === 1,
      "real wrong-password submission reveals exactly its targeted notice");
    await blank_capture_chirho(first_chirho, "account-wrong-current-chirho.png");
    await blank_capture_chirho(first_chirho, "account-wrong-current-viewport-chirho.png", false);
    const readable_chirho = await notice_readable_chirho(first_chirho);
    observations_chirho.mismatch_notice_readable_chirho = readable_chirho;
    if (readable_chirho) {
      check_chirho(true, "real mismatch fragment leaves its heading visible and unobscured");
    } else {
      findings_chirho.push("The mobile Account header obscures the targeted password-mismatch notice.");
      // A DOM-only counterfactual diagnoses positioning; it does not make the served CSS pass.
      await first_chirho.evaluate(() => {
        const header_chirho = document.querySelector<HTMLElement>(".workspace-page-chirho .site-nav-chirho");
        if (!header_chirho) throw Error("Missing Account header for positioning control");
        header_chirho.style.setProperty("position", "relative", "important");
      });
      await first_chirho.locator('[role="alert"]:visible').evaluate(alert_chirho => alert_chirho.scrollIntoView({ block: "start", behavior: "instant" }));
      observations_chirho.in_flow_notice_control_chirho = await notice_readable_chirho(first_chirho);
      check_chirho(observations_chirho.in_flow_notice_control_chirho === true, "DOM-only normal-flow control exposes the notice hidden by the header");
      await blank_capture_chirho(first_chirho, "account-notice-control-chirho.png", false);
      await display_chirho(first_chirho, account_chirho);
    }
    check_chirho(await cookie_status_chirho(first_context_chirho, first_cookie_chirho) === 200 && await cookie_status_chirho(second_context_chirho, second_cookie_chirho) === 200,
      "one wrong current password leaves both sessions usable");

    const accepted_chirho = await submit_change_chirho(first_chirho, credential_chirho, new_credential_chirho);
    // Browser Response.headers() omits security-sensitive headers. allHeaders() is required to
    // check Set-Cookie; never persist that map in evidence.
    const accepted_headers_chirho = await accepted_chirho.post_chirho.allHeaders();
    check_chirho(accepted_chirho.post_chirho.status() === 303 && accepted_headers_chirho.location === login_chirho,
      "real password change succeeds through canonical auth and redirects to sign-in");
    check_chirho(/Max-Age=0/i.test(accepted_headers_chirho["set-cookie"] ?? "") && !(await first_context_chirho.cookies()).some(cookie_chirho => cookie_chirho.name === "cairn_session_chirho"),
      "accepted change clears the submitting browser cookie");
    check_chirho(accepted_chirho.landing_chirho?.status() === 200 && new URL(first_chirho.url()).pathname === signin_chirho,
      "accepted change reaches the branded public sign-in page");
    check_chirho(await cookie_status_chirho(first_context_chirho, first_cookie_chirho) === 403 && await cookie_status_chirho(second_context_chirho, second_cookie_chirho) === 403,
      "accepted change revokes both real browser sessions on the server");
    await blank_capture_chirho(first_chirho, "account-change-return-chirho.png");
    const old_password_chirho = await first_context_chirho.request.post(base_chirho + login_chirho, {
      data: { principal_chirho, credential_chirho, redirect_chirho: workspace_chirho }, maxRedirects: 0,
    });
    check_chirho(old_password_chirho.status() === 400 && old_password_chirho.headers()["set-cookie"] === undefined, "old password cannot obtain a new session");
    const new_cookie_chirho = await login_session_chirho(first_chirho, new_credential_chirho);
    check_chirho(await first_chirho.locator('[name="headline_chirho"]').inputValue() === headline_chirho && await first_chirho.locator('[name="expected_version_chirho"]').inputValue() === "1",
      "new password opens the same saved page and version");
    const other_cookie_chirho = await login_session_chirho(second_chirho, new_credential_chirho);
    await display_chirho(first_chirho, account_chirho);

    // The browser follows the fifth POST's redirect automatically. That follow-up is already a
    // request with the revoked cookie; a curl check of just the 303 would miss the dead end.
    for (let attempt_chirho = 1; attempt_chirho <= 5; attempt_chirho++) {
      const attempt_result_chirho = await submit_change_chirho(first_chirho, wrong_credential_chirho, new_credential_chirho);
      check_chirho(attempt_result_chirho.post_chirho.status() === 303, `wrong-current attempt ${attempt_chirho} responds with a redirect`);
      if (attempt_chirho < 5) {
        check_chirho(attempt_result_chirho.landing_chirho?.status() === 200 && await first_chirho.locator('[role="alert"]:visible').count() === 1,
          `wrong-current attempt ${attempt_chirho} returns a usable retry form`);
      } else {
        observations_chirho.exhaustion_post_status_chirho = attempt_result_chirho.post_chirho.status();
        observations_chirho.exhaustion_landing_status_chirho = attempt_result_chirho.landing_chirho?.status() ?? 0;
        observations_chirho.exhaustion_landing_path_chirho = new URL(first_chirho.url()).pathname;
        observations_chirho.exhaustion_landing_fragment_chirho = new URL(first_chirho.url()).hash;
        observations_chirho.exhaustion_visible_alerts_chirho = await first_chirho.locator('[role="alert"]:visible').count();
        const reentry_chirho = first_chirho.locator(`form[action="${login_chirho}"], a[href="${signin_chirho}"], a[href="${login_chirho}"]`);
        observations_chirho.exhaustion_has_signin_chirho = await reentry_chirho.count() > 0;
        const exhausted_headers_chirho = await attempt_result_chirho.post_chirho.allHeaders();
        observations_chirho.exhaustion_cookie_cleared_chirho = /Max-Age=0/i.test(exhausted_headers_chirho["set-cookie"] ?? "")
          && !(await first_context_chirho.cookies()).some(cookie_chirho => cookie_chirho.name === "cairn_session_chirho");
        check_chirho(observations_chirho.exhaustion_cookie_cleared_chirho, "exhausting proof expires the revoked cookie in the real browser");
        check_chirho(exhausted_headers_chirho.location === login_chirho + exhausted_fragment_chirho,
          "exhausting proof redirects to the fixed public login explanation");
        const body_chirho = await first_chirho.content();
        observations_chirho.exhaustion_body_sha256_chirho = new Bun.CryptoHasher("sha256").update(body_chirho).digest("hex");
        await blank_capture_chirho(first_chirho, "account-exhaustion-return-chirho.png");
        if (attempt_result_chirho.landing_chirho?.status() !== 200 || !await reentry_chirho.count()
          || observations_chirho.exhaustion_visible_alerts_chirho !== 1
          || new URL(first_chirho.url()).pathname !== signin_chirho || new URL(first_chirho.url()).hash !== exhausted_fragment_chirho) {
          findings_chirho.push("The fifth wrong current password revokes the session but its browser return does not provide a served, explained sign-in path.");
        } else {
          check_chirho(await notice_readable_chirho(first_chirho), "automatic exhaustion return leaves its explanation visible and unobscured");
          check_chirho(await first_chirho.locator('[role="alert"]:visible p').innerText() !== "", "exhaustion notice has customer-facing explanation");
        }
      }
    }
    check_chirho(await cookie_status_chirho(first_context_chirho, new_cookie_chirho) === 403, "fifth failed proof revokes the attacking session");
    check_chirho(await cookie_status_chirho(second_context_chirho, other_cookie_chirho) === 200, "exhausting one session does not revoke the other session");
    await display_chirho(second_chirho, account_chirho);
    const other_wrong_chirho = await submit_change_chirho(second_chirho, wrong_credential_chirho, new_credential_chirho);
    check_chirho(other_wrong_chirho.landing_chirho?.status() === 200 && await cookie_status_chirho(second_context_chirho, other_cookie_chirho) === 200,
      "other session has its own current-password guess budget");
    check_chirho(new URL(first_chirho.url()).pathname === signin_chirho && await first_chirho.locator(`form[action="${login_chirho}"]`).count() === 1,
      "automatic exhaustion return already supplies the re-entry form without manual navigation");
    const fresh_cookie_chirho = await login_session_chirho(first_chirho, new_credential_chirho, true);
    check_chirho(fresh_cookie_chirho !== new_cookie_chirho, "the returned sign-in form obtains a new session after exhaustion");
    check_chirho(await first_chirho.locator('[name="headline_chirho"]').inputValue() === headline_chirho, "exhaustion and re-entry preserve saved content");
    await display_chirho(first_chirho, account_chirho);
    const fresh_wrong_chirho = await submit_change_chirho(first_chirho, wrong_credential_chirho, new_credential_chirho);
    check_chirho(fresh_wrong_chirho.landing_chirho?.status() === 200 && await cookie_status_chirho(first_context_chirho, fresh_cookie_chirho) === 200,
      "fresh login starts a usable new current-password guess budget");
    await private_bytes_absent_chirho(first_chirho, "final Account return contains no credential, principal or session bytes");
    check_chirho(await first_chirho.locator("script").count() === 0 && errors_chirho.length === 0, "Account templates have no scripts or browser exceptions");
    observations_chirho.successful_change_and_saved_content_chirho = true;
    observations_chirho.session_revocation_and_budget_isolation_chirho = true;
    return { findings_chirho, observations_chirho };
  } finally {
    for (const context_chirho of contexts_chirho) await context_chirho.close();
  }
}
