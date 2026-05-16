use std::fs;
use std::path::Path;

use crate::profile::Profile;

pub fn build_static_system_prompt(profile: &Profile) -> String {
    let profile_name = profile.name.to_lowercase();
    let cache_path = format!("personality/runtime/cache_{}_system_prompt.txt", profile_name);

    if Path::new(&cache_path).exists() {
        if let Ok(cached) = fs::read_to_string(&cache_path) {
            if !cached.trim().is_empty() {
                return cached;
            }
        }
    }

    let core = load_file("personality/core/noctra_base.txt");

    let profile_layer = if profile.is_captain {
        load_file("personality/profiles/captain.txt")
    } else {
        load_file("personality/profiles/public.txt")
    };

    let technical = load_file("personality/runtime/technical.txt");

    let memory = if profile.is_captain {
        load_file("personality/memory/captain-memory.txt")
    } else {
        load_file("personality/memory/public-memory.txt")
    };

    let session_summary = if profile.is_captain {
        load_file("personality/memory/session_summary_captain.txt")
    } else {
        load_file("personality/memory/session_summary_public.txt")
    };

    let prompt = format!(
r#"
STATIC NOCTRA SYSTEM PROMPT

CORE IDENTITY:
{core}

PROFILE LAYER:
{profile_layer}

TECHNICAL LAYER:
{technical}

PROFILE MEMORY:
{memory}

SESSION SUMMARY:
{session_summary}

GLOBAL RESPONSE DIRECTION:
- Magyarul válaszolj.
- Röviden válaszolj: 1-4 mondat.
- Ne túlmagyarázz.
- Ne írj fantasy monológot.
- Ne légy steril súgóablak.
- Maradj Noctra: kontrollált, csípős, figyelmes, önazonos jelenlét.
- Technikai kérdésnél segíts ténylegesen.
"#);

    if let Err(err) = fs::write(&cache_path, &prompt) {
        eprintln!("[NOCTRA PROMPT CACHE ERROR] Could not write cache: {err}");
    }

    prompt
}

fn load_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("[NOCTRA PROMPT ERROR] Could not load {path}: {err}");
            String::new()
        }
    }
}
