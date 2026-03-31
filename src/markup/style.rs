// Style block parser — parses the <style> section of a .mgt file.
//
// Supports:
// - Class rules:         .card { bg: bg.elevated; radius: radius.lg; }
// - Pseudo states:       .card:hover { bg: bg.elevated.hover; }
// - Token references:    color: fg.primary;  (resolved at runtime via ThemeContext)
// - Scoped by default:   rules apply only to the component they are defined in
