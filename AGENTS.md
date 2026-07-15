# AGENTS.md — AI Agent Guidelines for This Project

## Project Architecture

The application consists of three layers:

| Layer | Module | Purpose |
|-------|--------|---------|
| **CLI** | `cmd::cli::Cli` | Command-line argument parsing (clap) |
| **Controller** | `controllers::index::IndexController` | Orchestration: fetching data, rendering, output |
| **Util** | `util::universal_source::UniversalSource` | Loading Markdown from file or URL |
| **Util** | `util::handler::Handler` | Common trait for integrating the controller with the CLI |

Execution flow: CLI → Controller → UniversalSource (reading) → marcli (rendering) → minus (pager).

For a detailed description, see `README.md`.

## Code Writing Guidelines

### Prohibited

- `unwrap()` — strictly forbidden. Any call to `unwrap()` must be replaced with proper error handling via `anyhow`, `?`, or match.
- `panic!()` — only allowed in extreme cases and must be explicitly agreed upon.
- `todo!()`, `unreachable!()`, `unimplemented!()` — prohibited in production code.
- Index-based access to collection elements (`slice[i]`, `vec[i]`) — may cause a panic. Use `.get(i)` with `None` handling instead.
- Arithmetic operations that may overflow — use `checked_*`, `wrapping_*`, or `saturating_*` methods.

### Allowed After Approval

- `Option::expect(msg)` and `Result::expect(msg)` — may only be used if:
  1. The agent explicitly asks for permission in a message, explaining why `expect` is safe in that context (e.g., "the field is known to always be set, the code path won't execute without this value").
  2. The agent waits for a response before using it.

### Recommended

- Handle errors via `anyhow::Result` and the `?` operator.
- Use `match` or `if let` for safely unwrapping `Option` and `Result`.
- For CLI validation, use `clap`'s built-in capabilities (checks in `#[clap(...)]`).
- Write meaningful messages in `anyhow::bail!()` and use contextual `.context()` / `.with_context()` from `anyhow`.

## README.md

Do not include the project's directory tree in `README.md` — it is redundant and quickly becomes outdated. Instead, describe the architecture, key components, and data flow.