//! State matrix renderer: displays all variant x state combinations for a component.
//!
//! The state matrix uses [`ComponentContract`] metadata to determine which states
//! and variants a component supports, then renders each combination in a grid.
//! This reduces boilerplate: instead of manually writing every combination in the
//! story, the matrix generates the grid from contract metadata.

use components::{ComponentContract, ComponentState};
use gpui::*;
use theme::ActiveTheme;

/// A state matrix derived from a [`ComponentContract`].
///
/// Contains the list of states and variants to render in a grid. Stories use
/// this to drive their rendering without manually enumerating every combination.
#[derive(Debug, Clone)]
pub struct StateMatrix {
    /// Component name for the matrix header.
    name: String,
    /// The states this component can enter.
    states: Vec<ComponentState>,
    /// Named variants the component supports (may be empty for components
    /// without explicit visual variants).
    variants: Vec<String>,
    /// Token paths the component depends on (for documentation display).
    token_paths: Vec<String>,
}

impl StateMatrix {
    /// Build a state matrix from a component contract.
    pub fn from_contract(contract: &ComponentContract) -> Self {
        Self {
            name: contract.name.clone(),
            states: contract.states.clone(),
            variants: contract.variants.clone(),
            token_paths: contract
                .token_dependencies
                .iter()
                .map(|t| t.path.clone())
                .collect(),
        }
    }

    /// The component states from the contract.
    pub fn states(&self) -> &[ComponentState] {
        &self.states
    }

    /// The component variants from the contract.
    pub fn variants(&self) -> &[String] {
        &self.variants
    }

    /// The token dependency paths from the contract.
    pub fn token_paths(&self) -> &[String] {
        &self.token_paths
    }

    /// Render the state matrix as a visual grid.
    ///
    /// The `render_cell` callback is invoked for each (state, variant_index) pair.
    /// For components with no explicit variants, it is called once per state with
    /// `variant_index = 0`.
    ///
    /// The matrix renders:
    /// - A header row with the component name and state labels
    /// - One row per variant (or one row if no variants)
    /// - Each cell shows the component rendered in that state configuration
    pub fn render(
        &self,
        render_cell: impl Fn(ComponentState, Option<&str>, &mut Window, &mut App) -> AnyElement,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        let theme = cx.theme();
        let text_color = theme.text.default;
        let muted_color = theme.text.muted;
        let border_color = theme.border.default;
        let surface_bg = theme.surface.surface;

        let mut container = div().flex().flex_col().gap_4().p_4().w_full();

        // Matrix header
        container = container.child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_lg()
                        .font_weight(FontWeight::BOLD)
                        .text_color(text_color)
                        .child(format!("{} — State Matrix", self.name)),
                )
                .child(div().text_xs().text_color(muted_color).child(format!(
                    "{} states · {} variants · {} token deps",
                    self.states.len(),
                    if self.variants.is_empty() {
                        1
                    } else {
                        self.variants.len()
                    },
                    self.token_paths.len()
                ))),
        );

        // State labels header row
        let mut header_row = div().flex().flex_row().gap_2().child(
            div()
                .w(px(100.0))
                .flex_shrink_0()
                .text_xs()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(muted_color)
                .child("Variant / State"),
        );

        for state in &self.states {
            header_row = header_row.child(
                div()
                    .flex_1()
                    .min_w(px(120.0))
                    .text_xs()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(muted_color)
                    .child(format!("{:?}", state)),
            );
        }
        container = container.child(header_row);

        // Render rows: one per variant (or one "default" row if no variants)
        let variant_labels: Vec<Option<String>> = if self.variants.is_empty() {
            vec![None]
        } else {
            self.variants.iter().map(|v| Some(v.clone())).collect()
        };

        for variant_label in &variant_labels {
            let label_display: SharedString = variant_label
                .as_deref()
                .unwrap_or("default")
                .to_string()
                .into();
            let mut row = div().flex().flex_row().gap_2().items_start().child(
                div()
                    .w(px(100.0))
                    .flex_shrink_0()
                    .text_xs()
                    .text_color(muted_color)
                    .pt_2()
                    .child(label_display),
            );

            for &state in &self.states {
                let cell_element = render_cell(state, variant_label.as_deref(), window, cx);

                row = row.child(
                    div()
                        .flex_1()
                        .min_w(px(120.0))
                        .min_h(px(60.0))
                        .p_2()
                        .bg(surface_bg)
                        .border_1()
                        .border_color(border_color)
                        .rounded_md()
                        .overflow_hidden()
                        .child(cell_element),
                );
            }

            container = container.child(row);
        }

        // Token dependencies section
        if !self.token_paths.is_empty() {
            let mut tokens_section = div().flex().flex_col().gap_1().pt_4().child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(muted_color)
                    .child("Token Dependencies"),
            );

            let mut token_row = div().flex().flex_row().flex_wrap().gap_1();
            for path in &self.token_paths {
                token_row = token_row.child(
                    div()
                        .text_xs()
                        .text_color(muted_color)
                        .px_2()
                        .py(px(2.0))
                        .bg(surface_bg)
                        .border_1()
                        .border_color(border_color)
                        .rounded_sm()
                        .child(path.clone()),
                );
            }
            tokens_section = tokens_section.child(token_row);
            container = container.child(tokens_section);
        }

        container.into_any_element()
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Render a labeled section header for story content.
///
/// Used by individual stories to group related component examples.
pub fn section(title: impl Into<SharedString>, cx: &App) -> Div {
    let theme = cx.theme();
    div()
        .flex()
        .flex_col()
        .gap_3()
        .p_4()
        .bg(theme.surface.surface)
        .border_1()
        .border_color(theme.border.default)
        .rounded_lg()
        .child(
            div()
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(theme.text.default)
                .pb_2()
                .border_b_1()
                .border_color(theme.border.default)
                .child(title.into()),
        )
}

/// Render a state label badge for a state matrix cell.
pub fn state_badge(label: impl Into<SharedString>, cx: &App) -> Div {
    let theme = cx.theme();
    div()
        .text_xs()
        .text_color(theme.text.muted)
        .px_2()
        .py(px(1.0))
        .bg(theme.ghost_element.background)
        .rounded_sm()
        .child(label.into())
}
