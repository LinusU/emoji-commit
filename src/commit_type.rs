use std::mem;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum CommitType {
    Breaking,
    Feature,
    Bugfix,
    Patch,
    Other,
}

pub struct CommitTypeIterator {
    current: Option<CommitType>
}

impl CommitType {
    pub fn first_variant() -> CommitType { CommitType::Breaking }
    pub fn last_variant() -> CommitType { CommitType::Other }

    pub fn next_variant(&self) -> Option<CommitType> {
        match *self {
            CommitType::Breaking => Some(CommitType::Feature),
            CommitType::Feature => Some(CommitType::Bugfix),
            CommitType::Bugfix => Some(CommitType::Patch),
            CommitType::Patch => Some(CommitType::Other),
            CommitType::Other => None,
        }
    }

    pub fn prev_variant(&self) -> Option<CommitType> {
        match *self {
            CommitType::Breaking => None,
            CommitType::Feature => Some(CommitType::Breaking),
            CommitType::Bugfix => Some(CommitType::Feature),
            CommitType::Patch => Some(CommitType::Bugfix),
            CommitType::Other => Some(CommitType::Patch),
        }
    }

    pub fn iter_variants() -> CommitTypeIterator {
        CommitTypeIterator { current: Some(CommitType::first_variant()) }
    }

    pub fn emoji(&self) -> &'static str {
        match *self {
            CommitType::Breaking => "💥",
            CommitType::Feature => "🎉",
            CommitType::Bugfix => "🐛",
            CommitType::Patch => "🔥",
            CommitType::Other => "🌹",
        }
    }

    pub fn description(&self) -> &'static str {
        match *self {
            CommitType::Breaking => "Breaking",
            CommitType::Feature => "Feature",
            CommitType::Bugfix => "Bugfix",
            CommitType::Patch => "Cleanup / Performance",
            CommitType::Other => "Other",
        }
    }
}

impl Iterator for CommitTypeIterator {
    type Item = CommitType;

    fn next(&mut self) -> Option<CommitType> {
        match self.current {
            Some(commit_type) => mem::replace(&mut self.current, commit_type.next_variant()),
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.current {
            Some(CommitType::Breaking) => (5, Some(5)),
            Some(CommitType::Feature) => (4, Some(4)),
            Some(CommitType::Bugfix) => (3, Some(3)),
            Some(CommitType::Patch) => (2, Some(2)),
            Some(CommitType::Other) => (1, Some(1)),
            None => (0, Some(0)),
        }
    }
}

impl ExactSizeIterator for CommitTypeIterator {
    fn len(&self) -> usize {
        5
    }
}
