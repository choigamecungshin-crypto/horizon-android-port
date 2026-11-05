#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

#[cfg(not(target_os = "android"))]
fn main() {
    horizon_launcher::run_desktop();
}
