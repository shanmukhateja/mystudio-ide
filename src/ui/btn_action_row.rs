use gtk::glib;
use gtk::prelude::{
    BuilderExtManual, DialogExt, FileChooserExt, FileExt, WidgetExt,
};
use gtk::Button;

use crate::comms::CommEvents;
use crate::workspace::Workspace;

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

pub fn on_open_dir_clicked(tx: &glib::Sender<CommEvents>) {
    let dir_filter = gtk::FileFilter::new();
    dir_filter.add_mime_type("inode/directory");

    let chooser = gtk::FileChooserDialogBuilder::new()
        .action(gtk::FileChooserAction::SelectFolder)
        .title("Open Folder")
        .default_width(600)
        .default_height(400)
        .focus_on_click(true)
        .filter(&dir_filter)
        .show_hidden(false)
        .build();

    chooser.add_button("Select Folder", gtk::ResponseType::Ok);
    chooser.add_button("Cancel", gtk::ResponseType::Cancel);

    if let gtk::ResponseType::Ok = chooser.run() {
        let chosen_dir = chooser.file().unwrap();
        let dir_path_buf = chosen_dir.path().unwrap();
        let dir_path = dir_path_buf.to_str().unwrap();

        // update global workspace path
        Workspace::update_path(dir_path.to_string());

        // update UI
        tx.send(CommEvents::UpdateRootTree()).ok();
    };

    chooser.hide();
}

pub fn on_save_changes_clicked(tx: &glib::Sender<CommEvents>) {
    tx.send(CommEvents::SaveEditorChanges()).ok();
}
