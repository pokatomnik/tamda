use clap::Args;
use minus::{Pager, page_all};

use crate::util::{handler::Handler, universal_source::UniversalSource};

#[derive(Args, Clone)]
#[clap(rename_all = "kebab-case")]
pub(crate) struct IndexController {
    /// Source of file: URL or file path
    source: String,
}

impl Handler for IndexController {
    type Arguments = ();

    type Return = anyhow::Result<()>;

    fn handle(&self, _: Self::Arguments) -> Self::Return {
        let universal_source = UniversalSource::from(self.source.as_str());
        let data = universal_source.read()?;
        let result = marcli::render(data.as_str(), &Default::default());

        let pager = Pager::new();
        pager.set_text(result)?;
        page_all(pager)?;

        Ok(())
    }
}
