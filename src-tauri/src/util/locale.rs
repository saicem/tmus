use crate::util::str_util::{dump_wstring_vec, load_wstring_vec};
use serde::{Deserialize, Serialize};
use windows::core::HSTRING;
use windows::Win32::Globalization::{
    GetLocaleInfoEx, GetUserDefaultLocaleName, LOCALE_IFIRSTDAYOFWEEK, LOCALE_SLONGDATE,
    LOCALE_SSHORTDATE, LOCALE_SSHORTTIME, LOCALE_STIMEFORMAT, MAX_LOCALE_NAME,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocaleInfo {
    pub name: String,
    pub first_day_of_week: i64,
    pub short_date_format: String,
    pub long_date_format: String,
    pub long_time_format: String,
    pub short_time_format: String,
}

pub fn get_user_default_locale_info() -> LocaleInfo {
    let mut locale = LocaleInfo {
        name: "en-US".to_string(),
        first_day_of_week: 0,
        short_date_format: "yyyy/M/d".to_string(),
        long_date_format: "yyyy/M/d".to_string(),
        long_time_format: "H:mm:ss".to_string(),
        short_time_format: "H:mm".to_string(),
    };
    locale.name = get_user_default_locale_name().unwrap_or("en-US".to_string());
    if let Some(first_day_of_week) = get_locale_info(&locale.name, LOCALE_IFIRSTDAYOFWEEK) {
        locale.first_day_of_week = first_day_of_week.parse().unwrap();
    }
    if let Some(date_format) = get_locale_info(&locale.name, LOCALE_SSHORTDATE) {
        locale.short_date_format = date_format;
    }
    if let Some(date_format) = get_locale_info(&locale.name, LOCALE_SLONGDATE) {
        locale.long_date_format = date_format;
    }
    if let Some(time_format) = get_locale_info(&locale.name, LOCALE_STIMEFORMAT) {
        locale.short_time_format = time_format;
    }
    if let Some(time_format) = get_locale_info(&locale.name, LOCALE_SSHORTTIME) {
        locale.long_time_format = time_format;
    }
    locale
}

fn get_user_default_locale_name() -> Option<String> {
    let mut locale_name_buffer = vec![0u16; MAX_LOCALE_NAME as usize];
    let len = unsafe { GetUserDefaultLocaleName(&mut locale_name_buffer) };
    if len == 0 {
        return None;
    }
    Some(load_wstring_vec(&locale_name_buffer))
}

fn get_locale_info(locale_name: &str, lc_type: u32) -> Option<String> {
    let locale_name_w: Vec<u16> = dump_wstring_vec(locale_name);

    let mut buffer = vec![0u16; 256];
    let len = unsafe {
        GetLocaleInfoEx(
            &HSTRING::from(load_wstring_vec(&locale_name_w)),
            lc_type,
            Some(&mut buffer),
        )
    };

    if len == 0 {
        return None;
    }
    Some(load_wstring_vec(&buffer))
}

#[cfg(test)]
mod tests {
    use crate::util::locale::{get_locale_info, get_user_default_locale_name};
    use windows::Win32::Globalization::{
        LOCALE_IFIRSTDAYOFWEEK, LOCALE_SLONGDATE, LOCALE_SSHORTDATE, LOCALE_SSHORTTIME,
        LOCALE_STIMEFORMAT,
    };

    #[test]
    fn test_get_locale() {
        println!("--- Windows Locale Information ---");

        if let Some(locale_name) = get_user_default_locale_name() {
            println!("User Default Locale Name: {}", locale_name);

            // Monday is 0, Sunday is 6
            if let Some(first_day_of_week) = get_locale_info(&locale_name, LOCALE_IFIRSTDAYOFWEEK) {
                println!("  First Day of Week: {}", first_day_of_week);
            }

            if let Some(date_format) = get_locale_info(&locale_name, LOCALE_SSHORTDATE) {
                println!("  Short Date Format: {}", date_format);
            }

            if let Some(date_format) = get_locale_info(&locale_name, LOCALE_SLONGDATE) {
                println!("  Long Date Format: {}", date_format);
            }

            if let Some(time_format) = get_locale_info(&locale_name, LOCALE_STIMEFORMAT) {
                println!("  Time Format: {}", time_format);
            }

            if let Some(time_format) = get_locale_info(&locale_name, LOCALE_SSHORTTIME) {
                println!("  Short Time Format: {}", time_format);
            }
        } else {
            eprintln!("Failed to get user default locale name.");
        }
    }
}
