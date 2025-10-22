// CheckListItem - Individual task/item in a checklist

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Medium
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckListItem {
    pub id: Uuid,
    pub content: String,
    pub completed: bool,
    pub priority: Priority,

    /// UUIDs of items that must be completed before this one
    pub dependencies: HashSet<Uuid>,

    pub notes: String,
    pub tags: Vec<String>,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl CheckListItem {
    pub fn new(content: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            content,
            completed: false,
            priority: Priority::default(),
            dependencies: HashSet::new(),
            notes: String::new(),
            tags: Vec::new(),
            created: now,
            modified: now,
            completed_at: None,
        }
    }

    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self.modified = Utc::now();
        self
    }

    pub fn with_notes(mut self, notes: String) -> Self {
        self.notes = notes;
        self.modified = Utc::now();
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self.modified = Utc::now();
        self
    }

    pub fn add_dependency(&mut self, dependency_id: Uuid) {
        self.dependencies.insert(dependency_id);
        self.modified = Utc::now();
    }

    pub fn remove_dependency(&mut self, dependency_id: &Uuid) {
        self.dependencies.remove(dependency_id);
        self.modified = Utc::now();
    }

    pub fn complete(&mut self) {
        self.completed = true;
        self.completed_at = Some(Utc::now());
        self.modified = Utc::now();
    }

    pub fn uncomplete(&mut self) {
        self.completed = false;
        self.completed_at = None;
        self.modified = Utc::now();
    }

    pub fn is_blocked_by(&self, other_id: &Uuid) -> bool {
        self.dependencies.contains(other_id)
    }

    pub fn has_dependencies(&self) -> bool {
        !self.dependencies.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_item() {
        let item = CheckListItem::new("Test task".to_string());
        assert_eq!(item.content, "Test task");
        assert!(!item.completed);
        assert_eq!(item.priority, Priority::Medium);
    }

    #[test]
    fn test_complete_item() {
        let mut item = CheckListItem::new("Test".to_string());
        assert!(!item.completed);
        assert!(item.completed_at.is_none());

        item.complete();
        assert!(item.completed);
        assert!(item.completed_at.is_some());
    }

    #[test]
    fn test_dependencies() {
        let mut item = CheckListItem::new("Blocked task".to_string());
        let blocker_id = Uuid::new_v4();

        assert!(!item.has_dependencies());
        item.add_dependency(blocker_id);
        assert!(item.has_dependencies());
        assert!(item.is_blocked_by(&blocker_id));
    }
}
