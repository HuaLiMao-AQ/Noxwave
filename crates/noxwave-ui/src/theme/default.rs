use gpui::{Hsla, rgb};

use super::theme::{Appearance, Theme, ThemeColors};

fn color(hex: u32) -> Hsla {
    rgb(hex).into()
}

/// Creates the built-in light theme.
pub(crate) fn light() -> Theme {
    Theme {
        id: "light".to_owned(),
        appearance: Appearance::Light,
        colors: ThemeColors {
            background: color(0xffffff),
            surface: color(0xf6f8fa),
            elevated_surface: color(0xffffff),
            text: color(0x1f2328),
            text_muted: color(0x656d76),
            text_disabled: color(0x8c959f),
            border: color(0xd0d7de),
            border_focused: color(0x0969da),
            accent: color(0x0969da),
            accent_hover: color(0x0550ae),
            selection: color(0xddf4ff),
            error: color(0xcf222e),
            warning: color(0x9a6700),
            success: color(0x1a7f37),
        },
    }
}

impl Default for Theme {
    fn default() -> Self {
        light()
    }
}
