# Cabochon context

This document defines Cabochon's shared domain vocabulary. The terms of
reference and design document are authoritative for product scope and technical
behaviour respectively.

## Terms

| Term                 | Definition                                                                                                                                                                                       |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Cabochon application | An application that uses the Cabochon runtime and participates in its document, tool, capability, and transaction contracts.                                                                     |
| Cabochon runtime     | The user-session services that host Cabochon objects and tools when another desktop environment owns the shell.                                                                                  |
| Capability           | An explicit, inspectable grant permitting an object or tool to perform an operation on a resource.                                                                                               |
| Document             | A durable semantic object that can contain or reference other objects and can have more than one presentation.                                                                                   |
| Embed                | A document-owned presentation of another semantic object. An embed does not flatten or copy away the object's identity.                                                                          |
| Enfilade             | The proposed object graph that resolves object identity, semantic links, selectors, capabilities, and live relationships.                                                                        |
| Live relationship    | A persisted dependency between objects whose derived presentation can change when its source changes.                                                                                            |
| Object               | A semantic entity whose stable identity is independent of its presentation, containing application, and provider location.                                                                       |
| Objective Rust       | A candidate typed object layer that exposes declared selectors and message-passing semantics over Rust implementations. It is not yet an accepted language or application programming interface. |
| Presentation         | A view of an object's state for a context, intent, or format. Presentation does not define object identity.                                                                                      |
| Project              | A user-owned collection of documents, objects, links, tools, and views that form one working context.                                                                                            |
| Selection            | The active object or addressable part of an object against which tool applicability is evaluated. A selection carries no authority by itself.                                                    |
| Selector             | A declared operation that an object may expose through the runtime. Selectors are discoverable, typed at their provider boundary, and capability checked when invoked.                           |
| Service              | Historical term for a reusable operation offered to other objects or applications. Cabochon uses *tool* in user-facing prose.                                                                    |
| Tool                 | A discoverable operation or grouping of selectors for compatible objects or selections. Discovery describes applicability but never grants the capabilities required to invoke it.               |
| Tool-aware           | Able to discover and present relevant tools from the semantic type and capabilities of the active object or selection.                                                                           |
| Transaction          | An auditable group of mutations that commits atomically or leaves the affected objects unchanged.                                                                                                |

*Table 1: Cabochon domain vocabulary.*
