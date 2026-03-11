# plg CLI implementation plan

## Goals

Build a Rust CLI named `plg` for navigating and analyzing Planguage markdown documents.

The CLI should:
- provide a clean, kubectl-style verb-oriented command surface,
- use ports-and-adapters architecture so parsing, storage, indexing, and prompt generation remain replaceable,
- treat local markdown as the source of truth,
- support both human-friendly and automation-friendly output modes,
- implement LLM-facing commands as prompt emitters first,
- ship with strong automated test coverage,
- and pass formatting, lint, and documentation checks.

## Design principles

- **Local-first**: operate directly on markdown files in the repository.
- **Read-first**: prioritize navigation, search, inspection, and analysis before any editing features.
- **Parse, do not grep, in the core**: shell tools may inspire workflows, but core behavior should use parsed document structure.
- **Human and machine friendly**: every read command should have readable defaults and structured output modes.
- **Tolerant reader**: parse imperfect documents without panic and surface diagnostics instead of failing hard.
- **Thin CLI, strong core**: argument parsing should stay shallow while application and domain layers hold behavior.
- **Prompt generation, not model execution**: `convert` and `qa` emit prompts only in v1.

## Recommended v1 scope

To avoid overbuilding the first release, define a hard v1 subset.

### Must ship in v1

- `plg get`
- `plg search`
- `plg tree`
- `plg lint`
- `plg fmt`
- `plg diff`
- `plg convert`
- `plg qa`
- `plg init`
- `plg new`
- `plg stats`
- `plg version`
- `plg completion`

### Nice to have after v1 core is green

- `plg index`
- `plg graph`
- `plg doctor`
- `plg config`
- `plg export`
- `plg import`

### Explicitly out of scope for initial implementation

- direct model or API calls,
- semantic document mutation commands,
- persisted search infrastructure beyond a simple local cache seam,
- remote repositories or SaaS backends,
- multi-user synchronization,
- background daemons,
- rich TUI experiences.

## Proposed command surface

### Core commands

- `plg get` — Display documents, tags, or selected fields in concise, detailed, or script-friendly formats.
- `plg search` — Search across Planguage markdown for tags, terms, sources, owners, risks, or fuzzy markers.
- `plg tree` — Show the hierarchy of documents, sections, and dotted Planguage tags as a tree.
- `plg lint` — Check Planguage files for structural, syntax, and style issues without rewriting them.
- `plg fmt` — Reformat Planguage markdown into a consistent canonical layout.
- `plg diff` — Compare two documents or versions by semantic fields instead of plain text only.
- `plg init` — Create a new Planguage workspace, repo layout, or starter document templates.
- `plg new` — Generate a new Planguage document or object from a template.
- `plg stats` — Report counts, coverage, defect density, fuzzy terms, and other repository metrics.

### Prompt-emitting commands

- `plg convert` — Emit a prompt for transforming prose or semi-structured specs into grounded Planguage syntax.
- `plg qa` — Emit a prompt for specification quality analysis using SQC and Planguage rules.

### Follow-on commands

- `plg index` — Build or refresh a local search index for fast document lookup.
- `plg graph` — Visualize links between tags, sources, owners, assumptions, and dependent specs.
- `plg doctor` — Diagnose workspace, config, parser, or indexing problems and suggest fixes.
- `plg config` — View or manage CLI configuration, defaults, and workspace settings.
- `plg completion` — Generate shell completion scripts for supported shells.
- `plg export` — Render Planguage content into JSON, CSV, HTML, or other downstream formats.
- `plg import` — Ingest markdown or structured data into the Planguage workspace model.
- `plg help` — Show help for commands, flags, concepts, and common task workflows.
- `plg version` — Print the CLI version, build info, and supported capability set.

## Architecture

### Architectural style

Use a ports-and-adapters layout with a small application core:

- **Domain layer**: Planguage concepts, validation rules, semantic IDs, metrics, and document model.
- **Application layer**: use cases for each command, query services, formatters, diagnostics, and prompt generation.
- **Ports**: traits for document repositories, parsers, index stores, prompt template providers, clocks, filesystem access, and output rendering.
- **Adapters**: markdown filesystem repository, parser implementation, search adapter, prompt-file adapter, terminal renderer, JSON renderer, and config loader.
- **CLI layer**: command parsing, flag validation, output mode selection, exit code mapping, and user-facing errors.

### Suggested crate layout

