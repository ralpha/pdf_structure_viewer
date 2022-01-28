mod cursor_info;
mod legend;
mod pdf_content_stream;
mod pdf_objects;
mod stream_operations;
mod tree_display_settings;

pub use cursor_info::TreeCursorSettings;
use cursor_info::{DepthInfo, TreeCursorInfo};
use legend::print_legend;
use lopdf::{Dictionary, Document, Error, Object};
pub use pdf_objects::{get_object_print_info, ObjectPrintInfo};
pub use tree_display_settings::TreeDisplaySettings;
use yansi::{Color, Paint, Style};

lazy_static::lazy_static! {
    pub(self) static ref TREE_STYLE: Style = Style::new(Color::Cyan).dimmed();
    pub(self) static ref HELPER_CHARS_STYLE: Style = Style::new(Color::Cyan);
    pub(self) static ref TYPE_STYLE: Style = Style::new(Color::Default).dimmed().italic();
    pub(self) static ref VALUE_STYLE: Style = Style::new(Color::Default).bold();
    pub(self) static ref EXPAND_INFO_STYLE: Style = Style::new(Color::Default).dimmed().italic();
    pub(self) static ref EXTRA_INFO_STYLE: Style = Style::new(Color::Default).italic();
    pub(self) static ref SKIPPED_STYLE: Style = Style::new(Color::Blue).italic();
    pub(self) static ref ERROR_STYLE: Style = Style::new(Color::Red).bold();
}

pub fn print_pdf_tree(
    display_settings: &TreeDisplaySettings,
    tree_cursor_settings: &TreeCursorSettings,
    raw_doc: &Document,
    file_name: String,
) -> Result<(), Error> {
    let trailer = &raw_doc.trailer;
    let cursor = TreeCursorInfo::new(tree_cursor_settings);

    if display_settings.display_legend {
        print_legend();
    }

    println!("{}", Paint::default(file_name).bold());
    print_pdf_dictionary(display_settings, trailer, raw_doc, &cursor)?;
    Ok(())
}

pub fn get_pdf_object_info(
    display_settings: &TreeDisplaySettings,
    label: Option<String>,
    obj: &Object,
) -> Result<String, Error> {
    let obj_print_info = get_object_print_info(obj, display_settings);

    let type_name_styled = if display_settings.display_type_names {
        format!(
            "{}{}",
            HELPER_CHARS_STYLE.paint(":"),
            TYPE_STYLE.paint(obj_print_info.type_name)
        )
    } else {
        "".to_owned()
    };
    if let Some(label) = label {
        if !obj_print_info.value.is_empty() {
            // Print with values
            Ok(format!(
                "{:<2} {}{} {} {} {}",
                obj_print_info.symbol_style.paint(obj_print_info.symbol),
                label,
                type_name_styled,
                HELPER_CHARS_STYLE.paint("="),
                VALUE_STYLE.paint(obj_print_info.value),
                EXTRA_INFO_STYLE.paint(obj_print_info.extra_info.unwrap_or_default())
            ))
        } else {
            // Print without values
            Ok(format!(
                "{:<2} {}{} {}",
                obj_print_info.symbol_style.paint(obj_print_info.symbol),
                label,
                type_name_styled,
                EXTRA_INFO_STYLE.paint(obj_print_info.extra_info.unwrap_or_default())
            ))
        }
    } else if !obj_print_info.value.is_empty() {
        Ok(format!(
            "{:<2} {} {}",
            obj_print_info.symbol_style.paint(obj_print_info.symbol),
            VALUE_STYLE.paint(obj_print_info.value),
            EXTRA_INFO_STYLE.paint(obj_print_info.extra_info.unwrap_or_default())
        ))
    } else {
        Ok(format!(
            "{:<2} {} {}",
            obj_print_info.symbol_style.paint(obj_print_info.symbol),
            type_name_styled,
            EXTRA_INFO_STYLE.paint(obj_print_info.extra_info.unwrap_or_default())
        ))
    }
}

