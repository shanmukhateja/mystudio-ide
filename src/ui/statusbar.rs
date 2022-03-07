use gtk::traits::StatusbarExt;

use crate::G_STATUS_BAR;

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

    status_bar_ref.push(0, &message);
}

pub fn show_status_message(message: String) {
    // Show message
    update_status_message(message);

    // TODO: hide message after few secs
}
