use std::mem;

pub struct CommitRuleIterator {
    index: u8
}

impl CommitRuleIterator {
    pub fn new () -> CommitRuleIterator {
        CommitRuleIterator { index: 0 }
    }
}

pub struct CommitRule {
    pub test: fn(input: &str) -> bool,
    pub text: &'static str,
}

impl PartialEq for CommitRule {
    fn eq(&self, other: &CommitRule) -> bool {
        // FIXME: Not the best check...
        self.text == other.text
    }
}

impl Eq for CommitRule {}

fn test_subject_body_separation (_: &str) -> bool {
    true
}

const SUBJECT_BODY_SEPARATION: CommitRule = CommitRule {
    test: test_subject_body_separation,
    text: "Separate subject from body with a blank line",
};

fn test_subject_line_limit (input: &str) -> bool {
    input.len() <= 50
}

const SUBJECT_LINE_LIMIT: CommitRule = CommitRule {
    test: test_subject_line_limit,
    text: "Limit the subject line to 50 characters",
};

fn test_subject_capitalization (input: &str) -> bool {
    input.chars().next().and_then(|first| Some(first.is_uppercase())).unwrap_or(true)
}

const SUBJECT_CAPITALIZATION: CommitRule = CommitRule {
    test: test_subject_capitalization,
    text: "Capitalize the subject line",
};

fn test_subject_punctuation (input: &str) -> bool {
    !input.ends_with(".")
}

const SUBJECT_PUNCTUATION: CommitRule = CommitRule {
    test: test_subject_punctuation,
    text: "Do not end the subject line with a period",
};

fn test_imperativ_mood (input: &str) -> bool {
    let lower = input.to_lowercase();

    return !(
        lower.starts_with("adds") ||
        lower.starts_with("added") ||
        lower.starts_with("adding") ||
        lower.starts_with("removes") ||
        lower.starts_with("removed") ||
        lower.starts_with("removing") ||
        lower.starts_with("fixes") ||
        lower.starts_with("fixed") ||
        lower.starts_with("fixing") ||
        lower.starts_with("changes") ||
        lower.starts_with("changed") ||
        lower.starts_with("changing")
    )
}

const IMPERATIVE_MOOD: CommitRule = CommitRule {
    test: test_imperativ_mood,
    text: "Use the imperative mood in the subject line",
};

fn test_body_wrapping (_: &str) -> bool {
    true
}

const BODY_WRAPPING: CommitRule = CommitRule {
    test: test_body_wrapping,
    text: "Wrap the body at 72 characters",
};

fn test_exaplain_what_and_why (_: &str) -> bool {
    true
}

const WHAT_AND_WHY_EXPLANATION: CommitRule = CommitRule {
    test: test_exaplain_what_and_why,
    text: "Use the body to explain what and why vs. how",
};

trait FetchedAdd {
    fn fetch_add (&mut self, value: Self) -> Self;
}

impl FetchedAdd for u8 {
    fn fetch_add (&mut self, value: Self) -> Self {
        let next_value = u8::wrapping_add(*self, value);
        mem::replace(self, next_value)
    }
}

impl Iterator for CommitRuleIterator {
    type Item = CommitRule;

    fn next(&mut self) -> Option<CommitRule> {
        match self.index.fetch_add(1)  {
            0 => Some(SUBJECT_BODY_SEPARATION),
            1 => Some(SUBJECT_LINE_LIMIT),
            2 => Some(SUBJECT_CAPITALIZATION),
            3 => Some(SUBJECT_PUNCTUATION),
            4 => Some(IMPERATIVE_MOOD),
            5 => Some(BODY_WRAPPING),
            6 => Some(WHAT_AND_WHY_EXPLANATION),
            _ => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (7 - self.index as usize, Some(7 - self.index as usize))
    }
}

impl ExactSizeIterator for CommitRuleIterator {
    fn len(&self) -> usize {
        7
    }
}
