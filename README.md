<div align="center">

# sqlitx

[![version](https://badgen.net/github/release/sudosubin/sqlitx?label=version)](https://github.com/sudosubin/sqlitx/releases)
[![license](https://badgen.net/github/license/sudosubin/sqlitx?color=green)](LICENSE)

`fzf`-powered connection picker for [sqlit](https://github.com/Maxteabag/sqlit).

<a href="docs/assets/sqlitx-demo.webp">
  <img src="docs/assets/sqlitx-demo.webp" alt="sqlitx demo" width="800" />
</a>

</div>

## Quick Start

```sh
cargo install sqlitx
sqlitx
```

## Installation

Requires `fzf` and `sqlit` on `PATH`.

```sh
cargo install sqlitx
```

## Usage

Run `sqlitx` to open an interactive picker over all saved connections. Selecting one runs `sqlit -c "<name>"` in the current shell.

## How It Works

Connection config is resolved in order:

1. `$SQLIT_CONFIG_DIR/connections.json`
2. `$XDG_CONFIG_HOME/sqlit/connections.json`
3. `~/.config/sqlit/connections.json`

sqlitx reads sqlit's connection store directly without spawning sqlit just to list connections. The actual database connection is delegated entirely to the `sqlit` binary. Built in Rust, startup is under 10ms.

## Development

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo build --all-features
```

## License

MIT, see [LICENSE](./LICENSE).
