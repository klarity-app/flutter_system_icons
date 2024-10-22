use flutter_rust_bridge::frb;
use base64::Engine;
use windows::Win32::UI::WindowsAndMessaging::{SHGFI_ICON, SHGFI_SMALLICON, SHFILEINFOW, SHGetFileInfoW};
use windows::Win32::UI::WindowsAndMessaging::DestroyIcon;
use windows::Win32::Graphics::Gdi::{CreateBitmap, GetDC, CreateCompatibleDC, SelectObject, BitBlt, DeleteDC, DeleteObject};
use windows::Win32::Graphics::Gdi::SRCCOPY;
use std::mem::size_of;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use crate::api::common::SystemIcon;

pub fn get_icon_for_file_impl(path: String) -> SystemIcon {
    unsafe { find_icon_by_path(path) }
}

unsafe fn find_icon_by_path(path: String) -> SystemIcon {
    let mut icon_info = SystemIcon {
        data: "".to_owned(),
        height: 0,
        width: 0,
    };

    let wide_path: Vec<u16> = OsStr::new(&path).encode_wide().chain(Some(0)).collect();
    let mut shfi = SHFILEINFOW::default();

    if SHGetFileInfoW(
        wide_path.as_ptr(),
        0,
        &mut shfi,
        size_of::<SHFILEINFOW>() as u32,
        SHGFI_ICON | SHGFI_SMALLICON,
    ) != 0 {
        if !shfi.hIcon.is_invalid() {
            let hdc = GetDC(None);
            let hdc_mem = CreateCompatibleDC(hdc);
            
            let hbm_color = CreateBitmap(16, 16, 1, 32, None);
            let hbm_old = SelectObject(hdc_mem, hbm_color);
            
            BitBlt(hdc_mem, 0, 0, 16, 16, hdc, 0, 0, SRCCOPY);
            
            let mut buffer = vec![0u8; 16 * 16 * 4];
            let bitmap_info = windows::Win32::Graphics::Gdi::BITMAPINFO {
                bmiHeader: windows::Win32::Graphics::Gdi::BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<windows::Win32::Graphics::Gdi::BITMAPINFOHEADER>() as u32,
                    biWidth: 16,
                    biHeight: -16,
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: windows::Win32::Graphics::Gdi::BI_RGB,
                    biSizeImage: 0,
                    biXPelsPerMeter: 0,
                    biYPelsPerMeter: 0,
                    biClrUsed: 0,
                    biClrImportant: 0,
                },
                bmiColors: [windows::Win32::Graphics::Gdi::RGBQUAD::default()],
            };
            
            windows::Win32::Graphics::Gdi::GetDIBits(
                hdc_mem,
                hbm_color,
                0,
                16,
                Some(buffer.as_mut_ptr() as *mut std::ffi::c_void),
                &bitmap_info,
                windows::Win32::Graphics::Gdi::DIB_RGB_COLORS,
            );
            
            icon_info.data = base64::prelude::BASE64_STANDARD.encode(&buffer);
            icon_info.width = 16;
            icon_info.height = 16;
            
            SelectObject(hdc_mem, hbm_old);
            DeleteObject(hbm_color);
            DeleteDC(hdc_mem);
            DestroyIcon(shfi.hIcon);
        }
    }

    icon_info
}
