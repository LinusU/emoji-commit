/*
 * From libgit2 "log" example written by the libgit2 contributors
 * <https://github.com/rust-lang/git2-rs/blob/master/examples/log.rs>
 *
 * To the extent possible under law, the author(s) have dedicated all copyright
 * and related and neighboring rights to this software to the public domain
 * worldwide. This software is distributed without any warranty.
 *
 * You should have received a copy of the CC0 Public Domain Dedication along
 * with this software. If not, see
 * <http://creativecommons.org/publicdomain/zero/1.0/>.
 */

use std::path::Path;
use std::error::Error;

use git2::{ObjectType, Repository};

pub(crate) fn get_commit_messages<P: AsRef<Path>>(repo_path: P, refspecs: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    let repo = Repository::discover(repo_path)?;
    let mut revwalk = repo.revwalk()?;

    for commit in refspecs {
        if commit.starts_with('^') {
            let obj = repo.revparse_single(&commit[1..])?;
            revwalk.hide(obj.id())?;
            continue;
        }
        let revspec = repo.revparse(commit)?;
        if revspec.mode().contains(git2::RevparseMode::SINGLE) {
            revwalk.push(revspec.from().unwrap().id())?;
        } else {
            let from = revspec.from().unwrap().id();
            let to = revspec.to().unwrap().id();
            revwalk.push(to)?;
            if revspec.mode().contains(git2::RevparseMode::MERGE_BASE) {
                let base = repo.merge_base(from, to)?;
                let o = repo.find_object(base, Some(ObjectType::Commit))?;
                revwalk.push(o.id())?;
            }
            revwalk.hide(from)?;
        }
    }

    let mut messages: Vec<String> = Vec::new();

    for object_id in revwalk {
        let commit = repo.find_commit(object_id?)?;
        messages.push(String::from(commit.message().unwrap()));
    }

    Result::Ok(messages)
}
