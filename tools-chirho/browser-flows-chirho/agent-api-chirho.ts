// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// Workflows: product-delivery-chirho/reviewed-product-chirho.md and hosted-product-install-chirho.md.
// Operator-issued synthetic bearer transport, not self-service keys or permission certification.
import { chmod, rm } from "node:fs/promises";
import { join } from "node:path";
import type { Locator as LocatorChirho, Page as PageChirho } from "playwright-core";
import type { ReviewedDeploymentChirho } from "../browser-chirho/reviewed-deployment-chirho";
import { write_private_chirho } from "./private-file-chirho";
import type { JourneyFixtureChirho } from "./workspace-session-chirho";

// Where the bearers and review evidence come from. Local: the owned deployment issues operator
// tokens and holds the review reports and the root. Hosted: the host owner supplies one bearer per
// synthetic user; review reports and the root scan stay host-owner evidence, named as unexercised.
export type AgentApiSourceChirho =
  | { kind_chirho: "local-chirho"; deployment_chirho: ReviewedDeploymentChirho }
  | { kind_chirho: "hosted-chirho"; bearers_chirho: string[] };

interface DocumentChirho {
  schema_version_chirho: number;
  business_name_chirho: string;
  headline_chirho: string;
  summary_chirho: string;
  services_chirho: { title_chirho: string; body_chirho: string }[];
  contact_heading_chirho: string;
  contact_phone_chirho: string;
  contact_email_chirho: string;
  contact_details_chirho: string;
}
interface SurfaceChirho {
  routes_chirho: { method_chirho: string; path_chirho: string }[];
  sample_page_chirho: DocumentChirho;
  page_document_chirho: { fields_chirho: { field_path_chirho: string; max_characters_chirho: number }[] };
  save_chirho: { max_body_bytes_chirho: number };
}
interface ReadChirho {
  state_chirho: string;
  current_version_chirho: number | null;
  page_chirho: DocumentChirho | null;
  surface_chirho: SurfaceChirho;
}
interface SaveChirho {
  saved_chirho: boolean;
  refusal_chirho: string | null;
  current_version_chirho: number | null;
  page_chirho: DocumentChirho | null;
  issues_chirho: { field_path_chirho: string; code_chirho: string }[];
}

export class AgentApiJourneyChirho {
  readonly observations_chirho: Record<string, unknown> = {};
  readonly known_gaps_chirho: string[] = [];
  readonly unexercised_chirho: string[] = [];
  readonly workspace_chirho: string;
  private readonly keys_chirho: string[] = [];
  private readonly marker_chirho = `SyntheticApi${crypto.randomUUID().replaceAll("-", "")}`;
  private readonly refused_marker_chirho = `RefusedApi${crypto.randomUUID().replaceAll("-", "")}`;
  private save_path_chirho = "";
  private preview_path_chirho = "";
  private surface_chirho!: SurfaceChirho;
  private first_chirho!: DocumentChirho;
  private first_request_chirho = "";
  private other_chirho!: DocumentChirho;
  private readonly headers_chirho: Record<string, string | null>[] = [];

