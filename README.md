# Sauce

<p align="center">
<img src="https://img.shields.io/crates/l/sauce.svg" alt="license">
<a href="https://crates.io/crates/sauce">
<img src="https://img.shields.io/crates/v/sauce.svg?colorB=319e8c" alt="Version info">
</a>
<a href="https://github.com/DanCardin/sauce/actions?query=workflow%3ATest">
<img src="https://github.com/DanCardin/sauce/workflows/Test/badge.svg" alt="Build Status">
</a> <a href="https://codecov.io/gh/DanCardin/sauce">
<img src="https://codecov.io/gh/DanCardin/sauce/branch/main/graph/badge.svg?token=U7NQIWXWKW"/>
</a><br>
</p>

> The central truth is the central truth, and nothing that I care about
> is relative

A tool to help manage context/project specific shell-things like
environment variables.

## Table of Contents

- [Docs](./doc)

  - [Configuration Reference](./doc/config.md)
  - [Flag and Subcommand Reference](./doc/options.md)
  - [Comparison to other tools
    (i.e. direnv/dotenv)](./doc/comparison.md)
  - [Plans](./doc/plans.md)

- [Example Workflow](#example-workflow)

- [Setup](#setup)

  - [Install](#install)
  - [Shell Hook](#shell-hook)

- [Targets](#targets)

- [Features](#features)

  - [sauce](#sauce)
  - [Central Storage](#central-storage)
  - [Cascaded Loading](#cascaded-loading)
  - [Autoloading](#autoloading)

- [Local Development](#local-development)

## Example Workflow

``` bash
# Suppose you've got some new directory structure
❯ mkdir -p foo/bar
❯ cd foo

# You want to start recording things here
❯ sauce new

# My "foo" project has got some corresponding aws profile
❯ sauce set var AWS_PROFILE=foo

# The "bar" subdirectory has something more specific
❯ cd bar
❯ sauce set var foo=bar

# The core purpose!
❯ sauce
Sourced ~/.local/share/sauce/foo/bar.toml

# Note the cascaded loading of upstream values!
❯ env
...
AWS_PROFILE=foo
foo=bar
```

## Setup

### Install

#### With Cargo

- `cargo install sauce`

#### Download Release

- Download Linux/Mac binary from
  [Releases](https://github.com/DanCardin/sauce/releases)

### Shell Hook

Currently explicitly supported shells include: `zsh`, `bash`, and
`fish`. The scaffolding exists to support other shells, which should
make supporting other common shells that might require `"$SHELL"`
specific behavior.

Loading things into the environment requires a minimal amount of shell
code to be executed, so after installing the binary (suggestion below),
you will need to add add a hook to your bashrc/zshrc/config.fish, etc.

- bash `eval "$(sauce --shell bash shell init)"`
- zsh `eval "$(sauce --shell zsh shell init)"`
- fish `sauce --shell fish shell init | source`

Depending on the level of similarity to the above shells, you may be
able to get away with using one of the above `shell init` hooks until
explicit support is added

## Targets

A thing which `sauce` can load/unload is called a “target”.

Currently supported targets include:

- environment variables

  ``` bash
  sauce set var FOO=bar
  ```

- aliases

  ``` bash
  sauce set alias g=git
  ```

- functions

  ``` bash
  sauce set function add 'echo $(expr $1 + $2)'
  ```

## Features

### `sauce` command

This is primary usecase is the `sauce` command, no subcommand, no
arguments. This loads the current shell with all sauce targets (env
vars, aliases, and function) which apply to the current directory.

There are also a bunch of [options](./doc/options.md) to allow you to
customize the behavior of `sauce`, for example `sauce --glob DATABASE*`,
`sauce --filter env:AWS_PROFILE`, or `sauce --path ~`.

### Central Storage

The original motivation for central storage was due to getting a new
computer and needing to comb through \~50 repos to find all the random
`.env` files and gitignored notes and whatnot littered all over the
place to make sure nothing got left behind.

However just generally, colocating the sauce data with the actual folder
introduces a number of technical, security, and usability issues that
are circumvented through central storage.

### Cascaded loading

A key feature of `sauce` is that values are loaded in a cascading
fashion relative to the home directory.

This makes it easier to compose targets (env vars, aliases, and shell
functions) among various locations, likely by utilizing the natural
directory structure you might already have.

Given a directory structure

    ~/
      work/
        project/
          repo/
          repo2/
            src/
        otherproject/

Support you run `sauce` at any folder level/depth, say
`~/work/project/repo/`. The values saved for the folders: `~`, `~/work`,
`~/work/project`, and `~/work/project/repo` will all be loaded.

The more specific/deep folder’s values will take precedence over the
values of more general/shallow folders.

All saucefiles are located in the `$XDG_DATA_HOME/sauce` folder, after
which the folder structure mirrors that of the folders who’s values are
being tracked. Given the above example, if every folder had a saucefile,
you might see:

    ~/.local/share/
      sauce.toml
      sauce/
        project.toml
        project/
          repo.toml
          repo2.toml
          repo2/
            src.toml
        otherproject.toml

### Autoloading

See the [Configuration Reference](./doc/config.md) on `autoload-hook`
and `autoload`.

## Local development

For local development, it can be useful to enable the `--feature dev`.
This alters the behavior so that the shell hook(s) point to the absolute
location of the debug build.

An example alias that might be helpful could be:

``` toml
[alias]
build = 'cargo build --features dev && eval "$(./target/debug/sauce shell init)"'
```

At which point, you’re a quick `build` away from being able to `cd`
around to test `sauce`, while always pointing at your project version of
`sauce` for the current shell.
