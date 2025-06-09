#[derive(Debug, Clone)]
pub enum ColorBlindPalette {
    Normal,
    Protanopia,
    Deuteranopia,
    Tritanopia,
}

#[derive(Debug, Clone)]
pub struct AccessibilitySettings {
    pub palette: ColorBlindPalette,
    pub font_scale: f32,
    pub subtitles: bool,
}

impl Default for AccessibilitySettings {
    fn default() -> Self {
        Self { palette: ColorBlindPalette::Normal, font_scale: 1.0, subtitles: false }
    }
}

#[derive(Debug, Clone)]
pub struct OptionsMenu {
    pub accessibility: AccessibilitySettings,
}

impl OptionsMenu {
    pub fn new() -> Self {
        Self { accessibility: AccessibilitySettings::default() }
    }
}
