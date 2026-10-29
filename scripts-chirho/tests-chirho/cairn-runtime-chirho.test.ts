// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
import { afterEach, beforeEach, expect, test } from "bun:test";
import { chmodSync, mkdirSync, mkdtempSync, realpathSync, rmSync } from "node:fs";
import { join, resolve } from "node:path";
import { tmpdir } from "node:os";

const selector_chirho = resolve(import.meta.dir, "../lib-chirho/cairn-runtime-chirho.sh");
let root_chirho: string;
let product_chirho: string;
let engine_chirho: string;
let pin_chirho: string;
let older_chirho: string;

function git_chirho(...args_chirho: string[]): string {
  const result_chirho = Bun.spawnSync(["git", "-C", engine_chirho, ...args_chirho], { stdout: "pipe", stderr: "pipe" });
  if (result_chirho.exitCode !== 0) throw Error(result_chirho.stderr.toString());
  return result_chirho.stdout.toString().trim();
}

async function commit_chirho(path_chirho: string, value_chirho: string): Promise<string> {
  await Bun.write(join(engine_chirho, path_chirho), value_chirho);
  git_chirho("add", "--", path_chirho);
  git_chirho("-c", "user.name=Runtime fixture Chirho", "-c", "user.email=fixture@example.invalid", "-c", "commit.gpgsign=false",
    "commit", "--quiet", "-m", "Synthetic runtime fixture Chirho");
  return git_chirho("rev-parse", "HEAD");
}

async function binary_chirho(profile_chirho: string, stamp_chirho: string, exit_chirho = 0): Promise<string> {
  const path_chirho = join(engine_chirho, "target", profile_chirho, "cairn-chirho");
  await Bun.write(path_chirho, `#!/bin/sh
# Generated synthetic runtime Chirho. Only the read-only version operation is supported.
[ "$#" = 1 ] && [ "$1" = --version ] || exit 88
printf '%s\\n' 'cairn-chirho 0.1.0 (${stamp_chirho})'
exit ${exit_chirho}
`);
  chmodSync(path_chirho, 0o700);
  return path_chirho;
}

function run_chirho(override_chirho?: string, source_chirho = false, candidate_chirho?: string, sha_chirho?: string) {
  const env_chirho = { ...process.env };
  delete env_chirho.CAIRN_CLI_CHIRHO;
  delete env_chirho.CANDIDATE_CAIRN_COMMIT_CHIRHO;
  delete env_chirho.CANDIDATE_CAIRN_SHA256_CHIRHO;
  if (override_chirho !== undefined) env_chirho.CAIRN_CLI_CHIRHO = override_chirho;
  if (candidate_chirho !== undefined) env_chirho.CANDIDATE_CAIRN_COMMIT_CHIRHO = candidate_chirho;
  if (sha_chirho !== undefined) env_chirho.CANDIDATE_CAIRN_SHA256_CHIRHO = sha_chirho;
  return Bun.spawnSync(source_chirho ? ["bash", "-euc", 'source "$1"; resolve_cairn_runtime_chirho "$2" "$3"',
    "runtime-test-chirho", selector_chirho, product_chirho, engine_chirho]
    : ["bash", selector_chirho, product_chirho, engine_chirho], { env: env_chirho, stdout: "pipe", stderr: "pipe" });
}

function refused_chirho(override_chirho?: string): void {
  const result_chirho = run_chirho(override_chirho);
  expect(result_chirho.exitCode).not.toBe(0);
  expect(result_chirho.stdout.toString()).toBe("");
}

beforeEach(async () => {
  root_chirho = realpathSync(mkdtempSync(join(tmpdir(), "runtime-selector-chirho-")));
  product_chirho = join(root_chirho, "product with spaces-chirho");
  engine_chirho = join(root_chirho, "engine with spaces-chirho");
  mkdirSync(product_chirho);
  mkdirSync(engine_chirho);
  git_chirho("init", "--quiet");
  older_chirho = await commit_chirho("source-chirho.rs", "// older fixture chirho\n");
  pin_chirho = await commit_chirho("source-chirho.rs", "// selected fixture chirho\n");
  await Bun.write(join(product_chirho, "CAIRN-PIN-chirho.txt"), `# Synthetic fixture Chirho\n${pin_chirho}\n`);
});