pub fn print_pdf_object_content(
    display_settings: &TreeDisplaySettings,
    obj: &Object,
    raw_doc: &Document,
    cursor: &TreeCursorInfo,
) -> Result<(), Error> {
    match obj {
        Object::Null => {}
        Object::Boolean(_) => {}
        Object::Integer(_) => {}
        Object::Real(_) => {}
        Object::Name(_) => {}
        Object::String(_, _) => {}
        Object::Array(array_value) => {
            let array_count = array_value.len();
            for (index, item) in array_value.iter().enumerate() {
                if let Some(display_limit) = display_settings.array_display_limit {
                    if index < display_limit.max(2) - 1 || index == array_count - 1 {
                        // print first x items || print last item
                    } else if index == array_count - 2 {
                        // print `...`
                        let skipped_items = array_count - display_limit.max(2);
                        cursor.print_subitem(
                            SKIPPED_STYLE
                                .paint(format!("...skipped {} items...", skipped_items))
                                .to_string(),
                            false,
                        );
                        continue;
                    } else {
                        // print nothing (skipped)
                        continue;
                    }
                }

                let is_last = index + 1 == array_count;
                let new_cursor = cursor.add_depth(DepthInfo {
                    name: None,
                    indent_line: !is_last,
                });
                cursor.print_subitem(get_pdf_object_info(display_settings, None, item)?, is_last);
                print_pdf_object_content(display_settings, item, raw_doc, &new_cursor)?;
            }
        }
        Object::Dictionary(dict_value) => {
            // Do not use new cursor here.
            print_pdf_dictionary(display_settings, dict_value, raw_doc, cursor)?;
        }
        Object::Stream(stream_value) => {
            pdf_content_stream::print_content_stream(display_settings, stream_value, cursor)?;
        }
        Object::Reference(object_id) => {
            let mut new_cursor = cursor.add_depth(DepthInfo {
                name: None,
                indent_line: false,
            });
            let ref_obj = match raw_doc.objects.get(object_id) {
                Some(ref_obj) => ref_obj,
                None => {
                    cursor.print_subitem(
                        ERROR_STYLE
                            .paint("Error in PDF: Indirect Reference not found.")
                            .to_string(),
                        true,
                    );
                    return Ok(());
                }
            };
            let print_ref_content = if display_settings.display_parent {
                true
            } else {
                // false if: this reference is to its parent.
                // true if: to a different reference.
                !cursor.check_parent_visited(object_id)
            };
            if print_ref_content {
                cursor.print_subitem(get_pdf_object_info(display_settings, None, ref_obj)?, true);
                new_cursor.add_parent_object_id(*object_id);
                print_pdf_object_content(display_settings, ref_obj, raw_doc, &new_cursor)?;
            } else {
                // So this reference is to its parent.
                cursor.print_subitem(
                    EXPAND_INFO_STYLE
                        .paint("... (display with `display-parent` flag)")
                        .to_string(),
                    true,
                );
            }
        }
    }
    Ok(())
}

pub fn print_pdf_dictionary(
    display_settings: &TreeDisplaySettings,
    dict: &Dictionary,
    raw_doc: &Document,
    cursor: &TreeCursorInfo,
) -> Result<(), Error> {
    // Return when we should not go deeper.
    if cursor.get_depth_count() >= display_settings.max_depth {
        if !dict.is_empty() {
            cursor.print_subitem(
                EXPAND_INFO_STYLE
                    .paint("... (reached `max-depth`)")
                    .to_string(),
                true,
            );
        }
        return Ok(());
    }

    // Get next expand item
    let next_expand_label = match cursor.next_expand_label(display_settings) {
        Ok(x) => x,
        Err(_) => {
            log::debug!("Took wrong path in tree somewhere.");
            return Ok(());
        }
    };

    let dict_count = dict.len();
    for (index, (label, obj)) in dict.iter().enumerate() {
        let label = String::from_utf8_lossy(label).to_string();
        // Check if item should be expended.
        let mut pre_expand = false;
        if let Some(expand_label) = &next_expand_label {
            if expand_label != &label {
                // Not one of the items we should expand
                continue;
            }
            pre_expand = true;
        }
        // Create new cursor
        let is_last = index + 1 == dict_count || pre_expand;
        let new_cursor = cursor.add_depth(DepthInfo {
            name: Some(label.clone()),
            indent_line: !is_last,
        });

        cursor.print_subitem(
            get_pdf_object_info(display_settings, Some(label.clone()), obj)?,
            is_last,
        );
        if !display_settings.display_font && &label == "Font" {
            cursor.print_subitem(
                EXPAND_INFO_STYLE
                    .paint("... (display with `display-font` flag)")
                    .to_string(),
                true,
            );
            continue;
        }
        print_pdf_object_content(display_settings, obj, raw_doc, &new_cursor)?;
    }
    Ok(())
}
