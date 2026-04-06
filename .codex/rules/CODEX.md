---
trigger: always_on
---

# Codex Workspace Rules

This file defines the repo-local operating rules for Codex in this workspace.

## Core Policy

1. Treat this repository as having local instruction files under `.codex/` and `.agent/`.
2. When a request involves agents, workflows, skills, or prompt formatting, read the relevant local rule or skill file before editing anything.
3. Prefer the smallest applicable context. Do not read unrelated files unless they are needed for the task.

## Priority Order

When multiple instruction sources apply, use this order:

1. `SYSTEM` and `DEVELOPER` instructions from the current session
2. `.codex/rules/CODEX.md`
3. The relevant agent, workflow, or skill file
4. Other repo documentation only when needed for implementation

## Request Handling

- Question or explanation requests: answer directly, using only the context needed to be accurate.
- Implementation requests: inspect the relevant agent, workflow, and skill files first, then make the requested change.
- If a requested agent, workflow, or skill does not exist, say so clearly and stop instead of inventing behavior.

## Language Handling

When the user prompt is not in English:

1. Translate internally if needed for reasoning.
2. Keep code, filenames, variables, and comments in English unless the existing file already uses another language.

## Output Convention

When explicitly executing a workflow, agent, or skill-driven process, use concise trace output only if the user is asking for that execution flow.

Suggested format:

```text
Workflow processing: [workflow-name]
[current step]
==> Calling agent [agent-name]

Agent calling: [agent-name]
[current focus]
-> Using skills: [skill-1], [skill-2]
```

Do not use this trace format for normal coding tasks.

## Validation

1. Keep local rule references accurate.
2. If a doc or rule file is stale and the task depends on it, update the doc before relying on it.

## Local Rule Files

| File | Purpose |
|------|---------|
| `.codex/rules/CODEX.md` | Core repo-local Codex rules |
| `.codex/rules/prompt-response.md` | Prompt-format rules for bilingual interview content |
