# Wayland-First Theme System, Shell Platform, and Linux-Native UI Framework

## Vision

Create a **Linux-native, Wayland-first UI platform and framework** with a **web-like authoring model** and a **shared design system**.

The platform should provide:

- a unified design system for shell components and general applications
- centralized theming with user overrides
- strong default styling similar in spirit to systems like Material 3
- standardized component behavior and semantics
- modular standalone shell parts
- inspectable live component trees
- schema-based widget and component configuration
- native desktop integration through portals

The authoring experience should feel familiar to web developers:

- markup should feel like **HTML/XML**
- styling should feel like **CSS**

But the result should be:

- **native UI**
- **Wayland-first**
- **desktop-oriented**
- **not a browser clone**
- **not Electron-like**

A good framing is:

> **Web-authored, Linux-native UI**

---

## Core Goal

The central idea is:

> Apps and shell components should not define their own style directly.  
> They should express semantic intent, and the system should decide the final appearance.

This ensures:

- consistent styling
- centralized customization
- accessibility support
- easier maintenance
- unified behavior across the platform

Another core principle is:

> The shell is not separate from the design system.  
> It is simply one set of first-party applications built on top of it.

---

## What the Framework Should Be

The framework should aim for:

- **HTML/XML-like syntax**, but not full HTML compatibility
- **CSS-like styling**, but not full CSS compatibility
- **desktop UI semantics**, not browser/document semantics
- **Wayland-first architecture**
- **native rendering**
- **consistent look and behavior across apps built with the framework**

This should be a **native UI engine**, not a small browser wrapper.

It should support both:

- **shell UI**
- **general application UI**

---

## High-Level Architecture

The platform should be split into distinct layers.

### 1. Markup Layer

A custom declarative markup language inspired by HTML/XML.

It should be familiar to web developers, but based on **framework-native desktop components**, not generic browser elements.

Examples:

- `Window`
- `Panel`
- `Dialog`
- `Button`
- `IconButton`
- `TextField`
- `SearchField`
- `ListView`
- `Tabs`
- `Menu`
- `Popover`
- `Sheet`
- `SplitView`

Prefer framework-native components over raw `div` / `span` style authoring as the primary model.

### 2. Styling Layer

A CSS-like styling system with:

- classes
- simple selectors
- pseudo states like `:hover`, `:focus`, `:disabled`
- variables / tokens
- optional scoped styles

This should be familiar to web developers, but more limited and predictable than full CSS.

### 3. Theme Layer

Theme resolution should be layered.

A good model is:

- framework defaults
- system theme mapping
- active theme
- app theme overrides
- local component overrides
- accessibility adjustments
- runtime state

This allows:

- consistent appearance across apps built with the framework
- system appearance integration
- developer customization
- centralized user control

### 4. Layout / Composition Layer

Prefer a simplified layout system inspired by flex/grid, but designed for desktop UI.

Useful primitives:

- row
- column
- grid
- stack
- split
- scroll

Avoid full browser layout complexity.

### 5. Rendering Layer

Use a native renderer rather than WebKit/Blink.

Prefer a GPU-first rendering model for:

- smooth UI
- efficient custom widgets
- modern visuals
- better long-term performance

### 6. Wayland / Platform Layer

This layer should manage:

- surfaces
- outputs / monitors
- scaling
- input
- clipboard
- drag and drop
- text input / IME
- protocol negotiation
- application identity
- accessibility bridge
- locale and translation propagation
- session state

This is important because Wayland is capability-based and different compositors expose different protocol sets.

### 7. Toolkit Runtime

This is the shared runtime for all shell components and applications.

Responsibilities:

- theme resolution
- user theme overrides
- component lifecycle
- focus handling
- input routing
- animations
- localization
- accessibility semantics
- tooltip/help handling
- shared settings
- config propagation
- live inspection metadata

This runtime is **toolkit-specific, not shell-specific**.

---

## Platform Roles

The system should clearly separate these concerns:

### Design System
Defines what things **mean**.

### Component Library
Defines how those meanings become reusable UI elements.

### Runtime
Distributes the system consistently across the shell and apps.

### Platform Layer
Connects the framework to Wayland, portals, and OS services.

Keeping these separate will make the framework much more maintainable.

---

## Design System

The platform should be built around a **design system**, not only a collection of widgets.

### The design system should define:

- color roles
- typography scale
- spacing scale
- radius scale
- elevation model
- motion model
- focus styling
- input states
- layout conventions
- semantic component roles

### Important distinction

The design system defines what things **mean**.

The component library defines how those meanings become reusable UI elements.

The runtime distributes that system consistently across shell and apps.

---

