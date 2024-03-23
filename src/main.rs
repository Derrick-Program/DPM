use colored::Colorize;
use DPM::MyResult;

fn main() -> MyResult<()> {
    if cfg!(target_os = "linux") {
        if sudo::check() == sudo::RunningAs::Root {
            sudo::escalate_if_needed()?;
            if let Err(e) = DPM::get_args().and_then(DPM::entry) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        } else {
            eprintln!("{}", "Please run this command as root.".red());
            std::process::exit(1);
        }
    } else if cfg!(target_os = "macos") {
        if let Err(e) = DPM::get_args().and_then(DPM::entry) {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}
