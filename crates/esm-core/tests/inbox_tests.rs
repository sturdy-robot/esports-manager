use std::str::FromStr;

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

// ---------------------------------------------------------------------------
// MessagePriority: string conversions
// ---------------------------------------------------------------------------

#[test]
fn priority_as_str() {
    assert_eq!(MessagePriority::ReadOptional.as_str(), "ReadOptional");
    assert_eq!(
        MessagePriority::RequiresResponse.as_str(),
        "RequiresResponse"
    );
    assert_eq!(MessagePriority::HardBlock.as_str(), "HardBlock");
}

#[test]
fn priority_from_str_valid() {
    assert_eq!(
        MessagePriority::from_str("ReadOptional").unwrap(),
        MessagePriority::ReadOptional
    );
    assert_eq!(
        MessagePriority::from_str("RequiresResponse").unwrap(),
        MessagePriority::RequiresResponse
    );
    assert_eq!(
        MessagePriority::from_str("HardBlock").unwrap(),
        MessagePriority::HardBlock
    );
}

#[test]
fn priority_from_str_invalid() {
    assert!(MessagePriority::from_str("Critical").is_err());
}

// ---------------------------------------------------------------------------
// MessageCategory: string conversions
// ---------------------------------------------------------------------------

#[test]
fn category_as_str() {
    assert_eq!(MessageCategory::News.as_str(), "News");
    assert_eq!(MessageCategory::Transfer.as_str(), "Transfer");
    assert_eq!(MessageCategory::Contract.as_str(), "Contract");
    assert_eq!(MessageCategory::Scrim.as_str(), "Scrim");
    assert_eq!(MessageCategory::Board.as_str(), "Board");
    assert_eq!(MessageCategory::Staff.as_str(), "Staff");
    assert_eq!(MessageCategory::Injury.as_str(), "Injury");
    assert_eq!(MessageCategory::MetaShift.as_str(), "MetaShift");
}

#[test]
fn category_from_str_valid() {
    assert_eq!(
        MessageCategory::from_str("News").unwrap(),
        MessageCategory::News
    );
    assert_eq!(
        MessageCategory::from_str("Transfer").unwrap(),
        MessageCategory::Transfer
    );
    assert_eq!(
        MessageCategory::from_str("Board").unwrap(),
        MessageCategory::Board
    );
    assert_eq!(
        MessageCategory::from_str("MetaShift").unwrap(),
        MessageCategory::MetaShift
    );
}

#[test]
fn category_from_str_invalid() {
    assert!(MessageCategory::from_str("Unknown").is_err());
}
