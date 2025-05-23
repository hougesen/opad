# crosspmv

Easily manage package version across multiple package manager systems in mono repositories.

## Supported package management systems

| Package manager           | Update lock file command                                  |
| ------------------------- | --------------------------------------------------------- |
| cargo (`Cargo.toml`)      | `cargo check` (`Cargo.lock`)                              |
| npm (`package.json`)      | `npm install` (`package-lock.json`)                       |
| pnpm (`package.json`)     | `pnpm install` (`pnpm-lock.yam`)                          |
| yarn (`package.json`)     | `yarn install` (`yarn.lock`)                              |
| bun (`package.json`)      | `bun install` (`bun.lockb`)                               |
| deno (`deno.json`)        | `deno install` (`deno.lock`)                              |
| uv (`pyproject.toml`)     | `uv lock` (`uv.lock`)                                     |
| rye (`pyproject.toml`)    | `rye lock` (`requirements.lock`, `requirements-dev.lock`) |
| poetry (`pyproject.toml`) | n/a (`poetry.lock` does not include version)              |
| gleam (`gleam.toml`)      | n/a (`manifest.toml` does not include version)            |

## Disclaimer

Not all file parsers used by this tool respect prior formatting, so you might want to run your preferred formatting tool on the files after usage.
