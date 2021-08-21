use std::cell::RefCell;

use gtk::glib;

mod imp {
    use gtk::{
        prelude::ToValue,
        subclass::prelude::{ObjectImpl, ObjectImplExt, ObjectSubclass},
    };

    use super::*;

    #[derive(Default, Debug)]
    pub struct RootTreeModel {
        file_name: RefCell<Option<String>>,
        abs_path: RefCell<Option<String>>,
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
                    glib::ParamSpec::new_string(
                        "file-name",
                        "Name of File",
                        "File Name",
                        None,
                        glib::ParamFlags::READWRITE,
                    ),
                    glib::ParamSpec::new_string(
                        "abs-path",
                        "Absolute Path of file",
                        "abs-path",
                        None,
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
                e => {
                    println!("got e: {} for set_property", e);
                }
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            match pspec.name() {
                "file-name" => self.file_name.borrow().to_value(),
                "abs-path" => self.abs_path.borrow().to_value(),
                e => {
                    println!("e: {}", e);
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
