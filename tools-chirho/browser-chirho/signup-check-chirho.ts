#!/usr/bin/env bun
// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/signup-chirho.md.
// Real HTTP consumer proof on an owned loopback root. No CLI-created User, build, host or provider.
import { chmod, mkdir, mkdtemp, rm, unlink } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { SignupFixtureChirho } from "./signup-fixture-chirho";
import { write_private_chirho } from "../browser-flows-chirho/private-file-chirho";

interface HttpResultChirho {
  status_chirho: number;
  headers_chirho: Headers;
  body_chirho: Record<string, unknown>;
}
interface UserChirho { username_chirho: string; credential_chirho: string }
interface ChallengeChirho { human_check_token_chirho: string; human_check_chirho: string }

const product_chirho = resolve(import.meta.dir, "../..");
const engine_chirho = resolve(process.env.CAIRN_REPO_CHIRHO ?? join(product_chirho, "../cairn-opensource-chirho"));
const output_chirho = resolve(process.env.SIGNUP_QA_OUTPUT_CHIRHO ?? join(product_chirho, "target/signup-qa-chirho"));
const closed_only_chirho = process.argv.includes("--closed-control-chirho");
if (process.argv.slice(2).some(argument_chirho => argument_chirho !== "--closed-control-chirho")) throw Error("Unknown signup check argument");
const port_chirho = Number(process.env.SIGNUP_QA_PORT_CHIRHO ?? "18974");
if (!Number.isInteger(port_chirho) || port_chirho < 1024 || port_chirho > 65535) throw Error("Invalid SIGNUP_QA_PORT_CHIRHO");
await mkdir(output_chirho, { recursive: true });
const scratch_chirho = await mkdtemp(join(tmpdir(), "perffection-signup-chirho-"));
await chmod(scratch_chirho, 0o700);
const fixture_chirho = new SignupFixtureChirho(product_chirho, engine_chirho, scratch_chirho, port_chirho);
const observations_chirho: Record<string, unknown> = {};
const findings_chirho: string[] = [];
let phase_chirho = "prepare-owned-root-chirho";
let failure_chirho: string | undefined;
const check_chirho = (condition_chirho: unknown, label_chirho: string): void => fixture_chirho.check_chirho(condition_chirho, label_chirho);

async function evidence_chirho(result_chirho: string): Promise<void> {
  const bytes_chirho = JSON.stringify({ result_chirho, at_chirho: new Date().toISOString(), phase_chirho,
    scope_chirho: closed_only_chirho ? "closed-registration-control-only-chirho" : "local-http-private-username-signup-chirho",
    failure_chirho, findings_chirho, observations_chirho, checks_chirho: fixture_chirho.checks_chirho,
    artifacts_chirho: fixture_chirho.artifacts_chirho,
    limits_chirho: ["Not branded-browser or hosted acceptance", "Synthetic arithmetic challenge is not production anti-abuse evidence",
      "No public activation, provider, spend or real customer", "Plaintext scan is not a proof against cryptographic inference"],
  }, null, 2) + "\n";
  if (fixture_chirho.redact_chirho(bytes_chirho) !== bytes_chirho) throw Error("Private fixture bytes in proposed evidence");
  await Bun.write(join(output_chirho, "signup-result-chirho.json"), bytes_chirho);
}

function user_chirho(): UserChirho {
  const value_chirho = { username_chirho: `member_${crypto.randomUUID().replaceAll("-", "")}`, credential_chirho: crypto.randomUUID() + crypto.randomUUID() };
  fixture_chirho.private_values_chirho.push(value_chirho.username_chirho, value_chirho.username_chirho.toUpperCase(), value_chirho.credential_chirho);
  return value_chirho;
}

