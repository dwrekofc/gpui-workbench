//! Textarea story: demonstrates Textarea states and configurations.

use crate::{Story, matrix::section};
use components::{ComponentContract, Textarea};
use gpui::*;
use theme::ActiveTheme;

pub struct TextareaStory;

impl Story for TextareaStory {
    fn name(&self) -> &'static str {
        "Textarea"
    }

    fn description(&self) -> &'static str {
        "Multi-line text input with configurable rows and validation states."
    }

    fn contract(&self) -> ComponentContract {
        Textarea::contract()
    }

    fn render_story(&self, _window: &mut Window, cx: &mut App) -> AnyElement {
        let theme = cx.theme();
        let muted_color = theme.text.muted;

        let mut container = div().flex().flex_col().gap_6().p_4().w_full();

        // Default
        let default_section = section("Default Textarea", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("3-row textarea with placeholder."),
            )
            .child(Textarea::new("default-textarea").placeholder("Enter text..."));
        container = container.child(default_section);

        // With value and rows
        let value_section = section("With Value (5 rows)", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Pre-filled textarea with 5 rows."),
            )
            .child(
                Textarea::new("value-textarea")
                    .value("Line 1\nLine 2\nLine 3")
                    .rows(5),
            );
        container = container.child(value_section);

        // Error state
        let error_section = section("Error State", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Textarea with validation error."),
            )
            .child(
                Textarea::new("error-textarea")
                    .value("Too short")
                    .error_message("Minimum 50 characters required"),
            );
        container = container.child(error_section);

        // Disabled
        let disabled_section = section("Disabled", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Disabled textarea."),
            )
            .child(
                Textarea::new("disabled-textarea")
                    .value("Cannot edit this")
                    .disabled(true),
            );
        container = container.child(disabled_section);

        container.into_any_element()
    }
}
