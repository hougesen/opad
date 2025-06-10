# opad

Easily manage package version across multiple package manager systems in mono repositories.

## Installation

### Using Cargo

opad can be installed using Cargo, the package manager for Rust ([crates.io](https://crates.io/crates/opad)).

```shell
cargo install opad
```

### Using Homebrew

If you're on macOS or Linux, you can install opad using Homebrew:

```shell
# Tap and install
brew tap hougesen/tap
brew install opad

# Or install directly in one command
brew install hougesen/tap/opad
```

### Using npm/npx

You can install `opad` using [npm](https://www.npmjs.com/package/opad):

```shell
npm install -g opad

opad format .
```

Or run it directly using npx:

```shell
npx opad format .
```

### Precompiled Binaries

If you do not have/want Rust or Homebrew installed on your device, you can find precompiled binaries on the [release](https://github.com/hougesen/opad/releases) page or run one of the installers below.

#### Linux & MacOS

```shell
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/hougesen/opad/releases/latest/download/opad-installer.sh | sh
```

#### Windows

```powershell
powershell -ExecutionPolicy ByPass -c "irm https://github.com/hougesen/opad/releases/latest/download/opad-installer.ps1 | iex"
```

## Usage

<!-- START_SECTION:base-command-help -->

```
opad 0.1.0
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
| `JavaScript`, `TypeScript` | `yarn`          | `package.json`   | `yarn install` (`yarn.lock`)                              |
| `JavaScript`, `TypeScript` | `deno`          | `deno.json`      | `deno install` (`deno.lock`,)                             |
| `JavaScript`, `TypeScript` | `lerna`         | `lerna.json`     | Depends on the `npmClient` field                          |
| `Python`                   | `uv`            | `pyproject.toml` | `uv lock` (`uv.lock`)                                     |
| `Python`                   | `rye`           | `pyproject.toml` | `rye lock` (`requirements.lock`, `requirements-dev.lock`) |
| `Python`                   | `poetry`        | `pyproject.toml` | n/a (`poetry.lock` does not include version)              |
| `Gleam`                    | `gleam`         | `gleam.toml`     | n/a (`manifest.toml` does not include version)            |
| `Dart`                     | `pub`           | `pubspec.yaml`   | n/a (`pubspec.lock` does not include version)             |
| `Crystal`                  | `shards`        | `shard.yml`      | n/a (`shard.lock` does not include version)               |
| `Elm`                      | `elm`           | `elm.json`       | n/a (elm does not have a lock file (?))                   |

## Shell completion

Shell completion can be generated using the `opad --completions $SHELL` command.

### Bash

Add the following to your `.bashrc`.

```bash
eval "$(opad --completions bash)"
```

### Zsh

Add the following to your `.zshrc`:

```zsh
eval "$(opad --completions zsh)"
```

### Fish

Add the following to `~/.config/fish/config.fish`.

```fish
opad --completions fish | source
```

### PowerShell

Add the following to your PowerShell configuration (Can be found by running `$PROFILE`).

```powershell
Invoke-Expression (&opad --completions powershell)
```

### Elvish

Add the following to `~/.elvish/rc.elv`.

```elvish
eval (opad --completions elvish)
```

### Nushell

Generate completions for [nushell](https://github.com/nushell/nushell).

```nushell
opad --completions nushell
```

## Disclaimer

I wrote `opad` since I got tired of manually updating package version in my mono repositories that included multiple different languages and build tools. It might be useful for you, or it might not be. Feel free to let me know if something isn't working as you would expect, or if a feature is missing.

Please be aware that not all file parsers used by this tool respect prior formatting, so you might want to run your preferred formatting tool on the files after usage.
