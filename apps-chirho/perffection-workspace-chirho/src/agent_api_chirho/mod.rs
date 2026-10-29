// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Her own agent talks to Perffection directly: the pure layer of the agent API.
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/agent-api-chirho.md`.
//!
//! L.J., 2026-09-20/21: "a person might use Claude Code on their own machine and interact ... via
//! the rest or mcp api"; "we have endpoints for their own agent to describe the surface"; "why
//! didn't you make a REST endpoint?". Two JSON routes of the workspace app speak ONE content
//! format - the page document the product already exports and imports - through the same strict
//! parser, the same validator and the same optimistic write door as her browser. An API save has
//! no reviewer, so nothing is clamped, stripped or repaired: every problem is a refusal that names
//! its field, and nothing is saved.

pub mod agent_page_chirho;
pub mod agent_save_chirho;
pub mod agent_surface_chirho;
