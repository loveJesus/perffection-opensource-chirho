<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Perffection page and figures tools for your own agent Chirho

**This is software a Perffection customer runs on their own machine.** It lets your own AI agent
(Claude Code, or any app that speaks MCP, the Model Context Protocol) read and save your private
Perffection page and your private business figures. It is not part of the Perffection website, and
nothing here runs on our servers.

> **Not handed to anyone yet.** It needs an access key. The engine can now limit a key to named
> actions (read the page, save the figures, and so on), but customers cannot create keys yet, and the
> figures routes are on no released server. Until then it runs only against local test roots with
> operator-issued keys.

## What it touches, and what it never does

It calls exactly five routes of your workspace, fixed in the code; nothing your agent sends can
change which address is called. When one of them answers 404, it also asks the address's health
route (`/healthz-chirho`), WITHOUT your key, to tell a live address that lacks that tool's route from
an address that is wrong or down:

| Tool | Route | What it does |
| --- | --- | --- |
| `read_page_chirho` | `GET /app-chirho/perffection-workspace/agent-page-chirho` | Your saved private page, the version to save against, and a description of every field, limit and refusal |
| `save_page_chirho` | `POST /app-chirho/perffection-workspace/agent-save-page-chirho` | Saves the whole page as a new **private** version |
| `preview_page_chirho` | `GET /app-chirho/perffection-workspace/preview-chirho` | Your saved page as a visitor would see it once published |
| `read_figures_chirho` | `GET /app-chirho/perffection-workspace/agent-figures-chirho` | Your saved business figures, the report made from them, and how to read every number. Read-only |
| `save_figures_chirho` | `POST /app-chirho/perffection-workspace/agent-save-figures-chirho` | Saves your whole figures document as a new **private** version |

**The figures routes are not on any released server yet.** They are held until their activation is
decided, so today both figures tools answer `tool-route-absent-chirho` (below). They run only in the
figures gates' own scratch staging.

**Reading figures.** Money is in whole minor units (cents), and `null` means unknown, never zero.
`report_chirho.complete_chirho` says whether every kind of work has a figure for every month. When it
is not true, totals and shares cover only the figures you gave, no trend is drawn, and the tool says
so first.

**Saving figures.** `save_figures_chirho` takes the whole `figures_chirho` document that
`read_figures_chirho` returned, changed as you mean, with the `current_version_chirho` you read. It
replaces your figures as a new private version; it is not a patch, and the workspace's one figures
validator decides. While nothing is saved, the read answer's `start_chirho` shows the exact shape with
an illustrative sample, and how the first save is made (against version 0). The sample is never your
figures: your agent should ask you for your real ones, and a save of the sample unchanged is refused
(`sample-not-her-figures-chirho`). A server older than this adapter sends no such example; the tool
then says so plainly, and your agent should not guess a shape. MCP apps see it marked destructive (it replaces your current figures) and
idempotent (an exact repeat of an identical save is answered again, with no second version).

**It never publishes.** Publishing stays your own click, signed in to your workspace. Every save
keeps a new private version. Perffection keeps your most recent versions (16 today): once that
history is full, each save retires the oldest one, which can then no longer be restored. Restore an
earlier version from your workspace while it is kept. So MCP apps see the save tool marked
destructive, and idempotent: an exact repeat of an identical save stores nothing new and retires
nothing.

## Your key

- Put the key in a file only you can open (`chmod 600`) and name that file in
  `PERFFECTION_KEY_FILE_CHIRHO`. `PERFFECTION_KEY_CHIRHO` (the key itself) also works, but a file
  keeps it out of configuration you might share.
- A key file other users of your machine can open is refused, the way ssh refuses a loose private
  key. On Windows, which has no such permission bits, the file is not checked.
- The key travels only as `Authorization: Bearer ...` to the address you configure. Plain `http://`
  is refused except to this machine (127.0.0.1 or localhost), so the key never crosses a network
  unencrypted. Redirects are never followed.
- The key is scrubbed from everything the tools write, even if an answer were to echo it.
- A key can be limited to some of the tools. A tool its key was never granted is refused before it
  runs (below).

## Set it up

