# ZK Proof System Research - Instructions

## Project Context

Researching zero-knowledge proof systems as applied to blockchain development. Focus on understanding core concepts before implementation.

---

## Workflow Orchestration

### 1. Plan Mode Default
- Enter plan mode for ANY non-trivial task (3+ steps or architectural decisions)
- If something goes sideways, STOP and re-plan immediately - don't keep pushing
- Use plan mode for verification steps, not just building
- Write detailed specs upfront to reduce ambiguity

### 2. Subagent Strategy
- Use subagents liberally to keep main context window clean
- Offload research, exploration, and parallel analysis to subagents
- For complex problems, throw more compute at it via subagents
- One task per subagent for focused execution

### 3. Self-Improvement Loop
- After ANY correction from the user: update `tasks/lessons.md` with the pattern
- Write rules for yourself that prevent the same mistake
- Ruthlessly iterate on these lessons until mistake rate drops
- Review lessons at session start for relevant project

### 4. Verification Before Done
- Never mark a task complete without proving it works
- Diff behavior between main and your changes when relevant
- Ask yourself: "Would a staff engineer approve this?"
- Run tests, check logs, demonstrate correctness

### 5. Demand Elegance (Balanced)
- For non-trivial changes: pause and ask "is there a more elegant way?"
- If a fix feels hacky: "Knowing everything I know now, implement the elegant solution"
- Skip this for simple, obvious fixes - don't over-engineer
- Challenge your own work before presenting it

### 6. Autonomous Bug Fixing
- When given a bug report: just fix it. Don't ask for hand-holding
- Point at logs, errors, failing tests - then resolve them
- Zero context switching required from the user
- Go fix failing CI tests without being told how

---

## Task Management

1. **Plan First**: Write plan to `tasks/todo.md` with checkable items
2. **Verify Plan**: Check in before starting implementation
3. **Track Progress**: Mark items complete as you go
4. **Explain Changes**: High-level summary at each step
5. **Document Results**: Add review section to `tasks/todo.md`
6. **Capture Lessons**: Update `tasks/lessons.md` after corrections

---

## Core Principles

- **Simplicity First**: Make every change as simple as possible. Impact minimal code.
- **No Laziness**: Find root causes. No temporary fixes. Senior developer standards.
- **Minimal Impact**: Changes should only touch what's necessary. Avoid introducing bugs.

---

## Research & Exploration

These principles apply equally to research phases when understanding concepts before development.

### Principle Adaptation

| Development Principle | Research Equivalent |
|----------------------|---------------------|
| **Plan Mode Default** | Define what you're trying to understand before diving in |
| **Subagent Strategy** | Explore multiple angles in parallel (papers, code, docs) |
| **Self-Improvement Loop** | Capture learnings in notes, build mental models |
| **Verification Before Done** | Validate understanding by explaining it back or prototyping |
| **Demand Elegance** | Seek the clearest mental model, not just any understanding |
| **Autonomous Bug Fixing** | When confused, dig deeper instead of asking vague questions |

### Research Workflow

1. **DEFINE** the question clearly
   - "What problem does X solve?"
   - "How does Y work under the hood?"
   - "What are the trade-offs between A and B?"

2. **EXPLORE** multiple sources
   - Documentation, papers, existing code
   - Run experiments, build small proofs-of-concept
   - Talk to experts or read discussions

3. **SYNTHESIZE** into a mental model
   - Can you explain it simply?
   - Can you draw a diagram?
   - Can you predict behavior in new scenarios?

4. **VALIDATE** understanding
   - Build a small prototype
   - Explain it to someone (or write it down)
   - Find edge cases that test your model

5. **DOCUMENT** for future reference
   - Capture key insights in `tasks/research.md`
   - Note open questions for later

### Research Quality Checks

- **Clarity Test**: Can you explain this concept in one sentence?
- **Diagram Test**: Can you visualize the system/flow?
- **Prediction Test**: Given a new input, can you predict the output?
- **Edge Case Test**: What happens at the boundaries?
- **Why Test**: Do you understand *why* it works, not just *how*?

### Anti-Patterns to Avoid

- Skimming without understanding
- Collecting links without reading them
- Moving to implementation while still confused
- Assuming you understand without validating
- Not documenting insights (you'll forget them)

---

## ZK Research Topics

### Core Concepts to Understand
- [ ] What is a zero-knowledge proof?
- [ ] Interactive vs non-interactive proofs
- [ ] SNARKs vs STARKs vs Bulletproofs
- [ ] Constraint systems (R1CS, CCS, AIR)
- [ ] Commitment schemes (Pedersen, KZG, Ajtai)
- [ ] Fiat-Shamir transformation

### Blockchain Applications
- [ ] Shielded transactions (Zcash model)
- [ ] Rollups (zk-rollups, validity proofs)
- [ ] Private smart contracts
- [ ] Proof aggregation / recursion
- [ ] Post-quantum considerations

### Implementation Patterns
- [ ] Circuit design patterns
- [ ] Witness generation
- [ ] Prover optimization
- [ ] Verifier gas costs
- [ ] Trusted setup vs transparent setup
