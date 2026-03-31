use anyhow::{anyhow, Context, Result};
use quick_xml::{
    events::{BytesStart, Event},
    Reader,
};

use crate::markup::ast::{
    Attribute, AttributeValue, Element, Node, RawScript, SfcFile, TextPart,
};
use crate::markup::style::parse_style;

// ── Public entry point ────────────────────────────────────────────────────────

/// Parse a complete `.mgt` single-file component from source text.
pub fn parse_sfc(source: &str) -> Result<SfcFile> {
    let sections = split_sections(source)?;

    let script = match (sections.script_lang, sections.script_source) {
        (Some(lang), Some(src)) => Some(RawScript { lang, source: src }),
        _ => None,
    };

    let styles = match sections.style_source {
        Some(src) => parse_style(&src).context("Failed to parse <style> block")?,
        None => vec![],
    };

    let root = parse_markup(sections.markup_source.trim())
        .context("Failed to parse markup section")?;

    Ok(SfcFile { script, styles, root })
}

// ── SFC splitter ──────────────────────────────────────────────────────────────

struct RawSections {
    script_lang: Option<String>,
    script_source: Option<String>,
    style_source: Option<String>,
    markup_source: String,
}

fn split_sections(source: &str) -> Result<RawSections> {
    let (script_lang, script_source, after_script) = extract_tagged_block(source, "script");
    let (_, style_source, after_style) = extract_tagged_block(&after_script, "style");

    Ok(RawSections {
        script_lang,
        script_source,
        style_source,
        markup_source: after_style.trim().to_string(),
    })
}

/// Find a `<tag ...>content</tag>` block, extract its content, and return the
/// source with the block removed.
///
/// Returns `(lang_attr, content, remainder)`.
fn extract_tagged_block(source: &str, tag: &str) -> (Option<String>, Option<String>, String) {
    let open_tag = format!("<{}", tag);
    let close_tag = format!("</{}>", tag);

    let Some(start) = source.find(&open_tag) else {
        return (None, None, source.to_string());
    };

    let after_open = &source[start + open_tag.len()..];

    let Some(tag_end) = after_open.find('>') else {
        return (None, None, source.to_string());
    };

    let raw_attrs = &after_open[..tag_end];
    let lang = if tag == "script" {
        Some(extract_lang_attr(raw_attrs).unwrap_or_else(|| "mgt".to_string()))
    } else {
        None
    };

    let content_start = start + open_tag.len() + tag_end + 1;
    let rest = &source[content_start..];

    let Some(end_pos) = rest.find(&close_tag) else {
        return (None, None, source.to_string());
    };

    let content = rest[..end_pos].trim().to_string();
    let remainder = format!(
        "{}{}",
        &source[..start],
        &source[content_start + end_pos + close_tag.len()..]
    );

    (lang, Some(content), remainder)
}

fn extract_lang_attr(attrs: &str) -> Option<String> {
    let pos = attrs.find("lang=")?;
    let after = &attrs[pos + 5..];

    let (quote, rest) = if after.starts_with('"') {
        ('"', &after[1..])
    } else if after.starts_with('\'') {
        ('\'', &after[1..])
    } else {
        return None;
    };

    let end = rest.find(quote)?;
    Some(rest[..end].to_string())
}

// ── Markup parser ─────────────────────────────────────────────────────────────

