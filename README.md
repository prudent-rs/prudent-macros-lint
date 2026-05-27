# See prudent instead

`prudent-macros-lint` is internal to `prudent-rs`. Don't use directly/on its own. Instead, see and
use [`crates.io/crates/prudent`](https://crates.io/crates/prudent). See its code at
[`prudent-rs/prudent`](https://github.com/prudent-rs/prudent).

## No version check of prudent

Unlike `prudent-macros-enforce`, `prudent-macros-lint` does NOT check that it's of the same version
as `prudent` that it is used from. Doing so would complicate it, and it's a debug-only lint anyway.
