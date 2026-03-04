use esm_core::inbox::{Inbox, Message, MessageCategory, MessagePriority};

// ---------------------------------------------------------------------------
// MessagePriority enum
// ---------------------------------------------------------------------------

#[test]
fn message_priority_variants_exist() {
    let _ = MessagePriority::ReadOptional;
    let _ = MessagePriority::RequiresResponse;
    let _ = MessagePriority::HardBlock;
}

// ---------------------------------------------------------------------------
// MessageCategory enum
// ---------------------------------------------------------------------------

#[test]
fn message_category_variants_exist() {
    let _ = MessageCategory::News;
    let _ = MessageCategory::Transfer;
    let _ = MessageCategory::Contract;
    let _ = MessageCategory::Scrim;
    let _ = MessageCategory::Board;
    let _ = MessageCategory::Staff;
    let _ = MessageCategory::Injury;
    let _ = MessageCategory::MetaShift;
}

// ---------------------------------------------------------------------------
// Message construction
// ---------------------------------------------------------------------------

fn make_info_message() -> Message {
    Message::new(
        "League Update".to_string(),
        "Patch 14.5 is live.".to_string(),
        MessagePriority::ReadOptional,
        MessageCategory::News,
        1,
    )
}

fn make_actionable_message() -> Message {
    Message::new(
        "Transfer Request".to_string(),
        "Player X wants to leave.".to_string(),
        MessagePriority::RequiresResponse,
        MessageCategory::Transfer,
        3,
    )
}

fn make_blocking_message() -> Message {
    Message::new(
        "Roster Gap".to_string(),
        "You must fill the Mid position before matchday.".to_string(),
        MessagePriority::HardBlock,
        MessageCategory::Board,
        5,
    )
}

#[test]
fn message_stores_fields() {
    let msg = make_info_message();
    assert_eq!(msg.subject(), "League Update");
    assert_eq!(msg.body(), "Patch 14.5 is live.");
    assert_eq!(msg.priority(), MessagePriority::ReadOptional);
    assert_eq!(msg.category(), MessageCategory::News);
    assert_eq!(msg.day_received(), 1);
    assert!(!msg.is_resolved());
}

#[test]
fn message_can_be_resolved() {
    let mut msg = make_actionable_message();
    assert!(!msg.is_resolved());
    msg.resolve();
    assert!(msg.is_resolved());
}

// ---------------------------------------------------------------------------
// Inbox
// ---------------------------------------------------------------------------

#[test]
fn inbox_starts_empty() {
    let inbox = Inbox::new();
    assert_eq!(inbox.len(), 0);
    assert!(inbox.is_empty());
}

#[test]
fn inbox_push_adds_message() {
    let mut inbox = Inbox::new();
    inbox.push(make_info_message());
    assert_eq!(inbox.len(), 1);
}

#[test]
fn inbox_has_unresolved_returns_true_when_unresolved_exist() {
    let mut inbox = Inbox::new();
    inbox.push(make_actionable_message());
    assert!(inbox.has_unresolved());
}

#[test]
fn inbox_has_unresolved_returns_false_when_all_resolved() {
    let mut inbox = Inbox::new();
    let mut msg = make_actionable_message();
    msg.resolve();
    inbox.push(msg);
    assert!(!inbox.has_unresolved());
}

#[test]
fn inbox_has_blocking_returns_true_for_hard_block() {
    let mut inbox = Inbox::new();
    inbox.push(make_blocking_message());
    assert!(inbox.has_blocking());
}

#[test]
fn inbox_has_blocking_returns_false_when_none() {
    let mut inbox = Inbox::new();
    inbox.push(make_info_message());
    assert!(!inbox.has_blocking());
}

#[test]
fn inbox_has_blocking_returns_false_when_blocking_resolved() {
    let mut inbox = Inbox::new();
    let mut msg = make_blocking_message();
    msg.resolve();
    inbox.push(msg);
    assert!(!inbox.has_blocking());
}

#[test]
fn inbox_can_continue_when_no_blocking_messages() {
    let mut inbox = Inbox::new();
    inbox.push(make_info_message());
    inbox.push(make_actionable_message());
    assert!(inbox.can_continue());
}

#[test]
fn inbox_cannot_continue_when_blocking_messages_exist() {
    let mut inbox = Inbox::new();
    inbox.push(make_blocking_message());
    assert!(!inbox.can_continue());
}

#[test]
fn inbox_unresolved_messages_filters_correctly() {
    let mut inbox = Inbox::new();
    inbox.push(make_info_message());
    inbox.push(make_actionable_message());

    let mut resolved = make_blocking_message();
    resolved.resolve();
    inbox.push(resolved);

    let unresolved = inbox.unresolved();
    assert_eq!(unresolved.len(), 2);
}

#[test]
fn inbox_messages_by_priority_filters_correctly() {
    let mut inbox = Inbox::new();
    inbox.push(make_info_message());
    inbox.push(make_info_message());
    inbox.push(make_actionable_message());
    inbox.push(make_blocking_message());

    let news = inbox.by_priority(MessagePriority::ReadOptional);
    assert_eq!(news.len(), 2);

    let actionable = inbox.by_priority(MessagePriority::RequiresResponse);
    assert_eq!(actionable.len(), 1);

    let blocking = inbox.by_priority(MessagePriority::HardBlock);
    assert_eq!(blocking.len(), 1);
}

#[test]
fn inbox_resolve_by_index() {
    let mut inbox = Inbox::new();
    inbox.push(make_actionable_message());
    inbox.push(make_blocking_message());

    assert!(inbox.resolve_at(0));
    assert!(inbox.messages()[0].is_resolved());
    assert!(!inbox.messages()[1].is_resolved());
}

#[test]
fn inbox_resolve_by_index_out_of_bounds_returns_false() {
    let mut inbox = Inbox::new();
    assert!(!inbox.resolve_at(0));
}
