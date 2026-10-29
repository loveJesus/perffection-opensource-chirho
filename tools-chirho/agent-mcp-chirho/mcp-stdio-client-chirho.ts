// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// A minimal MCP client over stdio, for checking perffection-mcp-chirho.ts: it starts the adapter
// as a child process, writes JSON-RPC lines to it and waits for the answer to each request id.
// It keeps EVERY stdout line and all of stderr, so a check can prove what the adapter wrote (and
// what it never wrote). Test support only; Bun runs it. Nothing here is imported by the adapter.
import type { Subprocess } from "bun";

export type JsonObjectChirho = { [key_chirho: string]: unknown };
export const ADAPTER_PATH_CHIRHO = new URL("./perffection-mcp-chirho.ts", import.meta.url).pathname;
export const MODERN_VERSION_CHIRHO = "2026-07-28";

/** Params for a modern (2026-07-28) request: every request carries its own version and capabilities. */
export function modern_params_chirho(params_chirho: JsonObjectChirho = {}): JsonObjectChirho {
  return {
    ...params_chirho,
    _meta: {
      "io.modelcontextprotocol/protocolVersion": MODERN_VERSION_CHIRHO,
      "io.modelcontextprotocol/clientInfo": { name: "mcp-stdio-client-chirho", version: "0.1.0" },
      "io.modelcontextprotocol/clientCapabilities": {},
    },
  };
}

export class McpStdioClientChirho {
  readonly stdout_lines_chirho: string[] = [];
  stderr_chirho = "";
  private readonly child_chirho: Subprocess<"pipe", "pipe", "pipe">;
  private readonly answered_chirho = new Map<string, JsonObjectChirho>();
  private readonly null_id_answers_chirho: JsonObjectChirho[] = [];
  private readonly reading_chirho: Promise<void>;

  /** `runtime_chirho` is the program that runs the adapter file: bun (default) or node. */
  constructor(env_chirho: Record<string, string>, runtime_chirho: string = process.execPath) {
    this.child_chirho = Bun.spawn([runtime_chirho, ADAPTER_PATH_CHIRHO], { stdin: "pipe", stdout: "pipe", stderr: "pipe", env: env_chirho });
    this.reading_chirho = Promise.all([this.read_stdout_chirho(), this.read_stderr_chirho()]).then(() => undefined);
  }

  private async read_stdout_chirho(): Promise<void> {
    const decoder_chirho = new TextDecoder();
    let pending_chirho = "";
    for await (const chunk_chirho of this.child_chirho.stdout) {
      pending_chirho += decoder_chirho.decode(chunk_chirho, { stream: true });
      let newline_chirho = pending_chirho.indexOf("\n");
      while (newline_chirho !== -1) {
        const line_chirho = pending_chirho.slice(0, newline_chirho);
        pending_chirho = pending_chirho.slice(newline_chirho + 1);
        this.stdout_lines_chirho.push(line_chirho);
        try {
          const message_chirho = JSON.parse(line_chirho) as JsonObjectChirho;
          if (message_chirho.id === null) this.null_id_answers_chirho.push(message_chirho);
          else this.answered_chirho.set(JSON.stringify(message_chirho.id), message_chirho);
        } catch {
          // kept in stdout_lines_chirho: a check that every line is JSON-RPC will see it
        }
        newline_chirho = pending_chirho.indexOf("\n");
      }
    }
    if (pending_chirho) this.stdout_lines_chirho.push(pending_chirho);
  }

  private async read_stderr_chirho(): Promise<void> {
    const decoder_chirho = new TextDecoder();
    for await (const chunk_chirho of this.child_chirho.stderr) this.stderr_chirho += decoder_chirho.decode(chunk_chirho, { stream: true });
  }

  send_raw_chirho(text_chirho: string): void {
    this.child_chirho.stdin.write(text_chirho);
    this.child_chirho.stdin.flush();
  }

  send_chirho(message_chirho: JsonObjectChirho): void {
    this.send_raw_chirho(`${JSON.stringify(message_chirho)}\n`);
  }

  /** Sends a request and waits for the answer carrying its id. */
  async request_chirho(id_chirho: string | number, method_chirho: string, params_chirho?: JsonObjectChirho, timeout_ms_chirho = 10_000): Promise<JsonObjectChirho> {
    const message_chirho: JsonObjectChirho = { jsonrpc: "2.0", id: id_chirho, method: method_chirho };
    if (params_chirho !== undefined) message_chirho.params = params_chirho;
    this.send_chirho(message_chirho);
    return this.answer_for_chirho(id_chirho, timeout_ms_chirho);
  }

  async answer_for_chirho(id_chirho: string | number, timeout_ms_chirho = 10_000): Promise<JsonObjectChirho> {
    const key_chirho = JSON.stringify(id_chirho);
    const deadline_chirho = Date.now() + timeout_ms_chirho;
    while (!this.answered_chirho.has(key_chirho)) {
      if (Date.now() > deadline_chirho) throw new Error(`no answer for id ${key_chirho} within ${timeout_ms_chirho} ms`);
      await Bun.sleep(5);
    }
    return this.answered_chirho.get(key_chirho) as JsonObjectChirho;
  }

  /** Waits until `count_chirho` answers with a null id (answers to unreadable input) have arrived. */
  async null_id_answers_after_chirho(count_chirho: number, timeout_ms_chirho = 5_000): Promise<JsonObjectChirho[]> {
    const deadline_chirho = Date.now() + timeout_ms_chirho;
    while (this.null_id_answers_chirho.length < count_chirho) {
      if (Date.now() > deadline_chirho) throw new Error(`only ${this.null_id_answers_chirho.length} null-id answers`);
      await Bun.sleep(5);
    }
    return [...this.null_id_answers_chirho];
  }

  private closing_chirho: Promise<number> | undefined;

  /** Ends stdin and waits for the adapter to finish on its own. Safe to call more than once. */
  close_chirho(): Promise<number> {
    this.closing_chirho ??= (async () => {
      this.child_chirho.stdin.end();
      const exit_code_chirho = await this.child_chirho.exited;
      await this.reading_chirho;
      return exit_code_chirho;
    })();
    return this.closing_chirho;
  }
}

/** A legacy session: initialize at `version_chirho`, then the initialized notification; the answer's result. */
export async function legacy_session_chirho(client_chirho: McpStdioClientChirho, version_chirho = "2025-06-18"): Promise<JsonObjectChirho> {
  const answer_chirho = await client_chirho.request_chirho("init", "initialize", { protocolVersion: version_chirho, capabilities: {}, clientInfo: { name: "test-chirho", version: "0" } });
  client_chirho.send_chirho({ jsonrpc: "2.0", method: "notifications/initialized" });
  return answer_chirho.result as JsonObjectChirho;
}

/** One tools/call, with or without arguments; its result. */
export async function call_tool_chirho(client_chirho: McpStdioClientChirho, id_chirho: number, name_chirho: string, arguments_chirho?: JsonObjectChirho): Promise<JsonObjectChirho> {
  const params_chirho: JsonObjectChirho = arguments_chirho === undefined ? { name: name_chirho } : { name: name_chirho, arguments: arguments_chirho };
  return (await client_chirho.request_chirho(id_chirho, "tools/call", params_chirho)).result as JsonObjectChirho;
}
