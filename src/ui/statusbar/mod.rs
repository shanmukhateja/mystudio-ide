pub mod line_indicator;
pub mod message;

use std::cell::RefCell;

use gtk::{prelude::BuilderExtManual, Button, Statusbar};

use crate::G_BUILDER;

thread_local! { pub static G_STATUS_BAR: RefCell<Option<Statusbar>> = RefCell::new(None) }
thread_local! { pub(self) static G_LINE_NUMBER: RefCell<Option<Button>> = RefCell::new(None) }

pub(self) fn get_status_bar() -> Statusbar {
    G_STATUS_BAR.with(|status_bar| {
        let status_bar_ref = status_bar.borrow();
        status_bar_ref
            .clone()
            .unwrap_or_else(|| panic!("{}", "Unable to find Status Bar"))
    })
}

pub fn init() {
    let builder = G_BUILDER.with(|builder| builder.borrow().clone().unwrap());

    G_STATUS_BAR.with(|status_bar| {
        *status_bar.borrow_mut() = builder.object("main_status_bar");
        assert!(status_bar.borrow().is_some());
    });

    line_indicator::init();
}
