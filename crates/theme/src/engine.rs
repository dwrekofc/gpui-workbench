//! Theme engine: global state management, registry, switching, and import/export.
//!
//! The engine provides two GPUI globals:
//! - [`Theme`] -- the currently active theme, wrapping [`ThemeTokens`] with `Deref` for
//!   direct color access (e.g. `cx.theme().border.default`).
//! - [`ThemeRegistry`] -- the collection of all loaded themes, keyed by name.
//!
//! The [`ActiveTheme`] extension trait adds an ergonomic `.theme()` accessor to
//! `gpui::App` (and any other context type you extend it for).

use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use gpui::{App, Global, Hsla};
use serde_json;

use crate::tokens::{self, ThemeTokens, parse_hex_color};

// ---------------------------------------------------------------------------
// Theme (active theme global)
// ---------------------------------------------------------------------------

/// The currently active theme, stored as a GPUI global.
///
/// `Theme` wraps [`ThemeTokens`] and implements `Deref`/`DerefMut` to it,
/// allowing direct field access like `theme.border.default`.
#[derive(Debug, Clone)]
pub struct Theme {
    tokens: ThemeTokens,
}

impl Global for Theme {}

impl Deref for Theme {
    type Target = ThemeTokens;

    fn deref(&self) -> &Self::Target {
        &self.tokens
    }
}

impl DerefMut for Theme {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tokens
    }
}

impl Theme {
    /// Create a new `Theme` wrapping the given token set.
    pub fn new(tokens: ThemeTokens) -> Self {
        Self { tokens }
    }

    /// Returns a reference to the global `Theme`.
    #[inline(always)]
    pub fn global(cx: &App) -> &Theme {
        cx.global::<Theme>()
    }

    /// Returns a mutable reference to the global `Theme`.
    #[inline(always)]
    pub fn global_mut(cx: &mut App) -> &mut Theme {
        cx.global_mut::<Theme>()
    }

    /// Returns a reference to the inner [`ThemeTokens`].
    pub fn tokens(&self) -> &ThemeTokens {
        &self.tokens
    }

    /// Returns a mutable reference to the inner [`ThemeTokens`].
    pub fn tokens_mut(&mut self) -> &mut ThemeTokens {
        &mut self.tokens
    }

    // -- Theme switching ---------------------------------------------------

    /// Switch to a named theme from the registry.
    ///
    /// Looks up the theme by `name` in the [`ThemeRegistry`] global, replaces
    /// the active tokens, and refreshes all windows so components re-render.
    ///
    /// Returns `Err` if no theme with the given name exists in the registry.
    pub fn change(name: &str, cx: &mut App) -> Result<(), ThemeError> {
        let registry = cx.global::<ThemeRegistry>();
        let tokens = registry
            .get(name)
            .ok_or_else(|| ThemeError::NotFound(name.to_string()))?
            .clone();

        let theme = cx.global_mut::<Theme>();
        theme.tokens = tokens;

        cx.refresh_windows();
        Ok(())
    }

    // -- Token mutation ----------------------------------------------------

    /// Set an individual token value by dot-path (e.g. `"border.default"`).
    ///
    /// The path must match one of the internal token paths defined in
    /// [`tokens::TOKEN_MAPPING`]. The color is specified as a `#RRGGBB` or
    /// `#RRGGBBAA` hex string.
    ///
    /// After mutation the global is updated in place. The caller should call
    /// `cx.refresh_windows()` (or otherwise trigger a re-render) to see the
    /// change reflected in the UI.
    ///
    /// Returns `Err` if the path is not recognized or the hex string is invalid.
    pub fn set_token(path: &str, hex: &str, cx: &mut App) -> Result<(), ThemeError> {
        // Validate the hex first (panics on invalid, so we catch it).
        let color = std::panic::catch_unwind(|| parse_hex_color(hex))
            .map_err(|_| ThemeError::InvalidColor(hex.to_string()))?;

        let theme = cx.global_mut::<Theme>();
        set_token_by_path(&mut theme.tokens, path, color)?;
        cx.refresh_windows();
        Ok(())
    }

