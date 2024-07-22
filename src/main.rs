#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod dqx_iced_ui;
mod dqx_text_model;

use dqx_iced_ui::DqxTextApp;
use iced::{Application, Settings};

fn main() -> iced::Result {
    DqxTextApp::run(Settings::default())
}
