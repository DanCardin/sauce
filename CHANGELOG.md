# Changelog

### [v0.6.6](https://github.com/DanCardin/sauce/compare/v0.6.5...v0.6.6) (2021-02-21)

#### Features

* Describe the cascaded saucefiles being loaded. 1ffa279


### [v0.6.5](https://github.com/DanCardin/sauce/compare/v0.6.4...v0.6.5) (2021-02-15)

#### Features

* Add `sauce move` command. 5872f83

#### Fixes

* Add missing clear-ignore config setting for subcommend. b904035


### [v0.6.4](https://github.com/DanCardin/sauce/compare/v0.6.3...v0.6.4) (2021-02-12)

#### Fixes

* escape behavior of escaped newlines. e9d7e41


### [v0.6.3](https://github.com/DanCardin/sauce/compare/v0.6.2...v0.6.3) (2021-02-09)


### [v0.6.2](https://github.com/DanCardin/sauce/compare/v0.6.1...v0.6.2) (2021-02-06)


### [v0.6.1](https://github.com/DanCardin/sauce/compare/v0.6.0...v0.6.1) (2021-02-06)


## [v0.6.0](https://github.com/DanCardin/sauce/compare/v0.5.1...v0.6.0) (2021-02-06)

### Features

* Add shell exec command. 97a473a
* Add quiet and verbose flags and update some help text. 44ffd85


### [v0.5.1](https://github.com/DanCardin/sauce/compare/v0.5.0...v0.5.1) (2021-02-01)

#### Features

* Add options for configuring color output. d35f014
* Propagate output to more locations and colorize most output. 9b8487f

#### Fixes

* stdin blocking on set var when there is no stdin. c22dcf9


## [v0.5.0](https://github.com/DanCardin/sauce/compare/v0.4.0...v0.5.0) (2021-01-27)

### Features

* Add fish support and remove autodetect. 4afc9f9
* Add autoload functionality to bash. 8d87890

### Fixes

* Use grcov action instead of installing it manually. 5a7ac24
* Incorrect ci trigger branch name. fb9dd38


## [v0.4.0](https://github.com/DanCardin/sauce/compare/v0.3.0...v0.4.0) (2021-01-23)

### Features

* Add code coverage. eedfba6
* Add autoload feature and settings for zsh. 2511e3c
* Add the ability to autoload and settings to toggle it. fda3420

### Fixes

* The addition of the autoload flag caused normal `sauce` to start failing. f8b21d7


## [v0.3.0](https://github.com/DanCardin/sauce/compare/v0.2.1...v0.3.0) (2021-01-18)

### Features

* Protect ancestor saucefiles from modification. dcb86f1
* Detect and dispatch to per-shell behavior. da2c43e

### Fixes

* Avoid one of the collect calls in glob parser. 1754845


### [v0.2.1](https://github.com/DanCardin/sauce/compare/v0.2.0...v0.2.1) (2021-01-15)

#### Features

* Add CI/releases. 90cd2b4
* Add optional path to the "edit" command. 7641fd8

#### Fixes

* Cargo.toml license. 9b1c7e4


## [v0.2.0](https://github.com/DanCardin/sauce/compare/v0.1.2...v0.2.0) (2021-01-13)


### v0.1.2 (2021-01-12)

#### Fixes

* Multiline format preservation (shell quoting issue). 49c1c23
* Switch to snailquote to properly handle double-quoting multiline strings. 939a67b


