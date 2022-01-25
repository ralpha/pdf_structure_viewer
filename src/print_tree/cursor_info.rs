use super::TreeDisplaySettings;
use lopdf::ObjectId;

#[derive(Debug, Default, Clone)]
pub struct CursorInfo {
    pub depth_info: Vec<DepthInfo>,
    pub parent_refs: Vec<ObjectId>,
}

#[derive(Debug, Default, Clone)]
pub struct DepthInfo {
    pub name: Option<String>,
    pub indent_line: bool,
}

impl CursorInfo {
    #[allow(dead_code)]
    pub fn new(depth_info: Vec<DepthInfo>) -> Self {
        CursorInfo {
            depth_info,
            parent_refs: Vec::new(),
        }
    }

    pub fn add_depth(&self, depth_info: DepthInfo) -> Self {
        let mut new_cursor = self.clone();
        new_cursor.depth_info.push(depth_info);
        new_cursor
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
}
