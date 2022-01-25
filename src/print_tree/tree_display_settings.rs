use crate::StreamDisplay;

#[derive(Debug, Clone)]
pub struct TreeDisplaySettings {
    pub max_depth: usize,
    pub expand: Option<Vec<String>>,
    pub display_type_names: bool,
    pub array_display_limit: Option<usize>,
    pub hex_display_limit: Option<usize>,
    pub display_stream: StreamDisplay,
    pub display_legend: bool,
    pub display_font: bool,
    pub display_parent: bool,
}

impl Default for TreeDisplaySettings {
    fn default() -> Self {
        TreeDisplaySettings {
            max_depth: 20,
            expand: None,
            display_type_names: false,
            array_display_limit: Some(5),
            hex_display_limit: Some(16),
            display_stream: StreamDisplay::NoDisplay,
            display_font: false,
            display_parent: false,
            display_legend: true,
        }
    }
}
