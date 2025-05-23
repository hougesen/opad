# crosspmv

Easily manage package version across multiple package manager systems in mono repositories.

## Supported package management systems

| Language                   | Package manager | File             | Update lock file command                                  |
| -------------------------- | --------------- | ---------------- | --------------------------------------------------------- |
| `Rust`                     | `cargo`         | `Cargo.toml`     | `cargo check` (`Cargo.lock`)                              |
| `JavaScript`, `TypeScript` | `npm`           | `package.json`   | `npm install` (`package-lock.json`)                       |
| `JavaScript`, `TypeScript` | `pnpm`          | `package.json`   | `pnpm install` (`pnpm-lock.yam`)                          |
| `JavaScript`, `TypeScript` | `bun`           | `package.json`   | `bun install` (`bun.lock`, `bun.lockb`)                   |
| `JavaScript`, `TypeScript` | `yarn`          | `package.json`   | n/a (`yarn.lock` does not include version)                |
| `JavaScript`, `TypeScript` | `deno`          | `deno.json`      | `deno install` (`deno.lock`,)                             |
| `Python`                   | `uv`            | `pyproject.toml` | `uv lock` (`uv.lock`)                                     |
| `Python`                   | `rye`           | `pyproject.toml` | `rye lock` (`requirements.lock`, `requirements-dev.lock`) |
| `Python`                   | `poetry`        | `pyproject.toml` | n/a (`poetry.lock` does not include version)              |
| `Gleam`                    | `gleam`         | `gleam.toml`     | n/a (`manifest.toml` does not include version)            |
| `Dart`                     | `pub`           | `gleam.toml`     | n/a (`pubspec.lock` does not include version)             |
| `Crystal`                  | `shards`        | `shard.yml`      | n/a (`shard.lock` does not include version)               |

## Disclaimer

Not all file parsers used by this tool respect prior formatting, so you might want to run your preferred formatting tool on the files after usage.
