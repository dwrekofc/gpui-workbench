//! Design token types and frozen token sets for Zed One Dark / One Light themes.
//!
//! Tokens are the single source of truth for all component colors, surfaces, and states.
//! Components must resolve colors through tokens — never hard-code color values.
//!
//! Color format: All colors stored as `gpui::Hsla`. Serialized as `#rrggbbaa` hex strings
//! via gpui's built-in Hsla serde implementation (which round-trips through Rgba).

use gpui::Hsla;
use serde::{Deserialize, Serialize};

/// Parse a `#RRGGBBAA` or `#RRGGBB` hex string into an `Hsla` color.
///
/// Uses gpui's built-in `Rgba::try_from(&str)` which handles `#rgb`, `#rgba`,
/// `#rrggbb`, and `#rrggbbaa` formats, then converts to Hsla.
pub fn parse_hex_color(hex: &str) -> Hsla {
    let rgba =
        gpui::Rgba::try_from(hex).unwrap_or_else(|e| panic!("invalid hex color '{hex}': {e}"));
    rgba.into()
}

// ---------------------------------------------------------------------------
// Token category structs
// ---------------------------------------------------------------------------

/// Border tokens: outlines, separators, and focus/selection indicators.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorderTokens {
    pub default: Hsla,
    pub variant: Hsla,
    pub focused: Hsla,
    pub selected: Hsla,
    pub transparent: Hsla,
    pub disabled: Hsla,
}

/// Surface and background tokens: app, panel, and elevated surfaces.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfaceTokens {
    pub background: Hsla,
    pub surface: Hsla,
    pub elevated_surface: Hsla,
}

/// Element interaction state tokens: backgrounds for different interactive states.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementTokens {
    pub background: Hsla,
    pub hover: Hsla,
    pub active: Hsla,
    pub selected: Hsla,
    pub disabled: Hsla,
}

/// Ghost element state tokens: transparent-background variants of element states.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GhostElementTokens {
    pub background: Hsla,
    pub hover: Hsla,
    pub active: Hsla,
    pub selected: Hsla,
    pub disabled: Hsla,
}

/// Text color tokens: primary, muted, placeholder, disabled, and accent text.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextTokens {
    pub default: Hsla,
    pub muted: Hsla,
    pub placeholder: Hsla,
    pub disabled: Hsla,
    pub accent: Hsla,
}

/// Icon color tokens: mirrors text token categories.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconTokens {
    pub default: Hsla,
    pub muted: Hsla,
    pub disabled: Hsla,
    pub placeholder: Hsla,
    pub accent: Hsla,
}

/// Semantic status color triplet: foreground, background, and border.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusColorTriplet {
    pub foreground: Hsla,
    pub background: Hsla,
    pub border: Hsla,
}

/// Semantic status tokens: error, warning, info, success, and hint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusTokens {
    pub error: StatusColorTriplet,
    pub warning: StatusColorTriplet,
    pub info: StatusColorTriplet,
    pub success: StatusColorTriplet,
    pub hint: StatusColorTriplet,
}

/// Tab and tab bar chrome tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabTokens {
    pub bar_background: Hsla,
    pub inactive_background: Hsla,
    pub active_background: Hsla,
}

/// Panel tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelTokens {
    pub background: Hsla,
    pub focused_border: Option<Hsla>,
}

/// Chrome/shell tokens: title bar, status bar, toolbar.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChromeTokens {
    pub title_bar_background: Hsla,
    pub status_bar_background: Hsla,
    pub toolbar_background: Hsla,
}

/// Scrollbar tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollbarTokens {
    pub thumb_background: Hsla,
    pub thumb_hover_background: Hsla,
    pub thumb_border: Hsla,
    pub track_background: Hsla,
    pub track_border: Hsla,
}

/// Player accent tokens (cursor, background, selection from players[0]).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerTokens {
    pub cursor: Hsla,
    pub background: Hsla,
    pub selection: Hsla,
}

/// Link tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkTokens {
    pub hover: Hsla,
}

// ---------------------------------------------------------------------------
// Top-level token set
// ---------------------------------------------------------------------------

