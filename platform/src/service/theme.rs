use dioxus::prelude::*;

#[derive(Debug, Clone)]
pub struct ThemeData {
    pub font_theme: FontTheme,
}

#[derive(Debug, Clone)]
pub struct FontTheme {
    pub exbold15: String,
    pub bold15: String,
}

impl Default for FontTheme {
    fn default() -> Self {
        FontTheme {
            exbold15: "font-extrabold text-[15px] leading-[22.5px]".to_string(),
            bold15: "font-bold text-[15px] leading[22.5px]".to_string(),
        }
    }
}

impl Default for ThemeData {
    fn default() -> Self {
        ThemeData {
            font_theme: FontTheme::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Theme {
    pub data: Signal<ThemeData>,
}

impl Theme {
    pub fn init() {
        use_context_provider(|| Self {
            data: Signal::new(ThemeData::default()),
        });

        use_context_provider(|| by_components::theme::ColorTheme::default());
    }

    pub fn get_data(&self) -> ThemeData {
        (self.data)()
    }

    pub fn get_font_theme(&self) -> FontTheme {
        (self.data)().font_theme.clone()
    }
}
