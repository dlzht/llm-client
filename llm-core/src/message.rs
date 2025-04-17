use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Message {
    role: Role,
    content: String,
}

impl Message {
    pub fn system(content: impl Into<String>) -> Self {
        Message {
            role: Role::System,
            content: content.into(),
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Message {
            role: Role::User,
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Message {
            role: Role::Assistant,
            content: content.into(),
        }
    }

    pub fn tool(content: impl Into<String>) -> Self {
        Message {
            role: Role::Tool,
            content: content.into(),
        }
    }

}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Messages {
    messages: Vec<Message>,
}

impl Messages {
    pub fn new() -> Self {
        Messages {
            messages: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.messages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    pub fn push(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn pop(&mut self) -> Option<Message> {
        self.messages.pop()
    }

    pub fn clear(&mut self) {
        self.messages.clear()
    }

    pub fn message_ref(&self) -> &[Message] {
        &self.messages
    }
}

#[cfg(test)]
mod test {
    use crate::message::{Message, Messages, Role};

    #[test]
    fn test_message() {
        let message = Message::system("msg");
        assert_eq!(Message {role: Role::System, content: "msg".to_string()}, message);

        let message = Message::user("msg");
        assert_eq!(Message {role: Role::User, content: "msg".to_string()}, message);

        let message = Message::tool("msg");
        assert_eq!(Message {role: Role::Tool, content: "msg".to_string()}, message);

        let message = Message::assistant("msg");
        assert_eq!(Message {role: Role::Assistant, content: "msg".to_string()}, message);
    }

    #[test]
    fn test_messages() {
        let mut messages = Messages::new();
        assert_eq!(messages.len(), 0);
        assert!(messages.is_empty());

        messages.push(Message::system("msg"));
        assert_eq!(messages.len(), 1);
        assert!(!messages.is_empty());

    }
}