```text
src/
  main.rs
  cli/
    mod.rs
    args.rs
    commands/
      get.rs
      search.rs
      tree.rs
      convert.rs
      qa.rs
      lint.rs
      fmt.rs
      diff.rs
      init.rs
      new.rs
      stats.rs
      completion.rs
      version.rs
  application/
    mod.rs
    use_cases/
    services/
    dto/
  domain/
    mod.rs
    document.rs
    object.rs
    field.rs
    tag.rs
    diagnostics.rs
    rules.rs
    query.rs
  ports/
    mod.rs
    repository.rs
    parser.rs
    search.rs
    prompts.rs
    output.rs
    filesystem.rs
    clock.rs
  adapters/
    mod.rs
    fs_repository.rs
    markdown_parser.rs
    in_memory_search.rs
    prompt_files.rs
    terminal_output.rs
    json_output.rs
    config.rs
  testing/
    fixtures.rs
    builders.rs
```

Keep follow-on command modules behind later milestones until the v1 core is stable.

### Dependency guidance

Prefer small, established Rust crates only where justified by project needs. Likely candidates:

- `clap` for CLI parsing and shell completion,
- `serde` and `serde_json` for structured output,
- `thiserror` for typed errors,
- `ignore` for repository traversal with `.gitignore` support,
- `regex` only if needed and kept constrained,
- `assert_cmd` and `predicates` for CLI integration tests,
- `tempfile` for test workspaces,
- `similar` or a similarly small crate if semantic diff rendering needs support,
- `insta` only if snapshot tests become clearly valuable.

Avoid overcommitting to heavyweight parser frameworks until the document model stabilizes.

## Milestones and acceptance criteria

### Milestone A: project bootstrap

Acceptance criteria:
- `cargo run -- --help` works,
- `plg version` returns build information,
- repository discovery and config loading work in tests,
- the crate layout enforces domain/application/adapter separation.

### Milestone B: parse and query

Acceptance criteria:
- markdown fixtures parse into stable domain objects,
- `plg get`, `plg search`, `plg tree`, and `plg stats` work on fixture repositories,
- JSON output is stable enough for tests,
- malformed input produces diagnostics instead of panics.

### Milestone C: local quality tooling

Acceptance criteria:
- `plg lint` reports actionable diagnostics,
- `plg fmt --check` is reliable and `plg fmt` is idempotent,
- `plg diff` reports semantic changes at object and field level,
- exit codes are stable and documented.

### Milestone D: prompt generation

Acceptance criteria:
- `plg convert` and `plg qa` emit deterministic prompt text,
- prompt composition is covered by tests,
- no prompt-emitting command performs network access.

### Milestone E: release readiness

Acceptance criteria:
- `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test`, and `cargo doc --no-deps` all pass,
- help text includes concrete examples,
- README usage reflects the implemented command set,
- CI runs the same quality gates as local development.

## Implementation strategy

Use vertical slices, not horizontal scaffolding alone.

For each command shipped in v1:
1. define the CLI contract,
2. define the application use case,
3. add or extend domain types only as needed,
4. wire the filesystem/parser/output adapters,
5. write integration tests against a fixture repository,
6. then harden diagnostics and output formatting.

This prevents the architecture from becoming speculative and keeps ports grounded in real use cases.

## Command behavior plan

### Read/query commands

#### `plg get`

Purpose:
- primary read surface for documents and objects,
- can subsume old `describe` behavior via flags.

Expected capabilities:
- get documents by path, tag, owner, type, or source,
- support output presets such as table, wide, json, yaml-like plain text, and detailed,
- allow field projection like `--fields tag,type,owner,target`,
- support filters and sorting for scripting.

#### `plg search`

Purpose:
- full-text and structured search across markdown and parsed fields.

Expected capabilities:
- plain text search,
- tag-prefix search,
- field-aware search such as `owner:`, `source:`, `risk:`, `fuzzy:true`,
- regex mode only if clearly valuable,
- optional search against a refreshed index for speed.

#### `plg tree`

Purpose:
- visualize document and dotted-tag hierarchy.

Expected capabilities:
- show nested tags,
- collapse or expand by depth,
- optionally root at a specific document or tag.

#### `plg stats`

Purpose:
- summarize repository health and composition.

Expected capabilities:
- document counts,
- object counts by type,
- fields coverage,
- fuzzy term counts,
- source coverage,
- defect counts from lint,
- repository-level metrics suitable for CI.

#### `plg graph`

Purpose:
- expose relationships between objects.

Expected capabilities:
- output adjacency lists or DOT format first,
- optionally focus on source, owner, or tag lineage.

