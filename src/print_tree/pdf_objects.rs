use super::{TreeDisplaySettings, SKIPPED_STYLE};
use crate::StreamDisplay;
use lopdf::{Object, StringFormat};
use yansi::{Color, Style};

#[derive(Debug, Default, Clone)]
pub struct ObjectPrintInfo {
    pub symbol_style: Style,
    pub symbol: &'static str,
    pub type_name: &'static str,
    pub value: String,
    pub extra_info: Option<String>,
}

pub fn get_object_print_info(
    obj: &Object,
    display_settings: &TreeDisplaySettings,
) -> ObjectPrintInfo {
    match obj {
        Object::Null => ObjectPrintInfo {
            symbol_style: Style::new(Color::Magenta).bold(),
            symbol: "Nu",
            type_name: "Null",
            value: "<null>".to_owned(),
            ..Default::default()
        },
        Object::Boolean(bool_value) => ObjectPrintInfo {
            symbol_style: Style::new(Color::Black).bold(),
            symbol: "b",
            type_name: "Bool",
            value: match bool_value {
                true => "true".to_owned(),
                false => "false".to_owned(),
            },
            ..Default::default()
        },
        Object::Integer(int_value) => ObjectPrintInfo {
            symbol_style: Style::new(Color::Red).bold(),
            symbol: "Z",
            type_name: "Integer_Number",
            value: int_value.to_string(),
            ..Default::default()
        },
        Object::Real(float_value) => ObjectPrintInfo {
            symbol_style: Style::new(Color::Magenta).bold(),
            symbol: "R",
            type_name: "Real_Number",
            value: float_value.to_string(),
            ..Default::default()
        },
        Object::Name(name_value) => ObjectPrintInfo {
            symbol_style: Style::new(Color::Green).bold(),
            symbol: "Nm",
            type_name: "Name",
            value: format!("'{}'", String::from_utf8_lossy(name_value)),
            ..Default::default()
        },
        Object::String(string_value, string_format) => match string_format {
            StringFormat::Literal => ObjectPrintInfo {
                symbol_style: Style::new(Color::Yellow).bold(),
                symbol: "az",
                type_name: "Literal_String",
                value: format!("'{}'", String::from_utf8_lossy(string_value)),
                ..Default::default()
            },
            StringFormat::Hexadecimal => {
                let short_data = if let Some(display_limit) = display_settings.hex_display_limit {
                    if string_value.len() < display_limit {
                        // Shorter, so print all
                        format!("{:02x?}", string_value)
                    } else {
                        // Longer, so make shorter (skip items)
                        let mut temp_string = String::new();
                        let list_count = string_value.len();
                        for (index, item) in string_value.iter().enumerate() {
                            if index < display_limit.max(2) - 1 {
                                // print first x items
                                temp_string.push_str(&format!("{:02x?}, ", item));
                            } else if index == list_count - 1 {
                                // print last item
                                temp_string.push_str(&format!("{:02x?}", item));
                            } else if index == list_count - 2 {
                                // print `...`
                                let skipped_items = list_count - display_limit.max(2);
                                temp_string.push_str(&format!(
                                    "{}, ",
                                    SKIPPED_STYLE
                                        .paint(format!("...skipped {} bytes...", skipped_items)),
                                ));
                                continue;
                            } else {
                                // print nothing (skipped)
                                continue;
                            }
                        }
                        format!("[{}]", temp_string)
                    }
                } else {
                    // So not make shorter
                    format!("{:02x?}", string_value)
                };
                ObjectPrintInfo {
                    symbol_style: Style::new(Color::RGB(255, 165, 0)).bold(),
                    symbol: "0x",
                    type_name: "Hexadecimal_String",
                    value: short_data,
                    ..Default::default()
                }
            }
        },
        Object::Array(array_value) => ObjectPrintInfo {
            symbol_style: Style::new(Color::Blue).bold(),
            symbol: "[]",
            type_name: "Array",
            value: "".to_owned(),
            extra_info: Some(format!("(length: {} values)", array_value.len())),
        },
        Object::Dictionary(_dict_value) => ObjectPrintInfo {
            symbol_style: Style::new(Color::Cyan).bold(),
            symbol: "{}",
            type_name: "Dictionary",
            value: "".to_owned(),
            ..Default::default()
        },
        Object::Stream(stream_value) => ObjectPrintInfo {
            symbol_style: Style::new(Color::Green).bold(),
            symbol: "S",
            type_name: "Stream",
            value: match display_settings.display_stream {
                StreamDisplay::NoDisplay => "".to_owned(),
                StreamDisplay::Hex => format!("{:02x?}", stream_value.content),
                StreamDisplay::Tree => {
                    log::error!("Setting `display-stream` = `Tree` is not implemented yet.");
                    "".to_owned()
                }
            },
            extra_info: Some(format!("(length: {} bytes)", stream_value.content.len())),
        },
        Object::Reference(object_id) => ObjectPrintInfo {
            symbol_style: Style::new(Color::White).dimmed().bold(),
            symbol: "IR",
            type_name: "Indirect_Reference",
            value: format!("({},{})", object_id.0, object_id.1),
            ..Default::default()
        },
    }
}
