#![recursion_limit = "2048"]
//! Story framework: trait-based story system for rendering components in the workbench.
//!
//! Stories enable systematic visual validation of every variant and state combination
//! for each component. The framework provides:
//!
//! - A [`Story`] trait that components implement to provide workbench rendering
//! - A [`StoryRegistry`] for discovering available stories at runtime
//! - A [`StateMatrix`] renderer that displays all variant x state combinations
//!
//! # Why trait-based?
//! Traits are Rust-idiomatic, allow compile-time verification, and keep stories
//! co-located with the components they exercise. Adding a new story only requires
//! implementing the trait and calling `StoryRegistry::register()`.

pub mod matrix;
pub mod stories;

use components::ComponentContract;
use gpui::*;

// Re-export for convenience.
pub use matrix::StateMatrix;
pub use stories::{DialogStory, SelectStory, TabsStory};

// ---------------------------------------------------------------------------
// Story trait
// ---------------------------------------------------------------------------

/// A story renders one or more visual examples of a component for the workbench.
///
/// Implementing this trait is the only requirement for adding a new component
/// story to the workbench. The story framework discovers stories through the
/// [`StoryRegistry`] global.
pub trait Story {
    /// Human-readable name shown in the workbench sidebar.
    fn name(&self) -> &'static str;

    /// Optional description shown below the name.
    fn description(&self) -> &'static str {
        ""
    }

    /// The component contract for the component this story exercises.
    /// Used to generate state matrix columns and validate coverage.
    fn contract(&self) -> ComponentContract;

    /// Render the story content. This is the main rendering entry point.
    ///
    /// Implementations should render the component in multiple configurations
    /// demonstrating all variants and states.
    fn render_story(&self, window: &mut Window, cx: &mut App) -> AnyElement;
}

// ---------------------------------------------------------------------------
// StoryEntry (type-erased story storage)
// ---------------------------------------------------------------------------

/// A type-erased story entry stored in the registry.
///
/// Wraps a `Box<dyn Story>` so the registry can hold heterogeneous story types.
pub struct StoryEntry {
    story: Box<dyn Story>,
}

impl StoryEntry {
    /// Create a new entry from a concrete story implementation.
    pub fn new(story: impl Story + 'static) -> Self {
        Self {
            story: Box::new(story),
        }
    }

    /// Access the inner story trait object.
    pub fn story(&self) -> &dyn Story {
        self.story.as_ref()
    }

    /// The story name (convenience delegate).
    pub fn name(&self) -> &'static str {
        self.story.name()
    }

    /// The story description (convenience delegate).
    pub fn description(&self) -> &'static str {
        self.story.description()
    }

    /// The component contract (convenience delegate).
    pub fn contract(&self) -> ComponentContract {
        self.story.contract()
    }

    /// Render the story (convenience delegate).
    pub fn render_story(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        self.story.render_story(window, cx)
    }
}

// ---------------------------------------------------------------------------
// StoryRegistry
// ---------------------------------------------------------------------------

/// Registry of all available stories, stored as a GPUI global.
///
/// Stories register themselves during `story::init(cx)` and the workbench
/// reads the registry to populate its sidebar and render story content.
pub struct StoryRegistry {
    entries: Vec<StoryEntry>,
}

impl Global for StoryRegistry {}

impl StoryRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Register a new story.
    pub fn register(&mut self, story: impl Story + 'static) {
        self.entries.push(StoryEntry::new(story));
    }

    /// Returns the number of registered stories.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true if no stories are registered.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns a slice of all registered story entries.
    pub fn entries(&self) -> &[StoryEntry] {
        &self.entries
    }

    /// Look up a story by name. Returns None if not found.
    pub fn get(&self, name: &str) -> Option<&StoryEntry> {
        self.entries.iter().find(|e| e.name() == name)
    }

    /// Returns an iterator over all story names.
    pub fn names(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.entries.iter().map(|e| e.name())
    }
}

impl Default for StoryRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Initialization
// ---------------------------------------------------------------------------

/// Initialize the story framework.
///
/// Registers the [`StoryRegistry`] global and populates it with the built-in
/// stories for all POC components (Dialog, Select, Tabs).
///
/// Must be called after `theme::init(cx)` and `components::init(cx)`.
pub fn init(cx: &mut App) {
    let mut registry = StoryRegistry::new();

    // Register all built-in POC stories.
    registry.register(DialogStory);
    registry.register(SelectStory);
    registry.register(TabsStory);

    cx.set_global(registry);
}

// Tests are in tests/story_tests.rs (integration test) to avoid
// stack overflow from GPUI IntoElement derive macro expansion in test mode.
// See AGENTS.md: "Test stack overflow" pattern.
