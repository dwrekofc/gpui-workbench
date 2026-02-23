//! DropdownMenu component: trigger + menu with keyboard navigation.
//!
//! Fork disposition: adapted from Zed `dropdown_menu.rs` with gpui-component convenience API.
//! Normalized to internal token/primitive contracts.
//!
//! Provenance:
//! - Zed `crates/ui/src/components/dropdown_menu.rs` (GPL-3.0/AGPL-3.0, Zed Industries)
//! - gpui-component menu patterns (MIT, Zed Industries)
//! - Modifications: Simplified to internal token system, uses internal primitives
//!   for keyboard navigation and popover positioning.

use gpui::prelude::FluentBuilder;
use gpui::*;
use primitives::{Orientation, classify_nav_key, is_activation_key, navigate_index};
use theme::ActiveTheme;

/// A single item in a dropdown menu.
#[derive(Debug, Clone)]
pub struct MenuItem {
    /// Display label.
    pub label: SharedString,
    /// Whether this item is disabled.
    pub disabled: bool,
    /// Whether this item is a separator (visual divider).
    pub separator: bool,
}

impl MenuItem {
    /// Create a new enabled menu item.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            disabled: false,
            separator: false,
        }
    }

    /// Create a disabled menu item.
    pub fn disabled(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            disabled: true,
            separator: false,
        }
    }

    /// Create a separator item.
    pub fn separator() -> Self {
        Self {
            label: SharedString::default(),
            disabled: true,
            separator: true,
        }
    }
}

/// Callback when a menu item is selected.
type OnSelectCallback = Box<dyn Fn(usize, &MenuItem, &mut Window, &mut App) + 'static>;

/// A dropdown menu component with trigger button, popover menu list,
/// keyboard navigation, and builder-pattern API.
///
/// # Usage
/// ```ignore
/// DropdownMenu::new("file-menu", vec![
///     MenuItem::new("New File"),
///     MenuItem::new("Open..."),
///     MenuItem::separator(),
///     MenuItem::new("Save"),
/// ])
///     .trigger_label("File")
///     .on_select(|idx, item, _window, _cx| {
///         println!("Selected: {}", item.label);
///     })
/// ```
#[derive(IntoElement)]
pub struct DropdownMenu {
    id: ElementId,
    items: Vec<MenuItem>,
    trigger_label: SharedString,
    open: bool,
    highlighted_index: usize,
    disabled: bool,
    on_select: Option<OnSelectCallback>,
    tooltip: Option<SharedString>,
    width: Pixels,
}

impl DropdownMenu {
    /// Create a new dropdown menu.
    pub fn new(id: impl Into<ElementId>, items: Vec<MenuItem>) -> Self {
        Self {
            id: id.into(),
            items,
            trigger_label: "Menu".into(),
            open: false,
            highlighted_index: 0,
            disabled: false,
            on_select: None,
            tooltip: None,
            width: px(180.0),
        }
    }

    /// Set the trigger button label.
    pub fn trigger_label(mut self, label: impl Into<SharedString>) -> Self {
        self.trigger_label = label.into();
        self
    }

    /// Set whether the menu is open.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Set the highlighted item index.
    pub fn highlighted_index(mut self, index: usize) -> Self {
        self.highlighted_index = index;
        self
    }

    /// Set the disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the select handler.
    pub fn on_select(
        mut self,
        handler: impl Fn(usize, &MenuItem, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Box::new(handler));
        self
    }

    /// Set a tooltip.
    pub fn set_tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// Set the menu width.
    pub fn width(mut self, width: Pixels) -> Self {
        self.width = width;
        self
    }