  constructor(readonly fixture_chirho: JourneyFixtureChirho, readonly origin_chirho: string, readonly output_chirho: string,
    readonly source_chirho: AgentApiSourceChirho) {
    this.workspace_chirho = `${origin_chirho}/app-chirho/perffection-workspace/`;
    this.observations_chirho.headers_chirho = this.headers_chirho;
    this.observations_chirho.known_gaps_chirho = this.known_gaps_chirho;
  }
  private get needles_chirho(): string {
    return join(this.output_chirho, "host-scan-needles-chirho.txt");
  }
  private check_chirho(condition_chirho: unknown, label_chirho: string): void {
    this.fixture_chirho.check_chirho(condition_chirho, `agent API: ${label_chirho}`);
  }
  private get deployment_chirho(): ReviewedDeploymentChirho | undefined {
    return this.source_chirho.kind_chirho === "local-chirho" ? this.source_chirho.deployment_chirho : undefined;
  }
  private gap_chirho(label_chirho: string): void {
    if (!this.known_gaps_chirho.includes(label_chirho)) this.known_gaps_chirho.push(label_chirho);
  }
  private cache_observation_chirho(headers_chirho: Headers, label_chirho: string): void {
    const policy_chirho = headers_chirho.get("cache-control");
    this.headers_chirho.push({ response_chirho: label_chirho, cache_control_chirho: policy_chirho });
    const directives_chirho = policy_chirho?.toLowerCase().split(",").map(value_chirho => value_chirho.trim()) ?? [];
    if (!["private", "no-store"].every(directive_chirho => directives_chirho.includes(directive_chirho))) {
      this.gap_chirho("protected-app-private-no-store-missing-chirho");
    }
  }
  private async request_chirho(path_chirho: string, key_chirho: string | undefined, body_chirho?: string): Promise<Response> {
    const target_chirho = new URL(path_chirho, this.origin_chirho);
    if (target_chirho.origin !== this.origin_chirho) throw Error("Refusing to send a fixture bearer off-origin");
    return await fetch(target_chirho, { method: body_chirho === undefined ? "GET" : "POST", redirect: "manual",
      headers: { ...(key_chirho === undefined ? {} : { Authorization: `Bearer ${key_chirho}` }),
        ...(body_chirho === undefined ? {} : { "Content-Type": "application/json" }) },
      body: body_chirho, signal: AbortSignal.timeout(15000) });
  }
  private async json_chirho<ResponseChirho>(path_chirho: string, viewer_chirho: number, label_chirho: string, body_chirho?: string): Promise<ResponseChirho> {
    const response_chirho = await this.request_chirho(path_chirho, this.keys_chirho[viewer_chirho], body_chirho);
    this.check_chirho(response_chirho.status === 200, `${label_chirho}: HTTP 200`);
    this.check_chirho(response_chirho.headers.get("content-type")?.split(";")[0] === "application/json"
      && response_chirho.headers.get("x-content-type-options") === "nosniff", `${label_chirho}: actual JSON MIME and nosniff`);
    this.cache_observation_chirho(response_chirho.headers, label_chirho);
    const bytes_chirho = await response_chirho.text();
    this.check_chirho(this.fixture_chirho.redact_chirho(bytes_chirho) === bytes_chirho, `${label_chirho}: no credential or principal echoed`);
    return JSON.parse(bytes_chirho) as ResponseChirho;
  }
  private async read_chirho(viewer_chirho = 0): Promise<ReadChirho> {
    return this.json_chirho<ReadChirho>(`${this.workspace_chirho}agent-page-chirho`, viewer_chirho, `viewer ${viewer_chirho} read`);
  }
  private async saved_chirho(version_chirho: number, document_chirho: DocumentChirho, viewer_chirho = 0): Promise<void> {
    const state_chirho = await this.read_chirho(viewer_chirho);
    this.check_chirho(state_chirho.state_chirho === "saved-chirho" && state_chirho.current_version_chirho === version_chirho
      && JSON.stringify(state_chirho.page_chirho) === JSON.stringify(document_chirho), `viewer ${viewer_chirho}: stored document and version ${version_chirho}`);
  }
  private async save_chirho(document_chirho: DocumentChirho, expected_chirho: number, viewer_chirho = 0): Promise<SaveChirho> {
    return this.json_chirho<SaveChirho>(this.save_path_chirho, viewer_chirho, `save based on ${expected_chirho}`,
      JSON.stringify({ expected_version_chirho: expected_chirho, page_chirho: document_chirho }));
  }
  private form_chirho(page_chirho: PageChirho): LocatorChirho {
    return page_chirho.locator('form[action$="edit-page-chirho"]');
  }
  private async editor_chirho(page_chirho: PageChirho, version_chirho: number, document_chirho: DocumentChirho): Promise<void> {
    const response_chirho = await page_chirho.goto(this.workspace_chirho);
    this.check_chirho(response_chirho?.status() === 200, "browser editor serves");
    this.cache_observation_chirho(new Headers(await response_chirho!.allHeaders()), "browser editor");
    const form_chirho = this.form_chirho(page_chirho);
    this.check_chirho(await form_chirho.locator('[name="expected_version_chirho"]').inputValue() === String(version_chirho), `browser editor is version ${version_chirho}`);
    for (const [field_chirho, value_chirho] of Object.entries(document_chirho)) {
      if (typeof value_chirho === "string") this.check_chirho(await form_chirho.locator(`[name="${field_chirho}"]`).inputValue() === value_chirho,
        `browser preserves ${field_chirho} from API`);
    }
  }
  private async submit_chirho(page_chirho: PageChirho, form_chirho: LocatorChirho): Promise<void> {
    this.check_chirho(await form_chirho.evaluate(form_chirho => (form_chirho as HTMLFormElement).checkValidity()), "native browser constraints remain enabled");
    const pending_chirho = page_chirho.waitForResponse(response_chirho => response_chirho.request().isNavigationRequest() && response_chirho.request().method() === "POST");
    pending_chirho.catch(() => undefined); // guarded: a failed action must not leave this rejection unhandled; awaiting it still raises
    await Promise.all([page_chirho.waitForNavigation({ waitUntil: "networkidle" }), form_chirho.locator('button[type="submit"]').click()]);
    this.check_chirho((await pending_chirho).status() === 200, "real browser form POST succeeds");
  }

