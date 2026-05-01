mod icons;
mod sqlit;
mod types;

use std::io::Write;
use std::process::{Command, Stdio};

use colored::{Color, Colorize};
use icons::{color_for, icon_for};
use sqlit::{launch, load_connections};
use types::Row;

const DEFAULT_TYPE_WIDTH: usize = 12;

fn load_rows() -> Result<Vec<Row>, String> {
    let mut rows: Vec<Row> = load_connections()?.into_iter().map(Row::from).collect();
    rows.sort_by_cached_key(|r| r.name.to_lowercase());
    Ok(rows)
}

fn render_lines(rows: &[Row]) -> String {
    let type_width = rows
        .iter()
        .map(|r| r.db_type.len())
        .max()
        .unwrap_or(0)
        .max(DEFAULT_TYPE_WIDTH);
    rows.iter()
        .map(|row| {
            let dot = "·".color(Color::Black);
            let label = format!("{} {:<type_width$}", icon_for(&row.db_type), row.db_type)
                .color(color_for(&row.db_type));
            let info = row.info.as_str().color(Color::BrightBlack);
            let user = row.user.as_str().color(Color::Black);
            if row.user.is_empty() {
                format!("{label} {name} {dot} {info}", name = row.name)
            } else {
                format!("{label} {name} {dot} {info} {dot} {user}", name = row.name)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn fzf_pick(input: &str) -> Result<Option<String>, String> {
    if input.trim().is_empty() {
        return Ok(None);
    }

    let mut child = Command::new("fzf")
        .args([
            "--ansi",
            "--no-sort",
            "--prompt",
            " ",
            "--delimiter",
            "\t",
            "--with-nth",
            "1",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                "fzf executable not found in PATH".to_owned()
            } else {
                format!("failed to launch fzf: {e}")
            }
        })?;

    // stdin must be dropped before wait_with_output() to send EOF;
    // otherwise fzf waits for more input and deadlocks.
    {
        let mut stdin = child.stdin.take().expect("stdin was piped");
        stdin
            .write_all(input.as_bytes())
            .map_err(|e| format!("failed to write to fzf: {e}"))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|e| format!("fzf error: {e}"))?;

    if output.status.code() != Some(0) {
        return Ok(None);
    }

    let line = String::from_utf8(output.stdout).map_err(|e| format!("invalid fzf output: {e}"))?;
    let line = line.trim_end_matches('\n');
    if line.is_empty() {
        return Ok(None);
    }

    let (_, name) = line
        .split_once('\t')
        .ok_or_else(|| "unexpected fzf output format".to_owned())?;

    Ok(Some(name.trim().to_owned()))
}

fn run() -> i32 {
    colored::control::set_override(true);
    let rows = match load_rows() {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("sqlitx: failed to load connections: {e}");
            return 2;
        }
    };
    if rows.is_empty() {
        eprintln!("sqlitx: no saved connections");
        return 1;
    }

    let input = render_lines(&rows);
    match fzf_pick(&input) {
        Ok(Some(name)) => launch(&name),
        Ok(_) => 0,
        Err(e) => {
            eprintln!("sqlitx: {e}");
            2
        }
    }
}

fn main() {
    std::process::exit(run());
}
