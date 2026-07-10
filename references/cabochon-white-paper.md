# Cabochon: a compact, document-centred desktop for Wayland

## Abstract

This paper proposes **Cabochon**, a compact Wayland desktop environment
inspired by two abandoned but still fertile ideas: Digital Research GEM's clean
split between desktop services and device-independent graphics, and Étoilé's
ambition for a lightweight, modular, document-oriented GNUstep environment
built around recomposable user workflows.

The core claim is simple: the modern Linux desktop has powerful primitives, but
weak narrative coherence. Wayland gives us a lean display protocol. PipeWire
gives us graph-based audio/video plumbing. CUPS gives us IPP-based printing.
HarfBuzz gives us world-class text shaping. XDG portals give sandboxed
applications a controlled way to interact with the host. These parts work, but
desktop application development still often feels like assembling a radio from
cutlery during a thunderstorm.

Cabochon supplies the missing compact spine: a consistent application
framework, a vector-first rendering model, a printer/PDF-aware document
surface, and a desktop shell whose behaviour fits in one person's head. Its
live object layer then adds the stranger promise: documents, selections, tools,
queries, services, and projects can discover and recombine one another without
reducing the user to shell-script archaeology. The object layer uses what this
paper calls **Objective Rust** — Objective-C's message-passing spirit, in which
a caller asks whether an object responds to a selector rather than which class
it belongs to, but with every selector statically declared by its provider and
every invocation crossing a typed, capability-checked boundary.

The ambition is not nostalgia. This is not "Atari ST, but with rounded
corners". It is a serious proposal for a small, fast, coherent desktop
environment where native applications are pleasant to write, documents are
first-class citizens, and automation works through typed objects rather than
shell-script archaeology.

## 1. Problem statement

Modern desktop environments have become extremely capable, but they rarely feel
small, inspectable, or conceptually unified. GNOME and KDE solve vast numbers
of real problems, yet both carry the mass of long-lived platform ecosystems.
Minimal Wayland compositors solve the opposite problem: they give experts
elegant window management, but usually not a full application story, not a
unified document model, and not a friendly developer substrate for native
software.

Meanwhile, native desktop applications face an awkward decision tree. Use GTK
or Qt and inherit a large framework with its own philosophy. Use Electron or
Tauri and turn the desktop into a domesticated browser habitat. Use SDL, wgpu,
or custom rendering and then rebuild menus, file pickers, accessibility,
printing, text input, drag-and-drop, settings, and internationalization one
teaspoon at a time.

The historical GEM instinct remains useful: split the system into a **desktop
interaction layer** and a **device-independent graphics layer**. GEM's original
AES/VDI separation was beautifully direct. AES handled windows, menus, widgets,
dialogs, and events. VDI handled drawing, including graphics primitives and
output devices. Cabochon preserves that conceptual clarity through
`cabochon-clerestory` and `cabochon-lapidary`, while replacing the old
immediate-mode framebuffer assumptions with Wayland buffers, GPU vector
rendering, PDF output, colour management, accessibility semantics, and
sandbox-aware services.

Étoilé adds the wilder half of the thesis. Its stated goal was a user
environment organized around what people do, namely create, collaborate, and
learn, avoiding a UI polluted by low-level implementation details such as files
and operating-system processes. Its overview also described an innovative
GNUstep-based environment built from modular, lightweight components, with
project and document orientation, allowing users to reshape and recombine
services and components. ([etoileos.com][2])

Cabochon carries that idea through `cabochon-enfilade`, a live object graph
where documents, selections, services, and project artefacts can be linked,
inspected, transformed, and recombined. Cabochon is therefore both a desktop
and a wager: that a small system can still be ambitious if it puts its seams in
the right places.

## 2. Design principles

**Small enough to understand.** The system should prefer comprehensible
subsystems over majestic entanglement. A capable developer should be able to
understand the compositor, toolkit, and document rendering pipeline without
needing a sabbatical and a ceremonial hat.

