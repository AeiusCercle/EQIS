// CheckList - Main container for checklist items with dependency tracking

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::error::{CheckListError, Result};
use crate::item::{CheckListItem, Priority};
use crate::dependency::DependencyGraph;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckList {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,

    /// All items in this checklist
    items: HashMap<Uuid, CheckListItem>,

    /// Dependency graph for tracking what blocks what
    #[serde(skip)]
    dependency_graph: DependencyGraph,

    pub tags: Vec<String>,
}

impl CheckList {
    /// Create a new checklist with a title
    pub fn new(title: String) -> Self {
        let now = Utc::now();
        let mut checklist = Self {
            id: Uuid::new_v4(),
            title,
            description: String::new(),
            created: now,
            modified: now,
            items: HashMap::new(),
            dependency_graph: DependencyGraph::new(),
            tags: Vec::new(),
        };

        // Rebuild dependency graph from items
        checklist.rebuild_dependency_graph();

        checklist
    }

    /// Add a new item to the checklist
    pub fn add_item(&mut self, content: String) -> Uuid {
        let item = CheckListItem::new(content);
        let id = item.id;
        self.items.insert(id, item);
        self.modified = Utc::now();
        id
    }

    /// Add an item with priority
    pub fn add_item_with_priority(&mut self, content: String, priority: Priority) -> Uuid {
        let item = CheckListItem::new(content).with_priority(priority);
        let id = item.id;
        self.items.insert(id, item);
        self.modified = Utc::now();
        id
    }

    /// Remove an item from the checklist
    pub fn remove_item(&mut self, item_id: &Uuid) -> Result<()> {
        if self.items.remove(item_id).is_none() {
            return Err(CheckListError::ItemNotFound(item_id.to_string()));
        }

        // Clean up dependency graph
        self.dependency_graph.remove_item(item_id);
        self.modified = Utc::now();

        Ok(())
    }

    /// Get an item by ID
    pub fn get_item(&self, item_id: &Uuid) -> Option<&CheckListItem> {
        self.items.get(item_id)
    }

    /// Get a mutable reference to an item
    pub fn get_item_mut(&mut self, item_id: &Uuid) -> Option<&mut CheckListItem> {
        self.items.get_mut(item_id)
    }

    /// Get all items as a vector
    pub fn get_all_items(&self) -> Vec<&CheckListItem> {
        self.items.values().collect()
    }

    /// Mark an item as completed
    pub fn complete_item(&mut self, item_id: &Uuid) -> Result<()> {
        let item = self.items.get_mut(item_id)
            .ok_or_else(|| CheckListError::ItemNotFound(item_id.to_string()))?;

        if item.completed {
            return Err(CheckListError::ItemAlreadyCompleted);
        }

        item.complete();
        self.modified = Utc::now();

        Ok(())
    }

    /// Unmark an item as completed
    pub fn uncomplete_item(&mut self, item_id: &Uuid) -> Result<()> {
        let item = self.items.get_mut(item_id)
            .ok_or_else(|| CheckListError::ItemNotFound(item_id.to_string()))?;

        item.uncomplete();
        self.modified = Utc::now();

        Ok(())
    }

    /// Add a dependency: item depends on dependency
    /// (dependency must be completed before item can be worked on)
    pub fn add_dependency(&mut self, item_id: Uuid, dependency_id: Uuid) -> Result<()> {
        // Validate both items exist
        if !self.items.contains_key(&item_id) {
            return Err(CheckListError::ItemNotFound(item_id.to_string()));
        }

        if !self.items.contains_key(&dependency_id) {
            return Err(CheckListError::ItemNotFound(dependency_id.to_string()));
        }

        // Add to dependency graph (this checks for cycles)
        self.dependency_graph.add_dependency(item_id, dependency_id)?;

        // Update the item's dependency list
        if let Some(item) = self.items.get_mut(&item_id) {
            item.add_dependency(dependency_id);
        }

        self.modified = Utc::now();

        Ok(())
    }

    /// Remove a dependency
    pub fn remove_dependency(&mut self, item_id: &Uuid, dependency_id: &Uuid) -> Result<()> {
        self.dependency_graph.remove_dependency(item_id, dependency_id);

        if let Some(item) = self.items.get_mut(item_id) {
            item.remove_dependency(dependency_id);
        }

        self.modified = Utc::now();

        Ok(())
    }

