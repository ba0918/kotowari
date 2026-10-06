# Consistency phase

English | [日本語](consistency-phase.ja.md)

Covers finding and resolving, by an LLM in a dedicated check inside the cycle, the disagreements between code that was implemented or changed and the `IR`, and the gaps and contradictions in the `IR` itself. A person is involved only as a last resort.

## Requirements

### REQ-core-359: When the consistency phase runs

- kind: ubiquitous
- source: docs/decision/records/2026-10-06-changes-rethink.md#A3, docs/decision/records/2026-10-06-changes-rethink.md#A5, docs/decision/records/2026-10-06-changes-rethink.md#A11, docs/decision/records/2026-10-06-changes-rethink.md#A16, docs/decision/records/2026-10-06-changes-rethink.md#A17, docs/decision/records/2026-10-06-changes-rethink.md#A19, docs/decision/records/2026-10-06-changes-rethink.md#A27, docs/decision/records/2026-10-06-changes-rethink.md#A28, docs/decision/records/2026-10-06-changes-rethink.md#A29
- verification: review
- how_to_verify: Read "agent/skills/kotowari-cycle/", "agent/skills/kotowari-iterate/" and "agent/skills/kotowari-using-workflow/" and confirm that all of the following are written. The steps of the cycle are implementation, the `consistency phase`, then the quality review. The `consistency phase` runs once after implementation and once after the quality review's fixes converge, over the diff of all the fixes, and after its own fixes it runs until it raises no new finding. The cycle (the orchestrator) decides whether to run it, and for a direct fix in the main session the main session's agent decides by the same measure. It may be skipped only when the diff neither adds nor changes behavior a user can observe (documents such as guides only, tests only, a refactoring that keeps behavior); skill text counts as behavior and is not skipped; it runs when in doubt; a skip writes a one-line reason in the final report. The instructions handed to the phase are in one reference under "agent/skills/kotowari-cycle/". No new skill was added. kotowari-iterate passes through the phase via the cycle

kotowari-cycle always runs the `consistency phase` after implementation and after the quality review's fixes converge, as a step separate from the quality review, and skips it, leaving a reason, only when the diff neither adds nor changes behavior a user can observe. A direct fix in the main session runs it by the same measure.

### REQ-core-360: What the consistency phase reads

- kind: ubiquitous
- source: docs/decision/records/2026-10-06-changes-rethink.md#A2, docs/decision/records/2026-10-06-changes-rethink.md#A10, docs/decision/records/2026-10-06-changes-rethink.md#A12, docs/decision/records/2026-10-06-changes-rethink.md#A14, docs/decision/records/2026-10-06-changes-rethink.md#A15, docs/decision/records/2026-10-06-changes-rethink.md#A24, docs/decision/records/2026-10-06-changes-rethink.md#A25, docs/decision/records/2026-10-06-changes-rethink.md#A29
- verification: review
- how_to_verify: Read the reference for the `consistency phase` under "agent/skills/kotowari-cycle/" and confirm that all of the following are written. It reads the code itself, not records, and the code includes text shipped as product, such as skill text. Its range is the code the diff changed and its direct callers and callees, one step of where the values the diff uses are produced, the `item` in range (each `item` the diff touches, and each `item` whose ID the plan covers or a `mark` the diff adds carries), the terms those items use, the relations between items (definition tables, @about, references), and the `decision record` that the items and terms in range cite. For a topic with no `IR` the counterpart is the approved specification document, or the request when there is none. What it looks for is behavior absent from the `IR` (a gap), behavior different from the `IR`, and gaps and contradictions within the `IR` in that range. A comparison of the whole code base with the whole `IR` runs only when asked for separately

The `consistency phase` always reads the code the diff changed with its direct surroundings and one step of where its values come from, and the `item` in range with its terms, relations and cited `decision record`, and finds behavior absent from the `IR`, behavior different from the `IR`, and gaps and contradictions in that part of the `IR`.

