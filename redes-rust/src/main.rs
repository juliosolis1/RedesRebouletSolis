mod network;
mod ui;

use ui::RedesApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "RedesCall - Proyecto Semestral (UDP & TCP/IP)",
        options,
        Box::new(|_cc| Ok(Box::new(RedesApp::default()))),
    )
}