//! Radio story: demonstrates Radio group configurations.

use crate::{
    Story,
    matrix::{StateMatrix, section},
};
use components::{ComponentContract, ComponentState, Radio, RadioItem};
use gpui::*;
use primitives::Orientation;
use theme::ActiveTheme;

pub struct RadioStory;

impl Story for RadioStory {
    fn name(&self) -> &'static str {
        "Radio"
    }

    fn description(&self) -> &'static str {
        "Single-selection radio group with arrow-key navigation."
    }

    fn contract(&self) -> ComponentContract {
        Radio::contract()
    }

    fn render_story(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        let theme = cx.theme();
        let muted_color = theme.text.muted;

        let mut container = div().flex().flex_col().gap_6().p_4().w_full();

        // Default vertical
        let default_section = section("Vertical Radio Group", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Default vertical layout with selection."),
            )
            .child(
                Radio::new(
                    "color-radio",
                    vec![
                        RadioItem::new("Red"),
                        RadioItem::new("Green"),
                        RadioItem::new("Blue"),
                    ],
                )
                .selected_index(1),
            );
        container = container.child(default_section);

        // Horizontal
        let horizontal_section = section("Horizontal Radio Group", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Horizontal layout."),
            )
            .child(
                Radio::new(
                    "size-radio",
                    vec![
                        RadioItem::new("Small"),
                        RadioItem::new("Medium"),
                        RadioItem::new("Large"),
                    ],
                )
                .orientation(Orientation::Horizontal)
                .selected_index(0),
            );
        container = container.child(horizontal_section);

        // With disabled items
        let disabled_items_section = section("With Disabled Items", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Some options are disabled."),
            )
            .child(
                Radio::new(
                    "plan-radio",
                    vec![
                        RadioItem::new("Free"),
                        RadioItem::disabled("Pro (coming soon)"),
                        RadioItem::new("Enterprise"),
                    ],
                )
                .selected_index(0),
            );
        container = container.child(disabled_items_section);

        // Disabled group
        let disabled_section = section("Disabled Group", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Entire group is disabled."),
            )
            .child(
                Radio::new(
                    "disabled-radio",
                    vec![RadioItem::new("Option A"), RadioItem::new("Option B")],
                )
                .selected_index(0)
                .disabled(true),
            );
        container = container.child(disabled_section);

        // State Matrix
        let matrix = StateMatrix::from_contract(&self.contract());
        let matrix_element = matrix.render(
            |state, _variant, _window, _cx| render_radio_state_cell(state),
            window,
            cx,
        );
        container = container.child(matrix_element);

        container.into_any_element()
    }
}

fn render_radio_state_cell(state: ComponentState) -> AnyElement {
    let id = SharedString::from(format!("matrix-{state:?}"));
    let items = vec![RadioItem::new("A"), RadioItem::new("B")];
    let mut radio = Radio::new(id, items).selected_index(0);

    if state == ComponentState::Disabled {
        radio = radio.disabled(true);
    }

    radio.into_any_element()
}
