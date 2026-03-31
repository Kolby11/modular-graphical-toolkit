// Markup layer — .mgt single-file component (SFC) parser
//
// Each .mgt file is a Svelte-like component with three sections:
//
//   <script lang="..."> — component logic (opaque, handed to a ScriptBackend)
//   <style>             — scoped CSS-like styles
//   markup              — declarative XML-like widget tree
//
// The script block is language-agnostic. The `lang` attribute selects which
// ScriptBackend plugin handles it. Defaults to the built-in "mgt" declarative
// format when no lang is specified.
//
// Language plugins (e.g. mgt-lua, mgt-rhai) register a ScriptBackend with
// the runtime's ScriptRegistry — the core parser never interprets script source.

mod ast;
mod parser;
mod script;
mod style;

pub use ast::{
    Attribute, AttributeValue, Element, Node, RawScript, SfcFile,
    StyleProperty, StyleRule, StyleSelector, TextPart,
};
pub use parser::parse_sfc;
pub use script::{ComponentState, ScriptBackend, ScriptRegistry};
