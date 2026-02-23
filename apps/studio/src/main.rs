//! GPUI Workbench Studio — component explorer, story renderer, and theme editor.
//!
//! This is the P0.5-12 deliverable: a desktop GPUI application that serves as
//! the visual validation environment for the component library. It renders
//! component stories, supports theme switching, and provides a live token editor.

use gpui::prelude::FluentBuilder;
use gpui::*;
use story::StoryRegistry;
use theme::{ActiveTheme, Theme, ThemeAppearance};

// ---------------------------------------------------------------------------
// StudioApp — the root view
// ---------------------------------------------------------------------------

/// The root workbench view, holding all application state.
///
/// Implements `Render` (not `RenderOnce`) because it is a persistent stateful
/// view that tracks selected story, theme state, and token editor visibility.
struct StudioApp {
    /// Index of the currently selected story in the StoryRegistry.
    selected_story_index: Option<usize>,
    /// Whether the token editor panel is visible.
    show_token_editor: bool,
    /// Whether the metadata panel is visible.
    show_metadata: bool,
    // search_query reserved for future story filtering (Phase 1)
    /// Token editor: which token path is being edited (if any).
    editing_token_path: Option<String>,
    /// Token editor: the hex value being typed.
    editing_token_value: String,
}

impl StudioApp {
    fn new() -> Self {
        Self {
            selected_story_index: Some(0), // Select first story by default
            show_token_editor: false,
            show_metadata: false,
            editing_token_path: None,
            editing_token_value: String::new(),
        }
    }

    /// Toggle between One Dark and One Light themes.
    fn toggle_theme(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let current_appearance = cx.theme().appearance;
        let target = match current_appearance {
            ThemeAppearance::Dark => "One Light",
            ThemeAppearance::Light => "One Dark",
        };
        if let Err(e) = Theme::change(target, cx) {
            log::error!("Failed to switch theme: {}", e);
        }
        cx.notify();
    }

    /// Apply a token edit from the token editor.
    fn apply_token_edit(&mut self, cx: &mut Context<Self>) {
        if let Some(ref path) = self.editing_token_path {
            let hex = self.editing_token_value.trim();
            if !hex.is_empty() {
                match Theme::set_token(path, hex, cx) {
                    Ok(()) => {
                        log::info!("Token '{}' set to '{}'", path, hex);
                    }
                    Err(e) => {
                        log::error!("Failed to set token '{}': {}", path, e);
                    }
                }
            }
        }
        self.editing_token_path = None;
        self.editing_token_value.clear();
        cx.notify();
    }

    // -- Rendering helpers -------------------------------------------------

