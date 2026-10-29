#!/usr/bin/env bun
// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// Display acceptance over the actual WASM guest, NOT engine signup/session or hosted acceptance.
// Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/signup-chirho.md.
import { mkdir } from "node:fs/promises";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import type { BrowserType, ChromiumBrowser } from "playwright-core";

const root_chirho = resolve(import.meta.dir, "../..");
const output_chirho = resolve(process.env.AUTH_DISPLAY_QA_OUTPUT_CHIRHO ?? `${root_chirho}/target/auth-display-qa-chirho`);
const module_chirho = process.env.PLAYWRIGHT_MODULE_CHIRHO;
if (!module_chirho) throw Error("Set PLAYWRIGHT_MODULE_CHIRHO to an installed Playwright index.mjs; no installation is performed.");
const { chromium: chromium_chirho } = await import(pathToFileURL(module_chirho).href) as { chromium: BrowserType<ChromiumBrowser> };
const renderer_chirho = `${root_chirho}/target/debug/examples/render_auth_native_chirho`;
const guest_chirho = resolve(process.env.AUTH_DISPLAY_GUEST_CHIRHO ?? `${root_chirho}/target/wasm32-unknown-unknown/release/examples/render_auth_guest_chirho.wasm`);
// Exactly what the auth view loads on FIRST LIGHT (2026-10-01): its stylesheet, the icon, the three
// preloaded faces and the night's two star tiles. Anything else it asks for answers 404 here.
const assets_chirho = ["login-chirho.css", "seal-chirho.png", "fell-canon-chirho.woff2", "fell-canon-italic-chirho.woff2",
  "cabin-text-chirho.woff2", "stars-far-chirho.png", "stars-near-chirho.png"];
const csp_chirho = "default-src 'none'; script-src 'none'; object-src 'none'; style-src 'self' 'unsafe-inline'; img-src 'self'; font-src 'self'; form-action 'self'; frame-ancestors 'none'; base-uri 'none'";
const checks_chirho: { check_chirho: string; passed_chirho: boolean }[] = [];
const fuel_chirho: Record<string, number> = {};
const browser_errors_chirho: string[] = [];
await mkdir(output_chirho, { recursive: true });
const result_path_chirho = `${output_chirho}/result-chirho.json`;
await Bun.write(result_path_chirho, JSON.stringify({status_chirho:"running-chirho", at_chirho:new Date().toISOString()}));

function expect_chirho(check_chirho: string, passed_chirho: boolean): void {
  checks_chirho.push({ check_chirho, passed_chirho });
  if (!passed_chirho) throw Error(check_chirho);
}

async function hash_chirho(path_chirho: string): Promise<string> {
  return new Bun.CryptoHasher("sha256").update(await Bun.file(path_chirho).arrayBuffer()).digest("hex");
}

const registration_form_chirho = { action_route_chirho: "/auth-chirho/register-chirho",
  field_names_chirho: ["kind_chirho", "username_chirho", "credential_chirho", "human_check_chirho", "human_check_token_chirho"] };
const login_form_chirho = { action_route_chirho: "/auth-chirho/login-chirho",
  field_names_chirho: ["username_chirho", "credential_chirho", "redirect_chirho"] };
const signup_chirho = {page_kind_chirho:"challenge_chirho",posture_chirho:"registration_open_chirho",
  challenge_chirho:{question_chirho:"2 + 3",token_chirho:"synthetic-once-chirho"},form_chirho:registration_form_chirho};