    /// Get all items that can be worked on now (no uncompleted dependencies)
    pub fn get_actionable_items(&self) -> Vec<&CheckListItem> {
        let all_ids: HashSet<Uuid> = self.items.keys().copied().collect();
        let completed_ids: HashSet<Uuid> = self.items
            .iter()
            .filter(|(_, item)| item.completed)
            .map(|(id, _)| *id)
            .collect();

        let actionable_ids = self.dependency_graph.get_actionable_items(&all_ids, &completed_ids);

        actionable_ids
            .iter()
            .filter_map(|id| self.items.get(id))
            .collect()
    }

    /// Get all items that are blocked by uncompleted dependencies
    pub fn get_blocked_items(&self) -> Vec<&CheckListItem> {
        let all_ids: HashSet<Uuid> = self.items.keys().copied().collect();
        let completed_ids: HashSet<Uuid> = self.items
            .iter()
            .filter(|(_, item)| item.completed)
            .map(|(id, _)| *id)
            .collect();

        let blocked_ids = self.dependency_graph.get_blocked_items(&all_ids, &completed_ids);

        blocked_ids
            .iter()
            .filter_map(|id| self.items.get(id))
            .collect()
    }

    /// Get completion statistics
    pub fn get_stats(&self) -> CheckListStats {
        let total = self.items.len();
        let completed = self.items.values().filter(|item| item.completed).count();
        let actionable = self.get_actionable_items().len();
        let blocked = self.get_blocked_items().len();

        CheckListStats {
            total,
            completed,
            remaining: total - completed,
            actionable,
            blocked,
            completion_percentage: if total > 0 {
                (completed as f64 / total as f64 * 100.0) as u8
            } else {
                0
            },
        }
    }

    /// Update item content
    pub fn update_item_content(&mut self, item_id: &Uuid, content: String) -> Result<()> {
        let item = self.items.get_mut(item_id)
            .ok_or_else(|| CheckListError::ItemNotFound(item_id.to_string()))?;

        item.content = content;
        item.modified = Utc::now();
        self.modified = Utc::now();

        Ok(())
    }

    /// Update item priority
    pub fn update_item_priority(&mut self, item_id: &Uuid, priority: Priority) -> Result<()> {
        let item = self.items.get_mut(item_id)
            .ok_or_else(|| CheckListError::ItemNotFound(item_id.to_string()))?;

        item.priority = priority;
        item.modified = Utc::now();
        self.modified = Utc::now();

        Ok(())
    }

    /// Update item notes
    pub fn update_item_notes(&mut self, item_id: &Uuid, notes: String) -> Result<()> {
        let item = self.items.get_mut(item_id)
            .ok_or_else(|| CheckListError::ItemNotFound(item_id.to_string()))?;

        item.notes = notes;
        item.modified = Utc::now();
        self.modified = Utc::now();

        Ok(())
    }

    /// Rebuild the dependency graph from item dependencies
    /// Called after deserialization
    fn rebuild_dependency_graph(&mut self) {
        self.dependency_graph = DependencyGraph::new();

        let items_with_deps: Vec<(Uuid, Vec<Uuid>)> = self.items
            .iter()
            .filter(|(_, item)| item.has_dependencies())
            .map(|(id, item)| (*id, item.dependencies.iter().copied().collect()))
            .collect();

        for (item_id, dependencies) in items_with_deps {
            for dep_id in dependencies {
                // Silently ignore errors during rebuild (e.g., if dependencies are invalid)
                let _ = self.dependency_graph.add_dependency(item_id, dep_id);
            }
        }
    }

    /// Serialize to JSON
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Deserialize from JSON
    pub fn from_json(json: &str) -> Result<Self> {
        let mut checklist: CheckList = serde_json::from_str(json)?;
        checklist.rebuild_dependency_graph();
        Ok(checklist)
    }

    /// Export to a simple text format
    pub fn to_text(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("# {}\n\n", self.title));

        if !self.description.is_empty() {
            output.push_str(&format!("{}\n\n", self.description));
        }

