# AGENTS.md

## Project Overview

This repository is a Rust code project for `plg`, a command-line tool for navigating and analyzing Planguage markdown documents.

Planguage (Planning Language) is a structured notation for writing measurable, testable requirements and specifications. The tool is intended to help developers and coding agents inspect, search, lint, format, diff, and generate LLM-ready prompts for Planguage content stored in local markdown files.

### Current state

- The project has been bootstrapped as a Rust binary crate.
- The executable name is `plg`.
- The current implemented CLI surface is minimal and includes a `version` subcommand plus standard Clap-generated help.
- The codebase is being organized around a **ports-and-adapters** architecture.
- A detailed implementation roadmap lives in `docs/plan/cli-init.md`.

### Main technologies

- **Rust** (edition `2024`)
- **clap** for CLI parsing
- **thiserror** for typed error handling
- **serde** / **serde_json** for structured output
- **assert_cmd** and **predicates** for CLI integration tests

## Repository Layout

- `Cargo.toml` — Rust package manifest and dependency definitions.
- `src/main.rs` — Binary entry point; maps CLI execution failures to exit codes and stderr.
- `src/cli/` — CLI parsing and command dispatch.
- `src/application/` — Application layer and shared app state.
- `src/domain/` — Domain-level types such as build metadata and, eventually, Planguage models.
- `src/ports/` — Interface boundaries for repositories, parsers, search, prompts, and output.
- `src/adapters/` — Concrete implementations of ports.
- `tests/` — Integration tests for the CLI.
- `prompts/` — Prompt templates for LLM-facing commands.
- `docs/plan/cli-init.md` — Primary implementation plan for the CLI.
- `README.md` — High-level project context and Planguage background.
- `cmd.txt` — Proposed command inventory and one-line descriptions.

## Architecture Notes

Follow the planned **ports-and-adapters** structure:

- **CLI layer**: parse command-line input and render user-facing output.
- **Application layer**: orchestrate use cases and command behavior.
- **Domain layer**: define stable business concepts and validation rules.
- **Ports**: traits/interfaces for parsing, repository access, searching, prompt loading, and rendering.
- **Adapters**: filesystem, parser, output, and other concrete implementations.

### Important architectural guidance

- Keep the CLI layer thin.
- Put behavior in application/domain code, not directly in argument-parsing modules.
- Prefer parsed document structure over shell-driven grep logic in core features.
- Treat local markdown files as the source of truth.
- For v1, **LLM-facing commands should emit prompts only**; they should not call remote APIs or models.
- The current plan is **read-first**: prioritize navigation, inspection, search, linting, formatting, diffing, stats, and prompt generation over document mutation.

## Build, Run, and Test

Use Cargo from the repository root.

### Build

```bash
cargo build
```

### Run the CLI

```bash
cargo run -- --help
cargo run -- version
```

### Test

```bash
cargo test
```

### Lint

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Format

```bash
cargo fmt
cargo fmt --check
```

### Docs

```bash
cargo doc --no-deps
```

## Development Conventions

### Code organization

- Prefer small, composable modules.
- Add new command behavior under `src/cli/commands/`, but keep nontrivial logic in `application/` and `domain/`.
- Add new ports before introducing infrastructure-heavy adapters.
- Keep future modules aligned with the implementation plan in `docs/plan/cli-init.md`.

### Error handling

- Use typed application errors via `thiserror`.
- Surface concise CLI-facing errors on stderr.
- Keep exit behavior predictable and testable.

### Testing practices

- Add integration tests in `tests/` for user-visible CLI behavior.
- Prefer end-to-end CLI assertions for commands, output, and exit codes.
- Add unit tests for domain parsing/validation logic once those modules exist.
- Maintain green checks for:
  - `cargo fmt --check`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo test`
  - `cargo doc --no-deps`

### Output behavior

- Human-readable output should be the default.
- Structured output should be added behind explicit flags or output modes.
- Keep stdout for normal command output and stderr for diagnostics/errors.

## Key Files to Read First

If you are starting work in this repository, read these first:

1. `README.md` — project purpose and Planguage context.
2. `docs/plan/cli-init.md` — authoritative implementation plan and scope.
3. `Cargo.toml` — crate metadata and dependencies.
4. `src/main.rs` — entrypoint and error-to-exit mapping.
5. `src/cli/mod.rs` and `src/cli/args.rs` — current CLI wiring.
6. `tests/version_cli.rs` — current testing style for CLI behavior.
7. `prompts/planguage_conversion.md` and `prompts/planguage_spec_quality_control.md` — source material for prompt-emitting commands.

## Planned Command Direction

The current plan centers on these command families:

- **Core read/query**: `get`, `search`, `tree`, `stats`
- **Quality**: `lint`, `fmt`, `diff`
- **Prompt emission**: `convert`, `qa`
- **Workspace/bootstrap**: `init`, `new`, `completion`, `version`
- **Follow-on**: `index`, `graph`, `doctor`, `config`, `export`, `import`

Do not assume all planned commands are implemented. Check `src/cli/args.rs` and tests before extending behavior.

## Practical Guidance for Future Agents

- Start by checking the implementation status against `docs/plan/cli-init.md`.
- Preserve the read-first scope unless explicitly asked to expand it.
- Avoid introducing mutation/editing commands unless the plan is intentionally revised.
- Prefer incremental vertical slices: CLI contract -> application use case -> domain types -> adapter wiring -> tests.
- When adding dependencies, keep them small and well-justified.
- Match existing Rust style and keep code straightforward and testable.
