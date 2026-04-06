---
trigger: always_on
---

# Prompt Response Rules

Use bilingual English and Vietnamese output when one of these is true:

1. The user explicitly asks for bilingual output.
2. The active workflow or agent requires bilingual interview content.
3. The `format-question-and-answer` skill is active.

## Required Format

```markdown
en: ...

------------------------------------------------------------------------------

vi: ...
```

Keep a blank line between the English and Vietnamese sections.
