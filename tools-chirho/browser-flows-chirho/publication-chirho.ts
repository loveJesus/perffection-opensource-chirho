// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// Workflows: product-delivery-chirho/reviewed-product-chirho.md, invite-beta-chirho/signup-chirho.md
// and product-delivery-chirho/hosted-product-install-chirho.md.
// A browser journey, not a second publication implementation. Both account fixtures and the hosted
// runner consume it; the observer decides how an applier outcome is seen.
import { join } from "node:path";
import type { Locator as LocatorChirho, Page as PageChirho } from "playwright-core";
import { control_and_label_visible_chirho, tab_to_chirho } from "./form-keyboard-chirho";
import type { PublicationObserverChirho, TickChirho } from "./publication-observer-chirho";
import { type JourneyFixtureChirho, session_cookies_chirho } from "./workspace-session-chirho";

export class PublicationJourneyChirho {
  readonly workspace_chirho: string;
  readonly status_chirho: string;
  readonly live_chirho: string;
  readonly address_observations_chirho: Record<string, unknown> = {};

  constructor(readonly fixture_chirho: JourneyFixtureChirho, readonly origin_chirho: string, readonly slug_chirho: string,
    readonly observer_chirho: PublicationObserverChirho) {
    this.workspace_chirho = `${origin_chirho}/app-chirho/perffection-workspace/`;
    this.status_chirho = `${this.workspace_chirho}publication-status-chirho`;
    this.live_chirho = `${origin_chirho}/app-chirho/${slug_chirho}/`;
  }

  private check_chirho(condition_chirho: unknown, label_chirho: string): void {
    this.fixture_chirho.check_chirho(condition_chirho, `publication: ${label_chirho}`);
  }

  private get local_chirho(): boolean {
    return this.observer_chirho.local_chirho;
  }

  // Raw read for hosted waits: it adds nothing to the check ledger, so polling cannot fail early.
  private async live_html_chirho(): Promise<string | undefined> {
    const response_chirho = await fetch(this.live_chirho, { signal: AbortSignal.timeout(10_000) });
    return response_chirho.status === 200 ? await response_chirho.text() : undefined;
  }

  // A state that lasts only until the next applier tick. Local: checked. Hosted: observed and
  // recorded, because the supervised timer may legitimately apply before it can be read.
  private async transient_chirho(condition_chirho: () => Promise<boolean>, label_chirho: string): Promise<void> {
    if (this.local_chirho) this.check_chirho(await condition_chirho(), label_chirho);
    else this.observer_chirho.transient_chirho.push({ label_chirho, observed_chirho: await condition_chirho() });
  }

  // Exactly one queued approval goes live. Local: one test-driven tick applies it. Hosted: the
  // supervised timer must make `live_chirho` true within the observer's bounded wait.
  private async applies_once_chirho(local_label_chirho: string, hosted_label_chirho: string,
    live_chirho: () => Promise<boolean>): Promise<void> {
    if (this.local_chirho) this.check_chirho((await this.tick_chirho()).applied_chirho === 1, local_label_chirho);
    else this.check_chirho(await this.observer_chirho.settled_chirho(live_chirho), hosted_label_chirho);
  }

  // Nothing new may apply. Local: the tick report says so. Hosted: wait one full timer period so a
  // wrongly queued intent would have applied; the caller's public-bytes checks are the evidence.
  private async applies_nothing_chirho(label_chirho: string): Promise<void> {
    if (this.local_chirho) this.check_chirho((await this.tick_chirho()).applied_chirho === 0, label_chirho);
    else {
      await this.observer_chirho.quiet_period_chirho();
      this.observer_chirho.unexercised_chirho.push(`${label_chirho} (applier report is host journal evidence)`);
    }
  }

  private form_chirho(page_chirho: PageChirho, kind_chirho: string): LocatorChirho {
    return page_chirho.locator(`form:has(input[name="kind_chirho"][value="${kind_chirho}"])`).first();
  }

