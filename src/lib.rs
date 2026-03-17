pub mod tui;
pub use crate::tui::*;

pub fn setup_logger() {
    tui_logger::init_logger(tui_logger::LevelFilter::Info).unwrap();
    tui_logger::set_default_level(tui_logger::LevelFilter::Info);
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    /// Simple program to greet a person
    #[derive(Parser, Debug)]
    #[command(version, about, long_about = None)]
    struct Hello {
        /// the name of the person you are greeting
        name: String,
    }

    #[test]
    fn parse_clap() {
        let cmnd = TUICommand::new("hello world");
        let res = cmnd.parse_clap::<Hello>().expect("Couldn't parse Hello");
        assert_eq!("world", res.name);
    }
}
