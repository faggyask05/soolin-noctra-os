use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub is_captain: bool,
}

pub fn load_profile() -> Profile {
    if let Ok(value) = env::var("NOCTRA_PROFILE") {
        return profile_from_name(&value);
    }

    let root = find_project_root();
    let active_profile_path = format!("{}/personality/runtime/active_profile.txt", root);

    if Path::new(&active_profile_path).exists() {
        if let Ok(value) = fs::read_to_string(&active_profile_path) {
            return profile_from_name(value.trim());
        }
    }

    Profile {
        name: "public".to_string(),
        is_captain: false,
    }
}

fn profile_from_name(value: &str) -> Profile {
    match value.trim().to_lowercase().as_str() {
        "captain" => Profile {
            name: "captain".to_string(),
            is_captain: true,
        },
        _ => Profile {
            name: "public".to_string(),
            is_captain: false,
        },
    }
}

fn find_project_root() -> String {
    let candidates = [
        ".",
        "../..",
        "../../..",
        "../../../..",
    ];

    for candidate in candidates {
        let marker = format!("{}/personality/core/noctra_base.txt", candidate);

        if Path::new(&marker).exists() {
            return candidate.to_string();
        }
    }

    ".".to_string()
}
