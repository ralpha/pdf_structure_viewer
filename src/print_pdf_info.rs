use lopdf::{Document, Error};
use yansi::{Paint, Style};

pub fn print_pdf_info(raw_doc: &Document) -> Result<(), Error> {
    let label_style = Style::default();
    let value_style = Style::default().bold();

    println!("--- {} ---", Paint::cyan("PDF Info").bold());
    println!(
        "{}: {}",
        label_style.paint("Version"),
        value_style.paint(raw_doc.version.to_string())
    );
    println!(
        "{}: {}",
        label_style.paint("Trailer"),
        value_style.paint(format!("{:#?}", raw_doc.trailer))
    );
    println!(
        "{}: {}",
        label_style.paint("Reference Table length"),
        value_style.paint(raw_doc.reference_table.entries.len())
    );
    println!(
        "{}: {}",
        label_style.paint("Reference Table size"),
        value_style.paint(raw_doc.reference_table.size)
    );
    println!(
        "{}: {}",
        label_style.paint("Objects amount"),
        value_style.paint(raw_doc.objects.len())
    );
    println!(
        "{}: {}",
        label_style.paint("Max Object Id"),
        value_style.paint(raw_doc.max_id)
    );
    println!(
        "{}: {}",
        label_style.paint("Max Bookmark Id"),
        value_style.paint(raw_doc.max_bookmark_id)
    );
    println!(
        "{}: {}",
        label_style.paint("Bookmark amount"),
        value_style.paint(raw_doc.bookmarks.len())
    );
    println!(
        "{}: {}",
        label_style.paint("Bookmark Table size"),
        value_style.paint(raw_doc.bookmark_table.len())
    );
    Ok(())
}
