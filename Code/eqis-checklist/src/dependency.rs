// Dependency tracking and cycle detection

use std::collections::{HashMap, HashSet, VecDeque};
use uuid::Uuid;
use crate::error::{CheckListError, Result};

/// Dependency graph for detecting circular dependencies and finding actionable items
pub struct DependencyGraph {
    /// Map of item_id -> set of items that BLOCK this item
    /// (i.e., dependencies that must be completed first)
    blocked_by: HashMap<Uuid, HashSet<Uuid>>,

    /// Map of item_id -> set of items that this item BLOCKS
    /// (i.e., items that depend on this one)
    blocks: HashMap<Uuid, HashSet<Uuid>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            blocked_by: HashMap::new(),
            blocks: HashMap::new(),
        }
    }

    /// Add a dependency: `item` depends on `dependency`
    /// Returns error if this would create a cycle
    pub fn add_dependency(&mut self, item: Uuid, dependency: Uuid) -> Result<()> {
        // Check for self-dependency
        if item == dependency {
            return Err(CheckListError::CircularDependency(
                "Item cannot depend on itself".to_string()
            ));
        }

        // Check if adding this dependency would create a cycle
        if self.would_create_cycle(item, dependency) {
            return Err(CheckListError::CircularDependency(
                format!("Adding dependency from {:?} to {:?} would create a cycle", item, dependency)
            ));
        }

        // Add to blocked_by map
        self.blocked_by
            .entry(item)
            .or_insert_with(HashSet::new)
            .insert(dependency);

        // Add to blocks map (reverse direction)
        self.blocks
            .entry(dependency)
            .or_insert_with(HashSet::new)
            .insert(item);

        Ok(())
    }

    /// Remove a dependency
    pub fn remove_dependency(&mut self, item: &Uuid, dependency: &Uuid) {
        if let Some(deps) = self.blocked_by.get_mut(item) {
            deps.remove(dependency);
        }

        if let Some(blocked) = self.blocks.get_mut(dependency) {
            blocked.remove(item);
        }
    }

    /// Check if adding a dependency would create a cycle
    /// Uses BFS to detect if `dependency` can reach `item`
    fn would_create_cycle(&self, item: Uuid, dependency: Uuid) -> bool {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(dependency);

        while let Some(current) = queue.pop_front() {
            if current == item {
                return true; // Found a path back to the original item
            }

            if visited.contains(&current) {
                continue;
            }

            visited.insert(current);

            // Add all items that `current` depends on
            if let Some(deps) = self.blocked_by.get(&current) {
                for dep in deps {
                    if !visited.contains(dep) {
                        queue.push_back(*dep);
                    }
                }
            }
        }

        false
    }

    /// Get all dependencies for an item
    pub fn get_dependencies(&self, item: &Uuid) -> HashSet<Uuid> {
        self.blocked_by.get(item).cloned().unwrap_or_default()
    }

    /// Check if an item is blocked by any uncompleted dependencies
    pub fn is_blocked(&self, item: &Uuid, completed_items: &HashSet<Uuid>) -> bool {
        if let Some(deps) = self.blocked_by.get(item) {
            // Item is blocked if ANY of its dependencies are not completed
            deps.iter().any(|dep| !completed_items.contains(dep))
        } else {
            false // No dependencies = not blocked
        }
    }

    /// Get all items that have no uncompleted dependencies (actionable items)
    pub fn get_actionable_items(
        &self,
        all_items: &HashSet<Uuid>,
        completed_items: &HashSet<Uuid>
    ) -> HashSet<Uuid> {
        all_items
            .iter()
            .filter(|item| {
                !completed_items.contains(item) && !self.is_blocked(item, completed_items)
            })
            .copied()
            .collect()
    }

    /// Get all items that are blocked by uncompleted dependencies
    pub fn get_blocked_items(
        &self,
        all_items: &HashSet<Uuid>,
        completed_items: &HashSet<Uuid>
    ) -> HashSet<Uuid> {
        all_items
            .iter()
            .filter(|item| {
                !completed_items.contains(item) && self.is_blocked(item, completed_items)
            })
            .copied()
            .collect()
    }

    /// Remove all references to an item (when item is deleted)
    pub fn remove_item(&mut self, item: &Uuid) {
        // Remove from blocked_by map
        self.blocked_by.remove(item);

        // Remove from all dependency lists
        for deps in self.blocked_by.values_mut() {
            deps.remove(item);
        }

        // Remove from blocks map
        self.blocks.remove(item);

        // Remove from all blocks lists
        for blocked in self.blocks.values_mut() {
            blocked.remove(item);
        }
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_dependency() {
        let mut graph = DependencyGraph::new();
        let item1 = Uuid::new_v4();
        let item2 = Uuid::new_v4();

        graph.add_dependency(item2, item1).unwrap();
        assert_eq!(graph.get_dependencies(&item2), HashSet::from([item1]));
    }

    #[test]
    fn test_self_dependency() {
        let mut graph = DependencyGraph::new();
        let item = Uuid::new_v4();

        let result = graph.add_dependency(item, item);
        assert!(result.is_err());
    }

    #[test]
    fn test_circular_dependency() {
        let mut graph = DependencyGraph::new();
        let item1 = Uuid::new_v4();
        let item2 = Uuid::new_v4();
        let item3 = Uuid::new_v4();

        // Create chain: item1 -> item2 -> item3
        graph.add_dependency(item2, item1).unwrap();
        graph.add_dependency(item3, item2).unwrap();

        // Try to create cycle: item3 -> item1 (would complete the cycle)
        let result = graph.add_dependency(item1, item3);
        assert!(result.is_err());
    }

    #[test]
    fn test_actionable_items() {
        let mut graph = DependencyGraph::new();
        let item1 = Uuid::new_v4();
        let item2 = Uuid::new_v4();
        let item3 = Uuid::new_v4();

        // item2 depends on item1, item3 has no dependencies
        graph.add_dependency(item2, item1).unwrap();

        let all_items = HashSet::from([item1, item2, item3]);
        let completed = HashSet::new();

        let actionable = graph.get_actionable_items(&all_items, &completed);

        // item1 and item3 are actionable (no dependencies)
        // item2 is blocked (depends on item1)
        assert!(actionable.contains(&item1));
        assert!(!actionable.contains(&item2));
        assert!(actionable.contains(&item3));
    }

    #[test]
    fn test_actionable_after_completion() {
        let mut graph = DependencyGraph::new();
        let item1 = Uuid::new_v4();
        let item2 = Uuid::new_v4();

        graph.add_dependency(item2, item1).unwrap();

        let all_items = HashSet::from([item1, item2]);
        let mut completed = HashSet::new();

        // Initially, only item1 is actionable
        let actionable = graph.get_actionable_items(&all_items, &completed);
        assert!(actionable.contains(&item1));
        assert!(!actionable.contains(&item2));

        // After completing item1, item2 becomes actionable
        completed.insert(item1);
        let actionable = graph.get_actionable_items(&all_items, &completed);
        assert!(!actionable.contains(&item1)); // Already completed
        assert!(actionable.contains(&item2)); // Now actionable
    }
}
