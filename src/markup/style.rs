use anyhow::{anyhow, Result};

use crate::markup::ast::{StyleProperty, StyleRule, StyleSelector};

/// Parse the raw content of a `<style>` block into a list of style rules.
///
/// Supported syntax:
/// ```text
/// .selector { property: value; property: value; }
/// .selector:pseudo { property: value; }
/// ```
pub fn parse_style(source: &str) -> Result<Vec<StyleRule>> {
    let mut rules = Vec::new();
    let mut input = source.trim();

    while !input.is_empty() {
        input = input.trim_start();
        if input.is_empty() {
            break;
        }

        let brace_open = input
            .find('{')
            .ok_or_else(|| anyhow!("Expected '{{' after selector in style block"))?;

        let selector_str = input[..brace_open].trim();
        let selector = parse_selector(selector_str)?;

        let rest = &input[brace_open + 1..];
        let brace_close = rest
            .find('}')
            .ok_or_else(|| anyhow!("Unclosed '{{' in style rule for '{}'", selector_str))?;

        let block = &rest[..brace_close];
        let properties = parse_properties(block)?;

        rules.push(StyleRule { selector, properties });
        input = &rest[brace_close + 1..];
    }

    Ok(rules)
}

fn parse_selector(s: &str) -> Result<StyleSelector> {
    let s = s.trim().trim_start_matches('.');

    if let Some(colon) = s.find(':') {
        Ok(StyleSelector {
            class: s[..colon].to_string(),
            pseudo: Some(s[colon + 1..].to_string()),
        })
    } else {
        Ok(StyleSelector {
            class: s.to_string(),
            pseudo: None,
        })
    }
}

fn parse_properties(block: &str) -> Result<Vec<StyleProperty>> {
    let mut props = Vec::new();

    // Split by `;` so both single-line and multi-line formats are handled equally.
    for declaration in block.split(';') {
        let decl = declaration.trim();
        if decl.is_empty() {
            continue;
        }

        // Split only on the first colon to preserve token values like "bg.elevated"
        if let Some(colon) = decl.find(':') {
            let name = decl[..colon].trim().to_string();
            let value = decl[colon + 1..].trim().to_string();
            if !name.is_empty() && !value.is_empty() {
                props.push(StyleProperty { name, value });
            }
        }
    }

    Ok(props)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_rule() {
        let rules = parse_style(".card { bg: bg.elevated; radius: radius.lg; }").unwrap();
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].selector.class, "card");
        assert!(rules[0].selector.pseudo.is_none());
        assert_eq!(rules[0].properties.len(), 2);
        assert_eq!(rules[0].properties[0].name, "bg");
        assert_eq!(rules[0].properties[0].value, "bg.elevated");
    }

    #[test]
    fn test_pseudo_state() {
        let rules = parse_style(".card:hover { bg: bg.elevated.hover; }").unwrap();
        assert_eq!(rules[0].selector.class, "card");
        assert_eq!(rules[0].selector.pseudo.as_deref(), Some("hover"));
    }

    #[test]
    fn test_multiple_rules() {
        let src = ".card { bg: bg.elevated; } .title { font: type.title; }";
        let rules = parse_style(src).unwrap();
        assert_eq!(rules.len(), 2);
    }
}
