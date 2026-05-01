<div align="center">

# sqlitx

[![license](https://badgen.net/github/license/sudosubin/sqlitx)](LICENSE)
[![release](https://badgen.net/github/release/sudosubin/sqlitx)](https://github.com/sudosubin/sqlitx/releases)
[![built with rust](https://badgen.net/badge/built%20with/Rust/orange)](https://www.rust-lang.org)

`fzf`-powered connection picker for [sqlit](https://github.com/Maxteabag/sqlit).

<a href="docs/sqlitx.webp">
  <img src="docs/sqlitx.webp" alt="sqlitx demo" width="800" />
</a>

</div>

## Installation

Requires `fzf` and `sqlit` on `PATH`.

```sh
cargo install sqlitx
```

## Usage

Run `sqlitx` to open an interactive picker over all saved connections. Selecting one runs `sqlit -c "<name>"` in the current shell.

## Design

Connection config is resolved in order:

1. `$SQLIT_CONFIG_DIR/connections.json`
2. `$XDG_CONFIG_HOME/sqlit/connections.json`
3. `~/.config/sqlit/connections.json`

sqlitx reads sqlit's connection store directly without spawning sqlit just to list connections. The actual database connection is delegated entirely to the `sqlit` binary. Built in Rust, startup is under 10ms.

## License

[MIT](LICENSE)
