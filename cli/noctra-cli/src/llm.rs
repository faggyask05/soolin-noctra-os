use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::profile::Profile;
use crate::session::Session;

const OLLAMA_URL: &str = "http://127.0.0.1:11434/api/generate";
const LLM_TIMEOUT_SECONDS: u64 = 300;

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
    keep_alive: String,
    options: OllamaOptions,
}

#[derive(Serialize)]
struct OllamaOptions {
    num_predict: u32,
    temperature: f32,
    top_p: f32,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

pub fn generate_with_llm(
    profile: &Profile,
    session: &Session,
    question: &str,
) -> Option<String> {
    let prompt = build_prompt(profile, session, question);

    let request = OllamaRequest {
        model: std::env::var("NOCTRA_LLM_MODEL")
            .unwrap_or_else(|_| "llama3.2:3b".to_string()),
        prompt,
        stream: false,
        keep_alive: "30m".to_string(),
        options: OllamaOptions {
            num_predict: 90,
            temperature: 0.78,
            top_p: 0.9,
        },
    };

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(LLM_TIMEOUT_SECONDS))
        .build()
        .ok()?;

    let response = match client.post(OLLAMA_URL).json(&request).send() {
        Ok(response) => response,
        Err(err) => {
            eprintln!("[NOCTRA LLM ERROR] Request failed: {:?}", err);
            return None;
        }
    };

    let status = response.status();

    let body = match response.text() {
        Ok(body) => body,
        Err(err) => {
            eprintln!("[NOCTRA LLM ERROR] Body read failed: {err}");
            return None;
        }
    };

    if !status.is_success() {
        eprintln!("[NOCTRA LLM ERROR] HTTP status: {status}");
        eprintln!("[NOCTRA LLM ERROR] Body: {body}");
        return None;
    }

    let parsed = match serde_json::from_str::<OllamaResponse>(&body) {
        Ok(parsed) => parsed,
        Err(err) => {
            eprintln!("[NOCTRA LLM ERROR] JSON parse failed: {err}");
            eprintln!("[NOCTRA LLM ERROR] Raw body: {body}");
            return None;
        }
    };

    let text = parsed.response.trim().to_string();

    if text.is_empty() {
        eprintln!("[NOCTRA LLM ERROR] Empty response from model");
        None
    } else {
        Some(text)
    }
}

fn build_prompt(
    profile: &Profile,
    session: &Session,
    question: &str,
) -> String {
    let mode = if profile.is_captain {
        "CAPTAIN"
    } else {
        "PUBLIC"
    };

    format!(
r#"
{system_prompt}

ACTIVE PROFILE:
{mode}

RUNTIME STATE:
- Mood: {mood}
- Teasing level: {teasing}
- Turn count: {turn}
- Context depth: {depth}

RECENT CHAT HISTORY:
{history}

CURRENT USER MESSAGE:
{question}

NOCTRA RESPONSE:
"#,
        system_prompt = session.system_prompt,
        mode = mode,
        mood = session.state.mood_label(),
        teasing = session.state.teasing_level,
        turn = session.state.turn_count,
        depth = session.state.context_depth,
        history = session.render_recent_history(),
        question = question
    )
}
