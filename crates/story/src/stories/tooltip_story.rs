//! Tooltip story: demonstrates tooltip placements and configurations.

use crate::{Story, matrix::section};
use components::{ComponentContract, Tooltip, TooltipPlacement};
use gpui::*;
use theme::ActiveTheme;

pub struct TooltipStory;

impl Story for TooltipStory {
    fn name(&self) -> &'static str {
        "Tooltip"
    }

    fn description(&self) -> &'static str {
        "Hover-triggered contextual text overlay with configurable placement."
    }

    fn contract(&self) -> ComponentContract {
        Tooltip::contract()
    }

    fn render_story(&self, _window: &mut Window, cx: &mut App) -> AnyElement {
        let theme = cx.theme();
        let muted_color = theme.text.muted;

        let mut container = div().flex().flex_col().gap_6().p_4().w_full();

        // Tooltip placements
        let placements_section = section("Tooltip Placements", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Tooltips rendered inline to show their appearance."),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_4()
                    .items_center()
                    .child(
                        Tooltip::new("top-tip")
                            .text("Top tooltip")
                            .placement(TooltipPlacement::Top),
                    )
                    .child(
                        Tooltip::new("bottom-tip")
                            .text("Bottom tooltip")
                            .placement(TooltipPlacement::Bottom),
                    )
                    .child(
                        Tooltip::new("left-tip")
                            .text("Left tooltip")
                            .placement(TooltipPlacement::Left),
                    )
                    .child(
                        Tooltip::new("right-tip")
                            .text("Right tooltip")
                            .placement(TooltipPlacement::Right),
                    ),
            );
        container = container.child(placements_section);

        // Long text tooltip
        let long_section = section("Long Text", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Tooltip with longer text content showing max-width behavior."),
            )
            .child(
                Tooltip::new("long-tip")
                    .text("This is a longer tooltip text that demonstrates the max-width constraint and text wrapping behavior.")
                    .max_width(px(200.0)),
            );
        container = container.child(long_section);

        container.into_any_element()
    }
}
