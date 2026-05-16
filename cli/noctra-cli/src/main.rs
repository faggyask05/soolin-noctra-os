mod mood;
mod prompt;
mod profile;
mod response;
mod session;
mod state;
mod llm;

use std::env;
use std::io::{self, Write};
use std::process::Command;

use session::Session;

const PURPLE: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";
const DIM: &str = "\x1b[2m";
const RESET: &str = "\x1b[0m";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        show_home();
        return;
    }

    match args[1].as_str() {
        "help" => show_help(),
        "version" => show_version(),
        "profile" => show_profile(),
        "chat" => start_chat_session(),
        "boot" => boot_noctra(),
        "ask" => {
            if args.len() > 2 {
                let question = args[2..].join(" ");
                ask_question(&question);
            } else {
                println!("{PURPLE}Kérdezz valamit. A gondolatolvasás még nincs bekötve. Még.{RESET}");
            }
        }
        "system" => {
            if args.len() > 2 && args[2] == "check" {
                system_check();
            } else {
                unknown_command(&args);
            }
        }
        "start" => start_app(&args),
        "shutdown" => shutdown_warning(),
        _ => unknown_command(&args),
    }
}

fn show_home() {
    let profile = profile::load_profile();

    print_header("NOCTRA TERMINAL EDITION");

    println!("{PURPLE}Aktív profil:{RESET} {}", profile.name);
    println!("Noctra CLI aktív.");
    println!();

    println!("{DIM}Próbáld:{RESET}");
    println!("  noctra help");
    println!("  noctra version");
    println!("  noctra system check");
    println!("  noctra profile");
    println!("  noctra ask rooftop");
    println!("  noctra chat
  noctra boot");

    print_side_glyph("N");
}

fn show_help() {
    print_header("NOCTRA HELP");

    println!("{CYAN}Elérhető parancsok:{RESET}");
    println!("  noctra help");
    println!("  noctra version");
    println!("  noctra system check");
    println!("  noctra start <app>");
    println!("  noctra shutdown");
    println!("  noctra profile");
    println!("  noctra ask <question>");
    println!("  noctra chat
  noctra boot");

    println!();
    println!("{PURPLE}Most már nem csak shell vagyok. Ez már stateful continuity runtime kezd lenni.{RESET}");

    print_side_glyph("?");
}

fn show_version() {
    print_header("NOCTRA VERSION");

    println!("Soolin Noctra OS - CLI Alpha");
    println!("Noctra Runtime: Phase 5");
    println!("Response Engine: stateful prototype");
    println!("Session Engine: active");

    print_side_glyph("0.5");
}

fn show_profile() {
    let profile = profile::load_profile();

    print_header("ACTIVE PROFILE");

    println!("{CYAN}Aktív runtime:{RESET} {}", profile.name);
    println!("{CYAN}Captain mode:{RESET} {}", if profile.is_captain { "YES" } else { "NO" });

    print_side_glyph("PRF");
}

fn ask_question(question: &str) {
    let profile = profile::load_profile();
    let mut session = Session::new();

    print_header("NOCTRA ASK");

    let reply = response::generate_response(
        &profile,
        &mut session,
        question,
    );

    println!("{CYAN}Noctra:{RESET} {PURPLE}{reply}{RESET}");
    println!();
    println!("{DIM}Kérdés:{RESET} {}", question);
    println!("{DIM}State:{RESET} {}", session.debug_summary());

    print_side_glyph("ASK");
}

fn start_chat_session() {
    let profile = profile::load_profile();
    let mut session = Session::new();

    print_header("NOCTRA CHAT SESSION");

    println!("{PURPLE}Noctra session előkészítés alatt.{RESET}");
    println!("{DIM}Aktív profil: {}{RESET}", profile.name);
    println!("{DIM}Az első indítás hosszabb lehet. Identitásrétegek, cache és modell warmup töltődik.{RESET}");
    println!();

    println!();
    println!("{PURPLE}Noctra done. Session aktív.{RESET}");
    println!("{DIM}Kilépés: exit vagy quit{RESET}");
    println!();

    let user_label = if profile.is_captain {
        "Kapitány"
    } else {
        "User"
    };

    loop {
        print!("{CYAN}{}:{RESET} ", user_label);

        if let Err(err) = io::stdout().flush() {
            println!("{PURPLE}Nem sikerült flusholni a promptot: {err}{RESET}");
            break;
        }

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!();
                println!("{PURPLE}Input stream lezárva. Noctra session vége.{RESET}");
                break;
            }
            Ok(_) => {
                let input = input.trim();

                if input.eq_ignore_ascii_case("exit")
                    || input.eq_ignore_ascii_case("quit")
                {
                    println!("{PURPLE}Noctra:{RESET} Kilépek a sessionből. Ne szokj hozzá, hogy ilyen engedelmes vagyok.");
                    break;
                }

                if input.is_empty() {
                    println!("{PURPLE}Noctra:{RESET} Írj valamit. A drámai csendet majd később implementáljuk.");
                    continue;
                }

                let reply = response::generate_response(
                    &profile,
                    &mut session,
                    input,
                );

                println!("{PURPLE}Noctra:{RESET} {}", reply);
                println!("{DIM}State: {}{RESET}", session.debug_summary());
                println!();
            }
            Err(err) => {
                println!("{PURPLE}Input hiba: {err}{RESET}");
                break;
            }
        }
    }

    print_side_glyph("CHAT");
}


