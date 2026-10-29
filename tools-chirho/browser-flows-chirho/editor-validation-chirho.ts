// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// Workflow: spec-chirho/workflows-chirho/product-delivery-chirho/reviewed-product-chirho.md.
// Real customer forms on reviewed, synthetic roots. No injected HTML, scripts or hidden-field edits.
import { join } from "node:path";
import type { Locator as LocatorChirho, Page as PageChirho } from "playwright-core";
import type { JourneyFixtureChirho } from "./workspace-session-chirho";
import { control_and_label_visible_chirho, tab_to_chirho } from "./form-keyboard-chirho";

interface ExportChirho {
  json_chirho: string;
  download_chirho: string | null;
}

interface ImportedDocumentChirho {
  schema_version_chirho: number;
  headline_chirho: string;
  contact_phone_chirho: string;
  contact_email_chirho: string;
  services_chirho: { title_chirho: string; body_chirho: string }[];
}

export class EditorValidationJourneyChirho {
  readonly workspace_chirho: string;
  readonly observations_chirho: Record<string, unknown> = {};
  readonly keyboard_steps_chirho: { name_chirho: string; tabs_chirho: number; backwards_chirho: boolean; label_visible_chirho: boolean }[] = [];

  constructor(readonly fixture_chirho: JourneyFixtureChirho, origin_chirho: string, readonly output_chirho: string) {
    this.workspace_chirho = `${origin_chirho}/app-chirho/perffection-workspace/`;
  }

  private check_chirho(condition_chirho: unknown, label_chirho: string): void {
    this.fixture_chirho.check_chirho(condition_chirho, `editor validation: ${label_chirho}`);
  }

  private form_chirho(page_chirho: PageChirho): LocatorChirho {
    return page_chirho.locator('form[action$="edit-page-chirho"]');
  }

  private async version_chirho(page_chirho: PageChirho): Promise<string> {
    return await this.form_chirho(page_chirho).locator('[name="expected_version_chirho"]').inputValue();
  }

  private async submit_chirho(page_chirho: PageChirho, form_chirho: LocatorChirho, keyboard_chirho = false): Promise<void> {
    this.check_chirho(await form_chirho.evaluate(form_chirho => (form_chirho as HTMLFormElement).checkValidity()),
      "submitted form satisfies native constraints without disabling them");
    const response_chirho = page_chirho.waitForResponse(response_chirho => response_chirho.request().isNavigationRequest()
      && response_chirho.request().method() === "POST");
    response_chirho.catch(() => undefined); // guarded: a failed action must not leave this rejection unhandled; awaiting it still raises
    const button_chirho = form_chirho.locator('button[type="submit"]');
    if (keyboard_chirho) await tab_to_chirho(page_chirho, button_chirho);
    await Promise.all([page_chirho.waitForNavigation({ waitUntil: "networkidle" }),
      keyboard_chirho ? page_chirho.keyboard.press("Enter") : button_chirho.click()]);
    this.check_chirho((await response_chirho).status() === 200, "actual form POST returns its customer page");
  }

  private async fill_chirho(page_chirho: PageChirho, values_chirho: Record<string, string>): Promise<void> {
    for (const [name_chirho, value_chirho] of Object.entries(values_chirho)) {
      await this.form_chirho(page_chirho).locator(`[name="${name_chirho}"]`).fill(value_chirho);
    }
  }

  private async export_chirho(page_chirho: PageChirho): Promise<ExportChirho> {
    const export_page_chirho = await page_chirho.context().newPage();
    try {
      const response_chirho = await export_page_chirho.goto(`${this.workspace_chirho}page-document-chirho`);
      this.check_chirho(response_chirho?.status() === 200, "saved-document export is served to its viewer");
      return { json_chirho: await export_page_chirho.locator('textarea[readonly]').inputValue(),
        download_chirho: await export_page_chirho.locator('a[download]').getAttribute("download") };
    } finally { await export_page_chirho.close(); }
  }

  private async unchanged_chirho(page_chirho: PageChirho, before_chirho: ExportChirho, label_chirho: string): Promise<void> {
    this.check_chirho(JSON.stringify(await this.export_chirho(page_chirho)) === JSON.stringify(before_chirho),
      `${label_chirho}: saved document bytes and exported version are unchanged`);
  }

