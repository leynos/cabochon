# Cabochon — terms of reference

- **Status:** Draft v0.1 with acknowledged open questions
- **Audience:** Product owners, engineering leads, contributors, and prospective
  application developers
- **Companion documents:** `docs/context.md`, `docs/cabochon-design.md`,
  `docs/roadmap.md`, the ADRs indexed in `docs/contents.md`, and
  `references/cabochon-white-paper.md`
- **Last substantive revision:** 2026-07-23
- **Version:** 0.1

## 1. Background and motivation

Cabochon addresses a gap between comprehensive Linux desktop ecosystems and
minimal Wayland compositors. Existing desktop components provide capable
display, media, printing, text, and sandboxing primitives, but do not by
themselves provide a compact, coherent environment for document-centred work or
for creating native applications that participate in that environment.

Requiring adoption of an entire desktop environment before either audience can
experience that value would impose a substantial switching cost. Cabochon's
document workspace and developer environment should therefore be useful as
applications within established desktops such as GNOME and KDE Plasma. A full
Cabochon desktop can offer deeper integration after users and developers have
reason to make that larger commitment.

The existing white paper describes both the problem and a candidate solution.
This document treats its problem claims as prior art. Its component boundaries,
technology choices, and implementation sequence are evaluated by the companion
design document, ADRs, and roadmap rather than accepted wholesale.

## 2. Domain

Cabochon operates in the domain of native Linux desktop environments and
application development, with particular attention to document-oriented work,
printing, and cooperation between user-facing tools. The project draws on the
compact architectural separation associated with Graphics Environment Manager
(GEM) and the modular, document-oriented workflows explored by Étoilé.

Étoilé's LanguageKit sought to let higher-level languages cooperate through a
shared object model and explicitly rejected an artificial divide between users
and programmers. NeXT and Apple developer tools provide a complementary
precedent: standard components, project templates, visual construction, and
interactive previews can produce an early visible result while leaving custom
behaviour and lower-level work available as developers progress.

Cabochon's developer environment should offer more than one entry path over a
shared dynamic object system. Rust remains available to developers who need or
prefer it. A more accessible path may use a dynamic language focused on
trait-based message passing. The white paper's “Objective Rust” concept may
provide an intermediate layer. These are candidate design directions, not
settled language or runtime choices.

Outside a full Cabochon desktop session, Cabochon behaves as a runtime and
application environment hosted by another desktop. The runtime allows Cabochon
applications to retain their shared document-centred and tool-aware behaviour
without requiring Cabochon to control the desktop shell.

*Tool-aware* means that an application presents relevant capabilities where a
user is working instead of hiding them in a global services catalogue. Tools
operate on semantic documents, entities, and selections rather than belonging
exclusively to one application or file format. For example, a user can insert a
live formula into a prose document using the same formula capability available
in a datagrid. The formula can address a source represented by a document
entity, an element in Scalable Vector Graphics (SVG), or a datagrid cell.

The white paper proposes Enfilade as the mechanism that maps those addressable
entities. The design document retains it as the proposed object graph; the
terms-of-reference requirement is the cross-document behaviour visible to the
user.

The [shared domain vocabulary](context.md) defines these terms independently of
the remaining design choices. The initial accessible authoring path and the
detailed transition rules for live relationships remain unresolved; the terms
used to discuss those choices are settled.

## 3. Market context

Cabochon occupies a landscape containing comprehensive desktops such as GNOME,
KDE Plasma, and COSMIC; minimal Wayland compositors; native toolkits such as
GTK and Qt; and web-based desktop frameworks such as Electron and Tauri. Its
provisional point of distinction is the combination of a compact desktop,
document and print literacy, and typed cooperation between applications.

GNOME and KDE Plasma are not only competitors. They are also potential host
environments for an incremental adoption path. Cabochon must establish value as
a good desktop citizen before asking a user to replace their existing
environment.

In a host desktop, the runtime is the enabling product surface and the
applications are the user-visible value. Cabochon's case for adoption cannot
rest on compatibility alone: its applications must make document-centred work
and cooperation with relevant tools materially useful.

Evidence that this combination addresses sufficient unmet demand remains to be
established.

Obsidian is the closest named adjacent product for the initial knowledge
workspace. Its plugin model lets developers extend the note-taking experience
with custom features. Cabochon does not initially need to displace an existing
Obsidian installation. The knowledge workspace instead tests whether a
document-centred environment can reach comparable richness while allowing
semantic content and tools to remain available when the user moves to another
Cabochon application.

## 4. Users and stakeholders

Cabochon has two interdependent primary audiences rather than one audience that
takes precedence over the other.

| Audience                           | Context                                                                             | Immediate value sought                                                                                                                     | Current alternative                                                                                                     |
| ---------------------------------- | ----------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------- |
| Desktop users                      | People using an established Linux desktop who may later consider a Cabochon session | A personal knowledge workspace that justifies trying Cabochon without first replacing their desktop                                        | A mature desktop and separate note, data, diagram, and image applications                                               |
| Exploratory application developers | Creative developers who enjoy exploring systems and methods of interactivity        | An environment that handles routine desktop obligations while exposing enough composable behaviour to support experiments with interaction | Established native toolkits, cross-platform frameworks, creative coding environments, or lower-level graphics libraries |

