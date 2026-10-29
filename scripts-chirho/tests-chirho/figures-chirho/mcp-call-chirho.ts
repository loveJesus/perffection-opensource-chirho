// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// One call of her own agent's tool, served (the K1 gate's MCP arms, key-grants-rehearsal-chirho.sh): the
// adapter in tools-chirho/agent-mcp-chirho runs over stdio with a key file, offers the tool, makes ONE
// tools/call, and is closed. Written to <out file> as JSON: whether the tool was offered, isError, the
// structured result exactly as the adapter passed it on, the first text it returned (a preview's page has no
// structured result), and a fail-closed scan of every line the adapter wrote for the key, with a planted control
// the scan must find.
// Usage: bun mcp-call-chirho.ts <base url> <key file> <tool> <arguments JSON file, or ""> <out file>
// Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/her-figures-chirho.md
import { readFileSync, writeFileSync } from "node:fs";
import { McpStdioClientChirho, modern_params_chirho, type JsonObjectChirho } from "../../../tools-chirho/agent-mcp-chirho/mcp-stdio-client-chirho";

const [base_chirho, key_file_chirho, tool_chirho, arguments_file_chirho, out_chirho] = process.argv.slice(2);
if (!out_chirho) throw new Error('usage: <base url> <key file> <tool> <arguments JSON file, or ""> <out file>');
const client_chirho = new McpStdioClientChirho({ PERFFECTION_BASE_URL_CHIRHO: base_chirho, PERFFECTION_KEY_FILE_CHIRHO: key_file_chirho });
const list_chirho = (await client_chirho.request_chirho(1, "tools/list", modern_params_chirho())).result as JsonObjectChirho;
const offered_chirho = (list_chirho.tools as JsonObjectChirho[]).some(tool_entry_chirho => tool_entry_chirho.name === tool_chirho);
const params_chirho: JsonObjectChirho = arguments_file_chirho
  ? { name: tool_chirho, arguments: JSON.parse(readFileSync(arguments_file_chirho, "utf8")) as JsonObjectChirho }
  : { name: tool_chirho };
const result_chirho = (await client_chirho.request_chirho(2, "tools/call", modern_params_chirho(params_chirho), 30_000)).result as JsonObjectChirho;
await client_chirho.close_chirho();
const written_chirho = `${client_chirho.stdout_lines_chirho.join("\n")}\n${client_chirho.stderr_chirho}`;
const key_chirho = readFileSync(key_file_chirho, "utf8").trim();
writeFileSync(out_chirho, `${JSON.stringify({
  offered_chirho,
  is_error_chirho: result_chirho.isError,
  structured_chirho: result_chirho.structuredContent ?? null,
  text_chirho: ((result_chirho.content as JsonObjectChirho[] | undefined) ?? []).find(entry_chirho => entry_chirho.type === "text")?.text ?? null,
  key_read_chirho: key_chirho.length > 0,
  key_written_chirho: written_chirho.includes(key_chirho),
  scan_finds_planted_key_chirho: `${written_chirho}\n${key_chirho}`.includes(key_chirho),
})}\n`);
