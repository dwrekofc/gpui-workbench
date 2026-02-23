//! Dialog story: demonstrates all Dialog states and configurations.
//!
//! Renders multiple Dialog instances showing:
//! - Default open dialog with title and description
//! - Dialog with action buttons (OK/Cancel footer)
//! - Dialog with body content
//! - Dialog without close button
//! - Dialog with non-closable overlay
//! - State matrix showing Open, Focused, Hover, Active states

use crate::{
    Story,
    matrix::{StateMatrix, section},
};
use components::{ComponentContract, ComponentState, Dialog};
use gpui::*;
use theme::ActiveTheme;

/// Story for the Dialog component.
///
/// Demonstrates focus trap, escape/outside-click dismiss, title/description/action slots,
/// controlled open/close state, and token-driven styling.
pub struct DialogStory;

impl Story for DialogStory {
    fn name(&self) -> &'static str {
        "Dialog"
    }

    fn description(&self) -> &'static str {
        "Modal overlay with focus trap, escape/outside-click dismiss, and action slots."
    }

    fn contract(&self) -> ComponentContract {
        Dialog::contract()
    }

    fn render_story(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        let theme = cx.theme();
        let muted_color = theme.text.muted;

        let mut container = div().flex().flex_col().gap_6().p_4().w_full();

        // Section 1: Default Dialog (open, with title and description)
        let default_section = section("Default Dialog", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Open dialog with title, description, and close button."),
            )
            .child(render_dialog_preview(
                DialogPreviewConfig::new("Default")
                    .title("Confirm Action")
                    .description("Are you sure you want to proceed?"),
                cx,
            ));
        container = container.child(default_section);

        // Section 2: Dialog with Actions
        let actions_section = section("Dialog with Actions", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Dialog with OK/Cancel action buttons in the footer."),
            )
            .child(render_dialog_preview(
                DialogPreviewConfig::new("WithActions")
                    .title("Delete Item?")
                    .description("This action cannot be undone.")
                    .actions(&["Cancel", "Delete"]),
                cx,
            ));
        container = container.child(actions_section);

        // Section 3: Dialog with Body Content
        let body_section = section("Dialog with Body Content", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Dialog with custom body content between description and actions."),
            )
            .child(render_dialog_preview(
                DialogPreviewConfig::new("WithBody")
                    .title("Settings")
                    .description("Configure your preferences below.")
                    .actions(&["Save"])
                    .body("This is custom body content rendered inside the dialog panel."),
                cx,
            ));
        container = container.child(body_section);

        // Section 4: Dialog without Close Button
        let no_close_section =
            section("No Close Button", cx)
                .child(div().text_xs().text_color(muted_color).child(
                    "Dialog without the X close button. Must use actions or Escape to dismiss.",
                ))
                .child(render_dialog_preview(
                    DialogPreviewConfig::new("NoClose")
                        .title("Required Action")
                        .description("Please choose an action below.")
                        .show_close(false)
                        .actions(&["Proceed"]),
                    cx,
                ));
        container = container.child(no_close_section);

        // Section 5: State Matrix
        let matrix = StateMatrix::from_contract(&self.contract());
        let matrix_element = matrix.render(
            |state, _variant, window, cx| render_dialog_state_cell(state, window, cx),
            window,
            cx,
        );
        container = container.child(matrix_element);

        container.into_any_element()
    }
}

/// Configuration for a dialog preview rendering.
struct DialogPreviewConfig {
    id: SharedString,
    title: Option<SharedString>,
    description: Option<SharedString>,
    show_close: bool,
    action_labels: Vec<SharedString>,
    body_text: Option<SharedString>,
}

impl DialogPreviewConfig {
    fn new(id: &str) -> Self {
        Self {
            id: id.to_string().into(),
            title: None,
            description: None,
            show_close: true,
            action_labels: Vec::new(),
            body_text: None,
        }
    }

    fn title(mut self, t: &str) -> Self {
        self.title = Some(t.to_string().into());
        self
    }

    fn description(mut self, d: &str) -> Self {
        self.description = Some(d.to_string().into());
        self
    }

    fn show_close(mut self, v: bool) -> Self {
        self.show_close = v;
        self
    }

    fn actions(mut self, labels: &[&str]) -> Self {
        self.action_labels = labels
            .iter()
            .map(|s| SharedString::from(s.to_string()))
            .collect();
        self
    }

    fn body(mut self, text: &str) -> Self {
        self.body_text = Some(text.to_string().into());
        self
    }
}

