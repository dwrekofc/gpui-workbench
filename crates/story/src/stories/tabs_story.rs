//! Tabs story: demonstrates all Tabs states and configurations.
//!
//! Renders multiple Tabs instances showing:
//! - Default tabs (2 tabs, first active)
//! - Multiple tabs (5 tabs)
//! - Tabs with disabled entries
//! - Different active tab states
//! - State matrix showing Focused, Hover, Active, Selected, Disabled

use crate::{
    Story,
    matrix::{StateMatrix, section},
};
use components::{ComponentContract, ComponentState, Tabs};
use gpui::prelude::FluentBuilder;
use gpui::*;
use theme::ActiveTheme;

/// Story for the Tabs component.
///
/// Demonstrates tab bar with arrow-key navigation, active tab indication,
/// disabled tabs, content panel switching, and token-driven styling.
pub struct TabsStory;

impl Story for TabsStory {
    fn name(&self) -> &'static str {
        "Tabs"
    }

    fn description(&self) -> &'static str {
        "Tab bar with arrow-key navigation, active indicator, and content panels."
    }

    fn contract(&self) -> ComponentContract {
        Tabs::contract()
    }

    fn render_story(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        let theme = cx.theme();
        let muted_color = theme.text.muted;

        let mut container = div().flex().flex_col().gap_6().p_4().w_full();

        // Section 1: Default (2 tabs)
        let default_section = section("Default Tabs (2 tabs)", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Simple tab bar with two tabs, first active."),
            )
            .child(render_tabs_preview(
                "default-2",
                &["Overview", "Details"],
                0,
                &[],
                cx,
            ));
        container = container.child(default_section);

        // Section 2: Multiple tabs (5 tabs)
        let multi_section = section("Multiple Tabs (5 tabs)", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Tab bar with five tabs, third active."),
            )
            .child(render_tabs_preview(
                "multi-5",
                &["Home", "Profile", "Settings", "Notifications", "Help"],
                2,
                &[],
                cx,
            ));
        container = container.child(multi_section);

        // Section 3: With Disabled Tab
        let disabled_section = section("With Disabled Tab", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Tab bar with some tabs disabled. Disabled tabs are dimmed and skipped during keyboard nav."),
            )
            .child(render_tabs_preview(
                "disabled",
                &["Active", "Disabled", "Also Active", "Also Disabled"],
                0,
                &[1, 3], // indices of disabled tabs
                cx,
            ));
        container = container.child(disabled_section);

        // Section 4: Different Active States
        let active_section = section("Active Tab Variations", cx).child(
            div()
                .text_xs()
                .text_color(muted_color)
                .child("Same tabs with different active indices to show the active indicator."),
        );
        let mut active_container = div().flex().flex_col().gap_4();
        for active_idx in 0..3 {
            active_container = active_container.child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_xs()
                            .text_color(muted_color)
                            .child(format!("Active: tab {}", active_idx)),
                    )
                    .child(render_tabs_preview(
                        &format!("active-{}", active_idx),
                        &["Tab A", "Tab B", "Tab C"],
                        active_idx,
                        &[],
                        cx,
                    )),
            );
        }
        let active_section = active_section.child(active_container);
        container = container.child(active_section);

        // Section 5: State Matrix
        let matrix = StateMatrix::from_contract(&self.contract());
        let matrix_element = matrix.render(
            |state, _variant, _window, cx| render_tabs_state_cell(state, cx),
            window,
            cx,
        );
        container = container.child(matrix_element);

        container.into_any_element()
    }
}

/// Render a tabs preview for story display.
fn render_tabs_preview(
    id: &str,
    labels: &[&str],
    active_index: usize,
    disabled_indices: &[usize],
    cx: &App,
) -> AnyElement {
    // Convert to owned SharedStrings for GPUI element builders requiring 'static.
    let labels: Vec<SharedString> = labels
        .iter()
        .map(|s| SharedString::from(s.to_string()))
        .collect();
    let theme = cx.theme();
    let bar_bg = theme.tab.bar_background;
    let active_bg = theme.tab.active_background;
    let inactive_bg = theme.tab.inactive_background;
    let border_color = theme.border.default;
    let selected_border = theme.border.selected;
    let text_color = theme.text.default;
    let muted_color = theme.text.muted;
    let disabled_color = theme.text.disabled;
    let hover_bg = theme.ghost_element.hover;

    // Build tab bar
    let mut tab_bar = div()
        .id(ElementId::Name(format!("tabs-story-{}", id).into()))
        .flex()
        .flex_row()
        .w_full()
        .h_9()
        .bg(bar_bg)
        .border_b_1()
        .border_color(border_color);

    for (idx, label) in labels.iter().enumerate() {
        let is_active = idx == active_index;
        let is_disabled = disabled_indices.contains(&idx);

        let tab_bg = if is_active { active_bg } else { inactive_bg };
        let tab_text = if is_disabled {
            disabled_color
        } else if is_active {
            text_color
        } else {
            muted_color
        };

        let tab_el = div()
            .flex()
            .items_center()
            .justify_center()
            .px_4()
            .h_full()
            .text_sm()
            .bg(tab_bg)
            .text_color(tab_text)
            .cursor_pointer()
            .border_b_2()
            .when(is_active, |this| {
                this.border_color(selected_border)
                    .font_weight(FontWeight::MEDIUM)
            })
            .when(!is_active, |this| {
                this.border_color(Hsla::transparent_black())
            })
            .when(!is_disabled && !is_active, |this| {
                this.hover(|s| s.bg(hover_bg))
            })
            .when(is_disabled, |this| this.cursor_default().opacity(0.5))
            .child(label.clone());

        tab_bar = tab_bar.child(tab_el);
    }

    // Content panel for the active tab
    let content_panel = div()
        .p_4()
        .child(div().text_sm().text_color(muted_color).child(format!(
                    "Content for: {}",
                    labels.get(active_index).map(|s| s.as_ref()).unwrap_or("(none)")
                )));

    // Container: tab bar + content
    div()
        .flex()
        .flex_col()
        .w_full()
        .border_1()
        .border_color(border_color)
        .rounded_lg()
        .overflow_hidden()
        .child(tab_bar)
        .child(content_panel)
        .into_any_element()
}

