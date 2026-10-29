//! In-terminal Glossa branding (dual: quiet default, compact banner on request).

const RESET: &str = "\u{1b}[0m";
const GRAPHITE: &str = "\u{1b}[38;2;28;31;36m";
const IRON_BLUE: &str = "\u{1b}[38;2;47;74;110m";
const MUTED: &str = "\u{1b}[38;2;92;99;112m";

/// Compact engraved-specimen banner (ASCII; optional ANSI colour).
pub fn print_banner(color: bool) {
    let lines = [
        r"   ██████╗",
        r"  ██╔════╝",
        r"  ██║  ███╗",
        r"  ██║   ██║",
        r"  ╚██████╔╝",
        r"   ╚═════╝",
    ];
    for line in lines {
        if color {
            println!("{GRAPHITE}{line}{RESET}");
        } else {
            println!("{line}");
        }
    }
    if color {
        println!("{GRAPHITE}  glossa{RESET}");
        println!(
            "{MUTED}  every linguist's research tool,{RESET}"
        );
        println!(
            "{MUTED}  in one fast terminal workspace{RESET}"
        );
        println!("{IRON_BLUE}  offline · reproducible · CLI/TUI{RESET}");
    } else {
        println!("  glossa");
        println!("  every linguist's research tool,");
        println!("  in one fast terminal workspace");
        println!("  offline · reproducible · CLI/TUI");
    }
}

/// True when the compact banner should show on bare `glossa`.
pub fn banner_env_enabled() -> bool {
    match std::env::var("GLOSSA_BANNER") {
        Ok(v) => {
            let v = v.trim();
            !(v.is_empty() || v == "0" || v.eq_ignore_ascii_case("false"))
        }
        Err(_) => false,
    }
}

pub fn use_color(no_color_flag: bool) -> bool {
    if no_color_flag || std::env::var_os("NO_COLOR").is_some() {
        return false;
    }
    // Only colour interactive TTYs.
    #[cfg(unix)]
    {
        use std::io::IsTerminal;
        std::io::stdout().is_terminal()
    }
    #[cfg(not(unix))]
    {
        true
    }
}
