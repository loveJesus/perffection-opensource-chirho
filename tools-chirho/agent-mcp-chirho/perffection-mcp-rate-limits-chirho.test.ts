// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// Checks for the adapter's two 429s (perffection-mcp-chirho.ts, FIXED_REFUSALS_CHIRHO; gpt #28569): the
// served gate's busy answer, whole, carries its wait in seconds; the key's own call budget, whole, keeps its
// own meaning and carries no wait; and every lookalike, a missing, malformed, duplicate or oversized
// Retry-After included, stays unknown. Nothing is ever retried, and the key is never written out. Behavior
// only, never prose. A raw loopback server answers every route with exactly the status line, headers (a
// header may repeat) and bytes a test sets, and records every request.
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
const ARGUMENTS_CHIRHO: Record<string, JsonObjectChirho | undefined> = {
  read_page_chirho: undefined,
  save_page_chirho: { expected_version_chirho: 2, page_chirho: { business_name_chirho: "Fixture Bakery" } },
  read_figures_chirho: undefined,
  save_figures_chirho: { expected_version_chirho: 4, figures_chirho: { schema_chirho: "perffection-figures-v1-chirho" } },
};
const SAVES_CHIRHO = ["save_page_chirho", "save_figures_chirho"];
const READS_CHIRHO = ["read_page_chirho", "read_figures_chirho"];
const KEY_CHIRHO = Array.from(crypto.getRandomValues(new Uint8Array(32)), byte_chirho => byte_chirho.toString(16).padStart(2, "0")).join("");

// The two 429s exactly as Cairn answers a request asking for JSON (R16 a07696ca): the served gate's constant
// body with one Retry-After, and the key budget's request-context body with none.
const BUSY_BODY_CHIRHO = '{"ok_chirho":false,"error_chirho":"served-request-unavailable-chirho"}';
const BUDGET_BODY_CHIRHO = '{"error_chirho":"rate-limit-exceeded-chirho","ok_chirho":false}';
const JSON_TYPE_CHIRHO: [string, string] = ["content-type", "application/json"];
// U+FEFF, written by the server as the three bytes EF BB BF before a body (gpt #28596).
const BOM_CHIRHO = "﻿";
type ReplyChirho = { status_chirho: number; headers_chirho: [string, string][]; body_chirho: string; end_chirho?: "whole" | "cut" | "stall" };
const busy_chirho = (...retry_after_chirho: string[]): ReplyChirho =>
  ({ status_chirho: 429, headers_chirho: [JSON_TYPE_CHIRHO, ...retry_after_chirho.map((value_chirho): [string, string] => ["retry-after", value_chirho])], body_chirho: BUSY_BODY_CHIRHO });
const BUDGET_CHIRHO: ReplyChirho = { status_chirho: 429, headers_chirho: [JSON_TYPE_CHIRHO], body_chirho: BUDGET_BODY_CHIRHO };

let reply_chirho: ReplyChirho = BUDGET_CHIRHO;
const recorded_chirho: string[] = [];
const written_chirho: Buffer[] = []; // each reply's whole body, exactly as the server wrote (or began to write) it
const held_sockets_chirho: Socket[] = [];
const spawned_chirho: McpStdioClientChirho[] = [];
let server_chirho: Server;

// One request per connection: read the head and its declared body, record the path, write the set reply
// (whole; or its head and 40 body bytes, then the end of the connection, or nothing more at all).
function answer_chirho(socket_chirho: Socket, path_chirho: string): void {
  recorded_chirho.push(path_chirho);
  const body_chirho = Buffer.from(reply_chirho.body_chirho, "utf8");
  written_chirho.push(body_chirho);
  const head_chirho = [`HTTP/1.1 ${reply_chirho.status_chirho} Status`, ...reply_chirho.headers_chirho.map(([name_chirho, value_chirho]) => `${name_chirho}: ${value_chirho}`),
    `content-length: ${body_chirho.length}`, "connection: close", "", ""].join("\r\n");
  const end_chirho = reply_chirho.end_chirho ?? "whole";
  if (end_chirho === "whole") {
    socket_chirho.end(Buffer.concat([Buffer.from(head_chirho, "latin1"), body_chirho]));
    return;
  }
  socket_chirho.write(Buffer.concat([Buffer.from(head_chirho, "latin1"), body_chirho.subarray(0, 40)]));
  if (end_chirho === "cut") setTimeout(() => socket_chirho.destroy(), 20);
}

