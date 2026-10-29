// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
// Workflow: docs-chirho/source-chirho/source-boundary-chirho.md. Read origin objects, never its worktree.
import { createHash as create_hash_chirho } from "node:crypto";
import { existsSync as exists_sync_chirho, lstatSync as lstat_sync_chirho, mkdirSync as mkdir_sync_chirho,
  readFileSync as read_file_sync_chirho, readlinkSync as readlink_sync_chirho, readdirSync as readdir_sync_chirho,
  writeFileSync as write_file_sync_chirho } from "node:fs";
import { dirname as dirname_chirho, join as join_chirho, relative as relative_chirho, resolve as resolve_chirho } from "node:path";

const ROOT_CHIRHO = resolve_chirho(import.meta.dir, "../..");
const INVENTORY_CHIRHO = join_chirho(ROOT_CHIRHO, "docs-chirho/source-chirho/provenance-chirho");
const ORIGIN_COMMIT_CHIRHO = "ee4dd25c8fb95b2f60a5693be02c61d0285636a8";
const HEADER_CHIRHO = { schema_chirho: "perffection-source-import-v1-chirho", source_commit_chirho: ORIGIN_COMMIT_CHIRHO,
  notice_chirho: "For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)" };
const EXCLUDED_ROOTS_CHIRHO = new Set(["AGENTS.md", "CLAUDE.md", "README.md", ".gitignore", ".gitleaks.toml", ".gitleaksignore"]);
const SHARD_SIZE_CHIRHO = 600;

interface ImportRecordChirho {
  path_chirho: string;
  source_blob_chirho: string;
  source_mode_chirho: string;
  source_sha256_chirho: string;
  imported_sha256_chirho: string;
  imported_mode_chirho: string;
  changed_from_source_chirho: boolean;
}

function digest_chirho(bytes_chirho: Uint8Array): string {
  return create_hash_chirho("sha256").update(bytes_chirho).digest("hex");
}

function git_chirho(origin_chirho: string, ...arguments_chirho: string[]): Buffer {
  const result_chirho = Bun.spawnSync(["git", "-C", origin_chirho, ...arguments_chirho], { stdout: "pipe", stderr: "pipe" });
  if (result_chirho.exitCode !== 0) throw Error(result_chirho.stderr.toString().trim());
  return Buffer.from(result_chirho.stdout);
}

function local_chirho(path_chirho: string): { bytes_chirho: Buffer; mode_chirho: string } {
  const absolute_chirho = resolve_chirho(ROOT_CHIRHO, path_chirho);
  if (!path_chirho || relative_chirho(ROOT_CHIRHO, absolute_chirho).startsWith("..") || absolute_chirho === ROOT_CHIRHO) {
    throw Error(`Inventory path escapes product source: ${path_chirho}`);
  }
  const status_chirho = lstat_sync_chirho(absolute_chirho);
  if (status_chirho.isSymbolicLink()) {
    const target_chirho = readlink_sync_chirho(absolute_chirho);
    const resolved_chirho = resolve_chirho(dirname_chirho(absolute_chirho), target_chirho);
    if (relative_chirho(ROOT_CHIRHO, resolved_chirho).startsWith("..") || !exists_sync_chirho(resolved_chirho)) {
      throw Error(`Unresolved or out-of-tree imported symlink: ${path_chirho}`);
    }
    return { bytes_chirho: Buffer.from(target_chirho), mode_chirho: "120000" };
  }
  if (!status_chirho.isFile()) throw Error(`Not an imported source file: ${path_chirho}`);
  return { bytes_chirho: read_file_sync_chirho(absolute_chirho), mode_chirho: status_chirho.mode & 0o111 ? "100755" : "100644" };
}

