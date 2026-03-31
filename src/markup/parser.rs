// SFC parser — splits a .mgt file into its three raw sections, then
// delegates style and markup parsing to their respective sub-parsers.
//
// Responsibilities:
// - Extract <script lang="..."> block as (lang: String, source: String) — opaque, not parsed here
// - Extract <style> block and pass to style parser
// - Extract root markup element and pass to markup/AST parser
// - Return a complete SfcFile AST
