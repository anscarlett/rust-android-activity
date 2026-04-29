//! Android NativeActivity launcher for the Bevy shapes app.
//!
//! This is a thin `cdylib` that provides the `android_main` entry point
//! expected by `android-activity`'s NativeActivity backend.  All actual game
//! logic lives in the shared `app` crate.

use android_activity::AndroidApp;
use bevy::{
    prelude::*,
    window::{MonitorSelection, WindowMode},
};

#[unsafe(no_mangle)]
fn android_main(android_app: AndroidApp) {
    // Pass the AndroidApp handle to Bevy's winit integration.
    // Bevy's LogPlugin (part of DefaultPlugins) sets up Android logcat logging.
    let mut bevy_app = App::new();
    bevy_app.insert_non_send_resource(android_app);

    // Fullscreen borderless window – appropriate for Android.
    bevy_app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resizable: false,
            mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
            ..default()
        }),
        ..default()
    }));

    // Register shared game logic.
    app::add_app_plugins(&mut bevy_app);

    bevy_app.run();
}
