use gtk::prelude::*;

use crate::workspace;

pub fn build_actions_button() -> gtk::Grid {
    let grid_view = gtk::GridBuilder::new().hexpand(true).vexpand(false).build();

    // Open Dir button
    let open_dir_button = gtk::ButtonBuilder::new()
        .label("Open Folder")
        .focus_on_click(true)
        .build();

    open_dir_button.connect_button_release_event(|_btn, _y| {
        on_open_dir_clicked();
        gtk::Inhibit(true)
    });

    grid_view.add(&open_dir_button);

    grid_view
}

fn on_open_dir_clicked() {
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
            workspace::Workspace::update_path(dir_path.to_string());
        }
        _ => (),
    };

    chooser.hide();
}
