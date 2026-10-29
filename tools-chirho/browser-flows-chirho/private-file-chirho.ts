// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// Owner-only files for synthetic credentials, configs and private scan controls. Bun.write ignores
// its `mode` option (measured on Bun 1.4.0: 0644 under umask 022); node:fs applies it at creation,
// with no readable window. The result is verified, so a pre-existing wider file refuses.
import { stat, writeFile } from "node:fs/promises";

export async function write_private_chirho(path_chirho: string, data_chirho: string | Uint8Array): Promise<void> {
  await writeFile(path_chirho, data_chirho, { mode: 0o600 });
  if (((await stat(path_chirho)).mode & 0o077) !== 0) throw Error(`Private file is not owner-only: ${path_chirho}`);
}
