use gtk::{
    prelude::{BuilderExtManual, LabelExt},
    traits::WidgetExt,
};
use libmystudio::{encoding::detect_encoding_str, workspace::Workspace};

use crate::G_BUILDER;

use super::G_FILE_ENCODING;

pub fn init() {
    let builder = G_BUILDER.with(|builder| builder.borrow().clone().unwrap());

    G_FILE_ENCODING.with(|status_bar| {
        *status_bar.borrow_mut() = builder.object("label_file_encoding");
        assert!(status_bar.borrow().is_some());
    });
}

pub(super) fn sync() {
    let indicator = G_FILE_ENCODING.with(|e| e.borrow().clone().unwrap());

    let open_file_path = Workspace::get_open_file_path();
    if open_file_path.is_none() {
        return;
    }

    if !indicator.is_visible() {
        indicator.set_visible(true);
    }

    let file_path_str = open_file_path.unwrap();
    let encoding: &str = detect_encoding_str(file_path_str.as_str());

    indicator.set_label(encoding);
}

pub(super) fn reset_and_hide_indicator() {
    G_FILE_ENCODING.with(|i| i.borrow().clone().unwrap().set_visible(false));
}
