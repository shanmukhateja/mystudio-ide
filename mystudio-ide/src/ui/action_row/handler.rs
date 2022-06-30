use gtk::{
    glib,
    prelude::{FileExt, TextBufferExt},
    traits::{DialogExt, FileChooserExt, WidgetExt},
    TextBuffer,
};
use libmystudio::{fs, workspace::Workspace};

use crate::comms::CommEvents;

pub fn save_file_changes(text_buffer: TextBuffer, file_absolute_path: String) -> Result<(), String> {
    let content_gstring = text_buffer
        .text(&text_buffer.start_iter(), &text_buffer.end_iter(), true)
        .unwrap();
    let content = content_gstring.as_str();

    fs::save_file_changes(file_absolute_path, content)
}

pub fn on_open_dir_clicked(tx: &glib::Sender<CommEvents>) {
    let dir_filter = gtk::FileFilter::new();
    dir_filter.add_mime_type("inode/directory");

    let chooser = gtk::FileChooserDialog::builder()
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
