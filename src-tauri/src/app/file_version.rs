use core::slice;
use serde::{Deserialize, Serialize};
use std::{ffi::c_void, ptr};
use windows::{
    core::HSTRING,
    Win32::Storage::FileSystem::{GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileVersion {
    comments: Option<String>,
    internal_name: Option<String>,
    product_name: Option<String>,
    company_name: Option<String>,
    legal_copyright: Option<String>,
    product_version: Option<String>,
    file_description: Option<String>,
    legal_trademarks: Option<String>,
    private_build: Option<String>,
    file_version: Option<String>,
    original_filename: Option<String>,
    special_build: Option<String>,
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

unsafe fn file_version_detail(
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
    VerQueryValueW(pblock, lpsubblock, &mut buffer, &mut len);
    if len == 0 {
        return None;
    }

    Some(String::from_utf16_lossy(
        &slice::from_raw_parts(buffer.cast(), (len - 1) as usize).to_vec(),
    ))
}

#[tauri::command]
pub fn file_version(path: &str) -> FileVersion {
    unsafe {
        let filename = &HSTRING::from(path);
        let size = GetFileVersionInfoSizeW(filename, None);
        let mut data: Vec<u16> = vec![0; size as usize];
        GetFileVersionInfoW(filename, 0, size, data.as_mut_ptr() as _)
            .expect("get file version info failed.");
        let pblock = data.as_ptr() as _;
        let lang_id = query_lang_id(pblock);

        let comments = file_version_detail(pblock, lang_id, "Comments");
        let internal_name = file_version_detail(pblock, lang_id, "InternalName");
        let product_name = file_version_detail(pblock, lang_id, "ProductName");
        let company_name = file_version_detail(pblock, lang_id, "CompanyName");
        let legal_copyright = file_version_detail(pblock, lang_id, "LegalCopyright");
        let product_version = file_version_detail(pblock, lang_id, "ProductVersion");
        let file_description = file_version_detail(pblock, lang_id, "FileDescription");
        let legal_trademarks = file_version_detail(pblock, lang_id, "LegalTrademarks");
        let private_build = file_version_detail(pblock, lang_id, "PrivateBuild");
        let file_version = file_version_detail(pblock, lang_id, "FileVersion");
        let original_filename = file_version_detail(pblock, lang_id, "OriginalFilename");
        let special_build = file_version_detail(pblock, lang_id, "SpecialBuild");

        FileVersion {
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
        }
    }
}
