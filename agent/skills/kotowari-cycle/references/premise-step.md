# Questioning the premise

When fixes keep raising new findings next to the ones they closed, the fixes usually share a premise
that is wrong, and another fix on the same premise moves the problem without removing it. This step
names that premise and tries once with it replaced before the loop is handed to the person.

Cycle runs it itself, as the one judge of the loop; it adds no agent and no seat. Cycle's
**Endings** says which clauses of ending 3 enter it. kotowari-iterate runs it as part of cycle's
loop, the main session runs it on a direct edit (kotowari-using-workflow), and an implementer runs
its own form of it (**In implement**).

## The step

1. **Write the premise ladder.** Read the fixes behind the clause that fired: their commits, the
   findings each addressed, and the premises the fixer reported for a finding still present after
   a fix. Write the premises those fixes shared, one sentence per rung, from the most specific
   (the place the last fix touched) up to the one the others rest on. Each rung says where the
   premise lives: in the implementation, or in the IR or a decision record.
2. **Replace the top premise** the findings call into question: state what replaces it and which
   findings point there.
3. **Record the attempt** in the findings file before delegating anything (the kotowari-review
   skill's `references/finding-schema.md`, `premise_attempts`).
4. **Send it by where the premise lives:**
   - the implementation → the fixer, with the visible findings, the ladder, and the replaced
     premise, to fix from the new premise rather than patch the last finding; the loop goes on with
     the diff review after that fix;
   - the IR or a decision record, and grounds and measurements can decide it → a consistency phase
     run, with the ladder and the replaced premise as a finding to resolve; the loop then goes on
     where the clause fired;
   - otherwise (a question of meaning no grounds or measurement settle) → end as ending 3; the
     terminal report carries the ladder.

## Once per firing

A firing gets one attempt. After an attempt, the next time any clause that enters this step holds,
end as ending 3 without another attempt; the terminal report carries the ladder of every attempt in
the findings file. On a resume or after "run more", read `premise_attempts` first: a firing that
already has an attempt is not attempted again, and the allowance is not renewed. Only a new start of
kotowari-iterate on the branch renews it, as that start also resets ending 3's streaks.

## In implement

An implementer whose approach has changed once without progress does not hand back yet. It writes
the premises its attempts shared, as a ladder like the one above, replaces the top one, and tries
once more from it. Still no progress: hand back to cycle with the ladder. When the premise lies in
the plan or the specification, hand back at once, as before; the implementer never replaces those.