**Document-centred, not app-centred.** Windows should be views onto documents,
projects, devices, conversations, queries, or live objects. Applications remain
useful packaging units, but the user's conceptual world should not reduce to
"which app owns this file?"

**Vector-first, PDF-aware, print-literate.** Page geometry, text shaping,
colour, images, and export should use one rendering model from the beginning.
Printing must not arrive as a late-stage goblin. CUPS is based on IPP and lets
clients communicate with a scheduler and printers to list destinations and
submit jobs; Cabochon should treat that as a core desktop concern, not a dusty
side corridor. ([openprinting.github.io][3])

**Wayland-native, not Wayland-naïve.** Wayland does not provide a drawing API.
It defines how clients and compositors communicate; clients render into buffers
and hand those buffers to the compositor. Cabochon must therefore provide its
own application graphics model above Wayland, while using Wayland as the
presentation and input protocol. ([wayland.freedesktop.org][1])

**Typed dynamism.** The object layer should allow late-bound composition,
service discovery, scripting, and live object inspection. But it should not
dissolve into stringly-typed fog. Dynamic dispatch must cross typed,
capability-checked boundaries.

**Native before nostalgic.** The desktop may carry GEM's architectural ghost,
but not its limitations. It must support HiDPI, fractional scaling,
accessibility, modern input methods, colour-managed output, international text,
screen capture, tablets, remoting, and sandboxed applications. Wayland has
protocols for desktop surfaces through xdg-shell, fractional scaling through
`fractional-scale-v1`, and colour metadata through colour-management
extensions; Cabochon should participate in these rather than inventing private
islands. ([wayland.app][4])

## 3. The two wolves

Inside Cabochon live two wolves.

The first wolf is **GEM**: disciplined, compact, suspicious of waste, fond of
clean architectural seams. It gives Cabochon its bones — compositor, toolkit,
scene model, renderer, resources, portal integration — and it insists that all
six be boringly correct before any of them are clever.

The second wolf is **Étoilé**: stranger, moonlit, carrying a notebook full of
object graphs. It gives Cabochon its nervous system: typed dynamic dispatch,
object identity, service discovery, live bindings, undoable commands, and
workflows composed from whatever happens to be selected.

The first wolf builds a tidy house. The second wolf quietly installs a
laboratory under it. 🜁

## 4. System overview

The proposed stack:

```text
Applications
  ↓
cabochon-clerestory: windows, menus, dialogs, widgets, commands, accessibility
  ↓
cabochon-escutcheon: compiled UI descriptions, menus, icons, strings, shortcuts
  ↓
cabochon-lapidary: vector/text/document scene model
  ↓
cabochon-burin: GPU, software, PDF, SVG, raster, and print rendering
  ↓
Wayland buffers / PDF streams / CUPS jobs / object streams
  ↓
cabochon-mullion, CUPS, portals, PipeWire, filesystem, services
```

The object layer crosses the application and desktop boundary:

```text
cabochon-enfilade
  Documents, selections, tools, queries, services, views, projects
  ↓
Typed selectors, capabilities, live bindings, undoable commands
  ↓
Cabochon desktop, application, and rendering primitives
```

The workspace:

```text
cabochon
├── cabochon-mullion       compositor and layout
├── cabochon-clerestory    AES
├── cabochon-lapidary      VDI
├── cabochon-burin         renderer
├── cabochon-escutcheon    resources
├── cabochon-portal        XDG portal backend
└── cabochon-enfilade      object graph
```

`cabochon-mullion` is the compositor and layout system, preferably built on
Smithay. Smithay describes itself as a general framework for building Wayland
compositors in Rust; it provides low-level helpers and abstractions for
system-level and protocol interactions, while leaving policy such as window
management and drawing to the compositor author. That is exactly the right
deal: use Smithay for the swamp boots, write Cabochon's personality ourselves.
( [Docs.rs][5]) Mullion owns Wayland presentation, window placement,
workspaces, focus, decorations, output configuration, and desktop spatial
policy. A mullion is the member that divides and supports panes. This component
divides and supports the visible desktop.

