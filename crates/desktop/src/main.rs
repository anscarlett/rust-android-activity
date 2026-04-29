//! Desktop launcher for the Bevy shapes app.
//!
//! Run with:
//! ```sh
//! cargo run -p desktop
//! ```

use bevy::prelude::*;

fn main() {
    let mut bevy_app = App::new();
    bevy_app.add_plugins(DefaultPlugins);
    app::add_app_plugins(&mut bevy_app);
    bevy_app.run();
}
