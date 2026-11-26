# The skill references agreeing with the binary

English | [日本語](skill-references.ja.md)

Covers the check that the values and forms of the binary copied into the references of the kotowari skill (the documents in "agent/skills/kotowari/references/") agree with the binary. This check is not behaviour of kotowari; the tests of this repository perform it. What they are compared against is the values held by the binary's code, not the tables in the specification. The configuration keys and default values themselves are decided in [The configuration keys are only those in the table, and a key not written takes the default value in the table](config.md#the-configuration-keys-are-only-those-in-the-table-and-a-key-not-written-takes-the-default-value-in-the-table).

## Values copied into the skill's references are compared with the binary's code by the tests of this repository

Status: implemented

### Problem

The skill's references carry finding kinds, configuration defaults and the wording of stops, copied from the binary. When the binary changed, the references went stale, and that drift was found and fixed by hand.

Tests need markers, and markers point at specification IDs, so the contract of this comparison is also placed in the specification. What is compared against is the code, which is what runs. The existing tests keep the specification's tables and the code in agreement, so if the references and the code agree, they also agree with the specification.

### Decision

The tests of this repository compare the following three against the binary's code.

- The set of the first column of the table of finding kinds in "agent/skills/kotowari/references/findings.md" equals the set of finding kinds the binary's code outputs.
- The configuration defaults in "agent/skills/kotowari/references/config.md" equal the configuration defaults held by the binary's code (how they are compared is in [Configuration defaults are compared by passing the YAML of the setup steps through the binary's reading](#configuration-defaults-are-compared-by-passing-the-yaml-of-the-setup-steps-through-the-binarys-reading)).
- The set of the first column of the table of stop wording in "agent/skills/kotowari/references/findings.md" equals the set of wording the binary's code outputs on the first line of standard error as the reason for a stop. The second column of the table is not read.

### Confirmations

- C-core-skillref-1 A test checks that the set of the first column of the table of finding kinds in findings.md equals the set of finding kinds the binary's code outputs
- C-core-skillref-2 A test checks that the set of the first column of the table of stop wording in findings.md equals the set of wording the binary's code outputs on the first line of standard error as the reason for a stop

## Configuration defaults are compared by passing the YAML of the setup steps through the binary's reading

Status: implemented

### Problem

The defaults in config.md are written both in a table for people to read and in the YAML block of step 1 of setup. The table stays for people. No reason is left for the decision of which one to check against.

However, keys not written in a configuration file are filled with defaults, so only comparing the result of reading with the defaults would not notice a key missing from the block. Also, a key with no default (the place of the list of equivalents for mutation tests, for example) would not equal a default if a value were written, and would stop with a configuration error if an empty value were written, so there is no point in writing it in the YAML that is compared.

### Decision

The defaults in config.md are checked not against the table but against the YAML code block in step 1 of setup. The table stays for people and is not read by machine.

The test first checks that every configuration key that has a default is written in that block, and then checks that the result of reading that block as a configuration file with the binary's code equals the configuration default values held by the binary's code. Keys with no default are not written in this block.

### Rejected options

- Give defaults to keys with no default too, so that they can be written in the block. This would overturn the decision not to give them defaults, and projects without a configuration file would stop.

### Confirmations

- C-core-skillref-3 A test checks that the YAML block of step 1 of setup in config.md has every key that has a default written in it, and that the result of reading it as a configuration file with the binary's code equals the configuration defaults

## The headings of the first columns that the tests key on are Kind and Message

Status: implemented

### Problem

If the skill's references are put into English, the tests' promise of keying on the Japanese column names ("種類", "文言") no longer holds.

### Decision

In "agent/skills/kotowari/references/findings.md", the heading of the first column that the tests of this repository key on is `Kind` for the table of finding kinds and `Message` for the table of stop wording. `Kind` was aligned with "kind" in the JSON of findings. The wording of a stop is the text on the first line of standard error, so it was made `Message`. The headings of the other columns may be translated freely into English.

### Confirmations

- C-core-skillref-4 findings.md has a table whose first-column heading is `Kind` and a table whose first-column heading is `Message`, and the tests read the first column of those tables
