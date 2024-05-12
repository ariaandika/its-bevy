mod chess;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(chess::plugins)
        .run()
}

