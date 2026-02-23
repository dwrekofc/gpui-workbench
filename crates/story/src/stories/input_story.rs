//! Input story: demonstrates Input states and configurations.

use crate::{
    Story,
    matrix::{StateMatrix, section},
};
use components::{ComponentContract, ComponentState, Input, InputSize};
use gpui::*;
use theme::ActiveTheme;

pub struct InputStory;

impl Story for InputStory {
    fn name(&self) -> &'static str {
        "Input"
    }

    fn description(&self) -> &'static str {
        "Single-line text input with placeholder, sizes, validation, and prefix/suffix."
    }

    fn contract(&self) -> ComponentContract {
        Input::contract()
    }

    fn render_story(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        let theme = cx.theme();
        let muted_color = theme.text.muted;

        let mut container = div().flex().flex_col().gap_6().p_4().w_full();

        // Default
        let default_section = section("Default Input", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Empty input with placeholder."),
            )
            .child(Input::new("default-input").placeholder("Enter text..."));
        container = container.child(default_section);

        // With value
        let value_section = section("With Value", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Input with pre-filled value."),
            )
            .child(Input::new("value-input").value("Hello, world!"));
        container = container.child(value_section);

        // Sizes
        let sizes_section = section("Sizes", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Small, Medium, and Large sizes."),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        Input::new("small-input")
                            .placeholder("Small")
                            .size(InputSize::Small),
                    )
                    .child(
                        Input::new("medium-input")
                            .placeholder("Medium")
                            .size(InputSize::Medium),
                    )
                    .child(
                        Input::new("large-input")
                            .placeholder("Large")
                            .size(InputSize::Large),
                    ),
            );
        container = container.child(sizes_section);

        // With prefix/suffix
        let affix_section = section("Prefix & Suffix", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Input with prefix and suffix labels."),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        Input::new("url-input")
                            .prefix("https://")
                            .placeholder("example.com"),
                    )
                    .child(
                        Input::new("price-input")
                            .prefix("$")
                            .suffix(".00")
                            .value("99"),
                    ),
            );
        container = container.child(affix_section);

        // Error state
        let error_section = section("Error State", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Input with error message."),
            )
            .child(
                Input::new("error-input")
                    .value("invalid-email")
                    .error_message("Please enter a valid email address"),
            );
        container = container.child(error_section);

        // Disabled and Readonly
        let disabled_section = section("Disabled & Readonly", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Disabled and readonly states."),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        Input::new("disabled-input")
                            .value("Disabled")
                            .disabled(true),
                    )
                    .child(
                        Input::new("readonly-input")
                            .value("Read-only value")
                            .readonly(true),
                    ),
            );
        container = container.child(disabled_section);

        // State Matrix
        let matrix = StateMatrix::from_contract(&self.contract());
        let matrix_element = matrix.render(
            |state, _variant, _window, _cx| render_input_state_cell(state),
            window,
            cx,
        );
        container = container.child(matrix_element);

        container.into_any_element()
    }
}

fn render_input_state_cell(state: ComponentState) -> AnyElement {
    let id = SharedString::from(format!("matrix-{state:?}"));
    let mut input = Input::new(id).placeholder(SharedString::from(format!("{state:?}")));

    match state {
        ComponentState::Disabled => input = input.disabled(true),
        ComponentState::Error => input = input.error(true),
        ComponentState::Readonly => input = input.readonly(true),
        _ => {}
    }

    input.into_any_element()
}
