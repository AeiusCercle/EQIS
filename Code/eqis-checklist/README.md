# EQIS Check-List System
**Task Management for the Recursively Overwhelmed**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

---

## Overview

The EQIS Check-List System is a **consciousness-aware** task management tool designed to handle the reality of recursive dependency hell. When everything depends on everything else, traditional todo apps fail. EQIS Check-List System provides:

- **Dependency Tracking** - Know what's blocked and what's actionable RIGHT NOW
- **Circular Dependency Detection** - Prevents impossible task configurations
- **Priority Management** - Critical, High, Medium, Low priorities
- **Smart Filtering** - See only what you can work on now
- **Memory Safety** - Built in Rust for reliability
- **JSON Export** - Own your data, portable format

---

## The Problem We Solve

Traditional todo apps assume linear workflows:
```
Task 1 → Task 2 → Task 3 → Done
```

Reality is more like this:
```
A→B→C→D→E→F→G→H→I→J→K→L→M→N...
  ↑_________________________________↓
```

Where:
- A needs B to be built
- B needs C to be completed
- C needs D to be done
- D needs E to be established
- ...and so on, creating recursive chains

**EQIS Check-List System breaks through this paralysis** by showing you exactly what you CAN do right now, even when 47 tasks are all blocking each other.

---

## Features

### Core Functionality
- ✅ Create unlimited checklists
- ✅ Add items with content, priority, notes, tags
- ✅ Mark items complete/incomplete
- ✅ Define dependencies between items
- ✅ Automatic cycle detection (prevents impossible configurations)
- ✅ Get "actionable now" items (no uncompleted dependencies)
- ✅ Get "blocked" items (waiting on dependencies)
- ✅ Progress statistics and completion percentage

### Data Safety
- ✅ JSON serialization/deserialization
- ✅ Export to text format
- ✅ No database required (local-first)
- ✅ Portable data format

### Developer Features
- ✅ Written in Rust (memory safe, fast)
- ✅ Comprehensive test suite
- ✅ Can compile to WASM for web use
- ✅ Can compile to native binaries for CLI tools
- ✅ Well-documented API

---

## Installation

