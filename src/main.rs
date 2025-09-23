use macroquad::prelude::*;
use macroquad::window;

fn window_conf() -> Conf {
    Conf {
        window_title: "My first game :)".to_string(),
        fullscreen: true,   // start in fullscreen
        window_width: 800,  // ignored when fullscreen = true
        window_height: 650, // ignored when fullscreen = true
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(BLACK);
        next_frame().await
    }
}