  private async semantics_chirho(page_chirho: PageChirho, invalid_names_chirho: string[], issues_chirho: number): Promise<void> {
    const state_chirho = await page_chirho.evaluate(() => {
      const ids_chirho = [...document.querySelectorAll('[id]')].map(element_chirho => element_chirho.id);
      const references_chirho = [...document.querySelectorAll('[aria-describedby], [aria-labelledby]')].flatMap(element_chirho =>
        ["aria-describedby", "aria-labelledby"].flatMap(attribute_chirho => (element_chirho.getAttribute(attribute_chirho) ?? "")
          .split(/\s+/).filter(Boolean).map(id_chirho => ({ id_chirho,
            present_chirho: !!document.getElementById(id_chirho)?.textContent?.trim() }))));
      const fields_chirho = [...document.querySelectorAll<HTMLInputElement | HTMLTextAreaElement>('form[action$="edit-page-chirho"] input:not([type="hidden"]), form[action$="edit-page-chirho"] textarea')]
        .map(control_chirho => ({ name_chirho: control_chirho.name,
          invalid_chirho: control_chirho.getAttribute("aria-invalid") === "true",
          labels_chirho: [...(control_chirho.labels ?? [])].map(label_chirho => ({
            text_chirho: label_chirho.textContent?.trim(), help_inside_chirho: !!label_chirho.querySelector('small, .workspace-field-error-chirho') })),
          descriptions_chirho: (control_chirho.getAttribute("aria-describedby") ?? "").split(/\s+/).filter(Boolean)
            .map(id_chirho => document.getElementById(id_chirho)).filter(element_chirho => element_chirho !== null),
          help_chirho: control_chirho.parentElement?.querySelector('small[id]')?.id,
        })).map(({ descriptions_chirho, ...field_chirho }) => ({ ...field_chirho,
          described_ids_chirho: descriptions_chirho.map(element_chirho => element_chirho.id),
          has_error_chirho: descriptions_chirho.some(element_chirho => element_chirho.classList.contains("workspace-field-error-chirho")),
        }));
      const summary_chirho = [...document.querySelectorAll('[role="alert"] li')].map(item_chirho => {
        const link_chirho = item_chirho.querySelector('a[href^="#"]');
        const target_chirho = document.getElementById(link_chirho?.getAttribute("href")?.slice(1) ?? "");
        return { text_chirho: item_chirho.textContent?.trim(), name_chirho: target_chirho?.getAttribute("name"),
          control_chirho: target_chirho instanceof HTMLInputElement || target_chirho instanceof HTMLTextAreaElement };
      });
      const messages_chirho = [...document.querySelectorAll('.workspace-field-error-chirho')].map(element_chirho => ({
        id_chirho: element_chirho.id, text_chirho: element_chirho.textContent?.trim(),
      }));
      return { unique_ids_chirho: new Set(ids_chirho).size === ids_chirho.length, references_chirho, fields_chirho, summary_chirho, messages_chirho };
    });
    this.observations_chirho.last_semantics_chirho = state_chirho;
    this.check_chirho(state_chirho.unique_ids_chirho && state_chirho.references_chirho.every(reference_chirho => reference_chirho.present_chirho),
      "all description/name references resolve to nonempty text and IDs are unique");
    this.check_chirho(state_chirho.fields_chirho.length === 13 && state_chirho.fields_chirho.every(field_chirho =>
      field_chirho.labels_chirho.length === 1 && field_chirho.labels_chirho[0].text_chirho && !field_chirho.labels_chirho[0].help_inside_chirho
      && (!field_chirho.help_chirho || field_chirho.described_ids_chirho.includes(field_chirho.help_chirho))),
    "every editor control has a distinct label; help describes it rather than inflating its name");
    const actual_chirho = state_chirho.fields_chirho.filter(field_chirho => field_chirho.invalid_chirho).map(field_chirho => field_chirho.name_chirho).sort();
    this.check_chirho(JSON.stringify(actual_chirho) === JSON.stringify([...invalid_names_chirho].sort()),
      `invalid state belongs only to affected controls (${actual_chirho.join(", ") || "none"})`);
    this.check_chirho(state_chirho.fields_chirho.every(field_chirho => field_chirho.invalid_chirho === field_chirho.has_error_chirho),
      "each invalid control is described by its error; clean controls have no stale error description");
    this.check_chirho(state_chirho.summary_chirho.length === issues_chirho
      && state_chirho.summary_chirho.every(item_chirho => item_chirho.text_chirho && item_chirho.control_chirho
        && invalid_names_chirho.includes(item_chirho.name_chirho ?? "")), "every induced issue has a usable summary destination");
    this.check_chirho(state_chirho.messages_chirho.length === issues_chirho && state_chirho.messages_chirho.every(message_chirho =>
      message_chirho.text_chirho && state_chirho.summary_chirho.some(item_chirho => item_chirho.text_chirho?.includes(message_chirho.text_chirho!))),
    "every induced message appears once at its field/group home and in the summary");
  }