async function request_chirho(path_chirho: string, body_chirho?: Record<string, string>, form_chirho = false): Promise<HttpResultChirho> {
  const response_chirho = await fetch(`${fixture_chirho.origin_chirho}${path_chirho}`, {
    method: body_chirho ? "POST" : "GET", redirect: "manual", signal: AbortSignal.timeout(15_000),
    headers: { accept: "application/json", ...(body_chirho ? { "content-type": form_chirho ? "application/x-www-form-urlencoded" : "application/json" } : {}) },
    body: body_chirho ? form_chirho ? new URLSearchParams(body_chirho) : JSON.stringify(body_chirho) : undefined,
  });
  const text_chirho = await response_chirho.text();
  let parsed_chirho: Record<string, unknown> = {};
  if (response_chirho.headers.get("content-type")?.includes("application/json")) {
    const value_chirho: unknown = JSON.parse(text_chirho);
    if (!value_chirho || typeof value_chirho !== "object" || Array.isArray(value_chirho)) throw Error("Auth JSON was not an object");
    parsed_chirho = value_chirho as Record<string, unknown>;
  }
  for (const key_chirho of ["principal_chirho", "token_chirho", "session_token_chirho"]) {
    if (typeof parsed_chirho[key_chirho] === "string") fixture_chirho.private_values_chirho.push(parsed_chirho[key_chirho]);
  }
  return { status_chirho: response_chirho.status, headers_chirho: response_chirho.headers, body_chirho: parsed_chirho };
}

function refused_chirho(response_chirho: HttpResultChirho, code_chirho: string, label_chirho: string): void {
  check_chirho(response_chirho.status_chirho >= 400 && response_chirho.body_chirho.ok_chirho === false
    && response_chirho.body_chirho.code_chirho === code_chirho && !response_chirho.headers_chirho.has("set-cookie"), label_chirho);
}

async function challenge_chirho(): Promise<ChallengeChirho> {
  const response_chirho = await request_chirho("/auth-chirho/human-check-chirho");
  check_chirho(response_chirho.status_chirho === 200 && response_chirho.body_chirho.ok_chirho === true,
    "open configured registration issues a challenge without an invitation token");
  check_chirho(/(?:^|,)\s*no-store\b/i.test(response_chirho.headers_chirho.get("cache-control") ?? ""), "single-use challenge response prohibits storage");
  const token_chirho = response_chirho.body_chirho.human_check_token_chirho;
  const question_chirho = response_chirho.body_chirho.question_chirho;
  if (typeof token_chirho !== "string" || typeof question_chirho !== "string") throw Error("Missing challenge fields");
  const parts_chirho = /^\s*(\d+)\s*\+\s*(\d+)\s*$/.exec(question_chirho);
  if (!parts_chirho) throw Error("This synthetic runner only knows the engine arithmetic challenge");
  fixture_chirho.private_values_chirho.push(token_chirho);
  return { human_check_token_chirho: token_chirho, human_check_chirho: String(Number(parts_chirho[1]) + Number(parts_chirho[2])) };
}

const register_chirho = (account_chirho: UserChirho, proof_chirho: ChallengeChirho): Promise<HttpResultChirho> =>
  request_chirho("/auth-chirho/register-chirho", { kind_chirho: "user", ...account_chirho, ...proof_chirho });

async function login_chirho(account_chirho: UserChirho): Promise<string> {
  // Never uses the principal returned by registration: the customer only knows her chosen name.
  const response_chirho = await request_chirho("/auth-chirho/login-chirho", {
    ...account_chirho, redirect_chirho: `/app-chirho/${fixture_chirho.app_chirho}/`,
  }, true);
  check_chirho(response_chirho.status_chirho === 303 && response_chirho.headers_chirho.get("location") === `/app-chirho/${fixture_chirho.app_chirho}/`, "chosen username and password reach the workspace redirect");
  const header_chirho = response_chirho.headers_chirho.get("set-cookie") ?? "";
  check_chirho(header_chirho.startsWith("cairn_session_chirho=") && /;\s*HttpOnly/i.test(header_chirho)
    && /;\s*Secure/i.test(header_chirho) && /;\s*SameSite=Lax/i.test(header_chirho), "session cookie retains the browser security attributes");
  const cookie_chirho = header_chirho.split(";")[0];
  fixture_chirho.private_values_chirho.push(cookie_chirho, cookie_chirho.slice(cookie_chirho.indexOf("=") + 1));
  return cookie_chirho;
}

