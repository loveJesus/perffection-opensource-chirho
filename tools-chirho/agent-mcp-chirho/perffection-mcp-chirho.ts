#!/usr/bin/env node
// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// perffection-mcp-chirho: software a Perffection customer runs on their OWN machine, so their own
// agent (Claude Code, or any MCP client) can read and save their PRIVATE page and their private
// business figures. It speaks MCP over stdio in both eras (the stateless 2026-07-28 revision, and the
// initialize handshake of 2025-11-25 and earlier) and wraps exactly five existing routes of the
// workspace: read the page, save the page, preview it, read the figures, save the figures. It never
// publishes: publishing stays the owner's own click in the workspace.
// Zero dependencies on purpose; `bun` or `node` (22.18 or later) runs this file as it is.
// Read README-chirho.md beside it. Workflows: spec-chirho/workflows-chirho/invite-beta-chirho/agent-api-chirho.md
// and her-figures-chirho.md beside it.
import { randomUUID } from "node:crypto";
import { readFileSync, statSync } from "node:fs";

type JsonChirho = null | boolean | number | string | JsonChirho[] | { [key_chirho: string]: JsonChirho };
type JsonObjectChirho = { [key_chirho: string]: JsonChirho };
type RpcIdChirho = string | number;
type EraChirho = "modern" | "legacy";
type ConfigChirho = { origin_chirho: string; key_chirho: string; timeout_ms_chirho: number };
type RouteOutcomeChirho =
  | { kind_chirho: "answer"; body_chirho: JsonObjectChirho | string }
  | RefusedOutcomeChirho
  | { kind_chirho: "none"; status_chirho: number | null; meaning_chirho: string; reason_chirho?: NoAnswerReasonChirho };
type RefusedOutcomeChirho = { kind_chirho: "refused" } & RefusalChirho;
// A fixed refusal recognized whole (FIXED_REFUSALS_CHIRHO). may_count_use_chirho: whether the save's lead
// may say the request may already be counted as a use of the key. reason_chirho is set only where a read's
// no-answer names the refusal; retry_after_seconds_chirho only where the answer carried its wait.
type RefusalChirho = {
  status_chirho: number; identity_chirho: string; meaning_chirho: string; may_count_use_chirho: boolean;
  reason_chirho?: RateRefusalIdentityChirho; retry_after_seconds_chirho?: number;
};
// What a 404 can be said to mean, as a code an agent can act on. A health answer is only a liveness
// hint: it does not prove Perffection is there, and a failed one may be the right address's outage
// (gpt #26555). So neither code claims more than that.
type NotFoundReasonChirho = "tool-route-absent-chirho" | "address-unconfirmed-chirho";
// Which of Cairn's two 429s a read's no-answer was, by the engine's own identity: the served gate busy
// (it says how long to wait), or the key's own call budget used up (it says no wait).
type RateRefusalIdentityChirho = "served-request-unavailable-chirho" | "rate-limit-exceeded-chirho";
type NoAnswerReasonChirho = NotFoundReasonChirho | RateRefusalIdentityChirho;

const SERVER_INFO_CHIRHO = { name: "perffection-mcp-chirho", title: "Perffection page and figures tools", version: "0.1.0" };
const MODERN_VERSIONS_CHIRHO: readonly string[] = ["2026-07-28"];
const LEGACY_VERSIONS_CHIRHO: readonly string[] = ["2025-11-25", "2025-06-18", "2025-03-26", "2024-11-05"];
const ALL_VERSIONS_CHIRHO: string[] = [...MODERN_VERSIONS_CHIRHO, ...LEGACY_VERSIONS_CHIRHO];
const META_VERSION_CHIRHO = "io.modelcontextprotocol/protocolVersion";
const META_CAPABILITIES_CHIRHO = "io.modelcontextprotocol/clientCapabilities";
const META_SERVER_INFO_CHIRHO = "io.modelcontextprotocol/serverInfo";
const CAPABILITIES_CHIRHO = { tools: {} };

// The five routes are fixed here: no tool argument ever becomes part of a URL. The figures routes are
// held out of every release until their activation is decided (gpt #26554, #26625), so a released
// server answers them 404 and the tools say the route is absent there, as for any server older than
// this adapter.
const READ_ROUTE_CHIRHO = "/app-chirho/perffection-workspace/agent-page-chirho";
const SAVE_ROUTE_CHIRHO = "/app-chirho/perffection-workspace/agent-save-page-chirho";
const PREVIEW_ROUTE_CHIRHO = "/app-chirho/perffection-workspace/preview-chirho";
const FIGURES_ROUTE_CHIRHO = "/app-chirho/perffection-workspace/agent-figures-chirho";
const SAVE_FIGURES_ROUTE_CHIRHO = "/app-chirho/perffection-workspace/agent-save-figures-chirho";
const SAVE_TOOLS_CHIRHO: ReadonlySet<string> = new Set(["save_page_chirho", "save_figures_chirho"]);

// Each JSON answer is recognized by its own kind, exactly (gpt #26637): a read schema and state only on a
// read, a save schema and result shape only on a save, so a save answer never reads as figures, a read
// answer never as a save, and a page answer never as figures. Anything else is no answer.
const PAGE_READ_SCHEMA_CHIRHO = "perffection-agent-page-v1-chirho";
const PAGE_SAVE_SCHEMA_CHIRHO = "perffection-agent-save-v1-chirho";
const FIGURES_READ_SCHEMA_CHIRHO = "perffection-agent-figures-v1-chirho";
const FIGURES_SAVE_SCHEMA_CHIRHO = "perffection-agent-figures-save-v1-chirho";
const READ_STATES_CHIRHO: ReadonlySet<unknown> = new Set(["saved-chirho", "nothing-saved-chirho", "unverifiable-chirho"]);

// Cairn's fixed answers for a limited key a route will not run (K1, landed at c0a12269), each exactly
// as it comes back to a request asking for JSON: this status, Content-Type application/json, and the
// body {"error_chirho":"<identity>","ok_chirho":false} with nothing before, after or between. The three
// 403s are served and measured in PFE's K1 gate. The 409 is NOT a PFE served measurement: it rests on
// Cairn's source and its own hold-point test (its window lies inside the engine), rendered by the same
// code. A save answered with one of these EXACTLY was refused before Perffection ran its DAG (gpt #26636,
// #26662): THIS request's DAG did not run, so it changed nothing through that DAG. The engine may already
// have counted the request as a use of the key (the 409 comes after that). It says nothing about an
// earlier save whose answer never came, and it authorizes no retry. Their Retry-After is not read.
//
// Cairn's two 429s, for a request asking for JSON (R13 Part 8, read in the R16 engine a07696ca), told
// apart whole, Retry-After included (gpt #28569):
// - Busy: the served gate's ONE refusal, whichever limit fired (served_request_rejected_response_chirho).
//   Content-Type application/json, the constant body below, and exactly one Retry-After. The gate admits
//   after the key check and before any DAG is selected or run (run_app_http_with_method_for_served_
//   request_and_uri_chirho), so THIS request's DAG did not run. The key's use is recorded before the gate
//   on one of that function's paths and after it on the other, so the request may already be counted.
// - The key's own call budget used up: the request-context refusal rate-limit-exceeded-chirho
//   (oauth_chirho::RateLimitExceededChirho through public_error_response_chirho), with K1's body form and
//   NO Retry-After. It is answered before any DAG run on both paths. Whether it counted as a use of the
//   key, this adapter does not say.
// Every other refusal, any other 429 included, is unknown.
type FixedRefusalChirho = RefusalChirho & { body_chirho: string; retry_after_chirho: "unread" | "seconds" | "none" };
const engine_refusal_body_chirho = (identity_chirho: string): string => `{"error_chirho":"${identity_chirho}","ok_chirho":false}`;
const k1_refusal_chirho = (status_chirho: number, identity_chirho: string, meaning_chirho: string): FixedRefusalChirho =>
  ({ status_chirho, identity_chirho, meaning_chirho, body_chirho: engine_refusal_body_chirho(identity_chirho), retry_after_chirho: "unread", may_count_use_chirho: true });
