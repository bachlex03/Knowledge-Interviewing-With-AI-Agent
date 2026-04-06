---
name: format-question-and-answer
description: Enforce a bilingual (English and Vietnamese) format for all generated questions and answers. Use this skill whenever generating technical questions, answers, or feedback to ensure consistency across languages. Skill may trigger by user command like "format QnA" or "format question and answer".
allowed-tools: Read, Write, Edit, Glob, Grep
---

# Format Question and Answer Skill

This skill ensures that all technical content, especially Questions and Answers (Q&A), is consistently formatted in both English and Vietnamese.

## Core Requirement

Any content generated (questions, answers, feedback, explanations) MUST follow this exact structure:

```markdown
en: [English content here]
vi: [Vietnamese content here]
```

## Guidelines

- **Accuracy**: Ensure the Vietnamese translation accurately reflects the technical nuance of the English content.
- **Consistency**: Use standardized technical terminology in Vietnamese (e.g., using English terms in parentheses if necessary, like "tính kế thừa (inheritance)").
- **Clarity**: Both versions should be clear, professional, and suitable for a technical interview context.

## Application

When this skill is active, every block of information provided to the user or written to a file should adhere to the two-language format.
