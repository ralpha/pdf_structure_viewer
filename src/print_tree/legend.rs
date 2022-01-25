use super::get_object_print_info;
use crate::print_tree::TreeDisplaySettings;
use lopdf::{Dictionary, Object, Stream, StringFormat};

lazy_static::lazy_static! {
    static ref DISPLAY_SETTINGS: TreeDisplaySettings = TreeDisplaySettings::default();
}

pub fn print_legend() {
    let table_width = 30;
    println!(
        "┏{} Legend {}┓",
        "━".repeat((table_width - 8) / 2),
        "━".repeat((table_width - 8) / 2)
    );
    print_table_line(table_width, &Object::Null);
    print_table_line(table_width, &Object::Boolean(true));
    print_table_line(table_width, &Object::Integer(0));
    print_table_line(table_width, &Object::Real(0.0));
    print_table_line(table_width, &Object::Name(vec![]));
    print_table_line(table_width, &Object::String(vec![], StringFormat::Literal));
    print_table_line(
        table_width,
        &Object::String(vec![], StringFormat::Hexadecimal),
    );
    print_table_line(table_width, &Object::Array(vec![]));
    print_table_line(table_width, &Object::Dictionary(Dictionary::new()));
    print_table_line(
        table_width,
        &Object::Stream(Stream::new(Dictionary::new(), vec![])),
    );
    print_table_line(table_width, &Object::Reference((0, 0)));
    println!("┗{}┛", "━".repeat(table_width));
}

pub fn print_table_line(table_width: usize, obj: &Object) {
    let obj_print_info = get_object_print_info(obj, &DISPLAY_SETTINGS);
    let styled_text = format!(
        "{:<2} {}",
        obj_print_info.symbol_style.paint(obj_print_info.symbol),
        obj_print_info.type_name
    );
    let plain_text = format!("{:<2} {}", obj_print_info.symbol, obj_print_info.type_name);
    let text_len = plain_text.chars().count();
    println!(
        "┃ {}{}┃",
        styled_text,
        " ".repeat(table_width - text_len - 1)
    );
}