### REQ-core-361: How findings are resolved

- kind: ubiquitous
- source: docs/decision/records/2026-10-06-changes-rethink.md#A6, docs/decision/records/2026-10-06-changes-rethink.md#A8, docs/decision/records/2026-10-06-changes-rethink.md#A9, docs/decision/records/2026-10-06-changes-rethink.md#A13, docs/decision/records/2026-10-06-changes-rethink.md#A26, docs/decision/records/2026-10-06-changes-rethink.md#A29
- verification: review
- how_to_verify: Read the reference for the `consistency phase` under "agent/skills/kotowari-cycle/" and confirm that all of the following are written. Every finding is resolved into a state where the `IR` and the code agree, and the code is never put back to match the `IR` when the `IR` has a hole. What observation can settle is settled by measuring or reproducing it. A concretisation consistent with the existing `IR` is added to the `IR` and a `decision record` with its grounds. A reading that widens or narrows the definition of a term or a table is a disagreement, not a concretisation. For a disagreement with the existing `IR`, the reasons in the `decision record` the `item` cites are read as constraints; the `IR` is revised, with the reason kept, only when it is shown to be wrong, and otherwise the code is brought to the `IR`. Only a contradiction between parts of the `IR` or a reproduction showing it cannot be realised shows the `IR` is wrong; passing tests are not grounds. A question of meaning no experiment can settle is resolved with a default that does not contradict the `IR`, work continues, it is left in the `flag record` as awaiting a person, and the final report states the default and the one word that reverses it. On agreement with the specification, the phase's decision takes precedence over the quality review's fixes. Only what still needs a judgment after the grounds and measurements are exhausted goes back to a person. The cycle's former handling, leaving the IR to the fixer and returning contradictions to the person, is replaced by this

The `consistency phase` always resolves its findings, on grounds and measurements, into a state where the `IR` and the code agree, and only for a question of meaning the grounds cannot settle continues with a default and leaves it awaiting a person.

### REQ-core-363: How the consistency phase fixes and ends

- kind: ubiquitous
- source: docs/decision/records/2026-10-06-changes-rethink.md#A13, docs/decision/records/2026-10-06-changes-rethink.md#A28, docs/decision/records/2026-10-06-changes-rethink.md#A29
- verification: review
- how_to_verify: Read the reference for the `consistency phase` under "agent/skills/kotowari-cycle/" and the cycle's steps, and confirm that all of the following are written. The phase itself fixes the `IR`, the `decision record`, the `flag record` and the code and commits them, and the cycle only records those commits. What was decided is written to a new `decision record` for that run, naming the `consistency phase` as the decider, with the choice, the reason and pointers to the grounds (the measuring commands and their results, files and lines). When it fixes the `IR`, it keeps the language pairs and their consistency records aligned and passes kotowari check. After a fix it runs again and also reads whether the recorded grounds support the choice. A run that raises no new finding is convergence, and the same finding twice in a row enters the cycle's ending for no progress

The `consistency phase` always fixes and commits by itself, keeps what it decided with its grounds in a new `decision record`, runs until it raises no new finding, and stops the cycle by its ending for no progress when the same finding appears twice in a row.

### REQ-core-362: Review holds only the quality perspective

- kind: ubiquitous
- source: docs/decision/records/2026-10-06-changes-rethink.md#A18, docs/decision/records/2026-10-06-changes-rethink.md#A29
- verification: review
- how_to_verify: Read "agent/skills/kotowari-review/", "agent/skills/kotowari-cycle/" and "agent/skills/kotowari-iterate/" and confirm that reviewers are launched with the quality perspective only, that there is neither a conformance seat comparing code with the specification nor a rule adding one, and that whether a plan or document contradicts the specification is looked at within the quality perspective

kotowari-review always launches reviewers with the quality perspective only and leaves the agreement of code and specification to the `consistency phase`.
