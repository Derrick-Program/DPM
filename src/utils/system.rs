use crate::MyResult;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, ExitStatus, Stdio};
enum PackageManager {
    Apt,
    Dnf,
    Yum,
    Pacman,
    Zypper,
    Brew,
    Unknown,
}
pub fn check_dir_exists(dir_name: &Path) -> bool {
    dir_name.exists() && dir_name.is_dir()
}
pub fn list_dir(path: &Path) {
    if let Ok(entries) = std::fs::read_dir(Path::new(path)) {
        for entry in entries {
            if let Ok(entry) = entry {
                println!("{:?}", entry.path());
            }
        }
    }
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
pub fn install_package(package_name: &str) {
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
    command_runner(command, args, err);
}
pub fn update_package_index() {
    let manager = detect_package_manager();

    let (command, args) = match manager {
        PackageManager::Apt => ("apt-get", vec!["update"]),
        PackageManager::Dnf | PackageManager::Yum => ("dnf", vec!["makecache"]),
        PackageManager::Pacman => ("pacman", vec!["-Sy"]),
        PackageManager::Zypper => ("zypper", vec!["refresh"]),
        PackageManager::Brew => ("brew", vec!["update"]),
        PackageManager::Unknown => panic!("Unsupported package manager."),
    };
}
pub fn uninstall_package(package_name: &str) {
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
    command_runner(command, args, err);
}
pub fn search_package(package_name: &str) {
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
    command_runner(command, args, err);
}

fn command_runner(command: &str, args: Vec<&str>, err_message: &str) -> MyResult<()> {
    let mut cmd = if cfg!(target_os = "linux") {
        let mut c = Command::new("sudo");
        c.arg(command);
        c
    } else {
        Command::new(command)
    };
    let status = cmd
        .args(&args)
        // .stdout(Stdio::null()) // 不顯示標準輸出
        // .stderr(Stdio::null()) // 不顯示錯誤輸出
        // .stdout(Stdio::piped())
        // .stderr(Stdio::piped())
        .status()?;
    if !status.success() {
        panic!("{}", err_message);
    }
    Ok(())
}
//
// fn command_runner(command: &str, args: Vec<&str>, err_message: &str) -> MyResult<()> {
//     let mut cmd = if cfg!(target_os = "linux") {
//         let mut c = Command::new("sudo");
//         c.arg(command);
//         c
//     } else {
//         Command::new(command)
//     };

//     let mut child = cmd
//         .args(&args)
//         .stdout(Stdio::piped())
//         .stderr(Stdio::piped())
//         .spawn()?;
//     let stdout = child.stdout.take().ok_or("Failed to open stdout")?;
//     let stderr = child.stderr.take().ok_or("Failed to open stderr")?;
//     let stdout_reader = BufReader::new(stdout);
//     let stderr_reader = BufReader::new(stderr);
//     let stdout_handle = std::thread::spawn(move || {
//         for line in stdout_reader.lines() {
//             match line {
//                 Ok(line) => println!("stdout: {}", line),
//                 Err(e) => eprintln!("stdout error: {}", e),
//             }
//         }
//     });
//     let stderr_handle = std::thread::spawn(move || {
//         for line in stderr_reader.lines() {
//             match line {
//                 Ok(line) => println!("stderr: {}", line),
//                 Err(e) => eprintln!("stderr error: {}", e),
//             }
//         }
//     });
//     let status = child.wait()?;
//     stdout_handle
//         .join()
//         .expect("The stdout thread has panicked");
//     stderr_handle
//         .join()
//         .expect("The stderr thread has panicked");
//     if !status.success() {
//         panic!("{}", err_message);
//     }

//     Ok(())
// }
