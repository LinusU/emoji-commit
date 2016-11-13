extern crate termion;
extern crate log_update;
extern crate default_editor;
extern crate emoji_commit_type;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::io::{Write, stderr, stdin};
use std::process::{Command, exit};

use std::env;
use std::fs::File;

use log_update::LogUpdate;
use emoji_commit_type::CommitType;

fn print_emoji_selector<W: Write>(log_update: &mut LogUpdate<W>, selected: &CommitType) {
    let text = CommitType::iter_variants()
        .map(|t| format!("{}  {}  - {}", if t == *selected { "👉" } else { " " }, t.emoji(), t.description()))
        .collect::<Vec<_>>()
        .join("\r\n");

    log_update.render(&text).unwrap();
}

fn select_emoji() -> Option<&'static str> {
    let mut log_update = LogUpdate::new(stderr()).unwrap();
    let mut raw_output = stderr().into_raw_mode().unwrap();

    let mut key_stream = stdin().keys();

    let mut aborted = false;
    let mut selected = CommitType::Breaking;

    loop {
        print_emoji_selector(&mut log_update, &selected);

        match key_stream.next().unwrap().unwrap() {
            Key::Ctrl('c') => { aborted = true; break },
            Key::Char('\n') => break,
            Key::Up => selected = selected.prev_variant().unwrap_or(CommitType::last_variant()),
            Key::Down => selected = selected.next_variant().unwrap_or(CommitType::first_variant()),
            _ => {},
        }
    }

    log_update.clear().unwrap();
    raw_output.flush().unwrap();

    if aborted { None } else { Some(selected.emoji()) }
}

fn abort() -> ! {
    let mut output = stderr();

    write!(output, "Aborted...\n").unwrap();
    output.flush().unwrap();

    exit(1)
}

fn run_cmd(cmd: &mut Command) {
    let status = cmd.status().unwrap();

    if !status.success() {
        exit(status.code().unwrap_or(1));
    }
}

fn launch_default_editor(out_path: String) {
    let editor = default_editor::get().unwrap();

    run_cmd(Command::new(&editor).arg(out_path))
}

fn launch_git_with_self_as_editor() {
    let self_path = std::env::current_exe().unwrap();

    run_cmd(Command::new("git").arg("commit").env("GIT_EDITOR", self_path))
}

fn collect_information_and_write_to_file(out_path: String) {
    let maybe_emoji = select_emoji();

    if maybe_emoji == None {
        abort();
    }

    if let Some(emoji) = maybe_emoji {
        let mut prompt = stderr();

        write!(prompt, "{}  ", emoji).unwrap();
        prompt.flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let result = format!("{} {}\n", emoji, input.trim());

        let mut f = File::create(out_path).unwrap();
        f.write_all(result.as_bytes()).unwrap();
    }
}

fn main() {
    if let Some(out_path) = env::args().nth(1) {
        if out_path.ends_with(".git/COMMIT_EDITMSG") {
            collect_information_and_write_to_file(out_path);
        } else {
            launch_default_editor(out_path);
        }
    } else {
        launch_git_with_self_as_editor();
    }
}
