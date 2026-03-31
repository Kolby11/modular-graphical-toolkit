// AST types for the markup and style sections of a .mgt file.
//
// Covers:
// - SfcFile      — top-level parsed .mgt file (script block, style block, root element)
// - Element      — a widget node with tag name, attributes, children
// - Attribute    — key/value, supports {expression} interpolation and on:event syntax
// - TextNode     — inline text content with optional {expr} interpolation
