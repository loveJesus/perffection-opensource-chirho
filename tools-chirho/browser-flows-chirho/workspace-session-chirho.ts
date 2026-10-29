// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// Workflows: product-delivery-chirho/reviewed-product-chirho.md and hosted-product-install-chirho.md.
// What a browser journey needs from whoever provisioned its accounts: a check ledger, redaction of
// private values and the issued synthetic users. A local root and a hosted origin both supply it.
import type { Locator as LocatorChirho, Page as PageChirho } from "playwright-core";

export interface SyntheticUserChirho { principal_chirho: string; credential_chirho: string }

// Chrome accepts the real Secure Set-Cookie on loopback; its separate API client did not send that
// cookie in the measured control, and its URL-filtered cookie lookup omits it too. So read the
// context's jar and select only this origin's host and the session name.
export async function session_cookies_chirho(page_chirho: PageChirho, origin_chirho: string): Promise<string[]> {
  const host_chirho = new URL(origin_chirho).hostname;
  return (await page_chirho.context().cookies()).filter(cookie_chirho =>
    cookie_chirho.name === "cairn_session_chirho" && cookie_chirho.domain === host_chirho).map(cookie_chirho => cookie_chirho.value);
}

export interface JourneyFixtureChirho {
  readonly private_values_chirho: string[];
  readonly users_chirho: SyntheticUserChirho[];
  check_chirho(condition_chirho: unknown, label_chirho: string): void;
  redact_chirho(value_chirho: string): string;
}

// Branded issued-account sign-in and the editor's real forms, shared by the local and hosted runners.
// `check_chirho` names the runner's own ledger, so each runner keeps its driver/shared split.
export class WorkspaceSessionChirho {
  readonly workspace_chirho: string;
  readonly signin_chirho: string;

  constructor(readonly fixture_chirho: JourneyFixtureChirho, origin_chirho: string,
    readonly check_chirho: (condition_chirho: unknown, label_chirho: string) => void) {
    this.workspace_chirho = `${origin_chirho}/app-chirho/perffection-workspace/`;
    this.signin_chirho = `${origin_chirho}/app-chirho/perffection/login-chirho`;
  }

  async submit_chirho(page_chirho: PageChirho, form_chirho: LocatorChirho, phase_chirho: string): Promise<{ bytes_chirho: string; html_chirho: string }> {
    const pending_chirho = page_chirho.waitForResponse(response_chirho => response_chirho.request().isNavigationRequest() && response_chirho.request().method() === "POST");
    pending_chirho.catch(() => undefined); // guarded: a failed action must not leave this rejection unhandled; awaiting it still raises
    await form_chirho.locator('button[type="submit"]').click();
    const response_chirho = await pending_chirho;
    await page_chirho.waitForLoadState("networkidle");
    this.check_chirho(response_chirho.status() === 200, `customer POST succeeds: ${phase_chirho}`);
    const bytes_chirho = response_chirho.request().postData();
    if (!bytes_chirho) throw Error("Missing actual form bytes");
    return { bytes_chirho, html_chirho: await response_chirho.text() };
  }

  async login_chirho(page_chirho: PageChirho, index_chirho: number): Promise<void> {
    await page_chirho.goto(this.signin_chirho);
    const user_chirho = this.fixture_chirho.users_chirho[index_chirho];
    if (!user_chirho) throw Error(`Missing synthetic user ${index_chirho}`);
    await page_chirho.locator('[name="principal_chirho"]').fill(user_chirho.principal_chirho);
    await page_chirho.locator('[name="credential_chirho"]').fill(user_chirho.credential_chirho);
    const pending_chirho = page_chirho.waitForResponse(response_chirho => response_chirho.request().method() === "POST");
    pending_chirho.catch(() => undefined); // guarded: a failed action must not leave this rejection unhandled; awaiting it still raises
    await page_chirho.locator('button[type="submit"]').click();
    this.check_chirho((await pending_chirho).status() === 303, "branded sign-in posts to real engine auth");
    await page_chirho.waitForURL(this.workspace_chirho);
    for (const cookie_chirho of await page_chirho.context().cookies()) this.fixture_chirho.private_values_chirho.push(cookie_chirho.value);
    this.check_chirho(await page_chirho.locator('form[action$="edit-page-chirho"]').count() === 1, "reviewed workspace serves the editor to its user");
  }

  async save_chirho(page_chirho: PageChirho, headline_chirho: string, phase_chirho: string,
    summary_chirho = "Synthetic bakery for a local reviewed publication proof."): Promise<void> {
    await page_chirho.goto(this.workspace_chirho);
    for (const [name_chirho, value_chirho] of Object.entries({ business_name_chirho: "Reviewed Bakery", headline_chirho,
      summary_chirho, service_1_title_chirho: "Fresh bread",
      service_1_body_chirho: "Baked every morning.", contact_heading_chirho: "Visit our counter", contact_phone_chirho: "+1 202 555 0147" })) {
      await page_chirho.locator(`[name="${name_chirho}"]`).fill(value_chirho);
    }
    await this.submit_chirho(page_chirho, page_chirho.locator('form[action$="edit-page-chirho"]'), phase_chirho);
  }
}
