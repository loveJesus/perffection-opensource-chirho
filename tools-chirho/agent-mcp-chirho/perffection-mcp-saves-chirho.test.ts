// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// Checks for the adapter's saves (perffection-mcp-chirho.ts): her figures saved by her own agent
// (save_figures_chirho, F6c); every JSON answer recognized only as its own kind, so a save answer never
// reads as figures and a read answer never as a save, and a save's result is only one its codes declare
// (gpt #26637, #26662); and K1's fixed refusals, whole, told as THIS request refused before its DAG ran,
// while every near miss stays unknown (gpt #26636). Behavior
// only, never prose. A fake workspace answers each route with exactly the status, type and bytes a test
// sets, and records every request.
// Run: bun test tools-chirho/agent-mcp-chirho/
import { afterAll, beforeAll, describe, expect, test } from "bun:test";
import { createServer, type AddressInfo, type Server, type Socket } from "node:net";
import { McpStdioClientChirho, call_tool_chirho, legacy_session_chirho, type JsonObjectChirho } from "./mcp-stdio-client-chirho";

const WORKSPACE_CHIRHO = "/app-chirho/perffection-workspace";
const ROUTES_CHIRHO: Record<string, string> = {
  read_page_chirho: `${WORKSPACE_CHIRHO}/agent-page-chirho`,
  save_page_chirho: `${WORKSPACE_CHIRHO}/agent-save-page-chirho`,
  read_figures_chirho: `${WORKSPACE_CHIRHO}/agent-figures-chirho`,
  save_figures_chirho: `${WORKSPACE_CHIRHO}/agent-save-figures-chirho`,
};
const KEY_CHIRHO = Array.from(crypto.getRandomValues(new Uint8Array(32)), byte_chirho => byte_chirho.toString(16).padStart(2, "0")).join("");

// One answer of each kind, as the workspace gives them (the fields each kind is recognized by).
const FIGURES_DOCUMENT_CHIRHO = {
  schema_chirho: "perffection-figures-v1-chirho", currency_chirho: "USD", note_chirho: "", months_chirho: ["2026-01"],
  kinds_chirho: ["Bread"], money_in_chirho: [[120050]], costs_chirho: [[null]], jobs_chirho: [[null]],
};
const ANSWERS_CHIRHO: Record<string, JsonObjectChirho> = {
  read_page_chirho: { schema_chirho: "perffection-agent-page-v1-chirho", state_chirho: "saved-chirho", current_version_chirho: 2, page_chirho: { business_name_chirho: "Fixture Bakery" }, surface_chirho: {} },
  save_page_chirho: { schema_chirho: "perffection-agent-save-v1-chirho", saved_chirho: true, refusal_chirho: null, current_version_chirho: 3, page_chirho: { business_name_chirho: "Fixture Bakery" }, issues_chirho: [] },
  read_figures_chirho: { schema_chirho: "perffection-agent-figures-v1-chirho", state_chirho: "saved-chirho", current_version_chirho: 4, as_of_month_chirho: "2026-01", report_chirho: { complete_chirho: true }, figures_chirho: FIGURES_DOCUMENT_CHIRHO, meanings_chirho: {} },
  save_figures_chirho: { schema_chirho: "perffection-agent-figures-save-v1-chirho", saved_chirho: true, refusal_chirho: null, current_version_chirho: 5, figures_chirho: FIGURES_DOCUMENT_CHIRHO, report_chirho: { complete_chirho: true }, issues_chirho: [] },
};
const ARGUMENTS_CHIRHO: Record<string, JsonObjectChirho | undefined> = {
  read_page_chirho: undefined,
  save_page_chirho: { expected_version_chirho: 2, page_chirho: { business_name_chirho: "Fixture Bakery" } },
  read_figures_chirho: undefined,
  save_figures_chirho: { expected_version_chirho: 4, figures_chirho: FIGURES_DOCUMENT_CHIRHO },
};
const SAVES_CHIRHO = ["save_page_chirho", "save_figures_chirho"];

// K1's fixed refusals exactly as Cairn answers a request asking for JSON (measured in PFE's K1 gate).
const k1_body_chirho = (identity_chirho: string) => `{"error_chirho":"${identity_chirho}","ok_chirho":false}`;
const K1_REFUSALS_CHIRHO: [number, string][] = [
  [403, "key-route-not-open-chirho"], [403, "key-action-not-granted-chirho"], [403, "key-action-authority-changed-chirho"], [409, "key-route-changed-chirho"],
];