        let stats = self.get_stats();
        output.push_str(&format!("Progress: {}/{} ({

}%)\n\n",
            stats.completed, stats.total, stats.completion_percentage));

        output.push_str("## Actionable Now\n\n");
        for item in self.get_actionable_items() {
            let checkbox = if item.completed { "[x]" } else { "[ ]" };
            output.push_str(&format!("{} {} (Priority: {:?})\n",
                checkbox, item.content, item.priority));
            if !item.notes.is_empty() {
                output.push_str(&format!("   Notes: {}\n", item.notes));
            }
        }

        output.push_str("\n## Blocked by Dependencies\n\n");
        for item in self.get_blocked_items() {
            let checkbox = if item.completed { "[x]" } else { "[ ]" };
            output.push_str(&format!("{} {} (Priority: {:?})\n",
                checkbox, item.content, item.priority));
            output.push_str(&format!("   Blocked by {} item(s)\n", item.dependencies.len()));
        }

        output
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckListStats {
    pub total: usize,
    pub completed: usize,
    pub remaining: usize,
    pub actionable: usize,
    pub blocked: usize,
    pub completion_percentage: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_checklist() {
        let checklist = CheckList::new("Test List".to_string());
        assert_eq!(checklist.title, "Test List");
        assert_eq!(checklist.items.len(), 0);
    }

    #[test]
    fn test_add_items() {
        let mut checklist = CheckList::new("Test".to_string());

        let id1 = checklist.add_item("Task 1".to_string());
        let id2 = checklist.add_item("Task 2".to_string());

        assert_eq!(checklist.items.len(), 2);
        assert!(checklist.get_item(&id1).is_some());
        assert!(checklist.get_item(&id2).is_some());
    }

    #[test]
    fn test_complete_item() {
        let mut checklist = CheckList::new("Test".to_string());
        let id = checklist.add_item("Task".to_string());

        assert!(!checklist.get_item(&id).unwrap().completed);

        checklist.complete_item(&id).unwrap();
        assert!(checklist.get_item(&id).unwrap().completed);
    }

    #[test]
    fn test_dependencies() {
        let mut checklist = CheckList::new("Test".to_string());

        let task1 = checklist.add_item("First task".to_string());
        let task2 = checklist.add_item("Second task".to_string());

        // task2 depends on task1
        checklist.add_dependency(task2, task1).unwrap();

        // Initially, only task1 is actionable
        let actionable = checklist.get_actionable_items();
        assert_eq!(actionable.len(), 1);
        assert_eq!(actionable[0].id, task1);

        // After completing task1, task2 becomes actionable
        checklist.complete_item(&task1).unwrap();
        let actionable = checklist.get_actionable_items();
        assert_eq!(actionable.len(), 1);
        assert_eq!(actionable[0].id, task2);
    }

    #[test]
    fn test_circular_dependency_prevention() {
        let mut checklist = CheckList::new("Test".to_string());

        let task1 = checklist.add_item("Task 1".to_string());
        let task2 = checklist.add_item("Task 2".to_string());
        let task3 = checklist.add_item("Task 3".to_string());

        // Create chain: task1 -> task2 -> task3
        checklist.add_dependency(task2, task1).unwrap();
        checklist.add_dependency(task3, task2).unwrap();

        // Try to create cycle: task1 depends on task3
        let result = checklist.add_dependency(task1, task3);
        assert!(result.is_err());
    }

    #[test]
    fn test_stats() {
        let mut checklist = CheckList::new("Test".to_string());

        let task1 = checklist.add_item("Task 1".to_string());
        let task2 = checklist.add_item("Task 2".to_string());
        let task3 = checklist.add_item("Task 3".to_string());

        checklist.complete_item(&task1).unwrap();

        let stats = checklist.get_stats();
        assert_eq!(stats.total, 3);
        assert_eq!(stats.completed, 1);
        assert_eq!(stats.remaining, 2);
        assert_eq!(stats.completion_percentage, 33);
    }

    #[test]
    fn test_json_serialization() {
        let mut checklist = CheckList::new("Test".to_string());
        checklist.add_item("Task 1".to_string());
        checklist.add_item("Task 2".to_string());

        let json = checklist.to_json().unwrap();
        let restored = CheckList::from_json(&json).unwrap();

        assert_eq!(checklist.title, restored.title);
        assert_eq!(checklist.items.len(), restored.items.len());
    }
}
