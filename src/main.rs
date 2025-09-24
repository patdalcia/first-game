use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Fullscreen Toggle".to_string(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    const MOVEMENT_SPEED: f32 = 200.0;
    // let the window initialize
    next_frame().await;

    // Wait one more frame so size updates
    next_frame().await;

    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        clear_background(BLACK);

        let delta_time = get_frame_time();
        if is_key_down(KeyCode::Right) {
            x += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            x -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            y += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            y -= MOVEMENT_SPEED * delta_time;
        }

        x = clamp(x, 0.0, screen_width());
        y = clamp(y, 0.0, screen_height());

        for touch in touches() {
            let (fill_color, size) = match touch.phase {
                TouchPhase::Started => (GREEN, 80.0),
                TouchPhase::Stationary => (WHITE, 60.0),
                TouchPhase::Moved => (YELLOW, 60.0),
                TouchPhase::Ended => (BLUE, 80.0),
                TouchPhase::Cancelled => (BLACK, 80.0),
            };
            draw_circle(touch.position.x, touch.position.y, size, fill_color);
        }

        draw_text("touch the screen!", 20.0, 20.0, 20.0, DARKGRAY);

        draw_circle(x, y, 16.0, YELLOW);
        next_frame().await
    }
}
