pub mod contracts;

pub use contracts::{
    AcceptanceChecklist, ComponentContract, ComponentState, ContractBuilder, Disposition,
    InteractionChecklist, PerfEvidence, PropDef, SharedIdentifiers, TokenRef, ValidationError,
};

pub fn init(_cx: &mut gpui::App) {
    // Component initialization will register all components here.
}
