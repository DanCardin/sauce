# Options

Options are generally applied as flags to the top-level `sauce` command,
and should generally trickle down to subcommands where it makes sense to
do so.

For example `sauce --path ~ edit` or `sauce --glob DATABASE* show`, etc
should all logically operate the way you would expect, given the
combination of the flag and the subcommand.

## `sauce --as foo`

Any key-value pair can be tagged with, you might call “namespaces”.

Consider an env var definition

``` toml
AWS_PROFILE = {default = "projectname-dev", uat = "projectname-uat", prod = "projectname-prod"}
```

Given `sauce`, you will get the “default” namespace
(i.e. AWS\_PROFILE=projectname-dev) for this value, as well as all other
unnamespaced values.

Given `sauce --as prod`, you will get the “prod” namespace
(i.e. AWS\_PROFILE=projectname-prod) for this value, as well as all
other unnamespaced values.

## `sauce --glob glob` and `sauce --filter filter`

Either `--glob` and/or `--filter` can be applied in order to filter down
the set of things which are returned from `sauce` (or any subcommand).

You can supply multiple globs/filters by separating them by `,`,
i.e. `--filter foo,bar,baz`.

You can also specify globs/filters specific to a particular target,
which might be important given that there can be overlap between
targets. Targets are separated from their search term by `:`,
i.e. `--glob env:database*,function:work-*`.

## `sauce --show`

This option is essentially a “dry run” option. vanilla `sauce`
invocations will output the `$SHELL` code which **would** have been
executed rather than actually executing it. Essentially, stdout gets
redirected to stderr.

Additionally, `sauce set` and `sauce config` will only print out the
target/config changes they would have made.

## `sauce --path ~`

Executes `sauce` as though you were at the provided path. This will
continue to use the normal cascading behavior and lookup mechanisms.

## `sauce --file foo.toml`

Choose a specific toml file to load, rather than using the default
cascading and lookup mechanisms.

# Commands

## `sauce clear`

`clear`ing will “unset” everything defined in any cascaded saucefiles,
abiding by any options (–filter/–glob/–as) provided to the command.

The general intent is that one would only/primarily be including targets
which would be safe to unset (or that you will avoid running `clear` if
that’s not true for you), given that they were overwritten when you run
`sauce`.

## `sauce config autoload=true`

Sets config values (with an optional `--global` flag to set global
config).

## `sauce edit`

Opens your `$EDITOR` with the saucefile for the current location. This
is useful for bulk update values rather than one-at-a-time `set`
commands.

Note this simply opens the editor, if you’re doing this at a location
for which there isn’t already a supporting folder structure
(i.e. `~/a/b/c/d` needs to create `~/.local/share/a/b/c/d.toml`) you may
need to first run `sauce new`.

## `sauce new`

Creates a new saucefile for the location (and the intervening folder
structure).

## `sauce set <target-type> NAME=value`

For example, `sauce set var AWS_PROFILE=foo FOO=bar`.

This is convenient when you realize you want to `sauce` a singular var.
There is also `sauce edit` command which will open your `$EDITOR` so you
can bulk update whatever values you like.

## `sauce shell`

### `sauce shell init`

Loading things into the environment requires a minimal amount of shell
code to be executed, so after installing the binary (suggestion below),
you will need to add add a hook to your bashrc/zshrc/config.fish, etc.

- bash `eval "$(sauce --shell bash shell init)"`
- zsh `eval "$(sauce --shell zsh shell init)"`
- fish `sauce --shell fish shell init | source`

### `sauce shell exec <cmd>`

Runs a given `<cmd>` after entering a subprocess shell that has had
`sauce` run inside of it. Potentially useful to avoid polluting the
current shell with target values.

## `sauce show env`

Pretty prints a table of the given target.
