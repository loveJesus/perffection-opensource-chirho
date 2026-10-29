// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// The agent API gate's ended-connection arm (agent-api-rehearsal-chirho.sh, arm 10b). After the owner
// ends her key on the owner key roads, her agent's adapter still holds that key. It must get no page and
// no preview, and a save's result must be UNKNOWN, never a guessed "nothing stored". It writes a summary
// of STRUCTURE for the gate, and the adapter's full stdout and stderr for the gate's fail-closed key scan.
// Usage: bun agent-mcp-ended-key-chirho.ts <base url> <ended key file> <save body file> <out dir>
// Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/agent-api-chirho.md (her connection's life).
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { McpStdioClientChirho, modern_params_chirho, type JsonObjectChirho } from "../../tools-chirho/agent-mcp-chirho/mcp-stdio-client-chirho";

const [base_chirho, key_file_chirho, save_body_file_chirho, out_chirho] = process.argv.slice(2);
if (!out_chirho) throw new Error("usage: <base url> <ended key file> <save body file> <out dir>");
const save_body_chirho = JSON.parse(readFileSync(save_body_file_chirho, "utf8")) as JsonObjectChirho;
const client_chirho = new McpStdioClientChirho({ PERFFECTION_BASE_URL_CHIRHO: base_chirho, PERFFECTION_KEY_FILE_CHIRHO: key_file_chirho });
const summary_chirho: JsonObjectChirho = {};
async function call_chirho(id_chirho: number, name_chirho: string, arguments_chirho?: JsonObjectChirho): Promise<JsonObjectChirho> {
  const params_chirho: JsonObjectChirho = arguments_chirho === undefined ? { name: name_chirho } : { name: name_chirho, arguments: arguments_chirho };
  return (await client_chirho.request_chirho(id_chirho, "tools/call", modern_params_chirho(params_chirho), 30_000)).result as JsonObjectChirho;
}
const structured_chirho = (result_chirho: JsonObjectChirho) => (result_chirho.structuredContent ?? {}) as JsonObjectChirho;

try {
  const read_chirho = await call_chirho(1, "read_page_chirho");
  summary_chirho.read_chirho = { is_error_chirho: read_chirho.isError, outcome_chirho: structured_chirho(read_chirho).adapter_outcome_chirho,
    status_chirho: structured_chirho(read_chirho).http_status_chirho, has_page_chirho: "page_chirho" in structured_chirho(read_chirho) };
  const save_chirho = await call_chirho(2, "save_page_chirho", save_body_chirho);
  summary_chirho.save_chirho = { is_error_chirho: save_chirho.isError, save_result_chirho: structured_chirho(save_chirho).save_result_chirho };
  const preview_chirho = await call_chirho(3, "preview_page_chirho");
  summary_chirho.preview_chirho = { is_error_chirho: preview_chirho.isError, outcome_chirho: structured_chirho(preview_chirho).adapter_outcome_chirho,
    status_chirho: structured_chirho(preview_chirho).http_status_chirho };
} finally {
  summary_chirho.exit_chirho = await client_chirho.close_chirho();
  writeFileSync(join(out_chirho, "mcp-ended-key-chirho.txt"), `${client_chirho.stdout_lines_chirho.join("\n")}\n--- stderr ---\n${client_chirho.stderr_chirho}`, { mode: 0o600 });
  writeFileSync(join(out_chirho, "mcp-ended-key-summary-chirho.json"), JSON.stringify(summary_chirho, null, 1), { mode: 0o600 });
}