`cabochon-clerestory` is the modern AES: the application environment system. It
provides windows, menus, actions, dialogs, toolbars, inspectors, panels,
alerts, keyboard navigation, focus, drag-and-drop, clipboard, settings,
accessibility roles, and command routing. A clerestory is the band of a
building where the windows are. Clerestory makes windows, and everything a
window contains.

`cabochon-lapidary` is the modern VDI: the virtual device interface. It defines
the document and scene model — paths, fills, strokes, text runs, images,
clipping, transforms, layers, hit regions, annotations, semantic anchors,
metadata, colour spaces, and output intents. *Lapidary*, of prose or of stone:
cut to the minimum, exact at every edge, meant to survive the pressman. A
Lapidary scene says precisely what the page is and nothing about how a pixel
gets there.

`cabochon-burin` is the renderer. It turns Lapidary scenes into GPU output,
software output, PDF, SVG, raster images, thumbnails, clipboard
representations, and printer-bound output. Vello is a plausible engine because
it is a Rust 2D graphics renderer using wgpu, aimed at large 2D scenes with
interactive performance. ([Docs.rs][6]) A burin engraves; this layer makes
marks.

`cabochon-escutcheon` is the resource system. It compiles declarative UI
descriptions, menus, icons, strings, keyboard shortcuts, command metadata,
localization data, and accessibility annotations into compact application
resources. This is the spiritual heir to GEM `.RSC` files, but textual,
diffable, localized, and type-checkable. An escutcheon carries identity and
ornament without becoming the whole door.

`cabochon-portal` is the XDG portal backend. XDG Desktop Portal exists so
sandboxed applications, including Flatpak applications and similar containment
frameworks, can interact with the host through secure, well-defined interfaces.
([Flatpak][7]) Cabochon should not dodge that ecosystem; it should make the
portal UX clean and consistent.

`cabochon-enfilade` is the live object graph. It maintains object identity,
typed selectors, service discovery, semantic links, project workspaces,
workflow composition, undo transactions, scripting bindings, and
capability-checked automation. An enfilade is a sequence of rooms aligned by
doorways; this component lets objects see and reach one another without
collapsing every room into one ungoverned hall.

## 5. Clerestory: the modern AES

The original AES provided windows, widgets, menus, dialogs, object trees, and
events. Clerestory, the modern AES, should provide the same category of thing,
but with modern obligations.

An application should declare commands, not merely callbacks. A command has a
name, icon, keyboard shortcut, enablement predicate, undo behaviour, menu
placement, toolbar presentation, accessibility label, and optional service
exposure. This lets the desktop inspect and recombine application behaviour
without treating each app as a sealed pebble.

A minimal application might look like this:

```rust
cabochon_app! {
    id: "org.cabochon.writer",
    name: "Cabochon Writer",

    commands {
        command save: SaveDocument;
        command export_pdf: ExportPdf;
        command print: PrintDocument;
        command insert_table: InsertTable;
    }

    windows {
        document_window WriterWindow {
            title: bind(document.title),
            menu: "writer-main",
            toolbar: "writer-toolbar",
            content: DocumentCanvas;
            inspector: StyleInspector;
        }
    }
}
```

Clerestory should enforce consistency where consistency helps users: menus,
keyboard navigation, text fields, file dialogs, alerts, inspectors, colour
pickers, font panels, print dialogs, and window controls. Wayland has an
xdg-decoration protocol that allows negotiation of server-side decoration for
toplevel surfaces; Cabochon should use this where available and impose a strict
house style for its own native applications. ([wayland.app][8])

