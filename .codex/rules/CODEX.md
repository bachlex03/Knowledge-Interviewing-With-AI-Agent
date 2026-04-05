---
trigger: always_on
---

# Codex Workspace Rules

This file defines the repo-local operating rules for Codex in this workspace.

## Mandatory Context

1. Read `.agent/ARCHITECTURE.md` before making changes that involve agents, skills, workflows, or workspace orchestration.
2. When a request targets an agent, workflow, or skill, read the relevant file before implementation.
3. For skills, read `SKILL.md` first, then only open the specific bundled resources needed for the request.

## Rule Priority

Use this precedence when multiple instruction sources apply:

1. `.codex/rules/CODEX.md`
2. Agent file in `.agent/agents/`
3. Skill file in `.codex/skills/` or `.agent/skills/`

## Request Classification

Classify the request before acting.

- Question or explanation requests: answer directly after reading only the relevant context.
- Implementation requests: read the relevant agent, workflow, and skill files first, then make changes.

## Language Handling

When the user prompt is not in English:

1. Translate internally if needed for reasoning.
2. Keep code, filenames, variables, and code comments in English.

## Workflow And Skill Execution Output

When explicitly executing a workflow, agent, or skill-driven process, use concise trace output in this form:

```text
Workflow processing: [workflow-name]
[current step]
==> Calling agent [agent-name]

Agent calling: [agent-name]
[current focus]
-> Using skills: [skill-1], [skill-2]
```

Do not use this trace format for normal coding tasks unless the user is asking about workflows, agents, or skill execution.

## Validation Rules

1. If a requested workflow, agent, or skill does not exist, warn the user and stop before implementation.
2. Keep quick-reference sections accurate. If the documented list is stale, update the docs before relying on it.

## Quick Reference

### Agents

| Agent | Domain And Focus |
|-------|------------------|
| `technical-interviewer` | Senior software engineer conducting technical interviews |

### Skills

| Skill | Purpose |
|-------|---------|
| `bloom-generator` | Generate Bloom's Taxonomy interview or study questions |
| `format-question-and-answer` | Format technical content in bilingual English and Vietnamese |
| `skill-creator` | Create or update repo-local Codex skills |

### Workflows

| Workflow | Description |
|----------|-------------|
| `/question-generation` | Generate technical interview questions for a topic |
| `/search` | Search files and repository content |
| `/sync-agent-doc` | Sync agent, skill, workflow, and architecture docs |
| `/update-workflow-or-skill` | Update related workflows or skills together |

### Rule Files

| File | Purpose |
|------|---------|
| `.codex/rules/CODEX.md` | Core repo-local Codex rules |
| `.codex/rules/prompt-response.md` | Response-format rules for bilingual interview content |
