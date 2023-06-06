# git_rnd_name

## Description

This is a tool to generate random names for git repositories.

This is useful when you want to create a `PR` but you don't know what name you should use.

## Features

- Generate random names for git repositories.
- Create new branch with random names.
- Support verbose mode.
- Support custom local repository path.

You'll get a random name like this:

```bash
w-mai@MacBook-Pro ~/P/X/E/lvgl (😀)> git branch | column
  ☹️				 😟
  👌🏻				 😣
* 😀				 😩
  😁				 😫
  😏				 🙁
  😒				 🤓
  😔				 🤩
  😕				 🥳
  😖				 🥺
  😗				 🫢
```

## Installation

```bash
cargo install git_rnd_name
```

## Usage

```bash
grn --help
```

```bash
Generate a random git branch name based on remote name you given.

Usage: grn [OPTIONS] <REMOTE>

Arguments:
  <REMOTE>  remote names

Options:
  -c, --repo <REPO>  local repo path
  -b, --branch       create new branch
  -v, --verbose...   verbose mode
  -h, --help         Print help
  -V, --version      Print version
```

## Example

Chane to your git repository directory.

```bash
grn origin
```

> You will get a random name like `😁`

```bash
grn origin -b
```

> You will get a random name like `😂` and a new branch will be created.

```bash
grn origin -c /path/to/repo
```

> You will get a random name like `??` in `/path/to/repo`. If you add `-b` option, a new branch will be created.

## How to build

1. Star and fork this repository
2. Clone your forked repository like `git clone https://github.com/yourname/git_rnd_name.git`
3. `cd git_rnd_name`

```bash
cargo build --release
```

## License

MIT