beforeAll(async () => {
  server_chirho = createServer(socket_chirho => {
    held_sockets_chirho.push(socket_chirho);
    let seen_chirho = Buffer.alloc(0);
    let answered_chirho = false;
    socket_chirho.on("data", data_chirho => {
      seen_chirho = Buffer.concat([seen_chirho, typeof data_chirho === "string" ? Buffer.from(data_chirho, "latin1") : data_chirho]);
      const head_end_chirho = seen_chirho.indexOf("\r\n\r\n");
      if (answered_chirho || head_end_chirho === -1) return;
      const head_chirho = seen_chirho.subarray(0, head_end_chirho).toString("latin1");
      const length_chirho = Number(/content-length:\s*(\d+)/i.exec(head_chirho)?.[1] ?? 0);
      if (seen_chirho.length < head_end_chirho + 4 + length_chirho) return;
      answered_chirho = true;
      answer_chirho(socket_chirho, head_chirho.split(" ")[1] ?? "");
    });
    socket_chirho.on("error", () => undefined);
  });
  await new Promise<void>(resolve_chirho => server_chirho.listen(0, "127.0.0.1", () => resolve_chirho()));
});

afterAll(async () => {
  for (const socket_chirho of held_sockets_chirho) socket_chirho.destroy();
  await Promise.all(spawned_chirho.map(client_chirho => client_chirho.close_chirho()));
  server_chirho.close();
});

async function session_chirho(timeout_ms_chirho = "2000", runtime_chirho?: string): Promise<McpStdioClientChirho> {
  const port_chirho = (server_chirho.address() as AddressInfo).port;
  const env_chirho = { PERFFECTION_BASE_URL_CHIRHO: `http://127.0.0.1:${port_chirho}`, PERFFECTION_KEY_CHIRHO: KEY_CHIRHO, PERFFECTION_TIMEOUT_MS_CHIRHO: timeout_ms_chirho };
  const client_chirho = new McpStdioClientChirho(env_chirho, runtime_chirho);
  spawned_chirho.push(client_chirho);
  await legacy_session_chirho(client_chirho);
  return client_chirho;
}
let next_id_chirho = 1;
// One tool call under the set reply: its structured result, and how many requests it sent.
async function call_chirho(client_chirho: McpStdioClientChirho, tool_chirho: string, set_chirho: ReplyChirho): Promise<{ seen_chirho: JsonObjectChirho; sent_chirho: number }> {
  reply_chirho = set_chirho;
  const before_chirho = recorded_chirho.length;
  const result_chirho = await call_tool_chirho(client_chirho, next_id_chirho++, tool_chirho, ARGUMENTS_CHIRHO[tool_chirho]);
  const sent_chirho = recorded_chirho.slice(before_chirho);
  expect(sent_chirho.every(path_chirho => path_chirho === ROUTES_CHIRHO[tool_chirho])).toBe(true);
  return { seen_chirho: result_chirho.structuredContent as JsonObjectChirho, sent_chirho: sent_chirho.length };
}

