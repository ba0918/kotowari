# Commands and exit

English | [日本語](cli.ja.md)

Covers kotowari's commands, the options each command accepts, argument errors, the output when kotowari stops, and the exit codes. How "--help" and "--version" are handled and the wording and details of the reason for a stop are decided in the [Usage display and target environment](cli-environment.md) topic ([The reason for a stop is shown with fixed English wording](cli-environment.md#the-reason-for-a-stop-is-shown-with-fixed-english-wording) and others), and the location of the configuration file and how "--config" is read are decided in [The configuration file is placed at .kotowari/config.yaml in the base directory, and --config can point at another file](config.md#the-configuration-file-is-placed-at-kotowariconfigyaml-in-the-base-directory-and---config-can-point-at-another-file).

## kotowari has seven commands, and the single command check does both the check of the specification and the check of the correspondence with tests

Status: implemented

### Problem

When kotowari was first built, the commands were narrowed to the single command check, and it was decided not to build generation of documents for people, tracing of the scope of impact, or hand-off to the plan and implementation loop.

After that, commands were added only as they became necessary. check already assembles the correspondence between the specification, markers and tests, and outputting it in a readable form is another view of the same data, adding neither new state nor external dependencies. So the read-only commands (list, query, status) were added. If the results of mutation testing were mixed into check, check, which is required to show zero findings before every push, would block the push while pointing at shifted lines of an old results file. Mutation testing takes more than ten minutes and cannot be run every time, so it became a separate command that reads the results, and which results were read is stated explicitly by an argument. Plans became a separate command that takes one file, because check takes no positional arguments and because findings on a plan would affect the completion judgment of status. For the overview, since it takes effort for a person to view the generated HTML, a command that builds it and a command that shows it were made a set. Because they are called from the brainstorm's approval, they were made subcommands of kotowari's single binary, so that no separate tool has to be installed.

### Decision

kotowari has only these seven commands: "kotowari check", "kotowari list", "kotowari mutants", "kotowari plan", "kotowari query", "kotowari status" and "kotowari overview". The single command "kotowari check" does both the check of the specification documents and the check of the correspondence with tests.

"kotowari overview" has two subcommands: "build", which builds static HTML, and "serve", which builds it and then starts a simple server to show it (the behavior is in [Commands that build and show the overview](overview-commands.md)).

### Rejected options

- Mix the results of mutation testing into check, and point at the results file with a configuration key. When check reads an old results file, it blocks the push while pointing at shifted lines.
- Make tracing of the scope of impact (trace) an independent command. It is the same information as what the output of list contains.
- Add the location of plans to check's configuration and check them together with the specification. It would run automatically in pre-commit, but findings on plans would affect the completion judgment of status, and the scope would grow too wide.
- Name the overview command "kotowari render" or "kotowari generate". The reason it was not taken is not recorded.

### Confirmations

- C-core-cli-1 Each of the seven commands ("overview" being "build" and "serve") works as a command, and putting any other word ("mutate") as the first positional argument stops with an argument error

## The options each command accepts are fixed, and options and positional arguments may be written in any order

Status: implemented

### Problem

The primary user is an LLM, so the output form is chosen with "--format". If the options a command accepts are not fixed, a person who adds an option that has no effect misunderstands it as having an effect. "kotowari plan" does not read the configuration, so if it accepted "--config" and silently ignored it, the person who added it would believe it had an effect. "kotowari overview serve" is an entry point for viewing and outputs only a URL, so it does not accept "--format".

The commands that take positional arguments are query, mutants and plan, and having only query be strict about order is hard to remember.

### Decision

