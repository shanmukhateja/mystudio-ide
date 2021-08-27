// Global keyboard events listener

use std::cell::RefCell;

use crate::comms::CommEvents;
use crate::{
    ui::btn_action_row::{on_open_dir_clicked, on_save_changes_clicked},
    G_WINDOW,
};

use gtk::gdk;
use gtk::glib::Sender;
use gtk::prelude::{AccelGroupExtManual, GtkWindowExt};

thread_local! {static KEY_EVENT_TRACKER : RefCell<Vec<gdk::EventKey>> = RefCell::new(Vec::new())}

pub fn listen_for_events(tx: Sender<CommEvents>) {
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

    G_WINDOW.with(|w| {
        w.borrow().clone().unwrap().add_accel_group(&accel_group);
    });

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

    G_WINDOW.with(|w| {
        w.borrow().clone().unwrap().add_accel_group(&accel_group);
    });
}
