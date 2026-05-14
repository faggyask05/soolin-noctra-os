use std::path::Path;

#[derive(Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub is_captain: bool,
}

pub fn load_profile() -> Profile {
    let root = find_project_root();

    let captain_profile = format!("{}/personality/profiles/captain-profile.md", root);
    let captain_memory = format!("{}/personality/memory/captain-memory.json", root);
    let captain_policy = format!("{}/personality/policies/captain-policy.md", root);

    let captain_available =
        Path::new(&captain_profile).exists()
        && Path::new(&captain_memory).exists()
        && Path::new(&captain_policy).exists();

    if captain_available {
        Profile {
            name: "captain".to_string(),
            is_captain: true,
        }
    } else {
        Profile {
            name: "public".to_string(),
            is_captain: false,
        }
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
        let marker = format!("{}/personality/core/noctra-core.md", candidate);

        if Path::new(&marker).exists() {
            return candidate.to_string();
        }
    }

    ".".to_string()
}