    /// Render the top toolbar with theme toggle and panel toggles.
    fn render_toolbar(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let theme_name: SharedString = theme.name.clone().into();
        let is_dark = theme.appearance == ThemeAppearance::Dark;

        div()
            .id("toolbar")
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .h_10()
            .px_4()
            .bg(theme.chrome.title_bar_background)
            .border_b_1()
            .border_color(theme.border.default)
            // Left: App title
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.text.default)
                            .child("GPUI Workbench"),
                    )
                    .child(div().text_xs().text_color(theme.text.muted).child("v0.1.0")),
            )
            // Right: Controls
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_3()
                    // Theme toggle button
                    .child(
                        div()
                            .id("theme-toggle")
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_1()
                            .px_3()
                            .py_1()
                            .bg(theme.element.background)
                            .border_1()
                            .border_color(theme.border.default)
                            .rounded_md()
                            .cursor_pointer()
                            .hover(|s| s.bg(theme.element.hover))
                            .on_mouse_down(MouseButton::Left, {
                                cx.listener(|this, _event, window, cx| {
                                    this.toggle_theme(window, cx);
                                })
                            })
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text.default)
                                    .child(if is_dark { "Dark" } else { "Light" }),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text.muted)
                                    .child(theme_name),
                            ),
                    )
                    // Token editor toggle
                    .child(
                        div()
                            .id("token-editor-toggle")
                            .px_3()
                            .py_1()
                            .bg(if self.show_token_editor {
                                theme.element.selected
                            } else {
                                theme.element.background
                            })
                            .border_1()
                            .border_color(theme.border.default)
                            .rounded_md()
                            .cursor_pointer()
                            .hover(|s| s.bg(theme.element.hover))
                            .on_mouse_down(MouseButton::Left, {
                                cx.listener(|this, _event, _window, cx| {
                                    this.show_token_editor = !this.show_token_editor;
                                    cx.notify();
                                })
                            })
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text.default)
                                    .child("Tokens"),
                            ),
                    )
                    // Metadata toggle
                    .child(
                        div()
                            .id("metadata-toggle")
                            .px_3()
                            .py_1()
                            .bg(if self.show_metadata {
                                theme.element.selected
                            } else {
                                theme.element.background
                            })
                            .border_1()
                            .border_color(theme.border.default)
                            .rounded_md()
                            .cursor_pointer()
                            .hover(|s| s.bg(theme.element.hover))
                            .on_mouse_down(MouseButton::Left, {
                                cx.listener(|this, _event, _window, cx| {
                                    this.show_metadata = !this.show_metadata;
                                    cx.notify();
                                })
                            })
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text.default)
                                    .child("Metadata"),
                            ),
                    ),
            )
    }

    /// Render the sidebar with component/story list.
    fn render_sidebar(&self, cx: &Context<Self>) -> Div {
        let theme = cx.theme();
        let registry = cx.global::<StoryRegistry>();

        let mut sidebar = div()
            .flex()
            .flex_col()
            .w(px(220.0))
            .flex_shrink_0()
            .h_full()
            .bg(theme.panel.background)
            .border_r_1()
            .border_color(theme.border.default);

        // Sidebar header
        sidebar = sidebar.child(
            div()
                .flex()
                .flex_col()
                .px_3()
                .py_2()
                .border_b_1()
                .border_color(theme.border.default)
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.text.muted)
                        .child("COMPONENTS"),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text.placeholder)
                        .child(format!("{} stories", registry.len())),
                ),
        );

        // Story entries
        let mut story_list = div().flex().flex_col().py_1().overflow_y_hidden();

        for (idx, entry) in registry.entries().iter().enumerate() {
            let is_selected = self.selected_story_index == Some(idx);
            let name: SharedString = entry.name().to_string().into();
            let description: SharedString = entry.description().to_string().into();

            let item_bg = if is_selected {
                theme.ghost_element.selected
            } else {
                Hsla::transparent_black()
            };

            let item_text = if is_selected {
                theme.text.default
            } else {
                theme.text.muted
            };

            story_list = story_list.child(
                div()
                    .id(ElementId::Name(format!("story-nav-{}", idx).into()))
                    .flex()
                    .flex_col()
                    .px_3()
                    .py(px(6.0))
                    .mx_1()
                    .bg(item_bg)
                    .rounded_md()
                    .cursor_pointer()
                    .hover(|s| s.bg(theme.ghost_element.hover))
                    .on_mouse_down(MouseButton::Left, {
                        cx.listener(move |this, _event, _window, cx| {
                            this.selected_story_index = Some(idx);
                            cx.notify();
                        })
                    })
                    .child(
                        div()
                            .text_sm()
                            .font_weight(if is_selected {
                                FontWeight::MEDIUM
                            } else {
                                FontWeight::NORMAL
                            })
                            .text_color(item_text)
                            .child(name),
                    )
                    .when(!description.is_empty(), |this| {
                        this.child(
                            div()
                                .text_xs()
                                .text_color(theme.text.placeholder)
                                .overflow_x_hidden()
                                .child(description),
                        )
                    }),
            );
        }

        sidebar = sidebar.child(story_list);

        // Theme info at bottom of sidebar
        sidebar = sidebar.child(
            div()
                .mt_auto()
                .px_3()
                .py_2()
                .border_t_1()
                .border_color(theme.border.default)
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text.placeholder)
                        .child(format!("Theme: {}", theme.name)),
                ),
        );

        sidebar
    }

    /// Render the main content area with the selected story.
    fn render_content(&self, window: &mut Window, cx: &mut Context<Self>) -> Div {
        let theme = cx.theme();
        let bg = theme.surface.background;
        let border = theme.border.default;
        let text_default = theme.text.default;
        let text_muted = theme.text.muted;

        let mut content = div()
            .flex()
            .flex_col()
            .flex_1()
            .h_full()
            .bg(bg)
            .overflow_y_hidden();

        if let Some(idx) = self.selected_story_index {
            // Extract metadata from registry (scoped borrow)
            let story_info = {
                let registry = cx.global::<StoryRegistry>();
                registry.entries().get(idx).map(|entry| {
                    (
                        SharedString::from(entry.name().to_string()),
                        SharedString::from(entry.description().to_string()),
                    )
                })
            };

            if let Some((story_name, story_desc)) = story_info {
                // Content header
                content = content.child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .px_6()
                        .py_3()
                        .border_b_1()
                        .border_color(border)
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .child(
                                    div()
                                        .text_lg()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(text_default)
                                        .child(story_name),
                                )
                                .when(!story_desc.is_empty(), |this| {
                                    this.child(
                                        div().text_sm().text_color(text_muted).child(story_desc),
                                    )
                                }),
                        ),
                );

                // Render the selected story directly (avoids holding registry borrow
                // across the mutable cx access needed by render_story).
                let story_element = render_story_by_index(idx, window, cx);
                if let Some(element) = story_element {
                    content =
                        content.child(div().flex_1().overflow_y_hidden().p_4().child(element));
                }
            }
        } else {
            // No story selected
            content = content.child(
                div().flex().flex_1().justify_center().items_center().child(
                    div()
                        .text_color(text_muted)
                        .child("Select a component from the sidebar"),
                ),
            );
        }

        content
    }

    /// Render the token editor panel (right sidebar).
    fn render_token_editor(&self, cx: &Context<Self>) -> Div {
        let theme = cx.theme();
        let all_paths = theme::engine::all_token_paths();

        let mut panel = div()
            .flex()
            .flex_col()
            .w(px(280.0))
            .flex_shrink_0()
            .h_full()
            .bg(theme.panel.background)
            .border_l_1()
            .border_color(theme.border.default);

        // Panel header
        panel = panel.child(
            div()
                .px_3()
                .py_2()
                .border_b_1()
                .border_color(theme.border.default)
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.text.muted)
                        .child("TOKEN EDITOR"),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text.placeholder)
                        .child(format!("{} tokens", all_paths.len())),
                ),
        );

        // Token list
        let mut token_list = div().flex().flex_col().py_1().overflow_y_hidden();

        // Group tokens by category (first segment before '.')
        let mut current_category = "";

        for path in &all_paths {
            let category = path.split('.').next().unwrap_or("");

            if category != current_category {
                current_category = category;
                token_list = token_list.child(
                    div()
                        .px_3()
                        .pt_3()
                        .pb_1()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.text.muted)
                        .child(SharedString::from(category.to_string())),
                );
            }

            let path_str = *path;
            let is_editing = self.editing_token_path.as_deref() == Some(path_str);

            // Get the current color value for this token for the color swatch
            let color_value = get_token_color(theme, path_str);

            let label: SharedString = path_str
                .split('.')
                .skip(1)
                .collect::<Vec<_>>()
                .join(".")
                .into();

            let mut token_row = div()
                .id(ElementId::Name(format!("token-{}", path_str).into()))
                .flex()
                .flex_row()
                .items_center()
                .gap_2()
                .px_3()
                .py(px(3.0))
                .cursor_pointer()
                .hover(|s| s.bg(theme.ghost_element.hover))
                .rounded_sm()
                .mx_1();

            // Color swatch
            if let Some(color) = color_value {
                token_row = token_row.child(
                    div()
                        .w(px(14.0))
                        .h(px(14.0))
                        .rounded_sm()
                        .border_1()
                        .border_color(theme.border.default)
                        .bg(color)
                        .flex_shrink_0(),
                );
            }

            token_row = token_row.child(
                div()
                    .text_xs()
                    .text_color(if is_editing {
                        theme.text.accent
                    } else {
                        theme.text.default
                    })
                    .overflow_x_hidden()
                    .child(label),
            );

            if !is_editing {
                let path_owned = path_str.to_string();
                token_row = token_row.on_mouse_down(MouseButton::Left, {
                    cx.listener(move |this, _event, _window, cx| {
                        this.editing_token_path = Some(path_owned.clone());
                        // Pre-fill with current hex value
                        if let Some(color) = get_token_color(cx.theme(), &path_owned) {
                            let rgba: Rgba = color.into();
                            this.editing_token_value = format!(
                                "#{:02x}{:02x}{:02x}{:02x}",
                                (rgba.r * 255.0) as u8,
                                (rgba.g * 255.0) as u8,
                                (rgba.b * 255.0) as u8,
                                (rgba.a * 255.0) as u8,
                            );
                        }
                        cx.notify();
                    })
                });
            }

            token_list = token_list.child(token_row);

            // Show input field if editing this token
            if is_editing {
                let edit_value: SharedString = self.editing_token_value.clone().into();
                token_list = token_list.child(
                    div()
                        .flex()
                        .flex_row()
                        .gap_1()
                        .px_3()
                        .py_1()
                        .mx_1()
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text.muted)
                                .flex_1()
                                .px_2()
                                .py(px(2.0))
                                .bg(theme.element.background)
                                .border_1()
                                .border_color(theme.border.focused)
                                .rounded_sm()
                                .child(edit_value),
                        )
                        // Apply button
                        .child(
                            div()
                                .id("token-apply")
                                .text_xs()
                                .text_color(theme.text.default)
                                .px_2()
                                .py(px(2.0))
                                .bg(theme.element.background)
                                .border_1()
                                .border_color(theme.border.default)
                                .rounded_sm()
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.element.hover))
                                .on_mouse_down(MouseButton::Left, {
                                    cx.listener(|this, _event, _window, cx| {
                                        this.apply_token_edit(cx);
                                    })
                                })
                                .child("OK"),
                        )
                        // Cancel button
                        .child(
                            div()
                                .id("token-cancel")
                                .text_xs()
                                .text_color(theme.text.muted)
                                .px_2()
                                .py(px(2.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.ghost_element.hover))
                                .rounded_sm()
                                .on_mouse_down(MouseButton::Left, {
                                    cx.listener(|this, _event, _window, cx| {
                                        this.editing_token_path = None;
                                        this.editing_token_value.clear();
                                        cx.notify();
                                    })
                                })
                                .child("X"),
                        ),
                );
            }
        }

        panel = panel.child(token_list);
        panel
    }

    /// Render the component metadata panel (below content or in a sidebar).
    fn render_metadata_panel(&self, cx: &Context<Self>) -> Div {
        let theme = cx.theme();
        let registry = cx.global::<StoryRegistry>();

        let mut panel = div()
            .flex()
            .flex_col()
            .w_full()
            .max_h(px(300.0))
            .overflow_y_hidden()
            .bg(theme.panel.background)
            .border_t_1()
            .border_color(theme.border.default);

        if let Some(idx) = self.selected_story_index {
            if let Some(entry) = registry.entries().get(idx) {
                let contract = entry.contract();

                // Metadata header
                panel = panel.child(
                    div()
                        .px_4()
                        .py_2()
                        .border_b_1()
                        .border_color(theme.border.default)
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(theme.text.muted)
                                .child(format!(
                                    "{} v{} — {:?}",
                                    contract.name, contract.version, contract.disposition
                                )),
                        ),
                );

                let mut info_row = div().flex().flex_row().gap_6().px_4().py_3();

                // Props column
                let mut props_col = div().flex().flex_col().gap_1().flex_1().child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.text.muted)
                        .child("Props"),
                );
                for prop in &contract.props {
                    let required_tag = if prop.required { " *" } else { "" };
                    props_col = props_col.child(
                        div()
                            .text_xs()
                            .text_color(theme.text.default)
                            .child(format!("{}: {}{}", prop.name, prop.type_name, required_tag)),
                    );
                }
                info_row = info_row.child(props_col);

                // States column
                let mut states_col = div().flex().flex_col().gap_1().flex_1().child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.text.muted)
                        .child("States"),
                );
                for state in &contract.states {
                    states_col = states_col.child(
                        div()
                            .text_xs()
                            .text_color(theme.text.default)
                            .child(format!("{:?}", state)),
                    );
                }
                info_row = info_row.child(states_col);

                // Interaction checklist column
                let mut interaction_col = div().flex().flex_col().gap_1().flex_1().child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.text.muted)
                        .child("Interaction"),
                );
                let ic = &contract.interaction_checklist;
                if let Some(ref focus) = ic.focus_behavior {
                    interaction_col = interaction_col.child(
                        div()
                            .text_xs()
                            .text_color(theme.text.default)
                            .child(format!("Focus: {}", truncate_str(focus, 60))),
                    );
                }
                if let Some(ref kb) = ic.keyboard_model {
                    interaction_col = interaction_col.child(
                        div()
                            .text_xs()
                            .text_color(theme.text.default)
                            .child(format!("Keyboard: {}", truncate_str(kb, 60))),
                    );
                }
                info_row = info_row.child(interaction_col);

                // Token deps column
                let mut tokens_col = div().flex().flex_col().gap_1().flex_1().child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.text.muted)
                        .child("Token Dependencies"),
                );
                for dep in &contract.token_dependencies {
                    tokens_col = tokens_col.child(
                        div()
                            .text_xs()
                            .text_color(theme.text.default)
                            .child(dep.path.clone()),
                    );
                }
                info_row = info_row.child(tokens_col);

                panel = panel.child(info_row);
            }
        } else {
            panel = panel.child(
                div().px_4().py_3().child(
                    div()
                        .text_xs()
                        .text_color(theme.text.muted)
                        .child("Select a component to view its metadata"),
                ),
            );
        }

        panel
    }
}