But Clerestory must not become a prison. Applications should embed custom
canvases, GPU views, terminal grids, games, and media surfaces. The toolkit
should distinguish between "standard desktop furniture" and
"application-specific instrument panel".

## 6. Lapidary: the modern VDI

Lapidary, the modern VDI, is the heart of the proposal.

Classic GEM's VDI offered device-independent drawing primitives. Cabochon
should update that idea into a **semantic 2D scene and document model**. The
key object is a `Surface`, but not merely a Wayland surface. A Lapidary surface
can target screen, PDF, printer, image export, thumbnail, clipboard
representation, accessibility extraction, or object preview.

A Lapidary scene contains:

```text
Paths
Text runs
Images
Clips
Transforms
Groups
Layers
Colour spaces
Output intents
Hit regions
Accessibility fragments
Semantic anchors
Metadata
```

The same document scene can render to a Wayland buffer for display, to PDF for
export, to CUPS for printing, to SVG for interchange when possible, or to
raster formats for thumbnails.

Text must be shaped properly. HarfBuzz describes text shaping as the conversion
of Unicode input into properly formatted and positioned glyph output for
writing systems and languages; that is not decorative polish, it is table
stakes for a humane desktop. ([harfbuzz.github.io][9])

A Lapidary text call should therefore not be:

```c
v_gtext(handle, x, y, "hello");
```

It should be closer to:

```rust
let run = TextRun::new("Dùn Èideann")
    .font(fira_sans)
    .features(["kern", "liga", "case"])
    .language("gd")
    .direction(Direction::Auto)
    .size(pt(11.0));

surface.text(run).at(point(72.0, 144.0));
```

Lapidary should understand physical units and logical units. A page layout view
needs points, millimetres, pixels, and CSS-like density-independent units
without turning every conversion into a small lawsuit. The system should keep
page-space, viewport-space, and device-space explicit.

Colour deserves similar respect. The Wayland colour-management extension aims
to let clients know output colour properties and tell the compositor about the
colour properties of surface content, so the compositor can manage content
across outputs. Cabochon should build colour semantics into Lapidary scenes and
then map them into Wayland, PDF, and print output. ([wayland.app][10])

## 7. Enfilade: Objective Rust without the haunted mansion

Enfilade needs dynamism, but Rust should guard the doors. The object layer
should support service discovery, scripting, object browsing, live bindings,
and cross-application workflows. It should not require every object to be an
`Any` soup tureen.

The central abstraction is an object reference:

```rust
struct ObjectRef {
    id: ObjectId,
    type_id: TypeId,
    capabilities: CapabilitySet,
}
```

Objects advertise typed selectors:

```rust
selector! {
    document.render_as(format: MimeType) -> Result<Bytes>;
}

selector! {
    selection.replace_with(content: ObjectRef) -> Result<EditId>;
}

selector! {
    table.bind_csv(source: DataSourceRef, range: CellRange) -> Result<BindingId>;
}
```

A selector is dynamic at the call site but statically declared by the provider.
The runtime can ask, "does this object respond to
`render_as(application/pdf)`?" without pretending every possible method exists
on every possible thing. This is what Objective Rust means in practice: it
borrows Objective-C's message-passing spirit, and it rejects unbounded runtime
chaos.

Services become composable objects:

```text
Selected text
  → summarize
  → translate
  → insert as margin note
  → export annotated PDF
```

A user should be able to build workflows visually, script them textually, or
invoke them through command palettes and context menus. The file manager,
editor, terminal, DTP app, image viewer, mail client, and IDE become
cooperating views over objects, not rival kingdoms throwing MIME types over a
fence.

GNUstep matters here as lineage, not as mandatory implementation. GNUstep
describes itself as a free, object-oriented, cross-platform development
environment based on Cocoa/OpenStep ideas. ([gnustep.org][11]) Étoilé
historically sat in that orbit. Cabochon can learn from that tradition while
implementing the dangerous parts in Rust.

## 8. Security and sandboxing

