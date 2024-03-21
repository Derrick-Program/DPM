use crate::{
    setting, ActionInfo, Db, JsonStorage, MyError, MyResult, BIN_DIR, CONFIG, INSTALL_DIR, MAIN_DIR,
};
use libc::{getpwuid, getuid, passwd};
use std::{
    collections::HashMap,
    error::Error,
    ffi::CStr,
    fs::{create_dir_all, File},
    io::{BufRead, BufReader, Write},
    path::Path,
    process::{Command, ExitStatus, Stdio},
    ptr,
};
enum PackageManager {
    Apt,
    Dnf,
    Yum,
    Pacman,
    Zypper,
    Brew,
    Unknown,
}

fn get_current_username() -> Option<String> {
    unsafe {
        let uid = getuid();
        let pwd = getpwuid(uid);
        if pwd != ptr::null_mut() {
            let c_str = CStr::from_ptr((*pwd).pw_name);
            c_str.to_str().ok().map(|s| s.to_owned())
        } else {
            None
        }
    }
}
pub fn permision_check() -> MyResult<()> {
    let username = match get_current_username() {
        Some(username) => username,
        None => panic!("Could not get current username"),
    };
    if cfg!(target_os = "linux") {
        system_command_runner(
            "chown",
            vec!["-R", "root:root", MAIN_DIR],
            "Can't run chown",
        )?;
    } else if cfg!(target_os = "macos") {
        system_command_runner(
            "chown",
            vec!["-R", format!("{}:admin", username).as_str(), MAIN_DIR],
            "Can't run chown",
        )?;
    }
    Ok(())
}

pub async fn init() -> MyResult<HashMap<String, String>> {
    system_command_runner(
        "mkdir",
        vec!["-p", INSTALL_DIR],
        "Can't '/opt/DPM/Software' dir",
    )?;
    system_command_runner("mkdir", vec!["-p", CONFIG], "Can't /opt/DPM/Setting dir")?;
    system_command_runner("mkdir", vec!["-p", BIN_DIR], "Can't /opt/DPM/bin dir")?;
    permision_check()?;
    let config_path = Path::new(CONFIG).join("config.json");
    let main_path = Path::new(MAIN_DIR);
    let db = Db::new(main_path)?;
    db.create_table()?;
    if !config_path.exists() {
        let mut file = File::create(&config_path)?;
        file.write_all(b"{}")?;
        let mut config: setting =
            JsonStorage::from_json(&config_path).unwrap_or_else(|_| HashMap::new());
        config.insert(
            "repo_url".to_string(),
            "https://github.com/Derrick-Program/DPM-Server/tree/main/Repo".to_string(),
        );
        config.insert(
            "repo_info".to_string(),
            "https://raw.githubusercontent.com/Derrick-Program/DPM-Server/main/RepoInfo.json"
                .to_string(),
        );
        JsonStorage::to_json(&config, &config_path);
        ActionInfo::init_update(config.get("repo_info").unwrap()).await?;
    }
    let config: setting = JsonStorage::from_json(&config_path)?;
    Ok(config)
}

