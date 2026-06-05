# Agent Guidelines

Rust workspace (`Cargo.toml` resolver 2) at the repo root; all crates live under `crates/`.

## Crates

- `crates/cli` — binary `auth-gateway-cli` (currently a stub `Hello, world!`).
- `crates/sdk` — library `auth-gateway-sdk`; depends on `auth-gateway-vectorscan`.
- `crates/vectorscan` — library `auth-gateway-vectorscan`; FFI bindings to the C++ VectorScan library. Has a `build.rs` that downloads + CMake-builds VectorScan at build time. Version pinned in `crates/vectorscan/.vectorscanrc.toml` (`[vendored] vectorscan_version`).

Dependency direction is strictly `cli` and `sdk` -> `vectorscan`. `cli` currently has no deps on the other crates.

## Build & target dir

- Cargo target dir is nested: `.build/target/` (set in `.cargo/config.toml`). Not the default `target/`. The `.gitignore` also ignores `.build/`.
- Toolchain is pinned to Rust `1.94.0` (`rust-toolchain.toml`, components `rustfmt` + `clippy`). `rustup` will fetch it automatically.
- `rustfmt.toml` uses **3-space indent, max width 120** (defaults are 4 / 100). Running stock `cargo fmt` from a fresh checkout without this file applied (or editing to 4-space indent) will fail `fmt:check`.

## Common commands (run from repo root)

- `npm run fmt` — `cargo fmt --all`
- `npm run fmt:check` — `cargo fmt --all --check`
- `npm run lint` — `cargo clippy --workspace --all-targets -- -D warnings` (warnings are denied)
- `npm run check` — `fmt:check` then `lint`; this is also what the Husky `pre-commit` hook runs (see `.husky/pre-commit`).
- `cargo build`, `cargo test` — work from the root. To target one crate: `cargo test -p auth-gateway-sdk` (or `-p auth-gateway-vectorscan`, `-p auth-gateway-cli`).
- `cargo build -p auth-gateway-vectorscan` first-time will: download the VectorScan tarball from GitHub (needs network), run cmake, link static `hs` + `hs_runtime`, plus `c++` on macOS / `stdc++` on Linux.

## First-time setup (host, outside dev container)

- Run `./.local/bin/prep-repo` to install system build deps: Homebrew/apt/dnf packages `boost cmake ragel pkg-config pcre libpcap sqlite` (+ `gcc` on macOS, `build-essential` on Debian/Ubuntu). Required for the VectorScan cmake build.
- Or use the dev container: `./.local/bin/build-dev` to build, `./.local/bin/start-dev` or `./.local/bin/enter-dev` to run/attach (auto-starts if not running), `./.local/bin/stop-dev` / `./.local/bin/destroy-dev` to tear down. These wrap `devcontainer` / `docker compose -f .devcontainer/docker-compose.yaml`.

## Repo-local scripts / env

- `.local/bin/` is added to `PATH` by `.envrc` (via direnv). `direnv allow` after cloning if direnv is installed.
- `node_modules/.bin` is also added to `PATH` by `.envrc`.

## Git restrictions

Never run, in any form:

- `git push` (including `--force`, specific remotes, etc.)
- `git checkout` (including `-b`, branch names, commit shas, paths)

These are blocked to avoid surprise changes to repo state. Branching/PR work is done out-of-band.
