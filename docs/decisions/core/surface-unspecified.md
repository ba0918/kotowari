# List of unspecified surfaces

English | [日本語](surface-unspecified.ja.md)

Covers the list of unspecified surfaces, which lists the surfaces not yet made part of the specification. It decides the place of the list and the form of one entry, which surface an entry of the list excludes, the findings on a malformed entry and on an entry no longer needed, and the stop when the list file cannot be read. Taking surfaces out of the code, judging whether a surface is written in the specification, outputting the number excluded, and errors in the combination of the surface configuration keys are decided in the topic [Checking surfaces](surface.md).

## A surface not yet made part of the specification is not an error if it is put on the list of unspecified surfaces with a reason

Status: implemented

### Problem

If surfaces not written in the specification were errors, then in an existing project, right after the surface rules are written, a large number of errors would appear and it could not be adopted. On the other hand, if surfaces that raise no error could be silently increased, the amount relying on the list would become invisible. With the same construction as the list of equivalents for mutation tests, a reason remains for each entry, and the amount relying on the list can be shown every time. The work of reducing the list is picked up by kotowari-adopt.

When an entry of the list is compared with a surface, removing surrounding whitespace or accepting a partial match would silently exclude surfaces wider than the writer intended. No reason for choosing an exact match is left in the records other than giving it the same construction as the list of equivalents.

### Decision

Surfaces not yet made part of the specification are listed one entry at a time in the YAML file that "surface.unspecified" in the configuration points at. An entry has three keys: the kind "kind", the name "name" and the reason "why", and the reason is always written.

An entry of the list excludes only a surface whose kind and name are both the same as that entry. What is compared is the values of "kind" and "name" as they are, without removing surrounding whitespace. For an excluded surface, surface_without_spec is not output even if it is not written in the specification. If the configuration has no "surface.unspecified" key, the check continues as if there were no list.

Outputting the number excluded in the output of check is decided in [The number of surfaces excluded by the list of unspecified surfaces is always output at the end of the output of check](surface.md#the-number-of-surfaces-excluded-by-the-list-of-unspecified-surfaces-is-always-output-at-the-end-of-the-output-of-check), and the handling of a configuration that has only the "surface.unspecified" key without surface rules in [When the surface configuration keys are not all present, every command that reads the configuration stops](surface.md#when-the-surface-configuration-keys-are-not-all-present-every-command-that-reads-the-configuration-stops).

### Confirmations

- C-core-unspec-1 A surface on the list gets no surface_without_spec error even if it is not written in the specification
- C-core-unspec-2 A surface whose kind or name differs from an entry of the list is not excluded, and gets a surface_without_spec error
- C-core-unspec-6 If an entry of the list has whitespace after its "name", a surface with the name without the whitespace is not excluded, and gets a surface_without_spec error

## A malformed entry is an error, and an entry no longer needed is a notice

Status: implemented

### Problem

The handling of an entry of the list that is broken and of one that is no longer needed had to be decided. The list of equivalents for mutation tests makes a malformed entry an error and an entry no longer needed a notice. The same handling is used.

If the conditions for a malformed entry were narrow, a broken entry or an entry whose reason is only whitespace would exclude surfaces as a well-formed entry. No reason for the detailed choice of conditions is left in the records other than giving it the same construction as the list of equivalents.

### Decision

An entry that is not a set of key-value pairs (a YAML mapping), an entry that is not in the form of the three keys "kind", "name" and "why", and an entry whose reason is empty are malformed, and a surface_unspecified_invalid error is output for each. A reason is regarded as empty when it is empty after removing leading and trailing half-width spaces and tabs. A malformed entry excludes no surface.

Among well-formed entries, one whose surface has disappeared from the code and one whose surface has come to be written in a decision document are entries no longer needed, and a surface_unspecified_stale notice is output for each. Because it is a notice, it does not change the exit code.

### Confirmations

- C-core-unspec-3 An entry whose reason is only whitespace, that lacks a key, that has a key other than the three, or whose value is not a string gets a surface_unspecified_invalid error, and the surface of that entry is not excluded
- C-core-unspec-4 An entry whose surface has disappeared from the code and an entry whose surface has been written in a decision document get a surface_unspecified_stale notice, and if there is no error besides the notices, the exit code stays 0
- C-core-unspec-7 An entry that is not a set of key-value pairs (an entry that is only a string) also gets a surface_unspecified_invalid error, and an entry whose reason is only half-width spaces and tabs is regarded as having an empty reason and gets a surface_unspecified_invalid error

## If the list file is missing or cannot be read, check and status stop

Status: implemented

### Problem

Only check and status read surfaces; list, query, plan and mutants do not. An error in the list file does not stop commands that do not read the list. No reason is left for the decision to stop, rather than continue the check, when the list file cannot be read.

If an unreadable list were continued as an empty list, the listed surfaces would silently become errors, or a broken list would go unnoticed. If the reason for the stop were mixed up, the person fixing it would not know what to look at (whether the file exists, the character encoding, or the form of the content). No reason for separating the kinds of reason is left in the records other than giving it the same construction as the list of equivalents.

### Decision

When the file that "surface.unspecified" in the configuration points at is missing, cannot be read, or cannot be read as a list (it is not UTF-8, cannot be read as YAML, or its top level is not a sequence), check and status stop. The reason for the stop is an unreadable file (unreadable file) if the file is missing or cannot be read, a non-UTF-8 file (non-UTF-8 file) if it is not UTF-8, and a configuration error (config error) if it cannot be read as YAML or its top level is not a sequence. Other commands do not read the list, so they do not stop on an error in the list file (which commands read surfaces is decided in [Only check and status read surfaces](surface.md#only-check-and-status-read-surfaces)).

### Confirmations

- C-core-unspec-5 When the list file is missing, is not UTF-8, cannot be read as YAML, or its top level is not a sequence, check and status stop
- C-core-unspec-8 The reason for the stop is "unreadable file" if the list file is missing, "non-UTF-8 file" if it is not UTF-8, and "config error" if it cannot be read as YAML or its top level is not a sequence