  async start_chirho(page_chirho: PageChirho): Promise<void> {
    const deployment_chirho = this.deployment_chirho;
    if (deployment_chirho) {
      await deployment_chirho.capture_chirho(join(deployment_chirho.product_chirho, "scripts-chirho/lib-chirho/root-scan-chirho.sh"), "agent-api-root-scanner-chirho");
      this.keys_chirho.push(...deployment_chirho.issue_synthetic_bearers_chirho());
    } else if (this.source_chirho.kind_chirho === "hosted-chirho") {
      if (this.source_chirho.bearers_chirho.length !== this.fixture_chirho.users_chirho.length) throw Error("Hosted run needs one bearer per synthetic user");
      this.fixture_chirho.private_values_chirho.push(...this.source_chirho.bearers_chirho);
      this.keys_chirho.push(...this.source_chirho.bearers_chirho);
      // The host owner scans the host root. This run's two markers exist nowhere else, so they are
      // handed over owner-only before anything is saved; a run that fails midway leaves them too.
      await write_private_chirho(this.needles_chirho, `${this.marker_chirho}\n${this.refused_marker_chirho}\n`);
    }
    const workspace_app_chirho = "perffection-workspace";
    for (const path_chirho of ["/app-chirho/perffection/agent-guide-chirho", "/app-chirho/perffection-workspace/own-ai-chirho", "/app-chirho/perffection-workspace/review-own-ai-draft-chirho"]) {
      const [, , app_chirho, route_chirho] = path_chirho.split("/");
      if (deployment_chirho) {
        const candidate_chirho = deployment_chirho.candidates_chirho.find(candidate_chirho => candidate_chirho.review_chirho.candidate_chirho.app_chirho === app_chirho);
        this.check_chirho(candidate_chirho && !candidate_chirho.review_chirho.candidate_chirho.routes_chirho.some(entry_chirho => entry_chirho.route_chirho === route_chirho),
          `retired route absent from reviewed candidate: ${path_chirho}`);
      } else this.unexercised_chirho.push(`retired route absent from reviewed candidate: ${path_chirho} (host review report)`);
      const response_chirho = await this.request_chirho(path_chirho, this.keys_chirho[0]);
      const body_chirho = this.fixture_chirho.redact_chirho(await response_chirho.text());
      this.observations_chirho[`retired-${route_chirho}`] = { status_chirho: response_chirho.status, body_chirho };
      this.check_chirho(response_chirho.status === 400
        && (body_chirho === "error: app-selection-refused-chirho\n"
          || body_chirho === `error: app route '${app_chirho}/${route_chirho}' is not deployed\n`),
        `retired route selection refuses as undeployed: ${path_chirho}`);
      if (path_chirho.includes("review-own")) {
        const post_chirho = await this.request_chirho(path_chirho, this.keys_chirho[0], "{}");
        this.check_chirho(post_chirho.status === 405 && await post_chirho.text() === "", "retired review POST is rejected at the pre-body policy");
      }
    }
    for (const path_chirho of ["/app-chirho/perffection/", "/app-chirho/perffection/help-chirho"]) {
      await page_chirho.goto(`${this.origin_chirho}${path_chirho}`);
      this.check_chirho(await page_chirho.locator('a[href*="agent-guide-chirho"], a[href*="/own-ai-chirho"]').count() === 0, "public entry offers no manual-agent path");
    }
    const first_chirho = await this.read_chirho();
    this.check_chirho(first_chirho.state_chirho === "nothing-saved-chirho" && first_chirho.current_version_chirho === 0 && first_chirho.page_chirho === null,
      "new viewer has no saved page, not an unverifiable zero");
    this.surface_chirho = first_chirho.surface_chirho;
    const save_chirho = this.surface_chirho.routes_chirho.filter(route_chirho => route_chirho.method_chirho === "POST");
    const preview_chirho = this.surface_chirho.routes_chirho.filter(route_chirho => route_chirho.path_chirho.endsWith("/preview-chirho"));
    this.check_chirho(save_chirho.length === 1 && preview_chirho.length === 1, "served surface identifies save and preview");
    for (const route_chirho of [...save_chirho, ...preview_chirho]) this.check_chirho(route_chirho.path_chirho.startsWith("/app-chirho/perffection-workspace/"), "surface destination remains in the authenticated app");
    this.save_path_chirho = save_chirho[0].path_chirho;
    this.preview_path_chirho = preview_chirho[0].path_chirho;
    const review_chirho = deployment_chirho?.reviews_chirho.find(review_chirho => review_chirho.app_chirho === workspace_app_chirho);
    for (const route_chirho of this.surface_chirho.routes_chirho) {
      const label_chirho = `reviewed ${route_chirho.method_chirho} ${route_chirho.path_chirho}: viewer-state authority only, no publication or AI`;
      if (!deployment_chirho) { this.unexercised_chirho.push(`${label_chirho} (host review report)`); continue; }
      const diff_chirho = review_chirho?.surfaces_chirho.find(surface_chirho => surface_chirho.route_chirho === route_chirho.path_chirho.split("/").at(-1))?.diff_chirho;
      const permitted_chirho = route_chirho.method_chirho === "GET" ? "ReadStoreChirho(prefix=viewer-state-chirho/)" : "WriteStoreChirho(prefix=viewer-state-chirho/)";
      this.check_chirho(diff_chirho && diff_chirho.added_capabilities_chirho.length === 1 && diff_chirho.added_capabilities_chirho[0] === permitted_chirho, label_chirho);
    }
    for (const key_chirho of [undefined, "synthetic-invalid-token-chirho"]) {
      for (const [path_chirho, body_chirho] of [[`${this.workspace_chirho}agent-page-chirho`, undefined], [this.save_path_chirho, "{}"]] as const) {
        const response_chirho = await this.request_chirho(path_chirho, key_chirho, body_chirho);
        this.check_chirho([401, 403].includes(response_chirho.status) && !(await response_chirho.text()).includes('"page_chirho"'), "absent/invalid bearer cannot read or save");
      }
    }
    const document_chirho = structuredClone(this.surface_chirho.sample_page_chirho);
    document_chirho.business_name_chirho = this.marker_chirho;
    document_chirho.headline_chirho = '</textarea><img src="/injected-chirho"> Café 日本語';
    document_chirho.summary_chirho = "  Her first line.\r\nHer second line.  ";
    document_chirho.contact_details_chirho = "  By appointment.  ";
    this.first_request_chirho = JSON.stringify({ expected_version_chirho: 0, page_chirho: document_chirho });
    const saved_chirho = await this.json_chirho<SaveChirho>(this.save_path_chirho, 0, "first API save", this.first_request_chirho);
    this.first_chirho = { ...document_chirho, summary_chirho: "Her first line.\nHer second line.", contact_details_chirho: "By appointment." };
    this.check_chirho(saved_chirho.saved_chirho && saved_chirho.refusal_chirho === null && saved_chirho.current_version_chirho === 1
      && JSON.stringify(saved_chirho.page_chirho) === JSON.stringify(this.first_chirho), "API save returns version one as normalized and actually stored");
    await this.saved_chirho(1, this.first_chirho);
    const replay_chirho = await this.json_chirho<SaveChirho>(this.save_path_chirho, 0, "immediate exact save retry", this.first_request_chirho);
    this.check_chirho(JSON.stringify(replay_chirho) === JSON.stringify(saved_chirho), "immediate exact retry returns the same outcome");
    await this.saved_chirho(1, this.first_chirho);
    const publishing_chirho = await this.request_chirho(`${this.workspace_chirho}publication-status-chirho`, this.keys_chirho[0]);
    this.observations_chirho.draft_scope_publication_status_chirho = publishing_chirho.status;
    if (publishing_chirho.status === 200) this.gap_chirho("draft-scope-can-read-publication-chirho");
    else this.check_chirho([401, 403].includes(publishing_chirho.status), "draft-scope publication read has an explicit auth refusal");
    await publishing_chirho.body?.cancel();
  }

