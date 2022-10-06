# Changelog

## [Unreleased](https://github.com/DanCardin/sauce/compare/v0.7.2...HEAD) (2022-10-06)

### Features

* Add the ability to configure default arguments supplied to the sauce command.
  ([49a617d](https://github.com/DanCardin/sauce/commit/49a617d8f386b2ac877c1a796b69b377b9629b1c))
* Add -t/--target option to only sauce a specific target.
  ([89ef948](https://github.com/DanCardin/sauce/commit/89ef948998b662d8cef663d17d350e85080dead9))
* Add file target.
  ([b539b5d](https://github.com/DanCardin/sauce/commit/b539b5d2c08ea1dfef7c48b25708a95331ba35b1))

### [v0.7.2](https://github.com/DanCardin/sauce/compare/v0.7.1...v0.7.2) (2022-10-04)

#### Fixes

* Inverted error handling logic for file creation.
  ([4fdee76](https://github.com/DanCardin/sauce/commit/4fdee76cadbff7b3933c575cb4120b847fc0afeb))

### [v0.7.1](https://github.com/DanCardin/sauce/compare/v0.7.0...v0.7.1) (2022-09-29)

#### Fixes

* Address bugs introduced by the switch to corpus which incorrectly prefixed  
relative paths to the new/move commands.
  ([5ce44d3](https://github.com/DanCardin/sauce/commit/5ce44d3483a34cb6b53ee3e56216fae93d6a0fee))

## [v0.7.0](https://github.com/DanCardin/sauce/compare/v0.6.6...v0.7.0) (2022-05-26)

### Features

* Quote shell special characters (and upgrade dependencies).
  ([7f7901d](https://github.com/DanCardin/sauce/commit/7f7901d6ebcfbdd0f80cddfdc2f7e689bbbccee4))
* Use corpus for path logic.
  ([da596c3](https://github.com/DanCardin/sauce/commit/da596c3f839bd2e88a1cc55f514d82a2beae36b4))

### Fixes

* Address clippy warnings.
  ([23ed36f](https://github.com/DanCardin/sauce/commit/23ed36f5052680e7e200b951f60746e532f1e489))

### [v0.6.6](https://github.com/DanCardin/sauce/compare/v0.6.5...v0.6.6) (2021-02-22)

#### Features

* Describe the cascaded saucefiles being loaded.
  ([ade5a15](https://github.com/DanCardin/sauce/commit/ade5a15d7993dbb4aa30d6c742913f7f13e85608))

### [v0.6.5](https://github.com/DanCardin/sauce/compare/v0.6.4...v0.6.5) (2021-02-15)

#### Features

* Add `sauce move` command.
  ([5872f83](https://github.com/DanCardin/sauce/commit/5872f83c7bb1fc3ae896d6c46e3500c33ee80363))

#### Fixes

* Add missing clear-ignore config setting for subcommend.
  ([b904035](https://github.com/DanCardin/sauce/commit/b904035684db3affe4b7cfe62eb139dcadbe2b5d))

### [v0.6.4](https://github.com/DanCardin/sauce/compare/v0.6.3...v0.6.4) (2021-02-12)

#### Fixes

* escape behavior of escaped newlines.
  ([e9d7e41](https://github.com/DanCardin/sauce/commit/e9d7e413882d1a6da005fca15cc38f815b688d3e))

### [v0.6.3](https://github.com/DanCardin/sauce/compare/v0.6.2...v0.6.3) (2021-02-09)

### [v0.6.2](https://github.com/DanCardin/sauce/compare/v0.6.1...v0.6.2) (2021-02-06)

### [v0.6.1](https://github.com/DanCardin/sauce/compare/v0.6.0...v0.6.1) (2021-02-06)

## [v0.6.0](https://github.com/DanCardin/sauce/compare/v0.5.1...v0.6.0) (2021-02-06)

### Features

* Add shell exec command.
  ([97a473a](https://github.com/DanCardin/sauce/commit/97a473aa85b27efcc9ab7aed8b5e5b6457ae6580))
* Add quiet and verbose flags and update some help text.
  ([44ffd85](https://github.com/DanCardin/sauce/commit/44ffd853ff6e5c7a8996a29ff48d75cb459515c8))

### [v0.5.1](https://github.com/DanCardin/sauce/compare/v0.5.0...v0.5.1) (2021-02-01)

#### Features

* Add options for configuring color output.
  ([d35f014](https://github.com/DanCardin/sauce/commit/d35f014cb67f1bf9c5ed8fcef75d3863aa5978c3))
* Propagate output to more locations and colorize most output.
  ([9b8487f](https://github.com/DanCardin/sauce/commit/9b8487f1d1e5cad7599bca6027b8e30565c79f11))

#### Fixes

* stdin blocking on set var when there is no stdin.
  ([c22dcf9](https://github.com/DanCardin/sauce/commit/c22dcf9f19c5c301b370d4ed46ccdf27df0bbbd3))

## [v0.5.0](https://github.com/DanCardin/sauce/compare/v0.4.0...v0.5.0) (2021-01-27)

### Features

* Add fish support and remove autodetect.
  ([4afc9f9](https://github.com/DanCardin/sauce/commit/4afc9f96ec2fa1e75253f95a3194509e43d6ead6))
* Add autoload functionality to bash.
  ([8d87890](https://github.com/DanCardin/sauce/commit/8d87890499884f8056ff6ee893e7c587a0d2d118))

### Fixes

* Use grcov action instead of installing it manually.
  ([5a7ac24](https://github.com/DanCardin/sauce/commit/5a7ac24e5c481bfca2f907561c9a2739292ba890))
* Incorrect ci trigger branch name.
  ([fb9dd38](https://github.com/DanCardin/sauce/commit/fb9dd38c23d126801b63d5e193020564f6487a3d))

## [v0.4.0](https://github.com/DanCardin/sauce/compare/v0.3.0...v0.4.0) (2021-01-23)

### Features

* Add code coverage.
  ([eedfba6](https://github.com/DanCardin/sauce/commit/eedfba6960a2f44e7f2b8c0eb19f74f64add6dba))
* Add autoload feature and settings for zsh.
  ([2511e3c](https://github.com/DanCardin/sauce/commit/2511e3ccf4eed9bc73e7a7f8c275aaaa23c14116))
* Add the ability to autoload and settings to toggle it.
  ([fda3420](https://github.com/DanCardin/sauce/commit/fda34204331d6cc362c6c0f718b762fa30c8fd7f))

### Fixes

* The addition of the autoload flag caused normal `sauce` to start failing.
  ([f8b21d7](https://github.com/DanCardin/sauce/commit/f8b21d7b9500aab40a125839686593b35fc4dd13))

## [v0.3.0](https://github.com/DanCardin/sauce/compare/v0.2.1...v0.3.0) (2021-01-18)

### Features

* Protect ancestor saucefiles from modification.
  ([dcb86f1](https://github.com/DanCardin/sauce/commit/dcb86f1db357c8a90af2e21b6397626c18c3af98))
* Detect and dispatch to per-shell behavior.
  ([da2c43e](https://github.com/DanCardin/sauce/commit/da2c43e6b1a4bdac6cfb5d469b1827f11441afc6))

### Fixes

* Avoid one of the collect calls in glob parser.
  ([1754845](https://github.com/DanCardin/sauce/commit/1754845194994c8aa5b0c260d3fc8d31b5773fb1))

### [v0.2.1](https://github.com/DanCardin/sauce/compare/v0.2.0...v0.2.1) (2021-01-15)

#### Features

* Add CI/releases.
  ([90cd2b4](https://github.com/DanCardin/sauce/commit/90cd2b4fdf9a99ba5bd05701234df7f4caf95318))
* Add optional path to the "edit" command.
  ([7641fd8](https://github.com/DanCardin/sauce/commit/7641fd8d854db072dfff41ad0b4cd2afda2c0992))

#### Fixes

* Cargo.toml license.
  ([9b1c7e4](https://github.com/DanCardin/sauce/commit/9b1c7e4f17903a523f6ba19e9638bcdd7a6916fc))

## [v0.2.0](https://github.com/DanCardin/sauce/compare/v0.1.2...v0.2.0) (2021-01-13)

### v0.1.2 (2021-01-12)

#### Fixes

* Multiline format preservation (shell quoting issue).
  ([49c1c23](https://github.com/DanCardin/sauce/commit/49c1c2310048bc0472f867464bb0a9907433e476))
* Switch to snailquote to properly handle double-quoting multiline strings.
  ([939a67b](https://github.com/DanCardin/sauce/commit/939a67ba13a37d081128d5ed7a28d1acfba6a647))
