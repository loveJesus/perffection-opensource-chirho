// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// The agent API gate's MCP arms (agent-api-rehearsal-chirho.sh, arm 8b): the owner's own agent
// reaches the REAL workspace routes through tools-chirho/agent-mcp-chirho over stdio, with the
// gate's operator-issued keys on the gate's own root. It writes a summary of STRUCTURE (booleans,
// versions, codes, never a sentence) for the gate to assert, and every adapter's full stdout and
// stderr for the gate's fail-closed key scan.
// Usage: bun agent-mcp-journey-chirho.ts <base url> <key file A> <key file B> <made-up key file> <out dir> <marker> <refused marker>
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { McpStdioClientChirho, modern_params_chirho, type JsonObjectChirho } from "../../tools-chirho/agent-mcp-chirho/mcp-stdio-client-chirho";

const [base_chirho, key_a_chirho, key_b_chirho, key_made_up_chirho, out_chirho, marker_chirho, refused_marker_chirho] = process.argv.slice(2);
if (!refused_marker_chirho) throw new Error("usage: <base url> <key file A> <key file B> <made-up key file> <out dir> <marker> <refused marker>");
mkdirSync(out_chirho, { recursive: true, mode: 0o700 });
const summary_chirho: JsonObjectChirho = {};
const adapters_chirho: [string, McpStdioClientChirho][] = [];

function start_chirho(name_chirho: string, key_file_chirho: string): McpStdioClientChirho {
  const client_chirho = new McpStdioClientChirho({ PERFFECTION_BASE_URL_CHIRHO: base_chirho, PERFFECTION_KEY_FILE_CHIRHO: key_file_chirho });
  adapters_chirho.push([name_chirho, client_chirho]);
  return client_chirho;
}
async function modern_call_chirho(client_chirho: McpStdioClientChirho, id_chirho: number, name_chirho: string, arguments_chirho?: JsonObjectChirho): Promise<JsonObjectChirho> {
  const params_chirho: JsonObjectChirho = arguments_chirho === undefined ? { name: name_chirho } : { name: name_chirho, arguments: arguments_chirho };
  return (await client_chirho.request_chirho(id_chirho, "tools/call", modern_params_chirho(params_chirho), 30_000)).result as JsonObjectChirho;
}
const structured_chirho = (result_chirho: JsonObjectChirho) => (result_chirho.structuredContent ?? {}) as JsonObjectChirho;
const page_of_chirho = (answer_chirho: JsonObjectChirho) => (answer_chirho.page_chirho ?? {}) as JsonObjectChirho;

