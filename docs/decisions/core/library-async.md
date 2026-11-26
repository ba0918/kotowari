# Optional Tokio support

English | [日本語](library-async.ja.md)

This document covers the entry points for awaiting the high-level operations of kotowari and kotowari-markdown-schema-io from Tokio, the number of concurrent executions, and the behaviour when the caller stops waiting. The execution slots, the cancellation of waiting, the relation to the runtime and the failure when there is no runtime, decided here, are common to kotowari's `AsyncProject` and kotowari-markdown-schema-io's `AsyncSchemaLoader`. That `AsyncSchemaLoader` returns the same results as synchronous loading is decided in the topic "The library that loads schemas and documents", in [A call that selects the Tokio feature also gets the same results as the synchronous one](../schema/library-io.md#a-call-that-selects-the-tokio-feature-also-gets-the-same-results-as-the-synchronous-one). Which packages do not depend on Tokio is decided in the topic "Crate boundaries of the library", in [The computation crates do not operate on the environment and have no Tokio or CLI dependencies](library-crates.md#the-computation-crates-do-not-operate-on-the-environment-and-have-no-tokio-or-cli-dependencies).

## The high-level operations can be awaited from Tokio through a tokio feature disabled by default

Status: implemented

### Problem

We want kotowari to be usable from asynchronous applications too. In doing so, we want to reuse the synchronous checking and the existing loading.

The packages users choose for asynchronous use are not increased, and users who use kotowari synchronously are not required to depend on Tokio.

### Decision

The synchronous API is the basis. kotowari provides `AsyncProject` behind the `tokio` feature, disabled by default, so that the high-level operations of the synchronous `Project` can be awaited. `AsyncProject` runs the synchronous processing by handing it to `spawn_blocking` of the caller's Tokio runtime, and holds no checking of its own. For the same input, it returns the same values, findings and failure kinds as the synchronous `Project`.

The kotowari command (CLI) uses the synchronous API.

### Rejected options

- Provide only the synchronous API and an example of calling it from an asynchronous application. The user chose the option of going as far as Tokio support in which the high-level operations can be awaited.

### Confirmations

- C-core-libasync-1 With the `tokio` feature enabled, the operations of `AsyncProject` return, on Tokio, the same values, findings and failure kinds as the synchronous `Project`
- C-core-libasync-2 (by reading) The `tokio` feature is disabled by default, `AsyncProject` runs the synchronous processing by handing it to `spawn_blocking`, and the CLI uses the synchronous API

## The number of concurrent executions is limited per asynchronous entry-point object, and clones share execution slots

Status: implemented

### Problem

We want to avoid the threads that run processing growing without bound when many asynchronous calls come in. We want to make explicit over what range the number is limited, and let users adjust it to their execution environment. It must not be possible to bypass the intended limit just by cloning an object.

### Decision

`AsyncProject` and `AsyncSchemaLoader` limit the number of processes running at the same time per object. The limit is passed in `AsyncOptions`. The default limit is 1; to change it, a positive integer is passed as `NonZeroUsize`.

Processing starts only after obtaining one execution slot. Objects created separately each have their own execution slots. Cloned objects share the same execution slots with each other.

### Confirmations

- C-core-libasync-3 The default limit of `AsyncOptions` is 1, and in an object whose limit is 2, two processes run at the same time and a third is not submitted until an execution slot frees up
- C-core-libasync-4 Objects created separately do not share execution slots, and cloned objects share execution slots with each other

## Stopping waiting does not stop processing that has started, and it holds its execution slot until it finishes

Status: implemented

### Problem

Processing handed to `spawn_blocking` cannot be stopped by abort once it has started. The caller stopping waiting and the processing itself stopping must be distinguished.

Also, repeated cancellation of waiting must not make the number actually running at the same time exceed the limit.

### Decision

Processing whose waiting is stopped while it waits for an execution slot is not submitted. Submitted processing keeps holding its execution slot until it finishes, even if the caller stops waiting.

Stopping waiting does not stop HTTP fetches, computation or cache writes that have started. Stopping them is not guaranteed, and this is shown to users as a promise of the public API.

### Rejected options

- Rebuild the waiting for HTTP fetches with asynchronous I/O, and also add a mechanism that stops started processing partway. The user chose the option of reusing the synchronous processing so that it can be called from asynchronous applications.

### Confirmations

- C-core-libasync-5 Processing whose waiting is stopped while it waits for an execution slot is not submitted even when an execution slot frees up later
- C-core-libasync-6 Even if waiting on submitted processing is stopped, the execution slot does not free up until that processing finishes, and other processing requested from a cloned object is not submitted
- C-core-libasync-7 (by reading) The guide for users says that stopping waiting does not stop HTTP fetches, computation or cache writes that have started

## Awaiting outside a runtime, or a runtime that stops, returns a failure whose kind can be told apart, without panicking

Status: implemented

### Problem

The asynchronous entry points hand processing to the caller's Tokio runtime. If they panicked when awaited outside a runtime, or when the runtime stopped during the wait, the caller's whole application would go down. The caller must be able to handle this failure apart from check results and other failures. The reason is not recorded.

### Decision

`AsyncProject` and `AsyncSchemaLoader` run on the caller's Tokio runtime and do not create an internal runtime.

Awaiting an operation outside a Tokio runtime does not panic, and returns a failure of the `RuntimeUnavailable` kind. When the processing ends without being able to run (when the runtime stops during the wait), it does not panic, and returns a failure of the `TaskFailure` kind.

### Confirmations

- C-core-libasync-8 Awaiting an operation of `AsyncProject` or `AsyncSchemaLoader` outside a Tokio runtime does not panic, and returns a failure of the `RuntimeUnavailable` kind
- C-core-libasync-9 When the runtime stops while there is processing waiting for an execution slot, the waiting processing does not panic, and returns a failure of the `TaskFailure` kind
