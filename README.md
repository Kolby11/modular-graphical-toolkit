# MGT — Modular Graphical Toolkit

A Linux-native, Wayland-first UI platform and framework with a web-like authoring model and a shared design system.

> **Web-authored, Linux-native UI**

---

## What is MGT?

MGT is a native UI engine for Linux. It is not a browser wrapper or an Electron-like runtime. It renders directly via GPU using wgpu and integrates natively with Wayland.

The authoring experience is designed to feel familiar to web developers — markup feels like XML, styling feels like CSS — but the output is fully native desktop UI.

---

## Authoring Model

Each component is a single `.mgt` file, inspired by Svelte:

```xml
<script lang="mgt">
  prop title: String
  state count: Int = 0
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
  .title {
    font: type.title;
    color: fg.primary;
  }
</style>

<Panel class="card">
  <Label class="title" text={title} />
  <Button role="primary" label="Count: {count}" on:click={handle_click} />
</Panel>
```

### Three sections per component

| Section | Purpose |
|---|---|
| `<script lang="...">` | Component logic — props, state, event handlers |
| `<style>` | Scoped styles for this component |
| Markup | Declarative XML widget tree |

---

## Design System

Apps and shell components express **semantic intent**, not raw styles. The design system resolves the final appearance.

```
final_style =
  design_system
  + active_theme
  + app_overrides
  + user_overrides
  + accessibility_adjustments
  + component_state
```

### Semantic tokens

Style properties reference named tokens, not raw values:

| Token | Meaning |
|---|---|
| `bg.surface` | Base background |
| `bg.elevated` | Raised surface (cards, panels) |
| `fg.primary` | Primary text |
| `fg.secondary` | Secondary / muted text |
| `accent.primary` | Primary accent color |
| `border.subtle` | Subtle border |
| `radius.sm` / `radius.lg` | Corner radius scale |
| `space.sm` / `space.md` | Spacing scale |
| `type.body` / `type.title` | Typography scale |
| `focus.ring` | Focus indicator style |

---

## Scripting

The `<script>` block is language-agnostic. The `lang` attribute selects the backend:

```xml
<!-- built-in, no plugin needed -->
<script lang="mgt">
  prop label: String
  state active: Bool = false
</script>

<!-- requires mgt-lua plugin -->
<script lang="lua">
  local count = 0
</script>

<!-- requires mgt-rhai plugin -->
<script lang="rhai">
  let count = 0;
</script>
```

The core parser never interprets script content — it extracts it as a raw string and hands it to the registered scripting backend.

---

## Architecture

```
src/
├── markup/      — .mgt SFC parser (script, style, markup sections)
├── layout/      — row, column, grid, stack, scroll
├── renderer/    — wgpu GPU renderer + cosmic-text
├── widget/      — widget tree, Widget trait, built-in components
├── platform/    — Wayland surface, input, protocols
└── runtime/     — theme resolution, focus, animations, accessibility
```

### Stack

| Concern | Crate |
|---|---|
| Wayland | `smithay-client-toolkit`, `wayland-client` |
| GPU rendering | `wgpu` |
| Text rendering | `cosmic-text` |
| Event loop | `calloop` |
| Error handling | `anyhow` |

---

## Status

Early development. Current milestone: markup parser (`src/markup/`).

| Layer | Status |
|---|---|
| Wayland window | Working (SHM, basic draw) |
| GPU renderer | Planned |
| Layout engine | Stub |
| Widget tree | Stub |
| Markup parser | Structure in place |
| Design system / theme | Planned |
| Runtime | Planned |
| Shell components | Planned |
