use super::TreeDisplaySettings;
use crate::print_tree::TREE_STYLE;
use lopdf::ObjectId;
use std::{cell::Cell, rc::Rc};

static TAB_WIDTH: usize = 2;
static ARROW_LAST_CHAR: &str = "└";
static ARROW_CHAR: &str = "├";
static INDENT_CHAR: &str = "│";

#[derive(Debug, Clone)]
pub struct TreeCursorInfo {
    /// Keeps track of the depth in the tree.
    depth_info: Vec<DepthInfo>,
    /// Keeps track of all parents `ObjectId`s to prevent loops.
    parent_refs: Vec<ObjectId>,
    /// Shared info among the all cursors in this tree.
    shared_info: Rc<Cell<SharedCursorInfo>>,
}

#[derive(Debug, Default, Clone)]
pub struct DepthInfo {
    pub name: Option<String>,
    pub indent_line: bool,
}

#[derive(Debug, Default, Clone, Copy)]
struct SharedCursorInfo {
    settings: TreeCursorSettings,
    line_number: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct TreeCursorSettings {
    pub print_line_numbers: bool,
    pub line_number_padding: u8,
}

impl Default for TreeCursorSettings {
    fn default() -> Self {
        TreeCursorSettings {
            print_line_numbers: true,
            line_number_padding: 4,
        }
    }
}

impl SharedCursorInfo {
    pub fn new(settings: &TreeCursorSettings) -> Self {
        Self {
            settings: *settings,
            ..Default::default()
        }
    }
}

impl TreeCursorInfo {
    /// Create a new Tree Cursor.
    ///
    /// This should be used to create a new independent tree.
    pub fn new(settings: &TreeCursorSettings) -> Self {
        let shared_info = Rc::new(Cell::new(SharedCursorInfo::new(settings)));
        Self {
            depth_info: Vec::new(),
            parent_refs: Vec::new(),
            shared_info,
        }
    }

    pub fn add_depth(&self, depth_info: DepthInfo) -> Self {
        let mut new_cursor = self.clone();
        new_cursor.depth_info.push(depth_info);
        new_cursor
    }

    pub fn get_depth_count(&self) -> usize {
        self.depth_info.len()
    }

    pub fn get_path(&self) -> Vec<String> {
        let mut path = Vec::new();
        for item in &self.depth_info {
            if let Some(name) = &item.name {
                path.push(name.clone());
            }
        }
        path
    }

    pub fn next_expand_label(&self, settings: &TreeDisplaySettings) -> Result<Option<String>, ()> {
        if let Some(expand_list) = &settings.expand {
            let path = self.get_path();
            for (index, item) in expand_list.iter().enumerate() {
                if let Some(path_item) = path.get(index) {
                    // Found item in path, this should match the next expand item.
                    if path_item == item {
                        // Everything okay, next
                        continue;
                    } else {
                        // There was a wrong path taken somewhere
                        return Err(());
                    }
                } else {
                    // No path item found, so return this expand item.
                    return Ok(Some(item.clone()));
                }
            }
            // We are inside the part of the tree that the `expand_list` described.
            Ok(None)
        } else {
            // There is no expand list, so no filter needed
            Ok(None)
        }
    }

    pub fn check_parent_visited(&self, check: &ObjectId) -> bool {
        self.parent_refs.contains(check)
    }

    pub fn add_parent_object_id(&mut self, parent: ObjectId) {
        self.parent_refs.push(parent)
    }

    pub fn print_subitem(&self, text: String, last: bool) {
        let mut shared_info = self.shared_info.get();

        let line_number = if shared_info.settings.print_line_numbers {
            // Increment line number
            shared_info.line_number += 1;
            self.shared_info.replace(shared_info);
            // Return line number prefix
            let number_string = shared_info.line_number.to_string();
            let padding_wanted = shared_info.settings.line_number_padding as usize;
            let padding_count = if padding_wanted > number_string.len() {
                padding_wanted - number_string.len()
            } else {
                0
            };
            format!("{}{}┃", " ".repeat(padding_count), number_string)
        } else {
            "".to_owned()
        };

        let arrow = if last { ARROW_LAST_CHAR } else { ARROW_CHAR };
        // Create indentation
        let mut indentation = String::new();
        for item in &self.depth_info {
            if TAB_WIDTH < 2 {
                indentation.push_str(&" ".repeat(TAB_WIDTH - 2));
            }
            if item.indent_line {
                indentation.push_str(&TREE_STYLE.paint(INDENT_CHAR).to_string());
            } else {
                indentation.push(' ');
            }
            indentation.push(' ');
        }

        println!(
            "{}{}{} {}",
            line_number,
            indentation,
            TREE_STYLE.paint(arrow),
            text
        );
    }
}
