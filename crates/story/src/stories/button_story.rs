//! Button story: demonstrates all Button variants, sizes, and states.
//!
//! Renders multiple Button instances showing:
//! - All variants (Primary, Secondary, Ghost, Danger)
//! - All sizes (Small, Medium, Large)
//! - With and without icons
//! - Disabled state
//! - Selected state
//! - Full-width button
//! - State matrix showing Hover, Active, Focused, Disabled, Selected

use crate::{
    Story,
    matrix::{StateMatrix, section},
};
use components::{
    Button, ButtonSize, ButtonVariant, ComponentContract, ComponentState, IconPosition,
};
use gpui::*;
use theme::ActiveTheme;

/// Story for the Button component.
///
/// Demonstrates variant styling, icon+label composition, sizes, disabled/selected
/// states, and token-driven styling across all configurations.
pub struct ButtonStory;

impl Story for ButtonStory {
    fn name(&self) -> &'static str {
        "Button"
    }

    fn description(&self) -> &'static str {
        "Clickable element with variants (Primary, Secondary, Ghost, Danger), sizes, and icon+label composition."
    }

    fn contract(&self) -> ComponentContract {
        Button::contract()
    }

    fn render_story(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        let theme = cx.theme();
        let muted_color = theme.text.muted;

        let mut container = div().flex().flex_col().gap_6().p_4().w_full();

        // Section 1: All Variants
        let variants_section = section("Button Variants", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Primary, Secondary (default), Ghost, and Danger variants."),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_3()
                    .items_center()
                    .child(
                        Button::new("primary-btn")
                            .label("Primary")
                            .variant(ButtonVariant::Primary),
                    )
                    .child(
                        Button::new("secondary-btn")
                            .label("Secondary")
                            .variant(ButtonVariant::Secondary),
                    )
                    .child(
                        Button::new("ghost-btn")
                            .label("Ghost")
                            .variant(ButtonVariant::Ghost),
                    )
                    .child(
                        Button::new("danger-btn")
                            .label("Danger")
                            .variant(ButtonVariant::Danger),
                    ),
            );
        container = container.child(variants_section);

        // Section 2: All Sizes
        let sizes_section = section("Button Sizes", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Small (24px), Medium (28px, default), and Large (32px)."),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_3()
                    .items_center()
                    .child(
                        Button::new("small-btn")
                            .label("Small")
                            .size(ButtonSize::Small),
                    )
                    .child(
                        Button::new("medium-btn")
                            .label("Medium")
                            .size(ButtonSize::Medium),
                    )
                    .child(
                        Button::new("large-btn")
                            .label("Large")
                            .size(ButtonSize::Large),
                    ),
            );
        container = container.child(sizes_section);

        // Section 3: With Icons
        let icon_section = section("With Icons", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Buttons with icons in start and end positions."),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_3()
                    .items_center()
                    .child(
                        Button::new("icon-start-btn")
                            .icon("+")
                            .label("Add Item")
                            .variant(ButtonVariant::Primary),
                    )
                    .child(
                        Button::new("icon-end-btn")
                            .icon(">")
                            .label("Next")
                            .icon_position(IconPosition::End),
                    )
                    .child(
                        Button::new("icon-only-btn")
                            .icon("X")
                            .variant(ButtonVariant::Ghost),
                    ),
            );
        container = container.child(icon_section);

        // Section 4: Disabled State
        let disabled_section = section("Disabled", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Disabled buttons show reduced opacity and ignore clicks."),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_3()
                    .items_center()
                    .child(
                        Button::new("disabled-primary")
                            .label("Primary")
                            .variant(ButtonVariant::Primary)
                            .disabled(true),
                    )
                    .child(
                        Button::new("disabled-secondary")
                            .label("Secondary")
                            .disabled(true),
                    )
                    .child(
                        Button::new("disabled-ghost")
                            .label("Ghost")
                            .variant(ButtonVariant::Ghost)
                            .disabled(true),
                    ),
            );
        container = container.child(disabled_section);

        // Section 5: Selected State
        let selected_section = section("Selected", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Selected buttons show a distinct selected background and border."),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_3()
                    .items_center()
                    .child(Button::new("selected-btn").label("Selected").selected(true))
                    .child(Button::new("not-selected-btn").label("Not Selected")),
            );
        container = container.child(selected_section);

        // Section 6: Full Width
        let full_width_section = section("Full Width", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Button taking the full width of its container."),
            )
            .child(
                div().w(px(300.0)).child(
                    Button::new("full-width-btn")
                        .label("Full Width Button")
                        .variant(ButtonVariant::Primary)
                        .full_width(),
                ),
            );
        container = container.child(full_width_section);

        // Section 7: State Matrix
        let matrix = StateMatrix::from_contract(&self.contract());
        let matrix_element = matrix.render(
            |state, variant, _window, cx| render_button_state_cell(state, variant, cx),
            window,
            cx,
        );
        container = container.child(matrix_element);

        container.into_any_element()
    }
}

/// Render a single cell in the state matrix.
fn render_button_state_cell(
    state: ComponentState,
    variant: Option<&str>,
    _cx: &mut App,
) -> AnyElement {
    let btn_variant = match variant {
        Some("Primary") => ButtonVariant::Primary,
        Some("Ghost") => ButtonVariant::Ghost,
        Some("Danger") => ButtonVariant::Danger,
        _ => ButtonVariant::Secondary,
    };

    let variant_label = variant.unwrap_or("Secondary");
    let id_str = format!("matrix-{variant_label}-{state:?}");

    let mut btn = Button::new(SharedString::from(id_str))
        .label(SharedString::from(variant_label.to_string()))
        .variant(btn_variant);

    match state {
        ComponentState::Disabled => {
            btn = btn.disabled(true);
        }
        ComponentState::Selected => {
            btn = btn.selected(true);
        }
        _ => {}
    }

    btn.into_any_element()
}
