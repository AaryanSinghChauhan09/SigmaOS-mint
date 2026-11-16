// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-mint: mint component model.
//!
//! Real implementation: mint_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MintItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MintError {
    EmptyMintItemName,
    DuplicateMintItem(String),
    EmptyEventSubject,
}

impl fmt::Display for MintError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyMintItemName => write!(f, "mint_item name must not be empty"),
            Self::DuplicateMintItem(id) => write!(f, "duplicate mint_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for MintError {}

pub struct Mint {
    pub name: String,
    mint_items: BTreeMap<String, MintItem>,
    events: BTreeMap<String, Event>,
}

impl Mint {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            mint_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_mint_item(&mut self, item: MintItem) -> Result<(), MintError> {
        if item.name.trim().is_empty() {
            return Err(MintError::EmptyMintItemName);
        }
        let id = item.id.clone();
        if self.mint_items.contains_key(&id) {
            return Err(MintError::DuplicateMintItem(id.clone()));
        }
        self.mint_items.insert(id, item);
        Ok(())
    }

    pub fn mint_item_count(&self) -> usize {
        self.mint_items.len()
    }

    pub fn get_mint_item(&self, id: &str) -> Option<&MintItem> {
        self.mint_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), MintError> {
        if event.subject.trim().is_empty() {
            return Err(MintError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_mint_item(id: &str, name: &str) -> MintItem {
        MintItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Mint::new("Mint");
        assert_eq!(m.name, "Mint");
        assert_eq!(m.mint_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_mint_item() {
        let mut m = Mint::new("Mint");
        m.add_mint_item(sample_mint_item("a1", "Alpha")).unwrap();
        assert_eq!(m.mint_item_count(), 1);
        assert!(m.get_mint_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Mint::new("Mint");
        assert!(m.add_mint_item(sample_mint_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Mint::new("Mint");
        m.add_mint_item(sample_mint_item("x", "One")).unwrap();
        assert!(m.add_mint_item(sample_mint_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Mint::new("Mint");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
