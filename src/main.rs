use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{Write, stderr, stdin};
use std::process::{Command, exit};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use default_editor;
use emoji_commit_type::CommitType;
use log_update::LogUpdate;

mod commit_rules;
mod git;

static PASS: &'static str = "\u{001b}[32m✔\u{001b}[39m";
static FAIL: &'static str = "\u{001b}[31m✖\u{001b}[39m";
static CURSOR: &'static str = "\u{001b}[4m \u{001b}[24m";

impl fmt::Display for commit_rules::CommitRuleValidationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", if self.pass { PASS } else { FAIL }, self.description)
    }
}

fn print_emoji_selector<W: Write>(log_update: &mut LogUpdate<W>, selected: &CommitType) {
    let text = CommitType::iter_variants()
        .map(|t| format!("{}  {}  \u{001b}[90m{:<5}\u{001b}[39m  {}", if t == *selected { "👉" } else { "  " }, t.emoji(), t.bump_level().name().to_lowercase(), t.description()))
        .collect::<Vec<_>>()
        .join("\r\n");

    log_update.render(&text).unwrap();
}

fn commit_type_at_index (index: u8) -> Option<CommitType> {
    return CommitType::iter_variants().nth(index as usize);
}

fn select_emoji() -> Option<&'static str> {
    let mut log_update = LogUpdate::new(stderr()).unwrap();
    let mut raw_output = stderr().into_raw_mode().unwrap();

    let mut key_stream = stdin().keys();

    let mut aborted = false;
    let mut selected = CommitType::Breaking;

    // Clear possibly printed "hint" from git
    log_update.render("").unwrap();

    loop {
        print_emoji_selector(&mut log_update, &selected);

        match key_stream.next().unwrap().unwrap() {
            Key::Ctrl('c') => { aborted = true; break },
            Key::Char('\n') => break,
            Key::Up | Key::Char('k') | Key::Char('K') => selected = selected.prev_variant().unwrap_or(CommitType::last_variant()),
            Key::Down | Key::Char('j') | Key::Char('J') => selected = selected.next_variant().unwrap_or(CommitType::first_variant()),
            Key::Char(key @ '1' ..= '9') => { commit_type_at_index((key as u8) - ('1' as u8)).map(|t| selected = t); },
            _ => {},
        }
    }

    log_update.clear().unwrap();
    raw_output.flush().unwrap();

    if aborted { None } else { Some(selected.emoji()) }
}

fn collect_commit_message(selected_emoji: &'static str) -> Option<String> {
    let mut log_update = LogUpdate::new(stderr()).unwrap();
    let mut raw_output = stderr().into_raw_mode().unwrap();

    let mut key_stream = stdin().keys();

    let mut aborted = false;
    let mut input = String::new();

    loop {
        let rule_text = commit_rules::check_message(&input)
            .map(|result| format!("{}", result))
            .collect::<Vec<_>>()
            .join("\r\n");
        let text = format!(
            "\r\nRemember the seven rules of a great Git commit message:\r\n\r\n{}\r\n\r\n{}  {}{}",
            rule_text,
            selected_emoji,
            input,
            CURSOR,
        );

        log_update.render(&text).unwrap();

        match key_stream.next().unwrap().unwrap() {
            Key::Ctrl('c') => { aborted = true; break },
            Key::Char('\n') => break,
            Key::Char(c) => input.push(c),
            Key::Backspace => { input.pop(); },
            _ => {},
        }
    }

    log_update.clear().unwrap();
    raw_output.flush().unwrap();

    if aborted { None } else { Some(String::from(input.trim())) }
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

fn launch_default_editor(out_path: &str) {
    let editor = default_editor::get().unwrap();

    run_cmd(Command::new(&editor).arg(out_path))
}

fn launch_git_with_self_as_editor() {
    let self_path = std::env::current_exe().unwrap();

    run_cmd(Command::new("git").arg("commit").env("GIT_EDITOR", self_path))
}

fn collect_information_and_write_to_file(out_path: &str) {
    let maybe_emoji = select_emoji();

    if maybe_emoji == None {
        abort();
    }

    if let Some(emoji) = maybe_emoji {
        let maybe_message = collect_commit_message(emoji);

        if maybe_message == None {
            abort();
        }

        if let Some(message) = maybe_message {
            let result = format!("{} {}\n", emoji, message);

            let mut f = File::create(out_path).unwrap();
            f.write_all(result.as_bytes()).unwrap();
        }
    }
}

#[derive(Debug)]
struct ValidationError;

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", "One or more commits has validation errors")
    }
}

impl Error for ValidationError {}

fn validate(refspecs: &[&str]) -> Result<(), Box<dyn Error>> {
    let repo_path = env::current_dir()?;
    let messages = git::get_commit_messages(repo_path, refspecs)?;

    let mut has_errors = false;
    for message in messages {
        let validation_errors = commit_rules::check_message_with_emoji(&message)
            .filter_map(|result| {
                if !result.pass { Some(format!("\t{}", result)) } else { None }
            })
            .collect::<Vec<_>>();
        if validation_errors.len() > 0 {
            has_errors = true;
            let validation_result_text = validation_errors.join("\r\n");
            let text = format!(
                "\r\nCommit:\r\n\t{}\r\nValidation Errors:\r\n{}\r\n",
                message,
                validation_result_text,
            );
            println!("{}", text);
        }
    }
    if has_errors {Err(Box::new(ValidationError{}))} else { Ok(()) }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<_>>();

    match args.iter().map(AsRef::as_ref).collect::<Vec<_>>().as_slice() {
        [_, "--validate", refspecs @ ..] => validate(refspecs),
        [_, out_path] => {
            if out_path.ends_with(".git/COMMIT_EDITMSG") {
                collect_information_and_write_to_file(out_path);
                Ok(())
            } else {
                launch_default_editor(out_path);
                Ok(())
            }
        },
        _ => {
            launch_git_with_self_as_editor();
            Ok(())
        }
    }
}