### LLM-facing commands

These should emit prompts only in the first implementation phase.

#### `plg convert`

Purpose:
- generate a prompt payload based on `prompts/planguage_conversion.md` plus user input.

Behavior:
- accept input from file, stdin, or direct text,
- print the composed prompt to stdout,
- optionally include metadata such as source file name or timestamp,
- avoid making network calls in the initial implementation.

#### `plg qa`

Purpose:
- generate a prompt payload based on `prompts/planguage_spec_quality_control.md` plus the target document content.

Behavior:
- accept one or more files,
- print a ready-to-send quality-audit prompt,
- optionally support prompt-only vs wrapped-document modes.

### Quality and formatting commands

#### `plg lint`

Purpose:
- provide local structural and semantic checks without external AI.

Checks to include:
- missing required fields such as `Tag`, `Type`, `Version`, `Status`, `Owner`,
- duplicate tags,
- malformed source markers,
- suspicious fuzzy terms not wrapped in angle brackets,
- invalid hierarchy formatting,
- commentary mixed with critical text when detectable.

#### `plg fmt`

Purpose:
- canonicalize field ordering, spacing, headings, indentation, and blank lines.

Rules:
- preserve meaning,
- avoid destructive rewriting of unknown content,
- support `--check` mode for CI.

#### `plg diff`

Purpose:
- compare semantic changes between two files or two revisions of one file.

Expected capabilities:
- field-level diff for objects with the same tag,
- added/removed object reporting,
- optional fallback to text diff when parsing fails.

### Workspace commands

#### `plg init`

Purpose:
- bootstrap a repo structure for Planguage work.

Suggested scaffold:
- `docs/planguage/`
- `docs/planguage/fragments/`
- `docs/planguage/templates/`
- `prompts/`
- example config file,
- example document with canonical fields.

#### `plg new`

Purpose:
- generate starter docs or objects.

Templates:
- requirement,
- performance goal,
- resource,
- function,
- process entry/task/exit block.

#### `plg index`

Purpose:
- build a fast reusable index for search and stats.

First phase:
- support in-memory rebuilds.

Later phase:
- optional persisted index under project-local cache.

#### `plg doctor`

Purpose:
- diagnose bad config, invalid prompt paths, parse failures, and index issues.

#### `plg config`

Purpose:
- inspect and update effective configuration.

#### `plg export`

Purpose:
- convert parsed Planguage content into machine-consumable formats.

#### `plg import`

Purpose:
- ingest compatible structured representations into markdown.

## Data model plan

Define a minimal domain model first.

### Core entities

- `Document`
  - path,
  - title,
  - raw markdown,
  - parsed objects,
  - diagnostics.
- `PlanguageObject`
  - tag,
  - type,
  - version,
  - status,
  - owner,
  - authority,
  - gist or ambition,
  - scale,
  - meter,
  - qualifiers,
  - source,
  - note/comment,
  - assumptions,
  - risks,
  - extra fields.
- `Diagnostic`
  - severity,
  - code,
  - message,
  - location,
  - suggested fix.

### Parsing strategy

Start with a tolerant line-oriented parser that:
- recognizes `Field: value` pairs,
- tracks heading context,
- supports repeated fields where valid,
- preserves unknown fields,
- records parse warnings rather than failing hard.

Avoid a strict grammar parser initially; correctness and recoverability matter more than perfect formalism at bootstrap.

## Output modes

Support consistent output contracts across commands.

- human table
- detailed text
- json
- json-lines where helpful
- dot for graph output

Design output rendering behind an output port so use cases remain presentation-agnostic.

## Error handling and exit codes

Define predictable exit codes.

- `0` success,
- `1` generic runtime or usage failure,
- `2` parse or validation failure,
- `3` lint or qa findings above threshold,
- `4` not found.

Translate domain and adapter errors into consistent user-facing messages.

## Configuration plan

Support layered config with clear precedence:
1. command flags,
2. environment variables,
3. workspace config,
4. user config.

Suggested config settings:
- workspace root,
- default docs paths,
- default output mode,
- prompt file locations,
- index cache location,
- formatting preferences,
- fuzzy-term dictionary overrides.

On Linux, prefer XDG-compatible user config placement.

## Testing strategy

Testing must be built in from the start.

### Unit tests

Cover:
- tag parsing,
- field parsing,
- dotted hierarchy behavior,
- diagnostics generation,
- formatter stability,
- prompt composition.

### Integration tests

