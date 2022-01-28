use super::cursor_info::DepthInfo;
use super::stream_operations::{operation_info, OperationInfoValue};
use super::{cursor_info::TreeCursorInfo, TreeDisplaySettings};
use super::{get_object_print_info, EXPAND_INFO_STYLE, EXTRA_INFO_STYLE, VALUE_STYLE};
use lopdf::content::Operation;
use lopdf::{Error, Object, Stream};

pub fn print_content_stream(
    display_settings: &TreeDisplaySettings,
    stream: &Stream,
    cursor: &TreeCursorInfo,
) -> Result<(), Error> {
    // Check is last in path is "Contents" or some other known names
    let last_path_label = cursor.get_path().pop();
    if last_path_label == Some("Contents".to_owned())
        || last_path_label == Some("N".to_owned())
        || last_path_label == Some("R".to_owned())
        || last_path_label == Some("D".to_owned())
        || last_path_label == Some("AP".to_owned())
        || display_settings.force_stream_decoding
    {
        // Decode stream
        let decoded_stream = stream.decode_content()?;
        for operation in decoded_stream.operations {
            print_operation_string(display_settings, &operation, cursor)?;
        }
    } else {
        cursor.print_subitem(
            EXPAND_INFO_STYLE
                .paint("... (no content stream, force decoding with `force-stream-decoding` flag)")
                .to_string(),
            false,
        );
    }

    Ok(())
}

/// Convert an operation to the correct printing format.
///
/// Each operation has special meanings, this allows to more informed printing.
/// If `stream_enhanced_operations` is `true` this formatting will be enhanced.
/// If `false` the formatter will just print the raw values.
fn print_operation_string(
    display_settings: &TreeDisplaySettings,
    operation: &Operation,
    cursor: &TreeCursorInfo,
) -> Result<(), Error> {
    if display_settings.stream_enhanced_operations {
        print_enhanced_operation(display_settings, operation, cursor)?;
    } else {
        print_basic_operation(display_settings, operation, cursor)?;
    }
    Ok(())
}

fn print_basic_operation(
    display_settings: &TreeDisplaySettings,
    operation: &Operation,
    cursor: &TreeCursorInfo,
) -> Result<(), Error> {
    let operands_string = get_operands_string(display_settings, &operation.operands)?;
    cursor.print_subitem(
        format!("{}({})", operation.operator, operands_string),
        false,
    );
    Ok(())
}

fn get_operands_string(
    display_settings: &TreeDisplaySettings,
    operands: &[Object],
) -> Result<String, Error> {
    let mut results = Vec::new();

    for object in operands {
        match &object {
            Object::Array(list) => {
                let obj_print_info = get_object_print_info(object, display_settings);
                let array_string = get_operands_string(display_settings, list)?;
                results.push(format!(
                    "{}{}{}",
                    obj_print_info.symbol_style.paint("["),
                    array_string,
                    obj_print_info.symbol_style.paint("]"),
                ));
            }
            Object::Dictionary(dict) => {
                let obj_print_info = get_object_print_info(object, display_settings);
                let mut temp_result = Vec::new();
                for (key, value) in dict {
                    temp_result.push(format!(
                        "{}:{}",
                        String::from_utf8_lossy(key),
                        get_operands_string(display_settings, &[value.clone()])?,
                    ));
                }
                results.push(format!(
                    "{}{}{}",
                    obj_print_info.symbol_style.paint("{"),
                    temp_result.join(", "),
                    obj_print_info.symbol_style.paint("}"),
                ));
            }
            Object::Reference(_) => {
                let obj_print_info = get_object_print_info(object, display_settings);
                results.push(format!(
                    "{} {}",
                    obj_print_info.symbol_style.paint(obj_print_info.symbol),
                    VALUE_STYLE.paint(obj_print_info.value),
                ));
            }
            Object::String(..) => {
                let obj_print_info = get_object_print_info(object, display_settings);
                results.push(format!(
                    "{} '{}'",
                    obj_print_info.symbol_style.paint(obj_print_info.symbol),
                    VALUE_STYLE.paint(obj_print_info.value),
                ));
            }
            _ => {
                let obj_print_info = get_object_print_info(object, display_settings);
                results.push(format!(
                    "{} {}",
                    obj_print_info.symbol_style.paint(obj_print_info.symbol),
                    VALUE_STYLE.paint(obj_print_info.value),
                ));
            }
        }
    }

    Ok(results.join(", "))
}

fn print_enhanced_operation(
    display_settings: &TreeDisplaySettings,
    operation: &Operation,
    cursor: &TreeCursorInfo,
) -> Result<(), Error> {
    let operation_info = operation_info(operation, display_settings);

    match operation_info {
        Ok(operation_info) => {
            if display_settings.stream_enhanced_operator_info {
                cursor.print_subitem(
                    format!(
                        "{}: {}",
                        operation_info.operator,
                        EXTRA_INFO_STYLE.paint(operation_info.description)
                    ),
                    false,
                );
            } else {
                cursor.print_subitem(operation_info.operator.to_string(), false);
            }

            let new_cursor = cursor.add_depth(DepthInfo {
                name: Some(operation_info.operator.to_owned()),
                indent_line: true,
            });
            match operation_info.values {
                OperationInfoValue::Arguments(values) => {
                    for (key, value) in values {
                        let obj_print_info = get_object_print_info(&value, display_settings);
                        new_cursor.print_subitem(
                            format!(
                                "{}: {:<2} {}",
                                key,
                                obj_print_info.symbol_style.paint(obj_print_info.symbol),
                                VALUE_STYLE.paint(obj_print_info.value),
                            ),
                            false,
                        );
                    }
                }
                OperationInfoValue::FormattedString(formatted_string) => {
                    new_cursor.print_subitem(formatted_string, false);
                }
            }
        }
        Err(err) => {
            log::warn!("PDF Error: {}", err);
            print_basic_operation(display_settings, operation, cursor)?;
        }
    }

    Ok(())
}