  async browser_chirho(page_chirho: PageChirho): Promise<void> {
    await this.editor_chirho(page_chirho, 1, this.first_chirho);
    for (const path_chirho of [this.workspace_chirho, `${this.workspace_chirho}import-page-document-chirho`]) {
      await page_chirho.goto(path_chirho);
      this.check_chirho(await page_chirho.locator('a[href*="/own-ai-chirho"], a[href*="agent-guide-chirho"]').count() === 0, "workspace entry offers no retired manual route");
    }
    const preview_chirho = await this.request_chirho(this.preview_path_chirho, this.keys_chirho[0]);
    this.check_chirho(preview_chirho.status === 200 && preview_chirho.headers.get("content-type")?.startsWith("text/html"), "bearer previews the same saved page as HTML");
    await preview_chirho.body?.cancel();
    await page_chirho.goto(`${this.origin_chirho}${this.preview_path_chirho}`);
    this.check_chirho((await page_chirho.locator("body").innerText()).includes(this.first_chirho.headline_chirho)
      && await page_chirho.locator('img[src="/injected-chirho"]').count() === 0, "browser preview preserves literal markup and Unicode without injection");
    await page_chirho.screenshot({ path: join(this.output_chirho, "agent-api-preview-chirho.png"), fullPage: true });
    await this.saved_chirho(1, this.first_chirho);
    const second_chirho = { ...this.first_chirho, headline_chirho: "Second API version" };
    const saved_chirho = await this.save_chirho(second_chirho, 1);
    this.check_chirho(saved_chirho.saved_chirho && saved_chirho.current_version_chirho === 2, "second API save advances once");
    await this.editor_chirho(page_chirho, 2, second_chirho);
    const third_chirho = { ...second_chirho, headline_chirho: "Changed in her browser" };
    await this.form_chirho(page_chirho).locator('[name="headline_chirho"]').fill(third_chirho.headline_chirho);
    await this.submit_chirho(page_chirho, this.form_chirho(page_chirho));
    await this.saved_chirho(3, third_chirho);
    const stale_chirho = await this.save_chirho({ ...second_chirho, headline_chirho: "Fresh stale request" }, 1);
    this.check_chirho(!stale_chirho.saved_chirho && stale_chirho.refusal_chirho === "stale-version-chirho" && stale_chirho.current_version_chirho === 3,
      "fresh stale API save reports the actual current version");
    await this.saved_chirho(3, third_chirho);
    const retry_chirho = await this.json_chirho<SaveChirho>(this.save_path_chirho, 0, "delayed exact save retry", this.first_request_chirho);
    this.check_chirho(retry_chirho.saved_chirho && retry_chirho.current_version_chirho === 1, "delayed exact retry retains its original receipt");
    await this.saved_chirho(3, third_chirho);
    await page_chirho.goto(this.workspace_chirho);
    const restore_chirho = page_chirho.locator('form[action$="restore-page-chirho"]').filter({ has: page_chirho.locator('[name="restore_from_version_chirho"][value="1"]') });
    this.check_chirho(await restore_chirho.count() === 1, "API-created version appears in the real browser restore form");
    await this.submit_chirho(page_chirho, restore_chirho);
    await this.saved_chirho(4, this.first_chirho);
    await this.editor_chirho(page_chirho, 4, this.first_chirho);
    await this.refusals_chirho();
    const layouts_chirho: unknown[] = [];
    this.observations_chirho.restored_editor_layouts_chirho = layouts_chirho;
    for (const width_chirho of [320, 390, 768, 1440]) {
      await page_chirho.setViewportSize({ width: width_chirho, height: 950 });
      const layout_chirho = await page_chirho.evaluate(() => {
        const cards_chirho = [...document.querySelectorAll(".workspace-history-item-chirho")];
        const text_fits_chirho = cards_chirho.length > 0 && cards_chirho.every(card_chirho => {
          const box_chirho = card_chirho.getBoundingClientRect();
          return [...card_chirho.querySelectorAll("h3, span")].every(text_chirho => {
            const range_chirho = document.createRange();
            range_chirho.selectNodeContents(text_chirho);
            return [...range_chirho.getClientRects()].every(rect_chirho => rect_chirho.left >= box_chirho.left - 1
              && rect_chirho.right <= box_chirho.right + 1 && rect_chirho.top >= box_chirho.top - 1 && rect_chirho.bottom <= box_chirho.bottom + 1);
          });
        });
        return { viewport_chirho: innerWidth, scroll_chirho: document.documentElement.scrollWidth, history_text_fits_chirho: text_fits_chirho,
          overflow_chirho: [...document.querySelectorAll("main *")].filter(element_chirho => element_chirho.getBoundingClientRect().right > innerWidth + 1)
            .slice(0, 12).map(element_chirho => ({ tag_chirho: element_chirho.tagName, class_chirho: element_chirho.className,
              right_chirho: element_chirho.getBoundingClientRect().right })) };
      });
      layouts_chirho.push(layout_chirho);
      this.check_chirho(layout_chirho.scroll_chirho <= layout_chirho.viewport_chirho && layout_chirho.history_text_fits_chirho,
        `restored API content fits browser editor and history text stays inside its cards at ${width_chirho}`);
    }
    await page_chirho.setViewportSize({ width: 390, height: 844 });
    await page_chirho.screenshot({ path: join(this.output_chirho, "agent-api-restored-editor-chirho.png"), fullPage: true });
    await page_chirho.locator(".workspace-history-chirho").screenshot({ path: join(this.output_chirho, "agent-api-restored-history-chirho.png") });
    this.observations_chirho.final_viewer_a_version_chirho = 4;
  }