The object desktop must not become a burglary API wearing a monocle.

Every dynamic operation needs a capability boundary. A document object can
expose "render preview" widely, "read content" to trusted services, and "mutate
content" only through explicit user-granted transactions. A service that
receives selected text should not silently receive the whole project directory.
A PDF export service should not gain camera access because it once shook hands
with a window.

Cabochon should integrate with Flatpak-style portals rather than bypassing
them. XDG portals provide controlled interfaces for sandboxed applications to
interact with host resources; `cabochon-portal` should present those permission
requests through Clerestory-native dialogs and record grants as object
capabilities. ([Flatpak][7])

PipeWire should handle audio, video, and capture plumbing. Its documentation
describes graph-based processing, flexible media negotiation, and low-latency
audio/video infrastructure; its portal access documentation describes
portal-mediated access to PipeWire for sandboxed clients. ([docs.pipewire.org][
12]) That maps neatly to Cabochon screen sharing, recording, camera access,
live thumbnails, media workflows, and object previews.

Security model:

```text
User intent
  ↓
Portal or Clerestory permission surface
  ↓
Capability token
  ↓
Object selector invocation
  ↓
Undoable transaction or read-only result
  ↓
Auditable event log
```

No ambient "because process X is running as user Y, it can rummage through
everything". That Unix model remains useful underneath, but the desktop should
present a narrower semantic permission model above it.

## 9. User experience

Cabochon should look clean, dense, and crisp. Not retro cosplay. Not glassy
theatre. It should feel like a machine designed by people who respect
typography, keyboard navigation, and quiet margins.

The visual language:

Light and dark themes, both restrained. Server-side decorations for native apps
where possible. Clear menu bars, not hamburger archaeology. Command palette for
expert use. Inspector panels rather than modal property mazes. Monochrome
symbolic icons with optional colour accents. Excellent keyboard traversal.
First-class zoom, rulers, grids, guides, and page previews. A file manager that
understands projects and documents, not just directories.

The desktop metaphor changes subtly. The desktop is not a dumping ground. It is
a workspace. A project can contain files, live queries, notes, references,
generated views, build tasks, documents, conversations, screenshots, and
services. A project window resembles a Finder, IDE, DTP pasteboard, and object
browser folded into one disciplined creature.

A selection becomes an object. Select a paragraph, a rectangle in a PDF, a
range in a spreadsheet, a symbol in source code, or an image region, and
Cabochon exposes relevant services. The context menu stops being a junk drawer
and becomes a typed affordance surface.

Example:

```text
Selected object: PDF region
Available services:
  Extract text
  Copy as image
  Add annotation
  Summarize
  Translate
  Create issue from selection
  Insert into layout frame
  Save as reusable clipping
```

That is the Étoilé spark: workflows assembled from objects and services, not
from application silos.

## 10. Developer model

A native Cabochon application should be smaller than the equivalent GTK/Qt app
for common desktop tasks, and dramatically smaller than an Electron app. The
promise is not raw minimalism; the promise is that the common desktop burdens
arrive pre-wired.

A document app gets:

```text
Windows
Menus
Commands
Undo/redo
Autosave
Recent documents
File dialogs
Print/export
Accessibility
Clipboard
Drag-and-drop
Font panel
Colour panel
Inspector panels
Object services
PDF output
Thumbnail generation
```

The developer defines document types, views, commands, and services.

```rust
#[derive(Document)]
struct Poster {
    pages: Vec<Page>,
    styles: StyleSheet,
    assets: AssetLibrary,
}

impl Renderable for Poster {
    fn render(&self, surface: &mut LapidarySurface, intent: RenderIntent) -> Result<()> {
        for page in &self.pages {
            page.render(surface, intent)?;
        }
        Ok(())
    }
}

#[service]
impl Poster {
    #[selector("document.render_as")]
    fn render_as(&self, format: MimeType) -> Result<Bytes> {
        cabochon_burin::render(self, format)
    }
}
```

