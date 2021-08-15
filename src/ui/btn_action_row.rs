use gtk::prelude::*;

use crate::comms::CommEvents;
use crate::workspace::Workspace;
use gtk::glib;

pub fn build_actions_button(tx: glib::Sender<CommEvents>) -> gtk::Grid {

    // FIXME: find better way than cloning `tx` for each closure
    let tx_arc = tx.clone();
    let tx_arc2 = tx_arc.clone();

    let grid_view = gtk::GridBuilder::new().hexpand(true).vexpand(false).build();

    // Open Dir button
    let open_dir_button = gtk::ButtonBuilder::new()
        .label("Open Folder")
        .focus_on_click(true)
        .build();

    open_dir_button.connect_button_release_event(move |_btn, _y| {
        on_open_dir_clicked(&tx_arc);
        gtk::Inhibit(true)
    });

    grid_view.add(&open_dir_button);

    // Save changes button
    let open_dir_button = gtk::ButtonBuilder::new()
        .label("Save Changes")
        .focus_on_click(true)
        .build();

    open_dir_button.connect_button_release_event(move |_btn, _y| {
        on_save_changes_clicked(&tx_arc2);
        gtk::Inhibit(true)
    });

    grid_view.add(&open_dir_button);

    grid_view
}

fn on_open_dir_clicked(tx: &glib::Sender<CommEvents>) {
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

    match chooser.run() {
        gtk::ResponseType::Ok => {
            let chosen_dir = chooser.file().unwrap();
            let dir_path_buf = chosen_dir.path().unwrap();
            let dir_path = dir_path_buf.to_str().unwrap();

            // update global workspace path
            Workspace::update_path(dir_path.to_string());

            // update UI
            tx.send(CommEvents::UpdateRootTree()).ok();
        }
        _ => (),
    };

    chooser.hide();
}

fn on_save_changes_clicked(tx: &glib::Sender<CommEvents>) {
    tx.send(CommEvents::SaveEditorChanges()).ok();
}
