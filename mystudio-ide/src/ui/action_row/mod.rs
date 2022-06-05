use gtk::{glib, prelude::BuilderExtManual, traits::WidgetExt, Button};

use crate::comms::CommEvents;

use self::handler::{on_open_dir_clicked, on_save_changes_clicked};

pub mod handler;

pub fn setup_actions(builder: &gtk::Builder, tx: glib::Sender<CommEvents>) {
    // FIXME: find better way than cloning `tx` for each closure
    let tx_arc = tx;
    let tx_arc2 = tx_arc.clone();

    let open_dir_btn: Button = builder
        .object("button_open_workspace")
        .expect("Unable to find button_open_workspace");

    open_dir_btn.connect_button_release_event(move |_btn, _y| {
        on_open_dir_clicked(&tx_arc);
        gtk::Inhibit(true)
    });

    let save_changes_btn: Button = builder
        .object("button_save_changes")
        .expect("button_save_changes");

    save_changes_btn.connect_button_release_event(move |_btn, _y| {
        on_save_changes_clicked(&tx_arc2);
        // Note: Fixes an issue where button has focus on hover after first use
        gtk::Inhibit(false)
    });

    // FIXME: remove these and work it out in Glade
    open_dir_btn.set_sensitive(true);
    save_changes_btn.set_sensitive(true);
}