    // -- Import / Export ---------------------------------------------------

    /// Import a theme from a JSON string, returning a [`ThemeTokens`].
    ///
    /// The JSON must conform to the serde representation of [`ThemeTokens`].
    pub fn import_json(json: &str) -> Result<ThemeTokens, ThemeError> {
        serde_json::from_str(json).map_err(|e| ThemeError::Import(format!("JSON: {e}")))
    }

    /// Export the active theme to a pretty-printed JSON string.
    pub fn export_json(&self) -> Result<String, ThemeError> {
        serde_json::to_string_pretty(&self.tokens)
            .map_err(|e| ThemeError::Export(format!("JSON: {e}")))
    }

    /// Import a theme from a TOML string, returning a [`ThemeTokens`].
    pub fn import_toml(toml_str: &str) -> Result<ThemeTokens, ThemeError> {
        toml::from_str(toml_str).map_err(|e| ThemeError::Import(format!("TOML: {e}")))
    }

    /// Export the active theme to a pretty-printed TOML string.
    pub fn export_toml(&self) -> Result<String, ThemeError> {
        toml::to_string_pretty(&self.tokens)
            .map_err(|e| ThemeError::Export(format!("TOML: {e}")))
    }
}

// ---------------------------------------------------------------------------
// ThemeRegistry (global registry of loaded themes)
// ---------------------------------------------------------------------------

/// Registry of all loaded themes, keyed by name.
///
/// Stored as a GPUI global. Themes can be added at startup (via [`init`]) or
/// dynamically at runtime.
#[derive(Debug, Clone, Default)]
pub struct ThemeRegistry {
    themes: HashMap<String, ThemeTokens>,
}

impl Global for ThemeRegistry {}

impl ThemeRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a reference to the global registry.
    #[inline(always)]
    pub fn global(cx: &App) -> &ThemeRegistry {
        cx.global::<ThemeRegistry>()
    }

    /// Returns a mutable reference to the global registry.
    #[inline(always)]
    pub fn global_mut(cx: &mut App) -> &mut ThemeRegistry {
        cx.global_mut::<ThemeRegistry>()
    }

    /// Register a theme. Overwrites any existing theme with the same name.
    pub fn register(&mut self, tokens: ThemeTokens) {
        self.themes.insert(tokens.name.clone(), tokens);
    }

    /// Look up a theme by name.
    pub fn get(&self, name: &str) -> Option<&ThemeTokens> {
        self.themes.get(name)
    }

    /// Remove a theme by name. Returns the removed tokens, if any.
    pub fn remove(&mut self, name: &str) -> Option<ThemeTokens> {
        self.themes.remove(name)
    }

    /// Returns the number of registered themes.
    pub fn len(&self) -> usize {
        self.themes.len()
    }

    /// Returns `true` if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.themes.is_empty()
    }

    /// Returns an iterator over all registered theme names.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.themes.keys().map(|s| s.as_str())
    }

    /// Returns a reference to the inner map.
    pub fn themes(&self) -> &HashMap<String, ThemeTokens> {
        &self.themes
    }
}

// ---------------------------------------------------------------------------
// ActiveTheme trait
// ---------------------------------------------------------------------------

/// Extension trait providing ergonomic `.theme()` access on GPUI context types.
///
/// ```ignore
/// use theme::ActiveTheme;
/// let bg = cx.theme().surface.background;
/// ```
pub trait ActiveTheme {
    /// Returns a reference to the active [`Theme`] global.
    fn theme(&self) -> &Theme;
}

