use gtk::{
    prelude::{ObjectExt, StaticType, TreeStoreExtManual},
    traits::{TreeSelectionExt, TreeViewExt},
    TreeIter, TreeStore,
};

use std::cell::RefCell;

use gtk::glib;

use crate::{fs::read_dir_recursive, workspace::Workspace};
pub struct TreeInfo {
    pub value: String,
    pub iter: TreeIter,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, glib::Enum)]
#[repr(i32)]
#[enum_type(name = "TreeNodeType")]
pub enum TreeNodeType {
    Unknown = -1,
    Directory = 0,
    File = 1,
    Workspace = 2,
}

impl Default for TreeNodeType {
    fn default() -> Self {
        TreeNodeType::Unknown
    }
}

mod imp {
    use std::cell::Cell;

    use gtk::{
        prelude::{StaticType, ToValue},
        subclass::prelude::{ObjectImpl, ObjectImplExt, ObjectSubclass},
    };

    use super::*;

    #[derive(Default, Debug)]
    pub struct RootTreeModel {
        file_name: RefCell<Option<String>>,
        abs_path: RefCell<Option<String>>,
        item_type: Cell<Option<TreeNodeType>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RootTreeModel {
        const NAME: &'static str = "RootTreeModel";

        type Type = super::RootTreeModel;
        type ParentType = glib::Object;

        type Interfaces = ();
    }

    impl ObjectImpl for RootTreeModel {
        fn properties() -> &'static [glib::ParamSpec] {
            use glib::once_cell::sync::Lazy;
            static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                vec![
                    glib::ParamSpecString::new(
                        "file-name",
                        "Name of File",
                        "File Name",
                        None,
                        glib::ParamFlags::READWRITE,
                    ),
                    glib::ParamSpecString::new(
                        "abs-path",
                        "Absolute Path of file",
                        "abs-path",
                        None,
                        glib::ParamFlags::READWRITE,
                    ),
                    glib::ParamSpecEnum::new(
                        "item-type",
                        "Node item type",
                        "item-type",
                        imp::TreeNodeType::static_type(),
                        TreeNodeType::Unknown as i32,
                        glib::ParamFlags::READWRITE,
                    ),
                ]
            });

            PROPERTIES.as_ref()
        }

        fn set_property(
            &self,
            _obj: &Self::Type,
            _id: usize,
            value: &glib::Value,
            pspec: &glib::ParamSpec,
        ) {
            match pspec.name() {
                "file-name" => {
                    let name = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.file_name.replace(name);
                }
                "abs-path" => {
                    let abs_path = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.abs_path.replace(abs_path);
                }
                "item-type" => {
                    let item_type = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.item_type.replace(Some(item_type));
                }
                e => {
                    println!("requested set-property for unknown property: '{}'", e);
                }
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            match pspec.name() {
                "file-name" => self.file_name.borrow().to_value(),
                "abs-path" => self.abs_path.borrow().to_value(),
                "item-type" => self.item_type.get().unwrap_or_default().to_value(),
                e => {
                    println!("requested unknown property: '{}'", e);
                    e.to_value()
                }
            }
        }

        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
        }
    }
}

glib::wrapper! {
    pub struct RootTreeModel(ObjectSubclass<imp::RootTreeModel>);
}

impl RootTreeModel {
    pub fn new() -> Self {
        glib::Object::new(&[]).unwrap()
    }

    pub fn build_tree_model() -> TreeStore {
        let store = TreeStore::new(&[RootTreeModel::static_type()]);

        let root_dir = Workspace::get_path();
        let mut files = read_dir_recursive(root_dir);

        if files.is_empty() {
            return store;
        }

        let root_dir = files.first().unwrap();

        // Custom Model
        let tree_model_struct = RootTreeModel::new();
        tree_model_struct.set_property("file-name", &root_dir.file_name().to_str().unwrap());
        tree_model_struct.set_property(
            "abs-path",
            &root_dir.parent_path().as_os_str().to_str().unwrap(),
        );
        tree_model_struct.set_property("item-type", &TreeNodeType::Workspace);

        let root_iter = store.insert_with_values(None, Some(1_u32), &[(0_u32, &tree_model_struct)]);

        // Cache tree_iter with file name
        let mut tree_info = vec![TreeInfo {
            iter: root_iter,
            value: String::from(root_dir.file_name().to_str().unwrap()),
        }];

        // FIX: duplicate Parent node in Tree
        // TODO: find a better way
        files.remove(0);


        for entry in files.iter() {

            let entry_path = entry.path();

            let entry_path_str = entry_path.to_str().unwrap();
            let entry_parent_str = entry_path.parent().unwrap().to_str().unwrap();
            let entry_file_str = entry_path.file_name().unwrap().to_str().unwrap();

            // Try to locate parent TreeIter entry using parent
            let found_info = tree_info.iter().find(|e| e.value == entry_parent_str);

            // If parent isn't found, treat it as child of `root_iter`
            let parent_iter = match found_info {
                Some(info) => &info.iter,
                None => &root_iter,
            };
            // Custom Model
            let tree_model_struct = RootTreeModel::new();
            let item_type = if entry_path.is_dir() {
                &TreeNodeType::Directory
            } else {
                &TreeNodeType::File
            };
            tree_model_struct.set_property("file-name", &entry_file_str);
            tree_model_struct.set_property("abs-path", &entry_path_str);
            tree_model_struct.set_property("item-type", &item_type);

            let m_iter =
                store.insert_with_values(Some(parent_iter), None, &[(0, &tree_model_struct)]);

            // Save to info list
            tree_info.push(TreeInfo {
                iter: m_iter,
                value: String::from(entry_path_str),
            });
        }

        store
    }

    pub fn update_tree_model(tree: &gtk::TreeView) {
        tree.set_model(Some(&RootTreeModel::build_tree_model()));
        // Expand root node and select it
        let root_node_path = gtk::TreePath::from_indicesv(&[0]);
        tree.expand_row(&root_node_path, false);
        tree.selection().select_path(&root_node_path);
    }
}

impl Default for RootTreeModel {
    fn default() -> Self {
        RootTreeModel::new()
    }
}
