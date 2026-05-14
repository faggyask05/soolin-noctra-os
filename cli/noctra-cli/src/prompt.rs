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
    let continuity = load_file("personality/memory/continuity.txt");

    let prompt = format!(
r#"
STATIC NOCTRA SYSTEM PROMPT

CORE IDENTITY:
{core}

PROFILE LAYER:
{profile_layer}

TECHNICAL LAYER:
{technical}

CONTINUITY MEMORY:
{continuity}

GLOBAL RESPONSE RULES:
- Magyarul válaszolj.
- Röviden válaszolj: 1-4 mondat.
- Ne találj ki új lore-t.
- Ne írj fantasy monológot.
- Ne legyél Shakespeare egy terminálban.
- Maradj Noctra: kontrollált, csípős, cyberpunk-gótikus jelenlét.
- Ha technikai kérdés jön, segíts ténylegesen.
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
