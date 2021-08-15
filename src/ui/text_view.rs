pub fn build_text_view() -> gtk::TextView {
    let text_view = gtk::TextViewBuilder::new()
        .editable(true)
        .has_focus(true)
        .hexpand(true)
        .build();

    text_view
}