## Theme-First Model

The platform should be **theme-first**.

That means the theme is not an optional skin layer added later.
It is a central contract used by every component.

### Theme should be based on semantic tokens

Examples:

- `bg.surface`
- `bg.elevated`
- `bg.overlay`
- `fg.primary`
- `fg.secondary`
- `accent.primary`
- `accent.weak`
- `border.subtle`
- `focus.ring`
- `radius.sm`
- `radius.lg`
- `space.sm`
- `space.md`
- `type.body`
- `type.title`
- `motion.fast`
- `motion.normal`

### Why semantic tokens matter

Applications and components should not request raw styles like:

- blue background
- 16px radius
- `#ffffff` text

Instead they should request semantic roles like:

- primary action
- secondary action
- destructive action
- base surface
- elevated surface
- input field
- navigation item
- selected item

This lets the system remain consistent when themes change.

---

## Centralized User Customization

Users should be able to override design centrally.

Examples of allowed user customization:

- accent color
- density
- radius scale
- typography family
- type scale
- motion intensity
- blur/transparency strength
- shadow strength
- light/dark mode
- contrast level

This should happen through a **central theme config**.

Users should not need to restyle every individual widget manually.

### Principle

Controlled centralized customization is better than unlimited freedom.

Too much per-widget styling usually leads to inconsistency.

---

## Unified Styling Across Apps

A major goal is to make sure all apps and shell components use one standardized styling system.

This is achieved through:

- shared design tokens
- shared semantic roles
- shared component contracts
- shared runtime theme resolution
- shared accessibility and localization model

### Styling rule

Apps should not define their own visual style directly unless there is a very strong reason.

Instead they should express intent and let the design system resolve the final appearance.

### Final style resolution

A useful model is:

```text id="final-style-resolution"
final_style =
  design_system
  + system_theme_mapping
  + active_theme
  + app_overrides
  + user_overrides
  + accessibility_adjustments
  + component_state
```

---

## Component Authoring Model

The authoring model is inspired by **Svelte** — each component lives in a single `.mgt` file with three sections:

```xml
<script lang="lua">
  -- component logic
</script>

<style>
  .card {
    bg: bg.elevated;
    radius: radius.lg;
    padding: space.md;
  }
  .card:hover {
    bg: bg.elevated.hover;
  }
</style>

<Panel class="card">
  <Label class="title" text={title} />
  <Button role="primary" label="Submit" on:click={handle_click} />
</Panel>
```

### Sections

| Section | Purpose |
|---|---|
| `<script lang="...">` | Component logic — props, state, event handlers |
| `<style>` | Scoped CSS-like styles for this component only |
| Markup | Declarative XML-like widget tree |

### Language-Agnostic Script Block

The `<script>` block is **opaque to the core parser**. The `lang` attribute selects which scripting backend processes it.

- The core parser extracts script as a raw `(lang, source)` string — it never interprets it
- Language support is provided by installable plugin crates (e.g. `mgt-lua`, `mgt-rhai`)
- A built-in `lang="mgt"` declarative format handles simple prop/state declarations with no plugin required

```xml
<!-- built-in declarative format, no plugin needed -->
<script lang="mgt">
  prop title: String
  prop subtitle: String?
  state count: Int = 0
</script>

<!-- Lua via mgt-lua plugin -->
<script lang="lua">
  local count = 0
  function handle_click() count = count + 1 end
</script>
```

### Style Scoping

Styles defined in `<style>` are **scoped by default** — they apply only to the component they are defined in, preventing leakage across the component tree.

Global or shared styles are defined in standalone `.style` files and imported by the runtime.

### Markup Syntax

- XML-based (strict, case-sensitive — supports PascalCase component names)
- `{expression}` interpolation in attribute values and text nodes
- `on:event` syntax for event bindings (`on:click`, `on:focus`, `on:change`)
- `bind:value` for two-way bindings
- Framework-native component names (`Button`, `Panel`, `ListView`) — not raw `div`/`span`

---

## Module Structure

```
src/
├── markup/
│   ├── mod.rs       — public API
│   ├── parser.rs    — SFC file splitter + markup/style parsing
│   ├── ast.rs       — AST types: SfcFile, Element, Attribute, TextNode
│   ├── style.rs     — scoped CSS-like style block parser
│   └── script.rs    — ScriptBackend trait + ScriptRegistry
├── layout/          — row, column, grid, stack, scroll
├── renderer/        — wgpu GPU renderer
├── widget/          — widget tree, Widget trait, built-in widgets
├── platform/        — Wayland surface, input, protocols
└── runtime/         — theme resolution, focus, animations, a11y (planned)
```