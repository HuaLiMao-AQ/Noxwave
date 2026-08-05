//! Root page container responsible for selecting and rendering the active page.

use gpui::*;
use gpui_platform::application;

use crate::{pages::HomePage, theme::Theme};

/// Pages that can be displayed by the root container.
pub(crate) enum Page {
    /// The application's home page.
    Home,
}

/// The root GPUI entity that owns page state and page entities.
pub(crate) struct Root {
    /// The page currently selected for rendering.
    current_page: Page,

    /// The home page entity kept by the root container.
    home_page: Entity<HomePage>,
}

impl Root {
    /// Creates the root entity and its initial page entity.
    fn new(cx: &mut App) -> Self {
        Self {
            current_page: Page::Home,
            home_page: cx.new(|_| HomePage::new()),
        }
    }

    /// Starts the GPUI application and opens the root window.
    pub(crate) fn run() {
        application().run(move |cx| {
            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("Noxwave".into()),
                        appears_transparent: true,
                        traffic_light_position: None,
                    }),
                    ..Default::default()
                },
                |_window, cx| cx.new(|cx| Root::new(cx)),
            )
            .expect("failed to open window");
        });
    }
}

impl Render for Root {
    /// Renders the entity associated with the currently selected page.
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::prelude::Context<Self>,
    ) -> impl gpui::prelude::IntoElement {
        match self.current_page {
            Page::Home => self.home_page.clone(),
        }
    }
}