/// Complete set of design tokens for a theme.
///
/// POC scope covers: border, surface, element/ghost states, text, icon,
/// status colors, tab/panel/chrome, scrollbar, player accent, and link tokens.
/// Editor, terminal, and syntax tokens are deferred to Phase 1+.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeTokens {
    pub name: String,
    pub appearance: ThemeAppearance,
    pub border: BorderTokens,
    pub surface: SurfaceTokens,
    pub element: ElementTokens,
    pub ghost_element: GhostElementTokens,
    pub text: TextTokens,
    pub icon: IconTokens,
    pub status: StatusTokens,
    pub tab: TabTokens,
    pub panel: PanelTokens,
    pub chrome: ChromeTokens,
    pub scrollbar: ScrollbarTokens,
    pub player: PlayerTokens,
    pub link: LinkTokens,
}

/// Theme appearance mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeAppearance {
    Dark,
    Light,
}

// ---------------------------------------------------------------------------
// Frozen token sets
// ---------------------------------------------------------------------------

/// Returns the frozen One Dark token set, extracted from Zed's `one.json`.
pub fn one_dark() -> ThemeTokens {
    ThemeTokens {
        name: "One Dark".into(),
        appearance: ThemeAppearance::Dark,
        border: BorderTokens {
            default: parse_hex_color("#464b57ff"),
            variant: parse_hex_color("#363c46ff"),
            focused: parse_hex_color("#47679eff"),
            selected: parse_hex_color("#293b5bff"),
            transparent: parse_hex_color("#00000000"),
            disabled: parse_hex_color("#414754ff"),
        },
        surface: SurfaceTokens {
            background: parse_hex_color("#3b414dff"),
            surface: parse_hex_color("#2f343eff"),
            elevated_surface: parse_hex_color("#2f343eff"),
        },
        element: ElementTokens {
            background: parse_hex_color("#2e343eff"),
            hover: parse_hex_color("#363c46ff"),
            active: parse_hex_color("#454a56ff"),
            selected: parse_hex_color("#454a56ff"),
            disabled: parse_hex_color("#2e343eff"),
        },
        ghost_element: GhostElementTokens {
            background: parse_hex_color("#00000000"),
            hover: parse_hex_color("#363c46ff"),
            active: parse_hex_color("#454a56ff"),
            selected: parse_hex_color("#454a56ff"),
            disabled: parse_hex_color("#2e343eff"),
        },
        text: TextTokens {
            default: parse_hex_color("#dce0e5ff"),
            muted: parse_hex_color("#a9afbcff"),
            placeholder: parse_hex_color("#878a98ff"),
            disabled: parse_hex_color("#878a98ff"),
            accent: parse_hex_color("#74ade8ff"),
        },
        icon: IconTokens {
            default: parse_hex_color("#dce0e5ff"),
            muted: parse_hex_color("#a9afbcff"),
            disabled: parse_hex_color("#878a98ff"),
            placeholder: parse_hex_color("#a9afbcff"),
            accent: parse_hex_color("#74ade8ff"),
        },
        status: StatusTokens {
            error: StatusColorTriplet {
                foreground: parse_hex_color("#d07277ff"),
                background: parse_hex_color("#d072771a"),
                border: parse_hex_color("#4c2b2cff"),
            },
            warning: StatusColorTriplet {
                foreground: parse_hex_color("#dec184ff"),
                background: parse_hex_color("#dec1841a"),
                border: parse_hex_color("#5d4c2fff"),
            },
            info: StatusColorTriplet {
                foreground: parse_hex_color("#74ade8ff"),
                background: parse_hex_color("#74ade81a"),
                border: parse_hex_color("#293b5bff"),
            },
            success: StatusColorTriplet {
                foreground: parse_hex_color("#a1c181ff"),
                background: parse_hex_color("#a1c1811a"),
                border: parse_hex_color("#38482fff"),
            },
            hint: StatusColorTriplet {
                foreground: parse_hex_color("#788ca6ff"),
                background: parse_hex_color("#5a6f891a"),
                border: parse_hex_color("#293b5bff"),
            },
        },
        tab: TabTokens {
            bar_background: parse_hex_color("#2f343eff"),
            inactive_background: parse_hex_color("#2f343eff"),
            active_background: parse_hex_color("#282c33ff"),
        },
        panel: PanelTokens {
            background: parse_hex_color("#2f343eff"),
            focused_border: None,
        },
        chrome: ChromeTokens {
            title_bar_background: parse_hex_color("#3b414dff"),
            status_bar_background: parse_hex_color("#3b414dff"),
            toolbar_background: parse_hex_color("#282c33ff"),
        },
        scrollbar: ScrollbarTokens {
            thumb_background: parse_hex_color("#c8ccd44c"),
            thumb_hover_background: parse_hex_color("#363c46ff"),
            thumb_border: parse_hex_color("#363c46ff"),
            track_background: parse_hex_color("#00000000"),
            track_border: parse_hex_color("#2e333cff"),
        },
        player: PlayerTokens {
            cursor: parse_hex_color("#74ade8ff"),
            background: parse_hex_color("#74ade8ff"),
            selection: parse_hex_color("#74ade83d"),
        },
        link: LinkTokens {
            hover: parse_hex_color("#74ade8ff"),
        },
    }
}

