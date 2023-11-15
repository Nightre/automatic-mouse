// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use enigo::*;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref ENIGO: Mutex<Enigo> = Mutex::new(Enigo::new());
}

#[tauri::command]
fn move_mouse(x:i32,y:i32) -> bool {
    let mut enigo = ENIGO.lock().unwrap();
    enigo.mouse_move_to(x, y);
    true
}

#[tauri::command]
fn click_mouse() {
    let mut enigo = ENIGO.lock().unwrap();
    enigo.mouse_down(MouseButton::Left);
    enigo.mouse_up(MouseButton::Left);
}

#[tauri::command]
fn key_sequence(value:&str) {
    let mut enigo = ENIGO.lock().unwrap();
    enigo.key_sequence(value);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            move_mouse,
            click_mouse,
            key_sequence
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
