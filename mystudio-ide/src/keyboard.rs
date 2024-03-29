// Global keyboard events listener

use std::cell::RefCell;

use crate::ui::{
    action_row::handler::{on_open_dir_clicked, on_save_changes_clicked},
    features,
    statusbar::goto_line::show_goto_dialog,
};

use gtk::prelude::{AccelGroupExtManual, GtkWindowExt};
use gtk::{gdk, AccelFlags, ApplicationWindow};

use libmystudio::workspace::Workspace;

thread_local! {static KEY_EVENT_TRACKER : RefCell<Vec<gdk::EventKey>> = RefCell::new(Vec::new())}

pub fn listen_for_events(window: &ApplicationWindow) {
    // "Open Workspace" Keyboard shortcut
    let (accel_key, accel_mods) = gtk::accelerator_parse("<Ctrl><Shift>O");
    let accel_group = gtk::AccelGroup::new();

    accel_group.connect_accel_group(
        accel_key,
        accel_mods,
        AccelFlags::VISIBLE,
        move |_, _, _, _| {
            on_open_dir_clicked();
            true
        },
    );

    window.add_accel_group(&accel_group);

    // "Save Changes" Keyboard shortcut
    let (accel_key, accel_mods) = gtk::accelerator_parse("<Ctrl>S");
    let accel_group = gtk::AccelGroup::new();

    accel_group.connect_accel_group(
        accel_key,
        accel_mods,
        AccelFlags::VISIBLE,
        move |_, _, _, _| {
            on_save_changes_clicked();
            true
        },
    );

    window.add_accel_group(&accel_group);

    // Goto Line

    let (accel_key, accel_mods) = gtk::accelerator_parse("<Ctrl>G");
    let accel_group = gtk::AccelGroup::new();

    accel_group.connect_accel_group(
        accel_key,
        accel_mods,
        AccelFlags::VISIBLE,
        move |_, _, _, _| {
            if Workspace::get_open_file_path().is_some() {
                show_goto_dialog();
            }
            true
        },
    );

    window.add_accel_group(&accel_group);

    // Find in Files

    let (accel_key, accel_mods) = gtk::accelerator_parse("<Ctrl><Shift>F");
    let accel_group = gtk::AccelGroup::new();

    accel_group.connect_accel_group(
        accel_key,
        accel_mods,
        AccelFlags::VISIBLE,
        move |_, _, _, _| {
            if !Workspace::get_path().is_empty() {
                features::find_in_files::show_dialog();
            }
            true
        },
    );

    window.add_accel_group(&accel_group);
}