Cover:
- end-to-end CLI invocations with fixture repositories,
- `get`, `search`, `tree`, `lint`, `fmt --check`, `diff`, and `init`,
- stdout/stderr separation,
- exit codes,
- config precedence.

### Snapshot tests

Use for:
- table output,
- tree output,
- semantic diffs,
- generated prompt text where stable.

### Property or fuzz tests

Consider for:
- parser resilience,
- formatter idempotence.

## Linting and quality gates

The implementation is not complete until it passes all quality gates.

Planned gates:
- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test`
- `cargo doc --no-deps`

If useful, add:
- `cargo deny` for dependency policy,
- `cargo audit` for known vulnerabilities.

## Implementation sequence

### Phase 1: bootstrap

- [x] Create a Rust binary crate with executable name `plg`.
- [x] Add core dependencies for CLI parsing, serialization, errors, traversal, and testing.
- [x] Establish the ports-and-adapters module layout.
- [x] Add a minimal config loader and workspace root discovery.
- [x] Add a shared error type and exit code mapper.
- [x] Add fixture-based integration test infrastructure.

### Phase 2: parsing and repository core

- [x] Implement filesystem document discovery for markdown files.
- [ ] Implement a tolerant markdown-to-object parser for Planguage field blocks.
- [ ] Model `Document`, `PlanguageObject`, and `Diagnostic` domain types.
- [ ] Preserve unknown fields and source locations during parse.
- [ ] Add unit tests for parsing edge cases and malformed input.

### Phase 3: first useful read commands

- [x] Implement `plg get` with path, tag, and field selection.
- [x] Implement `plg search` with text search and field-aware filters.
- [x] Implement `plg tree` for document and dotted-tag hierarchy views.
- [x] Implement `plg stats` for repository summary metrics.
- [x] Add integration and snapshot tests for read commands.

### Phase 4: local quality commands

- [ ] Implement `plg lint` with structural and semantic checks.
- [ ] Implement `plg fmt` and `plg fmt --check` with stable canonical output.
- [ ] Implement `plg diff` with semantic comparison and parse-fallback behavior.
- [ ] Add idempotence tests for formatter behavior.
- [ ] Add regression tests for diagnostics and exit codes.

### Phase 5: prompt-emitting LLM commands

- [x] Implement a prompt template adapter that loads prompt files from the workspace.
- [x] Implement `plg convert` to compose and print a conversion prompt from document or stdin input.
- [x] Implement `plg qa` to compose and print a quality-audit prompt from one or more files.
- [x] Add tests proving prompt composition is deterministic and complete.
- [x] Add clear help text stating that these commands emit prompts and do not call external models.

### Phase 6: workspace and ecosystem commands

- [x] Implement `plg init` to scaffold a repository layout and starter files.
- [x] Implement `plg new` for template-based object generation.
- [ ] Implement `plg index` with in-memory rebuild behavior and future persistence seam.
- [ ] Implement `plg graph` with DOT export as the first output target.
- [ ] Implement `plg doctor`, `plg config`, `plg export`, `plg import`, `plg completion`, and `plg version`.
- [x] Add integration tests for scaffolding, config, and export flows.

### Phase 7: polish and release readiness

- [x] Improve help text, examples, and error messages across all commands.
- [x] Verify stdout/stderr discipline and stable exit codes.
- [x] Run `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test`, and `cargo doc --no-deps` cleanly.
- [ ] Add CI configuration for formatting, linting, tests, and docs.
- [x] Write end-user CLI documentation and command examples.

## Design decisions to lock early

- [ ] Confirm whether `get` should support a `--output detailed` mode instead of keeping a separate `describe` command.
- [x] Confirm the initial workspace root and default document search paths.
- [ ] Confirm whether persisted indexing is in scope for v1 or deferred.
- [ ] Confirm whether `import` is needed in v1 or should be postponed until the markdown model stabilizes.
- [ ] Confirm whether graph output should be DOT-only in v1.

## Non-goals for v1

- direct LLM API integration,
- collaborative multi-user editing,
- a fully formal markdown grammar,
- remote document stores,
- background daemons or always-on indexing.

## Definition of done

The initial `plg` CLI is done when:
- the Rust binary builds cleanly,
- core commands for read, search, lint, format, prompt emission, and workspace bootstrap work on fixture repositories,
- architecture boundaries are enforced through ports and adapters,
- automated tests cover domain, application, adapters, and CLI behavior,
- and formatting, linting, tests, and docs all pass in CI.