const SERVED_GATE_BUSY_BODY_CHIRHO = '{"ok_chirho":false,"error_chirho":"served-request-unavailable-chirho"}';
const FIXED_REFUSALS_CHIRHO: ReadonlyArray<FixedRefusalChirho> = [
  k1_refusal_chirho(403, "key-route-not-open-chirho", "this route does not accept this key"),
  k1_refusal_chirho(403, "key-action-not-granted-chirho", "this key is not granted this tool's action; the owner can issue a key that is"),
  k1_refusal_chirho(403, "key-action-authority-changed-chirho", "this route can now do more than this key was granted; the owner can issue a new key"),
  k1_refusal_chirho(409, "key-route-changed-chirho", "the route changed after this key was admitted, so this request's DAG did not run; the request may still count as a use of the key"),
  {
    status_chirho: 429, identity_chirho: "served-request-unavailable-chirho", reason_chirho: "served-request-unavailable-chirho",
    meaning_chirho: "Perffection is busy: its request gate refused this request and says to wait", // the wait is added from Retry-After
    body_chirho: SERVED_GATE_BUSY_BODY_CHIRHO, retry_after_chirho: "seconds", may_count_use_chirho: true,
  },
  {
    status_chirho: 429, identity_chirho: "rate-limit-exceeded-chirho", reason_chirho: "rate-limit-exceeded-chirho",
    meaning_chirho: "this key's call budget is used up for now; the answer names no wait",
    body_chirho: engine_refusal_body_chirho("rate-limit-exceeded-chirho"), retry_after_chirho: "none", may_count_use_chirho: false,
  },
];
const FIXED_REFUSAL_MAX_BYTES_CHIRHO = 256;
// A fixed body's exact bytes (each is ASCII, and an encoder writes no byte-order mark), compared byte for byte.
const UTF8_ENCODER_CHIRHO = new TextEncoder();
function same_bytes_chirho(received_chirho: Uint8Array, body_chirho: string): boolean {
  const wanted_chirho = UTF8_ENCODER_CHIRHO.encode(body_chirho);
  return received_chirho.length === wanted_chirho.length && received_chirho.every((byte_chirho, index_chirho) => byte_chirho === wanted_chirho[index_chirho]);
}
// The busy answer's wait. The engine writes Retry-After as a u64 in decimal: its window's whole seconds,
// rounded up (retry_after_secs_chirho), so at least 1, since a window is never 0. The window is a u64 of
// milliseconds that the host's TOML sets with no upper bound (ServedRequestAbuseLimitsChirho::validate_chirho
// refuses only 0), so the engine's own maximum is ceil((2^64 - 1) / 1000) = 18446744073709552 seconds, not
// the 60 of the default window. This adapter carries the wait as a JSON number, exact only up to 2^53 - 1,
// so it recognizes 1 to Number.MAX_SAFE_INTEGER: digits only, no leading zero. Anything else stays
// unknown rather than rounded: no header, an empty one, a date, a sign, a fraction, a leading zero, two
// headers joined by a comma, or a value past that bound.
const RETRY_AFTER_PATTERN_CHIRHO = /^[1-9][0-9]{0,15}$/;
function retry_after_seconds_chirho(value_chirho: string | null): number | undefined {
  if (value_chirho === null || !RETRY_AFTER_PATTERN_CHIRHO.test(value_chirho)) return undefined;
  const seconds_chirho = Number(value_chirho);
  return Number.isSafeInteger(seconds_chirho) ? seconds_chirho : undefined;
}

// Bounds: one stdin line, one answer body, tool calls waiting, the wait for an answer.
const LINE_MAX_CHARS_CHIRHO = 256 * 1024;
const ANSWER_MAX_BYTES_CHIRHO = 1024 * 1024;
const MAX_QUEUED_CALLS_CHIRHO = 16;
const DEFAULT_TIMEOUT_MS_CHIRHO = 30_000;
// RFC 6750 token68: the engine's keys are 64 hex characters, which this admits.
const KEY_PATTERN_CHIRHO = /^[A-Za-z0-9._~+/-]+=*$/;
const LOOPBACK_HOSTS_CHIRHO = new Set(["127.0.0.1", "localhost", "[::1]"]);

const INSTRUCTIONS_CHIRHO =
  "Perffection page tools for the page owner's own agent. Read first (read_page_chirho): the answer holds the page, " +
  "the version to save against, and surface_chirho, which names every field, limit and refusal. Save the COMPLETE page " +
  "against that version (save_page_chirho): a save keeps a new private version and never publishes. Preview with " +
  "preview_page_chirho. The owner publishes themselves, signed in to their workspace. If a save's result is unknown, " +
  "read the page again before saving again. The owner's saved business figures and their report: read_figures_chirho; " +
  "its meanings_chirho says how to read every number, and report_chirho.complete_chirho says whether the totals cover " +
  "every month. Save the COMPLETE figures against the version you read (save_figures_chirho); it never publishes. A key " +
  "may be granted some of these tools and not others: a tool its key was never granted is refused before it runs.";
