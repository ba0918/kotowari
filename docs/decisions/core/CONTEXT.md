# Glossary

English | [日本語](CONTEXT.ja.md)

| Term | Meaning |
|---|---|
| base directory | The first directory containing ".kotowari/" found by searching upward from the current directory. If there is none, the current directory |
| configuration file | The YAML file that states where to read and the values for the checks. The default is ".kotowari/config.yaml" |
| place of the decision documents | The directory the configuration's "decisions.documents" names ("docs/decisions" by default). Every ".md" file under it is read as a topic document, except the other-language sides of a pair and the glossary of a group |
| decision document | A Markdown file under the place of the decision documents that holds the decisions of one topic now in force. Also called a topic document. It holds only what a person decided; details the implementing side decided are in the code, the tests and the commit and pull request bodies |
| topic | The subject one decision document covers. One topic is one file |
| group | A folder directly under the place of the decision documents, one per product or crate (core, schema, view). The group name is part of the confirmation IDs of the documents in it |
| glossary | The "CONTEXT.md" directly in a group folder. A document for people: check does not read it as a topic document and checks nothing inside it |
| title | The first line of a document that starts with "# " |
| lead | The paragraphs between the title (and the switcher line) and the first decision, saying what the topic covers |
| decision | One "## " heading of a topic document other than the open section, with what follows up to the next "## " heading. The heading is one sentence that states the decision. It has a state line, "Problem" and "Decision", and may have "Rejected options" and "Confirmations" |
| state | The first paragraph under a decision's heading: "proposed" (decided, not built yet) or "implemented" (built; the behaviour of the product now). The implementing side turns a decision to implemented when every confirmation row of it holds |
| confirmation row | One list item under a decision's "Confirmations": one result that can be seen and shows the decision is kept. Its first word is its confirmation ID |
| confirmation ID | The identifier of a confirmation row, "C-<name>-<number>": the name built from the group and the topic, the number from 1 without a leading zero, never reused |
| by reading | A confirmation row with "(by reading)" right after its ID. A test cannot confirm it; the implementing side and the review read it and confirm it, and check asks for no test for it |
| open section | The "## Open" section at the end of a topic document. Each list item is something not decided yet or a question handed back to a person, with who decides it and when. It is not a decision |
| complete | The value of "kotowari status" that is true only when check has no error and no topic document has an item in its open section |
| decision table | A Markdown table of condition and result pairs written inside a decision's "Decision" |
| property | Something that holds for every input, written as a confirmation row and pointed at by a property test |
| frozen tree | "docs/frozen/": the documents of the old form from before the switch to decision documents. check does not read it, it is not grounds for the current rules, and a link to it from a decision document is an error |
| plan | A Markdown document of implementation steps. "kotowari plan" checks its form against the schema bundled in the binary |
| finding | One thing found by a check. Has kind, severity, path, line and detail |
| error | A finding whose severity is "error". It makes the exit code 1, except that "kotowari check" with "--allow-test-findings" does not count test-side findings toward the exit code |
| notice | A finding that does not change the exit code. Its severity is "notice" |
| test-side finding | An error caused by the tests not being in place yet: test_without_id and confirmation_without_test, invalid_marker and unresolved_reference whose path is a test file, and unparsable_file whose path is a test file and not a surface file |
| consistency phase | A dedicated step inside the cycle in which an LLM in a separate context finds and resolves the disagreements between the code a change implemented or altered and the decision documents, and the conflicts between the decisions it touches. Not called conformance or change conformance |
| stop | Ending with exit code 2 without being able to perform the check |
| marker | The "@kotowari[ID, ...]" sequence written in a test, pointing at confirmation IDs |
| guide | A file matched by the configuration's "guides.files". Its readers are the people who use the product, and it is meant to hold usage documents for users; it is not a place for decision documents, implementation plans or skills, but kotowari does not tell them apart. Not called just "document" |
| guide marker | The "@kotowari[<path>#<anchor>:fingerprint, ...]" sequence written inside an HTML comment of a guide or of overview data, pointing at decisions. Read by rules separate from those of markers in tests |
| fingerprint | A value of eight lowercase hexadecimal characters, taken from a decision's "Decision" and "Confirmations" |
| test | A syntax-tree node that a query matched. A function in Rust and Python, a call such as "it(...)" in TS/JS |
| test file | A file matched by the globs of the configuration's "tests.files" |
| query | A per-language ast-grep rule that finds tests. Some are bundled with kotowari, and some are added through the configuration's "tests.rules" |
| language with a query | A language, determined by the extension, that has one or more bundled or configured queries |
| language without a query | A language that is not a language with a query, including files whose language the extension does not determine |
| preceding comment block | The block of comment-only lines that goes upward from just above the first line of a test node until a blank line comes (not broken by whitespace-only lines inside a multi-line comment, an attribute or a decorator). A comment is a tree-sitter extra node |
| vague word | A word listed in the configuration's "vague_words" |
| exclusion | What the decision documents enumerate as things kotowari neither reads nor looks at, without raising a finding. Nothing outside that enumeration is skipped |
| code block | The part from a line starting with three or more "`" or "~" at the line head up to a line of the same character, at least as many |
| mutation | One small change at one place that a mutation testing tool puts into the implementation |
| mutation outcome | For one mutation, the file, the line, the description of the change (the tool's text as is) and the outcome (one of four values: caught, missed, timed out, unviable). Copied from the tool's outcome file |
| miss | A mutation outcome whose outcome is "missed": all the tests passed even with that mutation in. Not called a "survivor", a "surviving mutant" or a "live mutant" |
| equivalence | A person's or an LLM's judgement that a mutation does not change the behaviour users can observe (the output and the exit code). Differences in a function's return value or internal state do not count as observations |
| list of equivalents | The YAML file the configuration's "mutants.equivalents" points at. Lists the mutations judged equivalent, with their reasons |
| legacy material | Documents that existed before kotowari was adopted and write down a specification or decisions: old specifications, existing decision records and ADRs, and the parts of a README that explain behaviour. Code comments and commit logs are not included, nor are the decisions of a decision document the topic already has. Not called "existing material" or "old documents" |
| ratification | In an adoption, the person leaving a candidate row in the list, so that the behaviour the legacy material and the implementation agree on is written as an implemented decision without being reviewed item by item |
| surface | One piece of user-visible behaviour taken from the code by a surface rule. One syntax-tree node matched by a surface rule is one surface; it has the rule's id as its kind and the string captured by "$NAME" as its name. Not called "API", "interface" or "endpoint" |
| surface rule | An ast-grep rule written in the files listed in the configuration's "surface.rules". Takes out surfaces. kotowari bundles none. Separate from the queries that find tests, and not called a "query" |
| surface file | A file matched by the globs of the configuration's "surface.files". Surface rules are applied to it |
| list of unspecified surfaces | The YAML file the configuration's "surface.unspecified" points at. Lists the surfaces not yet written into a decision, one entry each with kind, name and reason; a listed surface is not an error even when no decision quotes it. Not called an "allowlist" or an "exclusion list" |
| overview | A rendered page for people, one per product topic. Its readers are the people who build the product (the person who brainstormed, the implementer, the reviewer), and it shows the current state and the plans. Not called a "summary" or a "synopsis" |
| overview data | A Markdown file of a fixed form that an LLM writes to draw an overview. Matched by the configuration's "overview.files"; its "documents" lists the decision documents it covers. People do not read it. Not called a "source" or an "intermediate file" |
| part | A unit of drawing of a fixed kind inside overview data. Written in YAML inside a fence whose info string is "view <kind>". Not called a "block" or a "component" |
| reference table | A mapping from the reference strings inside a part (a decision or a confirmation ID) to display name, body and status. kotowari builds it by reading the decision documents, and passes it to the rendering engine. Not called a "link table" |
| table of contents | A YAML file declaring how the listing page bundles each overview and in which order. The configuration's "overview.toc" points at it, and its outermost level is written as one contents group. Not called a "map" or an "index" |
| contents group | A bundle inside the table of contents. Has "title" for its title, "note" for an optional one-line description, and "items", which lists in order names of an overview and nested contents groups. Not called a "category" or a "folder" |
| language list | The sequence of language tags listed in the configuration's "languages". When "languages" is absent or an empty list, the sequence of only "en" |
| first language | The first language of the language list. Its side has a name without a language tag, and it is the side check reads decisions, confirmation IDs and fingerprints from |
| pair | The set of files that write one document in each language of the language list. Exists only when the language list has two or more languages. Not called a "translation" or a "language variant" |
| side | The file of one language within a pair. The side of the first language is named "<stem><extension>", and the side of any other language "<stem>.<tag><extension>" |
| agreement record | The "<stem>.i18n.yaml" placed in the same directory as the side of the first language of a pair. Holds the blob hash of each side as of the last time the sides were confirmed to say the same thing |
| skeleton | The parts of a side of a pair that are not natural-language sentences. They must match in every side |
| switcher line | The line placed after the title of each side of a decision document and of a guide, which lists the name of each language and links to the side of every other language |
| UI text | Text that kotowari writes into an overview and a switcher line and that is in neither the overview data nor the decision documents. kotowari holds the English text; other languages are written in the configuration's "labels" |
