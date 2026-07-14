use crate::controllers::index::IndexController;
use clap::Parser;

#[derive(Parser)]
#[command(name = "asq")]
#[command(about = "Quick LLM asker")]
#[command(version)]
pub(crate) struct Cli {
    #[clap(flatten)]
    pub index: IndexController,
}
