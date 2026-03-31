// Script backend system — language-agnostic scripting plugin interface.
//
// The parser extracts the <script lang="..."> block as a raw string.
// At runtime, the matching ScriptBackend plugin is looked up by lang name
// and handed the raw source to interpret or compile.
//
// Responsibilities:
// - ScriptBackend trait: lang() -> &str, load(source) -> ComponentState
// - ScriptRegistry: register and look up backends by lang name
// - Built-in "mgt" lang: simple declarative prop/state declarations (no external plugin needed)
//
// Users add scripting support by installing a plugin crate that registers
// a ScriptBackend (e.g. mgt-lua, mgt-rhai) with the runtime.
