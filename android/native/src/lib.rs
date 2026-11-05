//! Android NativeActivity entry point for Horizon.
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub fn android_main(app: winit::platform::android::activity::AndroidApp) {
    let Some(data_dir) = app.internal_data_path() else {
        eprintln!("Horizon: Android did not provide an internal app data directory");
        return;
    };

    // Keep game files inside Android's app-specific files directory.
    let game_dir = data_dir.join("horizon");
    if let Err(error) = std::fs::create_dir_all(&game_dir) {
        eprintln!("Horizon: cannot create app data directory: {error}");
        return;
    }
    if let Err(error) = std::env::set_current_dir(&game_dir) {
        eprintln!("Horizon: cannot enter app data directory: {error}");
        return;
    }

    horizon_client::logging::set_game_home(game_dir.clone());
    horizon_client::logging::set_player_dir(game_dir);
    horizon_client::android::install_android_app(app);

    // Run the existing Horizon startup, title GUI, game screens and systems.
    horizon_client::entry::main();
}