/// Returns the frozen One Light token set, extracted from Zed's `one.json`.
pub fn one_light() -> ThemeTokens {
    ThemeTokens {
        name: "One Light".into(),
        appearance: ThemeAppearance::Light,
        border: BorderTokens {
            default: parse_hex_color("#c9c9caff"),
            variant: parse_hex_color("#dfdfe0ff"),
            focused: parse_hex_color("#7d82e8ff"),
            selected: parse_hex_color("#cbcdf6ff"),
            transparent: parse_hex_color("#00000000"),
            disabled: parse_hex_color("#d3d3d4ff"),
        },
        surface: SurfaceTokens {
            background: parse_hex_color("#dcdcddff"),
            surface: parse_hex_color("#ebebecff"),
            elevated_surface: parse_hex_color("#ebebecff"),
        },
        element: ElementTokens {
            background: parse_hex_color("#ebebecff"),
            hover: parse_hex_color("#dfdfe0ff"),
            active: parse_hex_color("#cacacaff"),
            selected: parse_hex_color("#cacacaff"),
            disabled: parse_hex_color("#ebebecff"),
        },
        ghost_element: GhostElementTokens {
            background: parse_hex_color("#00000000"),
            hover: parse_hex_color("#dfdfe0ff"),
            active: parse_hex_color("#cacacaff"),
            selected: parse_hex_color("#cacacaff"),
            disabled: parse_hex_color("#ebebecff"),
        },
        text: TextTokens {
            default: parse_hex_color("#242529ff"),
            muted: parse_hex_color("#58585aff"),
            placeholder: parse_hex_color("#7e8086ff"),
            disabled: parse_hex_color("#7e8086ff"),
            accent: parse_hex_color("#5c78e2ff"),
        },
        icon: IconTokens {
            default: parse_hex_color("#242529ff"),
            muted: parse_hex_color("#58585aff"),
            disabled: parse_hex_color("#7e8086ff"),
            placeholder: parse_hex_color("#58585aff"),
            accent: parse_hex_color("#5c78e2ff"),
        },
        status: StatusTokens {
            error: StatusColorTriplet {
                foreground: parse_hex_color("#d36151ff"),
                background: parse_hex_color("#fbdfd9ff"),
                border: parse_hex_color("#f6c6bdff"),
            },
            warning: StatusColorTriplet {
                foreground: parse_hex_color("#a48819ff"),
                background: parse_hex_color("#faf2e6ff"),
                border: parse_hex_color("#f4e7d1ff"),
            },
            info: StatusColorTriplet {
                foreground: parse_hex_color("#5c78e2ff"),
                background: parse_hex_color("#e2e2faff"),
                border: parse_hex_color("#cbcdf6ff"),
            },
            success: StatusColorTriplet {
                foreground: parse_hex_color("#669f59ff"),
                background: parse_hex_color("#dfeadbff"),
                border: parse_hex_color("#c8dcc1ff"),
            },
            hint: StatusColorTriplet {
                foreground: parse_hex_color("#7274a7ff"),
                background: parse_hex_color("#e2e2faff"),
                border: parse_hex_color("#cbcdf6ff"),
            },
        },
        tab: TabTokens {
            bar_background: parse_hex_color("#ebebecff"),
            inactive_background: parse_hex_color("#ebebecff"),
            active_background: parse_hex_color("#fafafaff"),
        },
        panel: PanelTokens {
            background: parse_hex_color("#ebebecff"),
            focused_border: None,
        },
        chrome: ChromeTokens {
            title_bar_background: parse_hex_color("#dcdcddff"),
            status_bar_background: parse_hex_color("#dcdcddff"),
            toolbar_background: parse_hex_color("#fafafaff"),
        },
        scrollbar: ScrollbarTokens {
            thumb_background: parse_hex_color("#383a414c"),
            thumb_hover_background: parse_hex_color("#dfdfe0ff"),
            thumb_border: parse_hex_color("#dfdfe0ff"),
            track_background: parse_hex_color("#00000000"),
            track_border: parse_hex_color("#eeeeeeff"),
        },
        player: PlayerTokens {
            cursor: parse_hex_color("#5c78e2ff"),
            background: parse_hex_color("#5c78e2ff"),
            selection: parse_hex_color("#5c78e23d"),
        },
        link: LinkTokens {
            hover: parse_hex_color("#5c78e2ff"),
        },
    }
}