type ReplyChirho = { status_chirho: number; type_chirho: string; body_chirho: string };
const json_reply_chirho = (value_chirho: unknown, status_chirho = 200): ReplyChirho => ({ status_chirho, type_chirho: "application/json; charset=utf-8", body_chirho: JSON.stringify(value_chirho) });
const replies_chirho = new Map<string, ReplyChirho>();
type RecordedChirho = { method_chirho: string; path_chirho: string; authorization_chirho: string; content_type_chirho: string; accept_chirho: string; caller_key_chirho: string; body_chirho: string };
const recorded_chirho: RecordedChirho[] = [];
const spawned_chirho: McpStdioClientChirho[] = [];
const held_sockets_chirho: Socket[] = [];
let fake_chirho: ReturnType<typeof Bun.serve>;
let cut_server_chirho: Server;

beforeAll(async () => {
  fake_chirho = Bun.serve({
    hostname: "127.0.0.1", port: 0,
    async fetch(request_chirho) {
      const path_chirho = new URL(request_chirho.url).pathname;
      recorded_chirho.push({
        method_chirho: request_chirho.method, path_chirho, authorization_chirho: request_chirho.headers.get("authorization") ?? "",
        content_type_chirho: request_chirho.headers.get("content-type") ?? "", accept_chirho: request_chirho.headers.get("accept") ?? "",
        caller_key_chirho: request_chirho.headers.get("cairn-idempotency-key-chirho") ?? "", body_chirho: await request_chirho.text(),
      });
      const reply_chirho = replies_chirho.get(path_chirho);
      if (!reply_chirho) return new Response("no such route", { status: 404 });
      return new Response(reply_chirho.body_chirho, { status: reply_chirho.status_chirho, headers: { "content-type": reply_chirho.type_chirho } });
    },
  });
  // A refusal cut short: K1's whole status and headers, then 40 of its 66 promised bytes, then the end.
  cut_server_chirho = createServer(socket_chirho => {
    held_sockets_chirho.push(socket_chirho);
    let seen_chirho = "";
    socket_chirho.on("data", data_chirho => {
      seen_chirho += data_chirho.toString("latin1");
      const head_end_chirho = seen_chirho.indexOf("\r\n\r\n");
      if (head_end_chirho === -1) return;
      const length_chirho = Number(/content-length:\s*(\d+)/i.exec(seen_chirho.slice(0, head_end_chirho))?.[1] ?? 0);
      if (seen_chirho.length < head_end_chirho + 4 + length_chirho) return;
      const body_chirho = k1_body_chirho("key-action-not-granted-chirho");
      socket_chirho.write(`HTTP/1.1 403 Forbidden\r\ncontent-type: application/json\r\ncontent-length: ${body_chirho.length}\r\n\r\n${body_chirho.slice(0, 40)}`);
      setTimeout(() => socket_chirho.destroy(), 20);
    });
    socket_chirho.on("error", () => undefined);
  });
  await new Promise<void>(resolve_chirho => cut_server_chirho.listen(0, "127.0.0.1", () => resolve_chirho()));
});

afterAll(async () => {
  for (const socket_chirho of held_sockets_chirho) socket_chirho.destroy();
  await Promise.all(spawned_chirho.map(client_chirho => client_chirho.close_chirho()));
  fake_chirho.stop(true);
  cut_server_chirho.close();
});

async function session_chirho(port_chirho = fake_chirho.port): Promise<McpStdioClientChirho> {
  const client_chirho = new McpStdioClientChirho({ PERFFECTION_BASE_URL_CHIRHO: `http://127.0.0.1:${port_chirho}`, PERFFECTION_KEY_CHIRHO: KEY_CHIRHO, PERFFECTION_TIMEOUT_MS_CHIRHO: "2000" });
  spawned_chirho.push(client_chirho);
  await legacy_session_chirho(client_chirho);
  return client_chirho;
}
const structured_chirho = (result_chirho: JsonObjectChirho) => result_chirho.structuredContent as JsonObjectChirho;
let next_id_chirho = 1;
const call_chirho = (client_chirho: McpStdioClientChirho, tool_chirho: string) => call_tool_chirho(client_chirho, next_id_chirho++, tool_chirho, ARGUMENTS_CHIRHO[tool_chirho]);
const sent_to_chirho = (path_chirho: string) => recorded_chirho.filter(request_chirho => request_chirho.path_chirho === path_chirho).length;