impl Render for StudioApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.surface.background)
            // Top toolbar
            .child(self.render_toolbar(cx))
            // Main area: sidebar + content + optional token editor
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_1()
                    .overflow_hidden()
                    // Left sidebar: component list
                    .child(self.render_sidebar(cx))
                    // Center: content area (story + optional metadata below)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .overflow_hidden()
                            // Story content
                            .child(self.render_content(window, cx))
                            // Metadata panel (conditionally shown)
                            .when(self.show_metadata, |this| {
                                this.child(self.render_metadata_panel(cx))
                            }),
                    )
                    // Right sidebar: token editor (conditionally shown)
                    .when(self.show_token_editor, |this| {
                        this.child(self.render_token_editor(cx))
                    }),
            )
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Render a story by index, using the concrete story types directly.
///
/// This avoids the borrow conflict that would occur if we held a reference to
/// the `StoryRegistry` global while also passing `&mut App` to `render_story`.
fn render_story_by_index(idx: usize, window: &mut Window, cx: &mut App) -> Option<AnyElement> {
    use story::{DialogStory, SelectStory, Story, TabsStory};
    match idx {
        0 => Some(DialogStory.render_story(window, cx)),
        1 => Some(SelectStory.render_story(window, cx)),
        2 => Some(TabsStory.render_story(window, cx)),
        _ => None,
    }
}

