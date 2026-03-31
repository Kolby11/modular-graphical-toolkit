/// Top-level parsed representation of a `.mgt` single-file component.
#[derive(Debug, Clone)]
pub struct SfcFile {
    /// Optional script block — opaque source handed to a ScriptBackend.
    pub script: Option<RawScript>,
    /// Scoped style rules parsed from the `<style>` block.
    pub styles: Vec<StyleRule>,
    /// Root element of the markup section.
    pub root: Element,
}

/// Raw script block extracted from `<script lang="...">`.
/// The source is never interpreted by the core parser.
#[derive(Debug, Clone)]
pub struct RawScript {
    /// Scripting language name (e.g. "mgt", "lua", "rhai").
    pub lang: String,
    /// Raw source text, handed to the matching ScriptBackend at runtime.
    pub source: String,
}

/// A widget element node in the markup tree.
#[derive(Debug, Clone)]
pub struct Element {
    /// PascalCase component name, e.g. "Panel", "Button", "Label".
    pub tag: String,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Node>,
}

/// A single attribute on an element.
#[derive(Debug, Clone)]
pub struct Attribute {
    /// Attribute name, including namespace prefix if present (e.g. "on:click", "bind:value").
    pub name: String,
    pub value: AttributeValue,
}

/// The value form of an attribute.
#[derive(Debug, Clone)]
pub enum AttributeValue {
    /// Plain string with no interpolation, e.g. `class="card"`.
    Static(String),
    /// Pure expression, e.g. `text="{title}"` → Expression("title").
    Expression(String),
    /// Mixed literal + expression, e.g. `label="Count: {count}"`.
    Interpolated(Vec<TextPart>),
}

/// A child node — either a nested element or inline text.
#[derive(Debug, Clone)]
pub enum Node {
    Element(Element),
    Text(Vec<TextPart>),
}

/// A segment of text content or an attribute value that may contain expressions.
#[derive(Debug, Clone)]
pub enum TextPart {
    Literal(String),
    Expression(String),
}

// --- Style AST ---

/// A single CSS-like style rule: selector + property declarations.
#[derive(Debug, Clone)]
pub struct StyleRule {
    pub selector: StyleSelector,
    pub properties: Vec<StyleProperty>,
}

/// A class selector with an optional pseudo-state.
/// e.g. `.card:hover` → class="card", pseudo=Some("hover")
#[derive(Debug, Clone)]
pub struct StyleSelector {
    pub class: String,
    pub pseudo: Option<String>,
}

/// A single property declaration, e.g. `bg: bg.elevated`.
#[derive(Debug, Clone)]
pub struct StyleProperty {
    pub name: String,
    /// Value referencing a theme token (e.g. "bg.elevated") or a literal.
    pub value: String,
}