*Table 1: Cabochon's interdependent primary audiences.*

The initial applications serve two purposes: they give desktop users enough
utility to begin using Cabochon, and they provide developers with concrete,
inspectable examples of native application development. The intended precedent
is the welcoming utility of Windows 3.x Accessories combined with the creative
developer proposition associated with NeXT and Mac OS X. This precedent does
not by itself settle which applications belong in the initial set.

The initial set is a personal knowledge workspace centred on a tree of
interconnected notes. Notes can embed dataframes, Mermaid logical diagrams, and
bitmap images. Focused editors for those embedded document types provide the
initial data, diagramming, and painting capabilities. This arrangement makes
the starter applications cooperating examples of Cabochon's document model
rather than an arbitrary utility bundle.

When GNOME or KDE Plasma hosts Cabochon, these applications use the Cabochon
runtime rather than pretending to be a complete desktop environment. They
remain recognizable applications within the host while retaining shared
document semantics and awareness of tools relevant to the active document or
selection.

Contributors are stakeholders because the breadth and quality of the
application ecosystem depend on their participation. Secondary users,
commercial stakeholders, and further secondary users remain unresolved.

Cabochon is not initially for absolute programming novices. Its progressive
developer experience should reduce platform-specific friction, but it is not a
course in basic programming or software construction. The baseline developer
understands the semantics of a general-purpose language such as Python or
JavaScript and is familiar with object concepts. Rust, systems programming,
graphics, and desktop-framework expertise are not entry requirements.

Cabochon is also not initially for organizations seeking an enterprise desktop
environment. Initial work must benefit an individual user directly. Features
whose value exists only at organizational scale belong to later product phases,
if they belong at all.

## 5. Job to be done

The precise jobs remain under elicitation. The current working formulations are:

**Desktop-user job:**

> When collecting and developing personal knowledge, a desktop user wants to
> organize interconnected notes containing prose, structured data, logical
> diagrams, and bitmap images, so that related material remains in one coherent
> workspace rather than being divided between application silos.

During that work, the user wants relevant tools at the active document or
selection and wants their semantics to carry across document types. This lets
the user express relationships such as a live formula without first translating
the work into the native representation of a separate application.

The user outcome is a body of knowledge whose rich semantics survive movement
between applications. A structured embed such as a dataframe remains usable as
the same meaningful object when another Cabochon application presents or edits
it; changing applications does not require flattening the embed, exporting it,
or abandoning its tools.

The personal knowledge workspace is the accepted initial proving ground. Its
frequency of use and representative evaluation workflows remain unresolved.

**Application-developer job:**

> When exploring systems and methods of interactivity, a creative developer
> wants to exercise Cabochon's composable document and tool affordances from
> their existing desktop while routine desktop obligations are handled
> consistently, so that they can test and realize distinctive interactive
> ideas before committing to the full environment.

This audience is defined by its exploratory motivation, not yet by employment,
sector, or seniority. Cabochon should not require prior mastery of its platform
before a developer can produce a rewarding result. The experience should make
the next layer of capability visible and provide a smooth path towards harder,
more specialized work. The first interaction must not require Rust knowledge,
but Rust expertise should not be penalized by an artificially restricted
authoring surface. The exact authoring path remains unresolved.

The accessible entry path may assume familiarity with the semantics of a
language such as Python or JavaScript and with object concepts. It must not
assume Rust, systems programming, graphics, or desktop-framework knowledge.

The canonical onboarding artefact is a currency object, following the spirit of
the early Mac OS X currency-converter example. The object has an identity in a
base currency, can be presented in multiple currencies, and can be inserted
into a document. This small exercise demonstrates the separation of semantic
identity from presentation, multiple views over one object, and participation
in the document environment. It is a teaching example, not a commitment to
financial or accounting functionality.

## 6. Scope

### 6.1. Goals

- Let users evaluate the personal knowledge workspace without replacing their
  existing desktop environment.
- Let developers evaluate and use Cabochon's development environment from an
  established desktop.
- Make the full Cabochon desktop a progressive integration step rather than a
  prerequisite for first value.
- Preserve document-centred and tool-aware application behaviour when another
  desktop environment hosts the Cabochon runtime.
- Present relevant tools at the user's active document or selection rather
  than requiring discovery through a global services menu.
- Allow one tool's semantics to address compatible entities across different
  document types.
- Give exploratory developers an immediately rewarding first interaction that
  does not require prior Cabochon platform knowledge.
- Make progressively deeper capabilities discoverable so that early ease does
  not impose a low ceiling on later work.
- Let a developer's first exercise define a semantic object, give it multiple
  presentations, and insert it into a document.
- Support multiple developer entry paths, ranging from an accessible dynamic
  object language to direct Rust development, over compatible object semantics.
