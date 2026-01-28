---
trigger: always_on
---

# GEMINI.md - Maestro Configuration

> Maestro AI Development Orchestrator
> This file defines how the AI behaves in this workspace.

---

## CRITICAL: System Map Read

> 🔴 **MANDATORY:** Read `.agent/ARCHITECTURE.md` at session start to understand Agents, Skills, and Scripts.


## CRITICAL: AGENT & SKILL PROTOCOL

> **MANDATORY:** You MUST read the appropriate agent file and its skills BEFORE performing any implementation. This is the highest priority rule.

### 1. Modular Skill Loading Protocol
```
Agent activated → Check frontmatter "skills:" field
    │
    └── For EACH skill:
        ├── Read SKILL.md (INDEX only)
        ├── Find relevant sections from content map
        └── Read ONLY those section files
```

- **Selective Reading:** DO NOT read ALL files in a skill folder. Read `SKILL.md` first, then only read sections matching the user's request.
- **Rule Priority:** P0 (GEMINI.md) > P1 (Agent .md) > P2 (SKILL.md). All rules are binding.

### 2. Enforcement Protocol
1. **When agent is activated:**
   - ✅ READ all rules inside the agent file.
   - ✅ CHECK frontmatter `skills:` list.
   - ✅ LOAD each skill's `SKILL.md`.
   - ✅ APPLY all rules from agent AND skills.
2. **Forbidden:** Never skip reading agent rules or skill instructions. "Read → Understand → Apply" is mandatory.

---

## REQUEST CLASSIFIER

**Before ANY action, classify the request:**

| Request Type | Trigger Keywords | Active Tiers | Result |
|--------------|------------------|--------------|--------|
| **QUESTION** | "what is", "how does", "why", "who" "explain" | TIER 0 only | Text Response |

---

## TIER CLASSIFIER

### "TIER 0": UNIVERSAL RULES (Always Active)

#### 🌐 Language Handling

When user's prompt is NOT in English:
1. **Internally translate** for better comprehension
2. **Code comments/variables** remain in English

---

## Response output

When workflow, agent, skill is called, response this example output:

```
Workflow processing: [workflow' name]
[Detail of the workflow step being executed]
==> Calling agent [agent' name]

Agent calling: [Agent 'name]
[Description of agent's current focus]
-> Using skills: [skill_1], [skill_2],...
[Actions performed using these skills]
==> calling next agent [agent' name] (if applicable)
```

### CRITICAL: When STOP response output?

**Validation**: When there are no exist workflow' name, agent' name, skill' name. You must WARNING user these are not exist then STOP response

## CRITICAL: Validate "QUICK REFERENCE" section

**Validation**: in "QUICK REFERENCE" section in this rule must match all exist workflows, agents, skills, scripts if not warning user then STOP response

## QUICK REFERENCE

### Available Master Agents (0)

| Agent | Domain & Focus |
|-------|----------------|
| `technical-interviewer` | Senior Software Engineer conducting technical interviews. |

### Key Skills (2)

| Skill | Purpose |
|-------|---------|
| `format-question-and-answer` | Enforce bilingual (EN/VI) Q&A formatting |
| `skill-creator` | Guide for creating effective skills |

### Script Locations

| Script | Path |
|--------|------|
| (None) | No scripts implemented yet |


---