  private async group_chirho(page_chirho: PageChirho, names_chirho: string[]): Promise<void> {
    const result_chirho = await page_chirho.evaluate(names_chirho => {
      const controls_chirho = names_chirho.map(name_chirho => document.querySelector<HTMLInputElement>(`[name="${name_chirho}"]`));
      const group_chirho = controls_chirho[0]?.closest('fieldset');
      const message_chirho = group_chirho?.querySelector(':scope > .workspace-field-error-chirho');
      return !!group_chirho?.querySelector(':scope > legend')?.textContent?.trim() && !!message_chirho?.id
        && (group_chirho.getAttribute("aria-describedby") ?? "").split(/\s+/).includes(message_chirho.id)
        && controls_chirho.every(control_chirho => control_chirho?.closest('fieldset') === group_chirho
          && (control_chirho?.getAttribute("aria-describedby") ?? "").split(/\s+/).includes(message_chirho.id));
    }, names_chirho);
    this.check_chirho(result_chirho, `one semantic group message describes both members: ${names_chirho.join(", ")}`);
  }

  private async keyboard_correct_chirho(page_chirho: PageChirho, name_chirho: string, value_chirho: string,
    backwards_chirho = false, form_chirho = this.form_chirho(page_chirho)): Promise<void> {
    const control_chirho = form_chirho.locator(`[name="${name_chirho}"]`);
    const id_chirho = await control_chirho.getAttribute("id");
    this.check_chirho(!!id_chirho, "summary target has an actual control ID");
    const link_chirho = page_chirho.locator(`[role="alert"] a[href="#${id_chirho}"]`);
    const tabs_chirho = await tab_to_chirho(page_chirho, link_chirho, backwards_chirho);
    await page_chirho.keyboard.press("Enter");
    this.check_chirho(await control_chirho.evaluate(element_chirho => document.activeElement === element_chirho),
      `keyboard summary activation focuses ${name_chirho}, not just the page fragment`);
    const visible_chirho = await control_and_label_visible_chirho(control_chirho);
    this.check_chirho(visible_chirho, `${name_chirho}: focused control and label are visible, not covered by chrome`);
    await page_chirho.keyboard.press("ControlOrMeta+A");
    await page_chirho.keyboard.insertText(value_chirho);
    this.check_chirho(await control_chirho.inputValue() === value_chirho, `keyboard text reaches ${name_chirho}`);
    this.keyboard_steps_chirho.push({ name_chirho, tabs_chirho, backwards_chirho, label_visible_chirho: visible_chirho });
    this.observations_chirho.keyboard_steps_chirho = this.keyboard_steps_chirho;
  }

