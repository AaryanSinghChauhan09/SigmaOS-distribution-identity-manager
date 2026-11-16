// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-distribution-identity-manager: distribution identity manager component model.
//!
//! Real implementation: identitys, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identity {
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
pub enum DistributionIdentityManagerError {
    EmptyIdentityName,
    DuplicateIdentity(String),
    EmptyEventSubject,
}

impl fmt::Display for DistributionIdentityManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentityName => write!(f, "identity name must not be empty"),
            Self::DuplicateIdentity(id) => write!(f, "duplicate identity {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for DistributionIdentityManagerError {}

pub struct DistributionIdentityManager {
    pub name: String,
    identitys: BTreeMap<String, Identity>,
    events: BTreeMap<String, Event>,
}

impl DistributionIdentityManager {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            identitys: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_identity(&mut self, item: Identity) -> Result<(), DistributionIdentityManagerError> {
        if item.name.trim().is_empty() {
            return Err(DistributionIdentityManagerError::EmptyIdentityName);
        }
        let id = item.id.clone();
        if self.identitys.contains_key(&id) {
            return Err(DistributionIdentityManagerError::DuplicateIdentity(id.clone()));
        }
        self.identitys.insert(id, item);
        Ok(())
    }

    pub fn identity_count(&self) -> usize {
        self.identitys.len()
    }

    pub fn get_identity(&self, id: &str) -> Option<&Identity> {
        self.identitys.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), DistributionIdentityManagerError> {
        if event.subject.trim().is_empty() {
            return Err(DistributionIdentityManagerError::EmptyEventSubject);
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

    fn sample_identity(id: &str, name: &str) -> Identity {
        Identity {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = DistributionIdentityManager::new("DistributionIdentityManager");
        assert_eq!(m.name, "DistributionIdentityManager");
        assert_eq!(m.identity_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_identity() {
        let mut m = DistributionIdentityManager::new("DistributionIdentityManager");
        m.add_identity(sample_identity("a1", "Alpha")).unwrap();
        assert_eq!(m.identity_count(), 1);
        assert!(m.get_identity("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = DistributionIdentityManager::new("DistributionIdentityManager");
        assert!(m.add_identity(sample_identity("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = DistributionIdentityManager::new("DistributionIdentityManager");
        m.add_identity(sample_identity("x", "One")).unwrap();
        assert!(m.add_identity(sample_identity("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = DistributionIdentityManager::new("DistributionIdentityManager");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