describe("her figures saved by her own agent", () => {
  test("one fixed POST, the key only as a bearer header, the arguments ARE the body, the answer passed through whole", async () => {
    replies_chirho.clear();
    replies_chirho.set(ROUTES_CHIRHO.save_figures_chirho, json_reply_chirho(ANSWERS_CHIRHO.save_figures_chirho));
    const client_chirho = await session_chirho();
    const before_chirho = recorded_chirho.length;
    const saved_chirho = await call_chirho(client_chirho, "save_figures_chirho");
    expect(saved_chirho.isError).toBe(false);
    expect(structured_chirho(saved_chirho)).toEqual(ANSWERS_CHIRHO.save_figures_chirho);
    const mine_chirho = recorded_chirho.slice(before_chirho);
    expect(mine_chirho.map(request_chirho => `${request_chirho.method_chirho} ${request_chirho.path_chirho}`)).toEqual([`POST ${ROUTES_CHIRHO.save_figures_chirho}`]);
    expect([mine_chirho[0].authorization_chirho, mine_chirho[0].content_type_chirho, mine_chirho[0].accept_chirho]).toEqual([`Bearer ${KEY_CHIRHO}`, "application/json", "application/json"]);
    expect(JSON.parse(mine_chirho[0].body_chirho)).toEqual(ARGUMENTS_CHIRHO.save_figures_chirho as JsonObjectChirho);
  });

  // A geo-elected Perffection admits a write only with exactly one caller key (the carrier contract:
  // spec-chirho/workflows-chirho/platform-route-architecture-chirho/browser-caller-key-carrier-chirho.md).
  test("every request carries exactly one fresh caller key, never reused and never written out", async () => {
    replies_chirho.clear();
    for (const tool_chirho of Object.keys(ROUTES_CHIRHO)) replies_chirho.set(ROUTES_CHIRHO[tool_chirho], json_reply_chirho(ANSWERS_CHIRHO[tool_chirho]));
    const client_chirho = await session_chirho();
    const before_chirho = recorded_chirho.length;
    for (const tool_chirho of [...Object.keys(ROUTES_CHIRHO), "save_page_chirho"]) await call_chirho(client_chirho, tool_chirho);
    const mine_chirho = recorded_chirho.slice(before_chirho);
    expect(mine_chirho.length).toBe(5); // control: four tools, and one save twice, one request each
    const uuid_chirho = /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/;
    for (const request_chirho of mine_chirho) expect([request_chirho.path_chirho, uuid_chirho.test(request_chirho.caller_key_chirho)]).toEqual([request_chirho.path_chirho, true]);
    expect(new Set(mine_chirho.map(request_chirho => request_chirho.caller_key_chirho)).size).toBe(mine_chirho.length);
    const written_chirho = `${client_chirho.stdout_lines_chirho.join("\n")}\n${client_chirho.stderr_chirho}`;
    for (const request_chirho of mine_chirho) expect(written_chirho.includes(request_chirho.caller_key_chirho)).toBe(false);
  });

  test("offered as a destructive, idempotent save of exactly two fields, beside the four tools before it", async () => {
    const client_chirho = await session_chirho();
    const tools_chirho = ((await client_chirho.request_chirho("list", "tools/list")).result as JsonObjectChirho).tools as JsonObjectChirho[];
    expect(tools_chirho.map(tool_chirho => tool_chirho.name)).toEqual(["read_page_chirho", "save_page_chirho", "preview_page_chirho", "read_figures_chirho", "save_figures_chirho"]);
    const save_chirho = tools_chirho[4];
    const annotations_chirho = save_chirho.annotations as JsonObjectChirho;
    expect([annotations_chirho.readOnlyHint, annotations_chirho.destructiveHint, annotations_chirho.idempotentHint]).toEqual([false, true, true]);
    const schema_chirho = save_chirho.inputSchema as JsonObjectChirho;
    expect([schema_chirho.required, schema_chirho.additionalProperties]).toEqual([["expected_version_chirho", "figures_chirho"], false]);
  });

  test("its own refusals come through whole as errors; a route a released server lacks is absent, the save unknown", async () => {
    replies_chirho.clear();
    replies_chirho.set("/healthz-chirho", { status_chirho: 200, type_chirho: "text/plain", body_chirho: "ok" });
    const client_chirho = await session_chirho();
    const base_chirho = { schema_chirho: "perffection-agent-figures-save-v1-chirho", saved_chirho: false, figures_chirho: null, report_chirho: null };
    for (const refusal_chirho of [
      { ...base_chirho, refusal_chirho: "stale-version-chirho", current_version_chirho: 6, issues_chirho: [] },
      { ...base_chirho, refusal_chirho: "invalid-figures-chirho", current_version_chirho: null, issues_chirho: ["month-without-figures-chirho"] },
    ]) {
      replies_chirho.set(ROUTES_CHIRHO.save_figures_chirho, json_reply_chirho(refusal_chirho));
      const result_chirho = await call_chirho(client_chirho, "save_figures_chirho");
      expect(result_chirho.isError).toBe(true);
      expect(structured_chirho(result_chirho)).toEqual(refusal_chirho);
    }
    replies_chirho.delete(ROUTES_CHIRHO.save_figures_chirho);
    const absent_chirho = structured_chirho(await call_chirho(client_chirho, "save_figures_chirho"));
    expect([absent_chirho.http_status_chirho, absent_chirho.reason_chirho, absent_chirho.save_result_chirho]).toEqual([404, "tool-route-absent-chirho", "unknown-chirho"]);
  });
});