The options each command accepts are as in the following table. An option not in the table is not accepted by that command ([An option not accepted, an unknown command, an extra positional argument, or an option with a wrong value stops with an argument error](#an-option-not-accepted-an-unknown-command-an-extra-positional-argument-or-an-option-with-a-wrong-value-stops-with-an-argument-error)).

| Command | Accepted options |
|---|---|
| check | "--format", "--config", "--help", "--version", "--allow-test-findings" |
| list, query, status | "--format", "--config", "--help", "--version" |
| mutants | "--tool", "--format", "--config", "--help", "--version" |
| plan | "--format", "--help", "--version" |
| overview build | "--format", "--config", "--help", "--version" |
| overview serve | "--port", "--config", "--help", "--version" |

The behavior of "--allow-test-findings" is decided in [With --allow-test-findings, check does not count test-side findings toward the exit code](test-side-findings.md#with---allow-test-findings-check-does-not-count-test-side-findings-toward-the-exit-code), and "--tool" in [kotowari mutants always takes the tool's name with --tool, and takes exactly one results file](#kotowari-mutants-always-takes-the-tools-name-with---tool-and-takes-exactly-one-results-file).

In every command, options can be written before or after the command, and the order of positional arguments and options does not matter.

### Confirmations

- C-core-cli-2 The options of check, list, query, status, mutants, plan and overview build can be written before the command, after the command, and after the positional arguments
- C-core-cli-3 check accepts "--allow-test-findings" both before and after the command

## An option not accepted, an unknown command, an extra positional argument, or an option with a wrong value stops with an argument error

Status: implemented

### Problem

kotowari does not silently let through input it cannot read or input that does not meet the contract ([Unreadable input and broken input become a stop or an error finding, and are not silently skipped](cli-environment.md#unreadable-input-and-broken-input-become-a-stop-or-an-error-finding-and-are-not-silently-skipped)). Arguments are the same: if what is not accepted is silently ignored, the person who added it misunderstands it as having an effect. The reason for making each of the argument errors a stop is not recorded anywhere else.

### Decision

When neither "--help" nor "--version" is present (for when one is, see ["--help" and "--version" are taken before the other arguments, and kotowari outputs the usage or the version and ends](cli-environment.md#--help-and---version-are-taken-before-the-other-arguments-and-kotowari-outputs-the-usage-or-the-version-and-ends)), kotowari stops with an argument error as the reason if any of the following applies.

| Argument error | Example |
|---|---|
| There are no arguments at all. There are only options and no first positional argument | "kotowari", "kotowari --format text" |
| The first positional argument is none of the seven commands | "kotowari foo" |
| An unknown option | "kotowari check --verbose" |
| An option the command does not accept (one not in the table of the previous decision) | "--tool" on a command other than "mutants", "--port" on a command other than "overview serve", "--format" on "overview serve", "--allow-test-findings" on a command other than "check", "--config" on "plan" |
| An option that takes a value has no value. The same option appears twice. The value of "--format" is an unknown value | "kotowari check --format", "kotowari check --format xml" |
| What "--config" points at does not exist or is a directory | "kotowari check --config ." |
| There is a positional argument after "check", "list" or "status" | "kotowari check extra" |
| The positional arguments after "overview" are not exactly one of "build" or "serve". The value of "--port" cannot be read as a port number | "kotowari overview", "kotowari overview serve --port x" |

The argument errors of "mutants" are decided in [kotowari mutants always takes the tool's name with --tool, and takes exactly one results file](#kotowari-mutants-always-takes-the-tools-name-with---tool-and-takes-exactly-one-results-file). The positional argument errors of "query" are decided in [If the positional argument is not exactly one ID, or that ID does not exist, it is a stop with an argument error](query.md#if-the-positional-argument-is-not-exactly-one-id-or-that-id-does-not-exist-it-is-a-stop-with-an-argument-error), and the positional argument errors of "plan" in [kotowari plan reads no configuration, and handles arguments and unreadable files in the same way as mutants](plan.md#kotowari-plan-reads-no-configuration-and-handles-arguments-and-unreadable-files-in-the-same-way-as-mutants).

### Confirmations

- C-core-cli-4 Adding an unknown option ("--verbose") stops with an argument error
- C-core-cli-5 If the first positional argument is none of the seven commands, kotowari stops with an argument error
- C-core-cli-6 When there are no arguments at all, and when there are only options and no first positional argument, kotowari stops with an argument error
- C-core-cli-7 Adding an option the command does not accept ("--tool" on "check", "list", "query", "status" or "plan", "--port" on "overview build", "--format" on "overview serve", "--allow-test-findings" on a command other than "check") stops with an argument error
- C-core-cli-8 An option without a value, a second occurrence of the same option, and an unknown value of "--format" stop with an argument error
- C-core-cli-9 If what "--config" points at does not exist or is a directory, kotowari stops with an argument error
- C-core-cli-10 If there is a positional argument after "check", "list" or "status", kotowari stops with an argument error
- C-core-cli-11 When the positional arguments after "overview" are not exactly one of "build" or "serve", and when the value of "--port" cannot be read as a port number, kotowari stops with an argument error

## kotowari mutants always takes the tool's name with --tool, and takes exactly one results file

Status: implemented

### Problem

That which tool's results they are must always be specified with "--tool", its reason, and the rejected options are decided in [Mutation outcomes are held in a form that does not know the tool, and the results file of cargo-mutants is mapped as in the table](mutants-input.md#mutation-outcomes-are-held-in-a-form-that-does-not-know-the-tool-and-the-results-file-of-cargo-mutants-is-mapped-as-in-the-table). This decides the handling when those arguments cannot be accepted.

The path of the results file is read the same way as "--config".

### Decision

"kotowari mutants" always takes, with "--tool", which tool's results they are. The only value accepted now is "cargo-mutants". When "--tool" is absent, or its value is not "cargo-mutants", kotowari stops with an argument error.

The positional arguments after "mutants" take exactly one path of a results file. If there are zero, or two or more, kotowari stops with an argument error. The path is read as a path relative to the current directory. The stop when the results file does not match the form (results error) is decided in [A results file whose form does not fit stops with a results error](mutants-input.md#a-results-file-whose-form-does-not-fit-stops-with-a-results-error).

### Confirmations

- C-core-cli-12 In "kotowari mutants", when "--tool" is absent, and when the value of "--tool" is not "cargo-mutants" ("stryker"), kotowari stops with an argument error
- C-core-cli-13 If there are zero, or two or more, positional arguments after "kotowari mutants", kotowari stops with an argument error

## When kotowari stops, it outputs nothing to standard output and outputs the reason and the details on the first line of standard error

Status: implemented

### Problem

A stop represents not the result of a check but that kotowari could not run. The reason is not recorded.

### Decision

When it stops, kotowari outputs nothing to standard output and outputs the reason for the stop to standard error. The first line of standard error is the wording of the reason for the stop, ": " and the details, joined in this order ("argument error: unknown command: foo").

The wording of the reason is decided in [The reason for a stop is shown with fixed English wording](cli-environment.md#the-reason-for-a-stop-is-shown-with-fixed-english-wording), and the details in [The details of a stop show the file the person fixing it should look at, as a path relative to the base directory](cli-environment.md#the-details-of-a-stop-show-the-file-the-person-fixing-it-should-look-at-as-a-path-relative-to-the-base-directory). In which situation kotowari stops and for which reason is decided in the topic that covers that situation (argument errors in this topic, configuration errors and missing locations in the [Configuration](config.md) topic, unreadable input and input that is not UTF-8 in [Unreadable input and broken input become a stop or an error finding, and are not silently skipped](cli-environment.md#unreadable-input-and-broken-input-become-a-stop-or-an-error-finding-and-are-not-silently-skipped)).

### Confirmations

- C-core-cli-14 When kotowari stops, nothing is output to standard output, and the first line of standard error has the form "reason wording: details"

## The exit code is 0 when there is no error, 1 when there is an error, and 2 when kotowari stops

Status: implemented

### Problem

The side that uses kotowari's result as a gate decides by the exit code whether to pass or block. A notice is a finding that does not change the exit code. The reason for deciding the exit code values this way is not recorded.

### Decision

The exit codes are as in the following table.

| Exit code | Situation |
|---|---|
| 0 | There is no error (including when there are only notices). In check with "--allow-test-findings", all errors are test-side findings. kotowari ended with "--help" or "--version" |
| 1 | There are one or more errors. In check with "--allow-test-findings", there are one or more errors that are not test-side findings |
| 2 | kotowari stopped |

### Confirmations

- C-core-cli-15 If there are one or more errors, kotowari ends with exit code 1, and if there are only notices, with exit code 0
- C-core-cli-16 When kotowari stops, it ends with exit code 2

## Documents for people that copy the structure and content of the specification documents are not generated

Status: implemented

### Problem

Documents for people copied mechanically from the specification lack information, are hard to read, and have to be kept in two places along with the specification. When kotowari was first built, it was decided not to build a command that generates documents for people (render). After that, it was decided to build the overview, but the overview draws overview source data that an LLM wrote and does not copy the specification. The intent of this prohibition is kept for the overview as well.

### Decision

kotowari does not generate documents for people that copy the structure and content of the specification documents ([The specification lives only in decision documents, one per topic, holding the decisions now in force](decision-documents.md#the-specification-lives-only-in-decision-documents-one-per-topic-holding-the-decisions-now-in-force)). The overview draws the overview source data that an LLM wrote ([Overview source data](overview-data.md)), and is not such a document.

### Rejected options

- Remove this prohibition when building the overview. The judgment not to copy the specification mechanically would be removed with it.

### Confirmations

- C-core-cli-17 Putting "render" as the first positional argument stops with an argument error