fn detect_package_manager() -> PackageManager {
    let managers = vec![
        ("apt-get", PackageManager::Apt),
        ("dnf", PackageManager::Dnf),
        ("yum", PackageManager::Yum),
        ("pacman", PackageManager::Pacman),
        ("zypper", PackageManager::Zypper),
        ("brew", PackageManager::Brew),
    ];
    for (command, manager) in managers {
        if Command::new(command).arg("--version").output().is_ok() {
            return manager;
        }
    }
    PackageManager::Unknown
}
pub fn install_package(package_name: &str, verbose: bool) -> MyResult<()> {
    let manager = detect_package_manager();
    let err = format!("Failed to install package: {}", package_name);
    let err = err.as_str();
    let (command, args) = match manager {
        PackageManager::Apt => ("apt-get", vec!["install", "-y", package_name]),
        PackageManager::Dnf => ("dnf", vec!["install", "-y", package_name]),
        PackageManager::Yum => ("yum", vec!["install", "-y", package_name]),
        PackageManager::Pacman => ("pacman", vec!["-S", "--noconfirm", package_name]),
        PackageManager::Zypper => ("zypper", vec!["install", "-y", package_name]),
        PackageManager::Brew => ("brew", vec!["install", package_name]),
        PackageManager::Unknown => panic!("Unsupported package manager."),
    };
    command_runner(command, args, err, verbose)?;
    Ok(())
}
pub fn update_package_index(verbose: bool) {
    let manager = detect_package_manager();
    let err = "Failed to update package index";
    let (command, args) = match manager {
        PackageManager::Apt => ("apt-get", vec!["update"]),
        PackageManager::Dnf | PackageManager::Yum => ("dnf", vec!["makecache"]),
        PackageManager::Pacman => ("pacman", vec!["-Sy"]),
        PackageManager::Zypper => ("zypper", vec!["refresh"]),
        PackageManager::Brew => ("brew", vec!["update"]),
        PackageManager::Unknown => panic!("Unsupported package manager."),
    };
    command_runner(command, args, err, verbose);
}
pub fn uninstall_package(package_name: &str, verbose: bool) {
    let manager = detect_package_manager();
    let err = format!("Failed to remove package: {}", package_name);
    let err = err.as_str();
    let (command, args) = match manager {
        PackageManager::Apt => ("apt-get", vec!["remove", "-y", package_name]),
        PackageManager::Dnf | PackageManager::Yum => ("dnf", vec!["remove", "-y", package_name]),
        PackageManager::Pacman => ("pacman", vec!["-R", package_name]),
        PackageManager::Zypper => ("zypper", vec!["remove", "-y", package_name]),
        PackageManager::Brew => ("brew", vec!["uninstall", package_name]),
        PackageManager::Unknown => panic!("Unsupported package manager."),
    };
    command_runner(command, args, err, verbose);
}
pub fn search_package(package_name: &str, verbose: bool) {
    let manager = detect_package_manager();
    let err = format!("Failed to search package: {}", package_name);
    let err = err.as_str();
    let (command, args) = match manager {
        PackageManager::Apt => ("apt-cache", vec!["search", package_name]),
        PackageManager::Dnf => ("dnf", vec!["search", package_name]),
        PackageManager::Yum => ("yum", vec!["search", package_name]),
        PackageManager::Pacman => ("pacman", vec!["-Ss", package_name]),
        PackageManager::Zypper => ("zypper", vec!["search", package_name]),
        PackageManager::Brew => ("brew", vec!["search", package_name]),
        PackageManager::Unknown => panic!("Unsupported package manager."),
    };
    command_runner(command, args, err, verbose);
}
pub fn upgrade_package(package_name: &str, verbose: bool) {
    let manager = detect_package_manager();
    let err = format!("Failed to upgrade package: {}", package_name);
    let err = err.as_str();
    let (command, args) = match manager {
        PackageManager::Apt => (
            "apt-get",
            vec!["install", "--only-upgrade", "-y", package_name],
        ),
        PackageManager::Dnf | PackageManager::Yum => ("dnf", vec!["upgrade", "-y", package_name]),
        PackageManager::Pacman => ("pacman", vec!["-Syu", package_name]),
        PackageManager::Zypper => ("zypper", vec!["update", "-y", package_name]),
        PackageManager::Brew => ("brew", vec!["upgrade", package_name]),
        PackageManager::Unknown => panic!("Unsupported package manager."),
    };
    command_runner(command, args, err, verbose);
}

pub fn list_packages(verbose: bool) {
    let manager = detect_package_manager();
    let err = "Failed to list packages";

    let (command, args) = match manager {
        PackageManager::Apt => ("apt", vec!["list", "--installed"]),
        PackageManager::Dnf => ("dnf", vec!["list", "installed"]),
        PackageManager::Yum => ("yum", vec!["list", "installed"]),
        PackageManager::Pacman => ("pacman", vec!["-Q"]),
        PackageManager::Zypper => ("zypper", vec!["search", "--installed-only"]),
        PackageManager::Brew => ("brew", vec!["list"]),
        PackageManager::Unknown => panic!("Unsupported package manager."),
    };

    command_runner(command, args, err, verbose);
}

fn command_runner(
    command: &str,
    args: Vec<&str>,
    err_message: &str,
    verbose: bool,
) -> MyResult<()> {
    let mut cmd = if cfg!(target_os = "linux") {
        let mut c = Command::new("sudo");
        c.arg(command);
        c
    } else {
        Command::new(command)
    };

    if verbose {
        cmd.stdout(Stdio::inherit()); // 繼承標準輸出
        cmd.stderr(Stdio::inherit()); // 繼承標準錯誤
    } else {
        cmd.stdout(Stdio::null()); // 忽略標準輸出
        cmd.stderr(Stdio::null()); // 忽略標準錯誤
    }

    cmd.args(&args);

    let status = cmd.status()?;
    if !status.success() {
        return Err(Box::new(MyError::new(err_message)));
    }

    Ok(())
}

pub fn system_command_runner(command: &str, args: Vec<&str>, err_message: &str) -> MyResult<()> {
    let mut cmd = Command::new(command);
    if !(cfg!(target_os = "linux") || cfg!(target_os = "macos")) {
        panic!("Unsupported OS");
    }
    let mut cmd = Command::new("sudo");
    cmd.arg(command);
    // cmd.stdout(Stdio::null());
    // cmd.stderr(Stdio::null());
    cmd.args(&args);
    let status = cmd.status()?;
    if !status.success() {
        return Err(Box::new(MyError::new(err_message)));
    }
    Ok(())
}
