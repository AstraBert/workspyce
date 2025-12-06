# workspyce

A minimal, no-nonsense version manager for [uv](https://docs.astral.sh/uv/) workspaces, inspired by [changesets](https://github.com/changesets/changesets).

> _Disclaimer: I am doing this to learn Rust! This might not be the most efficient way to manage versions in your uv workspace (at least no yet ;)_

## Usage

Clone this repo:

```bash
git clone https://github.com/AstraBert/workspyce
cd workspyce
```

Build the project:

```bash
cargo build
```

Use the resulting build as an entrypoint:

```bash
./target/debug/workspyce --help
```

Use it within a python project:

```bash
../projects/workspyce/target/debug/workspyce --pyproject pyproject.toml
```

The program should find the members within your `uv` workspace (should also compile with a regular expression for the `*` wildcard), check the current status of your git repository and ask for what kind of version bump you want to perform for a specific package, saving the info to a markdown file in the `.workspyce/` folder.

> _Please note that all of this is still **work in progress** and it is not complete yet_