async function editor_chirho(cookie_chirho: string, status_chirho = 200): Promise<string> {
  const response_chirho = await fetch(fixture_chirho.workspace_chirho, { headers: { cookie: cookie_chirho }, signal: AbortSignal.timeout(15_000) });
  check_chirho(response_chirho.status === status_chirho, `workspace access answers ${status_chirho}`);
  const html_chirho = await response_chirho.text();
  if (status_chirho === 200) {
    check_chirho(/<form\b[^>]*action="[^"]*edit-page-chirho"/.test(html_chirho), "authenticated response is the actual page editor");
    check_chirho(fixture_chirho.redact_chirho(html_chirho) === html_chirho, "editor does not expose synthetic account credentials or identity markers");
  }
  return html_chirho;
}

async function login_budget_chirho(known_chirho: UserChirho, repeated_unknown_chirho: UserChirho, fresh_unknown_chirho: UserChirho): Promise<void> {
  // Match failure history, not just one cold response. A shared unknown-name bucket can otherwise
  // reveal whether another name exists after an unrelated typo exhausts that shared bucket.
  phase_chirho = "login-budget-isolation-chirho";
  let exhausted_chirho = false;
  let attempts_chirho = 0;
  for (; attempts_chirho < 16; attempts_chirho++) {
    const response_chirho = await request_chirho("/auth-chirho/login-chirho", { ...repeated_unknown_chirho });
    if (response_chirho.body_chirho.code_chirho === "auth-rate-limited-chirho") {
      refused_chirho(response_chirho, "auth-rate-limited-chirho", "repeated unknown-name failures reach an enforced budget");
      exhausted_chirho = true;
      attempts_chirho++;
      break;
    }
    refused_chirho(response_chirho, "invalid-credential-chirho", "unknown-name failure before exhaustion stays non-enumerating");
  }
  check_chirho(exhausted_chirho, "bounded fixture reaches the login failure ceiling");
  const unknown_chirho = await request_chirho("/auth-chirho/login-chirho", { ...fresh_unknown_chirho });
  const known_failure_chirho = await request_chirho("/auth-chirho/login-chirho", { ...known_chirho, credential_chirho: crypto.randomUUID() });
  observations_chirho.login_budget_chirho = { attempts_to_limit_chirho: attempts_chirho,
    fresh_unknown_status_chirho: unknown_chirho.status_chirho, fresh_unknown_code_chirho: unknown_chirho.body_chirho.code_chirho,
    known_wrong_status_chirho: known_failure_chirho.status_chirho, known_wrong_code_chirho: known_failure_chirho.body_chirho.code_chirho };
  refused_chirho(known_failure_chirho, "invalid-credential-chirho", "an unrelated name budget does not exhaust an existing account");
  check_chirho(!unknown_chirho.headers_chirho.has("set-cookie"), "fresh unknown-name refusal creates no session");
  if (unknown_chirho.status_chirho === known_failure_chirho.status_chirho
    && JSON.stringify(unknown_chirho.body_chirho) === JSON.stringify(known_failure_chirho.body_chirho)) {
    check_chirho(true, "an exhausted unrelated name cannot distinguish a fresh unknown name from a wrong credential");
  } else findings_chirho.push("Exhausting an unknown username changes a fresh unknown-name refusal but not an existing-name refusal");
  await editor_chirho(await login_chirho(known_chirho));
}

