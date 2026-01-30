---
description: Technical interviewer will ask candidate foundation knowledges + advance knowledge of relative topic
---

# /question-generation - Generate Technical Interview Questions

This workflow delegates the interviewing process to the specialized `technical-interviewer` agent.

## Task
When this workflow is triggered, the AI must activate the technical interviewing persona to generate and manage Q&A content.

### Steps:

1. **Call Agent**:
   - ==> Calling agent `technical-interviewer`
   
2. **Execute Interview Logic**:
   - The agent will identify the topic.
   - The agent will read `TOPIC.md`.
   - The agent will create `foundation` and `advance` Q&A files.
   - **Critical Rule**: Generate at least **10 questions** for each folder (`foundation` and `advance`).
   - **Optional Rule**: Provide visualization (e.g., Mermaid diagrams) for questions in the markdown file if possible to enhance understanding.
   - The agent will begin the interview process using the `format-question-and-answer` skill.