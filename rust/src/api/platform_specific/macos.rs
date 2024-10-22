use base64::Engine;
use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};

use crate::api::common::SystemIcon;

pub fn get_icon_for_file_impl(path: String) -> SystemIcon {
    unsafe { find_icon_by_path(path) }
}

pub fn get_icon_for_bundle_impl(bundle_identifier: String) -> SystemIcon {
    unsafe { find_icon_by_bundle_identifier(bundle_identifier) }
}

unsafe fn find_icon_by_bundle_identifier(bundle_identifier: String) -> SystemIcon {
    let mut icon_info = SystemIcon {
        data: "".to_owned(),
        height: 0,
        width: 0,
    };

    if bundle_identifier.ne("") {
        let workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
        let ns_string = NSString::alloc(nil).init_str(&bundle_identifier);
        let url: id = msg_send![workspace, URLForApplicationWithBundleIdentifier:ns_string];

        if !url.is_null() {
            let path: id = msg_send![url, path];
            let ns_image: id = msg_send![workspace, iconForFile:path];
    
            if !ns_image.is_null() {
                icon_info = ns_image_to_icon_info(ns_image);
            }
        }
        // Release allocated object
        release(ns_string);
    }
    icon_info
}

unsafe fn find_icon_by_path(path: String) -> SystemIcon {
    let mut icon_info = SystemIcon {
        data: "".to_owned(),
        height: 0,
        width: 0,
    };

    if path.ne("") {
        let ns_path: *mut Object = NSString::alloc(nil).init_str(&path);
        let ns_workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
        let ns_image: id = msg_send![ns_workspace, iconForFile: ns_path];
    
        if !ns_image.is_null() {
            icon_info = ns_image_to_icon_info(ns_image);
        }
        // Release allocated object
        release(ns_path);
    }
    icon_info
}

unsafe fn ns_image_to_icon_info(ns_image: id) -> SystemIcon {
    let ns_cg_ref: id = msg_send![
        ns_image,
        CGImageForProposedRect: nil
        context: nil
        hints: nil
    ];
    let ns_bitmapref: id = msg_send![class!(NSBitmapImageRep), alloc];
    let ns_imagerep: id = msg_send![ns_bitmapref, initWithCGImage: ns_cg_ref];
    let ns_imagesize: (f64, f64) = msg_send![ns_image, size];
    let _: () = msg_send![ns_imagerep, setSize: ns_imagesize];
    let ns_png_data: id = msg_send![ns_imagerep, representationUsingType:4 properties:nil];
    let ns_lenght: usize = msg_send![ns_png_data, length];
    let ns_bytes: *const u8 = msg_send![ns_png_data, bytes];
    let byte_slice: &[u8] = std::slice::from_raw_parts(ns_bytes, ns_lenght);
    let data = base64::prelude::BASE64_STANDARD.encode(byte_slice);
    let icon_info = SystemIcon {
        data,
        width: ns_imagesize.0 as u32,
        height: ns_imagesize.1 as u32,
    };
    // Release allocated object
    release(ns_bitmapref);
    icon_info
}

pub unsafe fn release(object: *mut Object) {
    let _: () = msg_send![object, release];
}