  private async refusals_chirho(): Promise<void> {
    const field_chirho = this.surface_chirho.page_document_chirho.fields_chirho.find(field_chirho => field_chirho.field_path_chirho === "headline_chirho");
    if (!field_chirho || !Number.isSafeInteger(field_chirho.max_characters_chirho) || field_chirho.max_characters_chirho < 1) throw Error("Missing served headline bound");
    const bound_chirho = this.surface_chirho.save_chirho.max_body_bytes_chirho;
    this.check_chirho(Number.isSafeInteger(bound_chirho) && bound_chirho > 0 && bound_chirho < 1_000_000, "served body bound is finite before allocating a probe");
    this.check_chirho(field_chirho.max_characters_chirho <= bound_chirho, "served field bound fits the bounded probe budget");
    const payload_chirho = { expected_version_chirho: 4, page_chirho: this.first_chirho };
    const arms_chirho: [string, unknown, string][] = [
      ["invalid JSON", `${this.refused_marker_chirho}{`, "unreadable-chirho"],
      ["unknown envelope/identity", { ...payload_chirho, principal_chirho: this.fixture_chirho.users_chirho[1].principal_chirho }, "unreadable-chirho"],
      ["unknown document member", { ...payload_chirho, page_chirho: { ...this.first_chirho, extra_chirho: this.refused_marker_chirho } }, "unreadable-chirho"],
      ["unsupported schema", { ...payload_chirho, page_chirho: { ...this.first_chirho, schema_version_chirho: 999 } }, "unsupported-schema-chirho"],
      ["two invalid fields", { ...payload_chirho, page_chirho: { ...this.first_chirho, business_name_chirho: "", headline_chirho: "", summary_chirho: this.refused_marker_chirho } }, "invalid-page-chirho"],
      ["overlong field", { ...payload_chirho, page_chirho: { ...this.first_chirho, headline_chirho: "é".repeat(field_chirho.max_characters_chirho + 1) } }, "invalid-page-chirho"],
      ["oversized body", " ".repeat(bound_chirho + 1), "oversize-chirho"],
    ];
    for (const [label_chirho, body_chirho, code_chirho] of arms_chirho) {
      const refused_chirho = await this.json_chirho<SaveChirho>(this.save_path_chirho, 0, label_chirho,
        typeof body_chirho === "string" ? body_chirho : JSON.stringify(body_chirho));
      this.check_chirho(!refused_chirho.saved_chirho && refused_chirho.refusal_chirho === code_chirho && refused_chirho.page_chirho === null,
        `${label_chirho}: typed refusal, not a successful HTTP status mistaken for Save`);
      this.check_chirho(!JSON.stringify(refused_chirho).includes(this.refused_marker_chirho), `${label_chirho}: submitted private marker not reflected`);
      if (label_chirho === "two invalid fields") this.check_chirho(["business_name_chirho", "headline_chirho"].every(field_chirho =>
        refused_chirho.issues_chirho.some(issue_chirho => issue_chirho.field_path_chirho === field_chirho)), "all invalid field paths returned together");
      await this.saved_chirho(4, this.first_chirho);
    }
  }

