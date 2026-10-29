// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Compile editor-friendly HTML directly, using the same convention as the workspace and page app.
//! Workflow: spec-chirho/workflows-chirho/product-delivery-chirho/product-authoring-chirho.md.

use crate::types_chirho::{
    agent_guide_page_v1_chirho::AgentGuidePageV1Chirho, auth_page_v1_chirho::AuthPageV1Chirho,
    craft_page_v1_chirho::CraftPageV1Chirho, help_page_v1_chirho::HelpPageV1Chirho,
    home_page_v1_chirho::HomePageV1Chirho, login_page_v1_chirho::LoginPageV1Chirho,
    setup_page_v1_chirho::SetupPageV1Chirho, trust_page_v1_chirho::TrustPageV1Chirho,
};
use askama::Template;
use cairn_node_templating_chirho::HtmlPartialV1Chirho;

#[derive(Template)]
#[template(path = "pages_chirho/home_chirho.html", escape = "html")]
struct HomeTemplateChirho<'model_chirho> {
    model_chirho: &'model_chirho HomePageV1Chirho,
}

#[derive(Template)]
#[template(path = "pages_chirho/craft_chirho.html", escape = "html")]
struct CraftTemplateChirho<'model_chirho> {
    model_chirho: &'model_chirho CraftPageV1Chirho,
}

#[derive(Template)]
#[template(path = "pages_chirho/trust_chirho.html", escape = "html")]
struct TrustTemplateChirho<'model_chirho> {
    model_chirho: &'model_chirho TrustPageV1Chirho,
}

#[derive(Template)]
#[template(path = "pages_chirho/login_chirho.html", escape = "html")]
struct LoginTemplateChirho<'model_chirho> {
    model_chirho: &'model_chirho LoginPageV1Chirho,
}

#[derive(Template)]
#[template(path = "pages_chirho/setup_chirho.html", escape = "html")]
struct SetupTemplateChirho<'model_chirho> {
    model_chirho: &'model_chirho SetupPageV1Chirho,
}

#[derive(Template)]
#[template(path = "pages_chirho/help_chirho.html", escape = "html")]
struct HelpTemplateChirho<'model_chirho> {
    model_chirho: &'model_chirho HelpPageV1Chirho,
}

#[derive(Template)]
#[template(path = "pages_chirho/auth_chirho.html", escape = "html")]
struct AuthTemplateChirho<'model_chirho> {
    model_chirho: &'model_chirho AuthPageV1Chirho,
}

#[derive(Template)]
#[template(path = "pages_chirho/agent_guide_chirho.html", escape = "html")]
struct AgentGuideTemplateChirho<'model_chirho> {
    model_chirho: &'model_chirho AgentGuidePageV1Chirho,
}

fn render_chirho(template_chirho: &impl Template) -> HtmlPartialV1Chirho {
    HtmlPartialV1Chirho::new_chirho(
        template_chirho
            .render()
            .expect("compiled product template renders into an owned string"),
    )
}

pub fn render_home_template_chirho(model_chirho: &HomePageV1Chirho) -> HtmlPartialV1Chirho {
    render_chirho(&HomeTemplateChirho { model_chirho })
}

pub fn render_craft_template_chirho(model_chirho: &CraftPageV1Chirho) -> HtmlPartialV1Chirho {
    render_chirho(&CraftTemplateChirho { model_chirho })
}

pub fn render_trust_template_chirho(model_chirho: &TrustPageV1Chirho) -> HtmlPartialV1Chirho {
    render_chirho(&TrustTemplateChirho { model_chirho })
}

pub fn render_login_template_chirho(model_chirho: &LoginPageV1Chirho) -> HtmlPartialV1Chirho {
    render_chirho(&LoginTemplateChirho { model_chirho })
}

pub fn render_setup_template_chirho(model_chirho: &SetupPageV1Chirho) -> HtmlPartialV1Chirho {
    render_chirho(&SetupTemplateChirho { model_chirho })
}

pub fn render_help_template_chirho(model_chirho: &HelpPageV1Chirho) -> HtmlPartialV1Chirho {
    render_chirho(&HelpTemplateChirho { model_chirho })
}

pub fn render_auth_template_chirho(model_chirho: &AuthPageV1Chirho) -> HtmlPartialV1Chirho {
    render_chirho(&AuthTemplateChirho { model_chirho })
}

pub fn render_agent_guide_template_chirho(
    model_chirho: &AgentGuidePageV1Chirho,
) -> HtmlPartialV1Chirho {
    render_chirho(&AgentGuideTemplateChirho { model_chirho })
}

/// Receives only the engine's display projection, not an HTTP request, body or identity.
/// Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/signup-chirho.md.
pub fn render_auth_input_chirho(
    input_chirho: &[u8],
    locale_chirho: perffection_ui_chirho::UiLocaleChirho,
) -> Result<HtmlPartialV1Chirho, &'static str> {
    if input_chirho.len() > crate::types_chirho::auth_page_v1_chirho::MAX_AUTH_VIEW_BYTES_CHIRHO {
        return Err("auth display input exceeds its bound");
    }
    let view_chirho =
        serde_json::from_slice(input_chirho).map_err(|_| "invalid auth display input")?;
    let model_chirho = AuthPageV1Chirho::from_input_chirho(view_chirho, locale_chirho)?;
    Ok(render_auth_template_chirho(&model_chirho))
}
