// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A LOCAL development receiver for Stripe TEST events: the Stripe CLI forwards to it (`stripe listen
//! --forward-to http://127.0.0.1:<port>/stripe-webhook-chirho`), and every event runs through the real intake against
//! the stub top-up port, so real Stripe signatures and real payloads prove the intake end to end. It binds 127.0.0.1
//! only, caps a request at 1 MiB, speaks just enough HTTP/1.1 for the CLI, and is never a production webhook (that
//! stays closed until L.J.'s word). It prints one line per event: the outcome and the event id, never a secret.
//! Workflow: spec-chirho/workflows-chirho/selling-chirho/credit-purchase-chirho.md
//!
//! usage: STRIPE_WEBHOOK_SECRET_CHIRHO=whsec_... cargo run -p perffection-credits-chirho --example local_receiver_chirho -- \
//!        <port> <pack id> <amount in minor units> <currency> <credits> [events before exit]

use perffection_credits_chirho::intake_chirho::{intake_stripe_event_chirho, IntakeConfigChirho};
use perffection_credits_chirho::pack_catalog_chirho::{CreditPackChirho, PackCatalogChirho};
use perffection_credits_chirho::stripe_signature_chirho::DEFAULT_TOLERANCE_SECONDS_CHIRHO;
use perffection_credits_chirho::topup_port_chirho::StubTopupPortChirho;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_BODY_BYTES_CHIRHO: usize = 1 << 20;
const PATH_CHIRHO: &str = "/stripe-webhook-chirho";

fn main() {
    let args_chirho: Vec<String> = std::env::args().collect();
    if args_chirho.len() < 6 {
        eprintln!(
            "usage: <port> <pack id> <amount minor> <currency> <credits> [events before exit]"
        );
        std::process::exit(2);
    }
    let secret_chirho = std::env::var("STRIPE_WEBHOOK_SECRET_CHIRHO")
        .expect("STRIPE_WEBHOOK_SECRET_CHIRHO (whsec_ from the Stripe CLI)");
    let port_chirho: u16 = args_chirho[1].parse().expect("port");
    let catalog_chirho = PackCatalogChirho::default().with_pack_chirho(
        &args_chirho[2],
        CreditPackChirho {
            amount_minor_chirho: args_chirho[3].parse().expect("amount"),
            currency_chirho: args_chirho[4].clone(),
            credits_chirho: args_chirho[5].parse().expect("credits"),
        },
    );
    let limit_chirho: Option<usize> = args_chirho
        .get(6)
        .map(|value_chirho| value_chirho.parse().expect("events"));
    let listener_chirho = TcpListener::bind(("127.0.0.1", port_chirho)).expect("bind 127.0.0.1");
    println!("listening 127.0.0.1:{port_chirho}{PATH_CHIRHO}");
    let mut port_state_chirho = StubTopupPortChirho::default();
    let mut handled_chirho = 0usize;
    for stream_chirho in listener_chirho.incoming() {
        let Ok(mut stream_chirho) = stream_chirho else {
            continue;
        };
        let (status_chirho, line_chirho) = match read_request_chirho(&mut stream_chirho) {
            Ok((path_chirho, signature_chirho, body_chirho)) if path_chirho == PATH_CHIRHO => {
                let now_chirho = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d_chirho| d_chirho.as_secs())
                    .unwrap_or(0);
                let config_chirho = IntakeConfigChirho {
                    signing_secret_chirho: &secret_chirho,
                    tolerance_seconds_chirho: DEFAULT_TOLERANCE_SECONDS_CHIRHO,
                    catalog_chirho: &catalog_chirho,
                };
                let event_id_chirho = event_id_chirho(&body_chirho);
                match intake_stripe_event_chirho(
                    &body_chirho,
                    &signature_chirho,
                    now_chirho,
                    &config_chirho,
                    &mut port_state_chirho,
                ) {
                    Ok(outcome_chirho) => (
                        "200 OK",
                        format!(
                            "{event_id_chirho} {outcome_chirho:?} (account total {})",
                            port_state_chirho.credits_for_account_chirho("account-test-chirho")
                        ),
                    ),
                    Err(refusal_chirho) => (
                        "400 Bad Request",
                        format!("{event_id_chirho} REFUSED {refusal_chirho:?}"),
                    ),
                }
            }
            Ok((path_chirho, _, _)) => ("404 Not Found", format!("unknown path {path_chirho}")),
            Err(problem_chirho) => ("400 Bad Request", format!("bad request: {problem_chirho}")),
        };
        println!("{line_chirho}");
        let _ = stream_chirho.write_all(
            format!("HTTP/1.1 {status_chirho}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
                .as_bytes(),
        );
        handled_chirho += 1;
        if limit_chirho.is_some_and(|limit_chirho| handled_chirho >= limit_chirho) {
            break;
        }
    }
}

/// The request path, the Stripe-Signature header and the exact body bytes (verified as received).
fn read_request_chirho(
    stream_chirho: &mut std::net::TcpStream,
) -> Result<(String, String, Vec<u8>), String> {
    let mut reader_chirho = BufReader::new(stream_chirho);
    let mut request_line_chirho = String::new();
    reader_chirho
        .read_line(&mut request_line_chirho)
        .map_err(|e_chirho| e_chirho.to_string())?;
    let mut parts_chirho = request_line_chirho.split_whitespace();
    if parts_chirho.next() != Some("POST") {
        return Err("only POST".into());
    }
    let path_chirho = parts_chirho.next().unwrap_or_default().to_string();
    let (mut length_chirho, mut signature_chirho) = (0usize, String::new());
    loop {
        let mut header_chirho = String::new();
        reader_chirho
            .read_line(&mut header_chirho)
            .map_err(|e_chirho| e_chirho.to_string())?;
        let header_chirho = header_chirho.trim_end();
        if header_chirho.is_empty() {
            break;
        }
        if let Some((name_chirho, value_chirho)) = header_chirho.split_once(':') {
            match name_chirho.trim().to_ascii_lowercase().as_str() {
                "content-length" => {
                    length_chirho = value_chirho.trim().parse().map_err(|_| "bad length")?
                }
                "stripe-signature" => signature_chirho = value_chirho.trim().to_string(),
                _ => {}
            }
        }
    }
    if length_chirho > MAX_BODY_BYTES_CHIRHO {
        return Err("body too large".into());
    }
    let mut body_chirho = vec![0u8; length_chirho];
    reader_chirho
        .read_exact(&mut body_chirho)
        .map_err(|e_chirho| e_chirho.to_string())?;
    Ok((path_chirho, signature_chirho, body_chirho))
}

fn event_id_chirho(body_chirho: &[u8]) -> String {
    serde_json::from_slice::<serde_json::Value>(body_chirho)
        .ok()
        .and_then(|value_chirho| {
            value_chirho
                .get("id")
                .and_then(|id_chirho| id_chirho.as_str())
                .map(str::to_string)
        })
        .unwrap_or_else(|| "(no id)".into())
}
