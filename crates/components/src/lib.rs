#![recursion_limit = "2048"]

pub mod button;
pub mod checkbox;
pub mod contracts;
pub mod dialog;
pub mod dropdown_menu;
pub mod input;
pub mod popover;
pub mod radio;
pub mod select;
pub mod tabs;
pub mod textarea;
pub mod toast;
pub mod tooltip;

pub use button::{Button, ButtonSize, ButtonVariant, IconPosition};
pub use checkbox::Checkbox;
pub use contracts::{
    AcceptanceChecklist, ComponentContract, ComponentState, ContractBuilder, Disposition,
    InteractionChecklist, PerfEvidence, PropDef, SharedIdentifiers, TokenRef, ValidationError,
};
pub use dialog::Dialog;
pub use dropdown_menu::{DropdownMenu, MenuItem};
pub use input::{Input, InputSize};
pub use popover::Popover;
pub use radio::{Radio, RadioItem};
pub use select::{Select, SelectItem};
pub use tabs::{TabItem, Tabs};
pub use textarea::Textarea;
pub use toast::{Toast, ToastVariant};
pub use tooltip::{Tooltip, TooltipPlacement};

pub fn init(_cx: &mut gpui::App) {
    // Component initialization will register all components here.
}
