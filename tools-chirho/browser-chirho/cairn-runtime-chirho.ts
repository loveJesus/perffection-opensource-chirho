// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
import { constants as fs_constants_chirho } from "node:fs";
import { chmod, copyFile, readFile } from "node:fs/promises";
import { join, resolve } from "node:path";

// Ordinary selection keeps the shell's pin policy. Candidate selection is explicit below,
// so another command's candidate environment cannot silently bypass this function's pin check.
export function resolve_cairn_runtime_chirho(product_chirho: string, engine_chirho: string,
  environment_chirho: NodeJS.ProcessEnv = process.env): string {
  const pinned_environment_chirho = { ...environment_chirho };
  delete pinned_environment_chirho.CANDIDATE_CAIRN_COMMIT_CHIRHO;
  delete pinned_environment_chirho.CANDIDATE_CAIRN_SHA256_CHIRHO;
  const result_chirho = Bun.spawnSync([
    "bash", join(product_chirho, "scripts-chirho/lib-chirho/cairn-runtime-chirho.sh"), product_chirho, engine_chirho,
  ], { cwd: product_chirho, env: pinned_environment_chirho, stdout: "pipe", stderr: "pipe" });
  if (result_chirho.exitCode !== 0) throw Error(result_chirho.stderr.toString().trim());
  return result_chirho.stdout.toString().trim();
}

export interface RuntimeSelectionChirho {
  cli_chirho: string;
  mode_chirho: "installed-pin-chirho" | "explicit-candidate-chirho";
  source_cli_chirho?: string;
  commit_chirho?: string;
  sha256_chirho?: string;
  version_chirho?: string;
  helper_hashes_chirho: Record<string, string>;
}

function sha256_chirho(bytes_chirho: Uint8Array): string {
  return new Bun.CryptoHasher("sha256").update(bytes_chirho).digest("hex");
}

// Workflow: reviewed-product-chirho.md. A version stamp is a consistency check, not a build
// attestation. Freeze only inside the caller's owned scratch directory; never compile or adopt.
export async function freeze_cairn_candidate_chirho(options_chirho: {
  source_cli_chirho: string;
  engine_chirho: string;
  commit_chirho: string;
  destination_chirho: string;
  expected_sha256_chirho?: string;
  helpers_chirho: readonly string[];
}): Promise<RuntimeSelectionChirho> {
  if (!/^[a-f0-9]{40}$/.test(options_chirho.commit_chirho)) throw Error("Candidate needs a full commit");
  if (!options_chirho.source_cli_chirho.trim()) throw Error("Candidate needs explicit CAIRN_CLI_CHIRHO");
  if (options_chirho.expected_sha256_chirho !== undefined && !/^[a-f0-9]{64}$/.test(options_chirho.expected_sha256_chirho)) {
    throw Error("Candidate needs a full SHA-256");
  }
  const commit_chirho = Bun.spawnSync(["git", "-C", options_chirho.engine_chirho, "rev-parse", "--verify", `${options_chirho.commit_chirho}^{commit}`], { stdout: "pipe", stderr: "pipe" });
  if (commit_chirho.exitCode !== 0 || commit_chirho.stdout.toString().trim() !== options_chirho.commit_chirho) throw Error("Candidate commit is unavailable");
  const helper_hashes_chirho: Record<string, string> = {};
  for (const relative_chirho of options_chirho.helpers_chirho) {
    const committed_chirho = Bun.spawnSync(["git", "-C", options_chirho.engine_chirho, "show", `${options_chirho.commit_chirho}:${relative_chirho}`], { stdout: "pipe", stderr: "pipe" });
    if (committed_chirho.exitCode !== 0) throw Error(`Candidate helper is unavailable: ${relative_chirho}`);
    const expected_chirho = sha256_chirho(committed_chirho.stdout);
    if (sha256_chirho(await readFile(join(options_chirho.engine_chirho, relative_chirho))) !== expected_chirho) throw Error(`Candidate helper differs from its commit: ${relative_chirho}`);
    helper_hashes_chirho[relative_chirho] = expected_chirho;
  }
  const source_cli_chirho = resolve(options_chirho.source_cli_chirho);
  const source_sha_chirho = sha256_chirho(await readFile(source_cli_chirho));
  if (options_chirho.expected_sha256_chirho !== undefined && source_sha_chirho !== options_chirho.expected_sha256_chirho) throw Error("Candidate SHA-256 differs from the handoff");
  await copyFile(source_cli_chirho, options_chirho.destination_chirho, fs_constants_chirho.COPYFILE_EXCL);
  const frozen_sha_chirho = sha256_chirho(await readFile(options_chirho.destination_chirho));
  if (frozen_sha_chirho !== source_sha_chirho) throw Error("Candidate changed while freezing");
  await chmod(options_chirho.destination_chirho, 0o700);
  const version_chirho = Bun.spawnSync([options_chirho.destination_chirho, "--version"], { stdout: "pipe", stderr: "pipe" });
  const stamp_chirho = version_chirho.stdout.toString().trim();
  const match_chirho = /^cairn-chirho \S+ \(([a-f0-9]{40})\)$/.exec(stamp_chirho);
  if (version_chirho.exitCode !== 0 || match_chirho?.[1] !== options_chirho.commit_chirho) throw Error("Candidate stamp does not match the exact requested commit");
  if (sha256_chirho(await readFile(options_chirho.destination_chirho)) !== frozen_sha_chirho) throw Error("Candidate bytes changed during identity check");
  return { cli_chirho: options_chirho.destination_chirho, source_cli_chirho, mode_chirho: "explicit-candidate-chirho",
    commit_chirho: options_chirho.commit_chirho, sha256_chirho: frozen_sha_chirho, version_chirho: stamp_chirho, helper_hashes_chirho };
}

// A resumed stand-in freezes its second binary under its own name, so the first run's frozen copy stays beside it.
export async function resolve_reviewed_runtime_chirho(product_chirho: string, engine_chirho: string, scratch_chirho: string,
  environment_chirho: NodeJS.ProcessEnv = process.env, destination_name_chirho = "cairn-reviewed-candidate-chirho"): Promise<RuntimeSelectionChirho> {
  const commit_chirho = environment_chirho.CANDIDATE_CAIRN_COMMIT_CHIRHO;
  const hash_chirho = environment_chirho.CANDIDATE_CAIRN_SHA256_CHIRHO;
  if (commit_chirho === undefined && hash_chirho === undefined) {
    return { cli_chirho: resolve_cairn_runtime_chirho(product_chirho, engine_chirho, environment_chirho), mode_chirho: "installed-pin-chirho", helper_hashes_chirho: {} };
  }
  if (commit_chirho === undefined || hash_chirho === undefined || !environment_chirho.CAIRN_CLI_CHIRHO) {
    throw Error("Candidate requires CANDIDATE_CAIRN_COMMIT_CHIRHO, CANDIDATE_CAIRN_SHA256_CHIRHO and explicit CAIRN_CLI_CHIRHO; no fallback");
  }
  return freeze_cairn_candidate_chirho({ source_cli_chirho: environment_chirho.CAIRN_CLI_CHIRHO,
    engine_chirho, commit_chirho, expected_sha256_chirho: hash_chirho,
    destination_chirho: join(scratch_chirho, destination_name_chirho),
    helpers_chirho: ["tools-chirho/app-deploy-chirho/host_ready_app_manifest_chirho.py"] });
}