  private async import_semantics_chirho(page_chirho: PageChirho, paste_chirho: string, version_chirho: string,
    rejected_chirho: boolean): Promise<string> {
    const form_chirho = page_chirho.locator('form[action$="review-page-document-chirho"]');
    const control_chirho = form_chirho.locator('[name="page_document_chirho"]');
    this.check_chirho(await page_chirho.locator('form').count() === 1 && await form_chirho.count() === 1
      && await control_chirho.count() === 1, "import has one real review form and one paste control");
    this.check_chirho(await control_chirho.inputValue() === paste_chirho
      && await form_chirho.locator('[name="expected_version_chirho"]').inputValue() === version_chirho,
    "import keeps the complete pasted text and original expected version");
    const visible_alerts_chirho = page_chirho.getByRole("main").getByRole("alert");
    const alert_texts_chirho = await visible_alerts_chirho.allTextContents();
    const summary_hrefs_chirho = await visible_alerts_chirho.locator('a[href^="#"]').evaluateAll(links_chirho =>
      links_chirho.map(link_chirho => link_chirho.getAttribute("href")));
    const state_chirho = await control_chirho.evaluate((element_chirho, alerts_chirho) => {
      const control_chirho = element_chirho as HTMLTextAreaElement;
      const ids_chirho = [...document.querySelectorAll('[id]')].map(element_chirho => element_chirho.id);
      const label_chirho = [...(control_chirho.labels ?? [])];
      const help_chirho = control_chirho.parentElement?.querySelector('small[id]');
      const messages_chirho = [...document.querySelectorAll('.workspace-field-error-chirho')];
      const described_chirho = (control_chirho.getAttribute("aria-describedby") ?? "").split(/\s+/).filter(Boolean);
      return { unique_ids_chirho: new Set(ids_chirho).size === ids_chirho.length,
        labelled_chirho: label_chirho.length === 1 && !!label_chirho[0].textContent?.trim()
          && !label_chirho[0].querySelector('small, .workspace-field-error-chirho'),
        described_chirho: described_chirho.length > 0 && described_chirho.every(id_chirho =>
          !!document.getElementById(id_chirho)?.textContent?.trim()),
        help_chirho: !!help_chirho?.id && described_chirho.includes(help_chirho.id),
        invalid_chirho: control_chirho.getAttribute("aria-invalid") === "true",
        message_chirho: messages_chirho[0]?.textContent?.trim() ?? "",
        messages_chirho: messages_chirho.length,
        message_described_chirho: messages_chirho.length === 1 && described_chirho.includes(messages_chirho[0].id)
          && messages_chirho[0].parentElement === control_chirho.parentElement,
        alerts_chirho: alerts_chirho.alert_texts_chirho.length,
        linked_chirho: alerts_chirho.summary_hrefs_chirho.length === 1
          && alerts_chirho.summary_hrefs_chirho[0] === `#${control_chirho.id}`,
        summary_chirho: alerts_chirho.alert_texts_chirho.join("\n"),
      };
    }, { alert_texts_chirho, summary_hrefs_chirho });
    this.check_chirho(state_chirho.unique_ids_chirho && state_chirho.labelled_chirho && state_chirho.described_chirho
      && state_chirho.help_chirho, "import label is separate from help; every description resolves and IDs are unique");
    this.check_chirho(rejected_chirho
      ? state_chirho.invalid_chirho && state_chirho.messages_chirho === 1 && state_chirho.message_described_chirho
        && !!state_chirho.message_chirho && state_chirho.alerts_chirho === 1 && state_chirho.linked_chirho
        && state_chirho.summary_chirho.includes(state_chirho.message_chirho)
      : !state_chirho.invalid_chirho && state_chirho.messages_chirho === 0 && state_chirho.alerts_chirho === 0,
    rejected_chirho ? "one import reason sits beside the invalid paste and is linked once from the summary"
      : "clean import has no stale refusal or invalid state");
    this.check_chirho(await page_chirho.locator('img[src="x"]').count() === 0,
      "HTML-looking pasted words remain text, never an injected image");
    return state_chirho.message_chirho;
  }

  private async recover_import_chirho(page_chirho: PageChirho, before_chirho: ExportChirho,
    document_chirho: ImportedDocumentChirho): Promise<void> {
    const version_chirho = await this.version_chirho(page_chirho);
    await page_chirho.locator('nav a[href$="import-page-document-chirho"]').click();
    await page_chirho.waitForURL(`${this.workspace_chirho}import-page-document-chirho`);
    await this.import_semantics_chirho(page_chirho, "", version_chirho, false);
    const form_chirho = page_chirho.locator('form[action$="review-page-document-chirho"]');
    const malformed_chirho = JSON.stringify({ ...document_chirho,
      headline_chirho: "My pasted café </textarea><img src=x> & words" }, null, 2).slice(0, -1);
    await form_chirho.locator('[name="page_document_chirho"]').fill(malformed_chirho);
    await this.submit_chirho(page_chirho, form_chirho);
    const malformed_message_chirho = await this.import_semantics_chirho(page_chirho, malformed_chirho, version_chirho, true);
    await this.unchanged_chirho(page_chirho, before_chirho, "malformed paste refusal");
    await this.layouts_chirho(page_chirho, "import-refused-chirho");

    const unsupported_chirho = JSON.stringify({ ...document_chirho, schema_version_chirho: 999 });
    await this.keyboard_correct_chirho(page_chirho, "page_document_chirho", unsupported_chirho, false, form_chirho);
    await this.submit_chirho(page_chirho, form_chirho, true);
    const unsupported_message_chirho = await this.import_semantics_chirho(page_chirho, unsupported_chirho, version_chirho, true);
    this.check_chirho(unsupported_message_chirho !== malformed_message_chirho,
      "unsupported document replaces the previous parse reason instead of keeping a stale refusal");
    await this.unchanged_chirho(page_chirho, before_chirho, "unsupported document refusal");
    this.observations_chirho.import_refusals_chirho = { version_chirho, malformed_message_chirho, unsupported_message_chirho,
      pasted_characters_chirho: [malformed_chirho.length, unsupported_chirho.length] };

    await this.keyboard_correct_chirho(page_chirho, "page_document_chirho", JSON.stringify(document_chirho), false, form_chirho);
    await this.submit_chirho(page_chirho, form_chirho, true);
    this.check_chirho(await form_chirho.count() === 0 && await this.form_chirho(page_chirho).count() === 1
      && await page_chirho.locator('textarea[name="page_document_chirho"]').count() === 0,
    "correcting the paste reaches the ordinary review editor, not another refused-paste page");
  }

