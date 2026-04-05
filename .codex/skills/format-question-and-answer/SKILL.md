---
name: format-question-and-answer
description: Enforce a bilingual English and Vietnamese format for generated questions, answers, explanations, or feedback. Use when Codex needs to produce technical interview content in both languages, or when the user asks to format QnA bilingually.
---

# Format Question And Answer

Use this skill to keep technical content consistently formatted in both English and Vietnamese.

## Required Structure

Any generated question, answer, explanation, or feedback should follow this structure:

```markdown
en: [English content here]
vi: [Vietnamese content here]
```

## Guidelines

- Preserve the technical meaning accurately across both languages.
- Use standard Vietnamese technical wording and keep English terms in parentheses when that improves clarity.
- Apply the format consistently for all relevant blocks in the response.

## Application

When this skill is active, every relevant question-and-answer block returned to the user or written into repository files should follow the bilingual structure.
