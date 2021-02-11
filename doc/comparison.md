# Comparison to existing tools

Why would you choose to use `sauce` over certain alternatives? `sauce`
**does** have significant conceptual feature overlap with, in
particular, `direnv`, but the additional features may make it worth
using `sauce` instead!

Features which distinguish `sauce` from **all** the below alternatives

- Cascading loading of values from upstream directories

  This makes it easier to compose targets (env vars, aliases, and shell
  functions) among various locations, likely by utilizing the natural
  directory structure you might already have.

  For work, I tend to end up with directory structures like:

    ~/
      work/
        project/
          project-repo1/
          project-repo2/
        project2/
          project-repo1/

  Given that kind of structure, it can be very easy and natural to
  manage env vars, but especially functions and aliases at the most
  obvious level of specificity without needing to repeat common things
  inside each project.

- Central storage of the values

  The original motivation for central storage was due to getting a new
  computer and needing to comb through \~50 repos to find all the random
  `.env` files and gitignored notes and whatnot littered all over the
  place to make sure nothing got left behind.

  In practice, I’ve found that there are numerous problems with storing
  your (equivalent to `.env`/`.envrc` files locally to the repo. In fact
  `direnv` added a whole additional feature `direnv allow` to work
  around that fact.

## `dotenv`

`dotenv` is the more obvious of the two, which is specifically for the
loading of environment variables, which is essentially the primary
usecase of `sauce`.

In a world where you only ever run `sauce` (and only use environment
variables), `dotenv` and `sauce` are very similar. Essentially the only
difference in this case would be that `sauce` stores its data centrally,
rather than inside the project directory.

The main advantage, are `sauce --as <context>`. In order to reproduce
this behavior, you need multiple dotenv files, (with all common values
duplicated, or contained in yet another file).

I find choice of `toml` files for `sauce`, while not necessarily
important to the actual featureset of `sauce`, to be useful because it
supports multiline strings. With `dotenv`, I often find myself needing
to manually quote or escape certain values containing shell special
characters in a way that I haven’t with toml (particularly with
multiline values or json).

## `direnv`

`direnv` is likely the other tool people are most likely to think of
looking at the initial pitch.

Given that `direnv`, very executes raw shell code directly for the given
folder, it can essentially do anything. And that includes anything
`sauce` can do, assuming you write all the shell code to actually do the
thing.

The advantage, lies in the fact that `sauce` is specifically tailored to
the usecases documented throughout. That means, it’s just a lot easier
to get the behaviors of `sauce` compared to writing the shell code (for
each project). Reproducing things like `sauce --as prod` or
`sauce --glob database*` would be significantly more involved.

Finally, the whole `direnv allow/deny` workflow exists specifically to
avoid the security concern originated due to executing shell code found
in some random folder (at least to my understanding). This is one of the
fatal flaws of storing the data directly within the folder being
operated on, and one of the reasons `sauce` stores its data centrally.