// ---------------------------------------------------------------------------
// Token-to-Zed-JSON key mapping table
// ---------------------------------------------------------------------------

/// Mapping from internal token paths to Zed theme JSON keys.
///
/// This table documents the correspondence between our semantic token identifiers
/// and their source keys in `one.json`. Used for provenance tracking and future
/// theme import/export.
pub const TOKEN_MAPPING: &[(&str, &str)] = &[
    // Border
    ("border.default", "border"),
    ("border.variant", "border.variant"),
    ("border.focused", "border.focused"),
    ("border.selected", "border.selected"),
    ("border.transparent", "border.transparent"),
    ("border.disabled", "border.disabled"),
    // Surface
    ("surface.background", "background"),
    ("surface.surface", "surface.background"),
    ("surface.elevated_surface", "elevated_surface.background"),
    // Element states
    ("element.background", "element.background"),
    ("element.hover", "element.hover"),
    ("element.active", "element.active"),
    ("element.selected", "element.selected"),
    ("element.disabled", "element.disabled"),
    // Ghost element states
    ("ghost_element.background", "ghost_element.background"),
    ("ghost_element.hover", "ghost_element.hover"),
    ("ghost_element.active", "ghost_element.active"),
    ("ghost_element.selected", "ghost_element.selected"),
    ("ghost_element.disabled", "ghost_element.disabled"),
    // Text
    ("text.default", "text"),
    ("text.muted", "text.muted"),
    ("text.placeholder", "text.placeholder"),
    ("text.disabled", "text.disabled"),
    ("text.accent", "text.accent"),
    // Icon
    ("icon.default", "icon"),
    ("icon.muted", "icon.muted"),
    ("icon.disabled", "icon.disabled"),
    ("icon.placeholder", "icon.placeholder"),
    ("icon.accent", "icon.accent"),
    // Status: error
    ("status.error.foreground", "error"),
    ("status.error.background", "error.background"),
    ("status.error.border", "error.border"),
    // Status: warning
    ("status.warning.foreground", "warning"),
    ("status.warning.background", "warning.background"),
    ("status.warning.border", "warning.border"),
    // Status: info
    ("status.info.foreground", "info"),
    ("status.info.background", "info.background"),
    ("status.info.border", "info.border"),
    // Status: success
    ("status.success.foreground", "success"),
    ("status.success.background", "success.background"),
    ("status.success.border", "success.border"),
    // Status: hint
    ("status.hint.foreground", "hint"),
    ("status.hint.background", "hint.background"),
    ("status.hint.border", "hint.border"),
    // Tab
    ("tab.bar_background", "tab_bar.background"),
    ("tab.inactive_background", "tab.inactive_background"),
    ("tab.active_background", "tab.active_background"),
    // Panel
    ("panel.background", "panel.background"),
    ("panel.focused_border", "panel.focused_border"),
    // Chrome
    ("chrome.title_bar_background", "title_bar.background"),
    ("chrome.status_bar_background", "status_bar.background"),
    ("chrome.toolbar_background", "toolbar.background"),
    // Scrollbar
    ("scrollbar.thumb_background", "scrollbar.thumb.background"),
    (
        "scrollbar.thumb_hover_background",
        "scrollbar.thumb.hover_background",
    ),
    ("scrollbar.thumb_border", "scrollbar.thumb.border"),
    ("scrollbar.track_background", "scrollbar.track.background"),
    ("scrollbar.track_border", "scrollbar.track.border"),
    // Player
    ("player.cursor", "players[0].cursor"),
    ("player.background", "players[0].background"),
    ("player.selection", "players[0].selection"),
    // Link
    ("link.hover", "link_text.hover"),
];

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_dark_loads_without_panic() {
        let tokens = one_dark();
        assert_eq!(tokens.name, "One Dark");
        assert_eq!(tokens.appearance, ThemeAppearance::Dark);
    }

    #[test]
    fn one_light_loads_without_panic() {
        let tokens = one_light();
        assert_eq!(tokens.name, "One Light");
        assert_eq!(tokens.appearance, ThemeAppearance::Light);
    }

    #[test]
    fn token_sets_differ_in_appearance() {
        let dark = one_dark();
        let light = one_light();
        assert_ne!(dark.appearance, light.appearance);
    }

    #[test]
    fn dark_text_is_light_colored() {
        let dark = one_dark();
        // One Dark text (#dce0e5ff) has high lightness
        let rgba: gpui::Rgba = dark.text.default.into();
        assert!(rgba.r > 0.8, "Dark theme default text should be light");
    }

    #[test]
    fn light_text_is_dark_colored() {
        let light = one_light();
        // One Light text (#242529ff) has low lightness
        let rgba: gpui::Rgba = light.text.default.into();
        assert!(rgba.r < 0.2, "Light theme default text should be dark");
    }

    #[test]
    fn transparent_border_is_zero_alpha() {
        let dark = one_dark();
        let rgba: gpui::Rgba = dark.border.transparent.into();
        assert!(rgba.a < 0.01, "Transparent border should have zero alpha");
    }

    #[test]
    fn null_panel_focused_border() {
        let dark = one_dark();
        assert!(dark.panel.focused_border.is_none());
        let light = one_light();
        assert!(light.panel.focused_border.is_none());
    }

    #[test]
    fn json_serialization_round_trip() {
        let dark = one_dark();
        let json = serde_json::to_string_pretty(&dark).expect("serialize");
        let restored: ThemeTokens = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(restored.name, dark.name);
        assert_eq!(restored.appearance, dark.appearance);
        // Verify a specific token round-trips (border.default)
        let orig_rgba: gpui::Rgba = dark.border.default.into();
        let rest_rgba: gpui::Rgba = restored.border.default.into();
        assert!((orig_rgba.r - rest_rgba.r).abs() < 0.01);
        assert!((orig_rgba.g - rest_rgba.g).abs() < 0.01);
        assert!((orig_rgba.b - rest_rgba.b).abs() < 0.01);
        assert!((orig_rgba.a - rest_rgba.a).abs() < 0.01);
    }

    #[test]
    fn toml_serialization_round_trip() {
        let light = one_light();
        let toml_str = toml::to_string_pretty(&light).expect("serialize to toml");
        let restored: ThemeTokens = toml::from_str(&toml_str).expect("deserialize from toml");
        assert_eq!(restored.name, light.name);
        assert_eq!(restored.appearance, light.appearance);
    }

    #[test]
    fn token_mapping_table_covers_all_categories() {
        // Verify token mapping table has entries for all major categories
        let categories: Vec<&str> = TOKEN_MAPPING
            .iter()
            .map(|(internal, _)| internal.split('.').next().unwrap())
            .collect();
        for expected in &[
            "border",
            "surface",
            "element",
            "ghost_element",
            "text",
            "icon",
            "status",
            "tab",
            "panel",
            "chrome",
            "scrollbar",
            "player",
            "link",
        ] {
            assert!(
                categories.contains(expected),
                "Token mapping missing category: {expected}"
            );
        }
    }

    #[test]
    fn parse_hex_color_valid() {
        let color = parse_hex_color("#ff0000ff");
        let rgba: gpui::Rgba = color.into();
        assert!((rgba.r - 1.0).abs() < 0.01);
        assert!(rgba.g < 0.01);
        assert!(rgba.b < 0.01);
        assert!((rgba.a - 1.0).abs() < 0.01);
    }

    #[test]
    #[should_panic(expected = "invalid hex color")]
    fn parse_hex_color_invalid() {
        parse_hex_color("not-a-color");
    }

    #[test]
    fn status_tokens_have_distinct_foreground_colors() {
        let dark = one_dark();
        let error_rgba: gpui::Rgba = dark.status.error.foreground.into();
        let success_rgba: gpui::Rgba = dark.status.success.foreground.into();
        // Error is red-ish (high r), success is green-ish (high g)
        assert!(error_rgba.r > error_rgba.g);
        assert!(success_rgba.g > success_rgba.r);
    }
}
