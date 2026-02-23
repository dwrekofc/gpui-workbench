//! Checkbox story: demonstrates all Checkbox states and configurations.

use crate::{
    Story,
    matrix::{StateMatrix, section},
};
use components::{Checkbox, ComponentContract, ComponentState};
use gpui::*;
use theme::ActiveTheme;

pub struct CheckboxStory;

impl Story for CheckboxStory {
    fn name(&self) -> &'static str {
        "Checkbox"
    }

    fn description(&self) -> &'static str {
        "Togglable boolean control with label, checked/unchecked/indeterminate states."
    }

    fn contract(&self) -> ComponentContract {
        Checkbox::contract()
    }

    fn render_story(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        let theme = cx.theme();
        let muted_color = theme.text.muted;

        let mut container = div().flex().flex_col().gap_6().p_4().w_full();

        // States
        let states_section = section("Checkbox States", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Unchecked, checked, indeterminate, and disabled."),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(Checkbox::new("unchecked").label("Unchecked"))
                    .child(Checkbox::new("checked").label("Checked").checked(true))
                    .child(
                        Checkbox::new("indeterminate")
                            .label("Indeterminate")
                            .indeterminate(true),
                    )
                    .child(
                        Checkbox::new("disabled-unchecked")
                            .label("Disabled (unchecked)")
                            .disabled(true),
                    )
                    .child(
                        Checkbox::new("disabled-checked")
                            .label("Disabled (checked)")
                            .checked(true)
                            .disabled(true),
                    ),
            );
        container = container.child(states_section);

        // Without label
        let no_label_section = section("Without Label", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Checkboxes without labels."),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_3()
                    .child(Checkbox::new("no-label-1"))
                    .child(Checkbox::new("no-label-2").checked(true))
                    .child(Checkbox::new("no-label-3").indeterminate(true)),
            );
        container = container.child(no_label_section);

        // State Matrix
        let matrix = StateMatrix::from_contract(&self.contract());
        let matrix_element = matrix.render(
            |state, _variant, _window, cx| render_checkbox_state_cell(state, cx),
            window,
            cx,
        );
        container = container.child(matrix_element);

        container.into_any_element()
    }
}

fn render_checkbox_state_cell(state: ComponentState, _cx: &mut App) -> AnyElement {
    let id = SharedString::from(format!("matrix-{state:?}"));
    let mut cb = Checkbox::new(id).label(SharedString::from(format!("{state:?}")));

    match state {
        ComponentState::Disabled => cb = cb.disabled(true),
        ComponentState::Selected => cb = cb.checked(true),
        _ => {}
    }

    cb.into_any_element()
}