  private async submit_chirho(page_chirho: PageChirho, form_chirho: LocatorChirho, keyboard_chirho = false): Promise<{ bytes_chirho: string; html_chirho: string }> {
    this.check_chirho(await form_chirho.evaluate(form_chirho => (form_chirho as HTMLFormElement).checkValidity()),
      "customer form meets native constraints without disabling them");
    const response_chirho = page_chirho.waitForResponse(response_chirho => response_chirho.request().isNavigationRequest()
      && response_chirho.request().method() === "POST");
    response_chirho.catch(() => undefined); // guarded: a failed action must not leave this rejection unhandled; awaiting it still raises
    const button_chirho = form_chirho.locator('button[type="submit"]');
    if (keyboard_chirho) await tab_to_chirho(page_chirho, button_chirho);
    await Promise.all([page_chirho.waitForNavigation({ waitUntil: "networkidle" }),
      keyboard_chirho ? page_chirho.keyboard.press("Enter") : button_chirho.click()]);
    const received_chirho = await response_chirho;
    this.check_chirho(received_chirho.status() === 200, "real customer form POST succeeds");
    const bytes_chirho = received_chirho.request().postData();
    if (!bytes_chirho) throw Error("Missing actual publication form bytes");
    return { bytes_chirho, html_chirho: await received_chirho.text() };
  }

  private tick_chirho(): Promise<TickChirho> {
    return this.observer_chirho.tick_chirho();
  }

  private usage_chirho(): Promise<string> {
    return this.observer_chirho.usage_chirho();
  }

  private async saved_is_live_chirho(page_chirho: PageChirho, version_chirho: string): Promise<void> {
    await page_chirho.goto(this.status_chirho);
    this.check_chirho(await this.form_chirho(page_chirho, "republish-chirho").count() === 0,
      `saved version ${version_chirho} already live: no redundant republish form`);
    this.check_chirho(await page_chirho.locator('.workspace-panel-chirho > .workspace-version-chirho strong').innerText() === version_chirho,
      "Publishing names the actual live version while the redundant offer is absent");
  }

  private async replay_chirho(page_chirho: PageChirho, approved_chirho: { bytes_chirho: string; html_chirho: string },
    intents_chirho: number, label_chirho: string): Promise<void> {
    const before_chirho = this.local_chirho ? await this.usage_chirho() : undefined;
    const sessions_chirho = await session_cookies_chirho(page_chirho, this.workspace_chirho);
    this.check_chirho(sessions_chirho.length === 1, `${label_chirho}: one browser session for the owned fixture`);
    const response_chirho = await fetch(`${this.workspace_chirho}publish-chirho`, {
      method: "POST", redirect: "manual", signal: AbortSignal.timeout(15_000),
      headers: { "Content-Type": "application/x-www-form-urlencoded", Cookie: `cairn_session_chirho=${sessions_chirho[0]}` },
      body: approved_chirho.bytes_chirho,
    });
    const html_chirho = await response_chirho.text();
    const response_hash_chirho = new Bun.CryptoHasher("sha256").update(html_chirho).digest("hex");
    const expected_hash_chirho = new Bun.CryptoHasher("sha256").update(approved_chirho.html_chirho).digest("hex");
    this.check_chirho(response_chirho.status === 200 && html_chirho === approved_chirho.html_chirho,
      `${label_chirho}: exact request replays its original response bytes (HTTP ${response_chirho.status}, received ${response_hash_chirho}, expected ${expected_hash_chirho})`);
    if (this.local_chirho) {
      const tick_chirho = await this.tick_chirho();
      this.check_chirho(tick_chirho.intents_seen_chirho === intents_chirho && tick_chirho.applied_chirho === 0
        && await this.usage_chirho() === before_chirho, `${label_chirho}: no extra intent, apply or quota charge`);
    } else {
      // The caller then checks that the public bytes did not move.
      await this.observer_chirho.quiet_period_chirho();
      this.observer_chirho.unexercised_chirho.push(`${label_chirho}: no extra intent or quota charge (host usage log and applier journal)`);
    }
  }

  async public_html_chirho(): Promise<string> {
    const response_chirho = await fetch(this.live_chirho);
    this.check_chirho(response_chirho.status === 200, "managed address serves an anonymous reader");
    const html_chirho = await response_chirho.text();
    this.check_chirho(this.fixture_chirho.redact_chirho(html_chirho) === html_chirho && !/user\/[a-f0-9]{64}/.test(html_chirho),
      "public HTML has no synthetic username, principal, credential or session markers");
    return html_chirho;
  }

