pub fn build_text_view() -> gtk::TextView {
    gtk::TextViewBuilder::new()
        .editable(true)
        .has_focus(true)
        .margin_start(5)
        .hexpand(true)
        .build()
}
