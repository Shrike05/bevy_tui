pub mod tui;

pub fn setup_logger() {
    tui_logger::init_logger(tui_logger::LevelFilter::Info).unwrap();
    tui_logger::set_default_level(tui_logger::LevelFilter::Info);
}

#[cfg(test)]
mod tests {}
