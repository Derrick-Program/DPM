use std::{
    collections::HashMap,
    fs::{self, remove_dir_all, remove_file, File, Permissions},
    io::Read,
    os::unix::fs::PermissionsExt,
    path::Path,
};

use colored::Colorize;
use futures_util::future::Inspect;
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

use crate::{
    hashes, read_file_from_zip, setting, system::*, unzip_file, Db, DbPackage, JsonStorage,
    MyError, MyResult, PackageInfo, Repo, Repos, BIN_DIR, INSTALL_DIR, MAIN_DIR,
};
#[derive(Debug)]
pub struct ActionInfo {
    pub pkgs: Vec<String>,
    pub verbose: bool,
    pub setting_config: setting,
    pub repo_info: Db,
}
impl ActionInfo {
    pub fn new(
        pkgs: Vec<String>,
        verbose: bool,
        setting_config: setting,
        repo_info: Db,
    ) -> ActionInfo {
        ActionInfo {
            pkgs,
            verbose,
            setting_config,
            repo_info,
        }
    }
    fn parse_mine(&self) -> (Vec<String>, Vec<String>) {
        let mut is: Vec<String> = Vec::new();
        let mut isnot: Vec<String> = Vec::new();
        for pkg in &self.pkgs {
            if self.repo_info.read_all().unwrap().contains_key(pkg) {
                is.push(pkg.clone());
            } else {
                isnot.push(pkg.clone());
            }
        }
        (is, isnot)
    }
    pub async fn install(&self) -> MyResult<()> {
        let (is, isnot) = self.parse_mine();
        if !is.is_empty() {
            for pkg in is {
                if self.verbose {
                    println!("{}\n\n  {}", &pkg.on_green(), "Downloading...".yellow());
                }
                self.repo_info.download_file(pkg.as_str()).await?;
                if self.verbose {
                    println!("  {}", "Download successed!".green());
                }
                let ori_path = Path::new("/tmp").join(
                    &self
                        .repo_info
                        .read_one_field("filename", pkg.as_str())
                        .unwrap(),
                );
                let repo_package_info = self.repo_info.read_one(pkg.as_str()).unwrap();
                let package_info_test: String =
                    read_file_from_zip(&ori_path, "packageInfo.json").unwrap();
                let package_info: PackageInfo =
                    JsonStorage::from_str(package_info_test.as_str()).unwrap();
                let package_hash_info: hashes = JsonStorage::from_str(
                    read_file_from_zip(&ori_path, "hashes.json")
                        .unwrap()
                        .as_str(),
                )
                .unwrap();
                if self.verbose {
                    println!(
                        "  {}",
                        "Checking Package Hash ...(May take a while)".yellow()
                    );
                }
                if self.repo_info.read_one_field("hash", &pkg).unwrap() != Self::hasher(&ori_path)?
                {
                    return Err(Box::new(MyError::new(
                        format!("{}", "Hash Value Not Same Very dangerous".red()).as_str(),
                    )));
                }
                if &package_info.hash != package_hash_info.get("hashes.json").unwrap() {
                    return Err(Box::new(MyError::new("Hash value not same")));
                }

                if self.verbose {
                    println!("  {}", "Hashes Passed".green());
                    println!("  {}", "Installing ...".yellow());
                }

                let install_path = Path::new(INSTALL_DIR).join(pkg.as_str());
                unzip_file(&ori_path, Path::new(INSTALL_DIR), pkg.as_str())?;
                if self.verbose {
                    println!("  {}", "Installed!".green());
                    println!("  {}", "Removing tmp file ...".blue());
                }
                remove_file(ori_path)?;
                if self.verbose {
                    println!("  {}", "Removed Success ...".green());
                    println!("  {}", "Create Links ...".yellow());
                }
                let main_file = install_path.join(&package_info.file_name);
                let ln_path = Path::new(BIN_DIR).join(pkg);
                fs::set_permissions(&main_file, Permissions::from_mode(0o755));
                system_command_runner(
                    "ln",
                    vec![
                        "-s",
                        main_file.display().to_string().as_str(),
                        ln_path.display().to_string().as_str(),
                    ],
                    "Can't create link",
                );
                if self.verbose {
                    println!("  {}", "Successed Create Link!".green());
                }
            }
        }
        if !isnot.is_empty() {
            for pkg in isnot {
                install_package(&pkg, self.verbose)?;
            }
        }
        Ok(())
    }

