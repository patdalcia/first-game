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
    // Wait until screen dims settle (if needed)
    next_frame().await;
    next_frame().await;

    let mut pos = vec2(screen_width() / 2.0, screen_height() / 2.0);

    // Optional: keep a touch target, or None if no touch
    let mut touch_target: Option<Vec2> = None;

    loop {
        clear_background(BLACK);

        let dt = get_frame_time();

        // KEY MOVEMENT
        if is_key_down(KeyCode::Right) {
            pos.x += MOVEMENT_SPEED * dt;
        }
        if is_key_down(KeyCode::Left) {
            pos.x -= MOVEMENT_SPEED * dt;
        }
        if is_key_down(KeyCode::Down) {
            pos.y += MOVEMENT_SPEED * dt;
        }
        if is_key_down(KeyCode::Up) {
            pos.y -= MOVEMENT_SPEED * dt;
        }

        // TOUCH: set / update touch_target when user touches
        for touch in touches() {
            match touch.phase {
                TouchPhase::Started | TouchPhase::Moved | TouchPhase::Stationary => {
                    touch_target = Some(vec2(touch.position.x, touch.position.y));
                }
                TouchPhase::Ended | TouchPhase::Cancelled => {
                    // maybe clear the target when touch ends?
                    touch_target = None;
                }
            }
        }

        // If there's a touch target, move towards it
        if let Some(target) = touch_target {
            // pos = pos.move_towards(target, speed * dt)
            pos = pos.move_towards(target, MOVEMENT_SPEED * dt);
        }

        // Clamp so the circle stays in screen bounds
        pos.x = pos.x.clamp(0.0, screen_width());
        pos.y = pos.y.clamp(0.0, screen_height());

        draw_text(
            "Use arrow keys or touch to move",
            20.0,
            20.0,
            20.0,
            DARKGRAY,
        );

        // draw the circle at its current position
        draw_circle(pos.x, pos.y, 16.0, YELLOW);

        // also draw touch marker(s) if you want
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

        next_frame().await;
    }
}
