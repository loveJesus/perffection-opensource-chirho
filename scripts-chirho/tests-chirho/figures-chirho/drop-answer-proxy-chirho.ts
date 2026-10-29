// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// A loopback proxy for the figures gate's uncertain-save arms (../figures-rehearsal-chirho.sh). Every
// connection is passed to the upstream unchanged, and carries ONE request: once an answer is whole the
// connection ends, so a browser, which reuses its connections, sends its next request on a fresh one,
// and each request is judged by its own first line. On a POST to the named path the Save is forwarded,
// and her connection is closed without any answer once the upstream's answer is whole (its headers
// and Content-Length bytes), or earlier if the upstream ends or fails first: either way her side
// learns nothing, which is exactly an uncertain Save. Which of the two happened is printed, one line
// per dropped answer (gpt #26583): `dropped-after-whole-answer-chirho` or
// `dropped-before-whole-answer-chirho`, so the gate can tell them apart rather than assume. Nothing is
// retried or rewritten.
// Usage: bun drop-answer-proxy-chirho.ts <listen port> <upstream port> <path to drop the answer of>
// Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/her-figures-chirho.md
import { writeSync } from "node:fs";
import { connect, createServer, type Socket } from "node:net";

const [listen_text_chirho, upstream_text_chirho, drop_path_chirho] = process.argv.slice(2);
if (!drop_path_chirho) throw new Error("usage: <listen port> <upstream port> <path to drop the answer of>");
const upstream_port_chirho = Number(upstream_text_chirho);

/// The answer is whole once its headers and Content-Length bytes have arrived.
function answer_complete_chirho(answer_chirho: Buffer): boolean {
  const end_chirho = answer_chirho.indexOf("\r\n\r\n");
  if (end_chirho < 0) return false;
  const length_chirho = /\r\ncontent-length:\s*(\d+)/i.exec(answer_chirho.subarray(0, end_chirho).toString("latin1"));
  return length_chirho !== null && answer_chirho.length >= end_chirho + 4 + Number(length_chirho[1]);
}

createServer((client_chirho: Socket) => {
  let upstream_chirho: Socket | undefined;
  let dropping_chirho = false;
  let dropped_chirho = false;
  let answer_chirho = Buffer.alloc(0);
  let pending_chirho = Buffer.alloc(0);
  // Her connection closes with no answer; the line is written first, synchronously, so it is in the
  // log before her side can see the close.
  const drop_chirho = (whole_chirho: boolean) => {
    if (dropped_chirho) return;
    dropped_chirho = true;
    writeSync(1, `${whole_chirho ? "dropped-after-whole-answer-chirho" : "dropped-before-whole-answer-chirho"}\n`);
    client_chirho.destroy();
    upstream_chirho?.destroy();
  };
  client_chirho.on("data", (chunk_chirho: Buffer) => {
    if (upstream_chirho) {
      upstream_chirho.write(chunk_chirho);
      return;
    }
    pending_chirho = Buffer.concat([pending_chirho, chunk_chirho]);
    if (!pending_chirho.includes("\r\n")) return;
    dropping_chirho = pending_chirho.toString("latin1").startsWith(`POST ${drop_path_chirho} `);
    const first_chirho = pending_chirho;
    upstream_chirho = connect(upstream_port_chirho, "127.0.0.1");
    // written now, not when the connection opens: writes made before it opens are queued in order, so a
    // request that arrives in several chunks (a browser's upload) reaches the upstream head first
    upstream_chirho.write(first_chirho);
    upstream_chirho.on("data", (data_chirho: Buffer) => {
      answer_chirho = Buffer.concat([answer_chirho, data_chirho]);
      if (!dropping_chirho) {
        client_chirho.write(data_chirho);
        if (answer_complete_chirho(answer_chirho)) {
          client_chirho.end();
          upstream_chirho?.end();
        }
        return;
      }
      if (answer_complete_chirho(answer_chirho)) drop_chirho(true);
    });
    upstream_chirho.on("end", () => (dropping_chirho ? drop_chirho(false) : client_chirho.end()));
    upstream_chirho.on("error", () => (dropping_chirho ? drop_chirho(false) : client_chirho.destroy()));
  });
  client_chirho.on("error", () => upstream_chirho?.destroy());
  client_chirho.on("close", () => upstream_chirho?.destroy());
}).listen(Number(listen_text_chirho), "127.0.0.1");
