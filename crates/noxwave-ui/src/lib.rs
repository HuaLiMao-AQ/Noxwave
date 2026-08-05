//! Application entry point and top-level orchestration for the Noxwave desktop UI.

mod pages;
mod theme;

use pages::Root;

/// The Noxwave desktop application.
pub struct NoxwaveUI;

impl NoxwaveUI {
    /// Starts the desktop application and opens its root window.
    pub fn run() {
        Root::run();
    }
}