afterEach(() => { rmSync(root_chirho, { recursive: true, force: true }); });

test("skips stale debug and uses matching release, with no non-version CLI operation", async () => {
  await binary_chirho("debug", older_chirho);
  const release_chirho = await binary_chirho("release", pin_chirho);
  for (const source_chirho of [false, true]) {
    const result_chirho = run_chirho(undefined, source_chirho);
    expect(result_chirho.exitCode).toBe(0);
    expect(result_chirho.stdout.toString().trim()).toBe(release_chirho);
    expect(result_chirho.stderr.toString()).toContain(pin_chirho);
  }
});

test("matching debug remains usable; profile is not evidence of freshness", async () => {
  const debug_chirho = await binary_chirho("debug", pin_chirho);
  await binary_chirho("release", older_chirho);
  expect(run_chirho().stdout.toString().trim()).toBe(debug_chirho);
});

test("explicit override is validated, never silently replaced with another installed CLI", async () => {
  const debug_chirho = await binary_chirho("debug", older_chirho);
  const release_chirho = await binary_chirho("release", pin_chirho);
  refused_chirho(debug_chirho);
  refused_chirho("");
  refused_chirho(join(engine_chirho, "missing-chirho"));
  expect(run_chirho(release_chirho).stdout.toString().trim()).toBe(release_chirho);
});

test("only stale or missing binaries refuse with no path", async () => {
  refused_chirho();
  await binary_chirho("debug", older_chirho);
  refused_chirho();
});

test("non-executable, malformed stamp and failed version process cannot pass", async () => {
  const debug_chirho = await binary_chirho("debug", pin_chirho);
  chmodSync(debug_chirho, 0o600);
  refused_chirho();
  await binary_chirho("debug", pin_chirho.slice(0, 8));
  refused_chirho();
  await binary_chirho("debug", pin_chirho, 1);
  refused_chirho();
});

test("missing, malformed and multiline pins refuse even beside a matching binary", async () => {
  await binary_chirho("debug", pin_chirho);
  for (const value_chirho of ["", "main_chirho", `${pin_chirho}\n${pin_chirho}`, `${pin_chirho.slice(0, 20)}\n${pin_chirho.slice(20)}`]) {
    await Bun.write(join(product_chirho, "CAIRN-PIN-chirho.txt"), value_chirho);
    refused_chirho();
  }
  rmSync(join(product_chirho, "CAIRN-PIN-chirho.txt"));
  refused_chirho();
});

test("pin tags resolve to commits; a missing tag or object refuses", async () => {
  const debug_chirho = await binary_chirho("debug", pin_chirho);
  git_chirho("tag", "pin-chirho/runtime-test-chirho", pin_chirho);
  await Bun.write(join(product_chirho, "CAIRN-PIN-chirho.txt"), "pin-chirho/runtime-test-chirho\n");
  expect(run_chirho().stdout.toString().trim()).toBe(debug_chirho);
  await Bun.write(join(product_chirho, "CAIRN-PIN-chirho.txt"), "pin-chirho/missing-chirho\n");
  refused_chirho();
  await Bun.write(join(product_chirho, "CAIRN-PIN-chirho.txt"), "f".repeat(40));
  refused_chirho();
});

test("documentation-only descendant is usable without rebuilding the runtime", async () => {
  const docs_chirho = await commit_chirho("docs-chirho/notes-chirho.md", "Fixture notes Chirho\n");
  const debug_chirho = await binary_chirho("debug", docs_chirho);
  expect(run_chirho().stdout.toString().trim()).toBe(debug_chirho);
});

test("code and config drift fail even when a newer commit contains the pin", async () => {
  for (const path_chirho of ["source-chirho.rs", "config-chirho.toml"]) {
    const changed_chirho = await commit_chirho(path_chirho, "// not the pinned bytes chirho\n");
    await binary_chirho("debug", changed_chirho);
    refused_chirho();
  }
});

