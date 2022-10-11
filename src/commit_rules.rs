use std::iter;
use emoji_commit_type::CommitType;

pub struct CommitRule {
    pub test: fn(input: &str) -> bool,
    pub text: &'static str,
}

pub struct CommitRuleValidationResult {
    pub description: &'static str,
    pub pass: bool
}

impl PartialEq for CommitRule {
    fn eq(&self, other: &CommitRule) -> bool {
        // FIXME: Not the best check...
        self.text == other.text
    }
}

impl Eq for CommitRule {}

fn test_subject_body_separation (input: &str) -> bool {
    input.lines().nth(1).map(|line| line.is_empty()).unwrap_or(true)
}

const SUBJECT_BODY_SEPARATION: CommitRule = CommitRule {
    test: test_subject_body_separation,
    text: "Separate subject from body with a blank line",
};

fn test_subject_line_limit (input: &str) -> bool {
    input.lines().next().unwrap_or("").len() <= 50
}

const SUBJECT_LINE_LIMIT: CommitRule = CommitRule {
    test: test_subject_line_limit,
    text: "Limit the subject line to 50 characters",
};

fn test_subject_capitalization (input: &str) -> bool {
    input.chars().next().map(|first| first.is_uppercase()).unwrap_or(true)
}

const SUBJECT_CAPITALIZATION: CommitRule = CommitRule {
    test: test_subject_capitalization,
    text: "Capitalize the subject line",
};

fn test_subject_punctuation (input: &str) -> bool {
    !input.ends_with('.')
}

const SUBJECT_PUNCTUATION: CommitRule = CommitRule {
    test: test_subject_punctuation,
    text: "Do not end the subject line with a period",
};

fn test_imperativ_mood (input: &str) -> bool {
    let lower = input.to_lowercase();

    !(
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

fn test_starting_emoji (input: &str) -> bool {
    CommitType::iter_variants().any(|commit_type| {
        input.starts_with(commit_type.emoji())
    })
}

const STARTING_EMOJI_EXPLANATION: CommitRule = CommitRule {
    test: test_starting_emoji,
    text: "Commit message has to begin with one of the following emojis: 💥, 🎉, 🐛, 🔥, 🌹",
};

const RULES: [CommitRule; 7] = [
    SUBJECT_BODY_SEPARATION,
    SUBJECT_LINE_LIMIT,
    SUBJECT_CAPITALIZATION,
    SUBJECT_PUNCTUATION,
    IMPERATIVE_MOOD,
    BODY_WRAPPING,
    WHAT_AND_WHY_EXPLANATION,
];

pub fn check_message(message: &str) -> impl Iterator<Item=CommitRuleValidationResult> + '_ {
    RULES.iter().map(move |rule| CommitRuleValidationResult{
        description: rule.text,
        pass: (rule.test)(message)
    })
}

pub fn check_message_with_emoji(message: &str) -> impl Iterator<Item=CommitRuleValidationResult> + '_ {
    let emoji_validation_result = CommitRuleValidationResult{
        description: STARTING_EMOJI_EXPLANATION.text,
        pass: (STARTING_EMOJI_EXPLANATION.test)(message)
    };
    let message_without_emoji = CommitType::iter_variants().fold(message, |message, commit_type| {
        let emoji = format!("{} ", commit_type.emoji());
        if message.starts_with(&emoji) { &message[emoji.len()..] } else { message }
    });
    iter::once(emoji_validation_result).chain(
        check_message(message_without_emoji)
    )
}
