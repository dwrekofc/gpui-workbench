//! Popover story: demonstrates Popover configurations.

use crate::{Story, matrix::section};
use components::{ComponentContract, Popover};
use gpui::*;
use theme::ActiveTheme;

pub struct PopoverStory;

impl Story for PopoverStory {
    fn name(&self) -> &'static str {
        "Popover"
    }

    fn description(&self) -> &'static str {
        "Positioned overlay anchored to a trigger with escape/outside-click dismiss."
    }

    fn contract(&self) -> ComponentContract {
        Popover::contract()
    }

    fn render_story(&self, _window: &mut Window, cx: &mut App) -> AnyElement {
        let theme = cx.theme();
        let muted_color = theme.text.muted;

        let mut container = div().flex().flex_col().gap_6().p_4().w_full();

        // Closed
        let closed_section = section("Closed Popover", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Popover in closed state (nothing rendered)."),
            )
            .child(Popover::new("closed-popover"));
        container = container.child(closed_section);

        // Open with content
        let open_section =
            section("Open Popover", cx)
                .child(
                    div()
                        .text_xs()
                        .text_color(muted_color)
                        .child("Popover with content visible."),
                )
                .child(
                    Popover::new("open-popover")
                        .open(true)
                        .width(px(200.0))
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_2()
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::MEDIUM)
                                        .child("Popover Title"),
                                )
                                .child(div().text_xs().text_color(muted_color).child(
                                    "This is popover content that can contain any elements.",
                                )),
                        ),
                );
        container = container.child(open_section);

        // With custom width
        let wide_section =
            section("Wide Popover", cx)
                .child(
                    div()
                        .text_xs()
                        .text_color(muted_color)
                        .child("Popover with wider content area."),
                )
                .child(
                    Popover::new("wide-popover")
                        .open(true)
                        .width(px(320.0))
                        .child(div().p_2().child(
                            "This popover has a wider content area for more complex content.",
                        )),
                );
        container = container.child(wide_section);

        container.into_any_element()
    }
}