async function full_journey_chirho(): Promise<void> {
  phase_chirho = "open-registration-chirho";
  await fixture_chirho.start_chirho(true, 3);
  const account_a_chirho = user_chirho();
  const account_b_chirho = user_chirho();
  const account_c_chirho = user_chirho();
  const account_d_chirho = user_chirho();
  const account_e_chirho = user_chirho();
  const proof_a_chirho = await challenge_chirho();
  const short_chirho = await register_chirho({ ...account_a_chirho, credential_chirho: "short" }, proof_a_chirho);
  refused_chirho(short_chirho, "request-malformed-chirho", "short credential refuses without creating an account");
  const first_chirho = await register_chirho(account_a_chirho, proof_a_chirho);
  check_chirho(first_chirho.status_chirho === 200 && first_chirho.body_chirho.ok_chirho === true, "HTTP signup succeeds and pre-mint credential rejection did not burn its challenge");
  refused_chirho(await register_chirho(account_d_chirho, proof_a_chirho), "human-check-rejected-chirho", "a consumed challenge cannot create another User");
  phase_chirho = "username-session-and-saved-page-chirho";
  const cookie_a_chirho = await login_chirho({ ...account_a_chirho, username_chirho: account_a_chirho.username_chirho.toUpperCase() });
  const first_editor_chirho = await editor_chirho(cookie_a_chirho);
  const fields_chirho = new URLSearchParams();
  await new HTMLRewriter().on('form[action$="edit-page-chirho"] input, form[action$="edit-page-chirho"] textarea', {
    element(element_chirho) {
      const name_chirho = element_chirho.getAttribute("name");
      const value_chirho = element_chirho.getAttribute("value");
      if (name_chirho) fields_chirho.set(name_chirho, value_chirho ?? "");
    },
  }).transform(new Response(first_editor_chirho)).text();
  check_chirho(fields_chirho.has("schema_version_chirho") && fields_chirho.get("expected_version_chirho") === "0", "save uses the schema and empty-head version from the served form");
  const page_marker_chirho = `Synthetic page ${crypto.randomUUID()}`;
  for (const [field_chirho, value_chirho] of Object.entries({ business_name_chirho: "Synthetic bakery", headline_chirho: page_marker_chirho,
    summary_chirho: "A locally tested bakery page.", service_1_title_chirho: "Bread", service_1_body_chirho: "Freshly baked.",
    contact_heading_chirho: "Get in touch", contact_phone_chirho: "+1 202 555 0147" })) fields_chirho.set(field_chirho, value_chirho);
  const saved_chirho = await fetch(`${fixture_chirho.workspace_chirho}edit-page-chirho`, {
    method: "POST", headers: { cookie: cookie_a_chirho, "content-type": "application/x-www-form-urlencoded" }, signal: AbortSignal.timeout(15_000),
    body: fields_chirho,
  });
  const save_body_chirho = await saved_chirho.text();
  observations_chirho.save_response_chirho = { status_chirho: saved_chirho.status, bytes_chirho: Buffer.byteLength(save_body_chirho),
    has_submitted_words_chirho: save_body_chirho.includes(page_marker_chirho),
    engine_refusal_chirho: save_body_chirho === "error: CairnAppExecutionRefusedChirho\n" };
  check_chirho(saved_chirho.status === 200 && save_body_chirho.includes(page_marker_chirho), "self-created account saves its private page through the real product route");
  const logout_chirho = await fetch(`${fixture_chirho.origin_chirho}/auth-chirho/logout-chirho`, {
    headers: { cookie: cookie_a_chirho }, redirect: "manual", signal: AbortSignal.timeout(15_000),
  });
  check_chirho(logout_chirho.status === 303 && /Max-Age=0/i.test(logout_chirho.headers.get("set-cookie") ?? ""), "logout redirects and clears the cookie");
  await editor_chirho(cookie_a_chirho, 403);
  await editor_chirho("", 401);
  const repeat_cookie_chirho = await login_chirho(account_a_chirho);
  check_chirho((await editor_chirho(repeat_cookie_chirho)).includes(page_marker_chirho), "repeat username sign-in retains the customer's saved page");

  phase_chirho = "non-enumerating-login-chirho";
  const wrong_chirho = await request_chirho("/auth-chirho/login-chirho", { ...account_a_chirho, credential_chirho: crypto.randomUUID() });
  const unknown_chirho = await request_chirho("/auth-chirho/login-chirho", { ...account_d_chirho });
  refused_chirho(wrong_chirho, "invalid-credential-chirho", "wrong credential has the stable non-enumerating refusal");
  refused_chirho(unknown_chirho, "invalid-credential-chirho", "unknown username has the stable non-enumerating refusal");
  check_chirho(wrong_chirho.status_chirho === unknown_chirho.status_chirho
    && JSON.stringify(wrong_chirho.body_chirho) === JSON.stringify(unknown_chirho.body_chirho), "unknown username and wrong credential expose the same JSON response");

  phase_chirho = "concurrent-name-and-capacity-chirho";
  const proof_b1_chirho = await challenge_chirho();
  const proof_b2_chirho = await challenge_chirho();
  const results_chirho = await Promise.all([register_chirho(account_b_chirho, proof_b1_chirho), register_chirho(account_b_chirho, proof_b2_chirho)]);
  check_chirho(results_chirho.filter(result_chirho => result_chirho.status_chirho === 200 && result_chirho.body_chirho.ok_chirho === true).length === 1,
    "two simultaneous requests for one username create only one usable account");
  const cookie_b_chirho = await login_chirho(account_b_chirho);
  check_chirho(!(await editor_chirho(cookie_b_chirho)).includes(page_marker_chirho), "another self-created account cannot read the first customer's saved page");
  const proof_c_chirho = await challenge_chirho();
  const proof_d_chirho = await challenge_chirho();
  const third_chirho = await register_chirho(account_c_chirho, proof_c_chirho);
  check_chirho(third_chirho.status_chirho === 200 && third_chirho.body_chirho.ok_chirho === true,
    "remaining capacity accepts a distinct username after this duplicate-name attempt");
  const capacity_chirho = await request_chirho("/auth-chirho/human-check-chirho");
  observations_chirho.capacity_challenge_chirho = { status_chirho: capacity_chirho.status_chirho,
    code_chirho: capacity_chirho.body_chirho.code_chirho ?? null, issued_challenge_chirho: typeof capacity_chirho.body_chirho.human_check_token_chirho === "string" };
  const closed_at_capacity_chirho = capacity_chirho.status_chirho >= 400 && capacity_chirho.body_chirho.ok_chirho === false
    && capacity_chirho.body_chirho.code_chirho === "registration-cohort-full-chirho"
    && !capacity_chirho.body_chirho.human_check_token_chirho && !capacity_chirho.headers_chirho.has("set-cookie");
  if (closed_at_capacity_chirho) check_chirho(true, "challenge issuance closes at the configured User ceiling");
  else findings_chirho.push("Challenge issuance is not closed with a typed capacity refusal after the User ceiling is reached");
  const capacity_form_chirho = await fetch(`${fixture_chirho.origin_chirho}/auth-chirho/register-chirho`, {
    headers: { accept: "text/html" }, redirect: "manual", signal: AbortSignal.timeout(15_000),
  });
  let has_challenge_field_chirho = false;
  await new HTMLRewriter().on('input[name="human_check_token_chirho"]', {
    element() { has_challenge_field_chirho = true; },
  }).transform(capacity_form_chirho).text();
  observations_chirho.capacity_form_chirho = { status_chirho: capacity_form_chirho.status, has_challenge_field_chirho };
  if (capacity_form_chirho.status >= 400 && !has_challenge_field_chirho && !capacity_form_chirho.headers.has("set-cookie")) {
    check_chirho(true, "HTML registration does not issue a challenge after the User ceiling is reached");
  } else findings_chirho.push("HTML registration still offers a challenge after the User ceiling is reached");
  refused_chirho(await register_chirho(account_d_chirho, proof_d_chirho), "registration-cohort-full-chirho", "pre-issued challenge cannot overshoot the in-lock User ceiling");
  await login_chirho(account_c_chirho);
  check_chirho((await editor_chirho(await login_chirho(account_a_chirho))).includes(page_marker_chirho), "capacity closure does not close existing-account sign-in or discard saved content");
  await login_budget_chirho(account_c_chirho, account_d_chirho, account_e_chirho);

  phase_chirho = "stored-name-privacy-chirho";
  await fixture_chirho.stop_chirho();
  const markers_chirho = [account_a_chirho, account_b_chirho, account_c_chirho, account_d_chirho, account_e_chirho]
    .flatMap(account_chirho => [account_chirho.username_chirho, account_chirho.username_chirho.toUpperCase(), account_chirho.credential_chirho]);
  const control_chirho = join(fixture_chirho.root_chirho, "privacy-positive-control-chirho.txt");
  await write_private_chirho(control_chirho, account_a_chirho.username_chirho);
  check_chirho(await fixture_chirho.leaked_markers_chirho(markers_chirho), "plaintext scanner detects an intentionally planted username");
  await unlink(control_chirho);
  check_chirho(!await fixture_chirho.leaked_markers_chirho(markers_chirho), "fresh runtime root contains no plaintext fixture username or password markers");
  observations_chirho.root_scan_chirho = fixture_chirho.last_scan_chirho;
}

