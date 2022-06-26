# Cosmwasm Linter
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](https://makeapullrequest.com)
[![first-timers-only](https://img.shields.io/badge/first--timers--only-friendly-blue.svg?style=flat-square)](https://www.firsttimersonly.com/)
[![Twitter handle][]][Twitter badge]

[Twitter handle]: https://img.shields.io/twitter/follow/0xFab1e.svg?style=social&label=Follow
[Twitter badge]: https://twitter.com/intent/follow?screen_name=0xFab1e


[Dylint](https://github.com/trailofbits/dylint) is a tool for running Rust lints from dynamic libraries. This repository is a "cosmwasm-focused" Dylint library you can use locally for one or all Cosmwasm contracts in your workspace! 

## Usage

First install [dylint](https://github.com/trailofbits/dylint), then add the following to your workspace's cargo.toml:

```toml
[workspace.metadata.dylint]
libraries = [
    { git = "https://github.com/0xFable/cw-lint" },
]
```

Now running `cargo dylint cw_lint --workspace` will download the linter and check your crates. Currently the following lints are used:

- `canon_addr`: provides a friendly warning about superfluous usage of CanonicalAddr in a contract.
  
More lints will be added in the future. If you can think of some pls submit a PR :D 

### Using cw-lint in Github Actions

Yes! You can automate the usage of cw-lint for when contributors submit new versions like so (assuming a github workflow yaml file): 

```yaml
# Install deps
- name: Install cargo-dylint
run: cargo install cargo-dylint

- name: Install dylint-link
run: cargo install dylint-link
# Run linter
- name: Run cosmwasm linter
run: cargo dylint cw_lint --workspace

```

---

**Experimental**

Choose a [Clippy lint](https://rust-lang.github.io/rust-clippy/master/) and run the following two commands:

```sh
./start_from_clippy_lint.sh CLIPPY_LINT_NAME NEW_LINT_NAME
cargo build
```

If the first command fails: sorry. Perhaps try another Clippy lint.

If the first command succeeds, but the second fails: you are probably halfway to having a functional Dylint library.

If both commands succeed: hooray! You might then try the following:

```sh
DYLINT_LIBRARY_PATH=$PWD/target/debug cargo dylint NEW_LINT_NAME -- --manifest-path=PATH_TO_OTHER_PACKAGES_MANIFEST
```

