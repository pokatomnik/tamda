use clap::Args;
use minus::{Pager, page_all};

use crate::util::{handler::Handler, universal_source::UniversalSource};

#[derive(Args, Clone)]
#[clap(rename_all = "kebab-case")]
pub(crate) struct IndexController {
    /// Source of file: URL or file path. TaMDa reads markdown source from stdin if not specified.
    source: Option<String>,
}

impl IndexController {
    fn handle_source(source: &str) -> anyhow::Result<String> {
        let universal_source = UniversalSource::from(source);
        let data = universal_source.read()?;

        Ok(data)
    }

    fn handle_input() -> anyhow::Result<String> {
        let lines = std::io::stdin()
            .lines()
            .filter_map(|l| l.ok())
            .collect::<Vec<String>>()
            .join("\n");

        Ok(lines)
    }
}

impl Handler for IndexController {
    type Arguments = ();

    type Return = anyhow::Result<()>;

    fn handle(&self, _: Self::Arguments) -> Self::Return {
        let result = match &self.source {
            Some(source) => Self::handle_source(source.as_str())?,
            None => Self::handle_input()?,
        };
        let result = marcli::render(result.as_str(), &Default::default());

        let pager = Pager::new();
        pager.set_text(result)?;
        page_all(pager)?;

        Ok(())
    }
}
