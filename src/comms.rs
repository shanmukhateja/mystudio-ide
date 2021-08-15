// A 'global' way to trigger GUI events
pub enum CommEvents {
    // Triggers TreeView#set_model
    UpdateRootTree(),

    // used to read text files
    RootTreeItemClicked(Option<String>),
    // Sets text to RootTextView
    UpdateRootTextViewContent(Option<String>),
    // Save Changes
    SaveEditorChanges()
}
