use anyhow::Result;

/// A plugin that knows how to load and run a specific scripting language.
///
/// Implement this trait in a separate crate (e.g. `mgt-lua`, `mgt-rhai`) and
/// register it with the runtime's `ScriptRegistry` at startup.
///
/// The built-in `"mgt"` lang is handled by the runtime directly and does not
/// require a plugin.
pub trait ScriptBackend {
    /// The language identifier this backend handles, e.g. `"lua"`, `"rhai"`.
    fn lang(&self) -> &str;

    /// Parse and load the raw script source into a live component state.
    fn load(&self, source: &str) -> Result<Box<dyn ComponentState>>;
}

/// Live runtime state for a component instance produced by a ScriptBackend.
pub trait ComponentState {
    /// Read a named prop or state value as a string.
    fn get(&self, key: &str) -> Option<String>;

    /// Write a named state value.
    fn set(&mut self, key: &str, value: String);

    /// Invoke a named event handler function.
    fn call(&mut self, fn_name: &str) -> Result<()>;
}

/// Runtime registry of installed scripting backends, keyed by lang name.
#[derive(Default)]
pub struct ScriptRegistry {
    backends: Vec<Box<dyn ScriptBackend>>,
}

impl ScriptRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a scripting backend plugin.
    pub fn register(&mut self, backend: Box<dyn ScriptBackend>) {
        self.backends.push(backend);
    }

    /// Look up a backend by lang name.
    pub fn get(&self, lang: &str) -> Option<&dyn ScriptBackend> {
        self.backends
            .iter()
            .find(|b| b.lang() == lang)
            .map(|b| b.as_ref())
    }
}
