use std::io::{self, Write};
use std::process::{exit, Command};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!(
            "usage: {} <host> [user]\n  host may be user@host or an ssh_config alias\n  env RMUX_TMUX: path to tmux on the remote host (default: tmux)",
            args[0]
        );
        exit(2);
    }
    let host = &args[1];
    let target = match args.get(2) {
        Some(user) => format!("{user}@{host}"),
        None => host.to_string(),
    };
    // Remote tmux command; non-interactive ssh may not have tmux on PATH.
    let tmux = std::env::var("RMUX_TMUX")
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "tmux".to_string());

    // List sessions (non-interactive ssh, so use the configured tmux path).
    let out = Command::new("ssh")
        .args([&target, &format!("{tmux} list-sessions -F '#{{session_name}}|#{{session_windows}}|#{{session_attached}}'")])
        .output()
        .expect("failed to run ssh");
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        if stderr.contains("error connecting to") || stderr.contains("no server running") {
            eprintln!("no tmux sessions on {target} (no tmux server running there)");
            exit(1);
        }
        eprintln!("could not list sessions:\n{stderr}");
        exit(1);
    }

    let sessions: Vec<(String, String, String)> = String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter_map(|l| {
            let mut p = l.splitn(3, '|');
            Some((p.next()?.to_string(), p.next()?.to_string(), p.next()?.to_string()))
        })
        .collect();

    if sessions.is_empty() {
        eprintln!("no tmux sessions on {target}");
        exit(1);
    }

    for (i, (name, wins, att)) in sessions.iter().enumerate() {
        println!("[{}] {name}  (windows={wins}, attached clients={att})", i + 1);
    }
    print!("Select session number (q to quit): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    let idx = match input.parse::<usize>() {
        Ok(n) if n >= 1 && n <= sessions.len() => n - 1,
        _ => {
            eprintln!("quitting");
            exit(0);
        }
    };

    let name = &sessions[idx].0;
    let status = Command::new("ssh")
        .args(["-t", &target, &format!("{tmux} attach -t '{name}'")])
        .status()
        .expect("failed to run ssh");
    exit(status.code().unwrap_or(1));
}
