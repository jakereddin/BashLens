# BashLens

BashLens is a native Rust desktop preview for Windows, macOS, and Linux. It lets you review clipboard text manually, applies a small set of local command-pattern checks, and explains the limits of what it can see.

## Run

Install the Rust toolchain, then from this folder:

```sh
cargo run --release
```

The app uses a native desktop window. It has no web frontend, account, server connection, command history, or command execution feature.

## Current scope

- Manual review by pasting into the command box or using **Paste from clipboard**.
- Every non-empty input gets a structural breakdown: command names, arguments, common shell connectors, redirections, environment assignments, and shell expansions.
- Built-in descriptions cover common Windows, macOS, Linux, and PowerShell tools. Additional local pattern checks highlight downloads, script execution, deletion, permissions, elevated privileges, package installs, configuration changes, network activity, encoded text, and chained commands.
- Original text remains visible; the app does not rewrite it.
- **Copy command** copies the reviewed text. Paste it into a destination yourself after reviewing.
- Minimal light monochrome appearance.
- Monitoring and terminal interception are not implemented. No automatic paste protection is claimed.

The analyzer never executes commands. Its structural breakdown is a best-effort text parser, not a complete implementation of Bash, PowerShell, or `cmd.exe`. Built-in descriptions are not exhaustive. For an unknown or custom program, BashLens still shows its name, arguments, and place in a pipeline, but only that program's documentation can explain its internal behavior. The findings are not a security verdict and do not prove that anything is safe.

## Build notes

The same Rust codebase targets Windows, macOS, and Linux. Platform packaging, signing, login startup, global shortcuts, and per-terminal paste interception are follow-on work and must be built and verified separately on each operating system.

## Logo

The monochrome vector mark is in `assets/bashlens-mark.svg`.
