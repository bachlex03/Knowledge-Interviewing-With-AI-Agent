# Antigravity Kit Architecture

> **Version 5.0** - Comprehensive AI Agent Capability Expansion Toolkit

---

## 📋 Overview

Antigravity Kit is a modular system consisting of:
- **1 Specialist Agent** - Role-based AI personas
- **2 Skills** - Domain-specific knowledge modules
- **4 Workflows** - Slash command procedures

---

## 🏗️ Directory Structure

```
.agent/
├── ARCHITECTURE.md          # This file
├── agents/                  # 1 Specialist Agent
├── skills/                  # 2 Skills
├── workflows/               # 4 Slash Commands
├── rules/                   # Global Rules
└── .shared/                 # Shared Resources
```

---

## 🤖 Agents (1)

Specialist AI personas for different domains.

| Agent | Focus | Skills Used |
|-------|-------|-------------|
| `technical-interviewer` | Senior Software Engineer conducting technical interviews. | `format-question-and-answer` |

---

## 🧠 Skills (3)

Domain-specific knowledge modules. Skills are loaded on-demand based on task context.

### Frontend & UI
| Skill | Description |
|-------|-------------|
| (None) | No agents implemented yet | - |

### Backend & API
| Skill | Description |
|-------|-------------|
| (None) | No agents implemented yet | - |

### Database
| Skill | Description |
|-------|-------------|
| (None) | No agents implemented yet | - |

### TypeScript/JavaScript
| Skill | Description |
|-------|-------------|
| (None) | No agents implemented yet | - |

### Cloud & Infrastructure
| Skill | Description |
|-------|-------------|
| (None) | No agents implemented yet | - |

### Testing & Quality
| Skill | Description |
|-------|-------------|
| (None) | No agents implemented yet | - |

### Security
| Skill | Description |
|-------|-------------|
| (None) | No agents implemented yet | - |

### Architecture & Planning
| Skill | Description |
|-------|-------------|
| (None) | No agents implemented yet | - |

### Mobile
| Skill | Description |
|-------|-------------|
| (None) | No agents implemented yet | - |

### Game Development
| Skill | Description |
|-------|-------------|
| (None) | No agents implemented yet | - |

### SEO & Growth
| Skill | Description |
|-------|-------------|
| (None) | No agents implemented yet | - |

### Shell/CLI
| Skill | Description |
|-------|-------------|
| (None) | No agents implemented yet | - |

### Other
| Skill | Description |
|-------|-------------|
| `bloom-generator` | (No SKILL.md content) |
| `format-question-and-answer` | Enforce bilingual (EN/VI) Q&A formatting |
| `skill-creator` | Guide for creating effective skills |

---

## 🔄 Workflows (4)

Slash command procedures. Invoke with `/command`.

| Command | Description |
|---------|-------------|
| `/question-generation` | Generate technical foundation and advanced Q&A |
| `/sync-agent-doc` | Check and sync agents, skills, workflows |
| `/update-workflow-or-skill` | Update related workflows and skills |
| `/search` | Search files and content |

---

## 🎯 Skill Loading Protocol

```
User Request → Skill Description Match → Load SKILL.md
                                            ↓
                                    Read references/
                                            ↓
                                    Read scripts/
```

### Skill Structure

```
skill-name/
├── SKILL.md           # (Required) Metadata & instructions
├── scripts/           # (Optional) Python/Bash scripts
├── references/        # (Optional) Templates, docs
└── assets/            # (Optional) Images, logos
```

<!-- ### Enhanced Skills (with scripts/references)

| Skill | Files | Coverage |
|-------|-------|----------|
| `typescript-expert` | 5 | Utility types, tsconfig, cheatsheet |
| `ui-ux-pro-max` | 27 | 50 styles, 21 palettes, 50 fonts |
| `app-builder` | 20 | Full-stack scaffolding | -->

---

## 📊 Statistics

| Metric | Value |
|--------|-------|
| **Total Agents** | 1 |
| **Total Skills** | 3 |
| **Total Workflows** | 4 |
| **Coverage** | ~10% |

---

## 🔗 Quick Reference

| Need | Agent | Skills |
|------|-------|--------|
| Sync Docs | - | - |