    /// Returns the component contract for DropdownMenu.
    pub fn contract() -> crate::ComponentContract {
        use crate::*;
        ComponentContract::builder("DropdownMenu", "0.1.0")
            .disposition(Disposition::Fork)
            .required_prop("id", "ElementId", "Unique identifier for the menu")
            .required_prop("items", "Vec<MenuItem>", "Menu items to display")
            .optional_prop(
                "trigger_label",
                "SharedString",
                "\"Menu\"",
                "Trigger button label",
            )
            .optional_prop("open", "bool", "false", "Whether the menu is open")
            .optional_prop("disabled", "bool", "false", "Whether the menu is disabled")
            .optional_prop("width", "Pixels", "180.0", "Menu dropdown width")
            .optional_prop("tooltip", "Option<SharedString>", "None", "Tooltip text")
            .state(ComponentState::Open)
            .state(ComponentState::Hover)
            .state(ComponentState::Active)
            .state(ComponentState::Focused)
            .state(ComponentState::Disabled)
            .token_dep("element.background", "Trigger button background")
            .token_dep("element.hover", "Trigger and item hover background")
            .token_dep("element.active", "Active item background")
            .token_dep("element.disabled", "Disabled state background")
            .token_dep("surface.elevated_surface", "Menu dropdown background")
            .token_dep("text.default", "Item text color")
            .token_dep("text.muted", "Trigger text color")
            .token_dep("text.disabled", "Disabled item text color")
            .token_dep("border.default", "Menu dropdown border")
            .token_dep("border.variant", "Separator line color")
            .focus_behavior(
                "Tab/Shift-Tab navigates to/from trigger. Focus moves into menu when opened.",
            )
            .keyboard_model(
                "Enter/Space opens menu. Arrow Up/Down navigates items, skipping disabled. \
                 Enter/Space selects item. Escape closes menu.",
            )
            .pointer_behavior("Click trigger toggles menu. Hover highlights items. Click selects.")
            .state_model(
                "Controlled open/close. Highlighted index tracks keyboard focus within menu.",
            )
            .disabled_behavior("Disabled menu ignores all interaction.")
            .required_file("crates/components/src/dropdown_menu.rs")
            .build()
    }
}

impl RenderOnce for DropdownMenu {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        let trigger_bg = theme.element.background;
        let trigger_border = theme.border.default;
        let trigger_text = theme.text.muted;
        let hover_bg = theme.element.hover;
        let disabled = self.disabled;

        // Trigger button
        let mut trigger = div()
            .id(SharedString::from(format!("{}-trigger", self.id)))
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .h_7()
            .px_3()
            .bg(trigger_bg)
            .border_1()
            .border_color(trigger_border)
            .rounded_md()
            .text_sm()
            .text_color(if disabled {
                theme.text.disabled
            } else {
                trigger_text
            })
            .cursor(if disabled {
                CursorStyle::default()
            } else {
                CursorStyle::PointingHand
            })
            .child(self.trigger_label)
            .child(div().text_xs().child("▾"));

        if !disabled {
            trigger = trigger.hover(move |s| s.bg(hover_bg));
        }

        let mut container = div().id(self.id.clone()).flex().flex_col();
        container = container.child(trigger);

        // Dropdown panel (when open)
        if self.open && !disabled {
            let menu_bg = theme.surface.elevated_surface;
            let menu_border = theme.border.default;
            let item_text = theme.text.default;
            let separator_color = theme.border.variant;
            let highlight_bg = theme.element.hover;
            let highlighted = self.highlighted_index;

            let mut menu = div()
                .id(SharedString::from(format!("{}-menu", self.id)))
                .w(self.width)
                .bg(menu_bg)
                .border_1()
                .border_color(menu_border)
                .rounded_md()
                .shadow_lg()
                .mt_1()
                .py_1()
                .overflow_hidden();

            // Keyboard navigation
            let items_for_nav = self.items.clone();
            let item_count = items_for_nav.len();
            menu = menu.on_key_down(move |event, _window, cx| {
                if primitives::is_escape_key(event) {
                    cx.stop_propagation();
                    return;
                }
                let nav = classify_nav_key(event, Orientation::Vertical);
                if let Some(dir) = nav {
                    let _next = navigate_index(highlighted, dir, item_count, |i| {
                        items_for_nav
                            .get(i)
                            .is_some_and(|item| item.disabled || item.separator)
                    });
                    cx.stop_propagation();
                }
                if is_activation_key(event) {
                    cx.stop_propagation();
                }
            });

            for (idx, item) in self.items.iter().enumerate() {
                if item.separator {
                    menu = menu.child(div().h(px(1.0)).mx_2().my_1().bg(separator_color));
                    continue;
                }

                let is_highlighted = idx == self.highlighted_index;
                let item_disabled = item.disabled;

                let item_el = div()
                    .id(SharedString::from(format!("{}-item-{idx}", self.id)))
                    .px_3()
                    .py_1()
                    .text_sm()
                    .text_color(if item_disabled {
                        theme.text.disabled
                    } else {
                        item_text
                    })
                    .cursor(if item_disabled {
                        CursorStyle::default()
                    } else {
                        CursorStyle::PointingHand
                    })
                    .when(is_highlighted && !item_disabled, |el| el.bg(highlight_bg))
                    .when(!item_disabled, move |el| {
                        el.hover(move |s| s.bg(highlight_bg))
                    })
                    .child(item.label.clone());

                menu = menu.child(item_el);
            }

            container = container.child(deferred(menu).with_priority(1));
        }

        container
    }
}
