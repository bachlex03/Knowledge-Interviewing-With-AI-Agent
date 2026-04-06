# Knowledge Interviewing with AI Agent

> **Maestro AI Development Orchestrator**
> This repository is a documentation-first knowledge base for simulating technical interviews using AI agents.

---

## 📋 Project Overview

The project is designed to create a structured environment for technical interview practice. It leverages custom AI agents, skills, and workflows to generate exercises and conduct mock interviews across various software engineering domains.

### Main Technologies
- **Markdown**: Primary format for documentation and Q&A content.
- **Rust**: Used for code examples, exercises, and syntax demonstrations.
- **Gemini CLI**: The engine driving the AI interaction and orchestration.

---

## 🏗️ Architecture & Organization

The project follows a modular architecture organized by domains and difficulty levels.

### 🤖 Agent System (`.agent/`)
- **Agents**: Specialist personas like `technical-interviewer`.
- **Skills**: Modular capabilities (e.g., `format-question-and-answer`, `bloom-generator`).
- **Workflows**: Slash command procedures for automation.
- **Rules**: Global mandates for agent behavior (defined in `.agent/rules/`).

### 📂 Directory Structure
- **Topics**: Top-level folders categorize domains (`backend/`, `frontend/`, `networking/`, `os/`, `DSA/`, `SOLID/`, `testing/`, `OOP/`).
- **Difficulty Levels**:
  - `foundation/QnA.md`: Core concepts and basic interview questions.
  - `advance/QnA.md`: Complex scenarios and deep-dive topics.
- **Rust Examples**: Located in `backend/rust/`, including standalone crates like `syntax/` and `hands-on-projects/`.

---

## 🚀 Key Commands & Workflows

### AI Slash Commands
| Command | Description |
|---------|-------------|
| `/question-generation` | Initiates a technical interview session (Foundation + Advance). |
| `/sync-agent-doc` | Synchronizes agent definitions, skills, and workflows with documentation. |
| `/update-workflow-or-skill` | Updates related workflows and skills in tandem. |
| `/search` | Specialized search across the knowledge base. |

### Development Commands (Rust)
- **Check Syntax**: `cargo check --manifest-path backend/rust/syntax/Cargo.toml`
- **Run Example**: `cargo run --manifest-path backend/rust/hands-on-projects/guessing_game/Cargo.toml`
- **Test**: `cargo test --manifest-path <crate-path>/Cargo.toml`

---

## ✍️ Development Conventions

### Bilingual (EN/VI) Formatting
All Q&A files must follow the bilingual prefix pattern:
```markdown
en: ## Question in English
vi: ## Câu hỏi bằng tiếng Việt

en: Answer in English.
vi: Câu trả lời bằng tiếng Việt.
```

### Coding Standards
- **Markdown**: Concise, instructional, using lowercase directory names and standard file patterns (`QnA.md`, `TOPIC.md`, `problems.md`).
- **Rust**: Standard `rustfmt`, `snake_case` for functions/files, `PascalCase` for types.
- **Git**: Short, imperative commit messages with prefixes (e.g., `docs:`, `fix:`, `feat:`).

### Quality Assurance
- Manually verify Markdown heading structure and link targets.
- Ensure all new Q&A content is bilingual.
- Run `cargo check` and `cargo test` for any Rust modifications.