try {
  // (a) the modern era with A's key: discover, list, read, save, a stale save, read again, preview
  const a_chirho = start_chirho("modern-a", key_a_chirho);
  const discover_chirho = (await a_chirho.request_chirho(1, "server/discover", modern_params_chirho())).result as JsonObjectChirho;
  summary_chirho.discover_result_type_chirho = discover_chirho.resultType;
  summary_chirho.discover_offers_2026_chirho = (discover_chirho.supportedVersions as string[]).includes("2026-07-28");
  const list_chirho = (await a_chirho.request_chirho(2, "tools/list", modern_params_chirho())).result as JsonObjectChirho;
  summary_chirho.tools_chirho = (list_chirho.tools as JsonObjectChirho[]).map(tool_chirho => String(tool_chirho.name)).sort();
  const read_chirho = await modern_call_chirho(a_chirho, 3, "read_page_chirho");
  const read_answer_chirho = structured_chirho(read_chirho);
  const version_chirho = Number(read_answer_chirho.current_version_chirho);
  summary_chirho.read_chirho = { is_error_chirho: read_chirho.isError, state_chirho: read_answer_chirho.state_chirho, version_chirho };
  const routes_chirho = ((read_answer_chirho.surface_chirho as JsonObjectChirho | undefined)?.routes_chirho ?? []) as JsonObjectChirho[];
  summary_chirho.surface_routes_chirho = routes_chirho.map(route_chirho => `${route_chirho.method_chirho} ${route_chirho.path_chirho}`).sort();

  const page_chirho = { ...page_of_chirho(read_answer_chirho), business_name_chirho: `${marker_chirho} Press` };
  const save_chirho = await modern_call_chirho(a_chirho, 4, "save_page_chirho", { expected_version_chirho: version_chirho, page_chirho });
  const saved_chirho = structured_chirho(save_chirho);
  summary_chirho.save_chirho = {
    is_error_chirho: save_chirho.isError, saved_chirho: saved_chirho.saved_chirho, version_chirho: saved_chirho.current_version_chirho,
    name_kept_chirho: page_of_chirho(saved_chirho).business_name_chirho === `${marker_chirho} Press`,
  };
  const stale_chirho = await modern_call_chirho(a_chirho, 5, "save_page_chirho", {
    expected_version_chirho: version_chirho, page_chirho: { ...page_chirho, business_name_chirho: `${refused_marker_chirho} Overwrite` },
  });
  summary_chirho.stale_chirho = { is_error_chirho: stale_chirho.isError, refusal_chirho: structured_chirho(stale_chirho).refusal_chirho, head_chirho: structured_chirho(stale_chirho).current_version_chirho };
  const read_after_chirho = structured_chirho(await modern_call_chirho(a_chirho, 6, "read_page_chirho"));
  const preview_chirho = await modern_call_chirho(a_chirho, 7, "preview_page_chirho");
  const preview_html_chirho = String(((preview_chirho.content ?? []) as JsonObjectChirho[])[0]?.text ?? "");
  summary_chirho.after_chirho = {
    version_chirho: read_after_chirho.current_version_chirho,
    name_is_the_mcp_save_chirho: page_of_chirho(read_after_chirho).business_name_chirho === `${marker_chirho} Press`,
    preview_is_error_chirho: preview_chirho.isError,
    preview_shows_the_mcp_save_chirho: preview_html_chirho.includes(`${marker_chirho} Press`),
  };

  // (b) the legacy era with A's key, a separate adapter process
  const legacy_chirho = start_chirho("legacy-a", key_a_chirho);
  const init_chirho = (await legacy_chirho.request_chirho(1, "initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "gate-chirho", version: "0" } })).result as JsonObjectChirho;
  legacy_chirho.send_chirho({ jsonrpc: "2.0", method: "notifications/initialized" });
  const legacy_read_chirho = structured_chirho((await legacy_chirho.request_chirho(2, "tools/call", { name: "read_page_chirho" }, 30_000)).result as JsonObjectChirho);
  summary_chirho.legacy_chirho = { version_chirho: init_chirho.protocolVersion, read_version_chirho: legacy_read_chirho.current_version_chirho };

  // (c) B's key reads B's page; (d) a made-up key gets no answer, and a save is then UNKNOWN
  const b_chirho = start_chirho("modern-b", key_b_chirho);
  const b_read_chirho = await modern_call_chirho(b_chirho, 1, "read_page_chirho");
  summary_chirho.b_chirho = { is_error_chirho: b_read_chirho.isError, sees_a_chirho: JSON.stringify(b_read_chirho).includes(marker_chirho) };
  const made_up_chirho = start_chirho("made-up-key", key_made_up_chirho);
  const made_up_read_chirho = structured_chirho(await modern_call_chirho(made_up_chirho, 1, "read_page_chirho"));
  const made_up_save_chirho = structured_chirho(await modern_call_chirho(made_up_chirho, 2, "save_page_chirho", { expected_version_chirho: 0, page_chirho }));
  summary_chirho.made_up_chirho = {
    read_outcome_chirho: made_up_read_chirho.adapter_outcome_chirho, read_status_chirho: made_up_read_chirho.http_status_chirho,
    save_result_chirho: made_up_save_chirho.save_result_chirho,
  };
} finally {
  const exits_chirho: JsonObjectChirho = {};
  for (const [name_chirho, client_chirho] of adapters_chirho) {
    exits_chirho[name_chirho] = await client_chirho.close_chirho();
    writeFileSync(join(out_chirho, `mcp-${name_chirho}-chirho.txt`), `${client_chirho.stdout_lines_chirho.join("\n")}\n--- stderr ---\n${client_chirho.stderr_chirho}`, { mode: 0o600 });
  }
  summary_chirho.exits_chirho = exits_chirho;
  writeFileSync(join(out_chirho, "mcp-journey-summary-chirho.json"), JSON.stringify(summary_chirho, null, 1), { mode: 0o600 });
}
