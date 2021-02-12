## Planned Work

- `sauce config color=false` color setting, to be composed on top of the
  existing color choosing options

- Ensure files are only being loaded at the time at which they’re
  necessary.

- `sauce move . ../newpath`

  Moves the saucefile from one location to another, to be used when the
  actual directory being sauced has moved locations.

  - The paths should be the intended paths of the folder, not the
    data-dir path.
  - Defaults to only moving the saucefile
  - Option to move the directory as well
  - Option to leave the source saucefile behind

- ability to subdivide targets by shell

  i.e. allow one to specify `[alias.fish]`.

- `sauce config subshell=false/true` (default `false`)

  Given subshell=true, a call to `sauce` would create a subprocess into
  which the alterations would be made. This would enable one to simply
  kill the current shell to revert state.

- more targets: arbitrary key-value pairs

- pipe `sauce show` to a pager when beyond a full terminal height