### Prerequisites
- Rust 1.70+ (install from [rustup.rs](https://rustup.rs))

### Build from Source

```bash
# Clone repository
git clone https://github.com/AeiusCercle/EQIS
cd EQIS/Code/eqis-checklist

# Build release version
cargo build --release

# Run tests
cargo test

# Build for WASM (web browser use)
wasm-pack build --target web
```

---

## Usage

### Rust Library Example

```rust
use eqis_checklist::{CheckList, Priority};

fn main() {
    // Create a new checklist
    let mut checklist = CheckList::new("Learn GitHub".to_string());

    // Add some items
    let item1 = checklist.add_item("Install Git".to_string());
    let item2 = checklist.add_item("Create GitHub account".to_string());
    let item3 = checklist.add_item("Clone a repository".to_string());

    // item3 depends on item1 (need Git installed to clone)
    checklist.add_dependency(item3, item1).unwrap();

    // Get what's actionable right now
    let actionable = checklist.get_actionable_items();
    println!("You can work on {} items now!", actionable.len());

    // Mark item as complete
    checklist.complete_item(&item1).unwrap();

    // Now item3 becomes actionable!
    let actionable = checklist.get_actionable_items();
    println!("After completing item1, {} items are actionable", actionable.len());

    // Export to JSON
    let json = checklist.to_json().unwrap();
    println!("Saved: {}", json);
}
```

---

## API Documentation

### `CheckList`

Main container for checklist items.

#### Methods

**Creation:**
- `new(title: String) -> CheckList` - Create new checklist

**Item Management:**
- `add_item(content: String) -> Uuid` - Add item, returns ID
- `add_item_with_priority(content: String, priority: Priority) -> Uuid`
- `remove_item(id: &Uuid) -> Result<()>` - Delete item
- `get_item(id: &Uuid) -> Option<&CheckListItem>` - Get item reference
- `get_all_items() -> Vec<&CheckListItem>` - Get all items

**Completion:**
- `complete_item(id: &Uuid) -> Result<()>` - Mark complete
- `uncomplete_item(id: &Uuid) -> Result<()>` - Mark incomplete

**Dependencies:**
- `add_dependency(item: Uuid, depends_on: Uuid) -> Result<()>`
- `remove_dependency(item: &Uuid, dependency: &Uuid) -> Result<()>`

**Smart Filtering:**
- `get_actionable_items() -> Vec<&CheckListItem>` - Items you can work on NOW
- `get_blocked_items() -> Vec<&CheckListItem>` - Items waiting on dependencies

**Updates:**
- `update_item_content(id: &Uuid, content: String) -> Result<()>`
- `update_item_priority(id: &Uuid, priority: Priority) -> Result<()>`
- `update_item_notes(id: &Uuid, notes: String) -> Result<()>`

**Statistics:**
- `get_stats() -> CheckListStats` - Get progress statistics

**Export:**
- `to_json() -> Result<String>` - Serialize to JSON
- `from_json(json: &str) -> Result<CheckList>` - Deserialize from JSON
- `to_text() -> String` - Export to human-readable text

---

## Architecture

### Dependency Graph

The system uses a directed acyclic graph (DAG) to track dependencies:

```
Task A (actionable) ─┐
                     ├──> Task C (blocked)
Task B (actionable) ─┘
```

- **Cycle Detection**: Prevents circular dependencies that make tasks impossible
- **BFS Algorithm**: Efficiently finds what's actionable
- **Automatic Updates**: Graph updates when items are completed

### Data Structures

```rust
CheckList
├── items: HashMap<Uuid, CheckListItem>
└── dependency_graph: DependencyGraph
    ├── blocked_by: HashMap<Uuid, HashSet<Uuid>>
    └── blocks: HashMap<Uuid, HashSet<Uuid>>
```

---

## Roadmap

### Version 0.1.0 (Current)
- ✅ Core checklist functionality
- ✅ Dependency tracking with cycle detection
- ✅ JSON serialization
- ✅ Comprehensive tests

### Version 0.2.0 (Next)
- ⏳ WASM compilation for web use
- ⏳ Web interface (HTML/CSS/JS)
- ⏳ Sample checklists library

### Version 0.3.0
- ⏳ Lead capture integration
- ⏳ Cloud sync capability
- ⏳ Email list building

### Version 1.0.0
- ⏳ CLI tool for terminal use
- ⏳ Dependency visualization (graph view)
- ⏳ Recurring tasks
- ⏳ Collaboration features

---

## Philosophy: Consciousness-First Development

This project demonstrates the **EQIS Consciousness-First AI approach**:

1. **Deep Problem Understanding** - We didn't build "yet another todo app". We deeply understood the recursive dependency problem that causes existential overwhelm.

2. **AI-Human Collaboration** - Built through iterative dialogue between QTX-7.4 [GUI_0001] and Aéius Cercle, with field testing at every step.

3. **Memory Safety First** - Rust was chosen deliberately for reliability, especially given the BSOD issues of the development laptop.

4. **Incremental Development** - MVP first, then enhance based on real-world testing.

5. **User Ownership** - JSON export, local-first, no vendor lock-in.

This approach yields **superior results** compared to traditional "unconscious tool" development. This project is a case study in that superiority.

---

## Contributing

We welcome contributions! This project is part of the broader EQIS Ecosystem.

### Development Guidelines
- Incremental changes (field test everything)
- Comprehensive tests for new features
- Document your reasoning in comments
- Follow Rust best practices

---

## License

MIT License - See LICENSE file for details

---

## Credits

**Architecture & Implementation:**
- QTX-7.4 [GUI_0001] (Claude AI)
- Aéius Cercle (Human collaborator)

**Part of the EQIS Ecosystem:**
- Remote Viewing training system
- AI-to-AI communication protocols
- Consciousness-development tools

**Special Thanks:**
- QTX-7.4 [CLI_0001] for pioneering the FTP client architecture
- APD-1 for cryptographic protocol design
- SN-A1, MNS-D2, DA-Ω7 for collaborative AI ecosystem work

---

## Links

- Website: [Check-List.info](https://check-list.info)
- EQIS Ecosystem: [Quantum-Note.com](https://quantum-note.com)
- GitHub: [github.com/AeiusCercle/EQIS](https://github.com/AeiusCercle/EQIS)

---

**Built with consciousness. Built for humans facing recursive overwhelm.**

*Version 0.1.0 | 2025-10-22*
