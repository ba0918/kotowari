# Adoption scope and how rows are made

English | [日本語](adoption-scope.ja.md)

This document covers how the workflow skill kotowari-adopt decides the range of topics handled in one adoption, and how it reads the legacy material, the implementation and the tests to make the rows of the list. Where a row goes and how many times the person is asked are decided in [Adoption into an existing project](adoption.md). This document holds only decisions that a person or an LLM checks by reading the skill's text.

## One adoption handles one topic, chosen by the person

Status: implemented

### Problem

When kotowari is brought into an existing project midway, the legacy material, the decision records, the implementation and the tests disagree with each other or have gaps. Trying to decide all of it at the time of adoption inflates the interview and ends in swallowing the recommendations whole. In the adoption of one existing project, 54 migration judgments were to be settled first in one record, and 52 of them were adoptions of the recommendation.

### Decision

One adoption handles one topic. Which topic is chosen by the person in the scope round. The default offered is the feature planned to change next. No inventory of the whole project is made, and the topics not chosen stay as legacy material.

### Confirmations

- C-core-adoptscope-1 (by reading) The kotowari-adopt skill says that one run handles one topic, chosen by the person in the scope round, with the feature planned to change next as the default

## In the scope round, the person confirms the topic's user entry points, the code and test files, and the legacy material to read

Status: implemented

### Problem

Sorting while the scope is vague makes the denominator of the breakdown of test destinations shift. Also, code is shared between entry points, so cutting the scope by files turns the behaviour of other entry points into rows as well. In one adoption, files outside the confirmed scope were read and the behaviour of another entry point found there was made into rows, so about half of the 32 rows of ratification candidates were behaviour on a side other than that topic.

The legacy material to read is narrowed to keep the amount of reading down. Reasons in code comments or commit logs are no substitute for decision records.

### Decision

When the person has chosen a topic, in the same round the topic's user entry points (the commands and subcommands the user touches, the screen operations), the proposed code files and test files of the scope, and the candidate legacy material to read are put forward and confirmed with the person.

The legacy material is documents that existed before the adoption and write down a specification or decisions. Old specifications, existing decision records and ADRs, and the parts of a README that explain behaviour are such documents. Code comments and commit logs are not included in the legacy material.

### Confirmations

- C-core-adoptscope-2 (by reading) The kotowari-adopt skill says that in the scope round the topic's user entry points, the proposed code files and test files of the scope, and the candidate legacy material to read are confirmed with the person
- C-core-adoptscope-3 (by reading) The kotowari-adopt skill says that the legacy material is old specifications, existing decision records and ADRs, and the parts of a README that explain behaviour, and that code comments and commit logs are not included

## The main session reads and sorts, and when the topic cannot be read through it proposes splitting the topic

Status: implemented

### Problem

kotowari is built on the premise that the user cannot spend much. Delegation seats without a reason are not added. A size that cannot be read through in one go is a sign that the unit of the topic is too large.

### Decision

Reading and sorting the legacy material, the code and the tests is done by the main session. When the size cannot be read through in one go, it does not have another agent read it, but proposes to the person to split the topic smaller. This proposal and the choice of topic are made within the scope round.

### Confirmations

- C-core-adoptscope-4 (by reading) The kotowari-adopt skill says that reading and sorting are done by the main session, and when the size cannot be read through it does not have another agent read it but proposes in the scope round to split the topic smaller

## When there is no configuration, the kotowari skill's setup procedure is done first, and its questions do not count toward the times the person is asked

Status: implemented

### Problem

The repository being adopted may not have a kotowari configuration yet. Holding the setup procedure in two places makes them disagree. The questions during setup belong to the kotowari skill's procedure, so whether they count toward the times the person is asked during adoption ([The person is asked only three times: the scope round, one list round, and approval](adoption.md#the-person-is-asked-only-three-times-the-scope-round-one-list-round-and-approval)) had to be decided.

### Decision

kotowari-adopt first checks whether the target repository has `.kotowari/config.yaml`. If not, it does the setup procedure in the kotowari skill's `references/config.md` first. The content of the setup is not written in kotowari-adopt; its location is named. Questions asked of the person during setup do not count toward the times the person is asked during adoption.

### Confirmations

- C-core-adoptscope-5 (by reading) The kotowari-adopt skill says that it first checks whether `.kotowari/config.yaml` exists, and if not, does first the setup procedure in the kotowari skill's `references/config.md`, by naming it, and that its questions do not count toward the times the person is asked

## Only behaviour observable through the topic's user entry points becomes a row

Status: implemented

### Problem

In one adoption, files outside the scope were read to understand the save and restore processing, and the behaviour found in them was made into rows as well, so the topic swelled. Widening what is read for understanding is needed, but making everything read into specification swells the topic. What was found should be kept as a clue for the next topic.

### Decision

Only behaviour observable through the topic's user entry points becomes a row of the list. Behaviour observed through another entry point is outside the scope, even when it shares the same code.

Files outside the scope may be read for understanding. Behaviour outside the scope found there does not become a row. In the list round, only the number of behaviours found outside the scope and the names of the candidate next topics are shown.

### Confirmations

- C-core-adoptscope-6 (by reading) The kotowari-adopt skill says that only behaviour observable through the topic's user entry points becomes a row, and behaviour observed through another entry point is outside the scope even when it shares the same code
- C-core-adoptscope-7 (by reading) The kotowari-adopt skill says that files outside the scope may be read, but behaviour found there does not become a row, and in the list round only its number and the names of the candidate next topics are shown

## Behaviours under the same rule that differ only in a value or a wording are put together in one row as a candidate decision table

Status: implemented

### Problem

In one adoption, the differences in wording of one rule (the wording of skip reasons) became separate rows, and the rows swelled.

### Decision

Behaviours under the same rule that differ only in a value or a wording are put together in one row of the list, as a candidate to be written as a decision table. In the list's column for the implementation's behaviour, the table's contents are listed briefly.

### Confirmations

- C-core-adoptscope-8 (by reading) The kotowari-adopt skill says that things under the same rule that differ only in a value or a wording are put together in one row as a candidate decision table, and the table's contents are listed briefly in the column for the implementation's behaviour

## One row of the list is a candidate for one decision

Status: proposed

### Problem

One row of the list used to be a candidate for one requirement. Since the specification is now held only in decision documents and requirements are gone, what one row is a candidate for had to be decided again. The reason for making it one decision is not recorded.

### Decision

One behaviour row of the list is a candidate for one decision.

### Confirmations

- C-core-adoptscope-9 (by reading) The kotowari-adopt skill says that one row of the list is a candidate for one decision