const NO_ARGUMENTS_CHIRHO = { type: "object", properties: {}, additionalProperties: false };
const READ_ONLY_CHIRHO = { readOnlyHint: true, destructiveHint: false, idempotentHint: true, openWorldHint: false };
const TOOLS_CHIRHO: JsonObjectChirho[] = [
  {
    name: "read_page_chirho",
    title: "Read the page",
    description:
      "Read the owner's saved PRIVATE Perffection page. The JSON answer holds page_chirho, current_version_chirho " +
      "(the version to save against; 0 when nothing is saved yet) and surface_chirho, which names every field, its " +
      "limit, how saving works and every refusal. Read before every save.",
    inputSchema: NO_ARGUMENTS_CHIRHO,
    annotations: READ_ONLY_CHIRHO,
  },
  {
    name: "save_page_chirho",
    title: "Save the page (private)",
    description:
      "Save a COMPLETE page as a new private version. expected_version_chirho is the current_version_chirho you just " +
      "read; page_chirho is the whole page in the shape surface_chirho describes (it replaces the page; it is not a " +
      "patch). It never publishes. Perffection keeps only the most recent versions (16 today): once that history is " +
      "full, every save retires the oldest version, which can then no longer be restored. A refusal names every " +
      "problem. If the result is unknown, read the page again before saving again.",
    inputSchema: {
      type: "object",
      properties: {
        expected_version_chirho: { type: "integer", minimum: 0, description: "current_version_chirho from your last read" },
        page_chirho: { type: "object", description: "the complete page, in the shape surface_chirho describes" },
      },
      required: ["expected_version_chirho", "page_chirho"],
      additionalProperties: false,
    },
    // Destructive: a save appends a version and, once the history is full (16 today), retires the oldest
    // (claude2 #26531, gpt #26537). Idempotent: an exact repeat of an identical save is answered again
    // without a second copy and retires nothing (the API's own description says so; gate arm 4c).
    annotations: { readOnlyHint: false, destructiveHint: true, idempotentHint: true, openWorldHint: false },
  },
  {
    name: "preview_page_chirho",
    title: "Preview the saved page",
    description:
      "Get the saved private page as the HTML a visitor would see once the owner publishes it. Read-only; it publishes nothing.",
    inputSchema: NO_ARGUMENTS_CHIRHO,
    annotations: READ_ONLY_CHIRHO,
  },
  {
    name: "read_figures_chirho",
    title: "Read the business figures",
    description:
      "Read the owner's saved PRIVATE business figures (money in by month and kind of work, with direct costs and jobs " +
      "where given) and the report Perffection made from them. The JSON answer holds state_chirho, current_version_chirho, " +
      "figures_chirho, report_chirho and meanings_chirho, which says how to read every number: money is in whole minor " +
      "units, and null is unknown, never zero. report_chirho.complete_chirho says whether every kind has a figure for " +
      "every month; when it is not true, totals and shares cover only the figures the owner gave. Read-only; read " +
      "before every save_figures_chirho.",
    inputSchema: NO_ARGUMENTS_CHIRHO,
    annotations: READ_ONLY_CHIRHO,
  },
  {
    name: "save_figures_chirho",
    title: "Save the business figures (private)",
    description:
      "Save the owner's COMPLETE business figures as a new private version. expected_version_chirho is the " +
      "current_version_chirho you just read (0 when nothing is saved yet); figures_chirho is the whole figures document " +
      "in exactly the shape read_figures_chirho returns as figures_chirho (it replaces the figures; it is not a patch). " +
      "Money is in whole minor units and null is unknown, never zero. It never publishes. A refusal names its code in " +
      "refusal_chirho, and figures that are not valid name the problem in issues_chirho. If the result is unknown, read " +
      "the figures again before saving again.",
    inputSchema: {
      type: "object",
      properties: {
        expected_version_chirho: { type: "integer", minimum: 0, description: "current_version_chirho from your last read_figures_chirho" },
        figures_chirho: { type: "object", description: "the complete figures document, in the shape read_figures_chirho returns as figures_chirho" },
      },
      required: ["expected_version_chirho", "figures_chirho"],
      additionalProperties: false,
    },
    // Destructive: a save replaces the owner's current figures with a new version. Idempotent: an exact
    // repeat of an identical save is answered again without a second version (the K1 gate measures it).
    annotations: { readOnlyHint: false, destructiveHint: true, idempotentHint: true, openWorldHint: false },
  },
];
const TOOL_NAMES_CHIRHO = new Set(TOOLS_CHIRHO.map(tool_chirho => String(tool_chirho.name)));

function is_object_chirho(value_chirho: unknown): value_chirho is JsonObjectChirho {
  return typeof value_chirho === "object" && value_chirho !== null && !Array.isArray(value_chirho);
}

// ---- configuration: read once at start; a problem is reported by every tool call, never guessed around ----
let key_to_scrub_chirho = "";
function load_config_chirho(env_chirho: NodeJS.ProcessEnv): ConfigChirho | string {
  const key_file_chirho = env_chirho.PERFFECTION_KEY_FILE_CHIRHO;
  let key_chirho = "";
  if (key_file_chirho) {
    try {
      // Like ssh with a private key: a key file other users of this machine can open is refused, not
      // warned about, because an MCP server's stderr is rarely seen (claude2 #25158). Checked before
      // the key is read. Windows has no such permission bits, so there the file is not checked.
      if (process.platform !== "win32" && (statSync(key_file_chirho).mode & 0o077) !== 0) {
        return "the key file is open to other users of this machine: run chmod 600 on it";
      }
      key_chirho = readFileSync(key_file_chirho, "utf8").trim();
    } catch {
      return "the key file named by PERFFECTION_KEY_FILE_CHIRHO could not be read";
    }
  } else {
    key_chirho = (env_chirho.PERFFECTION_KEY_CHIRHO ?? "").trim();
  }
  if (!key_chirho) return "no key: set PERFFECTION_KEY_FILE_CHIRHO to a file holding the key (or PERFFECTION_KEY_CHIRHO to the key)";
  if (key_chirho.length > 4096 || !KEY_PATTERN_CHIRHO.test(key_chirho)) return "the key is not in the form of an access key";
  key_to_scrub_chirho = key_chirho;
  const base_chirho = env_chirho.PERFFECTION_BASE_URL_CHIRHO ?? "";
  let url_chirho: URL;
  try {
    url_chirho = new URL(base_chirho);
  } catch {
    return "PERFFECTION_BASE_URL_CHIRHO must be the workspace's address, for example https://beta-chirho.perffection.com";
  }
  if (url_chirho.protocol !== "https:" && url_chirho.protocol !== "http:") return "PERFFECTION_BASE_URL_CHIRHO must start with https://";
  if (url_chirho.username || url_chirho.password || url_chirho.pathname !== "/" || url_chirho.search || url_chirho.hash) {
    return "PERFFECTION_BASE_URL_CHIRHO must be only the address, like https://beta-chirho.perffection.com, with no path";
  }
  if (url_chirho.protocol === "http:" && !LOOPBACK_HOSTS_CHIRHO.has(url_chirho.hostname)) {
    return "plain http is allowed only to this machine (127.0.0.1 or localhost): the key would travel unencrypted";
  }
  const timeout_chirho = Number(env_chirho.PERFFECTION_TIMEOUT_MS_CHIRHO ?? DEFAULT_TIMEOUT_MS_CHIRHO);
  const timeout_ms_chirho = Number.isInteger(timeout_chirho) && timeout_chirho >= 100 && timeout_chirho <= 120_000 ? timeout_chirho : DEFAULT_TIMEOUT_MS_CHIRHO;
  return { origin_chirho: url_chirho.origin, key_chirho, timeout_ms_chirho };
}

function scrub_chirho(text_chirho: string): string {
  return key_to_scrub_chirho && text_chirho.includes(key_to_scrub_chirho) ? text_chirho.split(key_to_scrub_chirho).join("[redacted-key]") : text_chirho;
}
function log_chirho(text_chirho: string): void {
  process.stderr.write(`perffection-mcp-chirho: ${scrub_chirho(text_chirho)}\n`);
}

// ---- output: only JSON-RPC lines on stdout, one per line, the key scrubbed from every one ----
function write_message_chirho(message_chirho: JsonObjectChirho): void {
  process.stdout.write(`${scrub_chirho(JSON.stringify(message_chirho))}\n`);
}
function reply_result_chirho(id_chirho: RpcIdChirho, result_chirho: JsonObjectChirho): void {
  write_message_chirho({ jsonrpc: "2.0", id: id_chirho, result: result_chirho });
}
function reply_error_chirho(id_chirho: RpcIdChirho | null, code_chirho: number, message_chirho: string, data_chirho?: JsonObjectChirho): void {
  const error_chirho: JsonObjectChirho = { code: code_chirho, message: message_chirho };
  if (data_chirho) error_chirho.data = data_chirho;
  write_message_chirho({ jsonrpc: "2.0", id: id_chirho, error: error_chirho });
}
function wrap_chirho(era_chirho: EraChirho, body_chirho: JsonObjectChirho): JsonObjectChirho {
  return era_chirho === "modern" ? { resultType: "complete", ...body_chirho, _meta: { [META_SERVER_INFO_CHIRHO]: SERVER_INFO_CHIRHO } } : body_chirho;
}
function tool_result_chirho(is_error_chirho: boolean, lead_chirho: string, structured_chirho: JsonObjectChirho): JsonObjectChirho {
  return { content: [{ type: "text", text: `${lead_chirho}\n${JSON.stringify(structured_chirho)}` }], structuredContent: structured_chirho, isError: is_error_chirho };
}

