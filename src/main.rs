use lopdf::Document;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "pdf_structure_viewer",
    about = "View how a PDF structure looks."
)]
struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    let mut raw_doc = Document::load(opt.input).unwrap();
    raw_doc.decompress();

    println!("{:#?}", raw_doc);
}
