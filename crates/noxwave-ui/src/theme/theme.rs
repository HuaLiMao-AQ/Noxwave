use gpui::Hsla;

/// The light or dark presentation of a theme.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Appearance {
    Light,
    Dark,
}

/// Semantic color roles consumed by UI components.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ThemeColors {
    pub(crate) background: Hsla,
    pub(crate) surface: Hsla,
    pub(crate) elevated_surface: Hsla,
    pub(crate) text: Hsla,
    pub(crate) text_muted: Hsla,
    pub(crate) text_disabled: Hsla,
    pub(crate) border: Hsla,
    pub(crate) border_focused: Hsla,
    pub(crate) accent: Hsla,
    pub(crate) accent_hover: Hsla,
    pub(crate) selection: Hsla,
    pub(crate) error: Hsla,
    pub(crate) warning: Hsla,
    pub(crate) success: Hsla,
}

/// A complete visual theme for the application.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Theme {
    pub(crate) id: String,
    pub(crate) appearance: Appearance,
    pub(crate) colors: ThemeColors,
}
