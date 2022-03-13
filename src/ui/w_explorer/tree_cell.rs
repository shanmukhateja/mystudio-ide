use gtk::{TreeViewColumn, CellRenderer, TreeModel, TreeIter, traits::TreeModelExt, prelude::ObjectExt, CellRendererText, CellRendererPixbuf};

use super::model::{RootTreeModel, TreeNodeType};

pub 
fn set_cell_data(
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
        
        let icon_name =  match icon_type {
            TreeNodeType::Unknown => "dialog-warning",
            TreeNodeType::Directory => filetype.as_str(),
            TreeNodeType::File => filetype.as_str(),
            TreeNodeType::Workspace => "folder-open",
        };
        cell.set_property("icon-name", icon_name);
    }
}


fn get_icon_for_name(filename: &str, icon_type: TreeNodeType) -> String {
    if icon_type == TreeNodeType::Directory {
        return "folder".to_owned();
    }

    let (guess, _) = gtk::gio::content_type_guess(Some(filename), &[]);
    guess.as_str().to_owned()
}