You need [Bun](https://bun.sh) or Node.js 22.18 or later (both run TypeScript files directly). There
is nothing to install: the one file `perffection-mcp-chirho.ts` has no dependencies.

Claude Code:

```bash
claude mcp add perffection-chirho \
  -e PERFFECTION_BASE_URL_CHIRHO=https://your-workspace-address \
  -e PERFFECTION_KEY_FILE_CHIRHO=/path/to/your-key.txt \
  -- bun /path/to/perffection-mcp-chirho.ts
```

Any other MCP app (stdio):

```json
{
  "mcpServers": {
    "perffection-chirho": {
      "command": "node",
      "args": ["/path/to/perffection-mcp-chirho.ts"],
      "env": {
        "PERFFECTION_BASE_URL_CHIRHO": "https://your-workspace-address",
        "PERFFECTION_KEY_FILE_CHIRHO": "/path/to/your-key.txt"
      }
    }
  }
}
```

If something is not set up right, every tool says what is wrong and sends nothing.

## Reading the answers

- A complete answer is passed through exactly as Perffection gave it, as structured content.
- Each answer is recognized only as its own kind: a page answer is never read as figures, and a
  read is never taken for a save. A save's result must also be one its codes declare: saved with a
  positive version and its document; stale with a positive head version (read again for the head
  itself); invalid with its issues; or a plain refusal. Anything else is no answer, and the save is
  unknown.
- A refused save (for example a field that is too long) names every problem; nothing was stored.
- If the engine refuses a save before running it with one of its fixed key refusals, exactly (status,
  type and every byte), the result is `refused-before-run-chirho`, naming `refusal_identity_chirho`:
  `key-route-not-open-chirho`, `key-action-not-granted-chirho` or
  `key-action-authority-changed-chirho` (403), or `key-route-changed-chirho` (409). THIS request's
  DAG did not run, so the request changed nothing through it, though the engine may already count it
  as a use of the key. It says nothing about an earlier save whose answer never came: read again
  before saving again. Nothing is retried.
- "Too many requests" (429) has two answers the adapter recognizes, each matched whole: status,
  `application/json`, every byte, and its `Retry-After`. Any other 429 says neither why nor for how
  long.
  - `served-request-unavailable-chirho`: Perffection is busy. Its request gate refused the request
    before any page logic ran, and says how long to wait in exactly one `Retry-After` of whole
    seconds, passed on as `retry_after_seconds_chirho`. The answer stays unknown if that header is
    missing or repeated, if it is not a whole number of seconds from 1 up (no sign, fraction, date or
    leading zero), or if it is larger than a JSON number holds exactly.
  - `rate-limit-exceeded-chirho`: this key's own call budget is used up for now. The answer carries
    no `Retry-After`; one that does is not this answer.
  A save given either is `refused-before-run-chirho`, as above, naming it in
  `refusal_identity_chirho`. A read given either is no answer that names it in `reason_chirho`. Busy
  carries the wait either way. The adapter itself never waits or retries: your agent decides.
- If Perffection gave no complete answer to a save (the connection dropped, a timeout, a key that
  was not accepted, any other refusal), the result is **unknown**: `save_result_chirho` is
  `unknown-chirho`. Read the page or the figures again before saving again. It never guesses "saved"
  or "not saved".
- A 404 carries `reason_chirho`, which claims only what the health route can show:
  - `tool-route-absent-chirho`: something live answers at the address but has no route for that
    tool. It may be a Perffection server without the tool (one older than this adapter), or a
    service that is not Perffection.
  - `address-unconfirmed-chirho`: the health check failed or did not answer. The address may be
    wrong, or the service may be down. Check `PERFFECTION_BASE_URL_CHIRHO`, then try again later.
  A save stays unknown in both cases, and nothing is retried.

## For developers

- It speaks both protocol eras: the stateless 2026-07-28 revision (`server/discover`, per-request
  metadata) and the `initialize` handshake of 2025-11-25 and earlier.
- Every request carries its own fresh caller key in `cairn-idempotency-key-chirho`, a random UUID, exactly
  once. A geo-elected Perffection admits a write only with one. Nothing retries, so a key is never reused. A
  save her agent sends again is a new request against its expected version.
- Bounds: one message line up to 262,144 characters, one answer up to 1 MiB, 30 s per answer,
  sixteen tool calls waiting at most, and calls run one at a time in arrival order.
- Executed with Bun 1.4.0 and Node.js 26.0.0, and with Claude Code 2.1.284 as the client (it used
  the initialize handshake at 2025-11-25).
- Tests: `bun test tools-chirho/agent-mcp-chirho/` (a fake workspace records every request;
  `perffection-mcp-saves-chirho.test.ts` holds the saves' answer kinds and key refusals;
  `perffection-mcp-rate-limits-chirho.test.ts` holds the two 429s and their lookalikes, byte for byte).
  `mcp-stdio-client-chirho.ts` is their small stdio client. The served proof is arm 8b of
  `scripts-chirho/tests-chirho/agent-api-rehearsal-chirho.sh`; arm 8c runs Claude Code itself as the
  client when asked. The figures read's served proof is arm 8b of
  `scripts-chirho/tests-chirho/figures-rehearsal-chirho.sh`; the figures save's, with limited keys,
  is `scripts-chirho/tests-chirho/figures-chirho/key-grants-rehearsal-chirho.sh`. This directory imports nothing from the
  rest of the repository, so it can move or ship as it is; its home is revisited when distribution is
  decided.
- Workflows: `spec-chirho/workflows-chirho/invite-beta-chirho/agent-api-chirho.md` and
  `her-figures-chirho.md` beside it.