// ---- HTTP: one of the five fixed routes, no redirect followed, a bounded body, a bounded wait ----
async function read_capped_chirho(response_chirho: Response, max_bytes_chirho = ANSWER_MAX_BYTES_CHIRHO): Promise<Uint8Array | "too-large"> {
  const reader_chirho = response_chirho.body?.getReader();
  if (!reader_chirho) return new Uint8Array(0);
  const chunks_chirho: Uint8Array[] = [];
  let total_chirho = 0;
  for (;;) {
    const { done: done_chirho, value: value_chirho } = await reader_chirho.read();
    if (done_chirho) break;
    total_chirho += value_chirho.byteLength;
    if (total_chirho > max_bytes_chirho) {
      await reader_chirho.cancel().catch(() => undefined);
      return "too-large";
    }
    chunks_chirho.push(value_chirho);
  }
  const bytes_chirho = new Uint8Array(total_chirho);
  let offset_chirho = 0;
  for (const chunk_chirho of chunks_chirho) {
    bytes_chirho.set(chunk_chirho, offset_chirho);
    offset_chirho += chunk_chirho.byteLength;
  }
  return bytes_chirho;
}

function status_meaning_chirho(status_chirho: number): string {
  if (status_chirho >= 300 && status_chirho < 400) return "the address redirected instead of answering; check PERFFECTION_BASE_URL_CHIRHO";
  const meanings_chirho: Record<number, string> = {
    400: "the request was stopped before or outside the page logic",
    401: "the key was not accepted (missing, unknown or expired)",
    403: "the key is not allowed to use this route",
    413: "the request was too large for the service",
    // Only the two 429s recognized whole (FIXED_REFUSALS_CHIRHO) say why; any other says neither why nor how long.
    429: "the service refused this request for now as too many requests, in an answer this adapter does not recognize, so why and for how long are unknown",
  };
  return meanings_chirho[status_chirho] ?? (status_chirho >= 500 ? "the service had a problem" : "an unexpected answer");
}

function none_chirho(status_chirho: number | null, meaning_chirho: string): RouteOutcomeChirho {
  return { kind_chirho: "none", status_chirho, meaning_chirho };
}

// On a 404 the health route says only whether something live answers at this address: then the route
// is absent there (a Perffection server without this tool, such as an older one, or another service);
// otherwise the address itself is unconfirmed (wrong, or down). It is asked WITHOUT the key, nothing else
// is sent, and nothing is retried.
const HEALTH_ROUTE_CHIRHO = "/healthz-chirho";
async function not_found_chirho(config_chirho: ConfigChirho): Promise<RouteOutcomeChirho> {
  try {
    const health_chirho = await fetch(new URL(HEALTH_ROUTE_CHIRHO, config_chirho.origin_chirho), {
      method: "GET",
      headers: { "user-agent": `${SERVER_INFO_CHIRHO.name}/${SERVER_INFO_CHIRHO.version}` },
      redirect: "manual",
      signal: AbortSignal.timeout(config_chirho.timeout_ms_chirho),
    });
    await health_chirho.body?.cancel().catch(() => undefined);
    if (health_chirho.status === 200) {
      return { kind_chirho: "none", status_chirho: 404, reason_chirho: "tool-route-absent-chirho",
        meaning_chirho: "the address answers its health check but has no route for this tool: it may be a Perffection server without this tool (one older than this adapter), or a service that is not Perffection" };
    }
  } catch { /* no health answer either: the address itself is in question */ }
  return { kind_chirho: "none", status_chirho: 404, reason_chirho: "address-unconfirmed-chirho",
    meaning_chirho: "no route for this tool, and the address's health check did not pass: the address may be wrong, or the service may be down; check PERFFECTION_BASE_URL_CHIRHO, then try again later" };
}

// Whether a refused answer is one of the fixed refusals (FIXED_REFUSALS_CHIRHO), whole: its status, its
// exact media type, every byte of its body read to its end within a small bound, then its Retry-After
// rule. A cut-off, longer, shorter or otherwise different answer is not one, and stays unknown. The body
// is compared as raw bytes, never decoded first: a decoder drops a leading byte-order mark (EF BB BF), so
// a decoded comparison would match a body three bytes longer than the engine's (gpt #28596).
async function fixed_refusal_chirho(response_chirho: Response): Promise<RefusedOutcomeChirho | undefined> {
  const candidates_chirho = FIXED_REFUSALS_CHIRHO.filter(refusal_chirho => refusal_chirho.status_chirho === response_chirho.status);
  const media_chirho = (response_chirho.headers.get("content-type") ?? "").trim().toLowerCase();
  if (candidates_chirho.length === 0 || media_chirho !== "application/json") {
    await response_chirho.body?.cancel().catch(() => undefined);
    return undefined;
  }
  let bytes_chirho: Uint8Array | "too-large";
  try {
    bytes_chirho = await read_capped_chirho(response_chirho, FIXED_REFUSAL_MAX_BYTES_CHIRHO);
  } catch {
    return undefined;
  }
  if (bytes_chirho === "too-large") return undefined;
  const received_chirho = bytes_chirho;
  const refusal_chirho = candidates_chirho.find(candidate_chirho => same_bytes_chirho(received_chirho, candidate_chirho.body_chirho));
  if (!refusal_chirho) return undefined;
  const refused_chirho: RefusedOutcomeChirho = {
    kind_chirho: "refused", status_chirho: refusal_chirho.status_chirho, identity_chirho: refusal_chirho.identity_chirho,
    meaning_chirho: refusal_chirho.meaning_chirho, may_count_use_chirho: refusal_chirho.may_count_use_chirho,
    ...(refusal_chirho.reason_chirho === undefined ? {} : { reason_chirho: refusal_chirho.reason_chirho }),
  };
  const retry_after_chirho = response_chirho.headers.get("retry-after");
  if (refusal_chirho.retry_after_chirho === "unread") return refused_chirho;
  if (refusal_chirho.retry_after_chirho === "none") return retry_after_chirho === null ? refused_chirho : undefined;
  const seconds_chirho = retry_after_seconds_chirho(retry_after_chirho);
  if (seconds_chirho === undefined) return undefined;
  return { ...refused_chirho, meaning_chirho: `${refused_chirho.meaning_chirho} ${seconds_chirho} seconds before trying again`, retry_after_seconds_chirho: seconds_chirho };
}