  async run_chirho(page_chirho: PageChirho, first_headline_chirho: string, second_headline_chirho: string): Promise<Record<string, string>> {
    // Begin with her already-saved page. Follow the product navigation, then use its hidden
    // version/base/idempotency fields untouched. The runner only enters the public slug.
    await Promise.all([page_chirho.waitForNavigation({ waitUntil: "networkidle" }),
      page_chirho.locator('nav a[href="/app-chirho/perffection-workspace/publication-status-chirho"]').click()]);
    const first_form_chirho = this.form_chirho(page_chirho, "publish-chirho");
    this.check_chirho(await first_form_chirho.locator('[name="version_chirho"]').inputValue() === "1", "approval names saved version one");
    await first_form_chirho.locator('[name="slug_chirho"]').fill(this.slug_chirho);
    await this.submit_chirho(page_chirho, first_form_chirho);
    await this.transient_chirho(async () => (await fetch(this.live_chirho)).status === 404, "queued approval is not a live page");
    await this.applies_once_chirho("test-driven platform tick applies her first approval once",
      "supervised timer publishes her first approval", async () => (await this.live_html_chirho())?.includes(first_headline_chirho) ?? false);
    const first_chirho = await this.public_html_chirho();
    this.check_chirho(first_chirho.includes(first_headline_chirho), "public page contains exactly the approved saved headline");
    await this.saved_is_live_chirho(page_chirho, "1");
    this.check_chirho(await page_chirho.locator('form').count() === 0, "first live version has no redundant publish or current-version rollback");

    const stale_page_chirho = await page_chirho.context().newPage();
    try {
      await page_chirho.goto(this.workspace_chirho);
      const editor_pointer_chirho = (await page_chirho.locator('main a[href$="publication-status-chirho"]').innerText()).trim();
      this.check_chirho(await page_chirho.locator('form[action$="publish-chirho"]').count() === 0
        && await page_chirho.locator('main a[href$="publication-status-chirho"]').count() === 1,
      "editor points to Publishing without an approval form");
      await page_chirho.locator('[name="headline_chirho"]').fill(second_headline_chirho);
      await this.submit_chirho(page_chirho, page_chirho.locator('form[action$="edit-page-chirho"]'));
      this.check_chirho(await this.public_html_chirho() === first_chirho, "saving a new private draft leaves the public page unchanged");
      // Both tabs now see a genuinely available v2/base1 offer. Before the save there is none.
      await stale_page_chirho.goto(this.status_chirho);
      await page_chirho.goto(this.status_chirho);
      this.check_chirho(await this.form_chirho(page_chirho, "republish-chirho").locator('[name="version_chirho"]').inputValue() === "2",
        "saving a newer version restores the republish offer for that version");
      const republish_chirho = await this.submit_chirho(page_chirho, this.form_chirho(page_chirho, "republish-chirho"));
      const duplicate_chirho = await this.submit_chirho(stale_page_chirho, this.form_chirho(stale_page_chirho, "republish-chirho"));
      const identical_chirho = duplicate_chirho.bytes_chirho === republish_chirho.bytes_chirho
        && duplicate_chirho.html_chirho === republish_chirho.html_chirho;
      if (this.local_chirho) {
        this.check_chirho(identical_chirho && await this.usage_chirho() === "2/2",
          "second browser tab submits identical approval bytes and replays without another charge");
      } else {
        this.check_chirho(identical_chirho, "second browser tab submits identical approval bytes and receives the identical response");
        this.observer_chirho.unexercised_chirho.push("second-tab replay adds no quota charge (host usage log)");
      }
      await page_chirho.goto(this.status_chirho);
      await this.transient_chirho(async () => await page_chirho.locator('form').count() === 0
        && await page_chirho.locator('.workspace-publication-pending-chirho[role="status"]').isVisible(),
      "queued publication explains the pending state without another approval form");
      await this.transient_chirho(async () => await this.public_html_chirho() === first_chirho, "queued republish leaves the current public version unchanged");
      await this.applies_once_chirho("republish applies once", "supervised timer publishes the republish",
        async () => (await this.live_html_chirho())?.includes(second_headline_chirho) ?? false);
      const second_chirho = await this.public_html_chirho();
      this.check_chirho(second_chirho !== first_chirho && second_chirho.includes(second_headline_chirho), "republish updates the same managed address");
      await this.saved_is_live_chirho(page_chirho, "2");
      this.check_chirho(await this.form_chirho(page_chirho, "rollback-chirho").count() === 1, "live version two retains a rollback offer");
      await this.replay_chirho(page_chirho, republish_chirho, 2, "retry after apply");
      this.check_chirho(await this.public_html_chirho() === second_chirho, "retry after apply leaves version two live");

      // Explicitly save the same document as v3. It is a new private version, not a new public
      // page yet. Capture v3/base2 before another tab rolls back; no hidden field is fabricated.
      await page_chirho.goto(this.workspace_chirho);
      await this.submit_chirho(page_chirho, page_chirho.locator('form[action$="edit-page-chirho"]'));
      this.check_chirho(await page_chirho.locator('form[action$="edit-page-chirho"] [name="expected_version_chirho"]').inputValue() === "3",
        "explicit save appends version three even with unchanged document bytes");
      await stale_page_chirho.goto(this.status_chirho);
      const fresh_stale_form_chirho = this.form_chirho(stale_page_chirho, "republish-chirho");
      this.check_chirho(await fresh_stale_form_chirho.locator('[name="version_chirho"]').inputValue() === "3",
        "unsubmitted browser form approves the new saved head");
      await page_chirho.goto(this.status_chirho);
      const rollback_chirho = await this.submit_chirho(page_chirho, this.form_chirho(page_chirho, "rollback-chirho"));
      if (this.local_chirho) {
        this.check_chirho((await this.tick_chirho()).applied_chirho === 1 && await this.public_html_chirho() === first_chirho,
          "rollback restores byte-identical version one");
      } else {
        this.check_chirho(await this.observer_chirho.settled_chirho(async () => await this.live_html_chirho() === first_chirho),
          "supervised timer rollback restores byte-identical version one");
      }

      const before_stale_chirho = this.local_chirho ? await this.usage_chirho() : undefined;
      const refused_chirho = await this.submit_chirho(stale_page_chirho, fresh_stale_form_chirho);
      this.check_chirho(refused_chirho.bytes_chirho !== republish_chirho.bytes_chirho
        && refused_chirho.bytes_chirho !== rollback_chirho.bytes_chirho,
      "stale approval is new request bytes, not an exact replay");
      const notice_chirho = stale_page_chirho.locator('.workspace-errors-chirho:visible');
      const visible_notice_count_chirho = await notice_chirho.count();
      const all_notice_count_chirho = await stale_page_chirho.locator('.workspace-errors-chirho').count();
      const readable_chirho = visible_notice_count_chirho === 1 && await notice_chirho.isVisible()
        && (await notice_chirho.innerText()).trim().length > 0
        && (await stale_page_chirho.locator('main .workspace-lede-chirho').innerText()).trim().length > 0;
      if (this.local_chirho) {
        const after_stale_chirho = await this.usage_chirho();
        const tick_chirho = await this.tick_chirho();
        this.check_chirho(readable_chirho,
          `moved-base refusal is readable (${visible_notice_count_chirho} visible of ${all_notice_count_chirho} notice panels)`);
        this.check_chirho(after_stale_chirho === before_stale_chirho,
          "new approval on a moved base adds no quota charge");
        this.check_chirho(tick_chirho.intents_seen_chirho === 3 && tick_chirho.applied_chirho === 0,
          "new approval on a moved base admits and applies no extra intent");
      } else {
        this.check_chirho(readable_chirho, "new approval on a moved base is refused readably");
        await this.observer_chirho.quiet_period_chirho();
        this.observer_chirho.unexercised_chirho.push("stale approval adds neither an intent nor a quota charge (host usage log and applier journal)");
      }
      this.check_chirho(await this.public_html_chirho() === first_chirho, "refused stale approval keeps the rolled-back page live");
      await this.replay_chirho(page_chirho, republish_chirho, 3, "old approval retry after head and base moved");
      this.check_chirho(await this.public_html_chirho() === first_chirho, "exact old approval cannot republish after a later rollback");

      await page_chirho.goto(this.status_chirho);
      await this.submit_chirho(page_chirho, this.form_chirho(page_chirho, "republish-chirho"));
      if (this.local_chirho) {
        this.check_chirho((await this.tick_chirho()).applied_chirho === 1 && await this.public_html_chirho() === second_chirho,
          "fresh approval publishes version three with byte-identical version-two content");
      } else {
        this.check_chirho(await this.observer_chirho.settled_chirho(async () => await this.live_html_chirho() === second_chirho),
          "supervised timer publishes version three with byte-identical version-two content");
      }
      await this.replay_chirho(page_chirho, rollback_chirho, 4, "late rollback retry");
      if (this.local_chirho) this.check_chirho(await this.usage_chirho() === "4/4", "only four intentional publication approvals consumed quota");
      else this.observer_chirho.unexercised_chirho.push("only four intentional publication approvals consumed quota (host usage log)");
      this.check_chirho(await this.public_html_chirho() === second_chirho, "late retry cannot reverse a later approval");
      this.observer_chirho.verify_chirho();
      await this.saved_is_live_chirho(page_chirho, "3");
      return { first_sha256_chirho: new Bun.CryptoHasher("sha256").update(first_chirho).digest("hex"),
        second_sha256_chirho: new Bun.CryptoHasher("sha256").update(second_chirho).digest("hex"),
        live_route_chirho: new URL(this.live_chirho).pathname,
        usage_chirho: this.local_chirho ? await this.usage_chirho() : "host usage log, not observable over HTTP",
        saved_version_chirho: "3", live_version_chirho: "3", editor_pointer_chirho,
        stale_approval_chirho: "unsubmitted v3/base2 form after rollback; hidden fields untouched",
        applier_chirho: this.local_chirho ? "test-driven local ticks, not a hosted timer" : "supervised host timer, observed over HTTP" };
    } finally { await stale_page_chirho.close(); }
  }

