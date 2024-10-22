#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
mod unsupported;

use crate::api::common::SystemIcon;

#[cfg(target_os = "macos")]
pub use self::macos::*;
#[cfg(target_os = "windows")]
pub use self::windows::*;
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub use self::unsupported::*;

pub fn get_icon_for_file(path: String) -> SystemIcon {
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    {
        get_icon_for_file_impl(path)
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        SystemIcon {
            data: "".to_owned(),
            height: 0,
            width: 0,
        }
    }
}

pub fn get_icon_for_bundle(bundle_identifier: String) -> SystemIcon {
    #[cfg(target_os = "macos")]
    {
        get_icon_for_bundle_impl(bundle_identifier)
    }
    #[cfg(not(target_os = "macos"))]
    {
        SystemIcon {
            data: "".to_owned(),
            height: 0,
            width: 0,
        }
    }
}