impl ActiveTheme for App {
    #[inline(always)]
    fn theme(&self) -> &Theme {
        Theme::global(self)
    }
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// Errors that can occur during theme operations.
#[derive(Debug, Clone)]
pub enum ThemeError {
    /// No theme with the given name exists in the registry.
    NotFound(String),
    /// A token path was not recognized.
    UnknownTokenPath(String),
    /// A hex color string was invalid.
    InvalidColor(String),
    /// An error occurred during theme import.
    Import(String),
    /// An error occurred during theme export.
    Export(String),
}

impl std::fmt::Display for ThemeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeError::NotFound(name) => write!(f, "theme not found: '{name}'"),
            ThemeError::UnknownTokenPath(path) => write!(f, "unknown token path: '{path}'"),
            ThemeError::InvalidColor(hex) => write!(f, "invalid hex color: '{hex}'"),
            ThemeError::Import(msg) => write!(f, "import error: {msg}"),
            ThemeError::Export(msg) => write!(f, "export error: {msg}"),
        }
    }
}

impl std::error::Error for ThemeError {}

// ---------------------------------------------------------------------------
// Token mutation helper
// ---------------------------------------------------------------------------

/// Set a single color token on a [`ThemeTokens`] by dot-path.
///
/// Supported paths correspond to the internal token paths from
/// [`tokens::TOKEN_MAPPING`], e.g. `"border.default"`, `"text.muted"`,
/// `"status.error.foreground"`.
fn set_token_by_path(
    tokens: &mut ThemeTokens,
    path: &str,
    color: Hsla,
) -> Result<(), ThemeError> {
    match path {
        // Border
        "border.default" => tokens.border.default = color,
        "border.variant" => tokens.border.variant = color,
        "border.focused" => tokens.border.focused = color,
        "border.selected" => tokens.border.selected = color,
        "border.transparent" => tokens.border.transparent = color,
        "border.disabled" => tokens.border.disabled = color,

        // Surface
        "surface.background" => tokens.surface.background = color,
        "surface.surface" => tokens.surface.surface = color,
        "surface.elevated_surface" => tokens.surface.elevated_surface = color,

        // Element
        "element.background" => tokens.element.background = color,
        "element.hover" => tokens.element.hover = color,
        "element.active" => tokens.element.active = color,
        "element.selected" => tokens.element.selected = color,
        "element.disabled" => tokens.element.disabled = color,

        // Ghost element
        "ghost_element.background" => tokens.ghost_element.background = color,
        "ghost_element.hover" => tokens.ghost_element.hover = color,
        "ghost_element.active" => tokens.ghost_element.active = color,
        "ghost_element.selected" => tokens.ghost_element.selected = color,
        "ghost_element.disabled" => tokens.ghost_element.disabled = color,

        // Text
        "text.default" => tokens.text.default = color,
        "text.muted" => tokens.text.muted = color,
        "text.placeholder" => tokens.text.placeholder = color,
        "text.disabled" => tokens.text.disabled = color,
        "text.accent" => tokens.text.accent = color,

        // Icon
        "icon.default" => tokens.icon.default = color,
        "icon.muted" => tokens.icon.muted = color,
        "icon.disabled" => tokens.icon.disabled = color,
        "icon.placeholder" => tokens.icon.placeholder = color,
        "icon.accent" => tokens.icon.accent = color,

        // Status
        "status.error.foreground" => tokens.status.error.foreground = color,
        "status.error.background" => tokens.status.error.background = color,
        "status.error.border" => tokens.status.error.border = color,
        "status.warning.foreground" => tokens.status.warning.foreground = color,
        "status.warning.background" => tokens.status.warning.background = color,
        "status.warning.border" => tokens.status.warning.border = color,
        "status.info.foreground" => tokens.status.info.foreground = color,
        "status.info.background" => tokens.status.info.background = color,
        "status.info.border" => tokens.status.info.border = color,
        "status.success.foreground" => tokens.status.success.foreground = color,
        "status.success.background" => tokens.status.success.background = color,
        "status.success.border" => tokens.status.success.border = color,
        "status.hint.foreground" => tokens.status.hint.foreground = color,
        "status.hint.background" => tokens.status.hint.background = color,
        "status.hint.border" => tokens.status.hint.border = color,

        // Tab
        "tab.bar_background" => tokens.tab.bar_background = color,
        "tab.inactive_background" => tokens.tab.inactive_background = color,
        "tab.active_background" => tokens.tab.active_background = color,

        // Panel
        "panel.background" => tokens.panel.background = color,
        "panel.focused_border" => tokens.panel.focused_border = Some(color),

        // Chrome
        "chrome.title_bar_background" => tokens.chrome.title_bar_background = color,
        "chrome.status_bar_background" => tokens.chrome.status_bar_background = color,
        "chrome.toolbar_background" => tokens.chrome.toolbar_background = color,

        // Scrollbar
        "scrollbar.thumb_background" => tokens.scrollbar.thumb_background = color,
        "scrollbar.thumb_hover_background" => tokens.scrollbar.thumb_hover_background = color,
        "scrollbar.thumb_border" => tokens.scrollbar.thumb_border = color,
        "scrollbar.track_background" => tokens.scrollbar.track_background = color,
        "scrollbar.track_border" => tokens.scrollbar.track_border = color,

        // Player
        "player.cursor" => tokens.player.cursor = color,
        "player.background" => tokens.player.background = color,
        "player.selection" => tokens.player.selection = color,

        // Link
        "link.hover" => tokens.link.hover = color,

        _ => return Err(ThemeError::UnknownTokenPath(path.to_string())),
    }
    Ok(())
}

