# Repository Guidelines

## Project Structure & Module Organization
This repository is organized as a collection of topic-focused modules and learning tracks. Key areas include:
- `backend/` (language-specific backend material, including Rust and Node.js)
- `frontend/` (frontend notes and exercises)
- `DSA/`, `oop/`, `os/`, `networking/`, `devops-devsecops/`, `microservices/`, `common/`, `others-topic/` (topic folders)

Rust examples live under `backend/rust/`, with standalone crates such as `backend/rust/syntax/` and `backend/rust/hands-on-projects/guessing_game/`.

## Build, Test, and Development Commands
Only the Rust crates are currently buildable with standard tooling:
- `cargo run` (from a crate directory like `backend/rust/syntax/`): build and run the example.
- `cargo test` (from a crate directory): run crate tests, if present.

If you add new runnable modules, document their commands in the relevant folder’s README.

## Coding Style & Naming Conventions
Follow language-standard conventions within each module:
- Rust: use `rustfmt` defaults and `snake_case` for functions/variables.
- General files: keep names descriptive and topic-oriented (e.g., `backend/rust/...`, `frontend/...`).

If you introduce linting or formatting tools, add a short note in this document or the module README.

## Testing Guidelines
No global testing framework is configured. For Rust crates, prefer standard `cargo test` with test names that describe behavior (e.g., `parses_valid_input`).

## Commit & Pull Request Guidelines
Recent commit history suggests short, prefix-based messages such as `docs: ...` for documentation updates. Keep commit subjects concise and scoped.

Pull requests should include:
- A brief summary of changes
- References to any related issues or topics
- Screenshots or output snippets when adding user-facing content

## Security & Configuration Tips
Do not commit secrets. If you add configuration files, use environment-based settings and document required variables in the relevant module README.
