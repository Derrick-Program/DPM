
fn main() {
    if let Err(e) = DPM::get_args().and_then(DPM::entry) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
