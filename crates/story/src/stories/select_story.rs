//! Select story: demonstrates all Select states and configurations.
//!
//! Renders multiple Select instances showing:
//! - Default (closed, with placeholder)
//! - With selected value
//! - Open with dropdown visible
//! - Disabled state
//! - With disabled items in the list
//! - State matrix showing Open, Focused, Hover, Active, Selected, Disabled

use crate::{
    Story,
    matrix::{StateMatrix, section},
};
use components::{ComponentContract, ComponentState, Select, SelectItem};
use gpui::prelude::FluentBuilder;
use gpui::*;
use theme::ActiveTheme;

/// Story for the Select component.
///
/// Demonstrates trigger + popover dropdown, arrow-key navigation, controlled/uncontrolled
/// selection, disabled state, and token-driven styling.
pub struct SelectStory;

impl Story for SelectStory {
    fn name(&self) -> &'static str {
        "Select"
    }

    fn description(&self) -> &'static str {
        "Trigger button + popover dropdown with keyboard navigation and selection."
    }

    fn contract(&self) -> ComponentContract {
        Select::contract()
    }

    fn render_story(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        let theme = cx.theme();
        let muted_color = theme.text.muted;

        let mut container = div().flex().flex_col().gap_6().p_4().w_full();

        // Section 1: Default (closed, placeholder)
        let default_section = section("Default Select", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Closed select with placeholder text."),
            )
            .child(render_select_preview(
                "default",
                &sample_items(),
                None,
                false, // closed
                false, // not disabled
                0,     // no highlight
                cx,
            ));
        container = container.child(default_section);

        // Section 2: With Selected Value
        let selected_section = section("With Selected Value", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Select with a pre-selected value showing in the trigger."),
            )
            .child(render_select_preview(
                "selected",
                &sample_items(),
                Some(1), // "Banana" selected
                false,
                false,
                1,
                cx,
            ));
        container = container.child(selected_section);

        // Section 3: Open with Dropdown
        let open_section = section("Open Dropdown", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Select with dropdown open, showing all items with highlight."),
            )
            .child(render_select_preview(
                "open",
                &sample_items(),
                Some(1),
                true, // open
                false,
                2, // Cherry highlighted
                cx,
            ));
        container = container.child(open_section);

        // Section 4: Disabled
        let disabled_section = section("Disabled Select", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Select in disabled state. Cannot open or interact."),
            )
            .child(render_select_preview(
                "disabled",
                &sample_items(),
                Some(0),
                false,
                true, // disabled
                0,
                cx,
            ));
        container = container.child(disabled_section);

        // Section 5: With Disabled Items
        let disabled_items_section = section("With Disabled Items", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Open dropdown showing mix of enabled and disabled items."),
            )
            .child(render_select_preview(
                "disabled-items",
                &sample_items_with_disabled(),
                None,
                true,
                false,
                0,
                cx,
            ));
        container = container.child(disabled_items_section);

        // Section 6: State Matrix
        let matrix = StateMatrix::from_contract(&self.contract());
        let matrix_element = matrix.render(
            |state, _variant, _window, cx| render_select_state_cell(state, cx),
            window,
            cx,
        );
        container = container.child(matrix_element);

        container.into_any_element()
    }
}

/// Sample items for select stories.
fn sample_items() -> Vec<SelectItem> {
    vec![
        SelectItem::new("Apple"),
        SelectItem::new("Banana"),
        SelectItem::new("Cherry"),
        SelectItem::new("Date"),
        SelectItem::new("Elderberry"),
    ]
}

/// Sample items with some disabled entries.
fn sample_items_with_disabled() -> Vec<SelectItem> {
    vec![
        SelectItem::new("Apple"),
        SelectItem::disabled("Banana (sold out)"),
        SelectItem::new("Cherry"),
        SelectItem::disabled("Date (seasonal)"),
        SelectItem::new("Elderberry"),
    ]
}