/// Render a state matrix cell for a given Tabs state.
fn render_tabs_state_cell(state: ComponentState, cx: &App) -> AnyElement {
    let theme = cx.theme();
    let text_color = theme.text.default;
    let muted_color = theme.text.muted;
    let border_color = theme.border.default;

    match state {
        ComponentState::Focused => {
            // Tab bar with focus ring
            div()
                .flex()
                .flex_row()
                .h(px(24.0))
                .w_full()
                .bg(theme.tab.bar_background)
                .border_2()
                .border_color(theme.border.focused)
                .rounded_sm()
                .child(
                    div()
                        .px_2()
                        .text_xs()
                        .text_color(text_color)
                        .border_b_2()
                        .border_color(theme.border.selected)
                        .flex()
                        .items_center()
                        .child("Tab A"),
                )
                .child(
                    div()
                        .px_2()
                        .text_xs()
                        .text_color(muted_color)
                        .flex()
                        .items_center()
                        .child("Tab B"),
                )
                .into_any_element()
        }
        ComponentState::Hover => {
            // Tab with hover background
            div()
                .flex()
                .flex_row()
                .h(px(24.0))
                .w_full()
                .bg(theme.tab.bar_background)
                .border_1()
                .border_color(border_color)
                .rounded_sm()
                .child(
                    div()
                        .px_2()
                        .text_xs()
                        .text_color(text_color)
                        .border_b_2()
                        .border_color(theme.border.selected)
                        .flex()
                        .items_center()
                        .child("Tab A"),
                )
                .child(
                    div()
                        .px_2()
                        .text_xs()
                        .text_color(muted_color)
                        .bg(theme.ghost_element.hover)
                        .flex()
                        .items_center()
                        .child("Tab B"),
                )
                .into_any_element()
        }
        ComponentState::Active => {
            // Tab in pressed state
            div()
                .flex()
                .flex_row()
                .h(px(24.0))
                .w_full()
                .bg(theme.tab.bar_background)
                .border_1()
                .border_color(border_color)
                .rounded_sm()
                .child(
                    div()
                        .px_2()
                        .text_xs()
                        .text_color(text_color)
                        .border_b_2()
                        .border_color(theme.border.selected)
                        .flex()
                        .items_center()
                        .child("Tab A"),
                )
                .child(
                    div()
                        .px_2()
                        .text_xs()
                        .text_color(muted_color)
                        .bg(theme.ghost_element.active)
                        .flex()
                        .items_center()
                        .child("Tab B"),
                )
                .into_any_element()
        }
        ComponentState::Selected => {
            // Active tab indicator
            div()
                .flex()
                .flex_row()
                .h(px(24.0))
                .w_full()
                .bg(theme.tab.bar_background)
                .border_1()
                .border_color(border_color)
                .rounded_sm()
                .child(
                    div()
                        .px_2()
                        .text_xs()
                        .text_color(muted_color)
                        .flex()
                        .items_center()
                        .child("Tab A"),
                )
                .child(
                    div()
                        .px_2()
                        .text_xs()
                        .text_color(text_color)
                        .font_weight(FontWeight::MEDIUM)
                        .bg(theme.tab.active_background)
                        .border_b_2()
                        .border_color(theme.border.selected)
                        .flex()
                        .items_center()
                        .child("Tab B"),
                )
                .into_any_element()
        }
        ComponentState::Disabled => {
            // Disabled tab
            div()
                .flex()
                .flex_row()
                .h(px(24.0))
                .w_full()
                .bg(theme.tab.bar_background)
                .border_1()
                .border_color(border_color)
                .rounded_sm()
                .child(
                    div()
                        .px_2()
                        .text_xs()
                        .text_color(text_color)
                        .border_b_2()
                        .border_color(theme.border.selected)
                        .flex()
                        .items_center()
                        .child("Tab A"),
                )
                .child(
                    div()
                        .px_2()
                        .text_xs()
                        .text_color(theme.text.disabled)
                        .opacity(0.5)
                        .flex()
                        .items_center()
                        .child("Tab B"),
                )
                .into_any_element()
        }
        _ => div()
            .text_xs()
            .text_color(muted_color)
            .child(format!("{:?} N/A", state))
            .into_any_element(),
    }
}
