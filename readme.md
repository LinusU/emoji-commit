# Emoji Commit

Make your git logs beautiful and readable with the help of emojis 🎉

The idea with the emoji committer is to tag each of your commit with an emoji that corresponds to a bump in [semver][1]. This information can the be used to automatically publish new versions, generate a change log and make the git log give you a quicker glance over whats been happening.

## Installation

```sh
cargo install emoji-commit
```

## Usage

The emoji committer can be used in two ways. Either invoked directly, or by configuring git to invoke it.

### Invoke directly

Simply call `emoji_commit` as you would any other command:

```sh
emoji_commit
```

### Configure GIT

You can set the `core.editor` configuration in git the the emoji committer to always use it when committing.

```sh
git configure --global core.editor 'emoji_commit'
```

## The emojis

The following emojis where chosen for the emoji committer:

|Emoji | Name         | Semver | Meaning               |
|------|--------------|--------|-----------------------|
|💥    | Collision    | major  | Breaking change       |
|🎉    | Party popper | minor  | New feature           |
|🐛    | Bug          | patch  | Bugfix                |
|🔥    | Fire         | patch  | Cleanup / Performance |
|🌹    | Rose         |        | Meta                  |

### 💥 Breaking change

Use this commit type if your change is in any way breaking to the intended consumer. Keep in mind that "breaking" has different meaning in different contexts, e.g. adding a field to a struct is a breaking change in Rust, but is generally considered a backwards compatible change in Node.js.

### 🎉 New feature

Use this commit type if you have added a new feature in a fully backwards compatible way. Keep in mind that adding documentation for a previous undocumented feature can qualify under this type, since undocumented APIs aren't a part of the public API.

### 🐛 Bugfix

Use this commit type if you have fixed a bug. The rationale for having two "patch" types is to be able to quickly get a list of all the bugs that have been fixed.

### 🔥 Cleanup / Performance

Use this commit if your change will impact the consumer in some way, be it a documentation change, optimizing an if-statement or simply removing some unnecessary semicolons.

### 🌹 Meta

Use this commit when you change _won't have any impact_ on the consumer. This _does not_ include changes to the code that still should have it "behave the same" since those changes should result in a new build being published.

A common use case for this emoji is editing your `.travis.yml` file to change something with the build, or adding some more tests.

## The version bump

Many publishing tools, e.g. npm, have a step where you'll bump the version in some file, committing that, and then publish everything to a registry. For this specific use case we have introduced a special emoji.

### 🚢 Release

Use this commit type when cutting a new release. Commits with this emoji should preferably be made automatically by some sort of continues delivery system, which also publishes the package.

I hope to release some tools for making this easier in the near future. In the mean time, you can use this handy shortcut for `npm`:

```sh
npm version <bump> -m '🚢 %s'
```

## Troubleshooting

### `git log` doesn't show emojis on macOS

macOS ships with a very outdated `less` that doesn't support showing emojis out-of-the box. Read more [in this blogpost][2] for a proper solution, or run the following command to make it work™.

```sh
git config --global core.pager 'less -r'
```

[1]: http://semver.org
[2]: http://www.recursion.org/2016/6/19/displaying-emoji-in-git-log
