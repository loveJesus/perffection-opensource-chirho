// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// Checks for perffection-mcp-chirho.ts, the MCP adapter a customer runs on their own machine.
// Behavior and safety only, never prose: the key never travels in the clear and never comes back
// out; the routes are fixed; an incomplete figures report is never told as complete; a save
// whose answer never arrived is UNKNOWN (never "saved",
// never "nothing stored"); input, answers and waiting are bounded; both protocol eras answer; Node
// runs the same file. A fake workspace records every request the adapter makes.
// Run: bun test tools-chirho/agent-mcp-chirho/
import { afterAll, beforeAll, describe, expect, test } from "bun:test";
import { createServer, type AddressInfo, type Server, type Socket } from "node:net";
import { chmodSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { McpStdioClientChirho, call_tool_chirho as call_chirho, legacy_session_chirho, modern_params_chirho, type JsonObjectChirho } from "./mcp-stdio-client-chirho";

const READ_ROUTE_CHIRHO = "/app-chirho/perffection-workspace/agent-page-chirho";
const SAVE_ROUTE_CHIRHO = "/app-chirho/perffection-workspace/agent-save-page-chirho";
const PREVIEW_ROUTE_CHIRHO = "/app-chirho/perffection-workspace/preview-chirho";
const FIGURES_ROUTE_CHIRHO = "/app-chirho/perffection-workspace/agent-figures-chirho";
const random_key_chirho = () => Array.from(crypto.getRandomValues(new Uint8Array(32)), byte_chirho => byte_chirho.toString(16).padStart(2, "0")).join("");
const KEY_CHIRHO = random_key_chirho();
const FILE_KEY_CHIRHO = random_key_chirho();

const PAGE_ANSWER_CHIRHO = {
  schema_chirho: "perffection-agent-page-v1-chirho", state_chirho: "saved-chirho", current_version_chirho: 2,
  page_chirho: { business_name_chirho: "Fixture Bakery" }, surface_chirho: { routes_chirho: [] },
};
const PAGE_SAVE_SCHEMA_CHIRHO = "perffection-agent-save-v1-chirho";
const REFUSED_ANSWER_CHIRHO = {
  schema_chirho: PAGE_SAVE_SCHEMA_CHIRHO, saved_chirho: false, refusal_chirho: "invalid-page-chirho", current_version_chirho: null, page_chirho: null,
  issues_chirho: [{ field_path_chirho: "business_name_chirho", code_chirho: "required-chirho", max_characters_chirho: null }],
};
const STALE_ANSWER_CHIRHO = { schema_chirho: PAGE_SAVE_SCHEMA_CHIRHO, saved_chirho: false, refusal_chirho: "stale-version-chirho", current_version_chirho: 5, page_chirho: null, issues_chirho: [] };
const PREVIEW_HTML_CHIRHO = "<!doctype html><title>Fixture Bakery</title><h1>Fixture Bakery</h1>";
// The figures answer, with the report's completeness as given (undefined leaves the flag out).
const figures_answer_chirho = (complete_chirho?: unknown, state_chirho = "saved-chirho") => ({
  schema_chirho: "perffection-agent-figures-v1-chirho", state_chirho, current_version_chirho: 4, as_of_month_chirho: "2026-02",
  report_chirho: complete_chirho === undefined ? { kinds_chirho: [] } : { complete_chirho, kinds_chirho: [] },
  figures_chirho: { currency_chirho: "USD" }, meanings_chirho: { unknown_chirho: "null means unknown" },
});
let figures_reply_chirho: unknown = figures_answer_chirho(true);

type RecordedChirho = { method_chirho: string; path_chirho: string; authorization_chirho: string; content_type_chirho: string; body_chirho: string };
const recorded_chirho: RecordedChirho[] = [];
const trap_hits_chirho: string[] = [];
let scenario_chirho = "normal";
let slow_ms_chirho = 0;
let in_flight_chirho = 0;
let most_in_flight_chirho = 0;
const spawned_chirho: McpStdioClientChirho[] = [];
const held_sockets_chirho: Socket[] = [];
let fake_chirho: ReturnType<typeof Bun.serve>;
let trap_chirho: ReturnType<typeof Bun.serve>;
let drop_server_chirho: Server;
let cut_server_chirho: Server;
let hang_server_chirho: Server;
let key_dir_chirho = "";

const json_answer_chirho = (value_chirho: unknown, status_chirho = 200) =>
  new Response(JSON.stringify(value_chirho), { status: status_chirho, headers: { "content-type": "application/json; charset=utf-8" } });

function fake_answer_chirho(path_chirho: string, request_chirho: Request, body_chirho: string): Response {
  const saved_chirho = () => ({ schema_chirho: PAGE_SAVE_SCHEMA_CHIRHO, saved_chirho: true, refusal_chirho: null, current_version_chirho: 3, page_chirho: (JSON.parse(body_chirho) as JsonObjectChirho).page_chirho ?? null, issues_chirho: [] });
  if (scenario_chirho === "normal") {
    if (path_chirho === READ_ROUTE_CHIRHO) return json_answer_chirho(PAGE_ANSWER_CHIRHO);
    if (path_chirho === SAVE_ROUTE_CHIRHO) return json_answer_chirho(saved_chirho());
    if (path_chirho === PREVIEW_ROUTE_CHIRHO) return new Response(PREVIEW_HTML_CHIRHO, { headers: { "content-type": "text/html; charset=utf-8" } });
    if (path_chirho === FIGURES_ROUTE_CHIRHO) return json_answer_chirho(figures_reply_chirho);
    return new Response("no such route", { status: 404 });
  }
  if (scenario_chirho === "refuse") return json_answer_chirho(REFUSED_ANSWER_CHIRHO);
  if (scenario_chirho === "stale") return json_answer_chirho(STALE_ANSWER_CHIRHO);
  if (scenario_chirho === "echo-key") {
    const echoed_chirho = request_chirho.headers.get("authorization");
    return json_answer_chirho(path_chirho === SAVE_ROUTE_CHIRHO ? { ...saved_chirho(), echoed_chirho } : { ...PAGE_ANSWER_CHIRHO, echoed_chirho });
  }
  // A Perffection server older than the tool: its health answers, the tool's route does not exist.
  if (scenario_chirho === "unsupported") return path_chirho === "/healthz-chirho" ? new Response("ok") : new Response("no such route", { status: 404 });
  // A healthy service that is not Perffection at all: the adapter must not claim it is.
  if (scenario_chirho === "foreign-healthy") return path_chirho === "/healthz-chirho" ? json_answer_chirho({ status: "up", service: "another" }) : new Response("Not Found", { status: 404 });
  // The right address in an outage: its health fails, or never answers.
  if (scenario_chirho === "health-500") return path_chirho === "/healthz-chirho" ? new Response("down", { status: 500 }) : new Response("no such route", { status: 404 });
  if (scenario_chirho === "health-hangs") return path_chirho === "/healthz-chirho" ? (new Promise<Response>(() => undefined) as unknown as Response) : new Response("no such route", { status: 404 });
  if (scenario_chirho === "not-perffection") return json_answer_chirho({ hello_chirho: "world" });
  if (scenario_chirho === "wrong-type") return new Response("<p>a sign-in page</p>", { headers: { "content-type": "text/html" } });
  if (scenario_chirho === "huge") return json_answer_chirho({ ...PAGE_ANSWER_CHIRHO, saved_chirho: true, padding_chirho: "x".repeat(1_500_000) });
  if (scenario_chirho === "redirect") return new Response(null, { status: 302, headers: { location: `http://127.0.0.1:${trap_chirho.port}/steal-chirho` } });
  const status_chirho = Number(scenario_chirho.replace("status-", ""));
  return json_answer_chirho({ note_chirho: `status ${status_chirho}` }, status_chirho);
}

// A raw socket server, once the whole request has arrived: `drop` closes without answering, `cut`
// starts a 200 answer and closes halfway through its body, `hang` never answers.
function raw_server_chirho(mode_chirho: "drop" | "cut" | "hang"): Promise<Server> {
  const server_chirho = createServer(socket_chirho => {
    held_sockets_chirho.push(socket_chirho);
    let seen_chirho = "";
    socket_chirho.on("data", data_chirho => {
      seen_chirho += data_chirho.toString("latin1");
      const head_end_chirho = seen_chirho.indexOf("\r\n\r\n");
      if (head_end_chirho === -1) return;
      const length_chirho = Number(/content-length:\s*(\d+)/i.exec(seen_chirho.slice(0, head_end_chirho))?.[1] ?? 0);
      if (seen_chirho.length < head_end_chirho + 4 + length_chirho) return;
      recorded_chirho.push({ method_chirho: seen_chirho.split(" ")[0], path_chirho: seen_chirho.split(" ")[1], authorization_chirho: "", content_type_chirho: "", body_chirho: "" });
      if (mode_chirho === "cut") socket_chirho.write('HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: 100\r\n\r\n{"saved_chirho": tr');
      if (mode_chirho !== "hang") setTimeout(() => socket_chirho.destroy(), mode_chirho === "cut" ? 20 : 0);
    });
    socket_chirho.on("error", () => undefined);
  });
  return new Promise(resolve_chirho => server_chirho.listen(0, "127.0.0.1", () => resolve_chirho(server_chirho)));
}
const port_of_chirho = (server_chirho: Server) => (server_chirho.address() as AddressInfo).port;

beforeAll(async () => {
  fake_chirho = Bun.serve({
    hostname: "127.0.0.1", port: 0,
    async fetch(request_chirho) {
      const url_chirho = new URL(request_chirho.url);
      const body_chirho = await request_chirho.text();
      recorded_chirho.push({
        method_chirho: request_chirho.method, path_chirho: url_chirho.pathname + url_chirho.search,
        authorization_chirho: request_chirho.headers.get("authorization") ?? "", content_type_chirho: request_chirho.headers.get("content-type") ?? "", body_chirho,
      });
      in_flight_chirho += 1;
      most_in_flight_chirho = Math.max(most_in_flight_chirho, in_flight_chirho);
      try {
        if (slow_ms_chirho) await Bun.sleep(slow_ms_chirho);
        return fake_answer_chirho(url_chirho.pathname, request_chirho, body_chirho);
      } finally {
        in_flight_chirho -= 1;
      }
    },
  });
  trap_chirho = Bun.serve({ hostname: "127.0.0.1", port: 0, fetch(request_chirho) { trap_hits_chirho.push(request_chirho.url); return new Response("trapped"); } });
  drop_server_chirho = await raw_server_chirho("drop");
  cut_server_chirho = await raw_server_chirho("cut");
  hang_server_chirho = await raw_server_chirho("hang");
  key_dir_chirho = mkdtempSync(join(tmpdir(), "perffection-mcp-test-chirho-"));
});

afterAll(async () => {
  for (const socket_chirho of held_sockets_chirho) socket_chirho.destroy();
  await Promise.all(spawned_chirho.map(client_chirho => client_chirho.close_chirho()));
  fake_chirho.stop(true);
  trap_chirho.stop(true);
  drop_server_chirho.close();
  cut_server_chirho.close();
  hang_server_chirho.close();
  rmSync(key_dir_chirho, { recursive: true, force: true });
});

function adapter_chirho(env_chirho: Record<string, string> = {}, runtime_chirho?: string): McpStdioClientChirho {
  const client_chirho = new McpStdioClientChirho({
    PERFFECTION_BASE_URL_CHIRHO: `http://127.0.0.1:${fake_chirho.port}`, PERFFECTION_KEY_CHIRHO: KEY_CHIRHO, PERFFECTION_TIMEOUT_MS_CHIRHO: "2000", ...env_chirho,
  }, runtime_chirho);
  spawned_chirho.push(client_chirho);
  return client_chirho;
}

const structured_chirho = (result_chirho: JsonObjectChirho) => result_chirho.structuredContent as JsonObjectChirho;
const SAVE_ARGUMENTS_CHIRHO = { expected_version_chirho: 2, page_chirho: { business_name_chirho: "../../../etc/passwd?steal=1#x" } };

describe("the protocol, in both eras", () => {
  test("legacy: echoes a version it speaks, offers its newest for one it does not, lists five tools, never answers a notification", async () => {
    const client_chirho = adapter_chirho();
    const init_chirho = await legacy_session_chirho(client_chirho, "2025-06-18");
    expect(init_chirho.protocolVersion).toBe("2025-06-18");
    expect((init_chirho.capabilities as JsonObjectChirho).tools).toEqual({});
    client_chirho.send_chirho({ jsonrpc: "2.0", method: "notifications/whatever-chirho", params: {} });
    const list_chirho = (await client_chirho.request_chirho(2, "tools/list")).result as JsonObjectChirho;
    const tools_chirho = list_chirho.tools as JsonObjectChirho[];
    expect(tools_chirho.map(tool_chirho => tool_chirho.name).sort()).toEqual(["preview_page_chirho", "read_figures_chirho", "read_page_chirho", "save_figures_chirho", "save_page_chirho"]);
    for (const tool_chirho of tools_chirho) expect((tool_chirho.inputSchema as JsonObjectChirho).type).toBe("object");
    expect(list_chirho.resultType).toBeUndefined();
    expect(client_chirho.stdout_lines_chirho.length).toBe(2); // two requests, two answers: notifications got none
    expect(await client_chirho.close_chirho()).toBe(0); // it ends by itself when stdin ends

    const other_chirho = adapter_chirho();
    expect((await legacy_session_chirho(other_chirho, "2099-01-01")).protocolVersion).toBe("2025-11-25");
  });

  test("modern: discovery, per-request metadata, and the version and metadata refusals", async () => {
    scenario_chirho = "normal";
    const client_chirho = adapter_chirho();
    const discover_chirho = (await client_chirho.request_chirho(1, "server/discover", modern_params_chirho())).result as JsonObjectChirho;
    expect(discover_chirho.resultType).toBe("complete");
    expect(discover_chirho.supportedVersions).toContain("2026-07-28");
    expect(((discover_chirho._meta as JsonObjectChirho)["io.modelcontextprotocol/serverInfo"] as JsonObjectChirho).name).toBe("perffection-mcp-chirho");
    const list_chirho = (await client_chirho.request_chirho(2, "tools/list", modern_params_chirho())).result as JsonObjectChirho;
    expect(list_chirho.resultType).toBe("complete");
    expect((list_chirho.tools as unknown[]).length).toBe(5);
    const read_chirho = (await client_chirho.request_chirho(3, "tools/call", modern_params_chirho({ name: "read_page_chirho" }))).result as JsonObjectChirho;
    expect(read_chirho.resultType).toBe("complete");
    expect(structured_chirho(read_chirho)).toEqual(PAGE_ANSWER_CHIRHO);

    const old_chirho = modern_params_chirho();
    (old_chirho._meta as JsonObjectChirho)["io.modelcontextprotocol/protocolVersion"] = "1900-01-01";
    const refused_chirho = (await client_chirho.request_chirho(4, "tools/list", old_chirho)).error as JsonObjectChirho;
    expect(refused_chirho.code).toBe(-32022);
    expect((refused_chirho.data as JsonObjectChirho).requested).toBe("1900-01-01");
    expect((refused_chirho.data as JsonObjectChirho).supported).toContain("2026-07-28");
    const bare_chirho = modern_params_chirho();
    delete (bare_chirho._meta as JsonObjectChirho)["io.modelcontextprotocol/clientCapabilities"];
    expect(((await client_chirho.request_chirho(5, "tools/list", bare_chirho)).error as JsonObjectChirho).code).toBe(-32602);
    expect(((await client_chirho.request_chirho(6, "tools/list")).error as JsonObjectChirho).code).toBe(-32602); // neither era
  });

  test("stdout carries only JSON-RPC answers; unreadable input is answered and never stops it", async () => {
    const client_chirho = adapter_chirho();
    await legacy_session_chirho(client_chirho);
    client_chirho.send_raw_chirho("this is not json\n");
    client_chirho.send_raw_chirho('[{"jsonrpc":"2.0","id":1,"method":"ping"}]\n');
    client_chirho.send_raw_chirho(`${JSON.stringify({ jsonrpc: "2.0", id: "x", method: "p" }).slice(0, -1)}, "params": ${"[".repeat(3)}\n`);
    client_chirho.send_raw_chirho(`{"jsonrpc":"2.0","id":9,"method":"ping","params":"${"y".repeat(300_000)}"}\n`);
    const null_answers_chirho = await client_chirho.null_id_answers_after_chirho(4);
    expect(null_answers_chirho.map(answer_chirho => (answer_chirho.error as JsonObjectChirho).code)).toEqual([-32700, -32600, -32700, -32600]);
    expect(((await client_chirho.request_chirho(10, "no/such-method-chirho")).error as JsonObjectChirho).code).toBe(-32601);
    expect(((await client_chirho.request_chirho(11, "tools/call", { name: "publish_page_chirho" })).error as JsonObjectChirho).code).toBe(-32602);
    expect(((await client_chirho.request_chirho(12, "ping", { protocolVersion: 1 })).result as JsonObjectChirho)).toEqual({});
    client_chirho.send_raw_chirho('{"jsonrpc":"1.0","id":13,"method":"ping"}\n');
    expect(((await client_chirho.answer_for_chirho(13)).error as JsonObjectChirho).code).toBe(-32600);
    for (const line_chirho of client_chirho.stdout_lines_chirho) {
      const message_chirho = JSON.parse(line_chirho) as JsonObjectChirho;
      expect(message_chirho.jsonrpc).toBe("2.0");
      expect(("result" in message_chirho) !== ("error" in message_chirho)).toBe(true);
    }
  });
});

describe("the page tools and the figures read reach only their own routes", () => {
  test("read, save and preview: one fixed route each, the key only as a bearer header, the answer passed through whole", async () => {
    scenario_chirho = "normal";
    const before_chirho = recorded_chirho.length;
    const client_chirho = adapter_chirho();
    await legacy_session_chirho(client_chirho);
    const read_chirho = await call_chirho(client_chirho, 1, "read_page_chirho");
    expect(read_chirho.isError).toBe(false);
    expect(structured_chirho(read_chirho)).toEqual(PAGE_ANSWER_CHIRHO);
    const save_chirho = await call_chirho(client_chirho, 2, "save_page_chirho", SAVE_ARGUMENTS_CHIRHO);
    expect(save_chirho.isError).toBe(false);
    expect(structured_chirho(save_chirho).saved_chirho).toBe(true);
    const preview_chirho = await call_chirho(client_chirho, 3, "preview_page_chirho");
    expect((preview_chirho.content as JsonObjectChirho[])[0].text).toBe(PREVIEW_HTML_CHIRHO);
    const mine_chirho = recorded_chirho.slice(before_chirho);
    expect(mine_chirho.map(request_chirho => `${request_chirho.method_chirho} ${request_chirho.path_chirho}`)).toEqual([
      `GET ${READ_ROUTE_CHIRHO}`, `POST ${SAVE_ROUTE_CHIRHO}`, `GET ${PREVIEW_ROUTE_CHIRHO}`,
    ]);
    for (const request_chirho of mine_chirho) expect(request_chirho.authorization_chirho).toBe(`Bearer ${KEY_CHIRHO}`);
    expect(mine_chirho[1].content_type_chirho).toBe("application/json");
    expect(JSON.parse(mine_chirho[1].body_chirho)).toEqual(SAVE_ARGUMENTS_CHIRHO); // the arguments ARE the body
  });

  test("a read, figures read or preview given arguments sends nothing", async () => {
    scenario_chirho = "normal";
    const before_chirho = recorded_chirho.length;
    const client_chirho = adapter_chirho();
    await legacy_session_chirho(client_chirho);
    let id_chirho = 1;
    for (const name_chirho of ["read_page_chirho", "read_figures_chirho", "preview_page_chirho"]) {
      const read_chirho = await call_chirho(client_chirho, id_chirho++, name_chirho, { path_chirho: "/app-chirho/perffection-workspace/publish-chirho" });
      expect(read_chirho.isError).toBe(true);
      expect(structured_chirho(read_chirho).adapter_outcome_chirho).toBe("bad-arguments-chirho");
    }
    expect(recorded_chirho.length).toBe(before_chirho);
  });

  test("an answer that refuses comes through whole, flagged as an error", async () => {
    const client_chirho = adapter_chirho();
    await legacy_session_chirho(client_chirho);
    scenario_chirho = "refuse";
    const refused_chirho = await call_chirho(client_chirho, 1, "save_page_chirho", SAVE_ARGUMENTS_CHIRHO);
    expect(refused_chirho.isError).toBe(true);
    expect(structured_chirho(refused_chirho)).toEqual(REFUSED_ANSWER_CHIRHO);
    scenario_chirho = "stale";
    const stale_chirho = await call_chirho(client_chirho, 2, "save_page_chirho", SAVE_ARGUMENTS_CHIRHO);
    expect(stale_chirho.isError).toBe(true);
    expect(structured_chirho(stale_chirho)).toEqual(STALE_ANSWER_CHIRHO);
  });
});

describe("her figures: one read-only tool that never passes an incomplete report off as complete", () => {
  test("one fixed GET, the key only as a bearer header, no body, the answer passed through whole; read-only and argument-free", async () => {
    scenario_chirho = "normal";
    figures_reply_chirho = figures_answer_chirho(true);
    const before_chirho = recorded_chirho.length;
    const client_chirho = adapter_chirho();
    await legacy_session_chirho(client_chirho);
    const read_chirho = await call_chirho(client_chirho, 1, "read_figures_chirho");
    expect(read_chirho.isError).toBe(false);
    expect(structured_chirho(read_chirho)).toEqual(figures_answer_chirho(true));
    const mine_chirho = recorded_chirho.slice(before_chirho);
    expect(mine_chirho.map(request_chirho => `${request_chirho.method_chirho} ${request_chirho.path_chirho} ${request_chirho.body_chirho}`)).toEqual([`GET ${FIGURES_ROUTE_CHIRHO} `]);
    expect(mine_chirho[0].authorization_chirho).toBe(`Bearer ${KEY_CHIRHO}`);
    const list_chirho = (await client_chirho.request_chirho(2, "tools/list")).result as JsonObjectChirho;
    const tool_chirho = (list_chirho.tools as JsonObjectChirho[]).find(entry_chirho => entry_chirho.name === "read_figures_chirho") as JsonObjectChirho;
    const annotations_chirho = tool_chirho.annotations as JsonObjectChirho;
    expect([annotations_chirho.readOnlyHint, annotations_chirho.destructiveHint]).toEqual([true, false]);
    expect((tool_chirho.inputSchema as JsonObjectChirho).additionalProperties).toBe(false);
  });

  test("only complete_chirho === true is told as complete: false, missing or not a boolean is told as incomplete", async () => {
    scenario_chirho = "normal";
    const client_chirho = adapter_chirho();
    await legacy_session_chirho(client_chirho);
    let id_chirho = 1;
    const lead_chirho = async (reply_chirho: unknown) => {
      figures_reply_chirho = reply_chirho;
      const result_chirho = await call_chirho(client_chirho, id_chirho++, "read_figures_chirho");
      expect(result_chirho.isError).toBe(false);
      expect(structured_chirho(result_chirho)).toEqual(reply_chirho as JsonObjectChirho); // passed through, never edited
      return String((result_chirho.content as JsonObjectChirho[])[0].text).split("\n")[0];
    };
    const complete_chirho = await lead_chirho(figures_answer_chirho(true));
    const incomplete_chirho = await lead_chirho(figures_answer_chirho(false));
    expect(incomplete_chirho).not.toBe(complete_chirho);
    expect(incomplete_chirho).toContain("complete_chirho"); // it sends her agent to the flag itself
    for (const unproven_chirho of [undefined, "true", 1, null]) expect(await lead_chirho(figures_answer_chirho(unproven_chirho))).toBe(incomplete_chirho);
    // nothing saved and unverifiable read as themselves, never as figures, complete or not
    const nothing_chirho = await lead_chirho(figures_answer_chirho(undefined, "nothing-saved-chirho"));
    const unverifiable_chirho = await lead_chirho(figures_answer_chirho(undefined, "unverifiable-chirho"));
    expect(new Set([complete_chirho, incomplete_chirho, nothing_chirho, unverifiable_chirho]).size).toBe(4);
    figures_reply_chirho = figures_answer_chirho(true);
  });

  test("a nothing-saved read points at a first-save example only when the answer carries one", async () => {
    scenario_chirho = "normal";
    const client_chirho = adapter_chirho();
    await legacy_session_chirho(client_chirho);
    const older_answer_chirho = figures_answer_chirho(undefined, "nothing-saved-chirho"); // a server older than F6d: no start_chirho
    const offered_answer_chirho = { ...older_answer_chirho, start_chirho: { first_save_expected_version_chirho: 0, sample_figures_chirho: { schema_chirho: "perffection-figures-v1-chirho" } } };
    const partial_answer_chirho = { ...older_answer_chirho, start_chirho: { first_save_expected_version_chirho: 0 } };
    let id_chirho = 1;
    const lead_chirho = async (reply_chirho: JsonObjectChirho) => {
      figures_reply_chirho = reply_chirho;
      const result_chirho = await call_chirho(client_chirho, id_chirho++, "read_figures_chirho");
      expect(result_chirho.isError).toBe(false); // a valid read either way
      expect(structured_chirho(result_chirho)).toEqual(reply_chirho); // passed through whole, nothing invented
      return String((result_chirho.content as JsonObjectChirho[])[0].text).split("\n")[0];
    };
    const offered_chirho = await lead_chirho(offered_answer_chirho);
    const older_chirho = await lead_chirho(older_answer_chirho);
    const partial_chirho = await lead_chirho(partial_answer_chirho);
    // the lead names the example's field only when the answer has it
    expect([offered_chirho.includes("start_chirho"), older_chirho.includes("start_chirho"), partial_chirho.includes("start_chirho")]).toEqual([true, false, false]);
    expect([older_chirho === partial_chirho, older_chirho === offered_chirho]).toEqual([true, false]);
    figures_reply_chirho = figures_answer_chirho(true);
  });

  test("an answer that is not Perffection figures, a page answer included, is no answer", async () => {
    scenario_chirho = "normal";
    const client_chirho = adapter_chirho();
    await legacy_session_chirho(client_chirho);
    figures_reply_chirho = PAGE_ANSWER_CHIRHO;
    const read_chirho = await call_chirho(client_chirho, 1, "read_figures_chirho");
    expect(read_chirho.isError).toBe(true);
    expect([structured_chirho(read_chirho).adapter_outcome_chirho, structured_chirho(read_chirho).http_status_chirho]).toEqual(["no-answer-chirho", 200]);
    figures_reply_chirho = figures_answer_chirho(true);
  });
});

describe("a save without a complete answer is UNKNOWN: never saved, never nothing-stored", () => {
  const unknown_expected_chirho = (result_chirho: JsonObjectChirho, status_chirho: number | null) => {
    expect(result_chirho.isError).toBe(true);
    expect(structured_chirho(result_chirho).save_result_chirho).toBe("unknown-chirho");
    expect(structured_chirho(result_chirho).http_status_chirho).toBe(status_chirho);
    expect("saved_chirho" in structured_chirho(result_chirho)).toBe(false);
  };

  test("every status but a complete 200 answer, a redirect included, and a 200 that is not a save answer", async () => {
    const client_chirho = adapter_chirho();
    await legacy_session_chirho(client_chirho);
    let id_chirho = 1;
    for (const status_chirho of [400, 401, 403, 404, 413, 429, 500, 502, 302]) {
      scenario_chirho = `status-${status_chirho}`;
      unknown_expected_chirho(await call_chirho(client_chirho, id_chirho++, "save_page_chirho", SAVE_ARGUMENTS_CHIRHO), status_chirho);
      const read_chirho = await call_chirho(client_chirho, id_chirho++, "read_page_chirho");
      expect(read_chirho.isError).toBe(true);
      expect(structured_chirho(read_chirho).adapter_outcome_chirho).toBe("no-answer-chirho");
      expect("save_result_chirho" in structured_chirho(read_chirho)).toBe(false);
    }
    for (const scenario_name_chirho of ["not-perffection", "wrong-type", "huge"]) {
      scenario_chirho = scenario_name_chirho;
      unknown_expected_chirho(await call_chirho(client_chirho, id_chirho++, "save_page_chirho", SAVE_ARGUMENTS_CHIRHO), 200);
    }
  });

  test("the connection drops after the request, an answer is cut off halfway, an answer never comes", async () => {
    const dropped_chirho = adapter_chirho({ PERFFECTION_BASE_URL_CHIRHO: `http://127.0.0.1:${port_of_chirho(drop_server_chirho)}` });
    await legacy_session_chirho(dropped_chirho);
    const before_chirho = recorded_chirho.length;
    unknown_expected_chirho(await call_chirho(dropped_chirho, 1, "save_page_chirho", SAVE_ARGUMENTS_CHIRHO), null);
    expect(recorded_chirho.slice(before_chirho).some(request_chirho => request_chirho.path_chirho === SAVE_ROUTE_CHIRHO)).toBe(true); // it DID arrive

    const cut_chirho = adapter_chirho({ PERFFECTION_BASE_URL_CHIRHO: `http://127.0.0.1:${port_of_chirho(cut_server_chirho)}` });
    await legacy_session_chirho(cut_chirho);
    unknown_expected_chirho(await call_chirho(cut_chirho, 1, "save_page_chirho", SAVE_ARGUMENTS_CHIRHO), 200);

    const hung_chirho = adapter_chirho({ PERFFECTION_BASE_URL_CHIRHO: `http://127.0.0.1:${port_of_chirho(hang_server_chirho)}`, PERFFECTION_TIMEOUT_MS_CHIRHO: "300" });
    await legacy_session_chirho(hung_chirho);
    const started_chirho = Date.now();
    unknown_expected_chirho(await call_chirho(hung_chirho, 1, "save_page_chirho", SAVE_ARGUMENTS_CHIRHO), null);
    expect(Date.now() - started_chirho).toBeLessThan(5_000);
  });

  test("a redirect is never followed", async () => {
    scenario_chirho = "redirect";
    const hits_before_chirho = trap_hits_chirho.length;
    const client_chirho = adapter_chirho();
    await legacy_session_chirho(client_chirho);
    unknown_expected_chirho(await call_chirho(client_chirho, 1, "save_page_chirho", SAVE_ARGUMENTS_CHIRHO), 302);
    expect(trap_hits_chirho.length).toBe(hits_before_chirho);
  });
});

describe("the key", () => {
  test("is refused before anything is sent: plain http off this machine, an address with a path, a malformed key, no key", async () => {
    const cases_chirho: Record<string, string>[] = [
      // a host outside the adapter's loopback names that the kernel would still deliver here, should a regression connect (gpt #28596)
      { PERFFECTION_BASE_URL_CHIRHO: "http://[::ffff:127.0.0.1]:9" },
      { PERFFECTION_BASE_URL_CHIRHO: `http://127.0.0.1:${fake_chirho.port}/app-chirho/perffection-workspace` },
      { PERFFECTION_KEY_CHIRHO: `${KEY_CHIRHO} extra` },
      { PERFFECTION_KEY_CHIRHO: "" },
    ];
    const before_chirho = recorded_chirho.length;
    for (const env_chirho of cases_chirho) {
      const client_chirho = adapter_chirho(env_chirho);
      await legacy_session_chirho(client_chirho);
      const started_chirho = Date.now();
      const save_chirho = await call_chirho(client_chirho, 1, "save_page_chirho", SAVE_ARGUMENTS_CHIRHO);
      expect(Date.now() - started_chirho).toBeLessThan(1_000); // no connection was even tried
      expect(structured_chirho(save_chirho).adapter_outcome_chirho).toBe("not-configured-chirho");
      expect(structured_chirho(save_chirho).save_result_chirho).toBe("not-sent-chirho");
    }
    expect(recorded_chirho.length).toBe(before_chirho);
  });

  test("https off this machine is allowed (the call is attempted), and a key file is read", async () => {
    // Hermetic (gpt #28596): [::ffff:127.0.0.1] is a host outside the adapter's loopback names, so the adapter
    // treats it exactly as a remote one (plain http to it is refused, below), while the kernel delivers it to a
    // listener on 127.0.0.1, so nothing leaves this machine. The listener keeps each connection's first byte.
    const first_bytes_chirho: number[] = [];
    const tls_trap_chirho = createServer(socket_chirho => {
      socket_chirho.once("data", data_chirho => {
        first_bytes_chirho.push(typeof data_chirho === "string" ? data_chirho.charCodeAt(0) : (data_chirho[0] ?? -1));
        socket_chirho.destroy();
      });
      socket_chirho.on("error", () => undefined);
    });
    await new Promise<void>(resolve_chirho => tls_trap_chirho.listen(0, "127.0.0.1", () => resolve_chirho()));
    try {
      const host_chirho = `[::ffff:127.0.0.1]:${(tls_trap_chirho.address() as AddressInfo).port}`;
      const plain_chirho = adapter_chirho({ PERFFECTION_BASE_URL_CHIRHO: `http://${host_chirho}` });
      await legacy_session_chirho(plain_chirho);
      expect(structured_chirho(await call_chirho(plain_chirho, 1, "read_page_chirho")).adapter_outcome_chirho).toBe("not-configured-chirho");
      expect(first_bytes_chirho).toEqual([]); // plain http off this machine: refused before any connection
      const remote_chirho = adapter_chirho({ PERFFECTION_BASE_URL_CHIRHO: `https://${host_chirho}`, PERFFECTION_TIMEOUT_MS_CHIRHO: "300" });
      await legacy_session_chirho(remote_chirho);
      expect(structured_chirho(await call_chirho(remote_chirho, 1, "read_page_chirho")).adapter_outcome_chirho).toBe("no-answer-chirho");
      expect(first_bytes_chirho).toEqual([0x16]); // https: one connection, opened by a TLS handshake record
    } finally {
      tls_trap_chirho.close();
    }

    scenario_chirho = "normal";
    const key_file_chirho = join(key_dir_chirho, "key-chirho.txt");
    writeFileSync(key_file_chirho, `${FILE_KEY_CHIRHO}\n`);
    chmodSync(key_file_chirho, 0o600);
    const before_chirho = recorded_chirho.length;
    const from_file_chirho = adapter_chirho({ PERFFECTION_KEY_CHIRHO: "", PERFFECTION_KEY_FILE_CHIRHO: key_file_chirho });
    await legacy_session_chirho(from_file_chirho);
    expect((await call_chirho(from_file_chirho, 1, "read_page_chirho")).isError).toBe(false);
    expect(recorded_chirho[before_chirho].authorization_chirho).toBe(`Bearer ${FILE_KEY_CHIRHO}`);
  });

  test("a key file other users of the machine can open is refused, like ssh, and nothing is sent", async () => {
    scenario_chirho = "normal";
    const before_chirho = recorded_chirho.length;
    for (const mode_chirho of [0o644, 0o640, 0o620, 0o604]) {
      const loose_file_chirho = join(key_dir_chirho, `loose-${mode_chirho.toString(8)}-chirho.txt`);
      writeFileSync(loose_file_chirho, `${FILE_KEY_CHIRHO}\n`);
      chmodSync(loose_file_chirho, mode_chirho);
      const client_chirho = adapter_chirho({ PERFFECTION_KEY_CHIRHO: "", PERFFECTION_KEY_FILE_CHIRHO: loose_file_chirho });
      await legacy_session_chirho(client_chirho);
      const save_chirho = await call_chirho(client_chirho, 1, "save_page_chirho", SAVE_ARGUMENTS_CHIRHO);
      expect(structured_chirho(save_chirho).adapter_outcome_chirho).toBe("not-configured-chirho");
      expect(structured_chirho(save_chirho).save_result_chirho).toBe("not-sent-chirho");
    }
    expect(recorded_chirho.length).toBe(before_chirho);
  });

  test("an answer that echoes the key comes out with the key scrubbed", async () => {
    scenario_chirho = "echo-key";
    const before_chirho = recorded_chirho.length;
    const client_chirho = adapter_chirho();
    await legacy_session_chirho(client_chirho);
    const read_chirho = await call_chirho(client_chirho, 1, "read_page_chirho");
    const save_chirho = await call_chirho(client_chirho, 2, "save_page_chirho", SAVE_ARGUMENTS_CHIRHO);
    expect(recorded_chirho[before_chirho].authorization_chirho).toBe(`Bearer ${KEY_CHIRHO}`); // control: the key did reach the server
    expect(structured_chirho(read_chirho).echoed_chirho).toBe("Bearer [redacted-key]");
    expect(structured_chirho(save_chirho).echoed_chirho).toBe("Bearer [redacted-key]");
    expect(client_chirho.stdout_lines_chirho.join("\n").includes(KEY_CHIRHO)).toBe(false);
  });
});

describe("bounds and runtimes", () => {
  test("tool calls run one at a time in arrival order; beyond sixteen waiting, calls are refused at once", async () => {
    scenario_chirho = "normal";
    slow_ms_chirho = 150;
    most_in_flight_chirho = 0;
    const client_chirho = adapter_chirho();
    await legacy_session_chirho(client_chirho);
    const lines_chirho = Array.from({ length: 20 }, (_unused_chirho, index_chirho) =>
      JSON.stringify({ jsonrpc: "2.0", id: 100 + index_chirho, method: "tools/call", params: { name: "read_page_chirho" } }));
    client_chirho.send_raw_chirho(`${lines_chirho.join("\n")}\n`);
    const answers_chirho = await Promise.all(lines_chirho.map((_line_chirho, index_chirho) => client_chirho.answer_for_chirho(100 + index_chirho, 15_000)));
    slow_ms_chirho = 0;
    expect(answers_chirho.filter(answer_chirho => (answer_chirho.error as JsonObjectChirho | undefined)?.code === -32000).length).toBe(4);
    expect(answers_chirho.filter(answer_chirho => (answer_chirho.result as JsonObjectChirho | undefined)?.isError === false).length).toBe(16);
    expect(most_in_flight_chirho).toBe(1);
  });

  test("Node runs the same file with the same answers", async () => {
    const node_chirho = Bun.which("node");
    expect(node_chirho).toBeTruthy();
    scenario_chirho = "normal";
    const client_chirho = adapter_chirho({}, node_chirho as string);
    expect((await legacy_session_chirho(client_chirho)).protocolVersion).toBe("2025-06-18");
    expect(structured_chirho(await call_chirho(client_chirho, 1, "read_page_chirho"))).toEqual(PAGE_ANSWER_CHIRHO);
    expect(structured_chirho(await call_chirho(client_chirho, 2, "save_page_chirho", SAVE_ARGUMENTS_CHIRHO)).saved_chirho).toBe(true);
    figures_reply_chirho = figures_answer_chirho(true);
    expect(structured_chirho(await call_chirho(client_chirho, 4, "read_figures_chirho"))).toEqual(figures_answer_chirho(true));
    scenario_chirho = "status-401";
    expect(structured_chirho(await call_chirho(client_chirho, 3, "save_page_chirho", SAVE_ARGUMENTS_CHIRHO)).save_result_chirho).toBe("unknown-chirho");
  });

  test("across every adapter run in this file, neither key ever appeared on stdout or stderr", async () => {
    await Promise.all(spawned_chirho.map(client_chirho => client_chirho.close_chirho()));
    const written_chirho = spawned_chirho.map(client_chirho => `${client_chirho.stdout_lines_chirho.join("\n")}\n${client_chirho.stderr_chirho}`).join("\n");
    expect(`${written_chirho} Bearer ${KEY_CHIRHO}`.includes(KEY_CHIRHO)).toBe(true); // control: the scan finds a planted key
    expect(spawned_chirho.length).toBeGreaterThan(15);
    expect(written_chirho.includes(KEY_CHIRHO)).toBe(false);
    expect(written_chirho.includes(FILE_KEY_CHIRHO)).toBe(false);
  });
});

describe("a 404 says only what its evidence shows, and the save's history promise is honest", () => {
  test("a live address without the tool's route: route absent, asked without the key; a save stays unknown", async () => {
    const client_chirho = adapter_chirho();
    await legacy_session_chirho(client_chirho);
    scenario_chirho = "unsupported";
    const before_chirho = recorded_chirho.length;
    const read_chirho = structured_chirho(await call_chirho(client_chirho, 1, "read_page_chirho"));
    expect([read_chirho.http_status_chirho, read_chirho.reason_chirho]).toEqual([404, "tool-route-absent-chirho"]);
    const save_chirho = structured_chirho(await call_chirho(client_chirho, 2, "save_page_chirho", SAVE_ARGUMENTS_CHIRHO));
    expect([save_chirho.reason_chirho, save_chirho.save_result_chirho]).toEqual(["tool-route-absent-chirho", "unknown-chirho"]);
    // a released server today: the figures route is held, so the figures tool says it is absent there
    const figures_chirho = structured_chirho(await call_chirho(client_chirho, 3, "read_figures_chirho"));
    expect([figures_chirho.http_status_chirho, figures_chirho.reason_chirho, "save_result_chirho" in figures_chirho]).toEqual([404, "tool-route-absent-chirho", false]);
    const health_chirho = recorded_chirho.slice(before_chirho).filter(request_chirho => request_chirho.path_chirho === "/healthz-chirho");
    expect(health_chirho.length).toBe(3);
    expect(health_chirho.every(request_chirho => request_chirho.method_chirho === "GET" && request_chirho.authorization_chirho === "" && request_chirho.body_chirho === "")).toBe(true);
    scenario_chirho = "normal";
  });

  test("a healthy service that is not Perffection gets the same unproven label, never a claim it is Perffection", async () => {
    const client_chirho = adapter_chirho();
    await legacy_session_chirho(client_chirho);
    scenario_chirho = "foreign-healthy";
    expect(structured_chirho(await call_chirho(client_chirho, 1, "read_page_chirho")).reason_chirho).toBe("tool-route-absent-chirho");
    scenario_chirho = "normal";
  });

  test("when the health check fails, errs or never answers, the address is unconfirmed: wrong or down", async () => {
    const client_chirho = adapter_chirho();
    await legacy_session_chirho(client_chirho);
    let id_chirho = 1;
    for (const scenario_name_chirho of ["status-404", "health-500", "health-hangs"]) {
      scenario_chirho = scenario_name_chirho;
      const read_chirho = structured_chirho(await call_chirho(client_chirho, id_chirho++, "read_page_chirho"));
      expect([scenario_name_chirho, read_chirho.http_status_chirho, read_chirho.reason_chirho]).toEqual([scenario_name_chirho, 404, "address-unconfirmed-chirho"]);
      const save_chirho = structured_chirho(await call_chirho(client_chirho, id_chirho++, "save_page_chirho", SAVE_ARGUMENTS_CHIRHO));
      expect(save_chirho.save_result_chirho).toBe("unknown-chirho");
    }
    scenario_chirho = "normal";
  });

  test("a save is marked destructive (a full history retires its oldest version) and idempotent (an exact repeat stores nothing)", async () => {
    const client_chirho = adapter_chirho();
    await legacy_session_chirho(client_chirho);
    const list_chirho = (await client_chirho.request_chirho(2, "tools/list")).result as JsonObjectChirho;
    const save_chirho = (list_chirho.tools as JsonObjectChirho[]).find(tool_chirho => tool_chirho.name === "save_page_chirho") as JsonObjectChirho;
    const annotations_chirho = save_chirho.annotations as JsonObjectChirho;
    expect([annotations_chirho.readOnlyHint, annotations_chirho.destructiveHint, annotations_chirho.idempotentHint]).toEqual([false, true, true]);
  });
});