/// Parse the markup section into an `Element` tree using a stack-based
/// approach over quick-xml's event stream.
fn parse_markup(source: &str) -> Result<Element> {
    let mut reader = Reader::from_str(source);
    let mut stack: Vec<Element> = Vec::new();

    loop {
        match reader.read_event()? {
            Event::Start(ref e) => {
                let tag = tag_name(e)?;
                let attributes = collect_attributes(e)?;
                stack.push(Element { tag, attributes, children: vec![] });
            }

            Event::End(_) => {
                let finished = stack
                    .pop()
                    .ok_or_else(|| anyhow!("Unexpected closing tag in markup"))?;

                if let Some(parent) = stack.last_mut() {
                    parent.children.push(Node::Element(finished));
                } else {
                    return Ok(finished);
                }
            }

            Event::Empty(ref e) => {
                let tag = tag_name(e)?;
                let attributes = collect_attributes(e)?;
                let elem = Element { tag, attributes, children: vec![] };

                if let Some(parent) = stack.last_mut() {
                    parent.children.push(Node::Element(elem));
                } else {
                    // Self-closing root element
                    return Ok(elem);
                }
            }

            Event::Text(ref e) => {
                let text = e.unescape()?.to_string();
                let trimmed = text.trim();
                if !trimmed.is_empty() {
                    let parts = interpolate(trimmed);
                    if let Some(parent) = stack.last_mut() {
                        parent.children.push(Node::Text(parts));
                    }
                }
            }

            Event::Eof => break,
            _ => {}
        }
    }

    Err(anyhow!("No root element found in markup section"))
}

fn tag_name(e: &BytesStart<'_>) -> Result<String> {
    Ok(std::str::from_utf8(e.name().as_ref())?.to_string())
}

fn collect_attributes(e: &BytesStart<'_>) -> Result<Vec<Attribute>> {
    let mut attrs = Vec::new();
    for result in e.attributes() {
        let attr = result?;
        let name = std::str::from_utf8(attr.key.as_ref())?.to_string();
        let raw_value = attr.unescape_value()?.to_string();
        let value = parse_attr_value(&raw_value);
        attrs.push(Attribute { name, value });
    }
    Ok(attrs)
}

/// Classify an attribute value as Static, Expression, or Interpolated.
///
/// - `"{title}"` → `Expression("title")`
/// - `"Count: {count}"` → `Interpolated([Literal("Count: "), Expression("count")])`
/// - `"card"` → `Static("card")`
fn parse_attr_value(raw: &str) -> AttributeValue {
    let t = raw.trim();

    // Pure expression: entire value is a single {expr}
    if t.starts_with('{') && t.ends_with('}') {
        let inner = &t[1..t.len() - 1];
        if !inner.contains('{') && !inner.contains('}') {
            return AttributeValue::Expression(inner.trim().to_string());
        }
    }

    // Mixed string with embedded expressions
    if t.contains('{') {
        return AttributeValue::Interpolated(interpolate(t));
    }

    AttributeValue::Static(t.to_string())
}

