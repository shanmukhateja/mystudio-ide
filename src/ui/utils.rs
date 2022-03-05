use gtk::traits::{TextViewExt, TextBufferExt};
use sourceview4::{
    traits::{BufferExt, LanguageManagerExt},
    LanguageManager,
};

pub fn set_text_on_editor(
    text_editor: &sourceview4::View,
    file_path: Option<String>,
    content: Option<String>,
) {
    match content {
        Some(content) => {
            let source_buffer = sourceview4::Buffer::builder()
                .text(content.as_str())
                .build();

            // Detect language for syntax highlight
            let lang_manager = LanguageManager::new();
            match lang_manager.guess_language(Some(file_path.unwrap()), None) {
                Some(lang) => {
                    source_buffer.set_language(Some(&lang));
                }
                None => {
                    source_buffer.set_language(sourceview4::Language::NONE);
                }
            }
            // update buffer in View
            text_editor.set_buffer(Some(&source_buffer));
            // Show cursor on text_view so user can start modifying file
            // FIXME: this is broken because of Notebook UI impl.
            // text_editor.grab_focus();
        }
        None => {
            // Reset text content
            text_editor.buffer().unwrap().set_text("");
        }
    }
}
