#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum MessageType {
    Info,
    Error,
    None,
}

impl Default for MessageType {
    fn default() -> Self {
        MessageType::None
    }
}

#[derive(Default)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

#[derive(Default)]
pub struct Message {
    pub text: String,
    pub count: usize,
    pub displayed: bool,
    pub message_type: MessageType,
}

#[derive(Default)]
pub struct YOrNState {
    pub is_active: bool,
    pub prompt: String,
}
