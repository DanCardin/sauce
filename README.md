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

Core Goals:

- Store all data centrally, not relative to the directory being sauced
- Cascade data from parent directories downwards

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

## Installation

### Install

#### With Cargo

- `cargo install sauce`

#### Download Release

- Download Linux/Mac binary from
  [Releases](https://github.com/DanCardin/sauce/releases)

### Setup

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

### `sauce set <target-type> NAME=value`

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

### `sauce --glob glob` and `sauce --filter filter`

Either `--glob` and/or `--filter` can be applied in order to filter down
the set of things which are returned from `sauce` (or any subcommand).

You can supply multiple globs/filters by separating them by `,`,
i.e. `--filter foo,bar,baz`.

You can also specify globs/filters specific to a particular target,
which might be important given that there can be overlap between
targets. Targets are separated from their search term by `:`,
i.e. `--glob env:database*,function:work-*`.

### `sauce clear`

`clear`ing will “unset” everything defined in any cascaded saucefiles,
abiding by any options (–filter/–glob/–as) provided to the command.

The general intent is that one would only/primarily be including targets
which would be safe to unset (or that you will avoid running `clear` if
that’s not true for you), given that they were overwritten when you run
`sauce`.

## Settings

### direnv-like automatic execution of `sauce` on `cd`

`sauce` loads configuration from `$XDG_CONFIG_HOME/sauce.toml`, and so
generally this will be `~/.config/sauce.toml`.

By default this feature is off (given that it changes the shell
generated on `sauce shell init` and causes it to be executed on every
`cd`).

To enable, add:

``` toml
# Enables the shell hook which makes this feature possible.
# You must start a new shell before autoload will work.
autoload-hook = true

# Enables the feature itself (globally).
autoload = true
```

You can additionally/alternatively omit `autoload` from the global
config, and instead opt to only include it in the saucefile for a given
directory i.e.

``` toml
# ~/.local/share/sauce/work/example.toml
[settings]
autoload = true
```

Which allows you to globally opt in, globally opt out, locally opt in,
or locally opt out.

## Alternatives

Why would you choose to use `sauce` over certain alternatives? `sauce`
**does** have significant conceptual feature overlap with, in
particular, `direnv`, but the additional features may make it worth
using `sauce` instead!

Features which distinguish `sauce` from **all** the below alternatives

- Cascading loading of values from upstream directories

  This makes it easier to compose targets (env vars, aliases, and shell
  functions) among various locations, likely by utilizing the natural
  directory structure you might already have.

- Central storage of the values

  The original motivation for central storage was due to getting a new
  computer and needing to comb through 50 repos to find all the random
  `.env` files littered all over the place to make sure nothing got left
  behind.

  In practice, I’ve found that there are numerous problems with storing
  your (equivalent to `.env`/`.envrc` files locally to the repo. In fact
  `direnv` added a whole additional feature `direnv allow` to work
  around that fact.

### `dotenv`

`dotenv` is the more obvious of the two, which is specifically for the
loading of environment variables, which is essentially the primary
usecase of `sauce`.

I find choice of `toml` files for `sauce`, while not necessarily
important to the actual featureset of `sauce`, to be useful because it
supports multiline strings.

The main advantage, is `sauce --as <context>`. In order to reproduce
this behavior, you need multiple dotenv files, (with all common values
duplicated, or contained in yet another file).

### `direnv`

`direnv` may be less obvious, but given that it executes user-supplied
shell code, directly, on `cd`, it can essentially do anything that
`sauce` can do.

The advantage, lies in the fact that `sauce` is specifically tailored to
the usecases documented above. That means, it’s just a lot easier to get
the behaviors of `sauce` compared to writing the shell code (for each
project) which would enable things like `sauce --as prod` or
`sauce --glob database*`.

## Planned Work

- Invoke autoload on shell startup instead of **just** on explicit chpwd

- ability to specify values which should not react to `clear`

  This might be useful for environment variables like `PATH` or
  `PROMPT`, which would otherwise be very unsafe to include, unless you
  **never** run `clear`.

- ability to subdivide targets by shell

  i.e. allow one to specify `[alias.fish]`.

- `sauce config subshell=false/true` (default `false`)

  Given subshell=true, a call to `sauce` would create a subprocess into
  which the alterations would be made. This would enable one to simply
  kill the current shell to revert state.

- more targets: arbitrary key-value pairs

- pipe `sauce show` to a pager when beyond a full terminal height

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