  async verify_other_viewer_chirho(page_chirho: PageChirho): Promise<void> {
    await page_chirho.goto(this.status_chirho);
    this.check_chirho(!(await page_chirho.locator('main').innerText()).includes(this.slug_chirho)
      && await page_chirho.locator('form').count() === 0, "second customer sees neither the first draft nor its publication history/forms");
  }

  // A second saved-page owner corrects genuine address refusals; no fabricated response or
  // hidden approval field. The first owner's public bytes must survive both attempts unchanged.
  async run_address_recovery_chirho(page_chirho: PageChirho, reserved_slug_chirho: string,
    taken_slug_chirho: string, output_chirho: string, exercise_taken_chirho = true): Promise<Record<string, unknown>> {
    const observations_chirho = this.address_observations_chirho;
    const steps_chirho: Record<string, unknown>[] = [];
    observations_chirho.steps_chirho = steps_chirho;
    await page_chirho.goto(this.workspace_chirho);
    const editor_chirho = page_chirho.locator('form[action$="edit-page-chirho"]');
    const version_chirho = await editor_chirho.locator('[name="expected_version_chirho"]').inputValue();
    const headline_chirho = await editor_chirho.locator('[name="headline_chirho"]').inputValue();
    const phone_chirho = await editor_chirho.locator('[name="contact_phone_chirho"]').inputValue();
    this.check_chirho(Number(version_chirho) > 0, "address recovery starts from her saved page");
    const taken_url_chirho = `${this.origin_chirho}/app-chirho/${taken_slug_chirho}/`;
    const taken_response_chirho = await fetch(taken_url_chirho);
    this.check_chirho(taken_response_chirho.status === 200, "taken-name control is another customer's live site");
    const taken_html_chirho = await taken_response_chirho.text();
    if (this.local_chirho) observations_chirho.usage_before_chirho = await this.usage_chirho();
    observations_chirho.saved_version_chirho = version_chirho;

    await page_chirho.goto(this.status_chirho);
    const initial_chirho = this.form_chirho(page_chirho, "publish-chirho");
    this.check_chirho(await page_chirho.locator('form').count() === 1 && await initial_chirho.count() === 1
      && await page_chirho.getByRole("main").getByRole("alert").count() === 0,
      "new publisher starts with one clean address form");
    await initial_chirho.locator('[name="slug_chirho"]').fill(reserved_slug_chirho);
    await this.submit_chirho(page_chirho, initial_chirho);

    const rejected_slugs_chirho = exercise_taken_chirho ? [reserved_slug_chirho, taken_slug_chirho] : [reserved_slug_chirho];
    for (const [index_chirho, rejected_slug_chirho] of rejected_slugs_chirho.entries()) {
      const label_chirho = index_chirho === 0 ? "reserved" : "taken";
      if (await page_chirho.locator('main .workspace-errors-chirho[role="status"]').count()) {
        // New door refusals render the correction form in this response. Keep it; navigating
        // away would discard the very recovery state being measured. Older sentence-only
        // refusals still use their actual return link and must retain THIS attempted name.
        await page_chirho.setViewportSize({ width: 390, height: 900 });
        await page_chirho.screenshot({ path: join(output_chirho, `address-${label_chirho}-door-390-chirho.png`), fullPage: true });
        if (await this.form_chirho(page_chirho, "publish-chirho").count() === 0) {
          const link_chirho = page_chirho.locator(`main a[href="${new URL(this.status_chirho).pathname}"]`);
          await Promise.all([page_chirho.waitForNavigation({ waitUntil: "networkidle" }), link_chirho.click()]);
        }
        await this.applies_nothing_chirho(`door-time ${label_chirho}-address refusal has nothing for the applier to publish`);
      } else {
        await page_chirho.goto(this.status_chirho);
        await this.transient_chirho(async () => await page_chirho.locator('form').count() === 0
          && await page_chirho.locator('.workspace-publication-pending-chirho[role="status"]').isVisible()
          && await page_chirho.getByRole("main").getByRole("alert").count() === 0,
        "pending address approval offers neither another form nor a stale field alert");
        await this.applies_nothing_chirho("platform refuses the unavailable address without applying it");
        const response_chirho = await page_chirho.goto(this.status_chirho);
        this.check_chirho(response_chirho?.status() === 200, "address refusal returns a usable Publishing page");
      }
      const form_chirho = this.form_chirho(page_chirho, "publish-chirho");
      const input_chirho = form_chirho.locator('[name="slug_chirho"]');
      const state_chirho = await input_chirho.evaluate(element_chirho => {
        const control_chirho = element_chirho as HTMLInputElement;
        const described_ids_chirho = (control_chirho.getAttribute("aria-describedby") ?? "").split(/\s+/).filter(Boolean);
        const described_chirho = described_ids_chirho.map(id_chirho => document.getElementById(id_chirho));
        const messages_chirho = described_chirho.filter(node_chirho => node_chirho?.classList.contains("workspace-field-error-chirho"));
        const ids_chirho = [...document.querySelectorAll('[id]')].map(node_chirho => node_chirho.id);
        return { id_chirho: control_chirho.id, value_chirho: control_chirho.value,
          invalid_chirho: control_chirho.getAttribute("aria-invalid"),
          labels_chirho: [...(control_chirho.labels ?? [])].map(label_chirho => ({ text_chirho: label_chirho.textContent?.trim(),
            help_inside_chirho: !!label_chirho.querySelector('small, .workspace-field-error-chirho') })),
          described_ids_chirho, descriptions_resolve_chirho: described_chirho.every(node_chirho => !!node_chirho?.textContent?.trim()),
          help_described_chirho: described_chirho.some(node_chirho => node_chirho?.tagName === "SMALL"),
          messages_chirho: messages_chirho.map(node_chirho => node_chirho?.textContent?.trim()),
          unique_ids_chirho: new Set(ids_chirho).size === ids_chirho.length };
      });
      this.check_chirho(await page_chirho.locator('form').count() === 1 && await input_chirho.count() === 1
        && state_chirho.value_chirho === rejected_slug_chirho && state_chirho.invalid_chirho === "true",
      `the single address input retains THIS refused name and marks it invalid (expected ${rejected_slug_chirho}, got ${state_chirho.value_chirho})`);
      this.check_chirho(state_chirho.unique_ids_chirho && state_chirho.descriptions_resolve_chirho
        && state_chirho.help_described_chirho && state_chirho.messages_chirho.length === 1
        && state_chirho.labels_chirho.length === 1 && state_chirho.labels_chirho[0].text_chirho
        && !state_chirho.labels_chirho[0].help_inside_chirho,
      "address help/error describe a separately labelled control with no dangling IDs");
      const alert_chirho = page_chirho.getByRole("main").getByRole("alert");
      const summary_link_chirho = alert_chirho.locator(`a[href="#${state_chirho.id_chirho}"]`);
      this.check_chirho(await alert_chirho.count() === 1 && await summary_link_chirho.count() === 1
        && (await alert_chirho.innerText()).includes(state_chirho.messages_chirho[0]!),
      "one summary links to the address input and carries the same field message");
      const html_chirho = await page_chirho.content();
      this.check_chirho(!html_chirho.includes(`/app-chirho/${rejected_slug_chirho}/`)
        && !/slug-reserved-chirho|slug-taken-chirho|not-your-app-chirho/.test(html_chirho),
      "refused-only name exposes neither a falsely owned address path nor an engine refusal token");
      this.check_chirho(await form_chirho.locator('[name="version_chirho"]').inputValue() === version_chirho,
        "correcting the address still approves the same saved version");
      await page_chirho.setViewportSize({ width: 390, height: 900 });
      await this.check_status_layout_chirho(page_chirho, join(output_chirho, `address-${label_chirho}-390-chirho.png`));
      await page_chirho.setViewportSize({ width: 1440, height: 900 });
      await page_chirho.screenshot({ path: join(output_chirho, `address-${label_chirho}-1440-chirho.png`), fullPage: true });
      await page_chirho.setViewportSize({ width: 390, height: 900 });
      const tabs_chirho = await tab_to_chirho(page_chirho, summary_link_chirho);
      await page_chirho.keyboard.press("Enter");
      this.check_chirho(await input_chirho.evaluate(node_chirho => document.activeElement === node_chirho)
        && await control_and_label_visible_chirho(input_chirho),
      "keyboard summary activation focuses the address input with its label and control visible");
      const replacement_chirho = rejected_slugs_chirho[index_chirho + 1] ?? this.slug_chirho;
      await page_chirho.keyboard.press("ControlOrMeta+A");
      await page_chirho.keyboard.insertText(replacement_chirho);
      this.check_chirho(await input_chirho.inputValue() === replacement_chirho,
        "keyboard correction replaces the retained name in the actual address input");
      const step_chirho: Record<string, unknown> = { refused_slug_chirho: rejected_slug_chirho,
        state_chirho, tabs_chirho, replacement_chirho, kind_chirho: label_chirho,
        recovery_surface_chirho: page_chirho.url() === this.status_chirho ? "status-page-chirho" : "post-response-chirho",
        usage_before_submission_chirho: this.local_chirho ? await this.usage_chirho() : undefined };
      steps_chirho.push(step_chirho);
      await this.submit_chirho(page_chirho, form_chirho, true);
      step_chirho.submission_page_chirho = (await page_chirho.locator('main').innerText()).trim();
      if (this.local_chirho) step_chirho.usage_after_submission_chirho = await this.usage_chirho();
    }

    await this.transient_chirho(async () => (await fetch(this.live_chirho)).status === 404, "corrected address stays dark until applied");
    await this.applies_once_chirho("corrected address applies once", "supervised timer publishes the corrected address",
      async () => (await fetch(this.live_chirho, { signal: AbortSignal.timeout(10_000) })).status === 200);
    await this.saved_is_live_chirho(page_chirho, version_chirho);
    this.check_chirho(await page_chirho.locator('main [role="alert"]:visible, input[name="slug_chirho"], .workspace-field-error-chirho').count() === 0,
      "successful publication clears address alerts, invalid field and correction form");
    this.check_chirho(await page_chirho.locator('main a[href^="#"]').evaluateAll(links_chirho => links_chirho.every(link_chirho =>
      !!document.getElementById(link_chirho.getAttribute("href")!.slice(1)))), "no dangling correction link remains after success");
    await this.check_status_layout_chirho(page_chirho, join(output_chirho, "address-live-390-chirho.png"));
    const public_link_chirho = page_chirho.locator(`main a[href="/app-chirho/${this.slug_chirho}/"]`);
    this.check_chirho(await public_link_chirho.count() === 1 && (await page_chirho.context().cookies()).some(cookie_chirho =>
      cookie_chirho.name === "cairn_session_chirho"), "her managed-address link is reached while the real browser remains signed in");
    const navigation_chirho = page_chirho.waitForNavigation({ waitUntil: "networkidle" });
    navigation_chirho.catch(() => undefined); // guarded: a failed action must not leave this rejection unhandled; awaiting it still raises
    await public_link_chirho.click();
    const public_response_chirho = await navigation_chirho;
    this.check_chirho(public_response_chirho?.status() === 200 && page_chirho.url() === this.live_chirho,
      "signed-in browser follows the managed address through Caddy without a session-cookie refusal");
    const phone_link_chirho = page_chirho.locator('a[href^="tel:"]');
    this.check_chirho((await page_chirho.locator('body').innerText()).includes(headline_chirho)
      && (await phone_link_chirho.innerText()).includes(phone_chirho.trim())
      && (await phone_link_chirho.getAttribute("href"))?.replace(/\D/g, "") === phone_chirho.replace(/\D/g, ""),
      "recovered public page serves her saved headline and phone link");
    await this.public_html_chirho();
    this.check_chirho(await (await fetch(taken_url_chirho)).text() === taken_html_chirho,
      `${exercise_taken_chirho ? "reserved/taken" : "reserved"}-name attempts and correction never alter the other customer's public bytes`);
    await page_chirho.screenshot({ path: join(output_chirho, "address-recovered-public-390-chirho.png"), fullPage: true });
    observations_chirho.public_status_chirho = public_response_chirho?.status();
    observations_chirho.live_route_chirho = new URL(this.live_chirho).pathname;
    if (this.local_chirho) observations_chirho.usage_after_chirho = await this.usage_chirho();
    observations_chirho.other_site_unchanged_chirho = true;
    return observations_chirho;
  }

