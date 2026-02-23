#![recursion_limit = "2048"]

pub mod contracts;
pub mod dialog;
pub mod select;
pub mod tabs;

pub use contracts::{
    AcceptanceChecklist, ComponentContract, ComponentState, ContractBuilder, Disposition,
    InteractionChecklist, PerfEvidence, PropDef, SharedIdentifiers, TokenRef, ValidationError,
};
pub use dialog::Dialog;
pub use select::{Select, SelectItem};
pub use tabs::{TabItem, Tabs};

pub fn init(_cx: &mut gpui::App) {
    // Component initialization will register all components here.
}
