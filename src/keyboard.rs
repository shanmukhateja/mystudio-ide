// Global keyboard events listener

use std::cell::RefCell;

use crate::{
    comms::CommEvents,
    ui::{
        action_row::ui::{on_open_dir_clicked, on_save_changes_clicked},
        statusbar::line_indicator::show_goto_dialog
    }, workspace::Workspace,
};

use gtk::glib::Sender;
use gtk::prelude::{AccelGroupExtManual, GtkWindowExt};
use gtk::{gdk, ApplicationWindow};

thread_local! {static KEY_EVENT_TRACKER : RefCell<Vec<gdk::EventKey>> = RefCell::new(Vec::new())}

pub fn listen_for_events(tx: Sender<CommEvents>, window: &ApplicationWindow) {
    let tx_clone = tx.clone();

    // "Open Workspace" Keyboard shortcut
    let (accel_key, accel_mods) = gtk::accelerator_parse("<Ctrl><Shift>O");
    let accel_group = gtk::AccelGroup::new();

    accel_group.connect_accel_group(
        accel_key,
        accel_mods,
        gtk::AccelFlags::VISIBLE,
        move |_, _, _, _| {
            on_open_dir_clicked(&tx.clone());
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
        gtk::AccelFlags::VISIBLE,
        move |_, _, _, _| {
            on_save_changes_clicked(&tx_clone);
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
        gtk::AccelFlags::VISIBLE,
        move |_, _, _, _| {
            if Workspace::get_open_file_path().is_some() {
                show_goto_dialog();
            }
            true
        },
    );

    window.add_accel_group(&accel_group);
}