try {
  await evidence_chirho("running-chirho");
  await fixture_chirho.initialize_signup_chirho();
  observations_chirho.engine_version_chirho = fixture_chirho.command_chirho([fixture_chirho.cli_chirho, "--version"], "engine version");
  observations_chirho.product_head_chirho = fixture_chirho.command_chirho(["git", "rev-parse", "HEAD"], "product version");
  observations_chirho.candidate_commit_chirho = process.env.SIGNUP_QA_ENGINE_COMMIT_CHIRHO ?? null;
  phase_chirho = "closed-registration-control-chirho";
  await fixture_chirho.start_chirho(false, 3);
  for (const [path_chirho, body_chirho] of [
    ["/auth-chirho/human-check-chirho", undefined], ["/auth-chirho/register-chirho", undefined], ["/auth-chirho/register-chirho", {}],
  ] as const) {
    check_chirho((await request_chirho(path_chirho, body_chirho)).status_chirho === 503, `disabled registration refuses ${body_chirho ? "POST" : "GET"} ${path_chirho}`);
  }
  await editor_chirho("", 401);
  if (!closed_only_chirho) await full_journey_chirho();
  if (findings_chirho.length > 0) failure_chirho = findings_chirho.join("; ");
} catch (error_chirho) {
  failure_chirho = fixture_chirho.redact_chirho(error_chirho instanceof Error ? error_chirho.message : "Unexpected signup checker failure");
} finally {
  await fixture_chirho.stop_chirho();
  try {
    try {
      await fixture_chirho.verify_inputs_chirho();
    } catch (error_chirho) {
      const input_failure_chirho = fixture_chirho.redact_chirho(error_chirho instanceof Error ? error_chirho.message : "Input verification failed");
      failure_chirho = [failure_chirho, input_failure_chirho].filter(Boolean).join("; ");
    }
    await evidence_chirho(failure_chirho ? "failed-chirho" : "passed-chirho");
  } finally {
    await rm(scratch_chirho, { recursive: true, force: true });
  }
}
console.log(JSON.stringify({ result_chirho: failure_chirho ? "failed-chirho" : "passed-chirho", phase_chirho,
  checks_chirho: fixture_chirho.checks_chirho.length, failure_chirho }));
process.exitCode = failure_chirho ? 1 : 0;
