use std::path::PathBuf;

use reqwest::Url;

pub(crate) enum UniversalSource {
    File(PathBuf),
    URL(Url),
}

impl UniversalSource {
    fn get_data_by_url(url: &Url) -> anyhow::Result<String> {
        let response = reqwest::blocking::get(url.clone())?.text()?;
        Ok(response)
    }

    fn get_data_by_path(path: &PathBuf) -> anyhow::Result<String> {
        let result = std::fs::read_to_string(path)?;
        Ok(result)
    }

    pub fn read(&self) -> anyhow::Result<String> {
        match self {
            UniversalSource::File(path_buf) => Self::get_data_by_path(path_buf),
            UniversalSource::URL(url) => Self::get_data_by_url(url),
        }
    }
}

impl From<&str> for UniversalSource {
    fn from(value: &str) -> Self {
        if let Ok(url) = Url::try_from(value) {
            return Self::URL(url);
        }

        return Self::File(PathBuf::from(value));
    }
}