/// Render a dialog panel preview (inline, not as overlay) for story display.
///
/// Renders just the dialog panel content without the full-screen overlay backdrop,
/// because stories display multiple dialogs simultaneously and overlay rendering
/// would obscure other content.
fn render_dialog_preview(config: DialogPreviewConfig, cx: &mut App) -> AnyElement {
    let DialogPreviewConfig {
        id,
        title,
        description,
        show_close,
        action_labels,
        body_text,
    } = config;
    let theme = cx.theme();
    let panel_bg = theme.surface.elevated_surface;
    let border_color = theme.border.default;
    let title_color = theme.text.default;
    let desc_color = theme.text.muted;
    let close_hover = theme.ghost_element.hover;

    let mut panel = div()
        .id(ElementId::Name(format!("dialog-story-{}", id).into()))
        .flex()
        .flex_col()
        .w(px(400.0))
        .overflow_hidden()
        .bg(panel_bg)
        .border_1()
        .border_color(border_color)
        .rounded_lg()
        .shadow_lg()
        .p_6()
        .gap_3();

    // Title row with optional close button
    if let Some(ref title_text) = title {
        let mut title_row = div()
            .flex()
            .flex_row()
            .justify_between()
            .items_center()
            .child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(title_color)
                    .child(title_text.clone()),
            );

        if show_close {
            title_row = title_row.child(
                div()
                    .text_sm()
                    .text_color(desc_color)
                    .cursor_pointer()
                    .rounded_md()
                    .p_1()
                    .hover(|s| s.bg(close_hover))
                    .child("X"),
            );
        }

        panel = panel.child(title_row);
    }

    // Description
    if let Some(ref desc_text) = description {
        panel = panel.child(
            div()
                .text_sm()
                .text_color(desc_color)
                .child(desc_text.clone()),
        );
    }

    // Body content
    if let Some(ref body) = body_text {
        panel = panel.child(
            div()
                .text_sm()
                .text_color(title_color)
                .p_3()
                .bg(theme.surface.background)
                .rounded_md()
                .child(body.clone()),
        );
    }

    // Action buttons
    if !action_labels.is_empty() {
        let mut footer = div().flex().flex_row().justify_end().gap_2().pt_2();
        for label in &action_labels {
            footer = footer.child(
                div()
                    .px_4()
                    .py_1()
                    .text_sm()
                    .text_color(title_color)
                    .bg(theme.element.background)
                    .border_1()
                    .border_color(border_color)
                    .rounded_md()
                    .cursor_pointer()
                    .hover(|s| s.bg(theme.element.hover))
                    .child(label.clone()),
            );
        }
        panel = panel.child(footer);
    }

    panel.into_any_element()
}

/// Render a state matrix cell for a given Dialog state.
fn render_dialog_state_cell(
    state: ComponentState,
    _window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let theme = cx.theme();
    let text_color = theme.text.default;
    let muted_color = theme.text.muted;
    let border_color = theme.border.default;

    match state {
        ComponentState::Open => {
            // Show a mini dialog panel in "open" state
            div()
                .flex()
                .flex_col()
                .gap_1()
                .p_2()
                .bg(theme.surface.elevated_surface)
                .border_1()
                .border_color(border_color)
                .rounded_md()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(text_color)
                        .child("Title"),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(muted_color)
                        .child("Description text"),
                )
                .into_any_element()
        }
        ComponentState::Focused => {
            // Show dialog with focus ring
            div()
                .flex()
                .flex_col()
                .gap_1()
                .p_2()
                .bg(theme.surface.elevated_surface)
                .border_2()
                .border_color(theme.border.focused)
                .rounded_md()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(text_color)
                        .child("Title (focused)"),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(muted_color)
                        .child("Focus ring visible"),
                )
                .into_any_element()
        }
        ComponentState::Hover => {
            // Show close button in hover state
            div()
                .flex()
                .flex_col()
                .gap_1()
                .p_2()
                .bg(theme.surface.elevated_surface)
                .border_1()
                .border_color(border_color)
                .rounded_md()
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .justify_between()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(text_color)
                                .child("Title"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(muted_color)
                                .bg(theme.ghost_element.hover)
                                .rounded_sm()
                                .px_1()
                                .child("X"),
                        ),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(muted_color)
                        .child("Close btn hovered"),
                )
                .into_any_element()
        }
        ComponentState::Active => {
            // Show close button in active/pressed state
            div()
                .flex()
                .flex_col()
                .gap_1()
                .p_2()
                .bg(theme.surface.elevated_surface)
                .border_1()
                .border_color(border_color)
                .rounded_md()
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .justify_between()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(text_color)
                                .child("Title"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(muted_color)
                                .bg(theme.ghost_element.active)
                                .rounded_sm()
                                .px_1()
                                .child("X"),
                        ),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(muted_color)
                        .child("Close btn pressed"),
                )
                .into_any_element()
        }
        _ => {
            // Unsupported state for Dialog
            div()
                .text_xs()
                .text_color(muted_color)
                .child(format!("{:?} N/A", state))
                .into_any_element()
        }
    }
}