function record_chirho(origin_chirho: string): void {
  const records_chirho: ImportRecordChirho[] = [];
  const tree_chirho = git_chirho(origin_chirho, "ls-tree", "-r", "-z", ORIGIN_COMMIT_CHIRHO).toString().split("\0");
  for (const entry_chirho of tree_chirho) {
    if (!entry_chirho) continue;
    const match_chirho = /^(100644|100755|120000) blob ([0-9a-f]{40})\t(.+)$/.exec(entry_chirho);
    if (!match_chirho) throw Error("Unexpected source-tree object");
    const [, mode_chirho, blob_chirho, path_chirho] = match_chirho;
    if (EXCLUDED_ROOTS_CHIRHO.has(path_chirho) || path_chirho.startsWith("spec-chirho/")) continue;
    const destination_chirho = join_chirho(ROOT_CHIRHO, path_chirho);
    if (!exists_sync_chirho(destination_chirho)) continue;
    const imported_chirho = local_chirho(path_chirho);
    const local_blob_chirho = create_hash_chirho("sha1").update(`blob ${imported_chirho.bytes_chirho.length}\0`)
      .update(imported_chirho.bytes_chirho).digest("hex");
    const source_bytes_chirho = local_blob_chirho === blob_chirho ? imported_chirho.bytes_chirho : git_chirho(origin_chirho, "cat-file", "blob", blob_chirho);
    records_chirho.push({ path_chirho, source_blob_chirho: blob_chirho, source_mode_chirho: mode_chirho,
      source_sha256_chirho: digest_chirho(source_bytes_chirho), imported_sha256_chirho: digest_chirho(imported_chirho.bytes_chirho),
      imported_mode_chirho: imported_chirho.mode_chirho, changed_from_source_chirho: local_blob_chirho !== blob_chirho || mode_chirho !== imported_chirho.mode_chirho });
  }
  mkdir_sync_chirho(INVENTORY_CHIRHO, { recursive: true });
  for (let offset_chirho = 0; offset_chirho < records_chirho.length; offset_chirho += SHARD_SIZE_CHIRHO) {
    const name_chirho = `import-${String(offset_chirho / SHARD_SIZE_CHIRHO).padStart(2, "0")}-chirho.jsonl`;
    const lines_chirho = [HEADER_CHIRHO, ...records_chirho.slice(offset_chirho, offset_chirho + SHARD_SIZE_CHIRHO)];
    write_file_sync_chirho(join_chirho(INVENTORY_CHIRHO, name_chirho), lines_chirho.map(value_chirho => JSON.stringify(value_chirho)).join("\n") + "\n");
  }
  console.log(JSON.stringify({ source_commit_chirho: ORIGIN_COMMIT_CHIRHO, imported_paths_chirho: records_chirho.length,
    changed_paths_chirho: records_chirho.filter(record_chirho => record_chirho.changed_from_source_chirho).map(record_chirho => record_chirho.path_chirho) }, null, 2));
}

function check_chirho(): void {
  let count_chirho = 0;
  for (const name_chirho of readdir_sync_chirho(INVENTORY_CHIRHO).sort()) {
    if (!/^import-[0-9]{2}-chirho\.jsonl$/.test(name_chirho)) continue;
    const lines_chirho = read_file_sync_chirho(join_chirho(INVENTORY_CHIRHO, name_chirho), "utf8").trim().split("\n");
    const header_chirho = JSON.parse(lines_chirho.shift()!);
    if (header_chirho.schema_chirho !== HEADER_CHIRHO.schema_chirho || header_chirho.source_commit_chirho !== ORIGIN_COMMIT_CHIRHO) throw Error("Inventory source boundary differs");
    for (const line_chirho of lines_chirho) {
      const record_chirho: ImportRecordChirho = JSON.parse(line_chirho);
      const imported_chirho = local_chirho(record_chirho.path_chirho);
      if (digest_chirho(imported_chirho.bytes_chirho) !== record_chirho.imported_sha256_chirho || imported_chirho.mode_chirho !== record_chirho.imported_mode_chirho) {
        throw Error(`Imported source changed since inventory: ${record_chirho.path_chirho}`);
      }
      count_chirho++;
    }
  }
  if (count_chirho === 0) throw Error("Empty import inventory");
  console.log(JSON.stringify({ schema_chirho: "perffection-import-check-v1-chirho", checked_paths_chirho: count_chirho, unchanged_since_inventory_chirho: true }));
}

const [mode_chirho, origin_chirho, ...extra_chirho] = process.argv.slice(2);
if (mode_chirho === "--record-chirho" && origin_chirho && extra_chirho.length === 0) record_chirho(resolve_chirho(origin_chirho));
else if (mode_chirho === "--check-chirho" && origin_chirho === undefined) check_chirho();
else throw Error("usage: bun tools-chirho/source-chirho/import-inventory-chirho.ts --check-chirho | --record-chirho <read-only-origin>");
