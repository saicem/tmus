use core::slice;
use log::info;
use serde::{Deserialize, Serialize};
use std::{ffi::c_void, ptr};
use windows::core::HSTRING;
use windows::Win32::Storage::FileSystem::GetFileVersionInfoSizeW;
use windows::Win32::Storage::FileSystem::GetFileVersionInfoW;
use windows::Win32::Storage::FileSystem::VerQueryValueW;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileVersion {
    pub comments: Option<String>,
    pub internal_name: Option<String>,
    pub product_name: Option<String>,
    pub company_name: Option<String>,
    pub legal_copyright: Option<String>,
    pub product_version: Option<String>,
    pub file_description: Option<String>,
    pub legal_trademarks: Option<String>,
    pub private_build: Option<String>,
    pub file_version: Option<String>,
    pub original_filename: Option<String>,
    pub special_build: Option<String>,
}

pub fn get_file_version(path: &str) -> Option<FileVersion> {
    unsafe {
        let size = GetFileVersionInfoSizeW(&HSTRING::from(path), None) as usize;
        if size == 0 {
            info!("File: {}, don't have file version.", path);
            return None;
        }
        let mut buffer: Vec<u16> = vec![0; size];
        GetFileVersionInfoW(
            &HSTRING::from(path),
            0,
            buffer.len() as u32,
            buffer.as_mut_ptr() as _,
        )
        .expect("get file version info failed.");
        let pblock = buffer.as_ptr() as _;
        let lang_id = query_lang_id(pblock);

        let comments = file_version_item(pblock, lang_id, "Comments");
        let internal_name = file_version_item(pblock, lang_id, "InternalName");
        let product_name = file_version_item(pblock, lang_id, "ProductName");
        let company_name = file_version_item(pblock, lang_id, "CompanyName");
        let legal_copyright = file_version_item(pblock, lang_id, "LegalCopyright");
        let product_version = file_version_item(pblock, lang_id, "ProductVersion");
        let file_description = file_version_item(pblock, lang_id, "FileDescription");
        let legal_trademarks = file_version_item(pblock, lang_id, "LegalTrademarks");
        let private_build = file_version_item(pblock, lang_id, "PrivateBuild");
        let file_version = file_version_item(pblock, lang_id, "FileVersion");
        let original_filename = file_version_item(pblock, lang_id, "OriginalFilename");
        let special_build = file_version_item(pblock, lang_id, "SpecialBuild");

        Some(FileVersion {
            comments,
            internal_name,
            product_name,
            company_name,
            legal_copyright,
            product_version,
            file_description,
            legal_trademarks,
            private_build,
            file_version,
            original_filename,
            special_build,
        })
    }
}

unsafe fn file_version_item(
    pblock: *const c_void,
    lang_id: i32,
    version_detail: &str,
) -> Option<String> {
    let lpsubblock = format!(
        "\\\\StringFileInfo\\\\{:08x}\\\\{}",
        lang_id, version_detail
    );
    let mut buffer: *mut c_void = ptr::null_mut();
    let mut len = 0;
    let lpsubblock = &HSTRING::from(lpsubblock);
    let ok = VerQueryValueW(pblock, lpsubblock, &mut buffer, &mut len);
    if ok == false || len == 0 {
        return None;
    }

    Some(utf16_to_string(
        &slice::from_raw_parts(buffer.cast(), len as usize).to_vec(),
    ))
}

fn utf16_to_string(raw: &[u16]) -> String {
    match raw.iter().position(|&c| c == 0) {
        Some(null_pos) => String::from_utf16_lossy(&raw[..null_pos]),
        None => String::from_utf16_lossy(raw),
    }
}

unsafe fn query_lang_id(data: *const c_void) -> i32 {
    let mut buffer: *mut c_void = ptr::null_mut();
    let mut len = 0;

    if VerQueryValueW(
        data,
        &HSTRING::from("\\VarFileInfo\\Translation"),
        &mut buffer as _,
        &mut len,
    )
    .as_bool()
    {
        let ret = *(buffer as *mut i32);
        return ((ret & 0xffff) << 16) + (ret >> 16);
    }
    0x040904E4
}
