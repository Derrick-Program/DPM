use reqwest::Error;
use serde::{Deserialize, Serialize};
use serde_json::to_writer_pretty;
use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageInfo {
    pub package_name: String,
    pub file_name: String,
    pub version: String,
    pub description: String,
    pub hash: String,
}

impl PackageInfo {
    pub fn new(
        package_name: String,
        file_name: String,
        version: String,
        description: String,
        hash: String,
    ) -> PackageInfo {
        PackageInfo {
            package_name,
            file_name,
            version,
            description,
            hash,
        }
    }
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct HashInfo {
//     pub file_name: String,
//     pub hash: String,
// }
// impl HashInfo {
//     pub fn new(file_name: String, hash: String) -> HashInfo {
//         HashInfo { file_name, hash }
//     }
// }

pub struct JsonStorage<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T> JsonStorage<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    pub fn from_json(path: &Path) -> io::Result<T> {
        let mut file_contents = String::new();
        let mut file = File::open(path)?;
        file.read_to_string(&mut file_contents)?;
        let data: T = serde_json::from_str(&file_contents)?;
        Ok(data)
    }

    pub fn to_json(data: &T, path: &Path) -> io::Result<()> {
        let file = File::create(path)?;
        to_writer_pretty(file, &data)?;
        Ok(())
    }
    pub async fn from_url(url: &str) -> Result<T, Box<dyn std::error::Error>> {
        let response = reqwest::get(url).await?.text().await?;
        let repo_info: T = serde_json::from_str(&response)?;
        Ok(repo_info)
    }
    pub fn from_str(file_contents: &str) -> io::Result<T> {
        let data: T = serde_json::from_str(&file_contents)?;
        Ok(data)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoInfo {
    pub file_name: String,
    pub version: String,
    pub description: String,
    pub hash: String,
    pub url: String,
    pub entry: String,
}

impl RepoInfo {
    pub fn new(
        file_name: String,
        version: String,
        description: String,
        hash: String,
        url: String,
        entry: String,
    ) -> RepoInfo {
        RepoInfo {
            file_name,
            version,
            description,
            hash,
            url,
            entry,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Setting {
    pub repo_url: String,
    pub repo_info: String,
}
impl Setting {
    pub fn new(repo_url: String, repo_info: String) -> Setting {
        Setting {
            repo_url,
            repo_info,
        }
    }
}
