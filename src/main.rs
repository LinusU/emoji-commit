extern crate termion;
extern crate log_update;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::io::{Write, stderr, stdout, stdin};
use std::process::exit;

use std::env;
use std::fs::File;

use log_update::LogUpdate;

fn print_emoji_selector<W: Write>(log_update: &mut LogUpdate<W>, selected: u32) {
    let text = format!("{}{}\r\n{}{}\r\n{}{}\r\n{}{}\r\n{}{}",
        if selected == 0 { "👉  " } else { "   " }, "💥  - Breaking",
        if selected == 1 { "👉  " } else { "   " }, "🎉  - Feature",
        if selected == 2 { "👉  " } else { "   " }, "🐛  - Bugfix",
        if selected == 3 { "👉  " } else { "   " }, "🔥  - Cleanup / Performance",
        if selected == 4 { "👉  " } else { "   " }, "🌹  - Other",
    );

    log_update.render(&text).unwrap();
}

fn select_emoji() -> Option<&'static str> {
    let mut log_update = LogUpdate::new(stderr()).unwrap();
    let mut raw_output = stderr().into_raw_mode().unwrap();

    let mut key_stream = stdin().keys();

    let mut aborted = false;
    let mut selected: u32 = 0;

    if !aborted {
        print_emoji_selector(&mut log_update, selected);

        loop {
            match key_stream.next().unwrap().unwrap() {
                Key::Ctrl('c') => { aborted = true; break },
                Key::Char('\n') => break,
                Key::Up => selected = (selected + 4) % 5,
                Key::Down => selected = (selected + 1) % 5,
                _ => {},
            }

            print_emoji_selector(&mut log_update, selected);
        }

        log_update.clear().unwrap();
    }

    raw_output.flush().unwrap();

    if aborted {
        return None
    }

    match selected {
        0 => Some("💥"),
        1 => Some("🎉"),
        2 => Some("🐛"),
        3 => Some("🔥"),
        4 => Some("🌹"),
        _ => panic!("Invalid value for selected"),
    }
}

fn abort() -> ! {
    let mut output = stderr();

    write!(output, "Aborted...\n").unwrap();
    output.flush().unwrap();

    exit(1)
}

fn main() {
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

        if let Some(out_path) = env::args().nth(1) {
            let mut f = File::create(out_path).unwrap();
            f.write_all(result.as_bytes()).unwrap();
        } else {
            write!(stdout(), "{}", result).unwrap();
        }
    }
}
