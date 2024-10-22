/// Represents a system icon encoded in Base64 format.
#[derive(Debug, Clone, PartialEq)]
pub struct SystemIcon {
    pub data: String,
    pub height: u32,
    pub width: u32,
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

#[flutter_rust_bridge::frb(sync)]
pub fn smoke_test() -> bool {
    true
}

/// Service for retrieving system icons.
pub struct SystemIconService {}

impl SystemIconService {
    /// Retrieves a system icon for the given file path.
    #[flutter_rust_bridge::frb(sync)]
    pub fn get_icon_for_file(path: String) -> SystemIcon {
        crate::api::platform_specific::get_icon_for_file(path)
    }

    /// Retrieves a system icon for the given bundle identifier (macOS only).
    #[flutter_rust_bridge::frb(sync)]
    pub fn get_icon_for_bundle(bundle_identifier: String) -> SystemIcon {
        crate::api::platform_specific::get_icon_for_bundle(bundle_identifier)
    }
}
