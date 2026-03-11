# Command Line Interface Guidelines (CLIG)

Source: <https://clig.dev/>

This document captures the main ideas from the Command Line Interface Guidelines and distills them into a practical reference for building better CLI tools.

## What CLIG is

CLIG is a guide for designing command-line programs that are both practical and pleasant to use. It combines classic Unix ideas with modern expectations for usability, discoverability, robustness, and empathy.

## Core philosophy

- **Design for humans first.** CLI tools are interfaces, not just machine endpoints.
- **Build simple parts that work together.** Prefer composable tools that cooperate through standard streams, pipes, and exit codes.
- **Be consistent.** Follow conventions users already know.
- **Say just enough.** Output should be informative without being noisy.
- **Make discovery easy.** Users should be able to learn the tool from help text, examples, and good defaults.
- **Treat the CLI like a conversation.** Good CLIs guide users, suggest fixes, and help them recover.
- **Be robust.** Tools should feel dependable, especially around errors and interruptions.
- **Design with empathy.** The experience should reduce frustration and increase confidence.

## Main guidelines

### Basics

- Use a real argument parser instead of hand-rolling parsing logic.
- Exit with `0` on success and non-zero on failure.
- Write primary output to `stdout` and diagnostics to `stderr`.

### Help and discoverability

- Support `-h` and `--help`.
- Keep default output concise; make help more detailed.
- Lead help text with examples.
- Show the most common commands and flags first.
- Offer typo suggestions for mistyped commands or flags.
- Avoid silently correcting user input when intent is uncertain.

### Documentation

- Provide web documentation for searchability.
- Provide terminal-friendly docs where appropriate, such as man pages.
- Make it easy for users to report issues or ask for help.

### Output

- Optimize default output for humans.
- Offer structured output such as `--json` for scripting.
- Use plain, readable formatting.
- Show progress for long-running operations.
- Use color carefully and disable it automatically when output is not a TTY.
- Respect common environment conventions like `NO_COLOR` and `TERM=dumb`.
- Consider paging long output.

### Errors

- Rewrite low-level errors into messages users can act on.
- Put the most important information first.
- Group related errors when helpful.
- Offer debugging details when the root cause is unclear.
- Make bug reports easier by including useful diagnostic context.

### Arguments and flags

- Prefer descriptive long flags such as `--verbose`.
- Reuse familiar short flags when they match common expectations.
- Prefer flags over positional arguments when clarity matters.
- Accept `-` to mean stdin or stdout where applicable.
- Prompt for missing required input in interactive mode.
- Confirm dangerous actions.
- Avoid putting secrets in command arguments; prefer stdin, files, or prompts.

### Interactivity

- Only prompt when stdin is a TTY.
- Provide a way to disable prompts, such as `--no-input`.
- Hide secret input such as passwords.
- Let users cancel cleanly with standard controls like `Ctrl-C`.

### Subcommands

- Use a clear, consistent command structure.
- Prefer names that are easy to predict and hard to confuse.
- Avoid overlapping or ambiguous subcommands.

### Robustness

- Validate input early.
- Be responsive; avoid long unexplained pauses.
- Show status or progress during long tasks.
- Use sensible timeouts where needed.
- Recover gracefully from partial failure when possible.

### Future-proofing

- Prefer additive changes over breaking changes.
- Warn users before incompatible interface changes.
- Do not encourage scripts to depend on human-oriented output.
- Reserve room for future commands and flags.

### Signals and interrupts

- Handle `Ctrl-C` promptly.
- If cleanup may take time, communicate that clearly.
- Allow a stronger second interrupt when appropriate.

### Configuration

- Use:
  - **flags** for per-invocation control,
  - **environment variables** for contextual settings,
  - **config files** for persistent project or user preferences.
- Follow standard config locations such as XDG on Linux.
- Use clear precedence rules, typically:
  - flags,
  - environment variables,
  - project config,
  - user config,
  - system config.

### Environment variables

- Use them for context-dependent settings.
- Prefer uppercase names.
- Respect established variables where possible.
- Avoid using environment variables for complex structured configuration if a config file is more appropriate.

### Naming and distribution

- Choose names that are short, memorable, and easy to type.
- Prefer lowercase names.
- Distribute as a single easy-to-install artifact when feasible.

### Analytics

- If collecting telemetry, make it opt-in.
- Be transparent about what is collected and why.
- Consider alternatives such as documentation analytics or direct user feedback.

## Practical takeaways

A strong CLI should:

- feel predictable,
- teach itself through help and examples,
- work well in both interactive and scripted contexts,
- fail clearly and recover gracefully,
- respect terminal conventions,
- and minimize user surprise.

## Good defaults to apply

- Add `--help` and examples to every command.
- Separate machine output from human output.
- Support `--json` for automation when structured output matters.
- Respect `stdout`/`stderr`, exit codes, and TTY detection.
- Confirm destructive actions.
- Avoid leaking secrets via command history.
- Keep command names and flag behavior consistent across the tool.

## Reference

- Command Line Interface Guidelines: <https://clig.dev/>