// The caller key a geo-elected Perffection needs on every write, exactly once
// (CALLER_IDEMPOTENCY_KEY_HEADER_CHIRHO in Cairn's write relay: 1 to 128 visible ASCII bytes). A read may carry one
// too, where the engine treats it as transport correlation only. Each request gets its own fresh random UUID, and a key
// is never reused, because nothing here retries. A save sent again by her agent is a new request against its expected
// version, so it can't run twice unseen. The key names one request and is not a credential; it is never logged.
// Workflow: spec-chirho/workflows-chirho/platform-route-architecture-chirho/browser-caller-key-carrier-chirho.md
// (browser forms carry the same key as a hidden field).
const CALLER_KEY_HEADER_CHIRHO = "cairn-idempotency-key-chirho";

async function call_route_chirho(config_chirho: ConfigChirho, route_chirho: string, expect_chirho: "json" | "html", body_chirho?: string): Promise<RouteOutcomeChirho> {
  const wanted_chirho = expect_chirho === "json" ? "application/json" : "text/html";
  const headers_chirho: Record<string, string> = {
    authorization: `Bearer ${config_chirho.key_chirho}`,
    accept: wanted_chirho,
    "user-agent": `${SERVER_INFO_CHIRHO.name}/${SERVER_INFO_CHIRHO.version}`,
    [CALLER_KEY_HEADER_CHIRHO]: randomUUID(),
  };
  if (body_chirho !== undefined) headers_chirho["content-type"] = "application/json";
  let response_chirho: Response;
  try {
    response_chirho = await fetch(new URL(route_chirho, config_chirho.origin_chirho), {
      method: body_chirho === undefined ? "GET" : "POST",
      headers: headers_chirho,
      body: body_chirho,
      redirect: "manual",
      signal: AbortSignal.timeout(config_chirho.timeout_ms_chirho),
    });
  } catch (error_chirho) {
    if (error_chirho instanceof Error && error_chirho.name === "TimeoutError") return none_chirho(null, `no answer within ${config_chirho.timeout_ms_chirho} ms`);
    return none_chirho(null, "the address could not be reached, or the connection dropped before an answer");
  }
  if (response_chirho.status !== 200) {
    // A fixed refusal is recognized for every tool, so its meaning names why; only a save turns it into
    // "refused before its DAG ran" (a read's is still no answer). fixed_refusal_chirho reads or cancels the body.
    if (response_chirho.status !== 404) {
      const refused_chirho = await fixed_refusal_chirho(response_chirho);
      if (refused_chirho) return refused_chirho;
    } else {
      await response_chirho.body?.cancel().catch(() => undefined);
    }
    if (response_chirho.status === 404) return not_found_chirho(config_chirho);
    return none_chirho(response_chirho.status, status_meaning_chirho(response_chirho.status));
  }
  let bytes_chirho: Uint8Array | "too-large";
  try {
    bytes_chirho = await read_capped_chirho(response_chirho);
  } catch {
    return none_chirho(200, "the answer was cut off before it ended");
  }
  if (bytes_chirho === "too-large") return none_chirho(200, `the answer was larger than ${ANSWER_MAX_BYTES_CHIRHO} bytes`);
  const media_chirho = (response_chirho.headers.get("content-type") ?? "").split(";")[0].trim().toLowerCase();
  if (media_chirho !== wanted_chirho) return none_chirho(200, `the answer was ${media_chirho || "untyped"}, not ${wanted_chirho}`);
  const text_chirho = new TextDecoder("utf-8").decode(bytes_chirho);
  if (expect_chirho === "html") return { kind_chirho: "answer", body_chirho: text_chirho };
  try {
    const parsed_chirho: unknown = JSON.parse(text_chirho);
    if (is_object_chirho(parsed_chirho)) return { kind_chirho: "answer", body_chirho: parsed_chirho };
  } catch {
    // falls through: not JSON
  }
  return none_chirho(200, "the answer was not readable JSON");
}

// ---- the five tools (workflows: agent-api-chirho.md, "her own agent through MCP"; her-figures-chirho.md) ----
// What each save reads again before it saves again.
const SAVE_SUBJECT_CHIRHO: Record<string, string> = { save_page_chirho: "the page", save_figures_chirho: "the figures" };
// Each save's own answer kind: its schema, the documents a saved answer carries, its invalid-document
// refusal, and the refusals it declares with no version, no document and no issue (unverifiable means
// unknown). Only the figures save refuses its own illustrative sample (F6d).
type SaveKindChirho = { schema_chirho: string; documents_chirho: string[]; invalid_chirho: string; plain_chirho: ReadonlySet<unknown> };
const PLAIN_REFUSALS_CHIRHO = ["oversize-chirho", "unreadable-chirho", "unsupported-schema-chirho", "unverifiable-chirho"];
const SAVE_KINDS_CHIRHO: Record<string, SaveKindChirho> = {
  save_page_chirho: { schema_chirho: PAGE_SAVE_SCHEMA_CHIRHO, documents_chirho: ["page_chirho"], invalid_chirho: "invalid-page-chirho", plain_chirho: new Set(PLAIN_REFUSALS_CHIRHO) },
  save_figures_chirho: {
    schema_chirho: FIGURES_SAVE_SCHEMA_CHIRHO, documents_chirho: ["figures_chirho", "report_chirho"], invalid_chirho: "invalid-figures-chirho",
    plain_chirho: new Set([...PLAIN_REFUSALS_CHIRHO, "sample-not-her-figures-chirho"]),
  },
};

function no_answer_chirho(
  tool_chirho: string,
  outcome_chirho: { status_chirho: number | null; meaning_chirho: string; reason_chirho?: NoAnswerReasonChirho; retry_after_seconds_chirho?: number },
): JsonObjectChirho {
  const said_chirho = outcome_chirho.status_chirho === null ? outcome_chirho.meaning_chirho : `HTTP ${outcome_chirho.status_chirho}: ${outcome_chirho.meaning_chirho}`;
  const structured_chirho: JsonObjectChirho = {
    adapter_outcome_chirho: "no-answer-chirho",
    http_status_chirho: outcome_chirho.status_chirho,
    meaning_chirho: outcome_chirho.meaning_chirho,
  };
  if (outcome_chirho.reason_chirho) structured_chirho.reason_chirho = outcome_chirho.reason_chirho;
  if (outcome_chirho.retry_after_seconds_chirho !== undefined) structured_chirho.retry_after_seconds_chirho = outcome_chirho.retry_after_seconds_chirho;
  if (!SAVE_TOOLS_CHIRHO.has(tool_chirho)) return tool_result_chirho(true, `No answer from Perffection (${said_chirho}).`, structured_chirho);
  // The API's own rule: any status but a complete 200 answer carries no answer; read before assuming.
  structured_chirho.save_result_chirho = "unknown-chirho";
  return tool_result_chirho(true, `No answer from Perffection (${said_chirho}). Whether this save stored anything is unknown: read ${SAVE_SUBJECT_CHIRHO[tool_chirho]} again before saving again.`, structured_chirho);
}

