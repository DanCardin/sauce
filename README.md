# Sauce
> The central truth is the central truth, and nothing that I care about is relative

```bash
❯ mkdir -p foo/bar
❯ cd foo
❯ sauce
No file at ~/.local/share/sauce/foo.toml
❯ sauce new
❯ sauce add var AWS_PROFILE=foo
❯ cd bar
❯ sauce add var foo=bar
❯ sauce
source ~/.local/share/sauce/foo.toml
source ~/.local/share/sauce/foo/bar.toml
❯ env
AWS_PROFILE=foo
foo=bar
# additionally
❯ sauce add alias meow=cowsay
❯ sauce add function foo 'echo "bar"'
❯ sauce add ...
```

Targets
* env vars
* aliases
* arbitrary kv
* functions

Ideal functionality
* Loads info that's stored centrally based on your current directory
* Loads in a cascading fashion from the top level downward
* ability to unset/revert changes to these things
* Namespaces/tags within a target

	```bash
	sauce as prod
	```

* option to automatically perform things on shell cd or whatever (i.e. direnv)

If possible
* wrap zsh/bash through i.e. `sauce shell`. Thus being able to intercept calls to `sauce` 
  inside the `sauce` so you could avoid needing to type `source sauce ...` without
	writing bash
