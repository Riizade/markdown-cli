use crate::core::render::render_to_html;
use std::fs;
use std::path::Path;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "CLI for generating HTML from Markdown files")]
#[command(author, version, long_about = None)]
struct Args {
    /// the markdown file used as input
    #[arg(short, long, value_name = "MARKDOWN")]
    markdown: String,

    /// the name of the output file
    #[arg(short, long, value_name = "FILENAME")]
    output: String,

    /// a CSS file to use for styling the resulting output
    #[arg(short, long, value_name = "CSS")]
    css: Option<String>,

    /// propagates embedded HTML to the output without escaping it; do NOT use this for untrusted input
    #[arg(short, long, default_value_t = false, default_missing_value = "true")]
    allow_embedded_html: bool,
}

pub fn cli_main() {
    let args = Args::parse();

    let markdown_text = fs::read_to_string(Path::new(&args.markdown)).unwrap();
    let css_text = args
        .css
        .map(|filename| fs::read_to_string(Path::new(&filename)).unwrap());

    let rendered_text = render_to_html(markdown_text, css_text, args.allow_embedded_html);

    fs::write(Path::new(&args.output), rendered_text).unwrap();
}
