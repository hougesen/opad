# opad

Easily manage package version across multiple package manager systems in mono repositories.

## Usage

<!-- START_SECTION:base-command-help -->

```
opad 0.0.1-dev
Easily manage package version across multiple package manager systems in mono repositories
Mads Hougesen <mads@mhouge.dk>

Usage: opad [OPTIONS]

Options:
      --check-hidden-files
          Check hidden files/foldersfor support package managers files

          Default: `false`

      --check-gitignored-files
          Check gitignored files/folders for support package managers files

          Default: `false`

      --completions <COMPLETIONS>
          Generate shell completions

          [possible values: bash, elvish, fish, nushell, powershell, zsh]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

<!-- END_SECTION:base-command-help -->

### Updating package versions

Start by selecting the packages you wish to update:

```
$ opad

? Which files do you wish to update?
  [x] Cargo.toml
  [ ] cli/Cargo.toml
> [ ] docs/package.json
[↑↓ to move, space to select one, → to all, ← to none, type to filter]
```

Input the new package version:

```
> Which files do you wish to update? Cargo.toml
? Cargo.toml: What do you wish to set the version to? 1.2.3
```

Choose whether to update the associated lock files:

```
> Which files do you wish to update? Cargo.toml
> Cargo.toml: What do you wish to set the version to? 1.2.3
🟩 Cargo.toml has been updated
> Do you wish to update the lock files (experimental) Yes
🟦 Updating lock files connected to Cargo.toml
    Checking opad v1.2.3 (/home/houge/Desktop/projects/opad/cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
🟩 Lock files has been updated
```

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
| `Elm`                      | `elm`           | `elm.json`       | n/a (elm does not have a lock file (?))                   |

## Disclaimer

Not all file parsers used by this tool respect prior formatting, so you might want to run your preferred formatting tool on the files after usage.
