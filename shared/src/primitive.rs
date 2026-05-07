#[derive(Debug, PartialEq)]
pub enum LoopAction {
    Continue,
    Stop,
}

/// Returns the byte offset of the character at the given char index within a UTF-8 string.
/// If `char_index` is past the end of the string, returns `text.len()` (one past the last byte).
pub fn byte_offset_at(text: &str, char_index: usize) -> usize {
    text.char_indices()
        .nth(char_index)
        .map(|(byte_index, _)| byte_index)
        .unwrap_or(text.len())
}

/// Inserts a character at the given char index within a UTF-8 string.
pub fn insert_char(text: &mut String, char_index: usize, ch: char) {
    let byte_index: usize = byte_offset_at(text, char_index);
    text.insert(byte_index, ch);
}

/// Removes the character at the given char index within a UTF-8 string.
pub fn remove_char(text: &mut String, char_index: usize) {
    let byte_index: usize = byte_offset_at(text, char_index);
    text.remove(byte_index);
}
