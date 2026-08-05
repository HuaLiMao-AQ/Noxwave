//! Home page view for the Noxwave desktop application.

use gpui::*;

use crate::theme::Theme;

/// The initial home page displayed by the desktop application.
pub struct HomePage;

impl HomePage {
    /// Creates a new home page.
    pub fn new() -> Self {
        Self
    }
}

impl Render for HomePage {
    /// Builds the home page's current UI tree.
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .items_center()
            .justify_center()
            .bg(Theme::default().colors.background)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .size_full()
                    .items_center()
                    .justify_center(),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .h(px(64.0))
                    .w_full()
                    .items_center()
                    .justify_center(),
            )
    }
}
