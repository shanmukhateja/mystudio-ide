use gtk::{
    prelude::ObjectExt, traits::TreeModelExt, CellRenderer, CellRendererPixbuf, CellRendererText,
    TreeIter, TreeModel, TreeViewColumn,
};

use super::tree_model::{RootTreeModel, TreeNodeType};

pub fn set_cell_data(
    _: &TreeViewColumn,
    cell: &CellRenderer,
    tree_model: &TreeModel,
    tree_iter: &TreeIter,
) {
    let tree_model = tree_model
        .value(tree_iter, 0)
        .get::<RootTreeModel>()
        .unwrap();

    // Set the text
    let filename = tree_model.property_value("file-name");
    if cell.is::<CellRendererText>() {
        cell.set_property("text", filename.clone());
    }

    // Set icon
    if cell.is::<CellRendererPixbuf>() {
        let icon_type = tree_model
            .property_value("item-type")
            .get::<TreeNodeType>()
            .unwrap();

        let filename = filename.get().unwrap();
        let filetype = get_icon_for_name(filename, icon_type);

        let icon_name = match icon_type {
            TreeNodeType::Unknown => "dialog-warning",
            TreeNodeType::Directory => filetype.as_str(),
            TreeNodeType::File => filetype.as_str(),
            TreeNodeType::Workspace => "folder-open",
        };
        cell.set_property("icon-name", icon_name);
    }
}

pub fn get_icon_for_name(filename: &str, icon_type: TreeNodeType) -> String {
    if icon_type == TreeNodeType::Directory {
        return "folder".to_owned();
    }

    get_icon_name(filename)
    // FIXME: find a better way
    .replace('/', "-")
}

#[cfg(target_os = "linux")]
fn get_icon_name(filename: &str) -> String {
    let (guess, _) = gtk::gio::content_type_guess(Some(filename), &[]);
    guess.to_string()
}

#[cfg(target_os = "windows")]
fn get_icon_name(filename: &str) -> String {
    new_mime_guess::from_path(filename)
        .first_or_text_plain()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::tree::tree_cell::get_icon_for_name;

    fn test_icon_name_mime<'a>(
        name: &'a str,
        mime: &'a str,
        fallback_mime: Option<&'a str>,
    ) -> bool {
        let fetched_mime = get_icon_for_name(name, crate::tree::tree_model::TreeNodeType::File);

        if let Some(fallback) = fallback_mime {
            fetched_mime == fallback || fetched_mime == mime
        } else {
            fetched_mime == mime
        }
    }

    /**
     * Note: This test needs to be run on Linux and Windows for 100% assurance.
     */
    #[test]
    fn get_icon_name_rust() {
        assert!(test_icon_name_mime(
            "file.rs",
            "text-rust",
            Some("text-x-rust")
        ));
    }

    /**
     * This test needs to be run on Linux and Windows for 100% assurance.
     */
    #[test]
    fn get_icon_name_js() {
        assert!(test_icon_name_mime(
            "file.js",
            "application-javascript",
            None
        ));
    }
}