The resource file remains readable:

```text
window PosterWindow {
    title: bind(document.title)
    content: canvas(id: "page-view")
    left: tool_palette("layout-tools")
    right: inspector("properties")
    bottom: status_bar {
        label bind(selection.summary)
        zoom_control bind(view.zoom)
    }
}

menu main {
    item command("file.new")
    item command("file.open")
    item command("file.save")
    separator
    item command("file.export_pdf")
    item command("file.print")
}
```

A cabochon is polished, not faceted: the stone shows its material rather than
the cutting. The toolkit should do the same. Boring things easy, precise things
possible, and no visible seam where the developer had to apologize for the
system.

## 11. Compatibility

Cabochon should not attempt heroic purity. Purity is how projects become
beautiful ruins.

Compatibility layers:

Native Cabochon applications use Clerestory and Lapidary directly. Existing
Wayland applications run as normal Wayland clients. X11 applications run
through XWayland. Flatpak applications interact through `cabochon-portal`.
Print output goes through PDF and CUPS. Screen capture and media routes go
through PipeWire. Document interchange uses PDF, SVG, PNG, OpenDocument where
feasible, plain text, Markdown, and application-defined structured formats.

Mullion should support normal Wayland desktop protocols where possible.
xdg-shell defines the basic functionality needed for desktop-style windows such
as dragging, resizing, maximizing, and popups. ([wayland.app][4]) Fractional
scaling and colour-management protocols matter because a modern desktop cannot
shrug at mixed-DPI monitors or wide-gamut and HDR workflows.

Native apps get the richest integration, but the system remains a good citizen
in the broader Linux ecosystem.

## 12. Implementation plan

The sane first milestone is not the object desktop. That beast has antlers.

Milestone 1: **Compositor and shell skeleton**

Build `cabochon-mullion` on Smithay. ([Docs.rs][5]) Implement toplevel windows,
popups, decorations, focus, keyboard and mouse input, basic workspaces,
launcher, panel, settings daemon, and XWayland support. Keep policy simple. No
tiling opera. No animation carnival.

Milestone 2: **Clerestory toolkit MVP**

Implement windows, menus, buttons, labels, text fields, scroll views, list
views, tree views, dialogs, command routing, keyboard navigation, and
accessibility metadata. Create `cabochon-escutcheon` with a textual resource
compiler.

Milestone 3: **Lapidary scene model**

Implement vector paths, text, images, clipping, transforms, layers, hit
testing, and invalidation. Back it with Vello and wgpu for GPU rendering, plus
a software or Cairo fallback where necessary. Vello's Rust and wgpu design
makes it a plausible engine for high-quality 2D scenes. ([Docs.rs][6])

Milestone 4: **Documents and output**

Add PDF export, print dialog, CUPS integration, thumbnailer, document autosave,
undo/redo transactions, and clipboard representations. Treat page layout as a
first-class test case from day one.

Milestone 5: **Portal backend**

Implement file picker, open URI, screenshots, screencast, notifications,
settings, secrets, and print portals with Clerestory-native UI. Integrate
PipeWire for screencast, audio, and video flows. ([Flatpak][7])

Milestone 6: **Enfilade object layer**

Implement object identity, typed selectors, service discovery, command palette
integration, object inspector, workflow editor, and scripting bindings. Start
with safe read-only services, then add user-confirmed mutable transactions.

Milestone 7: **Reference applications**

Ship small but serious native apps:

A text editor with print and export. A PDF and image viewer with annotation. A
DTP layout toy that is deliberately more capable than a toy. A file and project
browser. A terminal. A settings app. A font and colour demo. A service and
workflow editor.

The DTP app is strategically important. If Cabochon cannot make a newsletter,
poster, or booklet without sweating through its waistcoat, Lapidary has failed.

## 13. Risks

