# BashLens

BashLens is a native Rust desktop preview for Windows, macOS, and Linux. It lets you review clipboard text manually, applies local command-pattern checks, and explains what it can and cannot see.

## Run

Install Rust, then run from this repository:

```sh
cargo run --release
```

BashLens opens a native desktop window. It has no web frontend, account, server connection, command history, or command execution feature.

## Current scope

- Review text by pasting into the command box or choosing **Paste from clipboard**.
- Local pattern notes for downloads, script execution, deletion, permissions, elevated privileges, package installs, configuration changes, network activity, encoded text, and chained commands.
- The original text stays visible; BashLens does not rewrite it.
- **Copy command** copies the reviewed text for you to paste into a destination yourself.
- Minimal monochrome appearance and a vector logo at `assets/bashlens-mark.svg`.
- Automatic clipboard monitoring and terminal interception are not implemented. BashLens does not claim to protect pastes in other apps.

The analyzer is a set of visible heuristics, not a shell parser or safety verdict. It does not inspect local or remote scripts, run commands, or prove that a command is safe.

## Build notes

The same Rust codebase targets Windows, macOS, and Linux. Platform packaging, signing, login startup, global shortcuts, and per-terminal paste interception are follow-on work that must be built and checked separately on each operating system.
