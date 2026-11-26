# Languages of decision documents and guides

English | [日本語](languages-and-guides.ja.md)

This document decides which languages decision documents are kept in, and how it is found that a guide for users has become stale. The pairing mechanism itself (agreement records, skeleton comparison) is the existing one; here only how it is applied to decision documents is decided.

## Decision documents can also be kept as multilingual pairs

Status: implemented

### Problem

Until now, the IR was kept as English and Japanese pairs, and the decision records were kept in Japanese only. Since these two become one document, it has to be decided which treatment to follow. If the text the user reads and approves and the authoritative text the machine reads are in different languages, what was approved and the authoritative text can diverge.

### Decision

Decision documents can also be kept as multilingual pairs. The pairing mechanism is used as it is (paired files, an agreement record holding identifiers of the contents of both sides from the last time they were confirmed to say the same thing, skeleton comparison, an error on drift).

The side the machine reads (the first language) is decided by the configuration as before. There is one first language per repository, and the same one is used for decision documents, guides and the overview. No exception that changes the first language per location is made.

kotowari itself keeps English as the first language and has Japanese pairs. For decision documents, the file whose name has no language is English, and the file with `.ja` is Japanese. What the user reads at approval is the Japanese side, and whether it matches the English side is seen by the pair check.

### Rejected options

- Write in Japanese only, without a translation. Writing and fixing take only one, but people who cannot read Japanese cannot read the reasons for decisions. The user judged that it is better to have multilingual support as a mechanism.
- Previous decision: for kotowari itself, only decision documents have Japanese as the first language. This has the advantage that the text the user approved becomes the side the machine reads, but guides have English as the first language, so locations with different first languages would exist within the same repository. The user judged to unify by following the project's preferred language and make no exception.
- Make the first language configurable per location. The configuration and the pairing mechanism would be changed, and exceptions would be held as a mechanism.
- Drop the first language and read every language side equally. The current mechanism would be changed, for example how the side that takes the fingerprint is determined. With the current mechanism as is, the approved text can be made the side the machine reads.

### Confirmations

- C-core-lang-1 When the configuration has two or more languages, check reports an error for a decision document whose paired file does not exist
- C-core-lang-2 If only one side of a decision document is changed and the agreement record is not updated, check reports an error
- C-core-lang-3 Matching confirmation IDs against markers is done only on the first-language side
- C-core-lang-4 If the skeleton of the paired side (headings, state lines, confirmation IDs) differs from the first-language side, check reports an error

## The overview's source data points at decision documents

Status: proposed

### Problem

The overview's (a page that shows the specification with diagrams and text) source data held a list of the IR topic documents it covers and references from inside its parts to requirements and decision records. Since the IR and decision records go away, something for it to point at is needed.

### Decision

The overview's source data points at decision documents. The list of documents it covers is written as paths of decision documents. References from inside parts point at a decision's heading or at a confirmation ID.

Staleness is judged the same way as for guides. If the "Decision" or "Confirmations" of the decision a section points at changes, that section is stale.

### Rejected options

- Discontinue the overview in this version. The idea is that since decision documents have become readable prose, no separate page to show them is kept. Repositories that use the overview could no longer build it.

### Confirmations

- C-core-lang-7 If the list in the overview's source data points at a decision document that does not exist, check reports an error
- C-core-lang-8 If a reference in an overview part points at a decision heading that does not exist or a confirmation ID that does not exist, check reports an error
- C-core-lang-9 When the "Decision" or "Confirmations" of the decision an overview section points at changes, that section is reported as stale

## A guide is stale when the Decision or Confirmations change

Status: implemented

### Problem

A guide for users holds, in markers, which items of the specification it copied, and is asked to be reviewed when those items change. Until now, this was seen through the fingerprint of the requirement body with the source lines removed. Since the form of decisions changes, it has to be decided again which changes make it stale. If a review is asked for when only the state was changed or only a reason was added, work increases even though the behaviour has not changed.

### Decision

Markers on the guide side point at decision headings. If the "Decision" or "Confirmations" of a decision change, that guide section is stale. If the state, the Problem or the Rejected options change, it is not stale.

### Rejected options

- Stale when anything in the decision (except the state) changes. A guide review is asked for even for additions that do not change behaviour.

### Confirmations

- C-core-lang-5 When the "Decision" or "Confirmations" of the decision a guide points at changes, check reports that guide section as stale
- C-core-lang-6 When only the state, Problem or Rejected options of the decision a guide points at change, check does not report that guide section as stale
