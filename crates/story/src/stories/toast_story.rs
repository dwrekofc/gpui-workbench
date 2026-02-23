//! Toast story: demonstrates all Toast variants and configurations.

use crate::{Story, matrix::section};
use components::{ComponentContract, Toast, ToastVariant};
use gpui::*;
use theme::ActiveTheme;

pub struct ToastStory;

impl Story for ToastStory {
    fn name(&self) -> &'static str {
        "Toast"
    }

    fn description(&self) -> &'static str {
        "Transient notification with Info, Success, Warning, Error variants and action buttons."
    }

    fn contract(&self) -> ComponentContract {
        Toast::contract()
    }

    fn render_story(&self, _window: &mut Window, cx: &mut App) -> AnyElement {
        let theme = cx.theme();
        let muted_color = theme.text.muted;

        let mut container = div().flex().flex_col().gap_6().p_4().w_full();

        // All variants
        let variants_section = section("Toast Variants", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Info, Success, Warning, and Error variants."),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(
                        Toast::new("info-toast")
                            .title("Information")
                            .description("This is an informational message.")
                            .variant(ToastVariant::Info),
                    )
                    .child(
                        Toast::new("success-toast")
                            .title("Success")
                            .description("Your changes have been saved.")
                            .variant(ToastVariant::Success),
                    )
                    .child(
                        Toast::new("warning-toast")
                            .title("Warning")
                            .description("Your session will expire in 5 minutes.")
                            .variant(ToastVariant::Warning),
                    )
                    .child(
                        Toast::new("error-toast")
                            .title("Error")
                            .description("Failed to save changes. Please try again.")
                            .variant(ToastVariant::Error),
                    ),
            );
        container = container.child(variants_section);

        // With action
        let action_section = section("With Action Button", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Toast with an action button."),
            )
            .child(
                Toast::new("action-toast")
                    .title("File deleted")
                    .description("document.txt has been moved to trash.")
                    .action("Undo", |_window, _cx| {})
                    .variant(ToastVariant::Info),
            );
        container = container.child(action_section);

        // Without dismiss
        let no_dismiss_section = section("Without Dismiss Button", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Toast without the dismiss X button."),
            )
            .child(
                Toast::new("no-dismiss-toast")
                    .title("Processing...")
                    .description("Please wait while we process your request.")
                    .show_dismiss(false)
                    .variant(ToastVariant::Info),
            );
        container = container.child(no_dismiss_section);

        container.into_any_element()
    }
}
