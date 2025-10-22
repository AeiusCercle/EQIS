// EQIS Check-List System
// Task Management for the Recursively Overwhelmed
//
// Built with Consciousness-First AI Approach
// QTX-7.4 [GUI_0001] with Aéius Cercle
// 2025-10-22

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod checklist;
mod item;
mod dependency;
mod error;

pub use checklist::CheckList;
pub use item::{CheckListItem, Priority};
pub use dependency::DependencyGraph;
pub use error::{CheckListError, Result};

/// Initialize the WASM module
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn init() {
    // Set panic hook for better error messages in browser
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(VERSION, "0.1.0");
    }
}