// A save Perffection refused before running its DAG, by one of the fixed answers (FIXED_REFUSALS_CHIRHO):
// K1's, or one of the two 429s. Its own result, neither unknown nor an unqualified "not stored": THIS
// request's DAG did not run, so it changed nothing through that DAG; where the engine's path allows, the
// request may already count as a use of the key; an earlier save whose answer never came is still unknown.
// Nothing here invites a retry (gpt #26636, #26662): a busy answer passes on only the engine's own wait,
// and the adapter never retries.
function refused_before_run_chirho(tool_chirho: string, outcome_chirho: RefusalChirho): JsonObjectChirho {
  const structured_chirho: JsonObjectChirho = {
    adapter_outcome_chirho: "refused-before-run-chirho",
    save_result_chirho: "refused-before-run-chirho",
    http_status_chirho: outcome_chirho.status_chirho,
    refusal_identity_chirho: outcome_chirho.identity_chirho,
    meaning_chirho: outcome_chirho.meaning_chirho,
  };
  if (outcome_chirho.retry_after_seconds_chirho !== undefined) structured_chirho.retry_after_seconds_chirho = outcome_chirho.retry_after_seconds_chirho;
  return tool_result_chirho(
    true,
    `Refused before its DAG ran (HTTP ${outcome_chirho.status_chirho}, ${outcome_chirho.identity_chirho}): ${outcome_chirho.meaning_chirho}. ` +
      `Perffection did not run this tool's DAG for THIS request, so this request did not change ${SAVE_SUBJECT_CHIRHO[tool_chirho]} through it` +
      (outcome_chirho.may_count_use_chirho ? "; the request may already be counted as a use of the key. " : ". ") +
      `Whether an earlier save whose answer never came stored anything is still unknown: read ${SAVE_SUBJECT_CHIRHO[tool_chirho]} again before saving again.`,
    structured_chirho,
  );
}

// A save's answer is its own kind, with a result its own codes declare (gpt #26637, #26662): its exact
// schema, then exactly one of
// - saved: no refusal, a positive version, its documents, no issue;
// - stale-version-chirho: a positive head version only (no document: read again for the head);
// - its invalid-document refusal: no version, no document, at least one issue;
// - oversize, unreadable, unsupported-schema, unverifiable (and for figures, the sample refused): no
//   version, no document, no issue.
// Anything else, a refusal code it does not declare included, is no answer, and the save stays unknown.
// The answer-kind law only: the workspace's one validator decides what a document may hold.
function is_save_answer_chirho(answer_chirho: JsonObjectChirho, kind_chirho: SaveKindChirho): boolean {
  const version_chirho = answer_chirho.current_version_chirho;
  const issues_chirho = answer_chirho.issues_chirho;
  if (answer_chirho.schema_chirho !== kind_chirho.schema_chirho || typeof answer_chirho.saved_chirho !== "boolean" || !Array.isArray(issues_chirho)) return false;
  const positive_chirho = typeof version_chirho === "number" && Number.isSafeInteger(version_chirho) && version_chirho >= 1;
  const documents_chirho = kind_chirho.documents_chirho.map(field_chirho => answer_chirho[field_chirho]);
  if (answer_chirho.saved_chirho) {
    return answer_chirho.refusal_chirho === null && positive_chirho && documents_chirho.every(is_object_chirho) && issues_chirho.length === 0;
  }
  if (!documents_chirho.every(document_chirho => document_chirho === null)) return false;
  const refusal_chirho = answer_chirho.refusal_chirho;
  if (refusal_chirho === "stale-version-chirho") return positive_chirho && issues_chirho.length === 0;
  if (refusal_chirho === kind_chirho.invalid_chirho) return version_chirho === null && issues_chirho.length > 0;
  return kind_chirho.plain_chirho.has(refusal_chirho) && version_chirho === null && issues_chirho.length === 0;
}
// A read's answer: its exact schema and one of its three states.
function is_read_answer_chirho(answer_chirho: JsonObjectChirho, schema_chirho: string): boolean {
  return answer_chirho.schema_chirho === schema_chirho && READ_STATES_CHIRHO.has(answer_chirho.state_chirho);
}

async function read_page_chirho(config_chirho: ConfigChirho): Promise<JsonObjectChirho> {
  const outcome_chirho = await call_route_chirho(config_chirho, READ_ROUTE_CHIRHO, "json");
  if (outcome_chirho.kind_chirho !== "answer") return no_answer_chirho("read_page_chirho", outcome_chirho);
  const answer_chirho = outcome_chirho.body_chirho as JsonObjectChirho;
  if (!is_read_answer_chirho(answer_chirho, PAGE_READ_SCHEMA_CHIRHO)) {
    return no_answer_chirho("read_page_chirho", { status_chirho: 200, meaning_chirho: "the answer was not a Perffection page; check PERFFECTION_BASE_URL_CHIRHO" });
  }
  const version_chirho = answer_chirho.current_version_chirho;
  const lead_chirho =
    answer_chirho.state_chirho === "saved-chirho" ? `The saved private page, version ${version_chirho}. Save against version ${version_chirho}.`
    : answer_chirho.state_chirho === "nothing-saved-chirho" ? "Nothing is saved yet. Save against version 0; surface_chirho.sample_page_chirho shows the exact shape."
    : "The page could not be verified just now, so there is no version to save against: do not save; read again later.";
  return tool_result_chirho(false, lead_chirho, answer_chirho);
}

async function save_page_chirho(config_chirho: ConfigChirho, arguments_chirho: JsonObjectChirho): Promise<JsonObjectChirho> {
  // The arguments ARE the API's save body: the workspace's one validator decides, nothing is repaired here.
  const outcome_chirho = await call_route_chirho(config_chirho, SAVE_ROUTE_CHIRHO, "json", JSON.stringify(arguments_chirho));
  if (outcome_chirho.kind_chirho === "refused") return refused_before_run_chirho("save_page_chirho", outcome_chirho);
  if (outcome_chirho.kind_chirho === "none") return no_answer_chirho("save_page_chirho", outcome_chirho);
  const answer_chirho = outcome_chirho.body_chirho as JsonObjectChirho;
  if (!is_save_answer_chirho(answer_chirho, SAVE_KINDS_CHIRHO.save_page_chirho)) {
    return no_answer_chirho("save_page_chirho", { status_chirho: 200, meaning_chirho: "the answer was not a Perffection page save answer" });
  }
  if (answer_chirho.saved_chirho) {
    return tool_result_chirho(false, `Saved as private version ${answer_chirho.current_version_chirho}. Nothing was published: the owner previews and publishes from their workspace.`, answer_chirho);
  }
  const refusal_chirho = answer_chirho.refusal_chirho;
  const lead_chirho =
    refusal_chirho === "stale-version-chirho" ? `Not saved: version ${answer_chirho.current_version_chirho} was saved first. Read the page again, bring your change onto it, and save against that version.`
    : refusal_chirho === "unverifiable-chirho" ? "Unknown: this save's result could not be verified. Read the page again before saving again."
    : refusal_chirho === "invalid-page-chirho" ? "Not saved: the page has problems, each named in issues_chirho. Nothing was stored."
    : `Not saved (${String(refusal_chirho)}). surface_chirho.save_chirho.refusals_chirho says what it means; when in doubt, read the page again.`;
  return tool_result_chirho(true, lead_chirho, answer_chirho);
}

// Her figures. The lead says whether the report is complete only when the answer says so in as many
// words: anything short of complete_chirho === true is told as incomplete, so totals of the figures she
// gave are never handed over as whole-business totals (gpt #26575).
// Whether a nothing-saved figures read carries its first-save example (F6d). An older server's answer has
// none, and the lead must not point at a field the answer lacks (gpt #26678).
function has_first_save_example_chirho(answer_chirho: JsonObjectChirho): boolean {
  const start_chirho = answer_chirho.start_chirho;
  return is_object_chirho(start_chirho) && is_object_chirho(start_chirho.sample_figures_chirho);
}

