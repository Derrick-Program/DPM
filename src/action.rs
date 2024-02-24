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
pub fn list() {
    list_packages();
}
pub fn upgrade(pkgs: &Vec<String>) {
    for pkg in pkgs {
        upgrade_package(&pkg);
    }
}
