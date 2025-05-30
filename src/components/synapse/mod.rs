// Core functionality
pub mod core;

// UI components
pub mod ui;

// Modal components
pub mod modals;

// Main components
pub mod synapse_component;
pub mod synapse_with_auth;

// Re-exports for easy access
pub use core::*;
pub use modals::*;
pub use synapse_component::Synapse;
pub use synapse_with_auth::SynapseWithAuth;
pub use ui::*;
