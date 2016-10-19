extern crate termion;

mod ansi_escapes {
    pub static CURSOR_HIDE: &'static str = "\x1B[?25l";
    pub static CURSOR_SHOW: &'static str = "\x1B[?25h";

    pub static CURSOR_LEFT: &'static str = "\x1B[1000D";
    pub static ERASE_END_LINE: &'static str = "\x1B[K";

    pub fn cursor_up(count: u32) -> String {
        format!("\x1B[{}A", count)
    }

    pub fn erase_lines(count: u32) -> String {
        let mut result: String = "".to_owned();

        for idx in 0..count {
            if idx > 0 {
                result.push_str(&*cursor_up(1));
            }

            result.push_str(CURSOR_LEFT);
            result.push_str(ERASE_END_LINE);
        }

        result
    }
}

mod log_update {
    use ansi_escapes;
    use std::io::Write;

    use std::io::Error;

    pub struct LogUpdate<W: Write> {
        stream: W,
        previous_line_count: u32
    }

    impl<W: Write> LogUpdate<W> {
        pub fn new(mut stream: W) -> Result<Self, Error> {
            try!(write!(stream, "{}", ansi_escapes::CURSOR_HIDE));
            try!(stream.flush());

            Ok(LogUpdate { stream: stream, previous_line_count: 0 })
        }

        pub fn render(&mut self, text: &str) -> Result<(), Error> {
            try!(write!(self.stream, "{}{}", ansi_escapes::erase_lines(self.previous_line_count), text));
            try!(self.stream.flush());
            self.previous_line_count = text.chars().filter(|x| *x == '\n').count() as u32 + 1;

            Ok(())
        }

        pub fn clear(&mut self) -> Result<(), Error> {
            try!(write!(self.stream, "{}", ansi_escapes::erase_lines(self.previous_line_count)));
            try!(self.stream.flush());
            self.previous_line_count = 0;

            Ok(())
        }
    }

    impl<W: Write> Drop for LogUpdate<W> {
        fn drop(&mut self) {
            write!(self.stream, "{}", ansi_escapes::CURSOR_SHOW).unwrap();
            self.stream.flush().unwrap();
        }
    }
}

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
