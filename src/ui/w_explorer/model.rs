use gtk::TreeIter;

use std::cell::RefCell;

use gtk::glib;
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
}

impl Default for RootTreeModel {
    fn default() -> Self {
        RootTreeModel::new()
    }
}