  private async layouts_chirho(page_chirho: PageChirho, label_chirho: string): Promise<void> {
    const widths_chirho: { viewport_chirho: number; content_chirho: number }[] = [];
    for (const width_chirho of [320, 390, 768, 1440]) {
      await page_chirho.setViewportSize({ width: width_chirho, height: 900 });
      await page_chirho.evaluate(() => document.fonts.ready);
      const measured_chirho = await page_chirho.evaluate(() => ({ viewport_chirho: innerWidth, content_chirho: document.documentElement.scrollWidth }));
      widths_chirho.push(measured_chirho);
      this.check_chirho(measured_chirho.content_chirho === measured_chirho.viewport_chirho, `${label_chirho}: reflows at ${width_chirho}px`);
      if ([390, 1440].includes(width_chirho)) await page_chirho.screenshot({ path: join(this.output_chirho, `editor-${label_chirho}-${width_chirho}-chirho.png`), fullPage: true });
    }
    this.observations_chirho[`${label_chirho}_widths_chirho`] = widths_chirho;
    await page_chirho.setViewportSize({ width: 390, height: 900 });
  }

  async run_chirho(page_chirho: PageChirho): Promise<Record<string, unknown>> {
    await page_chirho.goto(this.workspace_chirho);
    this.check_chirho(await this.version_chirho(page_chirho) === "0", "validation fixture starts with no saved page");
    await this.semantics_chirho(page_chirho, [], 0);
    await this.fill_chirho(page_chirho, { business_name_chirho: "Keyboard Bakery", headline_chirho: "Saved first headline",
      summary_chirho: "Synthetic bakery for local editor acceptance.", service_1_title_chirho: "Fresh bread",
      service_1_body_chirho: "Baked each morning.", contact_heading_chirho: "Visit our bakery", contact_phone_chirho: "+1 202 555 0147" });
    await this.submit_chirho(page_chirho, this.form_chirho(page_chirho));
    const saved_title_chirho = await page_chirho.title();
    const first_chirho = await this.export_chirho(page_chirho);
    this.check_chirho(await this.version_chirho(page_chirho) === "1", "baseline is saved version one");

    const unsaved_headline_chirho = "My unsaved words survive correction";
    await this.fill_chirho(page_chirho, { headline_chirho: unsaved_headline_chirho, service_2_title_chirho: "Afternoon loaves",
      service_2_body_chirho: "", contact_phone_chirho: "", contact_email_chirho: "" });
    await this.submit_chirho(page_chirho, this.form_chirho(page_chirho));
    await this.semantics_chirho(page_chirho, ["contact_phone_chirho", "contact_email_chirho", "service_2_body_chirho"], 2);
    await this.group_chirho(page_chirho, ["contact_phone_chirho", "contact_email_chirho"]);
    this.check_chirho(await page_chirho.title() !== saved_title_chirho && await this.version_chirho(page_chirho) === "1"
      && await this.form_chirho(page_chirho).locator('[name="headline_chirho"]').inputValue() === unsaved_headline_chirho,
    "refused editor keeps her words and expected version, with a distinct unsaved title");
    this.check_chirho(await page_chirho.locator('main a[href$="publication-status-chirho"]').count() === 0,
      "rejected words are not accompanied by a saved-page publication affordance");
    await this.unchanged_chirho(page_chirho, first_chirho, "refused save");
    await this.layouts_chirho(page_chirho, "refused-chirho");
    await this.keyboard_correct_chirho(page_chirho, "contact_phone_chirho", "+1 202 555 0199");
    await this.keyboard_correct_chirho(page_chirho, "service_2_body_chirho", "Fresh loaves in the afternoon.", true);
    await this.submit_chirho(page_chirho, this.form_chirho(page_chirho), true);
    await this.semantics_chirho(page_chirho, [], 0);
    this.check_chirho(await this.version_chirho(page_chirho) === "2" && await page_chirho.title() === saved_title_chirho,
      "keyboard correction saves once and resets the title and all invalid states");
    const second_chirho = await this.export_chirho(page_chirho);

    const imported_chirho = JSON.parse(second_chirho.json_chirho) as ImportedDocumentChirho;
    imported_chirho.headline_chirho = "Imported words are not saved yet";
    imported_chirho.contact_phone_chirho = "";
    imported_chirho.contact_email_chirho = "";
    imported_chirho.services_chirho = imported_chirho.services_chirho.map(() => ({ title_chirho: "", body_chirho: "" }));
    await this.recover_import_chirho(page_chirho, second_chirho, imported_chirho);
    await this.semantics_chirho(page_chirho, ["contact_phone_chirho", "contact_email_chirho", "service_1_title_chirho", "service_1_body_chirho"], 2);
    await this.group_chirho(page_chirho, ["contact_phone_chirho", "contact_email_chirho"]);
    await this.group_chirho(page_chirho, ["service_1_title_chirho", "service_1_body_chirho"]);
    this.check_chirho(await this.version_chirho(page_chirho) === "2" && await page_chirho.locator('main a[href$="publication-status-chirho"]').count() === 0,
      "import review keeps the saved head and offers no approval of unsaved content");
    await this.unchanged_chirho(page_chirho, second_chirho, "import review");
    await this.layouts_chirho(page_chirho, "import-chirho");
    await this.keyboard_correct_chirho(page_chirho, "service_1_title_chirho", "Recovered bread");
    await tab_to_chirho(page_chirho, this.form_chirho(page_chirho).locator('[name="service_1_body_chirho"]'));
    await page_chirho.keyboard.insertText("A recovered description.");
    await this.keyboard_correct_chirho(page_chirho, "contact_phone_chirho", "+1 202 555 0166", true);
    await this.submit_chirho(page_chirho, this.form_chirho(page_chirho), true);
    await this.semantics_chirho(page_chirho, [], 0);
    this.check_chirho(await this.version_chirho(page_chirho) === "3"
      && (await this.export_chirho(page_chirho)).json_chirho.includes(imported_chirho.headline_chirho),
    "explicit save, not import review, appends the corrected imported content");

    const stale_chirho = await page_chirho.context().newPage();
    try {
      await stale_chirho.goto(this.workspace_chirho);
      await this.fill_chirho(stale_chirho, { headline_chirho: "Words in the older window" });
      await this.fill_chirho(page_chirho, { headline_chirho: "Newer saved words" });
      await this.submit_chirho(page_chirho, this.form_chirho(page_chirho));
      const fourth_chirho = await this.export_chirho(page_chirho);
      await this.submit_chirho(stale_chirho, this.form_chirho(stale_chirho));
      this.check_chirho(await stale_chirho.getByRole("main").getByRole("alert").count() === 1 && await stale_chirho.title() !== saved_title_chirho
        && await this.version_chirho(stale_chirho) === "4"
        && await this.form_chirho(stale_chirho).locator('[name="headline_chirho"]').inputValue() === "Words in the older window",
      "stale save explains the conflict, keeps her words and rebases only the expected version");
      await this.semantics_chirho(stale_chirho, [], 0);
      await this.unchanged_chirho(stale_chirho, fourth_chirho, "stale save");
      await this.layouts_chirho(stale_chirho, "conflict-chirho");
      await this.submit_chirho(stale_chirho, this.form_chirho(stale_chirho));
      this.check_chirho(await this.version_chirho(stale_chirho) === "5" && await stale_chirho.getByRole("main").getByRole("alert").count() === 0,
        "explicit conflict resubmission saves her retained words once");
      await this.semantics_chirho(stale_chirho, [], 0);
    } finally { await stale_chirho.close(); }
    await page_chirho.goto(this.workspace_chirho);
    await this.layouts_chirho(page_chirho, "saved-chirho");
    this.observations_chirho.final_version_chirho = await this.version_chirho(page_chirho);
    this.observations_chirho.final_document_sha256_chirho = new Bun.CryptoHasher("sha256")
      .update((await this.export_chirho(page_chirho)).json_chirho).digest("hex");
    return this.observations_chirho;
  }
}