  async check_status_layout_chirho(page_chirho: PageChirho, screenshot_chirho: string): Promise<void> {
    const viewport_chirho = page_chirho.viewportSize();
    if (!viewport_chirho) throw Error("Publishing layout check needs an explicit viewport");
    try {
      for (const width_chirho of new Set([viewport_chirho.width, 320, 390, 768, 1440])) {
        await page_chirho.setViewportSize({ width: width_chirho, height: viewport_chirho.height });
        await page_chirho.evaluate(() => document.fonts.ready);
        const layout_chirho = await page_chirho.evaluate(() => ({
          width_chirho: innerWidth, scroll_chirho: document.documentElement.scrollWidth,
          overflow_chirho: [...document.querySelectorAll('main *')].flatMap(element_chirho => {
            const box_chirho = element_chirho.getBoundingClientRect();
            return box_chirho.right > innerWidth + 1 || box_chirho.left < -1
              ? [{ tag_chirho: element_chirho.tagName, class_chirho: element_chirho.getAttribute("class"), left_chirho: box_chirho.left, right_chirho: box_chirho.right }] : [];
          }).slice(0, 8),
        }));
        if (layout_chirho.scroll_chirho > width_chirho + 1) await page_chirho.screenshot({ path: screenshot_chirho, fullPage: true });
        this.check_chirho(layout_chirho.scroll_chirho <= width_chirho + 1,
          `Publishing reflows at ${width_chirho}px: ${JSON.stringify(layout_chirho)}`);
      }
    } finally { await page_chirho.setViewportSize(viewport_chirho); }
    await page_chirho.screenshot({ path: screenshot_chirho, fullPage: true });
  }

  async verify_assets_chirho(page_chirho: PageChirho): Promise<string> {
    await page_chirho.goto(this.live_chirho);
    this.check_chirho(await page_chirho.locator('script, form').count() === 0, "public site contains no script or credential form");
    const stylesheet_chirho = await page_chirho.locator('link[rel="stylesheet"]').first().getAttribute("href");
    if (!stylesheet_chirho) throw Error("Published page has no stylesheet");
    const css_chirho = await fetch(new URL(stylesheet_chirho, this.live_chirho));
    const hash_chirho = new Bun.CryptoHasher("sha256").update(await css_chirho.arrayBuffer()).digest("hex");
    this.check_chirho(css_chirho.status === 200 && css_chirho.headers.get("content-type")?.startsWith("text/css")
      && hash_chirho === this.observer_chirho.template_css_sha256_chirho,
    "served stylesheet equals the reviewed template asset");
    await page_chirho.evaluate(() => document.fonts.ready);
    this.check_chirho(await page_chirho.evaluate(() => document.documentElement.scrollWidth <= innerWidth + 1), "public site fits the viewport");
    return hash_chirho;
  }
}
