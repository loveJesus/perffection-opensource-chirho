// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// Workflows: product-delivery-chirho/reviewed-product-chirho.md and hosted-product-install-chirho.md.
// Where a publication outcome is observed. A local run drives one applier tick at a time and reads
// the tenant usage log. A hosted run can only wait for the supervised timer and read HTTP, so every
// arm that needs the host root is recorded as unexercised, never counted as a pass.
import { readFile } from "node:fs/promises";
import { join } from "node:path";
import type { ReviewedDeploymentChirho } from "../browser-chirho/reviewed-deployment-chirho";

export interface TickChirho { applied_chirho: number; intents_seen_chirho: number }

export interface PublicationObserverChirho {
  readonly local_chirho: boolean;
  readonly template_css_sha256_chirho: string;
  // Arms this run could not observe; a hosted verdict names them instead of passing them.
  readonly unexercised_chirho: string[];
  // States that exist only between applier ticks; a timer may apply before they can be read.
  readonly transient_chirho: { label_chirho: string; observed_chirho: boolean }[];
  tick_chirho(): Promise<TickChirho>;
  usage_chirho(): Promise<string>;
  verify_chirho(): void;
  settled_chirho(condition_chirho: () => Promise<boolean>): Promise<boolean>;
  quiet_period_chirho(): Promise<void>;
}

export class LocalPublicationObserverChirho implements PublicationObserverChirho {
  readonly local_chirho = true;
  readonly unexercised_chirho: string[] = [];
  readonly transient_chirho: { label_chirho: string; observed_chirho: boolean }[] = [];

  constructor(readonly fixture_chirho: ReviewedDeploymentChirho) {}

  get template_css_sha256_chirho(): string {
    return this.fixture_chirho.artifacts_chirho["perffection-page-template-chirho/assets-chirho/page-chirho.css"] ?? "";
  }

  async tick_chirho(): Promise<TickChirho> {
    return JSON.parse(this.fixture_chirho.run_chirho(["publication-apply", "--for-tenant", this.fixture_chirho.tenant_chirho, "--json"]));
  }

  async usage_chirho(): Promise<string> {
    const path_chirho = join(this.fixture_chirho.root_chirho, "tenants-chirho", this.fixture_chirho.tenant_chirho, "resource-usage-chirho.jsonl");
    if (!await Bun.file(path_chirho).exists()) return "0/0";
    // Owned two-account fixture only, not a product hot path or an unbounded production scan.
    const rows_chirho: { decision_chirho: string; section_chirho: string; charges_chirho: { resource_chirho: string; amount_chirho: number }[] }[] =
      (await readFile(path_chirho, "utf8")).trim().split("\n").filter(Boolean).map(line_chirho => JSON.parse(line_chirho));
    const allowed_chirho = rows_chirho.filter(row_chirho => row_chirho.decision_chirho === "AllowedChirho" && row_chirho.section_chirho === "publication-intent-chirho");
    return `${allowed_chirho.length}/${allowed_chirho.reduce((sum_chirho, row_chirho) => sum_chirho + row_chirho.charges_chirho
      .filter(charge_chirho => charge_chirho.resource_chirho === "publications-chirho")
      .reduce((total_chirho, charge_chirho) => total_chirho + charge_chirho.amount_chirho, 0), 0)}`;
  }

  verify_chirho(): void {
    this.fixture_chirho.run_chirho(["publication-verify"]);
  }

  async settled_chirho(): Promise<boolean> {
    throw Error("A local run ticks the applier itself; it never waits for a timer");
  }

  async quiet_period_chirho(): Promise<void> {
    throw Error("A local run ticks the applier itself; it never waits for a timer");
  }
}

export class HostedPublicationObserverChirho implements PublicationObserverChirho {
  readonly local_chirho = false;
  readonly unexercised_chirho: string[] = [];
  readonly transient_chirho: { label_chirho: string; observed_chirho: boolean }[] = [];

  // The period is the host timer's interval. Waits are bounded: three periods plus 15 s to apply,
  // and two full periods before claiming that nothing was applied.
  constructor(readonly template_css_sha256_chirho: string, readonly period_seconds_chirho: number) {
    if (!/^[a-f0-9]{64}$/.test(template_css_sha256_chirho)) throw Error("Hosted run needs the reviewed template stylesheet SHA-256");
    if (!Number.isInteger(period_seconds_chirho) || period_seconds_chirho < 1 || period_seconds_chirho > 300) throw Error("Invalid applier period");
  }

  async tick_chirho(): Promise<TickChirho> {
    throw Error("A hosted run cannot read the applier report; it waits for the supervised timer");
  }

  async usage_chirho(): Promise<string> {
    throw Error("A hosted run cannot read the tenant usage log");
  }

  verify_chirho(): void {
    this.unexercised_chirho.push("publication-verify on the platform root (host owner evidence)");
  }

  async settled_chirho(condition_chirho: () => Promise<boolean>): Promise<boolean> {
    const deadline_chirho = Date.now() + (this.period_seconds_chirho * 3 + 15) * 1000;
    while (Date.now() < deadline_chirho) {
      if (await condition_chirho()) return true;
      await Bun.sleep(2000);
    }
    return condition_chirho();
  }

  // A "nothing applied" claim holds only if a complete tick ran inside the wait. systemd places each
  // tick up to AccuracySec after its period (beta: 30 s + 5 s), and the run takes time after that, so
  // one period plus a margin can end just before the tick the claim relies on. Two periods contain a
  // whole tick whenever the accuracy window is shorter than the period (#25345).
  async quiet_period_chirho(): Promise<void> {
    await Bun.sleep(this.period_seconds_chirho * 2 * 1000);
  }
}