/// Split a string into literal and expression parts.
///
/// `"Hello {name}"` → `[Literal("Hello "), Expression("name")]`
fn interpolate(text: &str) -> Vec<TextPart> {
    let mut parts = Vec::new();
    let mut remaining = text;

    while let Some(open) = remaining.find('{') {
        if open > 0 {
            parts.push(TextPart::Literal(remaining[..open].to_string()));
        }
        let after = &remaining[open + 1..];
        if let Some(close) = after.find('}') {
            parts.push(TextPart::Expression(after[..close].trim().to_string()));
            remaining = &after[close + 1..];
        } else {
            // Unclosed brace — treat rest as a literal
            parts.push(TextPart::Literal(remaining[open..].to_string()));
            return parts;
        }
    }

    if !remaining.is_empty() {
        parts.push(TextPart::Literal(remaining.to_string()));
    }

    parts
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::markup::ast::AttributeValue;

    const SAMPLE: &str = include_str!("../../tests/sample.mgt");

    #[test]
    fn test_split_extracts_script() {
        let s = split_sections(SAMPLE).unwrap();
        assert_eq!(s.script_lang.as_deref(), Some("mgt"));
        assert!(s.script_source.as_deref().unwrap().contains("prop title"));
    }

    #[test]
    fn test_split_extracts_style() {
        let s = split_sections(SAMPLE).unwrap();
        assert!(s.style_source.as_deref().unwrap().contains(".card"));
    }

    #[test]
    fn test_split_extracts_markup() {
        let s = split_sections(SAMPLE).unwrap();
        assert!(s.markup_source.starts_with("<Panel"));
    }

    #[test]
    fn test_parse_full_sfc() {
        let sfc = parse_sfc(SAMPLE).unwrap();
        assert!(sfc.script.is_some());
        assert_eq!(sfc.script.unwrap().lang, "mgt");
        assert!(!sfc.styles.is_empty());
        assert_eq!(sfc.root.tag, "Panel");
    }

    #[test]
    fn test_root_has_children() {
        let sfc = parse_sfc(SAMPLE).unwrap();
        assert_eq!(sfc.root.children.len(), 3); // Label, Label, Button
    }

    #[test]
    fn test_expression_attribute() {
        let sfc = parse_sfc(SAMPLE).unwrap();
        let label = sfc.root.children.iter().find_map(|n| {
            if let Node::Element(e) = n {
                if e.tag == "Label" { return Some(e); }
            }
            None
        }).unwrap();
        let text_attr = label.attributes.iter().find(|a| a.name == "text").unwrap();
        assert!(matches!(text_attr.value, AttributeValue::Expression(_)));
    }

    #[test]
    fn test_interpolated_attribute() {
        let sfc = parse_sfc(SAMPLE).unwrap();
        let button = sfc.root.children.iter().find_map(|n| {
            if let Node::Element(e) = n {
                if e.tag == "Button" { return Some(e); }
            }
            None
        }).unwrap();
        let label = button.attributes.iter().find(|a| a.name == "label").unwrap();
        assert!(matches!(label.value, AttributeValue::Interpolated(_)));
    }

    #[test]
    fn test_on_event_attribute() {
        let sfc = parse_sfc(SAMPLE).unwrap();
        let button = sfc.root.children.iter().find_map(|n| {
            if let Node::Element(e) = n {
                if e.tag == "Button" { return Some(e); }
            }
            None
        }).unwrap();
        assert!(button.attributes.iter().any(|a| a.name == "on:click"));
    }

    #[test]
    fn test_style_rules() {
        let sfc = parse_sfc(SAMPLE).unwrap();
        let card = sfc.styles.iter()
            .find(|r| r.selector.class == "card" && r.selector.pseudo.is_none())
            .unwrap();
        assert!(card.properties.iter().any(|p| p.name == "bg" && p.value == "bg.elevated"));
    }

    #[test]
    fn test_pseudo_state_rule() {
        let sfc = parse_sfc(SAMPLE).unwrap();
        let hover = sfc.styles.iter()
            .find(|r| r.selector.class == "card" && r.selector.pseudo.as_deref() == Some("hover"));
        assert!(hover.is_some());
    }

    #[test]
    fn test_interpolate_fn() {
        let parts = interpolate("Hello {name}, you have {count} items");
        // [Literal("Hello "), Expression("name"), Literal(", you have "), Expression("count"), Literal(" items")]
        assert_eq!(parts.len(), 5);
        assert!(matches!(&parts[1], TextPart::Expression(s) if s == "name"));
        assert!(matches!(&parts[3], TextPart::Expression(s) if s == "count"));
        assert!(matches!(&parts[4], TextPart::Literal(s) if s == " items"));
    }

    #[test]
    fn test_no_script_block() {
        let src = "<style>.btn { bg: accent.primary; }</style><Button class=\"btn\" />";
        let sfc = parse_sfc(src).unwrap();
        assert!(sfc.script.is_none());
        assert_eq!(sfc.root.tag, "Button");
    }

    #[test]
    fn test_no_style_block() {
        let src = "<Button role=\"primary\" label=\"OK\" />";
        let sfc = parse_sfc(src).unwrap();
        assert!(sfc.styles.is_empty());
        assert_eq!(sfc.root.tag, "Button");
    }
}
