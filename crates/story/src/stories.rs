//! Built-in stories for the three POC components: Dialog, Select, and Tabs.
//!
//! Each story implements the [`Story`] trait, providing:
//! - Component contract metadata for state matrix generation
//! - A full rendering demonstrating default, variant, and state configurations
//! - Theme-aware rendering (all surfaces resolve from the active theme)
//!
//! Stories render components in isolation — no inter-component dependencies.

mod dialog_story;
mod select_story;
mod tabs_story;

pub use dialog_story::DialogStory;
pub use select_story::SelectStory;
pub use tabs_story::TabsStory;
