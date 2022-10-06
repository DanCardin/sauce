# Configuration

All settings can be set under the top-level `[settings]` table, for
example:

    # ~/.config/sauce.toml
    [settings]
    autoload-hook = true

Future settings may be further namespaced into subtables, but will
always live under the top-level `[settings]` table.

## Reference

### `autoload-hook`

Defaults to `false`. When `true`, emits the requisite shell code (during
intitialization) to **enable** automatic running of `sauce` on shell
startup and changing directory.

Note this does not actually enable the autoload feature!

_Handy Tip!_ I set this to `true` at the **global** level, because
that’s what will always end up loaded at every shell start.

### `autoload`

Defaults to `false`. When `true` **actually** enables autoload behavior
(assuming `autoload-hook` was enabled at shell intitialization time).
“Autoload behavior” causes `sauce` to be invoked upon both new shells as
well as when changing directory.

_Handy Tip!_ I set this to `true` at the **local** level, enabling me to
opt in to autoload in whatever directories I like, which I find I more
frequently prefer.

### `autoload-args`

Defaults to `""`. By default, autoloaded invocations are effectively an
argument-less call to `sauce`. When set, this causes the `sauce shell init`
command to emit the `autoload-args`'s value into the shell wrapper around
sauce such that autoloaded invocations also include those arguments.

### `clear-ignore`

Defaults to `[]`. When set, values should be `string`s and their values
are interpreted as though from the `--filter` flag. That is to say, each
string can either be a literal exact string match like `"AWS_PROFILE"`,
or prefixed with the target name (env, alias, or function) like
`"env:AWS_PROFILE"`.

When a list item matches a given target’s key (i.e. env var name, alias
name, function name) it will then be **excluded** from from the set of
values returned for whatever subcommand you’re invoking.

_Handy Tip!_ I sometimes use this to set values like `$PATH` in
directories that i’m sure I’ll only `sauce` once, while not potentially
breaking my shell by unsetting `$PATH`.

## `sauce config` subcommand

There exists a `sauce config` command which you can use to set config
like `sauce config autoload=true` or
`sauce config --global autoload=true`.

## Local vs Global

Configuration is loaded with priority towards the most-specific config
always taking precedence over less specific config, similar to `git`.

The global configuration location is decided using XDG, using the
`XDG_CONFIG_HOME` environment variable or the XDG default. This will
typically be `~/.config/sauce.toml`.

The saucefile pertaining to the directory of a `sauce` invocation serves
double purpose as the local config for that `sauce` invocation.
Therefore, if a particular configuration value is included in your
saucefile, that value will override the global value.

### Example

Global:

    # ~/.config/sauce.toml
    autoload-hook = true
    autoload = false

Local:

    # ~/.local/share/sauce/example.toml
    autoload = true

In this case, invoking `sauce` from `~/example/` will target the above
saucefile.

- `autoload-hook`=`true`: Excluded from local config, included in global
  config, uses global value
- `autoload`=`true`: Included in local config, included in global
  config, uses local value
- `clear-ignore`=`[]`: Excluded from both local and global config, uses
  the global default
