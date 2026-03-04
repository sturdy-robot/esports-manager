use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessagePriority {
    ReadOptional,
    RequiresResponse,
    HardBlock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageCategory {
    News,
    Transfer,
    Contract,
    Scrim,
    Board,
    Staff,
    Injury,
    MetaShift,
}

// ---------------------------------------------------------------------------
// Message
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    subject: String,
    body: String,
    priority: MessagePriority,
    category: MessageCategory,
    day_received: u32,
    resolved: bool,
}

impl Message {
    pub fn new(
        subject: String,
        body: String,
        priority: MessagePriority,
        category: MessageCategory,
        day_received: u32,
    ) -> Self {
        Self {
            subject,
            body,
            priority,
            category,
            day_received,
            resolved: false,
        }
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn priority(&self) -> MessagePriority {
        self.priority
    }

    pub fn category(&self) -> MessageCategory {
        self.category
    }

    pub fn day_received(&self) -> u32 {
        self.day_received
    }

    pub fn is_resolved(&self) -> bool {
        self.resolved
    }

    pub fn resolve(&mut self) {
        self.resolved = true;
    }
}

// ---------------------------------------------------------------------------
// Inbox
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inbox {
    messages: Vec<Message>,
}

impl Inbox {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    pub fn push(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn len(&self) -> usize {
        self.messages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    pub fn has_unresolved(&self) -> bool {
        self.messages.iter().any(|m| !m.is_resolved())
    }

    pub fn has_blocking(&self) -> bool {
        self.messages
            .iter()
            .any(|m| m.priority() == MessagePriority::HardBlock && !m.is_resolved())
    }

    /// Returns `true` if there are no unresolved `HardBlock` messages,
    /// meaning the player can press "Continue" to advance the turn.
    pub fn can_continue(&self) -> bool {
        !self.has_blocking()
    }

    pub fn unresolved(&self) -> Vec<&Message> {
        self.messages.iter().filter(|m| !m.is_resolved()).collect()
    }

    pub fn by_priority(&self, priority: MessagePriority) -> Vec<&Message> {
        self.messages
            .iter()
            .filter(|m| m.priority() == priority)
            .collect()
    }

    /// Resolve the message at the given index. Returns `false` if out of bounds.
    pub fn resolve_at(&mut self, index: usize) -> bool {
        if let Some(msg) = self.messages.get_mut(index) {
            msg.resolve();
            true
        } else {
            false
        }
    }
}

impl Default for Inbox {
    fn default() -> Self {
        Self::new()
    }
}
