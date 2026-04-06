# Repository Guidelines

## Project Structure & Module Organization
This repository is a documentation-first knowledge base. Topic areas live in top-level folders such as `backend/`, `frontend/`, `common/`, `networking/`, `testing/`, and `DSA/`. Most content is Markdown, organized by topic and level, for example `foundation/QnA.md`, `advance/QnA.md`, `TOPIC.md`, and `problems.md`.

The only code-heavy areas are Rust examples under `backend/rust/`, including `syntax/` and `hands-on-projects/guessing_game/`. Each Rust example is a standalone Cargo crate with its own `Cargo.toml` and `src/`.

## Build, Test, and Development Commands
- `cargo check --manifest-path backend/rust/syntax/Cargo.toml` validates the Rust syntax crate without producing a binary.
- `cargo test --manifest-path backend/rust/syntax/Cargo.toml` runs the crate tests, if present.
- `cargo run --manifest-path backend/rust/hands-on-projects/guessing_game/Cargo.toml` runs the sample Rust project.
- `cargo test --manifest-path backend/rust/hands-on-projects/guessing_game/Cargo.toml` checks the project's test suite.

There is no repository-wide build system; use the relevant crate path or edit Markdown directly.

## Coding Style & Naming Conventions
Keep Markdown concise, instructional, and consistently titled with topic-focused headings. Prefer lowercase directory names and existing file patterns such as `foundation/`, `advance/`, and `TOPIC.md`.

For Rust, follow standard `rustfmt` formatting, `snake_case` for functions and files, and `PascalCase` for types. Keep example code small and focused on a single concept.

## Testing Guidelines
This repo does not have a central automated test suite for the documentation content. For Rust crates, use `cargo test` and `cargo check` before submitting changes. For Markdown changes, manually verify heading structure, link targets, and code fence formatting.

## Commit & Pull Request Guidelines
Commit history uses short, imperative messages with optional prefixes, such as `docs: TDD v1`, `docs: rust v12`, and `test codex`. Keep commits focused on one topic or fix.

Pull requests should summarize the affected topic area, list the files changed, and note any validation performed. Include screenshots only when a visual or rendered output change matters.

## Agent Notes
Do not add generated build artifacts such as `target/`. Keep edits aligned with the existing folder taxonomy so content stays easy to navigate.