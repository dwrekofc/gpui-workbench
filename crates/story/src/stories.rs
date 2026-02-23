//! Built-in stories for all implemented components.
//!
//! Each story implements the [`Story`] trait, providing:
//! - Component contract metadata for state matrix generation
//! - A full rendering demonstrating default, variant, and state configurations
//! - Theme-aware rendering (all surfaces resolve from the active theme)
//!
//! Stories render components in isolation — no inter-component dependencies.

mod button_story;
mod checkbox_story;
mod dialog_story;
mod dropdown_menu_story;
mod input_story;
mod popover_story;
mod radio_story;
mod select_story;
mod tabs_story;
mod textarea_story;
mod toast_story;
mod tooltip_story;

pub use button_story::ButtonStory;
pub use checkbox_story::CheckboxStory;
pub use dialog_story::DialogStory;
pub use dropdown_menu_story::DropdownMenuStory;
pub use input_story::InputStory;
pub use popover_story::PopoverStory;
pub use radio_story::RadioStory;
pub use select_story::SelectStory;
pub use tabs_story::TabsStory;
pub use textarea_story::TextareaStory;
pub use toast_story::ToastStory;
pub use tooltip_story::TooltipStory;
