use crate::controllers::index::IndexController;
use clap::Parser;

/// Terminal Markdown viewer
///
/// TaMDa fetches Markdown content from a file, URL, or stdin,
/// renders it to ANSI-formatted output using marcli, and displays it
/// in an interactive pager (minus) with search support.
#[derive(Parser)]
#[command(name = "tamda")]
#[command(author = "Tamda Contributors")]
#[command(version)]
#[command(after_long_help = "\
Examples:
  tamda README.md              Render a local file
  tamda https://example.com/doc.md    Render a URL
  cat README.md | tamda         Read from stdin
")]
pub(crate) struct Cli {
    #[clap(flatten)]
    pub index: IndexController,
}