test("renaming code into a Markdown path does not hide its deletion", async () => {
  git_chirho("mv", "source-chirho.rs", "source-chirho.md");
  const changed_chirho = await commit_chirho("source-chirho.md", "renamed fixture Chirho\n");
  await binary_chirho("debug", changed_chirho);
  refused_chirho();
});

test("unavailable stamp and a different branch cannot masquerade as docs-only drift", async () => {
  await binary_chirho("debug", "f".repeat(40));
  refused_chirho();
  git_chirho("checkout", "--quiet", "--detach", older_chirho);
  const sibling_chirho = await commit_chirho("notes-chirho.md", "Branch fixture Chirho\n");
  await binary_chirho("debug", sibling_chirho);
  refused_chirho();
});

// A CANDIDATE run re-gates the product against an engine successor before the pin moves. It is
// explicit (one full commit + the executable), exact (no descendant tolerance), and loud; it never
// weakens the ordinary selector, which still refuses the same successor without the variable.
test("a named candidate is selected exactly, and only when named together with its executable", async () => {
  const successor_chirho = await commit_chirho("source-chirho.rs", "// the engine fix under test chirho\n");
  const candidate_chirho = await binary_chirho("release", successor_chirho);
  // the ordinary selector refuses code drift, with or without the override
  refused_chirho();
  refused_chirho(candidate_chirho);
  // named as the candidate: selected, and the log says CANDIDATE and NOT the pin
  const named_chirho = run_chirho(candidate_chirho, false, successor_chirho);
  expect(named_chirho.exitCode).toBe(0);
  expect(named_chirho.stdout.toString().trim()).toBe(candidate_chirho);
  expect(named_chirho.stderr.toString()).toContain("CANDIDATE");
  expect(named_chirho.stderr.toString()).toContain("NOT the pin");
  // the pin's own build is NOT the named candidate: refused, never silently substituted
  const pinned_chirho = await binary_chirho("debug", pin_chirho);
  expect(run_chirho(pinned_chirho, false, successor_chirho).exitCode).not.toBe(0);
  // a docs-only descendant of the candidate is not the candidate either: exact stamp only
  const docs_chirho = await commit_chirho("notes-chirho.md", "Fixture notes Chirho\n");
  const docs_binary_chirho = await binary_chirho("docs-descendant", docs_chirho);
  expect(run_chirho(candidate_chirho, false, docs_chirho).exitCode).not.toBe(0);
  expect(run_chirho(docs_binary_chirho, false, successor_chirho).exitCode).not.toBe(0);
  // an abbreviated commit, or a candidate without the executable named, is a refusal, not a guess
  expect(run_chirho(candidate_chirho, false, successor_chirho.slice(0, 12)).exitCode).not.toBe(0);
  const unnamed_chirho = run_chirho(undefined, false, successor_chirho);
  expect(unnamed_chirho.exitCode).not.toBe(0);
  expect(unnamed_chirho.stdout.toString()).toBe("");
});

// The delivered SHA-256 is optional for the shell gates (they bind to the stamp) and, when given,
// must match the bytes. gpt's reviewed-app consumer requires it; a commit-only handoff is enough
// for these gates and not for that consumer.
test("a delivered SHA-256, when supplied, must match the candidate's bytes", async () => {
  const successor_chirho = await commit_chirho("source-chirho.rs", "// the engine fix under test chirho\n");
  const candidate_chirho = await binary_chirho("release", successor_chirho);
  const actual_chirho = new Bun.CryptoHasher("sha256").update(await Bun.file(candidate_chirho).arrayBuffer()).digest("hex");
  const matching_chirho = run_chirho(candidate_chirho, false, successor_chirho, actual_chirho);
  expect(matching_chirho.exitCode).toBe(0);
  expect(matching_chirho.stderr.toString()).toContain("verified");
  const unsupplied_chirho = run_chirho(candidate_chirho, false, successor_chirho);
  expect(unsupplied_chirho.exitCode).toBe(0);
  expect(unsupplied_chirho.stderr.toString()).toContain("not supplied");
  expect(run_chirho(candidate_chirho, false, successor_chirho, "0".repeat(64)).exitCode).not.toBe(0);
  expect(run_chirho(candidate_chirho, false, successor_chirho, actual_chirho.slice(0, 12)).exitCode).not.toBe(0);
});