    pub async fn update(&self) -> MyResult<()> {
        println!("{} Updating...", "==>".blue());
        let info = Repo::init(self.setting_config.get("repo_info").unwrap()).await?;
        let info = info.get_allInfo();
        let file = Path::new(MAIN_DIR).join("LocalRepo.db");
        if file.exists() {
            remove_file(file)?;
        }
        let db = Db::new(Path::new(MAIN_DIR))?;
        db.create_table()?;
        for (name, repo_info) in info {
            db.insert(DbPackage::new(
                &name,
                repo_info.version.as_str(),
                repo_info.url.as_str(),
                repo_info.description.as_str(),
                repo_info.file_name.as_str(),
                repo_info.hash.as_str(),
                repo_info.entry.as_str(),
            ))?;
        }
        // update_package_index(self.verbose);
        println!("{} Updated!", "==>".green());
        Ok(())
    }
    pub async fn init_update(url_json: &String) -> MyResult<()> {
        let info = Repo::init(url_json).await?;
        let info = info.get_allInfo();
        let db = Db::new(Path::new(MAIN_DIR))?;
        for (name, repo_info) in info {
            db.insert(DbPackage::new(
                &name,
                repo_info.version.as_str(),
                repo_info.url.as_str(),
                repo_info.description.as_str(),
                repo_info.file_name.as_str(),
                repo_info.hash.as_str(),
                repo_info.entry.as_str(),
            ))?;
        }
        Ok(())
    }

    pub fn uninstall(&self) -> MyResult<()> {
        let (is, isnot) = self.parse_mine();
        if !is.is_empty() {
            for pkg in is {
                let pre_rm_location = Path::new(INSTALL_DIR).join(&pkg);
                let pre_rm_ln = Path::new(BIN_DIR).join(&pkg);
                if self.verbose {
                    println!("{}\n\n  {}", &pkg.on_green(), "Removing...".red());
                }
                remove_dir_all(pre_rm_location)?;
                if self.verbose {
                    println!("  {}", "Removed!".green());
                    println!("  {}", "UnLinking...".red());
                }
                remove_file(pre_rm_ln)?;
                if self.verbose {
                    println!("  {}", "Done".green());
                }
            }
        }
        if !isnot.is_empty() {
            for pkg in isnot {
                uninstall_package(&pkg, self.verbose);
            }
        }
        Ok(())
    }

    pub fn search(&self) -> MyResult<()> {
        let (is, isnot) = self.parse_mine();
        if !is.is_empty() {
            println!();
            for pkg in is {
                println!("{} {}", &pkg, "Found!!".green());
            }
        }
        if !isnot.is_empty() {
            for pkg in &self.pkgs {
                search_package(pkg.as_str(), self.verbose);
            }
        }
        Ok(())
    }

    pub fn list(&self, sys: bool) -> MyResult<()> {
        if sys {
            list_packages(self.verbose);
        } else {
            let path = Path::new(INSTALL_DIR);
            for entry in WalkDir::new(path) {
                let entry = entry?;
                let path = entry.path();
            }
        }
        Ok(())
    }

    pub fn upgrade(&self) {
        let (is, isnot) = self.parse_mine();
        if !is.is_empty() {
            for pkg in is {
                println!("{:#?}", pkg);
            }
        }
        if !isnot.is_empty() {
            for pkg in isnot {
                upgrade_package(&pkg, self.verbose);
            }
        }
    }

    pub fn upgrade_self(&self) {
        println!("{} Upgrading self", "==>".blue());
    }

    fn hasher(file_path: &Path) -> MyResult<String> {
        let mut hasher = Sha256::new();
        let mut file = File::open(&file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        hasher.update(&buffer);
        let result = hasher.finalize();
        Ok(hex::encode(result))
    }
}