fn boot_noctra() {
    let profile = profile::load_profile();

    print_header("NOCTRA BOOT");

    println!("{PURPLE}Profil:{RESET} {}", profile.name);
    println!("{DIM}Noctra modell előkészítése a háttérfutáshoz.{RESET}");

    llm::warmup(&profile);

    print_side_glyph("BOOT");
}

fn system_check() {
    print_header("SYSTEM CHECK");

    run_and_print("Hostname", "hostname", &[]);
    run_and_print("Kernel", "uname", &["-r"]);
    run_and_print("Uptime", "uptime", &["-p"]);
    run_and_print("Disk", "df", &["-h", "/"]);
    run_and_print("Memory", "free", &["-h"]);

    println!();
    println!("{PURPLE}Rendszer él. Meglepő, de örülök.{RESET}");

    print_side_glyph("SYS");
}

fn start_app(args: &[String]) {
    if args.len() < 3 {
        println!("{PURPLE}Mit indítsak el? Gondolatolvasást még nem fordítottunk bele.{RESET}");
        return;
    }

    let app = &args[2];

    print_header("APP START");

    println!("{CYAN}Indítás:{RESET} {}", app);

    print_side_glyph(app);

    match Command::new(app).spawn() {
        Ok(_) => {
            println!("{PURPLE}Elindítottam. Ne gyújtsd fel a rendszert.{RESET}");
        }
        Err(err) => {
            println!("{PURPLE}Nem sikerült elindítani: {app}{RESET}");
            println!("{DIM}Hiba: {err}{RESET}");
        }
    }
}

fn shutdown_warning() {
    print_header("SHUTDOWN");

    println!("{PURPLE}Még nem adok magamnak teljes rendszerhatalmat. Egyelőre.{RESET}");

    print_side_glyph("OFF");
}

fn unknown_command(args: &[String]) {
    print_header("UNKNOWN COMMAND");

    println!("{PURPLE}Ezt még nem tanítottad meg nekem.{RESET}");
    println!("{DIM}Kapott parancs: {:?}{RESET}", &args[1..]);
    println!("Próbáld: noctra help");

    print_side_glyph("...");
}

fn run_and_print(label: &str, cmd: &str, args: &[&str]) {
    println!("{CYAN}{label}:{RESET}");

    match Command::new(cmd).args(args).output() {
        Ok(output) => {
            let text = String::from_utf8_lossy(&output.stdout);
            println!("{}", text.trim());
        }
        Err(err) => {
            println!("Nem sikerült futtatni: {cmd} ({err})");
        }
    }

    println!();
}

fn print_header(title: &str) {
    println!();
    println!("{PURPLE}╔══════════════════════════════════════╗{RESET}");
    println!("{PURPLE}║{RESET} {CYAN}{:<36}{RESET} {PURPLE}║{RESET}", title);
    println!("{PURPLE}╚══════════════════════════════════════╝{RESET}");
    println!();
}

fn print_side_glyph(label: &str) {
    println!();
    println!("{DIM}──────── Noctra visual field ────────{RESET}");
    println!("{PURPLE}        .     .       .       {RESET}");
    println!("{PURPLE}    .        ◯────────{}       {RESET}", label);
    println!("{PURPLE}        .     \\   /      .    {RESET}");
    println!("{PURPLE}              \\ N /           {RESET}");
    println!("{PURPLE}          .    \\_/     .      {RESET}");
    println!("{DIM}─────────────────────────────────────{RESET}");
}
