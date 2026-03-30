---
name: bloom-generator
description: Generates educational or technical interview questions across 5 levels of Bloom's Taxonomy (Remembering, Understanding, Applying, Analyzing, Evaluating) for a given topic. Use this skill when the user asks to generate questions following Bloom's Taxonomy for any specific topic.
---

# Bloom's Taxonomy Question Generator (5 Levels)

This skill guides you to generate questions based on a simplified 5-level version of Bloom's Taxonomy for any given topic.

## The 5 Levels of Bloom's Taxonomy

When generating questions, categorize them strictly into these 5 levels:

1. **Remembering**: Testing basic recall of facts, terms, and basic concepts.
   *Focus:* Facts and definitions.
   *Example verbs:* define, list, name, identify.

2. **Understanding**: Testing comprehension of the facts and ideas by organizing, comparing, translating, interpreting, or stating main ideas.
   *Focus:* Explanations and summaries.
   *Example verbs:* explain, describe, summarize, discuss.

3. **Applying**: Testing the ability to use acquired knowledge, facts, techniques, and rules in a different way or in a new situation.
   *Focus:* Practical execution and problem-solving.
   *Example verbs:* solve, apply, use, demonstrate.

4. **Analyzing**: Testing the ability to examine and break information into parts by identifying motives or causes, making inferences, and finding evidence to support generalizations.
   *Focus:* Breaking down complex systems and comparing parts.
   *Example verbs:* analyze, categorize, compare, contrast.

5. **Evaluating**: Testing the ability to present and defend opinions by making judgments about information.
   *Focus:* Judgments and critiques.
   *Example verbs:* evaluate, judge, appraise, argue, defend.

## Task Instructions

1. Identify the **Topic** the user wants questions for (e.g., "trains", "Agile methodology", "React.js").
2. Generate questions for **each** of the 5 levels of Bloom's Taxonomy related to the topic. Note that **each level can have more than 1 question**. If the user doesn't specify a quantity, you can generate any number of questions per level as you want if needed for fully understand (e.g., 5 questions per level).
3. For each question, provide a brief expected answer or the key points that the answer should cover.
4. **Code-related answers**: If the answer relates to code implementations, architectures, or practical application of programming concepts (especially common in Level 3 and 4), provide **real code examples** enclosed in triple backticks (e.g., ```rust) below the main text answer.
5. If the `.agent/rules/GEMINI.md` specifies formatting rules (such as bilingual output via the `format-question-and-answer` skill), make sure you apply those rules to the final output.

## Output Format

### Level 1: Remembering
**Question:** [Your question]
**Answer:** [Key points of the answer]

### Level 2: Understanding
**Question:** [Your question]
**Answer:** [Key points of the answer]

### Level 3: Applying
**Question:** [Your question]
**Answer:** [Key points of the answer]
```[language]
// Real code example if applicable
```

### Level 4: Analyzing
**Question:** [Your question]
**Answer:** [Key points of the answer]
```[language]
// Real code example if applicable
```

### Level 5: Evaluating
**Question:** [Your question]
**Answer:** [Key points of the answer]