async function read_figures_chirho(config_chirho: ConfigChirho): Promise<JsonObjectChirho> {
  const outcome_chirho = await call_route_chirho(config_chirho, FIGURES_ROUTE_CHIRHO, "json");
  if (outcome_chirho.kind_chirho !== "answer") return no_answer_chirho("read_figures_chirho", outcome_chirho);
  const answer_chirho = outcome_chirho.body_chirho as JsonObjectChirho;
  if (!is_read_answer_chirho(answer_chirho, FIGURES_READ_SCHEMA_CHIRHO)) {
    return no_answer_chirho("read_figures_chirho", { status_chirho: 200, meaning_chirho: "the answer was not Perffection figures; check PERFFECTION_BASE_URL_CHIRHO" });
  }
  const complete_chirho = is_object_chirho(answer_chirho.report_chirho) && answer_chirho.report_chirho.complete_chirho === true;
  const lead_chirho =
    answer_chirho.state_chirho === "saved-chirho"
      ? `The owner's saved figures, version ${answer_chirho.current_version_chirho}, as of ${answer_chirho.as_of_month_chirho}. ` +
        (complete_chirho
          ? "Complete: report_chirho.complete_chirho is true, so every kind of work has a figure for every month."
          : "INCOMPLETE: report_chirho.complete_chirho is not true, so totals and shares cover only the figures the owner gave and no trend is drawn; never present them as whole-business totals.") +
        ` meanings_chirho says how to read each number. Save changes against version ${answer_chirho.current_version_chirho}.`
    : answer_chirho.state_chirho === "nothing-saved-chirho"
      ? (has_first_save_example_chirho(answer_chirho)
          ? "No figures are saved yet. start_chirho.sample_figures_chirho shows the exact shape with illustrative values that are NOT the owner's: " +
            "ask the owner for their real figures, put them in that shape, and save them with save_figures_chirho against version 0. Never save the sample as theirs."
          : "No figures are saved yet, and this version of Perffection supplied no first-save example, so there is no shape to copy: do not guess one. " +
            "The owner can paste their figures in their workspace.")
    : "The figures could not be verified just now, so none are shown and there is no version to save against: do not save; read again later.";
  return tool_result_chirho(false, lead_chirho, answer_chirho);
}

async function save_figures_chirho(config_chirho: ConfigChirho, arguments_chirho: JsonObjectChirho): Promise<JsonObjectChirho> {
  // The arguments ARE the API's save body: the workspace's one figures validator decides, nothing is repaired here.
  const outcome_chirho = await call_route_chirho(config_chirho, SAVE_FIGURES_ROUTE_CHIRHO, "json", JSON.stringify(arguments_chirho));
  if (outcome_chirho.kind_chirho === "refused") return refused_before_run_chirho("save_figures_chirho", outcome_chirho);
  if (outcome_chirho.kind_chirho === "none") return no_answer_chirho("save_figures_chirho", outcome_chirho);
  const answer_chirho = outcome_chirho.body_chirho as JsonObjectChirho;
  if (!is_save_answer_chirho(answer_chirho, SAVE_KINDS_CHIRHO.save_figures_chirho)) {
    return no_answer_chirho("save_figures_chirho", { status_chirho: 200, meaning_chirho: "the answer was not a Perffection figures save answer" });
  }
  if (answer_chirho.saved_chirho) {
    return tool_result_chirho(false, `Saved as private figures version ${answer_chirho.current_version_chirho}: figures_chirho and report_chirho are exactly what was kept. Nothing was published.`, answer_chirho);
  }
  const refusal_chirho = answer_chirho.refusal_chirho;
  const lead_chirho =
    refusal_chirho === "stale-version-chirho" ? `Not saved: figures version ${answer_chirho.current_version_chirho} was saved first. Read the figures again, bring your change onto them, and save against that version.`
    : refusal_chirho === "unverifiable-chirho" ? "Unknown: this save's result could not be verified. Read the figures again before saving again."
    : refusal_chirho === "invalid-figures-chirho" ? "Not saved: the figures are not valid; issues_chirho names the problem. This request stored nothing."
    : refusal_chirho === "sample-not-her-figures-chirho" ? "Not saved: that is the illustrative sample, not the owner's figures. Ask the owner for their real figures and save those. This request stored nothing."
    : `Not saved (${String(refusal_chirho)}): send exactly expected_version_chirho and the whole figures_chirho document read_figures_chirho returned, nothing else. This request stored nothing.`;
  return tool_result_chirho(true, lead_chirho, answer_chirho);
}

async function preview_page_chirho(config_chirho: ConfigChirho): Promise<JsonObjectChirho> {
  const outcome_chirho = await call_route_chirho(config_chirho, PREVIEW_ROUTE_CHIRHO, "html");
  if (outcome_chirho.kind_chirho !== "answer") return no_answer_chirho("preview_page_chirho", outcome_chirho);
  return { content: [{ type: "text", text: String(outcome_chirho.body_chirho) }], isError: false };
}

async function run_tool_chirho(name_chirho: string, arguments_chirho: JsonObjectChirho): Promise<JsonObjectChirho> {
  if (typeof config_chirho === "string") {
    const structured_chirho: JsonObjectChirho = { adapter_outcome_chirho: "not-configured-chirho", meaning_chirho: config_chirho };
    if (SAVE_TOOLS_CHIRHO.has(name_chirho)) structured_chirho.save_result_chirho = "not-sent-chirho";
    return tool_result_chirho(true, `Not configured: ${config_chirho}. Nothing was sent.`, structured_chirho);
  }
  if (name_chirho === "save_page_chirho") return save_page_chirho(config_chirho, arguments_chirho);
  if (name_chirho === "save_figures_chirho") return save_figures_chirho(config_chirho, arguments_chirho);
  if (Object.keys(arguments_chirho).length > 0) {
    return tool_result_chirho(true, `${name_chirho} takes no arguments. Nothing was sent.`, { adapter_outcome_chirho: "bad-arguments-chirho" });
  }
  if (name_chirho === "read_page_chirho") return read_page_chirho(config_chirho);
  if (name_chirho === "read_figures_chirho") return read_figures_chirho(config_chirho);
  return preview_page_chirho(config_chirho);
}

// ---- JSON-RPC: requests answered, notifications never, tool calls one at a time in arrival order ----
let legacy_version_chirho: string | undefined;
let queued_calls_chirho = 0;
let call_chain_chirho: Promise<void> = Promise.resolve();

function enqueue_call_chirho(id_chirho: RpcIdChirho, params_chirho: JsonObjectChirho, era_chirho: EraChirho): void {
  const name_chirho = params_chirho.name;
  if (typeof name_chirho !== "string" || !TOOL_NAMES_CHIRHO.has(name_chirho)) {
    reply_error_chirho(id_chirho, -32602, `Unknown tool: ${String(name_chirho).slice(0, 128)}`);
    return;
  }
  const arguments_chirho = params_chirho.arguments ?? {};
  if (!is_object_chirho(arguments_chirho)) {
    reply_error_chirho(id_chirho, -32602, "Invalid params: arguments must be an object");
    return;
  }
  if (queued_calls_chirho >= MAX_QUEUED_CALLS_CHIRHO) {
    reply_error_chirho(id_chirho, -32000, "Too many tool calls waiting; wait for earlier answers");
    return;
  }
  queued_calls_chirho += 1;
  call_chain_chirho = call_chain_chirho.then(async () => {
    try {
      reply_result_chirho(id_chirho, wrap_chirho(era_chirho, await run_tool_chirho(name_chirho, arguments_chirho)));
    } catch {
      reply_result_chirho(id_chirho, wrap_chirho(era_chirho, tool_result_chirho(true, "The adapter failed unexpectedly; read again before saving again.", { adapter_outcome_chirho: "adapter-failure-chirho" })));
    } finally {
      queued_calls_chirho -= 1;
    }
  });
}

