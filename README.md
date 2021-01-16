# Sauce

<p align="center">
<a href="https://github.com/DanCardin/sauce/actions?query=workflow%3ATest"><img src="https://github.com/DanCardin/sauce/workflows/Test/badge.svg" alt="Build Status"></a>
<img src="https://img.shields.io/crates/l/sauce.svg" alt="license">
<a href="https://crates.io/crates/sauce"><img src="https://img.shields.io/crates/v/sauce.svg?colorB=319e8c" alt="Version info"></a><br>
</p>

> The central truth is the central truth, and nothing that I care about
> is relative

A tool to help manage context/project specific shell-things like
environment variables.

Core Goals:

- Store all data centrally, not relative to the directory being sauced
- Cascade data from parent directories downwards

## Installation

### With Cargo

- `cargo install sauce`

### Download Release

- Download Linux/Mac binary from
  [Releases](https://github.com/DanCardin/sauce/releases)

## Example Workflow

``` bash
# Suppose you've got some new directory structure
❯ mkdir -p foo/bar
❯ cd foo

# You want to start recording things here
❯ sauce
No file at ~/.local/share/sauce/foo.toml
❯ sauce new

# My "foo" project has got some corresponding aws profile
❯ sauce set var AWS_PROFILE=foo

# The "bar" subdirectory has something more specific
❯ cd bar
❯ sauce set var foo=bar

# The core purpose!
❯ sauce
Sourced ~/.local/share/sauce/foo/bar.toml

❯ env
...
AWS_PROFILE=foo
foo=bar
```

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

Planned/Ideally supported targets include:

- functions
- arbitrary kv

## Features

### `sauce` command

The primary usecase is the `sauce` command. Explicitly no arguments, you
load the environment with all sauce targets, cascaded from the uppermost
parent.

#### Cascades

Given a directory structure

  ~/
      projects/
          foo/
              subfoo/
          bar/

You can run `sauce` at any folder level/depth, say `subfoo`. The values
saved for the folders: `~`, `~/projects`, `~/projects/foo`, and
`~/projects/foo/subfoo` will all be loaded.

The more specific/deep folder’s values will take precedence over the
values of more general/shallow folders.

All saucefiles are located in the `$XDG_DATA_HOME/sauce` folder, after
which the folder structure mirrors that of the folders who’s values are
being tracked. Given the above example you might see:

  ~/.local/share/sauce/

          foo/
              subfoo.toml
          foo.toml
          bar.toml
      projects.toml

### `sauce add <target-type> NAME=value`

For example, `sauce set var AWS_PROFILE=foo FOO=bar`.

This is convenient when you realize you want to `sauce` a var or
whatever. There is also `sauce edit` which will open your `$EDITOR` so
you can bulk update whatever values you like.

### `sauce --as foo`

Any key-value pair can be tagged with, you might call “namespaces”.

Consider an env var definition

``` toml
AWS_PROFILE = {default = "projectname-dev", uat = "projectname-uat", prod = "projectname-prod"}
```

Given a `sauce`, you will get the “default” namespace
(i.e. AWS\_PROFILE=projectname-dev) for this value, as well as all other
unnamespaced values.

Given `sauce --as prod`, you will get the “prod” namespace
(i.e. AWS\_PROFILE=projectname-prod) for this value, as well as all
other unnamespaced values.

## Planned Work

- more robust shell-specific trait behavior
- protect cascaded lookups from editing by internally storing them
  separately
- tests/code coverage
- refactor into thin cli app + library
- “strategies” (nested shell vs in-place alterations of the current
  shell)
  - Given strategies, the ability to unset/revert alterations in a more
    robust way is enabled (i.e. kill the subshell). Compared to the
    in-place modification strategy which essentially requires that
    `sauce` maintains sole control over all tracked variables (because
    it can/will `unset` them if asked).
- more targets: arbitrary key-value pairs
- pipe `sauce show` to a pager when beyond a full terminal height
- colorized output
- Ability to use shell hooks to automatically perform i.e. `sauce` on
  `cd` (i.e. direnv)
