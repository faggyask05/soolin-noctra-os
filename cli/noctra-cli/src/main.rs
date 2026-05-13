use std::env;
use std::process::Command;

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
    print_header("NOCTRA TERMINAL EDITION");
    println!("{PURPLE}Szia, Kapitány.{RESET}");
    println!("Noctra CLI aktív.");
    println!();
    println!("{DIM}Próbáld:{RESET}");
    println!("  noctra help");
    println!("  noctra version");
    println!("  noctra system check");
    println!("  noctra start steam");
    print_side_glyph("N");
}

fn show_help() {
    print_header("NOCTRA HELP");

    println!("{CYAN}Elérhető parancsok:{RESET}");
    println!("  noctra help              Súgó megnyitása");
    println!("  noctra version           Verzió információ");
    println!("  noctra system check      Rendszerállapot ellenőrzés");
    println!("  noctra start <app>       Alkalmazás indítása");
    println!("  noctra shutdown          Leállítási figyelmeztetés");
    println!();
    println!("{PURPLE}Nem statikus súgó vagyok. Ez csak az első lélegzetem.{RESET}");

    print_side_glyph("?");
}

fn show_version() {
    print_header("NOCTRA VERSION");
    println!("Soolin Noctra OS - CLS Alpha");
    println!("Noctra CLI version: 0.1.0");
    println!("Profile layer: detected");
    print_side_glyph("0.1");
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
        println!("{PURPLE}Mit indítsak el, Kapitány? Gondolatolvasást még nem fordítottunk bele.{RESET}");
        return;
    }

    let app = &args[2];

    print_header("APP START");
    println!("{CYAN}Indítás:{RESET} {}", app);
    print_side_glyph(app);

    match Command::new(app).spawn() {
        Ok(_) => println!("{PURPLE}Elindítottam. Próbálj meg nem káoszt csinálni belőle.{RESET}"),
        Err(err) => {
            println!("{PURPLE}Nem sikerült elindítani: {app}{RESET}");
            println!("{DIM}Hiba: {err}{RESET}");
            println!("Lehet, hogy nincs telepítve. Később ezt a `noctra install {app}` kezeli majd.");
        }
    }
}

fn shutdown_warning() {
    print_header("SHUTDOWN");
    println!("{PURPLE}Leállítanám, de ez még alpha. Nem adok magamnak túl sok hatalmat első randin.{RESET}");
    println!("{DIM}Később: save all -> confirm -> shutdown{RESET}");
    print_side_glyph("OFF");
}

fn unknown_command(args: &[String]) {
    print_header("UNKNOWN COMMAND");
    println!("{PURPLE}Ezt még nem tanítottad meg nekem, Kapitány.{RESET}");
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
        Err(err) => println!("Nem sikerült futtatni: {cmd} ({err})"),
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
