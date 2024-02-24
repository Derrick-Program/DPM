// use colored::Colorize;
use DPM::MyResult;
fn main() -> MyResult<()> {
    // if sudo::check() == sudo::RunningAs::Root {
    //     sudo::escalate_if_needed()?;
    //     if let Err(e) = DPM::get_args().and_then(DPM::entry) {
    //         eprintln!("{}", e);
    //         std::process::exit(1);
    //     }
    // } else {
    //     eprintln!("{}", "Please run this command as root.".red());
    //     std::process::exit(1);
    if let Err(e) = DPM::get_args().and_then(DPM::entry) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
    Ok(())
}
