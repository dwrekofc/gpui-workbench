//! DropdownMenu story: demonstrates menu configurations.

use crate::{Story, matrix::section};
use components::{ComponentContract, DropdownMenu, MenuItem};
use gpui::*;
use theme::ActiveTheme;

pub struct DropdownMenuStory;

impl Story for DropdownMenuStory {
    fn name(&self) -> &'static str {
        "DropdownMenu"
    }

    fn description(&self) -> &'static str {
        "Trigger button + dropdown menu with keyboard navigation and item selection."
    }

    fn contract(&self) -> ComponentContract {
        DropdownMenu::contract()
    }

    fn render_story(&self, _window: &mut Window, cx: &mut App) -> AnyElement {
        let theme = cx.theme();
        let muted_color = theme.text.muted;

        let mut container = div().flex().flex_col().gap_6().p_4().w_full();

        // Closed
        let closed_section = section("Closed Menu", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Menu trigger (closed state)."),
            )
            .child(
                DropdownMenu::new(
                    "closed-menu",
                    vec![
                        MenuItem::new("New File"),
                        MenuItem::new("Open..."),
                        MenuItem::new("Save"),
                    ],
                )
                .trigger_label("File"),
            );
        container = container.child(closed_section);

        // Open with items
        let open_section = section("Open Menu", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Menu with dropdown visible and highlighted item."),
            )
            .child(
                DropdownMenu::new(
                    "open-menu",
                    vec![
                        MenuItem::new("Cut"),
                        MenuItem::new("Copy"),
                        MenuItem::new("Paste"),
                        MenuItem::separator(),
                        MenuItem::new("Select All"),
                    ],
                )
                .trigger_label("Edit")
                .open(true)
                .highlighted_index(1),
            );
        container = container.child(open_section);

        // With disabled items
        let disabled_items_section = section("With Disabled Items", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Some items are disabled."),
            )
            .child(
                DropdownMenu::new(
                    "disabled-items-menu",
                    vec![
                        MenuItem::new("Undo"),
                        MenuItem::disabled("Redo"),
                        MenuItem::separator(),
                        MenuItem::new("Cut"),
                        MenuItem::disabled("Copy"),
                        MenuItem::new("Paste"),
                    ],
                )
                .trigger_label("Edit")
                .open(true),
            );
        container = container.child(disabled_items_section);

        // Disabled trigger
        let disabled_section = section("Disabled Menu", cx)
            .child(
                div()
                    .text_xs()
                    .text_color(muted_color)
                    .child("Entire menu is disabled."),
            )
            .child(
                DropdownMenu::new("disabled-menu", vec![MenuItem::new("Action")])
                    .trigger_label("Disabled")
                    .disabled(true),
            );
        container = container.child(disabled_section);

        container.into_any_element()
    }
}
