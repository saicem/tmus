use crate::config::config::{Config, LangConfig};

#[derive(Debug, Clone)]
pub struct I18n {
    pub language: &'static str,
    pub theme: &'static str,
    pub theme_auto: &'static str,
    pub theme_light: &'static str,
    pub theme_dark: &'static str,
    pub exit: &'static str,
}

static LANG_EN: I18n = I18n {
    language: "Language",
    theme: "Theme",
    theme_auto: "Auto",
    theme_light: "Light",
    theme_dark: "Dark",
    exit: "Exit",
};

static LANG_ZH: I18n = I18n {
    language: "语言",
    theme: "主题",
    theme_auto: "自动",
    theme_light: "浅色",
    theme_dark: "深色",
    exit: "退出",
};

impl I18n {
    pub fn get() -> &'static I18n {
        match Config::get_mut().lang {
            LangConfig::Zh => &LANG_ZH,
            LangConfig::En => &LANG_EN,
        }
    }
}
