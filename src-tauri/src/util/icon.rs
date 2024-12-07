use image::ImageBuffer;
use image::RgbaImage;
use itertools::Itertools;
#[cfg(target_arch = "x86")]
use std::arch::x86::_mm_shuffle_epi8;
use std::arch::x86_64::__m128i;
use std::arch::x86_64::_mm_loadu_si128;
use std::arch::x86_64::_mm_setr_epi8;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::_mm_shuffle_epi8;
use std::arch::x86_64::_mm_storeu_si128;
use windows::core::{Error, HSTRING};
use windows::Win32::Graphics::Gdi::CreateCompatibleDC;
use windows::Win32::Graphics::Gdi::DeleteDC;
use windows::Win32::Graphics::Gdi::DeleteObject;
use windows::Win32::Graphics::Gdi::GetDIBits;
use windows::Win32::Graphics::Gdi::SelectObject;
use windows::Win32::Graphics::Gdi::BITMAPINFO;
use windows::Win32::Graphics::Gdi::BITMAPINFOHEADER;
use windows::Win32::Graphics::Gdi::DIB_RGB_COLORS;
use windows::Win32::UI::Shell::ExtractIconExW;
use windows::Win32::UI::WindowsAndMessaging::GetIconInfoExW;
use windows::Win32::UI::WindowsAndMessaging::HICON;
use windows::Win32::UI::WindowsAndMessaging::ICONINFOEXW;
use windows::Win32::UI::WindowsAndMessaging::{DestroyIcon, PrivateExtractIconsW};

pub fn extract_icon(exe_path: &str) -> Option<RgbaImage> {
    let hicon = unsafe { extract_hicon(exe_path) };
    if hicon.is_none() {
        extract_icon_classic(exe_path)
    } else {
        Some(hicon_to_image(&hicon.unwrap()).unwrap())
    }
}

unsafe fn extract_hicon(exe_path: &str) -> Option<HICON> {
    let mut path_vec: Vec<u16> = exe_path.encode_utf16().collect();
    path_vec.resize_with(260, || 0);
    let path_arr: [u16; 260] = path_vec.try_into().unwrap();
    let mut num_icons_total = PrivateExtractIconsW(&path_arr, -1, 256, 256, None, None, 0);
    if num_icons_total == 0 {
        return None;
    }
    let mut icons = vec![HICON::default(); num_icons_total as usize];
    let _ = PrivateExtractIconsW(
        &path_arr,
        0,
        256,
        256,
        Some(&mut icons),
        Some(&mut num_icons_total),
        0,
    );
    icons.into_iter().next()
}

fn extract_icon_classic(exe_path: &str) -> Option<RgbaImage> {
    unsafe {
        let path = &HSTRING::from(exe_path);
        let num_icons_total = ExtractIconExW(path, -1, None, None, 0);
        if num_icons_total == 0 {
            return None;
        }
        let mut large_icons = vec![HICON::default(); num_icons_total as usize];
        let mut small_icons = vec![HICON::default(); num_icons_total as usize];
        let num_icons_fetched = ExtractIconExW(
            path,
            0,
            Some(large_icons.as_mut_ptr()),
            Some(small_icons.as_mut_ptr()),
            num_icons_total,
        );

        if num_icons_fetched == 0 {
            return None;
        }

        let images = large_icons
            .iter()
            .chain(small_icons.iter())
            .map(|icon| hicon_to_image(icon))
            .filter_map(|r| match r {
                Ok(img) => Some(img),
                Err(e) => {
                    eprintln!("Failed to convert HICON to RgbaImage: {:?}", e);
                    None
                }
            })
            .collect_vec();

        images.into_iter().next()
    }
}

fn hicon_to_image(hicon: &HICON) -> Result<RgbaImage, String> {
    unsafe {
        let mut icon_info = ICONINFOEXW::default();
        icon_info.cbSize = size_of::<ICONINFOEXW>() as u32;

        if !GetIconInfoExW(*hicon, &mut icon_info).as_bool() {
            return Err(format!(
                "Win32 Error {} GetIconInfoExW: {} {}:{}",
                Error::from_win32().code(),
                file!(),
                line!(),
                column!()
            ));
        }
        if !hicon.is_invalid() {
            DestroyIcon(*hicon).unwrap();
        }

        let hdc_screen = CreateCompatibleDC(None);
        let hdc_mem = CreateCompatibleDC(hdc_screen);
        let hbm_old = SelectObject(hdc_mem, icon_info.hbmColor);

        let mut bmp_info = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: icon_info.xHotspot as i32 * 2,
                biHeight: -(icon_info.yHotspot as i32 * 2),
                biPlanes: 1,
                biBitCount: 32,
                biCompression: DIB_RGB_COLORS.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut buffer: Vec<u8> =
            vec![0; (icon_info.xHotspot * 2 * icon_info.yHotspot * 2 * 4) as usize];

        if GetDIBits(
            hdc_mem,
            icon_info.hbmColor,
            0,
            icon_info.yHotspot * 2,
            Some(buffer.as_mut_ptr() as *mut _),
            &mut bmp_info,
            DIB_RGB_COLORS,
        ) == 0
        {
            return Err(format!(
                "Win32 Error {} GetDIBits: {} {}:{}",
                Error::from_win32().code(),
                file!(),
                line!(),
                column!()
            ));
        }
        // Clean up
        SelectObject(hdc_mem, hbm_old);
        let _ = DeleteDC(hdc_mem);
        let _ = DeleteDC(hdc_screen);
        let _ = DeleteObject(icon_info.hbmColor);
        let _ = DeleteObject(icon_info.hbmMask);

        bgra_to_rgba(buffer.as_mut_slice());

        let image =
            ImageBuffer::from_raw(icon_info.xHotspot * 2, icon_info.yHotspot * 2, buffer).unwrap();
        Ok(image)
    }
}

/// Convert BGRA to RGBA
///
/// Uses SIMD to go fast
fn bgra_to_rgba(data: &mut [u8]) {
    // The shuffle mask for converting BGRA -> RGBA
    let mask: __m128i = unsafe {
        _mm_setr_epi8(
            2, 1, 0, 3, // First pixel
            6, 5, 4, 7, // Second pixel
            10, 9, 8, 11, // Third pixel
            14, 13, 12, 15, // Fourth pixel
        )
    };
    // For each 16-byte chunk in your data
    for chunk in data.chunks_exact_mut(16) {
        let mut vector = unsafe { _mm_loadu_si128(chunk.as_ptr() as *const __m128i) };
        vector = unsafe { _mm_shuffle_epi8(vector, mask) };
        unsafe { _mm_storeu_si128(chunk.as_mut_ptr() as *mut __m128i, vector) };
    }
}

#[cfg(test)]
mod tests {
    use crate::util::icon::extract_icon;

    #[test]
    fn test_convert_hicon_to_rgba_image() {
        let exe_path = r"D:\app\mpv\mpv.exe";
        let image = extract_icon(exe_path).unwrap();
        image.save("target/icon.png").unwrap();
    }
}
