use super::{get_object_print_info, TreeDisplaySettings, SKIPPED_STYLE, VALUE_STYLE};
use indexmap::{indexmap, IndexMap};
use lopdf::content::Operation;
use lopdf::{Error, Object, StringFormat};

pub struct OperationInfo {
    pub operator: &'static str,
    pub description: &'static str,
    pub values: OperationInfoValue,
}

pub enum OperationInfoValue {
    Arguments(IndexMap<String, Object>),
    FormattedString(String),
}

pub fn operation_info(
    operation: &Operation,
    display_settings: &TreeDisplaySettings,
) -> Result<OperationInfo, Error> {
    let operator = &operation.operator;
    let operands = &operation.operands;

    // For a list of all operations: see p643 (Table A.1) in PDF v1.7 Spec
    let operation_info = match operator.as_str() {
        "b" => {
            check_max_operands(operation, 0);
            OperationInfo {
                operator: "b",
                description: "Close, fill, and stroke path using nonzero winding number rule.",
                values: unknown_values(operands),
            }
        }
        "B" => {
            check_max_operands(operation, 0);
            OperationInfo {
                operator: "B",
                description: "Fill and stroke path using nonzero winding number rule.",
                values: unknown_values(operands),
            }
        }
        "b*" => {
            check_max_operands(operation, 0);
            OperationInfo {
                operator: "b*",
                description: "Close, fill, and stroke path using even-odd rule.",
                values: unknown_values(operands),
            }
        }
        "B*" => {
            check_max_operands(operation, 0);
            OperationInfo {
                operator: "B*",
                description: "Fill and stroke path using even-odd rule.",
                values: unknown_values(operands),
            }
        }
        "BDC" => {
            check_max_operands(operation, 2);
            OperationInfo {
                operator: "BDC",
                description: "(PDF 1.2) Begin marked-content sequence with property list.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "tag".to_owned() => get_operands_value(operation, 0)?,
                    "properties".to_owned() => get_operands_value(operation, 1)?,
                }),
            }
        }
        "BI" => {
            check_max_operands(operation, 0);
            OperationInfo {
                operator: "BI",
                description: "Begin inline image object.",
                values: unknown_values(operands),
            }
        }
        "BMC" => {
            check_max_operands(operation, 1);
            OperationInfo {
                operator: "BMC",
                description: "(PDF 1.2) Begin marked-content sequence.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "tag".to_owned() => get_operands_value(operation, 0)?,
                }),
            }
        }
        "BT" => {
            check_max_operands(operation, 0);
            OperationInfo {
                operator: "BT",
                description: "Begin text object.",
                values: unknown_values(operands),
            }
        }
        "BX" => {
            check_max_operands(operation, 0);
            OperationInfo {
                operator: "BX",
                description: "(PDF 1.1) Begin compatibility section.",
                values: unknown_values(operands),
            }
        }
        "c" => {
            check_max_operands(operation, 6);
            OperationInfo {
                operator: "c",
                description: "Append curved segment to path (three control points).",
                values: OperationInfoValue::Arguments(indexmap! {
                    "x1".to_owned() => get_operands_value(operation, 0)?,
                    "y1".to_owned() => get_operands_value(operation, 1)?,
                    "x2".to_owned() => get_operands_value(operation, 2)?,
                    "y2".to_owned() => get_operands_value(operation, 3)?,
                    "x3".to_owned() => get_operands_value(operation, 4)?,
                    "y3".to_owned() => get_operands_value(operation, 5)?,
                }),
            }
        }
        "cm" => {
            check_max_operands(operation, 6);
            OperationInfo {
                operator: "cm",
                description:
                    "Concatenate matrix to current transformation matrix. `[a b 0; c d 0; e f 1]`",
                values: OperationInfoValue::Arguments(indexmap! {
                    "a".to_owned() => get_operands_value(operation, 0)?,
                    "b".to_owned() => get_operands_value(operation, 1)?,
                    "c".to_owned() => get_operands_value(operation, 2)?,
                    "d".to_owned() => get_operands_value(operation, 3)?,
                    "e".to_owned() => get_operands_value(operation, 4)?,
                    "f".to_owned() => get_operands_value(operation, 5)?,
                }),
            }
        }
        "CS" => {
            check_max_operands(operation, 1);
            OperationInfo {
                operator: "CS",
                description: "(PDF 1.1) Set color space for stroking operations.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "name".to_owned() => get_operands_value(operation, 0)?,
                }),
            }
        }
        "cs" => {
            check_max_operands(operation, 1);
            OperationInfo {
                operator: "cs",
                description: "(PDF 1.1) Set color space for nonstroking operations.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "name".to_owned() => get_operands_value(operation, 0)?,
                }),
            }
        }
        "d" => {
            check_max_operands(operation, 2);
            OperationInfo {
                operator: "d",
                description: "Set line dash pattern.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "dashArray".to_owned() => get_operands_value(operation, 0)?,
                    "dashPhase".to_owned() => get_operands_value(operation, 1)?,
                }),
            }
        }
        "d0" => {
            check_max_operands(operation, 2);
            OperationInfo {
                operator: "d0",
                description: "Set glyph width in Type 3 font.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "wx".to_owned() => get_operands_value(operation, 0)?,
                    "wy".to_owned() => get_operands_value(operation, 1)?,
                }),
            }
        }
        "d1" => {
            check_max_operands(operation, 6);
            OperationInfo {
                operator: "d1",
                description: "Set glyph width and bounding box in Type 3 font.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "w_x".to_owned() => get_operands_value(operation, 0)?,
                    "w_y".to_owned() => get_operands_value(operation, 1)?,
                    "ll_y".to_owned() => get_operands_value(operation, 2)?,
                    "ll_x".to_owned() => get_operands_value(operation, 3)?,
                    "ur_x".to_owned() => get_operands_value(operation, 4)?,
                    "ur_y".to_owned() => get_operands_value(operation, 5)?,
                }),
            }
        }
        "Do" => {
            check_max_operands(operation, 1);
            OperationInfo {
                operator: "Do",
                description: "Invoke named XObject.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "name".to_owned() => get_operands_value(operation, 0)?,
                }),
            }
        }
        "DP" => {
            check_max_operands(operation, 2);
            OperationInfo {
                operator: "DP",
                description: "(PDF 1.2) Define marked-content point with property list.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "tag".to_owned() => get_operands_value(operation, 0)?,
                    "properties".to_owned() => get_operands_value(operation, 1)?,
                }),
            }
        }
        "EI" => {
            check_max_operands(operation, 0);
            OperationInfo {
                operator: "EI",
                description: "End inline image object.",
                values: unknown_values(operands),
            }
        }
        "EMC" => {
            check_max_operands(operation, 0);
            OperationInfo {
                operator: "EMC",
                description: "(PDF 1.2) End marked-content sequence.",
                values: unknown_values(operands),
            }
        }
        "ET" => {
            check_max_operands(operation, 0);
            OperationInfo {
                operator: "ET",
                description: "End text object.",
                values: unknown_values(operands),
            }
        }
        "EX" => {
            check_max_operands(operation, 0);
            OperationInfo {
                operator: "EX",
                description: "(PDF 1.1) End compatibility section.",
                values: unknown_values(operands),
            }
        }
        "f" => {
            check_max_operands(operation, 0);
            OperationInfo {
                operator: "f",
                description: "Fill path using nonzero winding number rule.",
                values: unknown_values(operands),
            }
        }
        "F" => {
            check_max_operands(operation, 0);
            OperationInfo {
                operator: "F",
                description: "Fill path using nonzero winding number rule (obsolete).",
                values: unknown_values(operands),
            }
        }
        "f*" => {
            check_max_operands(operation, 0);
            OperationInfo {
                operator: "f*",
                description: "Fill path using even-odd rule.",
                values: unknown_values(operands),
            }
        }
        "G" => {
            check_max_operands(operation, 1);
            OperationInfo {
                operator: "G",
                description: "Set gray level for stroking operations. (0=black, 1=while)",
                values: OperationInfoValue::Arguments(indexmap! {
                    "gray".to_owned() => get_operands_value(operation, 0)?,
                }),
            }
        }
        "g" => {
            check_max_operands(operation, 1);
            OperationInfo {
                operator: "g",
                description: "Set gray level for nonstroking operations. (0=black, 1=while)",
                values: OperationInfoValue::Arguments(indexmap! {
                    "gray".to_owned() => get_operands_value(operation, 0)?,
                }),
            }
        }
        "gs" => {
            check_max_operands(operation, 1);
            OperationInfo {
                operator: "gs",
                description: "(PDF 1.2) Set parameters from graphics state parameter dictionary.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "dictName".to_owned() => get_operands_value(operation, 0)?,
                }),
            }
        }
        "h" => {
            check_max_operands(operation, 0);
            OperationInfo {
                operator: "h",
                description: "Close subpath.",
                values: unknown_values(operands),
            }
        }
        "i" => {
            check_max_operands(operation, 1);
            OperationInfo {
                operator: "i",
                description: "Set flatness tolerance.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "flatness".to_owned() => get_operands_value(operation, 0)?,
                }),
            }
        }
        "ID" => {
            check_max_operands(operation, 0);
            OperationInfo {
                operator: "ID",
                description: "Begin inline image data.",
                values: unknown_values(operands),
            }
        }
        "j" => {
            check_max_operands(operation, 1);
            OperationInfo {
                operator: "j",
                description: "Set line join style.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "lineJoin".to_owned() => get_operands_value(operation, 0)?,
                }),
            }
        }
        "J" => {
            check_max_operands(operation, 1);
            OperationInfo {
                operator: "J",
                description: "Set line cap style.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "lineCap".to_owned() => get_operands_value(operation, 0)?,
                }),
            }
        }
        "K" => {
            check_max_operands(operation, 4);
            OperationInfo {
                operator: "K",
                description: "Set CMYK color for stroking operations.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "cyan".to_owned() => get_operands_value(operation, 0)?,
                    "magenta".to_owned() => get_operands_value(operation, 1)?,
                    "yellow".to_owned() => get_operands_value(operation, 2)?,
                    "key/black".to_owned() => get_operands_value(operation, 3)?,
                }),
            }
        }
        "k" => {
            check_max_operands(operation, 4);
            OperationInfo {
                operator: "k",
                description: "Set CMYK color for nonstroking operations.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "cyan".to_owned() => get_operands_value(operation, 0)?,
                    "magenta".to_owned() => get_operands_value(operation, 1)?,
                    "yellow".to_owned() => get_operands_value(operation, 2)?,
                    "key/black".to_owned() => get_operands_value(operation, 3)?,
                }),
            }
        }
        "l" => {
            check_max_operands(operation, 2);
            OperationInfo {
                operator: "l",
                description: "Append straight line segment to path.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "x".to_owned() => get_operands_value(operation, 0)?,
                    "y".to_owned() => get_operands_value(operation, 1)?,
                }),
            }
        }
        "m" => {
            check_max_operands(operation, 2);
            OperationInfo {
                operator: "m",
                description: "Begin new subpath.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "x".to_owned() => get_operands_value(operation, 0)?,
                    "y".to_owned() => get_operands_value(operation, 1)?,
                }),
            }
        }
        "M" => {
            check_max_operands(operation, 1);
            OperationInfo {
                operator: "M",
                description: "Set miter limit.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "miterLimit".to_owned() => get_operands_value(operation, 0)?,
                }),
            }
        }
        "MP" => {
            check_max_operands(operation, 1);
            OperationInfo {
                operator: "MP",
                description: "(PDF 1.2) Define marked-content point.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "tag".to_owned() => get_operands_value(operation, 0)?,
                }),
            }
        }
        "n" => {
            check_max_operands(operation, 0);
            OperationInfo {
                operator: "n",
                description: "End path without filling or stroking.",
                values: unknown_values(operands),
            }
        }
        "q" => {
            check_max_operands(operation, 0);
            OperationInfo {
                operator: "q",
                description: "Save graphics state.",
                values: unknown_values(operands),
            }
        }
        "Q" => {
            check_max_operands(operation, 0);
            OperationInfo {
                operator: "Q",
                description: "Restore graphics state.",
                values: unknown_values(operands),
            }
        }
        "re" => {
            check_max_operands(operation, 4);
            OperationInfo {
                operator: "re",
                description: "Append rectangle to path.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "x".to_owned() => get_operands_value(operation, 0)?,
                    "y".to_owned() => get_operands_value(operation, 1)?,
                    "width".to_owned() => get_operands_value(operation, 2)?,
                    "height".to_owned() => get_operands_value(operation, 3)?,
                }),
            }
        }
        "RG" => {
            check_max_operands(operation, 3);
            OperationInfo {
                operator: "RG",
                description: "Set RGB color for stroking operations.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "red".to_owned() => get_operands_value(operation, 0)?,
                    "green".to_owned() => get_operands_value(operation, 1)?,
                    "blue".to_owned() => get_operands_value(operation, 2)?,
                }),
            }
        }
        "rg" => {
            check_max_operands(operation, 3);
            OperationInfo {
                operator: "rg",
                description: "Set RGB color for nonstroking operations.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "red".to_owned() => get_operands_value(operation, 0)?,
                    "green".to_owned() => get_operands_value(operation, 1)?,
                    "blue".to_owned() => get_operands_value(operation, 2)?,
                }),
            }
        }
        "ri" => {
            check_max_operands(operation, 1);
            OperationInfo {
                operator: "ri",
                description: "Set color rendering intent.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "intent".to_owned() => get_operands_value(operation, 0)?,
                }),
            }
        }
        "s" => {
            check_max_operands(operation, 0);
            OperationInfo {
                operator: "s",
                description: "Close and stroke path.",
                values: unknown_values(operands),
            }
        }
        "S" => {
            check_max_operands(operation, 0);
            OperationInfo {
                operator: "S",
                description: "Stroke path.",
                values: unknown_values(operands),
            }
        }
        "SC" => {
            // No Limit
            OperationInfo {
                operator: "SC",
                description: "(PDF 1.1) Set color for stroking operations.",
                values: infinite_values(operands, "c"),
            }
        }
        "sc" => {
            // No Limit
            OperationInfo {
                operator: "sc",
                description: "(PDF 1.1) Set color for nonstroking operations.",
                values: infinite_values(operands, "c"),
            }
        }
        "SCN" => {
            // No Limit
            OperationInfo {
                operator: "SCN",
                description: "(PDF 1.2) Set color for stroking operations (ICCBased and special colour spaces).",
                values: infinite_values(operands, "c"),
            }
        }
        "scn" => {
            // No Limit
            OperationInfo {
                operator: "scn",
                description: "(PDF 1.2) Set color for nonstroking operations (ICCBased and special colour spaces).",
                values: infinite_values(operands, "c"),
            }
        }
        "sh" => {
            check_max_operands(operation, 1);
            OperationInfo {
                operator: "sh",
                description: "(PDF 1.3) Paint area defined by shading pattern.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "name".to_owned() => get_operands_value(operation, 0)?,
                }),
            }
        }
        "T*" => {
            check_max_operands(operation, 0);
            OperationInfo {
                operator: "T*",
                description: "Move to start of next text line.",
                values: unknown_values(operands),
            }
        }
        "Tc" => {
            check_max_operands(operation, 1);
            OperationInfo {
                operator: "Tc",
                description: "Set character spacing.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "charSpace".to_owned() => get_operands_value(operation, 0)?,
                }),
            }
        }
        "Td" => {
            check_max_operands(operation, 2);
            OperationInfo {
                operator: "Td",
                description: "Move text position.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "Tx".to_owned() => get_operands_value(operation, 0)?,
                    "Ty".to_owned() => get_operands_value(operation, 1)?,
                }),
            }
        }
        "TD" => {
            check_max_operands(operation, 2);
            OperationInfo {
                operator: "TD",
                description: "Move text position and set leading.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "Tx".to_owned() => get_operands_value(operation, 0)?,
                    "Ty".to_owned() => get_operands_value(operation, 1)?,
                }),
            }
        }
        "Tf" => {
            check_max_operands(operation, 2);
            OperationInfo {
                operator: "Tf",
                description: "Set text font and size.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "font".to_owned() => get_operands_value(operation, 0)?,
                    "size".to_owned() => get_operands_value(operation, 1)?,
                }),
            }
        }
        "Tj" => {
            check_max_operands(operation, 1);
            OperationInfo {
                operator: "Tj",
                description: "Show text.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "string".to_owned() => get_operands_value(operation, 0)?,
                }),
            }
        }
        "TJ" => {
            let mut formatted_string = String::new();
            check_max_operands(operation, 1);
            for item in get_operands_value(operation, 0)?.as_array()? {
                match item {
                    Object::String(string_value, string_format) => match string_format {
                        StringFormat::Literal => formatted_string
                            .push_str(&String::from_utf8_lossy(string_value).to_string()),
                        StringFormat::Hexadecimal => {
                            let obj_print_info = get_object_print_info(item, display_settings);
                            formatted_string.push_str(&format!(
                                "{}",
                                obj_print_info.symbol_style.paint(obj_print_info.value)
                            ))
                        }
                    },
                    Object::Integer(int_value) => {
                        if int_value.is_negative() {
                            formatted_string.push(' ');
                        }
                    }
                    _ => log::warn!("Only Strings and Integers expected in `TJ` operator."),
                }
            }
            OperationInfo {
                operator: "TJ",
                description: "Show text, allowing individual glyph positioning",
                values: OperationInfoValue::FormattedString(format!(
                    "'{}' {}",
                    VALUE_STYLE.paint(formatted_string),
                    SKIPPED_STYLE.paint("(abbreviated)")
                )),
            }
        }
        "TL" => {
            check_max_operands(operation, 1);
            OperationInfo {
                operator: "TL",
                description: "Set text leading.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "leading".to_owned() => get_operands_value(operation, 0)?,
                }),
            }
        }
        "Tm" => {
            check_max_operands(operation, 6);
            OperationInfo {
                operator: "Tm",
                description: "Set text matrix and text line matrix. `[a b 0; c d 0; e f 1]`",
                values: OperationInfoValue::Arguments(indexmap! {
                    "a".to_owned() => get_operands_value(operation, 0)?,
                    "b".to_owned() => get_operands_value(operation, 1)?,
                    "c".to_owned() => get_operands_value(operation, 2)?,
                    "d".to_owned() => get_operands_value(operation, 3)?,
                    "e".to_owned() => get_operands_value(operation, 4)?,
                    "f".to_owned() => get_operands_value(operation, 5)?,
                }),
            }
        }
        "Tr" => {
            check_max_operands(operation, 1);
            OperationInfo {
                operator: "Tr",
                description: "Set text rendering mode.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "render".to_owned() => get_operands_value(operation, 0)?,
                }),
            }
        }
        "Ts" => {
            check_max_operands(operation, 1);
            OperationInfo {
                operator: "Ts",
                description: "Set text rise.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "rise".to_owned() => get_operands_value(operation, 0)?,
                }),
            }
        }
        "Tw" => {
            check_max_operands(operation, 1);
            OperationInfo {
                operator: "Tw",
                description: "Set word spacing.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "wordSpace".to_owned() => get_operands_value(operation, 0)?,
                }),
            }
        }
        "Tz" => {
            check_max_operands(operation, 1);
            OperationInfo {
                operator: "Tz",
                description: "Set horizontal text scaling.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "scale".to_owned() => get_operands_value(operation, 0)?,
                }),
            }
        }
        "v" => {
            check_max_operands(operation, 4);
            OperationInfo {
                operator: "v",
                description: "Append curved segment to path (initial point replicated).",
                values: OperationInfoValue::Arguments(indexmap! {
                    "x2".to_owned() => get_operands_value(operation, 0)?,
                    "y2".to_owned() => get_operands_value(operation, 1)?,
                    "x3".to_owned() => get_operands_value(operation, 2)?,
                    "y3".to_owned() => get_operands_value(operation, 3)?,
                }),
            }
        }
        "w" => {
            check_max_operands(operation, 1);
            OperationInfo {
                operator: "w",
                description: "Set line width.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "lineWidth".to_owned() => get_operands_value(operation, 0)?,
                }),
            }
        }
        "W" => {
            check_max_operands(operation, 0);
            OperationInfo {
                operator: "W",
                description: "Set clipping path using nonzero winding number rule.",
                values: unknown_values(operands),
            }
        }
        "W*" => {
            check_max_operands(operation, 0);
            OperationInfo {
                operator: "W*",
                description: "Set clipping path using even-odd rule.",
                values: unknown_values(operands),
            }
        }
        "y" => {
            check_max_operands(operation, 4);
            OperationInfo {
                operator: "y",
                description: "Append curved segment to path (final point replicated).",
                values: OperationInfoValue::Arguments(indexmap! {
                    "x1".to_owned() => get_operands_value(operation, 0)?,
                    "y1".to_owned() => get_operands_value(operation, 1)?,
                    "x3".to_owned() => get_operands_value(operation, 2)?,
                    "y3".to_owned() => get_operands_value(operation, 3)?,
                }),
            }
        }
        "'" => {
            check_max_operands(operation, 1);
            OperationInfo {
                operator: "'",
                description: "Move to next line and show text.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "string".to_owned() => get_operands_value(operation, 0)?,
                }),
            }
        }
        "\"" => {
            check_max_operands(operation, 3);
            OperationInfo {
                operator: "\"",
                description: "Set word and character spacing, move to next line, and show text.",
                values: OperationInfoValue::Arguments(indexmap! {
                    "a_word".to_owned() => get_operands_value(operation, 0)?,
                    "a_char".to_owned() => get_operands_value(operation, 1)?,
                    "string".to_owned() => get_operands_value(operation, 2)?,
                }),
            }
        }
        unknown => return Err(Error::Syntax(format!("Operator {} is unknown", unknown))),
    };

    Ok(operation_info)
}

fn get_operands_value(operation: &Operation, index: usize) -> Result<Object, Error> {
    operation.operands.get(index).cloned().ok_or_else(|| {
        Error::Syntax(format!(
            "Value {} for operation {} is missing.",
            operation.operator, index,
        ))
    })
}

fn check_max_operands(operation: &Operation, max_len: usize) {
    if operation.operands.len() > max_len {
        log::warn!(
            "`{}` operation does not support more then {} values.",
            operation.operator,
            max_len
        );
    }
}

fn unknown_values(values: &[Object]) -> OperationInfoValue {
    infinite_values(values, "Unknown_")
}

fn infinite_values(values: &[Object], prefix: &str) -> OperationInfoValue {
    let mut result = IndexMap::new();
    for (index, value) in values.iter().enumerate() {
        result.insert(format!("{}{}", prefix, index), value.clone());
    }
    OperationInfoValue::Arguments(result)
}