describe("every JSON answer is recognized only as its own kind", () => {
  test("each tool given each kind of answer: its own reads as itself, every other is no answer, and a save stays unknown", async () => {
    const client_chirho = await session_chirho();
    for (const tool_chirho of Object.keys(ROUTES_CHIRHO)) {
      for (const [kind_chirho, answer_chirho] of Object.entries(ANSWERS_CHIRHO)) {
        replies_chirho.clear();
        replies_chirho.set(ROUTES_CHIRHO[tool_chirho], json_reply_chirho(answer_chirho));
        const result_chirho = await call_chirho(client_chirho, tool_chirho);
        const seen_chirho = structured_chirho(result_chirho);
        if (kind_chirho === tool_chirho) {
          expect([tool_chirho, result_chirho.isError, seen_chirho]).toEqual([tool_chirho, false, answer_chirho]);
          continue;
        }
        expect([tool_chirho, kind_chirho, seen_chirho.adapter_outcome_chirho, seen_chirho.http_status_chirho]).toEqual([tool_chirho, kind_chirho, "no-answer-chirho", 200]);
        expect([tool_chirho, kind_chirho, seen_chirho.save_result_chirho]).toEqual([tool_chirho, kind_chirho, SAVES_CHIRHO.includes(tool_chirho) ? "unknown-chirho" : undefined]);
      }
    }
  });

  // Each save's honest results as the workspace answers them, and the result law they keep (gpt #26662).
  const DOCUMENTS_CHIRHO: Record<string, string[]> = { save_page_chirho: ["page_chirho"], save_figures_chirho: ["figures_chirho", "report_chirho"] };
  const INVALID_CHIRHO: Record<string, string> = { save_page_chirho: "invalid-page-chirho", save_figures_chirho: "invalid-figures-chirho" };
  const ISSUE_CHIRHO: Record<string, unknown> = {
    save_page_chirho: { field_path_chirho: "business_name_chirho", code_chirho: "required-chirho", max_characters_chirho: null },
    save_figures_chirho: "month-without-figures-chirho",
  };
  const refused_answer_chirho = (tool_chirho: string, refusal_chirho: unknown, current_version_chirho: unknown = null, issues_chirho: unknown = []): JsonObjectChirho => ({
    schema_chirho: ANSWERS_CHIRHO[tool_chirho].schema_chirho, saved_chirho: false, refusal_chirho, current_version_chirho,
    ...Object.fromEntries(DOCUMENTS_CHIRHO[tool_chirho].map(field_chirho => [field_chirho, null])), issues_chirho,
  });

  test("every honest result reads as itself on both saves: saved, stale with its head, invalid with its issues, each plain refusal", async () => {
    const client_chirho = await session_chirho();
    for (const tool_chirho of SAVES_CHIRHO) {
      const honest_chirho: JsonObjectChirho[] = [
        ANSWERS_CHIRHO[tool_chirho],
        // the complete-good control at its smallest: version 1 and empty documents (no second validator here)
        { ...ANSWERS_CHIRHO[tool_chirho], current_version_chirho: 1, ...Object.fromEntries(DOCUMENTS_CHIRHO[tool_chirho].map(field_chirho => [field_chirho, {}])) },
        refused_answer_chirho(tool_chirho, "stale-version-chirho", 6),
        refused_answer_chirho(tool_chirho, INVALID_CHIRHO[tool_chirho], null, [ISSUE_CHIRHO[tool_chirho]]),
        ...["oversize-chirho", "unreadable-chirho", "unsupported-schema-chirho", "unverifiable-chirho"].map(code_chirho => refused_answer_chirho(tool_chirho, code_chirho)),
        // only the figures save refuses its own illustrative sample (F6d)
        ...(tool_chirho === "save_figures_chirho" ? [refused_answer_chirho(tool_chirho, "sample-not-her-figures-chirho")] : []),
      ];
      for (const answer_chirho of honest_chirho) {
        replies_chirho.clear();
        replies_chirho.set(ROUTES_CHIRHO[tool_chirho], json_reply_chirho(answer_chirho));
        const result_chirho = await call_chirho(client_chirho, tool_chirho);
        expect([tool_chirho, JSON.stringify(answer_chirho), result_chirho.isError, structured_chirho(result_chirho)])
          .toEqual([tool_chirho, JSON.stringify(answer_chirho), answer_chirho.saved_chirho !== true, answer_chirho]);
      }
    }
  });

  test("a result a save does not declare, or one that disagrees with itself, is no answer and the save stays unknown", async () => {
    const client_chirho = await session_chirho();
    for (const tool_chirho of SAVES_CHIRHO) {
      const saved_chirho = ANSWERS_CHIRHO[tool_chirho];
      const document_chirho = DOCUMENTS_CHIRHO[tool_chirho][0];
      const other_invalid_chirho = INVALID_CHIRHO[SAVES_CHIRHO.find(other_chirho => other_chirho !== tool_chirho) as string];
      const cases_chirho: [string, JsonObjectChirho][] = [
        // gpt's four (#26662), beside the complete-good control above
        ["saved at version 0", { ...saved_chirho, current_version_chirho: 0 }],
        ["saved at version -1", { ...saved_chirho, current_version_chirho: -1 }],
        ["stale without a head", refused_answer_chirho(tool_chirho, "stale-version-chirho", null)],
        ["a refusal this save does not declare", refused_answer_chirho(tool_chirho, "future-may-have-executed-chirho")],
        // the rest of the law
        ["saved, yet refused", { ...saved_chirho, refusal_chirho: "stale-version-chirho" }],
        ["not saved, with no reason", refused_answer_chirho(tool_chirho, null)],
        ["saved with no version", { ...saved_chirho, current_version_chirho: null }],
        ["a version as text", { ...saved_chirho, current_version_chirho: "5" }],
        ["a fractional version", { ...saved_chirho, current_version_chirho: 1.5 }],
        ["no issues list", { ...saved_chirho, issues_chirho: null }],
        ["saved without its document", { ...saved_chirho, [document_chirho]: null }],
        ["saved with an issue", { ...saved_chirho, issues_chirho: [ISSUE_CHIRHO[tool_chirho]] }],
        ["stale at version 0", refused_answer_chirho(tool_chirho, "stale-version-chirho", 0)],
        ["stale claiming a document", { ...refused_answer_chirho(tool_chirho, "stale-version-chirho", 6), [document_chirho]: {} }],
        ["invalid without an issue", refused_answer_chirho(tool_chirho, INVALID_CHIRHO[tool_chirho])],
        ["invalid with a version", refused_answer_chirho(tool_chirho, INVALID_CHIRHO[tool_chirho], 3, [ISSUE_CHIRHO[tool_chirho]])],
        ["the other save's invalid code", refused_answer_chirho(tool_chirho, other_invalid_chirho, null, [ISSUE_CHIRHO[tool_chirho]])],
        ["unverifiable with a version", refused_answer_chirho(tool_chirho, "unverifiable-chirho", 3)],
        ["oversize with an issue", refused_answer_chirho(tool_chirho, "oversize-chirho", null, [ISSUE_CHIRHO[tool_chirho]])],
        ["the sample refused with a version", refused_answer_chirho(tool_chirho, "sample-not-her-figures-chirho", 3)],
        ["no schema: saved_chirho alone proves nothing", Object.fromEntries(Object.entries(saved_chirho).filter(([key_chirho]) => key_chirho !== "schema_chirho"))],
        ["saved as text", { ...saved_chirho, saved_chirho: "true" }],
      ];
      if (tool_chirho === "save_page_chirho") cases_chirho.push(["the figures' sample refusal on the page save", refused_answer_chirho(tool_chirho, "sample-not-her-figures-chirho")]);
      for (const [label_chirho, answer_chirho] of cases_chirho) {
        replies_chirho.clear();
        replies_chirho.set(ROUTES_CHIRHO[tool_chirho], json_reply_chirho(answer_chirho));
        const seen_chirho = structured_chirho(await call_chirho(client_chirho, tool_chirho));
        expect([tool_chirho, label_chirho, seen_chirho.adapter_outcome_chirho, seen_chirho.save_result_chirho, "saved_chirho" in seen_chirho])
          .toEqual([tool_chirho, label_chirho, "no-answer-chirho", "unknown-chirho", false]);
      }
    }
  });

  test("a read in a state it does not have, or of another schema version, is no answer", async () => {
    const client_chirho = await session_chirho();
    const reads_chirho: [string, JsonObjectChirho][] = [
      ["read_figures_chirho", { ...ANSWERS_CHIRHO.read_figures_chirho, state_chirho: "published-chirho" }],
      ["read_page_chirho", { ...ANSWERS_CHIRHO.read_page_chirho, schema_chirho: "perffection-agent-page-v2-chirho" }],
    ];
    for (const [tool_chirho, answer_chirho] of reads_chirho) {
      replies_chirho.clear();
      replies_chirho.set(ROUTES_CHIRHO[tool_chirho], json_reply_chirho(answer_chirho));
      const seen_chirho = structured_chirho(await call_chirho(client_chirho, tool_chirho));
      expect([tool_chirho, seen_chirho.adapter_outcome_chirho, seen_chirho.save_result_chirho]).toEqual([tool_chirho, "no-answer-chirho", undefined]);
    }
  });
});

