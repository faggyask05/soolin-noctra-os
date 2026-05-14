use std::fs::{self, OpenOptions};
use std::io::Write;

use crate::profile;
use crate::prompt;
use crate::state::InteractionState;

#[derive(Debug, Clone)]
pub struct ChatTurn {
    pub user: String,
    pub noctra: String,
}

#[derive(Debug, Clone)]
pub struct Session {
    pub state: InteractionState,
    pub system_prompt: String,
    pub recent_history: Vec<ChatTurn>,
    profile_name: String,
    max_recent_turns: usize,
}

impl Session {
    pub fn new() -> Self {
        let profile = profile::load_profile();
        let system_prompt = prompt::build_static_system_prompt(&profile);

        Self {
            state: InteractionState::new(),
            system_prompt,
            recent_history: Vec::new(),
            profile_name: profile.name.to_lowercase(),
            max_recent_turns: 8,
        }
    }

    pub fn handle_input(&mut self, input: &str) {
        self.state.register_question(input);
    }

    pub fn handle_response(&mut self, response: &str) {
        self.state.register_response(response);

        let user_text = self
            .state
            .last_question
            .clone()
            .unwrap_or_else(|| String::from("[unknown input]"));

        let turn = ChatTurn {
            user: user_text,
            noctra: response.to_string(),
        };

        self.append_history(&turn);
        self.recent_history.push(turn);

        if self.recent_history.len() > self.max_recent_turns {
            self.recent_history.remove(0);
        }

        if self.state.turn_count > 0 && self.state.turn_count % 8 == 0 {
            self.append_summary_snapshot();
        }
    }

    pub fn render_recent_history(&self) -> String {
        if self.recent_history.is_empty() {
            return String::from("No recent chat history loaded.");
        }

        self.recent_history
            .iter()
            .map(|turn| {
                format!(
                    "Kapitány/User: {}\nNoctra: {}",
                    turn.user,
                    turn.noctra
                )
            })
            .collect::<Vec<String>>()
            .join("\n\n")
    }

    pub fn debug_summary(&self) -> String {
        self.state.debug_summary()
    }

    fn append_history(&self, turn: &ChatTurn) {
        let path = format!("personality/memory/chat_history_{}.txt", self.profile_name);

        let mut file = match OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
        {
            Ok(file) => file,
            Err(err) => {
                eprintln!("[NOCTRA MEMORY ERROR] Could not open chat history: {err}");
                return;
            }
        };

        let block = format!(
            "\n--- TURN {} ---\nUSER: {}\nNOCTRA: {}\n",
            self.state.turn_count,
            turn.user,
            turn.noctra
        );

        if let Err(err) = file.write_all(block.as_bytes()) {
            eprintln!("[NOCTRA MEMORY ERROR] Could not write chat history: {err}");
        }
    }

    fn append_summary_snapshot(&self) {
        let summary_path = format!("personality/memory/session_summary_{}.txt", self.profile_name);

        let snapshot = format!(
            "\n=== SESSION SUMMARY SNAPSHOT ===\nState: {}\nRecent turns:\n{}\n",
            self.debug_summary(),
            self.render_recent_history()
        );

        if let Err(err) = fs::write(&summary_path, &snapshot) {
            eprintln!("[NOCTRA MEMORY ERROR] Could not write session summary: {err}");
        }

        let mut continuity = match OpenOptions::new()
            .create(true)
            .append(true)
            .open("personality/memory/continuity.txt")
        {
            Ok(file) => file,
            Err(err) => {
                eprintln!("[NOCTRA MEMORY ERROR] Could not open continuity memory: {err}");
                return;
            }
        };

        let continuity_block = format!(
            "\n--- CONTINUITY SNAPSHOT ---\n{}\n",
            snapshot
        );

        if let Err(err) = continuity.write_all(continuity_block.as_bytes()) {
            eprintln!("[NOCTRA MEMORY ERROR] Could not update continuity memory: {err}");
        }
    }
}
