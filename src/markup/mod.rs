// Markup layer — .mgt single-file component (SFC) parser
//
// Each .mgt file is a Svelte-like component with three sections:
//
//   <script lang="..."> — component logic (opaque, handed to a ScriptBackend)
//   <style>             — scoped CSS-like styles
//   <markup>            — declarative XML-like widget tree
//
// The script block is language-agnostic. The `lang` attribute selects which
// ScriptBackend plugin handles it. If no lang is specified, the default
// built-in declarative format is used.
//
// Users install scripting language plugins (Lua, Rhai, etc.) separately.

mod ast;
mod parser;
mod script;
mod style;
