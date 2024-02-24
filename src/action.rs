use colored::Colorize;

use crate::system::*;
pub fn install(pkgs: &Vec<String>) {
    for pkg in pkgs {
        install_package(&pkg);
    }
}

pub fn update() {
    update_package_index();
}

pub fn uninstall(pkgs: &Vec<String>) {
    for pkg in pkgs {
        uninstall_package(&pkg);
    }
}
pub fn search(pkgs: &Vec<String>) {
    for pkg in pkgs {
        search_package(&pkg);
    }
}
pub fn list(sys: bool) {
    if sys {
        list_packages();
    } else {
        println!("\n{} List my Repo", "==>".blue());
    }
}
pub fn upgrade(pkgs: &Vec<String>) {
    for pkg in pkgs {
        upgrade_package(&pkg);
    }
}
pub fn upgrade_self() {
    println!("{} Upgrading self", "==>".blue());
}