The largest risk is scope intoxication. A compact desktop plus object system
plus renderer plus toolkit plus compositor is a banquet with knives in every
course. The project must resist becoming "GNOME, but written by three people
and a fox".

The second risk is Wayland protocol churn and desktop integration edge cases. A
small compositor must track enough protocols to be useful without trying to
match GNOME and KDE feature-for-feature. This argues for Smithay, strict MVP
boundaries, and ruthless compatibility prioritization.

The third risk is text and accessibility. These are not optional polish.
International text requires shaping, bidirectional handling, line breaking,
font fallback, input methods, caret movement by grapheme cluster, and
screen-reader semantics. HarfBuzz solves shaping, not the entire text stack
([harfbuzz.github.io][9]). This paper's cheery little text box hides a deep
well.

The fourth risk is dynamic object security. An object desktop sounds magical
until every service can see everything. Capabilities, audit trails, sandbox
boundaries, and user-mediated grants must exist from the start, not after the
first incident involving a clipboard daemon with delusions of grandeur.

The fifth risk is developer adoption. A new native toolkit must offer either a
better experience or a strongly differentiated niche. Cabochon's niche should
be document-heavy, print-aware, automation-friendly applications that want to
feel native without inheriting enormous frameworks.

## 14. Evaluation criteria

The project succeeds if a developer can write a native document application
that opens quickly, uses little memory, renders crisp text and vector graphics,
exports accurate PDFs, prints through CUPS, supports accessibility, and exposes
its selections and commands as composable services.

The desktop succeeds if users can run ordinary Wayland applications, manage
files and projects, print and export documents, use screen sharing through
portals, and build simple object workflows without knowing whether the
underlying thing is a process, file, buffer, stream, or service.

The architecture succeeds if each subsystem can be understood independently:

```text
Mullion: presentation and input
Clerestory: application interaction
Lapidary: document and scene representation
Burin: output backends
Escutcheon: declarative UI and assets
Portal: controlled host access
Enfilade: semantic composition
```

## 15. Conclusion

Cabochon is a proposal for a desktop with bones. It borrows GEM's clearest
idea, the separation between application environment and device-independent
graphics, and rebuilds it for Wayland, GPU rendering, PDF, modern text, colour
management, portals, and sandboxing.

Enfilade is the moonlit extension: a live object environment where documents,
selections, tools, services, and projects can discover and recombine one
another. Its job is to make the desktop programmable without reducing users to
sysadmins or forcing every workflow through a terminal pipe.

The combined system is not retrocomputing. It is an alternative branch of
desktop evolution: compact like GEM, object-literate like Étoilé, typed like
Rust, print-aware like a proper publishing system, and just strange enough to
deserve a workshop with brass labels on the drawers.

**Cabochon is a small desktop where rectangles behave, text knows what language
it is, printing is not a séance, and every useful thing can politely introduce
itself to every other useful thing.**

[1]: https://wayland.freedesktop.org/architecture.html "Wayland architecture"
[2]: https://etoileos.com/ "Étoilé"
[3]: https://openprinting.github.io/cups/doc/cupspm.html "CUPS Programming Manual"
[4]: https://wayland.app/protocols/xdg-shell "XDG shell protocol"
[5]: https://docs.rs/smithay "smithay - Rust"
[6]: https://docs.rs/vello "vello - Rust"
[7]: https://flatpak.github.io/xdg-desktop-portal/docs/ "XDG Desktop Portal documentation"
[8]: https://wayland.app/protocols/xdg-decoration-unstable-v1 "XDG decoration protocol"
[9]: https://harfbuzz.github.io/ "HarfBuzz Manual: HarfBuzz Manual"
[10]: https://wayland.app/protocols/color-management-v1 "Color management protocol"
[11]: https://www.gnustep.org/information/aboutGNUstep.html "GNUstep: Introduction"
[12]: https://docs.pipewire.org/ "PipeWire: PipeWire"
