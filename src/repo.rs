use futures_util::StreamExt;
use reqwest::Error;
use std::{collections::HashMap, path::Path};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
pub type Repos = HashMap<String, RepoInfo>;
use crate::{JsonStorage, MyError, MyResult, RepoInfo};

#[derive(Debug)]
pub struct Repo {
    info: Option<Repos>,
}
#[allow(non_snake_case)]
impl Repo {
    fn new() -> Repo {
        Repo { info: None }
    }
    pub async fn init(url: String) -> MyResult<Repo> {
        let mut repo = Repo::new();
        let info = Self::get_Repo_Info(url).await?;
        repo.info = Some(info);
        Ok(repo)
    }
    async fn get_Repo_Info(url: String) -> MyResult<Repos> {
        let url = url.as_str();
        let data: Repos = JsonStorage::from_url(url).await?;
        Ok(data)
    }
    pub fn get_allInfo(&self) -> &Repos {
        self.info.as_ref().unwrap()
    }
    pub fn get_oneInfo(&self, name: &str) -> Option<&RepoInfo> {
        self.info.as_ref().unwrap().get(name)
    }
    pub fn get_all_keys(&self) -> Vec<String> {
        if let Some(info) = &self.info {
            info.keys().cloned().collect()
        } else {
            Vec::new()
        }
    }
    pub fn get_oneInfo_oneField(&self, name: &str, field: &str) -> Option<&str> {
        self.info
            .as_ref()?
            .get(name)
            .and_then(|repo_info| match field {
                "version" => Some(repo_info.version.as_str()),
                "description" => Some(repo_info.description.as_str()),
                "hash" => Some(repo_info.hash.as_str()),
                "url" => Some(repo_info.url.as_str()),
                "file_name" => Some(repo_info.file_name.as_str()),
                _ => None,
            })
    }
    // pub fn set_oneInfo(&mut self, name: &str, repo_info: RepoInfo) {
    //     if let Some(info) = self.info.as_mut() {
    //         info.insert(name.to_string(), repo_info);
    //     } else {
    //         let mut info = Repos::new();
    //         info.insert(name.to_string(), repo_info);
    //         self.info = Some(info);
    //     }
    // }
    // pub fn set_oneInfo_oneField(
    //     &mut self,
    //     name: &str,
    //     field: &str,
    //     value: &str,
    // ) -> Result<(), &'static str> {
    //     match self.info.as_mut() {
    //         Some(info) => {
    //             if let Some(repo_info) = info.get_mut(name) {
    //                 match field {
    //                     "version" => repo_info.version = value.to_string(),
    //                     "description" => repo_info.description = value.to_string(),
    //                     "hash" => repo_info.hash = value.to_string(),
    //                     "url" => repo_info.url = value.to_string(),
    //                     "file_name" => repo_info.file_name = value.to_string(),
    //                     _ => return Err("Invalid field name"),
    //                 }
    //                 Ok(())
    //             } else {
    //                 Err("Repo name not found")
    //             }
    //         }
    //         None => Err("Repo info not initialized"),
    //     }
    // }
    pub async fn download_file(&self, name: &str) -> MyResult<()> {
        let url = self.get_oneInfo_oneField(name, "url").unwrap();
        let req = reqwest::get(url).await?;
        if !req.status().is_success() {
            return Err(Box::new(MyError::new(
                format!("Failed to download file: HTTP {}", req.status()).as_str(),
            )));
        }
        let filename =
            Path::new("/tmp").join(self.get_oneInfo_oneField(name, "file_name").unwrap());
        let mut file = File::create(filename).await?;

        let mut stream = req.bytes_stream();

        while let Some(item) = stream.next().await {
            let chunk = item?;
            file.write_all(&chunk).await?;
        }
        Ok(())
    }
}
