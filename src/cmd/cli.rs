use crate::controllers::index::IndexController;
use clap::Parser;

#[derive(Parser)]
#[command(name = "tamda")]
#[command(about = "Terminal Markdown viewer")]
#[command(version)]
pub(crate) struct Cli {
    #[clap(flatten)]
    pub index: IndexController,
}
