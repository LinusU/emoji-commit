use ansi_term::Colour::White;

pub struct InputString {
    value: String,
    position: usize,
    is_control_input: bool,
}

impl InputString {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            position: 0,
            is_control_input: false,
        }
    }

    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }

    pub fn trim(&self) -> &str {
        self.value.trim()
    }
    pub fn push(&mut self, c: char) {
        if self.is_control_input {
            self.handle_control(c);
            return;
        }
        if self.position == self.value.len() {
            self.value.push(c);
        } else {
            let mut new_string = String::new();
            new_string.push_str(&self.value.as_str()[0..self.position]);
            new_string.push(c);
            new_string.push_str(&self.value.as_str()[self.position..]);
            self.value = new_string;
        }
        self.position += 1;
    }

    pub fn delete(&mut self) {
        if self.position == self.value.len() {
            return;
        }
        if self.position == 0 {
            self.value.remove(0);
        } else {
            let mut new_string = String::new();
            new_string.push_str(&self.value.as_str()[0..self.position]);
            new_string.push_str(&self.value.as_str()[self.position + 1..]);
            self.value = new_string;
        }
    }

    pub fn backspace(&mut self) {
        if self.position == 0 {
            return;
        }
        if self.position == self.value.len() {
            self.value.pop();
        } else {
            let mut new_string = String::new();
            new_string.push_str(&self.value.as_str()[0..self.position - 1]);
            new_string.push_str(&self.value.as_str()[self.position..]);
            self.value = new_string;
        }
        self.position -= 1;
    }

    pub fn go_char_left(&mut self) {
        if self.position > 1 {
            self.position -= 1;
        } else {
            self.position = 0;
        }
    }
    pub fn go_char_right(&mut self) {
        if self.position < self.value.len() {
            self.position += 1;
        } else {
            self.position = self.value.len();
        }
    }
    pub fn handle_control(&mut self, c: char) {
        if !self.is_control_input && c.is_control() {
            self.is_control_input = true
        }
        if self.is_control_input {
            match c {
                'D' => self.go_word_left(),
                'C' => self.go_word_right(),
                _ => { return; }
            }
            self.is_control_input = false
        }

        match c {
            'b' => self.go_word_left(),
            'f' => self.go_word_right(),
            _ => {}
        }
    }
    pub fn go_word_left(&mut self) {
        if self.position == 0 {
            return;
        }
        let mut new_position = self.position;
        while new_position > 0
            && self
                .value
                .as_str()
                .chars()
                .nth(new_position - 1)
                .unwrap()
                .is_whitespace()
        {
            new_position -= 1;
        }
        while new_position > 0
            && !self
                .value
                .as_str()
                .chars()
                .nth(new_position - 1)
                .unwrap()
                .is_whitespace()
        {
            new_position -= 1;
        }
        self.position = new_position;
    }
    pub fn go_word_right(&mut self) {
        if self.position == self.value.len() {
            return;
        }
        let mut new_position = self.position;
        while new_position < self.value.len()
            && self
                .value
                .as_str()
                .chars()
                .nth(new_position)
                .unwrap()
                .is_whitespace()
        {
            new_position += 1;
        }
        while new_position < self.value.len()
            && !self
                .value
                .as_str()
                .chars()
                .nth(new_position)
                .unwrap()
                .is_whitespace()
        {
            new_position += 1;
        }
        self.position = new_position;
    }
    pub fn format(&self) -> String {
        if self.position == self.value.len() {
            format!("{}{}", &self.value.as_str(), White.underline().paint(" "))
        } else {
            format!(
                "{}{}{}",
                &self.value.as_str()[0..self.position],
                White
                    .underline()
                    .paint(&self.value.as_str()[self.position..self.position + 1]),
                &self.value.as_str()[self.position + 1..]
            )
        }
    }
}
