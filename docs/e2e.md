# plg end-to-end flow

This document defines a manual end-to-end flow for the currently implemented `plg` commands.

## Goals

- exercise every currently implemented command,
- run everything inside a gitignored workspace,
- verify expected outputs at a high level,
- leave the repository clean except for intentional source and docs changes.

## Safety and cleanup

All temporary artifacts for this flow live under:

- `.e2e/`

That directory is gitignored and may be deleted at the end of the run.

Cleanup command:

```bash
rm -rf .e2e
```

## Commands covered

Current implemented commands:

- `plg version`
- `plg init`
- `plg new`
- `plg get`
- `plg search`
- `plg tree`
- `plg stats`
- `plg convert`
- `plg qa`

## Test flow

### 1. Build the CLI

```bash
cargo build
```

Expectation:
- build succeeds.

### 2. Reset the e2e workspace

```bash
rm -rf .e2e
mkdir -p .e2e
```

Expectation:
- `.e2e/` exists and is empty.

### 3. Check version output

```bash
cargo run -- version
```

Expectation:
- prints `plg <version>`.

### 4. Initialize a workspace

```bash
cargo run -- init --dir .e2e/workspace
```

Expectation:
- creates:
  - `.e2e/workspace/docs/planguage/templates/`
  - `.e2e/workspace/docs/planguage/fragments/`
  - `.e2e/workspace/docs/planguage/example.md`
  - `.e2e/workspace/plg.toml`
- prints an `initialized ...` message.

### 5. Create additional starter documents

Run these from inside the initialized workspace so `plg` discovers `.e2e/workspace` as the active root:

```bash
( cd .e2e/workspace && ../../target/debug/plg new docs/planguage/req.md )
( cd .e2e/workspace && ../../target/debug/plg new --template performance docs/planguage/perf.md )
```

Expectation:
- both files are created,
- requirement doc contains `Tag: Example.Requirement`,
- performance doc contains `Type: Performance` and `Ambition:`.

### 6. List markdown documents

```bash
( cd .e2e/workspace && ../../target/debug/plg get --path-prefix docs/ )
```

Expectation:
- lists:
  - `docs/planguage/example.md`
  - `docs/planguage/req.md`
  - `docs/planguage/perf.md`
- does not list non-markdown files.

### 7. Search for a shared field

```bash
( cd .e2e/workspace && ../../target/debug/plg search "Owner: Team" --path-prefix docs/ )
```

Expectation:
- returns the starter markdown files under `docs/`.

### 8. Show the tree

```bash
( cd .e2e/workspace && ../../target/debug/plg tree --path-prefix docs/ )
```

Expectation:
- prints a tree containing `docs`, `planguage`, and the created markdown files.

### 9. Show stats

```bash
( cd .e2e/workspace && ../../target/debug/plg stats --path-prefix docs/ )
```

Expectation:
- reports at least:
  - `markdown_files`
  - `directories_with_markdown`
- current expected markdown file count is `3`.

### 10. Emit a conversion prompt from inline text

```bash
cargo run -- convert --text "Users need faster feedback from the reporting workflow."
```

Expectation:
- output begins with the conversion prompt template,
- output includes the inline input text after an `Input:` section.

### 11. Emit a QA prompt from a file

```bash
( cd .e2e/workspace && ../../target/debug/plg qa --file docs/planguage/req.md )
```

Expectation:
- output begins with the QA prompt template,
- output includes the contents of `req.md` after an `Input:` section.

### 12. Cleanup

```bash
rm -rf .e2e
```

Expectation:
- temporary workspace is removed.

## Pass criteria

The flow passes if:

- every command exits successfully,
- outputs match the expectations above,
- `.e2e/` can be removed cleanly,
- and `git status --short` shows only intentional tracked-file changes.
