// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// The figures gate's assistant arm (../figures-rehearsal-chirho.sh, arm 8b): each owner's own agent
// reads her figures through tools-chirho/agent-mcp-chirho over stdio, on the gate's own root, with the
// gate's operator-issued keys. It writes STRUCTURE only (booleans, states, versions, never a sentence):
// A's answer exactly as the adapter passed it on, for the gate to compare with her REST read; B's state
// and version; and a fail-closed scan of every line both adapters wrote for either key, with a planted
// control the scan must find.
// Usage: bun figures-mcp-read-chirho.ts <base url> <key file A> <key file B> <out dir>
// Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/her-figures-chirho.md
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { McpStdioClientChirho, modern_params_chirho, type JsonObjectChirho } from "../../../tools-chirho/agent-mcp-chirho/mcp-stdio-client-chirho";

const [base_chirho, key_a_chirho, key_b_chirho, out_chirho] = process.argv.slice(2);
if (!out_chirho) throw new Error("usage: <base url> <key file A> <key file B> <out dir>");
mkdirSync(out_chirho, { recursive: true, mode: 0o700 });

type ReadChirho = { client_chirho: McpStdioClientChirho; tools_chirho: string[]; result_chirho: JsonObjectChirho };
async function read_figures_as_chirho(key_file_chirho: string): Promise<ReadChirho> {
  const client_chirho = new McpStdioClientChirho({ PERFFECTION_BASE_URL_CHIRHO: base_chirho, PERFFECTION_KEY_FILE_CHIRHO: key_file_chirho });
  const list_chirho = (await client_chirho.request_chirho(1, "tools/list", modern_params_chirho())).result as JsonObjectChirho;
  const tools_chirho = (list_chirho.tools as JsonObjectChirho[]).map(tool_chirho => String(tool_chirho.name));
  const result_chirho = (await client_chirho.request_chirho(2, "tools/call", modern_params_chirho({ name: "read_figures_chirho" }), 30_000)).result as JsonObjectChirho;
  await client_chirho.close_chirho();
  return { client_chirho, tools_chirho, result_chirho };
}
const answer_of_chirho = (read_chirho: ReadChirho) => (read_chirho.result_chirho.structuredContent ?? {}) as JsonObjectChirho;

const a_chirho = await read_figures_as_chirho(key_a_chirho);
const b_chirho = await read_figures_as_chirho(key_b_chirho);
writeFileSync(join(out_chirho, "mcp-a-figures-chirho.json"), JSON.stringify(answer_of_chirho(a_chirho)));
const written_chirho = [a_chirho, b_chirho]
  .map(read_chirho => `${read_chirho.client_chirho.stdout_lines_chirho.join("\n")}\n${read_chirho.client_chirho.stderr_chirho}`)
  .join("\n");
const keys_chirho = [key_a_chirho, key_b_chirho].map(file_chirho => readFileSync(file_chirho, "utf8").trim());
const b_answer_chirho = answer_of_chirho(b_chirho);
const summary_chirho = {
  offered_chirho: a_chirho.tools_chirho.includes("read_figures_chirho"),
  a_chirho: { is_error_chirho: a_chirho.result_chirho.isError, state_chirho: answer_of_chirho(a_chirho).state_chirho, version_chirho: answer_of_chirho(a_chirho).current_version_chirho },
  b_chirho: { is_error_chirho: b_chirho.result_chirho.isError, state_chirho: b_answer_chirho.state_chirho, version_chirho: b_answer_chirho.current_version_chirho, figures_absent_chirho: b_answer_chirho.figures_chirho === null },
  // the control: the same scan over the same text with A's key planted must find it
  scan_finds_planted_key_chirho: `${written_chirho}\n${keys_chirho[0]}`.includes(keys_chirho[0]),
  keys_written_chirho: keys_chirho.filter(key_chirho => key_chirho.length > 0 && written_chirho.includes(key_chirho)).length,
  keys_read_chirho: keys_chirho.filter(key_chirho => key_chirho.length > 0).length,
};
writeFileSync(join(out_chirho, "mcp-figures-summary-chirho.json"), `${JSON.stringify(summary_chirho, null, 2)}\n`);
