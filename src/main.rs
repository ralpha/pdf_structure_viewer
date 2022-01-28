#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod print_pdf_info;
mod print_tree;
mod simple_logger;

use log::LevelFilter;
use lopdf::Document;
use print_tree::{TreeCursorSettings, TreeDisplaySettings};
use std::{
    io::{Error, ErrorKind},
    path::PathBuf,
    str::FromStr,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "pdf_structure_viewer",
    about = "Inspect how the PDF's structure looks."
)]
struct Opts {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Verbose mode (-v, -vv)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, Clone, StructOpt)]
enum Command {
    /// Print general info about the PDF.
    Info,
    /// Print the structure of the PDF in a tree structure.
    Tree {
        /// How deep the tree should be printed.
        ///
        /// Default: 20
        #[structopt(long)]
        max_depth: Option<usize>,

        /// Print tree, but only expend from this node.
        ///
        /// Each item should be separated by a dot (`.`)
        /// Example: `Root.Pages.Kids`
        #[structopt(short, long)]
        expand: Option<String>,

        /// Add type names after the property name for more info.
        ///
        /// Printing the type names is disabled by default to reduce clutter.
        /// If the `hide-legend` is not present a legend printed on top of the output for reference.
        #[structopt(long)]
        display_type_names: bool,

        /// Limit the amount of items printed in an array.
        ///
        /// Default: `5`.
        /// Minimum value is `2`.
        /// Using a value of `0` will not limit the amount of items printed.
        #[structopt(long)]
        array_display_limit: Option<usize>,

        /// Limit the amount of bytes printed in an hexadecimal string.
        ///
        /// Default: `16`.
        /// Minimum value is `2`.
        /// Using a value of `0` will not limit the amount of bytes printed.
        #[structopt(long)]
        hex_display_limit: Option<usize>,

        /// Continue expanding the tree after a `Font` items is found.
        ///
        /// Printing font data is disabled by default to reduce clutter.
        #[structopt(long)]
        display_font: bool,

        /// Continue expanding the tree after a parent reference is found.
        ///
        /// Printing parent data is disabled by default to reduce clutter.
        #[structopt(long)]
        display_parent: bool,

        /// Do not print the legend on top of the output.
        #[structopt(long)]
        hide_legend: bool,

        /// When added streams will be displayed.
        ///
        /// Options:
        /// `no_display`|`no`: (default) Do not display streams,
        /// `hex`: Print stream as hexadecimal array,
        /// `tree`: (TODO) Print the stream like other objects in the tree.
        #[structopt(long)]
        display_stream: Option<StreamDisplay>,

        /// Display stream with non-enhanced operation decoding.
        ///
        /// Prints stream with no simplified fields. Just print exact internal structure.
        #[structopt(long)]
        stream_raw_operations: bool,

        /// Display stream with enhanced operation and info about each operation.
        ///
        /// Requires `stream_raw_operations` not to be enabled.
        ///
        /// Prints helpful information about each operation in the stream.
        #[structopt(long)]
        stream_enhanced_operator_info: bool,

        /// Force the decoding of streams even if no content stream is expected.
        ///
        /// This might display incorrect results.
        #[structopt(long)]
        force_stream_decoding: bool,

        /// Print line numbers.
        #[structopt(long)]
        print_line_numbers: bool,

        /// The minimum amount of character the line will be padded to.
        ///
        /// Default is 4, so `   1` until `9999`.
        ///
        /// When the line number exceeds the padding width the number will just extend the margin.
        #[structopt(long)]
        line_number_padding_width: Option<u8>,
    },
    /// Print the internal structure of the PDF.
    /// This is similar to how the PDF is stored in the file.
    Structure,
}

#[derive(Debug, Clone, StructOpt, PartialEq)]
pub enum StreamDisplay {
    NoDisplay,
    Hex,
    Tree,
}

impl FromStr for StreamDisplay {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lowercase_s = s.to_lowercase();

        match lowercase_s.as_ref() {
            "no" | "no_display" => Ok(Self::NoDisplay),
            "hex" => Ok(Self::Hex),
            "tree" => Ok(Self::Tree),
            _ => Err("Unknown format.".to_owned()),
        }
    }
}

impl Default for StreamDisplay {
    fn default() -> Self {
        StreamDisplay::NoDisplay
    }
}

fn main() -> Result<(), Error> {
    // Get command line arguments
    let opts = Opts::from_args();
    // Get log settings
    initialize_logger(&opts);

    let file_name = opts
        .input
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| "<no_file_name>".to_owned());

    let mut raw_doc = match Document::load(opts.input) {
        Ok(doc) => doc,
        Err(lopdf::Error::IO(err)) => {
            log::error!("IO Error while reading file: {}", err);
            return Err(err);
        }
        Err(err) => {
            log::error!("Error while loading file: {}", err);
            return Err(Error::new(ErrorKind::InvalidData, err));
        }
    };

    match opts.cmd {
        Command::Info => {
            print_pdf_info::print_pdf_info(&raw_doc).unwrap();
        }
        Command::Tree {
            max_depth,
            expand,
            display_type_names,
            array_display_limit,
            hex_display_limit,
            display_stream,
            display_font,
            display_parent,
            hide_legend,
            stream_raw_operations,
            stream_enhanced_operator_info,
            force_stream_decoding,
            print_line_numbers,
            line_number_padding_width,
        } => {
            // Tree display settings
            let default_tree_settings = TreeDisplaySettings::default();
            let tree_display_settings = TreeDisplaySettings {
                max_depth: max_depth.unwrap_or(default_tree_settings.max_depth),
                expand: expand.map(|path| path.split('.').map(|s| s.to_owned()).collect()),
                display_type_names,
                array_display_limit: match array_display_limit {
                    Some(0) => None,
                    Some(x) => Some(x),
                    None => default_tree_settings.array_display_limit,
                },
                hex_display_limit: match hex_display_limit {
                    Some(0) => None,
                    Some(x) => Some(x),
                    None => default_tree_settings.hex_display_limit,
                },
                display_stream: display_stream.unwrap_or(default_tree_settings.display_stream),
                display_font,
                display_parent,
                display_legend: !hide_legend,
                stream_enhanced_operations: !stream_raw_operations,
                stream_enhanced_operator_info,
                force_stream_decoding,
            };
            // Tree cursor settings
            let default_cursor_settings = TreeCursorSettings::default();
            let tree_cursor_settings = TreeCursorSettings {
                print_line_numbers,
                line_number_padding: line_number_padding_width
                    .unwrap_or(default_cursor_settings.line_number_padding),
            };

            // Decode streams as this will be needed.
            raw_doc.decompress();
            if tree_display_settings.display_stream != StreamDisplay::NoDisplay {}
            print_tree::print_pdf_tree(
                &tree_display_settings,
                &tree_cursor_settings,
                &raw_doc,
                file_name,
            )
            .unwrap();
        }
        Command::Structure => {
            println!("{:#?}", raw_doc);
        }
    }
    Ok(())
}

/// Setup logger. This will select where to print the log message and how many.
fn initialize_logger(opts: &Opts) {
    let log_filter: LevelFilter = if opts.debug {
        if opts.verbose >= 2 {
            LevelFilter::Trace
        } else {
            LevelFilter::Debug
        }
    } else {
        match opts.verbose {
            0 => LevelFilter::Info,
            1 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        }
    };
    // Setup logger and log level
    log::set_logger(&simple_logger::LOGGER).unwrap();
    log::set_max_level(log_filter);
}