describe("the two 429s, each whole, read as themselves", () => {
  test("on both saves: refused before its DAG ran, busy with its wait in seconds, the budget with none; one request each", async () => {
    const client_chirho = await session_chirho();
    for (const tool_chirho of SAVES_CHIRHO) {
      for (const [set_chirho, identity_chirho, wait_chirho] of [[busy_chirho("60"), "served-request-unavailable-chirho", 60], [BUDGET_CHIRHO, "rate-limit-exceeded-chirho", undefined]] as const) {
        const { seen_chirho, sent_chirho } = await call_chirho(client_chirho, tool_chirho, set_chirho);
        expect([tool_chirho, seen_chirho.adapter_outcome_chirho, seen_chirho.save_result_chirho, seen_chirho.http_status_chirho, seen_chirho.refusal_identity_chirho, seen_chirho.retry_after_seconds_chirho, sent_chirho])
          .toEqual([tool_chirho, "refused-before-run-chirho", "refused-before-run-chirho", 429, identity_chirho, wait_chirho, 1]);
        expect("saved_chirho" in seen_chirho).toBe(false);
      }
    }
  });

  test("on both reads: no answer that names which 429 it was by the engine's identity, the wait only when busy", async () => {
    const client_chirho = await session_chirho();
    for (const tool_chirho of READS_CHIRHO) {
      for (const [set_chirho, identity_chirho, wait_chirho] of [[busy_chirho("60"), "served-request-unavailable-chirho", 60], [BUDGET_CHIRHO, "rate-limit-exceeded-chirho", undefined]] as const) {
        const { seen_chirho, sent_chirho } = await call_chirho(client_chirho, tool_chirho, set_chirho);
        expect([tool_chirho, seen_chirho.adapter_outcome_chirho, seen_chirho.http_status_chirho, seen_chirho.reason_chirho, seen_chirho.retry_after_seconds_chirho, seen_chirho.save_result_chirho, seen_chirho.refusal_identity_chirho, sent_chirho])
          .toEqual([tool_chirho, "no-answer-chirho", 429, identity_chirho, wait_chirho, undefined, undefined, 1]);
      }
    }
  });

  test("busy carries its wait exactly across the recognized range, up to the largest exact JSON number", async () => {
    const client_chirho = await session_chirho();
    for (const value_chirho of ["1", "60", "86400", String(Number.MAX_SAFE_INTEGER)]) {
      const { seen_chirho } = await call_chirho(client_chirho, "save_page_chirho", busy_chirho(value_chirho));
      expect([value_chirho, seen_chirho.refusal_identity_chirho, seen_chirho.retry_after_seconds_chirho]).toEqual([value_chirho, "served-request-unavailable-chirho", Number(value_chirho)]);
    }
  });

  test("under node as under bun: busy with its wait, the budget with none, a lookalike unknown", async () => {
    const node_chirho = Bun.which("node");
    expect(node_chirho).toBeTruthy();
    const client_chirho = await session_chirho("2000", node_chirho ?? undefined);
    for (const [set_chirho, outcome_chirho, identity_chirho, wait_chirho] of [
      [busy_chirho("60"), "refused-before-run-chirho", "served-request-unavailable-chirho", 60],
      [BUDGET_CHIRHO, "refused-before-run-chirho", "rate-limit-exceeded-chirho", undefined],
      [busy_chirho("60", "60"), "no-answer-chirho", undefined, undefined],
      [{ ...busy_chirho("60"), body_chirho: `${BOM_CHIRHO}${BUSY_BODY_CHIRHO}` }, "no-answer-chirho", undefined, undefined],
      [{ ...BUDGET_CHIRHO, body_chirho: `${BOM_CHIRHO}${BUDGET_BODY_CHIRHO}` }, "no-answer-chirho", undefined, undefined],
    ] as const) {
      const { seen_chirho, sent_chirho } = await call_chirho(client_chirho, "save_page_chirho", set_chirho);
      expect([seen_chirho.adapter_outcome_chirho, seen_chirho.refusal_identity_chirho, seen_chirho.retry_after_seconds_chirho, sent_chirho]).toEqual([outcome_chirho, identity_chirho, wait_chirho, 1]);
    }
  });

  test("nothing is retried: one request, even well past a one-second wait", async () => {
    const client_chirho = await session_chirho();
    for (const tool_chirho of ["save_figures_chirho", "read_figures_chirho"]) {
      const { seen_chirho, sent_chirho } = await call_chirho(client_chirho, tool_chirho, busy_chirho("1"));
      expect([tool_chirho, seen_chirho.retry_after_seconds_chirho, sent_chirho]).toEqual([tool_chirho, 1, 1]);
    }
    const before_chirho = recorded_chirho.length;
    await Bun.sleep(1_500);
    expect(recorded_chirho.length).toBe(before_chirho);
  });
});

