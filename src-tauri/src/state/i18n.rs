use crate::state::config::LangConfig;
use tauri_plugin_os::locale;

#[derive(Debug, Clone)]
pub struct I18n {
    pub language: &'static str,
    pub language_system: &'static str,
    pub theme: &'static str,
    pub theme_system: &'static str,
    pub theme_light: &'static str,
    pub theme_dark: &'static str,
    pub exit: &'static str,
}

static LANG_EN: I18n = I18n {
    language: "Language",
    language_system: "System",
    theme: "Theme",
    theme_system: "System",
    theme_light: "Light",
    theme_dark: "Dark",
    exit: "Exit",
};

static LANG_ZH: I18n = I18n {
    language: "语言",
    language_system: "系统",
    theme: "主题",
    theme_system: "系统",
    theme_light: "浅色",
    theme_dark: "深色",
    exit: "退出",
};

/// IETF BCP-47 language tag
impl I18n {
    pub fn get(lang: &LangConfig) -> &'static I18n {
        match lang {
            LangConfig::Zh => &LANG_ZH,
            LangConfig::En => &LANG_EN,
            LangConfig::System => match locale() {
                Some(locale) if locale.starts_with("zh") => &LANG_ZH,
                Some(locale) if locale == "en-US" => &LANG_EN,
                _ => &LANG_EN,
            },
        }
    }
}
