use colored::Colorize;

use crate::{setting, system::*, Repo};
#[derive(Debug)]
pub struct ActionInfo {
    pub pkgs: Vec<String>,
    pub verbose: bool,
    pub setting_config: setting,
    pub repo_info: Repo,
}
impl ActionInfo {
    pub fn new(
        pkgs: Vec<String>,
        verbose: bool,
        setting_config: setting,
        repo_info: Repo,
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
            if self.repo_info.get_all_keys().contains(pkg) {
                is.push(pkg.clone());
            } else {
                isnot.push(pkg.clone());
            }
        }
        (is, isnot)
    }
    //TODO 需要繼續寫install及其他函數
    pub fn install(&self) {
        let (is, isnot) = self.parse_mine();
        if !is.is_empty() {
            for pkg in is {
                println!("{:#?}", pkg);
            }
        }
        if !isnot.is_empty() {
            for pkg in isnot {
                install_package(&pkg, self.verbose);
            }
        }
    }

    pub fn update(&self) {
        update_package_index(self.verbose);
    }

    pub fn uninstall(&self) {
        let (is, isnot) = self.parse_mine();
        if !is.is_empty() {
            for pkg in is {
                println!("{:#?}", pkg);
            }
        }
        if !isnot.is_empty() {
            for pkg in isnot {
                uninstall_package(&pkg, self.verbose);
            }
        }
    }

    pub fn search(&self) {
        for pkg in &self.pkgs {
            search_package(pkg.as_str(), self.verbose);
        }
    }

    pub fn list(&self, sys: bool) {
        if sys {
            list_packages(self.verbose);
        } else {
            println!("\n{} List my Repo", "==>".blue());
        }
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
}
