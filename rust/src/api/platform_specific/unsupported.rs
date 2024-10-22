use crate::api::common::SystemIcon;

pub fn get_icon_for_file_impl(_path: String) -> SystemIcon {
    SystemIcon {
        data: "".to_owned(),
        height: 0,
        width: 0,
    }
}

pub fn get_icon_for_bundle_impl(_bundle_identifier: String) -> SystemIcon {
    SystemIcon {
        data: "".to_owned(),
        height: 0,
        width: 0,
    }
}
