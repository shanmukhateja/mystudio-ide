use std::time::Duration;

use gtk::traits::StatusbarExt;

use crate::G_STATUS_BAR;

const CONTENT_ID: u32 = 0;
const MESSAGE_HIDE_DELAY: Duration = Duration::from_secs(3);

fn get_status_bar() -> gtk::Statusbar {
    G_STATUS_BAR.with(|status_bar| {
        let status_bar_ref = status_bar.borrow();
        status_bar_ref
            .clone()
            .unwrap_or_else(|| panic!("{}", "Unable to find Status Bar"))
    })
}


fn update_status_message(message: String) {
    let status_bar_ref = get_status_bar();

    status_bar_ref.push(CONTENT_ID, &message);
}

fn reset_status_message() {
    let status_bar_ref = get_status_bar();

    status_bar_ref.remove_all(CONTENT_ID);
}

pub fn show_status_message(message: String) {
    // Show message
    update_status_message(message);

    gtk::glib::timeout_add_once::<_>(MESSAGE_HIDE_DELAY, || {
        reset_status_message();      
    });
}