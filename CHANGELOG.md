# Changelog

## Overview

- [unreleased](#unreleased)
- [`0.6.4`](#064) – _2021.02.12_
- [`0.6.3`](#063) – _2021.02.09_
- [`0.6.2`](#062) – _2021.02.06_
- [`0.6.1`](#061) – _2021.02.06_
- [`0.6.0`](#060) – _2021.02.06_
- [`0.5.1`](#051) – _2021.02.01_
- [`0.5.0`](#050) – _2021.01.27_
- [`0.4.0`](#040) – _2021.01.23_
- [`0.3.0`](#030) – _2021.01.18_
- [`0.2.1`](#021) – _2021.01.15_
- [`0.2.0`](#020) – _2021.01.13_
- [`0.1.2`](#012) – _2021.01.12_

## _[Unreleased]_

_nothing new to show for… yet!_

## [0.6.4]

_2021.02.12_

### Changes

## [0.6.3]

_2021.02.09_

### Changes

## [0.6.2]

_2021.02.06_

### Changes

## [0.6.1]

_2021.02.06_

### Changes

## [0.6.0]

_2021.02.06_

### Changes

## [0.5.1]

_2021.02.01_

### Changes

## [0.5.0]

_2021.01.27_

### Contributors


- Dan Cardin (<ddcardin@gmail.com>)

### Changes

#### Bug Fixes

- **Incorrect ci trigger branch name.** ([`fb9dd38`])

## [0.4.0]

_2021.01.23_

### Contributors


- Dan Cardin (<ddcardin@gmail.com>)

### Changes

#### Bug Fixes

- **The addition of the autoload flag caused normal `sauce` to start failing.** ([`f8b21d7`])

  Fix (and tests) reenables normal `sauce` behavior, and autoload behavior
  only kicks in once `--autoload` is used.

#### Features

- **Add autoload feature and settings for zsh.** ([`2511e3c`])

- **Add the ability to autoload and settings to toggle it.** ([`fda3420`])

#### Documentation

- **update readme to include better installation/setup instructions.** ([`f462e1a`])

#### Tests

- **Add saucefile tests.** ([`7cb1b44`])

## [0.3.0]

_2021.01.18_

### Contributors


- Dan Cardin (<ddcardin@gmail.com>)

### Changes

#### Tests

- **Add tests for filter parsing logic.** ([`b59f166`])

#### Features

- **Protect ancestor saucefiles from modification.** ([`dcb86f1`])

  Internally store them in a separate structure to prevent them
  from being accidentally updated through i.e. a bug.

- **Detect and dispatch to per-shell behavior.** ([`da2c43e`])

#### Bug Fixes

- **Avoid one of the collect calls in glob parser.** ([`1754845`])

## [0.2.1]

_2021.01.15_

### Contributors


- Dan Cardin (<ddcardin@gmail.com>)

### Changes

#### doc

- **Add badges to the readme.** ([`40796cc`])

#### Features

- **Add CI/releases.** ([`90cd2b4`])

- **Add optional path to the "edit" command.** ([`7641fd8`])

#### Bug Fixes

- **Cargo.toml license.** ([`9b1c7e4`])

## [0.2.0]

_2021.01.13_

### Changes

## [0.1.2]

_2021.01.12_

### Contributors


- Dan Cardin (<ddcardin@gmail.com>)

### Changes

#### Bug Fixes

- **Multiline format preservation (shell quoting issue).** ([`49c1c23`])

- **Switch to snailquote to properly handle double-quoting multiline strings.** ([`939a67b`])

[unreleased]: https://github.com/DanCardin/sauce/compare/v0.6.4...HEAD
[0.6.4]: https://github.com/DanCardin/sauce/releases/tag/v0.6.4
[0.6.3]: https://github.com/DanCardin/sauce/releases/tag/v0.6.3
[0.6.2]: https://github.com/DanCardin/sauce/releases/tag/v0.6.2
[0.6.1]: https://github.com/DanCardin/sauce/releases/tag/v0.6.1
[0.6.0]: https://github.com/DanCardin/sauce/releases/tag/v0.6.0
[0.5.1]: https://github.com/DanCardin/sauce/releases/tag/v0.5.1
[0.5.0]: https://github.com/DanCardin/sauce/releases/tag/v0.5.0
[0.4.0]: https://github.com/DanCardin/sauce/releases/tag/v0.4.0
[0.3.0]: https://github.com/DanCardin/sauce/releases/tag/v0.3.0
[0.2.1]: https://github.com/DanCardin/sauce/releases/tag/v0.2.1
[0.2.0]: https://github.com/DanCardin/sauce/releases/tag/v0.2.0
[0.1.2]: https://github.com/DanCardin/sauce/releases/tag/v0.1.2


[`fb9dd38`]: https://github.com/DanCardin/sauce/commit/fb9dd38c23d126801b63d5e193020564f6487a3d
[`f8b21d7`]: https://github.com/DanCardin/sauce/commit/f8b21d7b9500aab40a125839686593b35fc4dd13
[`2511e3c`]: https://github.com/DanCardin/sauce/commit/2511e3ccf4eed9bc73e7a7f8c275aaaa23c14116
[`f462e1a`]: https://github.com/DanCardin/sauce/commit/f462e1a4e32e3a16ad1f07de43e74b3be21c2781
[`fda3420`]: https://github.com/DanCardin/sauce/commit/fda34204331d6cc362c6c0f718b762fa30c8fd7f
[`7cb1b44`]: https://github.com/DanCardin/sauce/commit/7cb1b44f41f1075a7d87ce790a9a80cacc205654
[`b59f166`]: https://github.com/DanCardin/sauce/commit/b59f166250be6a37011a14329241d4d0628cb1fe
[`dcb86f1`]: https://github.com/DanCardin/sauce/commit/dcb86f1db357c8a90af2e21b6397626c18c3af98
[`1754845`]: https://github.com/DanCardin/sauce/commit/1754845194994c8aa5b0c260d3fc8d31b5773fb1
[`da2c43e`]: https://github.com/DanCardin/sauce/commit/da2c43e6b1a4bdac6cfb5d469b1827f11441afc6
[`40796cc`]: https://github.com/DanCardin/sauce/commit/40796cccaf4a3b62831fc72425200119d9d8e7f5
[`90cd2b4`]: https://github.com/DanCardin/sauce/commit/90cd2b4fdf9a99ba5bd05701234df7f4caf95318
[`9b1c7e4`]: https://github.com/DanCardin/sauce/commit/9b1c7e4f17903a523f6ba19e9638bcdd7a6916fc
[`7641fd8`]: https://github.com/DanCardin/sauce/commit/7641fd8d854db072dfff41ad0b4cd2afda2c0992
[`49c1c23`]: https://github.com/DanCardin/sauce/commit/49c1c2310048bc0472f867464bb0a9907433e476
[`939a67b`]: https://github.com/DanCardin/sauce/commit/939a67ba13a37d081128d5ed7a28d1acfba6a647
<!--
Config(
  github: ( repo: "DanCardin/sauce" ),
)

Template(
# Changelog

## Overview

- [unreleased](#unreleased)

{%- for release in releases %}
- [`{{ release.version }}`](#{{ release.version | replace(from=".", to="") }}) – _{{ release.date | date(format="%Y.%m.%d")}}_
{%- endfor %}

## _[Unreleased]_

{% if unreleased.changes -%}
{%- for change in unreleased.changes -%}
- {{ change.type }}: {{ change.description }} ([`{{ change.commit.short_id }}`])
{% endfor %}
{% else -%}
_nothing new to show for… yet!_

{% endif -%}
{%- for release in releases -%}
## [{{ release.version }}]{% if release.title %} – _{{ release.title }}_{% endif %}

_{{ release.date | date(format="%Y.%m.%d") }}_
{%- if release.notes %}

{{ release.notes }}
{% endif -%}
{%- if release.changeset.contributors %}

### Contributors

{% for contributor in release.changeset.contributors %}
- {{ contributor.name }} (<{{ contributor.email }}>)
{%- endfor %}
{%- endif %}

### Changes

{% for type, changes in release.changeset.changes | group_by(attribute="type") -%}

#### {{ type | typeheader }}

{% for change in changes -%}
- **{{ change.description }}** ([`{{ change.commit.short_id }}`])

{% if change.body -%}
{{ change.body | indent(n=2) }}

{% endif -%}
{%- endfor -%}

{% endfor %}
{%- endfor -%}

{% if config.github.repo -%}
  {%- set url = "https://github.com/" ~ config.github.repo -%}
{%- else -%}
  {%- set url = "#" -%}
{%- endif -%}
{% if releases -%}
[unreleased]: {{ url }}/compare/v{{ releases | first | get(key="version") }}...HEAD
{%- else -%}
[unreleased]: {{ url }}/commits
{%- endif -%}
{%- for release in releases %}
[{{ release.version }}]: {{ url }}/releases/tag/v{{ release.version }}
{%- endfor %}

{% for change in unreleased.changes %}
[`{{ change.commit.short_id }}`]: {{ url }}/commit/{{ change.commit.id }}
{%- endfor -%}
{%- for release in releases %}
{%- for change in release.changeset.changes %}
[`{{ change.commit.short_id }}`]: {{ url }}/commit/{{ change.commit.id }}
{%- endfor -%}
{%- endfor %}
)
-->