describe("every lookalike stays unknown", () => {
  const near_misses_chirho: [string, ReplyChirho][] = [
    ["busy with no Retry-After", busy_chirho()],
    ["busy with an empty Retry-After", busy_chirho("")],
    ["busy waiting 0", busy_chirho("0")],
    ["busy with a leading zero", busy_chirho("060")],
    ["busy with a sign", busy_chirho("+60")],
    ["busy with a negative wait", busy_chirho("-60")],
    ["busy with a fraction", busy_chirho("60.0")],
    ["busy in exponent form", busy_chirho("1e3")],
    ["busy with a unit", busy_chirho("60s")],
    ["busy with an HTTP date", busy_chirho("Wed, 21 Oct 2026 07:28:00 GMT")],
    ["busy one past the largest exact JSON number", busy_chirho("9007199254740992")],
    ["busy at the engine's own maximum, past exact JSON numbers", busy_chirho("18446744073709552")],
    ["busy with an oversized Retry-After", busy_chirho(`1${"0".repeat(4_000)}`)],
    ["busy with two equal Retry-After headers", busy_chirho("60", "60")],
    ["busy with two different Retry-After headers", busy_chirho("60", "30")],
    ["busy whose Retry-After is the key", busy_chirho(KEY_CHIRHO)],
    ["the budget with a wait it never carries", { ...BUDGET_CHIRHO, headers_chirho: [JSON_TYPE_CHIRHO, ["retry-after", "60"]] }],
    ["the budget with an empty Retry-After", { ...BUDGET_CHIRHO, headers_chirho: [JSON_TYPE_CHIRHO, ["retry-after", ""]] }],
    ["busy's body at 503", { ...busy_chirho("60"), status_chirho: 503 }],
    ["busy's body at 403", { ...busy_chirho("60"), status_chirho: 403 }],
    ["the budget's body at 403", { ...BUDGET_CHIRHO, status_chirho: 403 }],
    ["busy as JSON with a charset", { ...busy_chirho("60"), headers_chirho: [["content-type", "application/json; charset=utf-8"], ["retry-after", "60"]] }],
    ["busy as the engine's plain text", { status_chirho: 429, headers_chirho: [["content-type", "text/plain; charset=utf-8"], ["retry-after", "60"]], body_chirho: "served request unavailable\n" }],
    ["busy's fields in the budget's order", { ...busy_chirho("60"), body_chirho: '{"error_chirho":"served-request-unavailable-chirho","ok_chirho":false}' }],
    ["the budget's fields in busy's order", { ...BUDGET_CHIRHO, body_chirho: '{"ok_chirho":false,"error_chirho":"rate-limit-exceeded-chirho"}' }],
    ["the budget's identity in busy's form, with a wait", { ...busy_chirho("60"), body_chirho: '{"ok_chirho":false,"error_chirho":"rate-limit-exceeded-chirho"}' }],
    ["a key refusal's identity at 429", { ...BUDGET_CHIRHO, body_chirho: '{"error_chirho":"key-action-not-granted-chirho","ok_chirho":false}' }],
    ["busy after a UTF-8 byte-order mark", { ...busy_chirho("60"), body_chirho: `${BOM_CHIRHO}${BUSY_BODY_CHIRHO}` }],
    ["the budget after a UTF-8 byte-order mark", { ...BUDGET_CHIRHO, body_chirho: `${BOM_CHIRHO}${BUDGET_BODY_CHIRHO}` }],
    ["busy with a newline after", { ...busy_chirho("60"), body_chirho: `${BUSY_BODY_CHIRHO}\n` }],
    ["busy a byte short", { ...busy_chirho("60"), body_chirho: BUSY_BODY_CHIRHO.slice(0, -1) }],
    ["busy with a field more", { ...busy_chirho("60"), body_chirho: '{"ok_chirho":false,"error_chirho":"served-request-unavailable-chirho","retry_chirho":true}' }],
    ["busy past the body bound", { ...busy_chirho("60"), body_chirho: `${BUSY_BODY_CHIRHO}${" ".repeat(300)}` }],
    ["a 429 that echoes the key", { status_chirho: 429, headers_chirho: [JSON_TYPE_CHIRHO, ["retry-after", "60"]], body_chirho: `{"ok_chirho":false,"error_chirho":"${KEY_CHIRHO}"}` }],
    ["busy cut short", { ...busy_chirho("60"), end_chirho: "cut" }],
  ];

  test("on a save: no answer, the save unknown, no reason, no wait, one request; on a read: no answer, no reason, no wait", async () => {
    const client_chirho = await session_chirho();
    for (const [label_chirho, set_chirho] of near_misses_chirho) {
      const save_chirho = await call_chirho(client_chirho, "save_figures_chirho", set_chirho);
      expect([label_chirho, save_chirho.seen_chirho.adapter_outcome_chirho, save_chirho.seen_chirho.save_result_chirho, save_chirho.seen_chirho.http_status_chirho,
        save_chirho.seen_chirho.reason_chirho, save_chirho.seen_chirho.retry_after_seconds_chirho, save_chirho.seen_chirho.refusal_identity_chirho, save_chirho.sent_chirho])
        .toEqual([label_chirho, "no-answer-chirho", "unknown-chirho", set_chirho.status_chirho, undefined, undefined, undefined, 1]);
      const read_chirho = await call_chirho(client_chirho, "read_page_chirho", set_chirho);
      expect([label_chirho, read_chirho.seen_chirho.adapter_outcome_chirho, read_chirho.seen_chirho.reason_chirho, read_chirho.seen_chirho.retry_after_seconds_chirho, read_chirho.sent_chirho])
        .toEqual([label_chirho, "no-answer-chirho", undefined, undefined, 1]);
    }
  });

  test("a byte-order mark is three more bytes on the wire, EF BB BF, and never the exact body after it", async () => {
    const client_chirho = await session_chirho();
    for (const [label_chirho, exact_chirho, identity_chirho, length_chirho] of [
      ["busy", busy_chirho("60"), "served-request-unavailable-chirho", 70],
      ["the budget", BUDGET_CHIRHO, "rate-limit-exceeded-chirho", 63],
    ] as const) {
      const before_chirho = written_chirho.length;
      const plain_chirho = await call_chirho(client_chirho, "save_page_chirho", exact_chirho);
      const marked_chirho = await call_chirho(client_chirho, "save_page_chirho", { ...exact_chirho, body_chirho: `${BOM_CHIRHO}${exact_chirho.body_chirho}` });
      const [plain_bytes_chirho, marked_bytes_chirho] = written_chirho.slice(before_chirho);
      expect([label_chirho, plain_bytes_chirho.length, marked_bytes_chirho.length, [...marked_bytes_chirho.subarray(0, 3)], marked_bytes_chirho.subarray(3).equals(plain_bytes_chirho)])
        .toEqual([label_chirho, length_chirho, length_chirho + 3, [0xef, 0xbb, 0xbf], true]);
      expect([label_chirho, plain_chirho.seen_chirho.refusal_identity_chirho, marked_chirho.seen_chirho.adapter_outcome_chirho, marked_chirho.seen_chirho.save_result_chirho, marked_chirho.sent_chirho])
        .toEqual([label_chirho, identity_chirho, "no-answer-chirho", "unknown-chirho", 1]);
    }
  });

  test("an unrecognized 429 never reads as the key's budget or as busy", async () => {
    const client_chirho = await session_chirho();
    const meaning_chirho = async (set_chirho: ReplyChirho) => (await call_chirho(client_chirho, "save_page_chirho", set_chirho)).seen_chirho.meaning_chirho;
    const unknown_chirho = await meaning_chirho({ status_chirho: 429, headers_chirho: [["content-type", "text/plain"]], body_chirho: "slow down\n" });
    expect(unknown_chirho).not.toBe(await meaning_chirho(BUDGET_CHIRHO));
    expect(unknown_chirho).not.toBe(await meaning_chirho(busy_chirho("60")));
  });

  test("busy whose body stalls is no answer within the adapter's own time bound", async () => {
    const client_chirho = await session_chirho("1000");
    const started_chirho = Date.now();
    const { seen_chirho, sent_chirho } = await call_chirho(client_chirho, "save_page_chirho", { ...busy_chirho("60"), end_chirho: "stall" });
    expect([seen_chirho.adapter_outcome_chirho, seen_chirho.save_result_chirho, seen_chirho.retry_after_seconds_chirho, sent_chirho]).toEqual(["no-answer-chirho", "unknown-chirho", undefined, 1]);
    expect(Date.now() - started_chirho).toBeLessThan(4_000);
  });
});

test("the key never appeared on stdout or stderr in any run here", async () => {
  await Promise.all(spawned_chirho.map(client_chirho => client_chirho.close_chirho()));
  const written_chirho = spawned_chirho.map(client_chirho => `${client_chirho.stdout_lines_chirho.join("\n")}\n${client_chirho.stderr_chirho}`).join("\n");
  expect(`${written_chirho}${KEY_CHIRHO}`.includes(KEY_CHIRHO)).toBe(true); // control: the scan finds a planted key
  expect(spawned_chirho.length).toBeGreaterThan(5);
  expect(written_chirho.includes(KEY_CHIRHO)).toBe(false);
});
