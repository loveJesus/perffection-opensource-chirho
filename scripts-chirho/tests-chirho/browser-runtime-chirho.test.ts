// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
import { afterEach, beforeEach, expect, test } from "bun:test";
import { chmod, copyFile, mkdir, mkdtemp, readFile, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { freeze_cairn_candidate_chirho, resolve_reviewed_runtime_chirho } from "../../tools-chirho/browser-chirho/cairn-runtime-chirho";

let root_chirho: string;
let product_chirho: string;
let engine_chirho: string;
let scratch_chirho: string;
let binary_chirho: string;
let pin_chirho: string;
let commit_chirho: string;
let environment_chirho: NodeJS.ProcessEnv;
const helpers_chirho = ["tools-chirho/app-deploy-chirho/host_ready_app_manifest_chirho.py"];

function git_chirho(...args_chirho: string[]): string {
  const result_chirho = Bun.spawnSync(["git", "-C", engine_chirho, ...args_chirho], { stdout: "pipe", stderr: "pipe" });
  if (result_chirho.exitCode !== 0) throw Error(result_chirho.stderr.toString());
  return result_chirho.stdout.toString().trim();
}
async function hash_chirho(path_chirho: string): Promise<string> {
  return new Bun.CryptoHasher("sha256").update(await readFile(path_chirho)).digest("hex");
}
async function write_binary_chirho(stamp_chirho: string, exit_chirho = 0): Promise<void> {
  await Bun.write(binary_chirho, `#!/bin/sh\n[ "$1" = "--version" ] || exit 88\nprintf 'invoked' > ${JSON.stringify(join(root_chirho, "invoked-chirho"))}\nprintf '%s\\n' ${JSON.stringify(stamp_chirho)}\nexit ${exit_chirho}\n`);
  await chmod(binary_chirho, 0o700);
}
function select_chirho(): ReturnType<typeof resolve_reviewed_runtime_chirho> {
  return resolve_reviewed_runtime_chirho(product_chirho, engine_chirho, scratch_chirho, environment_chirho);
}
beforeEach(async () => {
  root_chirho = await mkdtemp(join(tmpdir(), "browser-runtime-chirho-"));
  product_chirho = join(root_chirho, "product with spaces-chirho");
  engine_chirho = join(root_chirho, "engine with spaces-chirho");
  scratch_chirho = join(root_chirho, "scratch-chirho");
  binary_chirho = join(root_chirho, "candidate-chirho");
  await mkdir(join(product_chirho, "scripts-chirho/lib-chirho"), { recursive: true });
  await mkdir(join(engine_chirho, "tools-chirho/app-deploy-chirho"), { recursive: true });
  await mkdir(scratch_chirho);
  await copyFile(resolve(import.meta.dir, "../lib-chirho/cairn-runtime-chirho.sh"), join(product_chirho, "scripts-chirho/lib-chirho/cairn-runtime-chirho.sh"));
  git_chirho("init", "-q");
  git_chirho("config", "user.name", "Synthetic Chirho");
  git_chirho("config", "user.email", "synthetic@example.invalid");
  git_chirho("config", "commit.gpgsign", "false");
  for (const relative_chirho of helpers_chirho) await Bun.write(join(engine_chirho, relative_chirho), "fixture-chirho\n");
  git_chirho("add", ".");
  git_chirho("commit", "-qm", "pin-chirho");
  pin_chirho = git_chirho("rev-parse", "HEAD");
  await Bun.write(join(product_chirho, "CAIRN-PIN-chirho.txt"), pin_chirho + "\n");
  await Bun.write(join(engine_chirho, "runtime-chirho.rs"), "// code-changing candidate-chirho\n");
  git_chirho("add", ".");
  git_chirho("commit", "-qm", "candidate-chirho");
  commit_chirho = git_chirho("rev-parse", "HEAD");
  await write_binary_chirho(`cairn-chirho 0.1.0 (${commit_chirho})`);
  environment_chirho = { ...process.env, CAIRN_CLI_CHIRHO: binary_chirho,
    CANDIDATE_CAIRN_COMMIT_CHIRHO: commit_chirho, CANDIDATE_CAIRN_SHA256_CHIRHO: await hash_chirho(binary_chirho) };
});
afterEach(async () => { await rm(root_chirho, { recursive: true, force: true }); });

test("explicit hash-bound candidate freezes bytes without changing the pin or relying on checkout HEAD", async () => {
  git_chirho("checkout", "--detach", pin_chirho);
  const selected_chirho = await select_chirho();
  expect(selected_chirho.mode_chirho).toBe("explicit-candidate-chirho");
  expect(selected_chirho.commit_chirho).toBe(commit_chirho);
  expect(environment_chirho.CANDIDATE_CAIRN_SHA256_CHIRHO).toBe(await hash_chirho(selected_chirho.cli_chirho));
  expect(Object.keys(selected_chirho.helper_hashes_chirho)).toEqual(helpers_chirho);
  await write_binary_chirho(`cairn-chirho 0.1.0 (${pin_chirho})`);
  expect(selected_chirho.sha256_chirho).toBe(await hash_chirho(selected_chirho.cli_chirho));
  expect((await readFile(join(product_chirho, "CAIRN-PIN-chirho.txt"), "utf8")).trim()).toBe(pin_chirho);
});
test("ordinary selection accepts the pin and refuses a code-changing candidate without opt-in", async () => {
  delete environment_chirho.CANDIDATE_CAIRN_COMMIT_CHIRHO;
  delete environment_chirho.CANDIDATE_CAIRN_SHA256_CHIRHO;
  await expect(select_chirho()).rejects.toThrow();
  await write_binary_chirho(`cairn-chirho 0.1.0 (${pin_chirho})`);
  expect((await select_chirho()).mode_chirho).toBe("installed-pin-chirho");
});
for (const key_chirho of ["CANDIDATE_CAIRN_COMMIT_CHIRHO", "CANDIDATE_CAIRN_SHA256_CHIRHO", "CAIRN_CLI_CHIRHO"]) {
  test(`partial candidate selection refuses without execution: ${key_chirho}`, async () => {
    delete environment_chirho[key_chirho];
    await expect(select_chirho()).rejects.toThrow("no fallback");
    expect(await Bun.file(join(root_chirho, "invoked-chirho")).exists()).toBe(false);
  });
}
for (const [key_chirho, value_chirho] of [["CANDIDATE_CAIRN_COMMIT_CHIRHO", "abc123"], ["CANDIDATE_CAIRN_COMMIT_CHIRHO", ""],
  ["CANDIDATE_CAIRN_COMMIT_CHIRHO", "f".repeat(40)], ["CANDIDATE_CAIRN_SHA256_CHIRHO", "abc123"], ["CANDIDATE_CAIRN_SHA256_CHIRHO", "f".repeat(64)]]) {
  test(`invalid candidate identity refuses before execution: ${key_chirho}/${value_chirho.slice(0, 7)}`, async () => {
    environment_chirho[key_chirho] = value_chirho;
    await expect(select_chirho()).rejects.toThrow();
    expect(await Bun.file(join(root_chirho, "invoked-chirho")).exists()).toBe(false);
  });
}
for (const relative_chirho of helpers_chirho) {
  test(`consumed helper mismatch refuses before executable invocation: ${relative_chirho}`, async () => {
    await Bun.write(join(engine_chirho, relative_chirho), "changed-chirho\n");
    await expect(select_chirho()).rejects.toThrow("helper differs");
    expect(await Bun.file(join(root_chirho, "invoked-chirho")).exists()).toBe(false);
  });
}
for (const kind_chirho of ["wrong-commit-chirho", "malformed-chirho", "failed-command-chirho"]) {
  test(`an exact hash cannot excuse a bad stamp: ${kind_chirho}`, async () => {
    await write_binary_chirho(kind_chirho === "malformed-chirho" ? "not-cairn-chirho" : `cairn-chirho 0.1.0 (${kind_chirho === "wrong-commit-chirho" ? pin_chirho : commit_chirho})`, kind_chirho === "failed-command-chirho" ? 1 : 0);
    environment_chirho.CANDIDATE_CAIRN_SHA256_CHIRHO = await hash_chirho(binary_chirho);
    await expect(select_chirho()).rejects.toThrow("stamp");
  });
}
test("missing executable refuses and an existing freeze is never overwritten", async () => {
  environment_chirho.CAIRN_CLI_CHIRHO = join(root_chirho, "missing-chirho");
  await expect(select_chirho()).rejects.toThrow();
  environment_chirho.CAIRN_CLI_CHIRHO = binary_chirho;
  const destination_chirho = join(scratch_chirho, "cairn-reviewed-candidate-chirho");
  await Bun.write(destination_chirho, "preserve-chirho");
  await expect(select_chirho()).rejects.toThrow();
  expect(await Bun.file(destination_chirho).text()).toBe("preserve-chirho");
});
test("signup compatibility uses the same freezer with its existing commit-only contract", async () => {
  const result_chirho = await freeze_cairn_candidate_chirho({ source_cli_chirho: binary_chirho,
    engine_chirho, commit_chirho, destination_chirho: join(scratch_chirho, "signup-candidate-chirho"), helpers_chirho: helpers_chirho.slice(0, 1) });
  expect(result_chirho.commit_chirho).toBe(commit_chirho);
  expect(result_chirho.sha256_chirho).toBe(await hash_chirho(binary_chirho));
});
