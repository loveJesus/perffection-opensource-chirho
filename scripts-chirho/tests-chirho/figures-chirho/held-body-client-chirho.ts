// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// A Save whose body is held open (the K1 gate's route-changed arm, key-grants-rehearsal-chirho.sh): the
// head and the first half of the body go at once, so the engine admits the key before it has the body;
// the rest goes only once <release file> exists, so the gate can redeploy in between. The whole answer
// is written to <answer file> (status line, then the body after a blank line); "held" is printed once
// the first half is sent. One request, one connection, nothing retried.
// Usage: bun held-body-client-chirho.ts <port> <path> <key file> <body file> <release file> <answer file>
// Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/her-figures-chirho.md
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { connect } from "node:net";

const [port_text_chirho, path_chirho, key_file_chirho, body_file_chirho, release_file_chirho, answer_file_chirho] = process.argv.slice(2);
if (!answer_file_chirho) throw new Error("usage: <port> <path> <key file> <body file> <release file> <answer file>");
const key_chirho = readFileSync(key_file_chirho, "utf8").trim();
const body_chirho = readFileSync(body_file_chirho);
const half_chirho = Math.floor(body_chirho.length / 2);
const head_chirho =
  `POST ${path_chirho} HTTP/1.1\r\nHost: 127.0.0.1:${port_text_chirho}\r\nAuthorization: Bearer ${key_chirho}\r\n` +
  `Accept: application/json\r\nContent-Type: application/json\r\nContent-Length: ${body_chirho.length}\r\nConnection: close\r\n\r\n`;
const received_chirho: Buffer[] = [];
const socket_chirho = connect(Number(port_text_chirho), "127.0.0.1", () => {
  socket_chirho.write(Buffer.concat([Buffer.from(head_chirho, "latin1"), body_chirho.subarray(0, half_chirho)]));
  console.log("held");
  const wait_chirho = setInterval(() => {
    if (!existsSync(release_file_chirho)) return;
    clearInterval(wait_chirho);
    socket_chirho.write(body_chirho.subarray(half_chirho));
  }, 50);
});
socket_chirho.on("data", chunk_chirho => received_chirho.push(typeof chunk_chirho === "string" ? Buffer.from(chunk_chirho, "latin1") : chunk_chirho));
socket_chirho.on("close", () => {
  const answer_chirho = Buffer.concat(received_chirho).toString("utf8");
  const split_chirho = answer_chirho.indexOf("\r\n\r\n");
  const status_chirho = answer_chirho.split("\r\n", 1)[0] ?? "";
  writeFileSync(answer_file_chirho, `${status_chirho}\n\n${split_chirho < 0 ? "" : answer_chirho.slice(split_chirho + 4)}`);
  process.exit(0);
});
socket_chirho.on("error", error_chirho => {
  writeFileSync(answer_file_chirho, `connection-error ${error_chirho.message}\n\n`);
  process.exit(1);
});
