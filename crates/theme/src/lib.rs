pub mod engine;
pub mod tokens;

pub use engine::{ActiveTheme, Theme, ThemeError, ThemeRegistry};
pub use tokens::{
    BorderTokens, ChromeTokens, ElementTokens, GhostElementTokens, IconTokens, LinkTokens,
    PanelTokens, PlayerTokens, ScrollbarTokens, StatusColorTriplet, StatusTokens, SurfaceTokens,
    TabTokens, TextTokens, ThemeAppearance, ThemeTokens,
};

/// Initialize the theme engine.
///
/// Registers the [`ThemeRegistry`] and [`Theme`] globals with GPUI,
/// loads the built-in One Dark and One Light themes, and sets One Dark
/// as the active default.
///
/// Must be called during app startup before any component accesses `cx.theme()`.
pub fn init(cx: &mut gpui::App) {
    engine::init(cx);
}