/// Truncate a string to a maximum length, appending "..." if truncated.
fn truncate_str(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max.saturating_sub(3)])
    }
}

/// Look up the current Hsla color value for a token path on the active theme.
fn get_token_color(theme: &Theme, path: &str) -> Option<Hsla> {
    match path {
        "border.default" => Some(theme.border.default),
        "border.variant" => Some(theme.border.variant),
        "border.focused" => Some(theme.border.focused),
        "border.selected" => Some(theme.border.selected),
        "border.transparent" => Some(theme.border.transparent),
        "border.disabled" => Some(theme.border.disabled),
        "surface.background" => Some(theme.surface.background),
        "surface.surface" => Some(theme.surface.surface),
        "surface.elevated_surface" => Some(theme.surface.elevated_surface),
        "element.background" => Some(theme.element.background),
        "element.hover" => Some(theme.element.hover),
        "element.active" => Some(theme.element.active),
        "element.selected" => Some(theme.element.selected),
        "element.disabled" => Some(theme.element.disabled),
        "ghost_element.background" => Some(theme.ghost_element.background),
        "ghost_element.hover" => Some(theme.ghost_element.hover),
        "ghost_element.active" => Some(theme.ghost_element.active),
        "ghost_element.selected" => Some(theme.ghost_element.selected),
        "ghost_element.disabled" => Some(theme.ghost_element.disabled),
        "text.default" => Some(theme.text.default),
        "text.muted" => Some(theme.text.muted),
        "text.placeholder" => Some(theme.text.placeholder),
        "text.disabled" => Some(theme.text.disabled),
        "text.accent" => Some(theme.text.accent),
        "icon.default" => Some(theme.icon.default),
        "icon.muted" => Some(theme.icon.muted),
        "icon.disabled" => Some(theme.icon.disabled),
        "icon.placeholder" => Some(theme.icon.placeholder),
        "icon.accent" => Some(theme.icon.accent),
        "status.error.foreground" => Some(theme.status.error.foreground),
        "status.error.background" => Some(theme.status.error.background),
        "status.error.border" => Some(theme.status.error.border),
        "status.warning.foreground" => Some(theme.status.warning.foreground),
        "status.warning.background" => Some(theme.status.warning.background),
        "status.warning.border" => Some(theme.status.warning.border),
        "status.info.foreground" => Some(theme.status.info.foreground),
        "status.info.background" => Some(theme.status.info.background),
        "status.info.border" => Some(theme.status.info.border),
        "status.success.foreground" => Some(theme.status.success.foreground),
        "status.success.background" => Some(theme.status.success.background),
        "status.success.border" => Some(theme.status.success.border),
        "status.hint.foreground" => Some(theme.status.hint.foreground),
        "status.hint.background" => Some(theme.status.hint.background),
        "status.hint.border" => Some(theme.status.hint.border),
        "tab.bar_background" => Some(theme.tab.bar_background),
        "tab.inactive_background" => Some(theme.tab.inactive_background),
        "tab.active_background" => Some(theme.tab.active_background),
        "panel.background" => Some(theme.panel.background),
        "panel.focused_border" => theme.panel.focused_border,
        "chrome.title_bar_background" => Some(theme.chrome.title_bar_background),
        "chrome.status_bar_background" => Some(theme.chrome.status_bar_background),
        "chrome.toolbar_background" => Some(theme.chrome.toolbar_background),
        "scrollbar.thumb_background" => Some(theme.scrollbar.thumb_background),
        "scrollbar.thumb_hover_background" => Some(theme.scrollbar.thumb_hover_background),
        "scrollbar.thumb_border" => Some(theme.scrollbar.thumb_border),
        "scrollbar.track_background" => Some(theme.scrollbar.track_background),
        "scrollbar.track_border" => Some(theme.scrollbar.track_border),
        "player.cursor" => Some(theme.player.cursor),
        "player.background" => Some(theme.player.background),
        "player.selection" => Some(theme.player.selection),
        "link.hover" => Some(theme.link.hover),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Application entry point
// ---------------------------------------------------------------------------

fn main() {
    gpui_platform::application().run(move |cx| {
        // Initialize all crates in dependency order.
        assets::init(cx);
        theme::init(cx);
        primitives::init(cx);
        components::init(cx);
        story::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: Size {
                            width: px(1280.0),
                            height: px(800.0),
                        },
                    })),
                    ..Default::default()
                },
                |_window, cx| cx.new(|_cx| StudioApp::new()),
            )?;
            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
