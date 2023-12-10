# mastodon-automations-rust

Currently only unfollows accounts that don't follow you. Other features hopefully soon.

## Prerequisites

You'll need [Rust installed](https://www.rust-lang.org/tools/install) on your system.


## Usage

Clone the repo. Then run it with `--feature toml`

```bash
cargo run --features toml
```

It should ask for your instance eg. `https://phocks.eu.org` and then send you to a URL to authenticate.

Good luck!