function route_request_chirho(id_chirho: RpcIdChirho, method_chirho: string, params_chirho: JsonObjectChirho, era_chirho: EraChirho): void {
  if (method_chirho === "server/discover") {
    reply_result_chirho(id_chirho, wrap_chirho("modern", { supportedVersions: ALL_VERSIONS_CHIRHO, capabilities: CAPABILITIES_CHIRHO, instructions: INSTRUCTIONS_CHIRHO }));
  } else if (method_chirho === "ping") {
    reply_result_chirho(id_chirho, wrap_chirho(era_chirho, {}));
  } else if (method_chirho === "tools/list") {
    reply_result_chirho(id_chirho, wrap_chirho(era_chirho, { tools: TOOLS_CHIRHO }));
  } else if (method_chirho === "tools/call") {
    enqueue_call_chirho(id_chirho, params_chirho, era_chirho);
  } else {
    reply_error_chirho(id_chirho, -32601, `Method not found: ${method_chirho.slice(0, 128)}`);
  }
}

function handle_request_chirho(id_chirho: RpcIdChirho, method_chirho: string, params_chirho: JsonObjectChirho): void {
  const meta_chirho = is_object_chirho(params_chirho._meta) ? params_chirho._meta : undefined;
  const requested_chirho = meta_chirho?.[META_VERSION_CHIRHO];
  if (meta_chirho && requested_chirho !== undefined) {
    // Modern: every request says its version and capabilities; nothing depends on an earlier message.
    if (typeof requested_chirho !== "string" || !MODERN_VERSIONS_CHIRHO.includes(requested_chirho)) {
      reply_error_chirho(id_chirho, -32022, "Unsupported protocol version", { supported: ALL_VERSIONS_CHIRHO, requested: typeof requested_chirho === "string" ? requested_chirho : null });
    } else if (!is_object_chirho(meta_chirho[META_CAPABILITIES_CHIRHO])) {
      reply_error_chirho(id_chirho, -32602, `Invalid params: params._meta must carry ${META_CAPABILITIES_CHIRHO}`);
    } else {
      route_request_chirho(id_chirho, method_chirho, params_chirho, "modern");
    }
  } else if (method_chirho === "initialize") {
    // Legacy: echo a version this adapter speaks, else offer its newest legacy one.
    const asked_chirho = params_chirho.protocolVersion;
    legacy_version_chirho = typeof asked_chirho === "string" && LEGACY_VERSIONS_CHIRHO.includes(asked_chirho) ? asked_chirho : LEGACY_VERSIONS_CHIRHO[0];
    reply_result_chirho(id_chirho, { protocolVersion: legacy_version_chirho, capabilities: CAPABILITIES_CHIRHO, serverInfo: SERVER_INFO_CHIRHO, instructions: INSTRUCTIONS_CHIRHO });
  } else if (method_chirho === "server/discover") {
    route_request_chirho(id_chirho, method_chirho, params_chirho, "modern");
  } else if (legacy_version_chirho !== undefined || method_chirho === "ping") {
    route_request_chirho(id_chirho, method_chirho, params_chirho, "legacy");
  } else {
    reply_error_chirho(id_chirho, -32602, `Invalid params: carry ${META_VERSION_CHIRHO} and ${META_CAPABILITIES_CHIRHO} in params._meta, or send initialize first`);
  }
}

function accept_line_chirho(line_chirho: string): void {
  if (line_chirho.trim() === "") return;
  let message_chirho: unknown;
  try {
    message_chirho = JSON.parse(line_chirho);
  } catch {
    reply_error_chirho(null, -32700, "Parse error: each line must be one JSON-RPC message");
    return;
  }
  if (Array.isArray(message_chirho)) {
    reply_error_chirho(null, -32600, "Invalid Request: batches are not supported");
    return;
  }
  const id_chirho = is_object_chirho(message_chirho) ? message_chirho.id : undefined;
  const valid_id_chirho = typeof id_chirho === "string" || typeof id_chirho === "number" ? id_chirho : null;
  if (!is_object_chirho(message_chirho) || message_chirho.jsonrpc !== "2.0") {
    reply_error_chirho(valid_id_chirho, -32600, "Invalid Request: not a JSON-RPC 2.0 message");
    return;
  }
  const method_chirho = message_chirho.method;
  if (typeof method_chirho !== "string" || !("id" in message_chirho)) return; // a notification or a reply: never answered
  if (valid_id_chirho === null) {
    reply_error_chirho(null, -32600, "Invalid Request: id must be a string or a number");
    return;
  }
  const params_chirho = message_chirho.params ?? {};
  if (!is_object_chirho(params_chirho)) {
    reply_error_chirho(valid_id_chirho, -32602, "Invalid params: params must be an object");
    return;
  }
  handle_request_chirho(valid_id_chirho, method_chirho, params_chirho);
}

// ---- stdio: newline-delimited, a line past the bound is dropped whole and answered once ----
const config_chirho = load_config_chirho(process.env);
log_chirho(typeof config_chirho === "string" ? `not configured: ${config_chirho}. Tool calls will say so; nothing is sent.` : `ready for ${config_chirho.origin_chirho}`);
process.stdout.on("error", () => process.exit(0)); // the client went away
const decoder_chirho = new TextDecoder("utf-8");
let pending_chirho = "";
let discarding_chirho = false;
function take_text_chirho(text_chirho: string): void {
  let rest_chirho = text_chirho;
  while (rest_chirho.length > 0) {
    const newline_chirho = rest_chirho.indexOf("\n");
    if (newline_chirho === -1) {
      if (!discarding_chirho) pending_chirho += rest_chirho;
      if (pending_chirho.length > LINE_MAX_CHARS_CHIRHO) {
        discarding_chirho = true;
        pending_chirho = "";
      }
      return;
    }
    const line_chirho = pending_chirho + rest_chirho.slice(0, newline_chirho);
    rest_chirho = rest_chirho.slice(newline_chirho + 1);
    pending_chirho = "";
    if (discarding_chirho || line_chirho.length > LINE_MAX_CHARS_CHIRHO) {
      discarding_chirho = false;
      reply_error_chirho(null, -32600, `Invalid Request: a message may be at most ${LINE_MAX_CHARS_CHIRHO} characters`);
      continue;
    }
    accept_line_chirho(line_chirho.endsWith("\r") ? line_chirho.slice(0, -1) : line_chirho);
  }
}
process.stdin.on("data", (chunk_chirho: Buffer) => take_text_chirho(decoder_chirho.decode(chunk_chirho, { stream: true })));
process.stdin.on("end", () => {
  take_text_chirho(decoder_chirho.decode());
  if (pending_chirho && !discarding_chirho) accept_line_chirho(pending_chirho);
  pending_chirho = "";
});
