use crate::system::*;
pub fn install(pkgs: &Vec<String>) {
    for pkg in pkgs {
        install_package(&pkg);
    }
}

pub fn update() {
    todo!();
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
    todo!();
}
pub fn upgrade() {
    todo!();
}