- Use the personal knowledge workspace as a proving ground for rich documents
  whose semantic embeds and relevant tools survive transitions between Cabochon
  applications.
- Bound the initial proving ground to interconnected notes, dataframes,
  Mermaid diagrams, and bitmap images with focused editing capabilities.

Further goals remain under elicitation.

### 6.2. Non-goals

- Teaching foundational programming concepts to people with no prior software
  development experience is out of scope. Cabochon's accessible entry path is
  for developers who can already reason about programs and interactive
  behaviour.
- Organization-only capabilities are out of scope for the initial product.
  This includes centralized organizational policy, fleet administration, and
  compliance controls that do not also benefit an individual user. Such work
  may be reconsidered only after the individual product is established.

Further non-goals remain under elicitation.

Additional document and editor types are outside the initial proving-ground
boundary unless they are required to validate the agreed note, dataframe,
diagram, or bitmap workflows.

## 7. Success criteria

An exploratory developer's first successful Cabochon interaction produces a
currency object with a base-currency identity, presents it in at least one
other currency, and inserts it into a document. The exercise must make the path
from that result to more complex object behaviour visible for a developer who
understands Python- or JavaScript-like language semantics and object concepts.
The acceptable time, number of steps, and degree of authored code remain
unresolved.

Further success criteria remain under elicitation.

The knowledge-workspace hypothesis succeeds when a document containing a rich
embed, such as a dataframe, can move between relevant Cabochon applications
without losing its identity, semantics, editability, or applicable tools.
Matching or exceeding the practical richness of an established Obsidian setup
is the comparative goal; the representative workflows and evaluation method
remain unresolved.

## 8. Constraints and assumptions

Cabochon must coexist with established Linux desktop environments during the
adoption path. When Cabochon does not provide the desktop session, it must
operate as a runtime for its applications without assuming control of the host
shell. The degree of integration required for that coexistence remains
unresolved.

Scope decisions use direct benefit to an individual user as a gate. A proposed
initial capability whose answer to “does this benefit an individual user?” is
no must be deferred. Organizational demand alone does not override this gate.

The project assumes that multiple authoring paths can share one coherent object
model without reducing Rust to a second-class escape hatch or exposing
beginners to the full implementation substrate. If this assumption fails,
Cabochon must either narrow its developer audience or accept incompatible
programming models. Constraints, assumptions, and dependencies otherwise remain
under elicitation.

## 9. Open questions

| Question                                                                                   | Why it matters                                                                                           | Resolution criterion                                                                                                    | Owner         | Suggested path                      |
| ------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------- | ------------- | ----------------------------------- |
| Which refresh triggers, retry policy, and source-move rules should live relationships use? | Refines the accepted visible safety states without weakening them                                        | Validate every transition against ADR 005 and the formula slice                                                         | Product owner | State-machine spike                 |
| Which authoring path should the first currency-object exercise use?                        | Determines whether the easiest entry path is Rust, Objective Rust, a dynamic language, or something else | Prototype the candidate paths and compare first-result effort, conceptual continuity, and access to deeper capabilities | Product owner | Design ADR and implementation spike |
| What measurable signals demonstrate value for each audience?                               | Makes product and developer-experience goals falsifiable                                                 | Define user-facing, operational, and strategic thresholds                                                               | Product owner | Resolve after goals                 |

*Table 2: Open questions during elicitation.*

## References

- Cabochon white paper, `references/cabochon-white-paper.md`.
- [Wayland architecture](https://wayland.freedesktop.org/architecture.html),
  accessed 9 July 2026.
- [Étoilé overview](https://etoileos.com/etoile/), accessed 9 July 2026.
- [Étoilé LanguageKit](https://etoileos.com/etoile/features/languagekit/),
  accessed 10 July 2026.
- [CUPS programming manual](https://openprinting.github.io/cups/doc/cupspm.html),
  accessed 9 July 2026.
- [XDG Desktop Portal
  documentation](https://flatpak.github.io/xdg-desktop-portal/docs/), accessed
  9 July 2026.
- [COSMIC toolkit documentation](https://pop-os.github.io/libcosmic/cosmic/),
  accessed 9 July 2026.
- [GNUstep introduction](https://www.gnustep.org/information/aboutGNUstep.html),
  accessed 9 July 2026.
- [Apple interface
  fundamentals](https://developer.apple.com/documentation/technologyoverviews/interface-fundamentals),
  accessed 10 July 2026.
- [Creating an interface with SwiftUI in
  Xcode](https://developer.apple.com/documentation/xcode/creating-your-app-s-interface-with-swiftui),
  accessed 10 July 2026.
- [Building an Obsidian
  plugin](https://docs.obsidian.md/Plugins/Getting+started/Build+a+plugin),
  accessed 10 July 2026.

## Shared vocabulary

The normative definitions for domain terms used here are maintained in the
[Cabochon context](context.md). This document defines product scope and uses
that vocabulary rather than maintaining a second glossary.