  async prepare_other_chirho(): Promise<void> {
    const second_chirho = await this.read_chirho(1);
    this.check_chirho(second_chirho.state_chirho === "nothing-saved-chirho" && second_chirho.current_version_chirho === 0
      && second_chirho.page_chirho === null && !JSON.stringify(second_chirho).includes(this.marker_chirho), "viewer B bearer cannot read A's page or history");
    this.other_chirho = { ...second_chirho.surface_chirho.sample_page_chirho, business_name_chirho: "Other synthetic customer", headline_chirho: "A separate private page" };
    const saved_chirho = await this.save_chirho(this.other_chirho, 0, 1);
    this.check_chirho(saved_chirho.saved_chirho && saved_chirho.current_version_chirho === 1, "viewer B saves in B's scope");
    await this.saved_chirho(4, this.first_chirho);
    const attempted_chirho = await this.json_chirho<ReadChirho>(`${this.workspace_chirho}agent-page-chirho?principal_chirho=${encodeURIComponent(this.fixture_chirho.users_chirho[0].principal_chirho)}`, 1, "foreign principal in query");
    this.check_chirho(attempted_chirho.current_version_chirho === 1 && JSON.stringify(attempted_chirho.page_chirho) === JSON.stringify(this.other_chirho), "query cannot select a foreign viewer");
  }
  async other_browser_chirho(page_chirho: PageChirho): Promise<void> {
    await this.editor_chirho(page_chirho, 1, this.other_chirho);
    this.check_chirho(!(await page_chirho.content()).includes(this.marker_chirho), "separate browser sees no first viewer content");
    this.check_chirho(await page_chirho.locator('form[action$="restore-page-chirho"]').count() === 0, "other viewer does not acquire A's history");
    await this.saved_chirho(1, this.other_chirho, 1);
    this.observations_chirho.final_viewer_b_version_chirho = 1;
  }