/// Returns the list of all supported token dot-paths for [`set_token_by_path`].
///
/// Useful for UI introspection, autocomplete, or validation.
pub fn all_token_paths() -> Vec<&'static str> {
    tokens::TOKEN_MAPPING
        .iter()
        .map(|(internal, _)| *internal)
        .collect()
}

// ---------------------------------------------------------------------------
// Initialization
// ---------------------------------------------------------------------------

/// Initialize the theme engine by registering GPUI globals.
///
/// This function:
/// 1. Creates and sets the [`ThemeRegistry`] global with One Dark and One Light.
/// 2. Creates and sets the [`Theme`] global with One Dark as the default.
///
/// Must be called during app startup before any component tries to access `cx.theme()`.
pub fn init(cx: &mut App) {
    let mut registry = ThemeRegistry::new();
    registry.register(tokens::one_dark());
    registry.register(tokens::one_light());
    cx.set_global(registry);

    let theme = Theme::new(tokens::one_dark());
    cx.set_global(theme);
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::{ThemeAppearance, one_dark, one_light};

    #[test]
    fn theme_deref_provides_token_access() {
        let theme = Theme::new(one_dark());
        // Access via Deref -- should compile and not panic.
        assert_eq!(theme.name, "One Dark");
        assert_eq!(theme.appearance, ThemeAppearance::Dark);
        let _color = theme.border.default;
    }

    #[test]
    fn theme_deref_mut_allows_modification() {
        let mut theme = Theme::new(one_dark());
        let original = theme.border.default;
        theme.border.default = parse_hex_color("#ff0000ff");
        assert_ne!(theme.border.default, original);
    }

    #[test]
    fn registry_register_and_get() {
        let mut registry = ThemeRegistry::new();
        assert!(registry.is_empty());

        registry.register(one_dark());
        registry.register(one_light());

        assert_eq!(registry.len(), 2);
        assert!(registry.get("One Dark").is_some());
        assert!(registry.get("One Light").is_some());
        assert!(registry.get("Nonexistent").is_none());
    }

    #[test]
    fn registry_remove() {
        let mut registry = ThemeRegistry::new();
        registry.register(one_dark());
        assert_eq!(registry.len(), 1);

        let removed = registry.remove("One Dark");
        assert!(removed.is_some());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn registry_names() {
        let mut registry = ThemeRegistry::new();
        registry.register(one_dark());
        registry.register(one_light());

        let mut names: Vec<&str> = registry.names().collect();
        names.sort();
        assert_eq!(names, vec!["One Dark", "One Light"]);
    }

    #[test]
    fn registry_overwrite() {
        let mut registry = ThemeRegistry::new();
        registry.register(one_dark());

        // Register again with same name -- should overwrite, not duplicate.
        registry.register(one_dark());
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn set_token_by_path_known_paths() {
        let mut tokens = one_dark();
        let red = parse_hex_color("#ff0000ff");

        // Test a sampling of paths from each category.
        let paths = [
            "border.default",
            "surface.background",
            "element.hover",
            "ghost_element.active",
            "text.muted",
            "icon.accent",
            "status.error.foreground",
            "status.warning.background",
            "tab.active_background",
            "panel.background",
            "panel.focused_border",
            "chrome.title_bar_background",
            "scrollbar.thumb_background",
            "player.cursor",
            "link.hover",
        ];

        for path in &paths {
            let result = set_token_by_path(&mut tokens, path, red);
            assert!(result.is_ok(), "set_token_by_path failed for '{path}'");
        }
    }

    #[test]
    fn set_token_by_path_unknown() {
        let mut tokens = one_dark();
        let red = parse_hex_color("#ff0000ff");
        let result = set_token_by_path(&mut tokens, "nonexistent.path", red);
        assert!(result.is_err());
        match result.unwrap_err() {
            ThemeError::UnknownTokenPath(p) => assert_eq!(p, "nonexistent.path"),
            other => panic!("expected UnknownTokenPath, got: {other}"),
        }
    }

    #[test]
    fn all_token_paths_covers_mapping() {
        let paths = all_token_paths();
        // Should match the number of entries in TOKEN_MAPPING.
        assert_eq!(
            paths.len(),
            crate::tokens::TOKEN_MAPPING.len(),
            "all_token_paths count should match TOKEN_MAPPING"
        );
    }

    #[test]
    fn all_token_paths_are_settable() {
        let mut tokens = one_dark();
        let color = parse_hex_color("#aabbccff");
        for path in all_token_paths() {
            let result = set_token_by_path(&mut tokens, path, color);
            assert!(
                result.is_ok(),
                "TOKEN_MAPPING path '{path}' is not handled by set_token_by_path"
            );
        }
    }

    #[test]
    fn json_import_export_round_trip() {
        let theme = Theme::new(one_dark());
        let json = theme.export_json().expect("export_json");
        let imported = Theme::import_json(&json).expect("import_json");
        assert_eq!(imported.name, "One Dark");
        assert_eq!(imported.appearance, ThemeAppearance::Dark);

        // Verify a token round-trips.
        let orig_rgba: gpui::Rgba = theme.border.default.into();
        let imported_rgba: gpui::Rgba = imported.border.default.into();
        assert!((orig_rgba.r - imported_rgba.r).abs() < 0.01);
    }

    #[test]
    fn toml_import_export_round_trip() {
        let theme = Theme::new(one_light());
        let toml_str = theme.export_toml().expect("export_toml");
        let imported = Theme::import_toml(&toml_str).expect("import_toml");
        assert_eq!(imported.name, "One Light");
        assert_eq!(imported.appearance, ThemeAppearance::Light);
    }

    #[test]
    fn json_import_invalid() {
        let result = Theme::import_json("{ not valid json");
        assert!(result.is_err());
    }

    #[test]
    fn toml_import_invalid() {
        let result = Theme::import_toml("[[[ not valid toml");
        assert!(result.is_err());
    }

    #[test]
    fn theme_error_display() {
        let err = ThemeError::NotFound("Foo".into());
        assert_eq!(err.to_string(), "theme not found: 'Foo'");

        let err = ThemeError::UnknownTokenPath("bad.path".into());
        assert_eq!(err.to_string(), "unknown token path: 'bad.path'");

        let err = ThemeError::InvalidColor("xyz".into());
        assert_eq!(err.to_string(), "invalid hex color: 'xyz'");
    }
}