const login_chirho = {page_kind_chirho:"username_login_chirho",posture_chirho:"sign_in_chirho",form_chirho:login_form_chirho};
const cases_chirho = [
  {name_chirho:"signup-chirho", input_chirho:signup_chirho, form_chirho:registration_form_chirho, rtl_chirho:false},
  {name_chirho:"login-chirho", input_chirho:login_chirho, form_chirho:login_form_chirho, rtl_chirho:false},
  {name_chirho:"invalid-login-chirho", input_chirho:{...login_chirho,page_kind_chirho:"refusal_chirho",refusal_code_chirho:"invalid-credential-chirho"}, form_chirho:login_form_chirho, rtl_chirho:false},
  {name_chirho:"closed-chirho",input_chirho:{page_kind_chirho:"refusal_chirho",posture_chirho:"registration_closed_chirho",refusal_code_chirho:"registration-disabled-chirho"},form_chirho:null,rtl_chirho:false},
  {name_chirho:"full-chirho",input_chirho:{page_kind_chirho:"refusal_chirho",posture_chirho:"registration_full_chirho",refusal_code_chirho:"registration-cohort-full-chirho"},form_chirho:null,rtl_chirho:false},
  {name_chirho:"challenge-refusal-chirho",input_chirho:{page_kind_chirho:"refusal_chirho",posture_chirho:"registration_open_chirho",refusal_code_chirho:"human-check-rejected-chirho"},form_chirho:null,rtl_chirho:false},
  {name_chirho:"success-chirho",input_chirho:{page_kind_chirho:"success_handoff_chirho",posture_chirho:"registration_open_chirho",success_handoff_chirho:{next_route_chirho:"/auth-chirho/login-chirho"}},form_chirho:null,rtl_chirho:false},
  {name_chirho:"signup-rtl-chirho",input_chirho:signup_chirho,form_chirho:registration_form_chirho,rtl_chirho:true},
  {name_chirho:"login-rtl-chirho",input_chirho:login_chirho,form_chirho:login_form_chirho,rtl_chirho:true},
];
const pages_chirho = new Map<string,string>();
const hashes_chirho: Record<string,string> = {};
let browser_chirho: ChromiumBrowser | undefined;
let server_chirho: ReturnType<typeof Bun.serve> | undefined;
let failure_chirho: string | undefined;
try {
  for (const path_chirho of [renderer_chirho,guest_chirho,...assets_chirho.map(asset_chirho=>`${root_chirho}/assets_chirho/${asset_chirho}`)]) {
    hashes_chirho[path_chirho] = await hash_chirho(path_chirho);
  }
  for (const case_chirho of cases_chirho) {
    const flags_chirho = case_chirho.rtl_chirho ? ["--rtl-chirho"] : ["--guest-chirho",guest_chirho];
    const process_chirho = Bun.spawnSync([renderer_chirho,...flags_chirho], {
      stdin:new TextEncoder().encode(JSON.stringify(case_chirho.input_chirho)),stdout:"pipe",stderr:"pipe",
    });
    expect_chirho(`${case_chirho.name_chirho}: compiled renderer succeeds`,process_chirho.exitCode===0);
    if (!case_chirho.rtl_chirho) {
      const match_chirho = new TextDecoder().decode(process_chirho.stderr).match(/auth display guest fuel: (\d+)/);
      expect_chirho(`${case_chirho.name_chirho}: real bounded WASM call`,!!match_chirho && Number(match_chirho[1])<=1_000_000);
      fuel_chirho[case_chirho.name_chirho]=Number(match_chirho![1]);
    }
    pages_chirho.set(`/${case_chirho.name_chirho}`,new TextDecoder().decode(process_chirho.stdout));
  }
  // An armed negative input must fail, not render a persuasive credential form from a foreign route.
  const poisoned_chirho = {...signup_chirho,form_chirho:{...registration_form_chirho,action_route_chirho:"/app-chirho/perffection/"}};
  const refused_chirho = Bun.spawnSync([renderer_chirho,"--guest-chirho",guest_chirho],{stdin:new TextEncoder().encode(JSON.stringify(poisoned_chirho)),stdout:"pipe",stderr:"pipe"});
  expect_chirho("foreign same-origin credential destination traps before HTML",refused_chirho.exitCode!==0 && refused_chirho.stdout.length===0);

  server_chirho=Bun.serve({hostname:"127.0.0.1",port:0,async fetch(request_chirho) {
    const path_chirho=new URL(request_chirho.url).pathname;
    if(request_chirho.method!=="GET") return new Response(null,{status:405});
    const asset_chirho=path_chirho.replace("/app-assets-chirho/perffection/by-name-chirho/","");
    if(path_chirho.startsWith("/app-assets-chirho/perffection/by-name-chirho/") && assets_chirho.includes(asset_chirho)) return new Response(Bun.file(`${root_chirho}/assets_chirho/${asset_chirho}`));
    const page_chirho=pages_chirho.get(path_chirho);
    return new Response(page_chirho??"not found",{status:page_chirho?200:404,headers:{"Content-Type":"text/html; charset=utf-8","Cache-Control":"no-store","Content-Security-Policy":csp_chirho}});
  }});
  const origin_chirho=`http://127.0.0.1:${server_chirho.port}`;
  browser_chirho=await chromium_chirho.launch({channel:"chrome",headless:true});
  const page_chirho=await browser_chirho.newPage({reducedMotion:"reduce"});
  page_chirho.on("pageerror",error_chirho=>browser_errors_chirho.push(error_chirho.message));
  page_chirho.on("console",message_chirho=>{if(message_chirho.type()==="error") browser_errors_chirho.push(message_chirho.text());});
  for(const case_chirho of cases_chirho) {
    for(const width_chirho of [320,390,768,1440]) {
      const label_chirho=`${case_chirho.name_chirho}/${width_chirho}`;
      await page_chirho.setViewportSize({width:width_chirho,height:900});
      await page_chirho.goto(`${origin_chirho}/${case_chirho.name_chirho}`);
      await page_chirho.evaluate(()=>document.fonts.ready);
      if(case_chirho.rtl_chirho) await page_chirho.evaluate(()=>{document.documentElement.style.fontSize="200%";});
      const dom_chirho=await page_chirho.evaluate(()=>({
        width_chirho:document.documentElement.clientWidth, scroll_chirho:document.documentElement.scrollWidth,
        headings_chirho:document.querySelectorAll("h1").length, nav_chirho:document.querySelectorAll("header nav").length,
        styled_chirho:getComputedStyle(document.querySelector("main")!).display==="grid",
        dir_chirho:document.documentElement.dir,
        forms_chirho:[...document.forms].map(form_chirho=>({action_chirho:form_chirho.getAttribute("action"),method_chirho:form_chirho.method,
          fields_chirho:[...form_chirho.querySelectorAll<HTMLInputElement>("input")].map(input_chirho=>({name_chirho:input_chirho.name,type_chirho:input_chirho.type,value_chirho:input_chirho.type==="hidden"?undefined:input_chirho.value,
            labelled_chirho:input_chirho.type==="hidden" || !!input_chirho.labels?.length,
            help_chirho:input_chirho.type==="hidden" || (input_chirho.getAttribute("aria-describedby")??"").split(" ").every(id_chirho=>!!document.getElementById(id_chirho))}))})),
        scripts_chirho:document.scripts.length,
        legacy_chirho:!!document.querySelector('[name="principal_chirho"], [name="registration_token_chirho"]'),
      }));
      expect_chirho(`${label_chirho}: shared styled shell and single heading`,dom_chirho.headings_chirho===1 && dom_chirho.nav_chirho===1 && dom_chirho.styled_chirho);
      expect_chirho(`${label_chirho}: reflow without horizontal scroll`,dom_chirho.width_chirho===dom_chirho.scroll_chirho);
      expect_chirho(`${label_chirho}: locale direction`,dom_chirho.dir_chirho===(case_chirho.rtl_chirho?"rtl":"ltr"));
      expect_chirho(`${label_chirho}: no script or legacy identity control`,dom_chirho.scripts_chirho===0 && !dom_chirho.legacy_chirho);
      expect_chirho(`${label_chirho}: only available form`,dom_chirho.forms_chirho.length===(case_chirho.form_chirho?1:0));
      if(case_chirho.form_chirho) {
        const form_chirho=dom_chirho.forms_chirho[0];
        expect_chirho(`${label_chirho}: fixed direct POST and exact protocol field set`,form_chirho.method_chirho==="post" && form_chirho.action_chirho===case_chirho.form_chirho.action_route_chirho && JSON.stringify(form_chirho.fields_chirho.map(field_chirho=>field_chirho.name_chirho).sort())===JSON.stringify([...case_chirho.form_chirho.field_names_chirho].sort()));
        expect_chirho(`${label_chirho}: labelled blank fields with help`,form_chirho.fields_chirho.every(field_chirho=>field_chirho.labelled_chirho && field_chirho.help_chirho && (field_chirho.type_chirho==="hidden" || field_chirho.value_chirho==="")));
      }
      if(width_chirho===390) await page_chirho.screenshot({path:`${output_chirho}/${case_chirho.name_chirho}.png`,fullPage:true});
    }
  }
  await page_chirho.goto(`${origin_chirho}/login-chirho#refusal-invalid-credential-chirho`);
  expect_chirho("one readable fixed-fragment login refusal",await page_chirho.locator('[role="alert"]:visible').count()===1);
  await page_chirho.goto(`${origin_chirho}/signup-chirho`);
  await page_chirho.keyboard.press("Tab");
  expect_chirho("keyboard reaches skip link first",await page_chirho.evaluate(()=>document.activeElement?.classList.contains("login-skip-chirho")===true));
  await page_chirho.keyboard.press("Enter");
  expect_chirho("skip link focuses the main task",await page_chirho.evaluate(()=>document.activeElement?.tagName==="MAIN"));
  expect_chirho("no browser or CSP console errors",browser_errors_chirho.length===0);
  for(const [path_chirho,before_chirho] of Object.entries(hashes_chirho)) expect_chirho(`consumed input unchanged: ${path_chirho.replace(root_chirho+"/","")}`,await hash_chirho(path_chirho)===before_chirho);
} catch(error_chirho) {
  failure_chirho=error_chirho instanceof Error?error_chirho.message:"unexpected auth display check failure";
} finally {
  await browser_chirho?.close();
  server_chirho?.stop(true);
  await Bun.write(result_path_chirho,JSON.stringify({status_chirho:failure_chirho?"failed-chirho":"passed-chirho",at_chirho:new Date().toISOString(),
    scope_chirho:"Actual WASM display with proposed auth CSP; RTL is native template QA. Not an engine signup/session journey, hosted proof or activation.",
    checks_chirho,fuel_chirho,hashes_chirho,browser_errors_chirho,...(failure_chirho?{failure_chirho}:{})},null,2)+"\n");
}
console.log(`${checks_chirho.filter(check_chirho=>check_chirho.passed_chirho).length}/${checks_chirho.length} display checks; ${result_path_chirho}`);
if(failure_chirho) throw Error(failure_chirho);