describe("K1's fixed refusals, whole, say THIS request was refused before its DAG ran", () => {
  test("on both saves: each fixed answer is its own result, one request was sent, and nothing claims an earlier save", async () => {
    const client_chirho = await session_chirho();
    for (const tool_chirho of SAVES_CHIRHO) {
      for (const [status_chirho, identity_chirho] of K1_REFUSALS_CHIRHO) {
        replies_chirho.clear();
        replies_chirho.set(ROUTES_CHIRHO[tool_chirho], { status_chirho, type_chirho: "application/json", body_chirho: k1_body_chirho(identity_chirho) });
        const before_chirho = sent_to_chirho(ROUTES_CHIRHO[tool_chirho]);
        const result_chirho = await call_chirho(client_chirho, tool_chirho);
        const seen_chirho = structured_chirho(result_chirho);
        expect([tool_chirho, result_chirho.isError, seen_chirho.adapter_outcome_chirho, seen_chirho.save_result_chirho, seen_chirho.http_status_chirho, seen_chirho.refusal_identity_chirho])
          .toEqual([tool_chirho, true, "refused-before-run-chirho", "refused-before-run-chirho", status_chirho, identity_chirho]);
        expect("saved_chirho" in seen_chirho).toBe(false);
        expect(sent_to_chirho(ROUTES_CHIRHO[tool_chirho]) - before_chirho).toBe(1); // no retry of any kind
      }
    }
  });

  test("a read given the same fixed answer is simply no answer: only a save asks whether it ran", async () => {
    const client_chirho = await session_chirho();
    for (const tool_chirho of ["read_page_chirho", "read_figures_chirho"]) {
      replies_chirho.clear();
      replies_chirho.set(ROUTES_CHIRHO[tool_chirho], { status_chirho: 403, type_chirho: "application/json", body_chirho: k1_body_chirho("key-action-not-granted-chirho") });
      const seen_chirho = structured_chirho(await call_chirho(client_chirho, tool_chirho));
      expect([tool_chirho, seen_chirho.adapter_outcome_chirho, seen_chirho.http_status_chirho, seen_chirho.save_result_chirho, seen_chirho.refusal_identity_chirho])
        .toEqual([tool_chirho, "no-answer-chirho", 403, undefined, undefined]);
    }
  });

  test("every near miss stays unknown: another status, a generic identity, a byte more or less, plain text, another type", async () => {
    const client_chirho = await session_chirho();
    const granted_chirho = k1_body_chirho("key-action-not-granted-chirho");
    const near_misses_chirho: ReplyChirho[] = [
      { status_chirho: 409, type_chirho: "application/json", body_chirho: granted_chirho }, // the right identity at the wrong status
      { status_chirho: 403, type_chirho: "application/json", body_chirho: k1_body_chirho("key-route-changed-chirho") },
      { status_chirho: 403, type_chirho: "application/json", body_chirho: k1_body_chirho("request-not-authorized-chirho") }, // a retired key: generic
      { status_chirho: 409, type_chirho: "application/json", body_chirho: k1_body_chirho("conflict-chirho") },
      { status_chirho: 403, type_chirho: "application/json", body_chirho: `${granted_chirho}\n` },
      { status_chirho: 403, type_chirho: "application/json", body_chirho: `﻿${granted_chirho}` }, // a UTF-8 byte-order mark first (gpt #28596)
      { status_chirho: 403, type_chirho: "application/json", body_chirho: granted_chirho.slice(0, -1) }, // an exact prefix, complete as sent
      { status_chirho: 403, type_chirho: "application/json", body_chirho: '{"ok_chirho":false,"error_chirho":"key-action-not-granted-chirho"}' },
      { status_chirho: 403, type_chirho: "application/json", body_chirho: '{"error_chirho":"key-action-not-granted-chirho","ok_chirho":false,"retry_chirho":true}' },
      { status_chirho: 403, type_chirho: "application/json", body_chirho: k1_body_chirho("key-action-not-granted-chirho-elsewhere") },
      { status_chirho: 403, type_chirho: "application/json", body_chirho: `${granted_chirho}${" ".repeat(300)}` }, // past the bound
      { status_chirho: 403, type_chirho: "text/plain; charset=utf-8", body_chirho: "error: key-action-not-granted-chirho\n" },
      { status_chirho: 403, type_chirho: "application/json; charset=utf-8", body_chirho: granted_chirho },
    ];
    // control: the mark is three real bytes (EF BB BF) ahead of the exact refusal, which alone is K1's whole answer
    expect([...new TextEncoder().encode(`﻿${granted_chirho}`)].slice(0, 3)).toEqual([0xef, 0xbb, 0xbf]);
    for (const reply_chirho of near_misses_chirho) {
      replies_chirho.clear();
      replies_chirho.set(ROUTES_CHIRHO.save_figures_chirho, reply_chirho);
      const seen_chirho = structured_chirho(await call_chirho(client_chirho, "save_figures_chirho"));
      expect([JSON.stringify(reply_chirho), seen_chirho.adapter_outcome_chirho, seen_chirho.save_result_chirho, seen_chirho.http_status_chirho])
        .toEqual([JSON.stringify(reply_chirho), "no-answer-chirho", "unknown-chirho", reply_chirho.status_chirho]);
    }
    // cut short: the whole status and headers, part of the body, then the connection ends
    const cut_chirho = structured_chirho(await call_chirho(await session_chirho((cut_server_chirho.address() as AddressInfo).port), "save_figures_chirho"));
    expect([cut_chirho.adapter_outcome_chirho, cut_chirho.save_result_chirho, cut_chirho.http_status_chirho]).toEqual(["no-answer-chirho", "unknown-chirho", 403]);
  });

  test("the key never appeared on stdout or stderr in any run here", async () => {
    await Promise.all(spawned_chirho.map(client_chirho => client_chirho.close_chirho()));
    const written_chirho = spawned_chirho.map(client_chirho => `${client_chirho.stdout_lines_chirho.join("\n")}\n${client_chirho.stderr_chirho}`).join("\n");
    expect(`${written_chirho}${KEY_CHIRHO}`.includes(KEY_CHIRHO)).toBe(true); // control: the scan finds a planted key
    expect(spawned_chirho.length).toBeGreaterThan(5);
    expect(written_chirho.includes(KEY_CHIRHO)).toBe(false);
  });
});