/// Render a select preview (trigger + optional dropdown) for story display.
fn render_select_preview(
    id: &str,
    items: &[SelectItem],
    selected_index: Option<usize>,
    is_open: bool,
    is_disabled: bool,
    highlighted_index: usize,
    cx: &App,
) -> AnyElement {
    let theme = cx.theme();
    let trigger_bg = theme.element.background;
    let trigger_hover = theme.element.hover;
    let border_color = theme.border.default;
    let text_color = theme.text.default;
    let placeholder_color = theme.text.placeholder;
    let disabled_color = theme.text.disabled;
    let popover_bg = theme.surface.elevated_surface;
    let item_hover = theme.ghost_element.hover;
    let item_selected = theme.ghost_element.selected;

    let display_text = if let Some(idx) = selected_index {
        items
            .get(idx)
            .map(|i| i.label.clone())
            .unwrap_or_else(|| "Select...".into())
    } else {
        "Select...".into()
    };

    let display_color = if is_disabled {
        disabled_color
    } else if selected_index.is_some() {
        text_color
    } else {
        placeholder_color
    };

    // Trigger button
    let trigger = div()
        .id(ElementId::Name(format!("select-story-{}", id).into()))
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .w(px(200.0))
        .h_8()
        .px_3()
        .bg(trigger_bg)
        .border_1()
        .border_color(border_color)
        .rounded_md()
        .cursor_pointer()
        .when(!is_disabled, |this| this.hover(|s| s.bg(trigger_hover)))
        .when(is_disabled, |this| this.opacity(0.5).cursor_default())
        .child(
            div()
                .text_sm()
                .text_color(display_color)
                .overflow_x_hidden()
                .child(display_text),
        )
        .child(
            div()
                .text_xs()
                .text_color(theme.icon.muted)
                .child(if is_open { "^" } else { "v" }),
        );

    let mut wrapper = div().flex().flex_col().relative();
    wrapper = wrapper.child(trigger);

    // Dropdown list if open
    if is_open && !is_disabled {
        let mut list = div()
            .mt_1()
            .w(px(200.0))
            .max_h(px(320.0))
            .overflow_hidden()
            .bg(popover_bg)
            .border_1()
            .border_color(border_color)
            .rounded_md()
            .shadow_lg()
            .py_1();

        for (idx, item) in items.iter().enumerate() {
            let is_sel = selected_index == Some(idx);
            let is_hl = highlighted_index == idx;
            let is_item_disabled = item.disabled;

            let item_bg = if is_sel {
                item_selected
            } else if is_hl {
                item_hover
            } else {
                Hsla::transparent_black()
            };

            let item_text_color = if is_item_disabled {
                disabled_color
            } else {
                text_color
            };

            list = list.child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .px_3()
                    .py_1()
                    .text_sm()
                    .text_color(item_text_color)
                    .bg(item_bg)
                    .rounded_sm()
                    .mx_1()
                    .when(!is_item_disabled, |this| this.cursor_pointer())
                    .when(is_item_disabled, |this| this.cursor_default().opacity(0.5))
                    .child(item.label.clone())
                    .when(is_sel, |this| {
                        this.child(
                            div()
                                .ml_auto()
                                .text_xs()
                                .text_color(theme.text.accent)
                                .child("*"),
                        )
                    }),
            );
        }

        wrapper = wrapper.child(list);
    }

    wrapper.into_any_element()
}

/// Render a state matrix cell for a given Select state.
fn render_select_state_cell(state: ComponentState, cx: &App) -> AnyElement {
    let theme = cx.theme();
    let text_color = theme.text.default;
    let muted_color = theme.text.muted;
    let border_color = theme.border.default;

    match state {
        ComponentState::Open => {
            // Mini trigger + dropdown
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .h(px(20.0))
                        .w_full()
                        .bg(theme.element.background)
                        .border_1()
                        .border_color(border_color)
                        .rounded_sm()
                        .px_1()
                        .flex()
                        .items_center()
                        .child(div().text_xs().text_color(text_color).child("Apple ^")),
                )
                .child(
                    div()
                        .w_full()
                        .bg(theme.surface.elevated_surface)
                        .border_1()
                        .border_color(border_color)
                        .rounded_sm()
                        .p_1()
                        .child(
                            div()
                                .text_xs()
                                .text_color(muted_color)
                                .child("Item 1\nItem 2"),
                        ),
                )
                .into_any_element()
        }
        ComponentState::Focused => div()
            .h(px(20.0))
            .w_full()
            .bg(theme.element.background)
            .border_2()
            .border_color(theme.border.focused)
            .rounded_sm()
            .px_1()
            .flex()
            .items_center()
            .child(div().text_xs().text_color(text_color).child("Apple v"))
            .into_any_element(),
        ComponentState::Hover => div()
            .h(px(20.0))
            .w_full()
            .bg(theme.element.hover)
            .border_1()
            .border_color(border_color)
            .rounded_sm()
            .px_1()
            .flex()
            .items_center()
            .child(div().text_xs().text_color(text_color).child("Apple v"))
            .into_any_element(),
        ComponentState::Active => div()
            .h(px(20.0))
            .w_full()
            .bg(theme.element.active)
            .border_1()
            .border_color(border_color)
            .rounded_sm()
            .px_1()
            .flex()
            .items_center()
            .child(div().text_xs().text_color(text_color).child("Apple v"))
            .into_any_element(),
        ComponentState::Selected => div()
            .h(px(20.0))
            .w_full()
            .bg(theme.element.background)
            .border_1()
            .border_color(border_color)
            .rounded_sm()
            .px_1()
            .flex()
            .items_center()
            .child(div().text_xs().text_color(text_color).child("Banana v"))
            .into_any_element(),
        ComponentState::Disabled => div()
            .h(px(20.0))
            .w_full()
            .bg(theme.element.background)
            .border_1()
            .border_color(border_color)
            .rounded_sm()
            .px_1()
            .flex()
            .items_center()
            .opacity(0.5)
            .cursor_default()
            .child(
                div()
                    .text_xs()
                    .text_color(theme.text.disabled)
                    .child("Select... v"),
            )
            .into_any_element(),
        _ => div()
            .text_xs()
            .text_color(muted_color)
            .child(format!("{:?} N/A", state))
            .into_any_element(),
    }
}