  // Run once nothing can write to the root, before the disposable root is removed. A local run's
  // only writers are its owned servers, now stopped, because it ticks the applier itself. A live
  // root shows only what is on disk at that moment (#25386). This is literal fixed-string visibility
  // only; sealed storage, other encodings and K are not discharged.
  async privacy_chirho(): Promise<void> {
    const deployment_chirho = this.deployment_chirho;
    if (!deployment_chirho) {
      this.unexercised_chirho.push(`fixed-string root scan after the journey (host owner privacy-scan), on a root no process can write to (the services stopped and the applier timer inactive), or else the result says "live" and when: this run's two markers are in ${this.needles_chirho} (mode 600), the users' credentials and bearers in the users files; each must scan to 0, with the positive, missing-root and unreadable-file controls`);
      return;
    }
    const helper_chirho = join(deployment_chirho.product_chirho, "scripts-chirho/lib-chirho/root-scan-chirho.sh");
    const scan_chirho = (needle_chirho: string, root_chirho: string): string => deployment_chirho.command_chirho(
      ["bash", "-c", 'source "$1"; scan_root_chirho "$2" "$3"', "agent-api-scan-chirho", helper_chirho, needle_chirho, root_chirho], "fail-closed root scan");
    const root_chirho = deployment_chirho.root_chirho;
    this.check_chirho(scan_chirho(this.marker_chirho, `${root_chirho}/missing-control-chirho`) === "scan-error:missing-root", "scanner refuses a missing root");
    const control_chirho = join(root_chirho, "agent-api-scan-control-chirho.txt");
    await write_private_chirho(control_chirho, this.marker_chirho);
    try {
      this.check_chirho(Number(scan_chirho(this.marker_chirho, root_chirho)) > 0, "scanner sees planted clear text in this root");
      await chmod(control_chirho, 0o000);
      this.check_chirho(scan_chirho(this.marker_chirho, root_chirho) === "scan-error:rc=2", "unreadable file cannot become a clean scan");
    } finally { await chmod(control_chirho, 0o600); await rm(control_chirho); }
    for (const [index_chirho, needle_chirho] of [this.marker_chirho, this.refused_marker_chirho, ...this.keys_chirho,
      ...this.fixture_chirho.users_chirho.map(user_chirho => user_chirho.credential_chirho)].entries()) {
      this.check_chirho(scan_chirho(needle_chirho, root_chirho) === "0", `no literal synthetic marker/credential in stopped root: probe ${index_chirho}`);
    }
    this.observations_chirho.privacy_scope_chirho = "Successful fixed-string root scan after API saves/refusals and browser edit/restore; positive, missing and unreadable controls. Not absence of sealed storage, other encodings, cache retention, or K confirmation leakage.";
  }
}
