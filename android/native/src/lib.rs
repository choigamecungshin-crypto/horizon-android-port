//! Android NativeActivity entry point for the original Horizon launcher.
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: android_activity::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("Horizon"),
    );
    let Some(data_dir) = app.internal_data_path() else {
        log::error!("Android did not provide an app-private files directory");
        return;
    };
    let home = data_dir.join("horizon");
    if let Err(error) = std::fs::create_dir_all(&home) {
        log::error!("Cannot create Horizon data directory: {error}");
        return;
    }
    if let Err(error) = std::env::set_current_dir(&home) {
        log::error!("Cannot enter Horizon data directory: {error}");
        return;
    }
    log::info!("Starting original Horizon launcher UI");
    horizon_launcher::run_android(app);
}
