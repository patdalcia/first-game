use macroquad::prelude::*;

const SHIP_HEIGHT: f32 = 25.;
const SHIP_BASE: f32 = 22.;

struct Ship {
    pos: Vec2,
    rot: f32, // in degrees
    vel: Vec2,
}

struct Bullet {
    pos: Vec2,
    vel: Vec2,
    shot_at: f64,
    collided: bool,
}

struct Asteroid {
    pos: Vec2,
    vel: Vec2,
    rot: f32,
    rot_speed: f32,
    size: f32,
    sides: u8,
    collided: bool,
}

enum GameState {
    Playing,
    Paused,
}

fn wrap_around(v: &Vec2) -> Vec2 {
    let mut vr = *v;
    if vr.x > screen_width() {
        vr.x = 0.;
    }
    if vr.x < 0. {
        vr.x = screen_width();
    }
    if vr.y > screen_height() {
        vr.y = 0.;
    }
    if vr.y < 0. {
        vr.y = screen_height();
    }
    vr
}

fn conf() -> Conf {
    Conf {
        window_title: "Asteroids".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut ship = Ship {
        pos: vec2(screen_width() / 2., screen_height() / 2.),
        rot: 0.0,
        vel: Vec2::ZERO,
    };
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut last_shot = get_time();
    let mut asteroids: Vec<Asteroid> = Vec::new();
    let mut gameover = false;
    let mut state = GameState::Playing;

    let mut joystick_touch_id: Option<u64> = None;
    let mut joystick_origin: Vec2 = Vec2::ZERO;

    // start screen
    loop {
        clear_background(LIGHTGRAY);
        let text = "Asteroids - Press Enter or Tap to Start";
        let fs = 30.0;
        let ts = measure_text(text, None, fs as _, 1.0);
        draw_text(
            text,
            screen_width() / 2. - ts.width / 2.,
            screen_height() / 2. - ts.height / 2.,
            fs,
            DARKGRAY,
        );
        if is_key_down(KeyCode::Enter) || !touches().is_empty() {
            break;
        }
        next_frame().await;
    }

    let screen_center = vec2(screen_width() / 2., screen_height() / 2.);
    for _ in 0..10 {
        asteroids.push(Asteroid {
            pos: screen_center
                + vec2(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)).normalize()
                    * screen_width().min(screen_height())
                    / 2.,
            vel: vec2(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)),
            rot: 0.,
            rot_speed: rand::gen_range(-2., 2.),
            size: screen_width().min(screen_height()) / 10.,
            sides: rand::gen_range(3u8, 8u8),
            collided: false,
        });
    }

    loop {
        if gameover {
            clear_background(LIGHTGRAY);
            let text = if asteroids.is_empty() {
                "You Win! Press Enter or Tap to Restart"
            } else {
                "Game Over. Press Enter or Tap to Restart"
            };
            let fs = 30.0;
            let ts = measure_text(text, None, fs as _, 1.0);
            draw_text(
                text,
                screen_width() / 2. - ts.width / 2.,
                screen_height() / 2. - ts.height / 2.,
                fs,
                DARKGRAY,
            );
            if is_key_down(KeyCode::Enter) || !touches().is_empty() {
                ship = Ship {
                    pos: vec2(screen_width() / 2., screen_height() / 2.),
                    rot: 0.0,
                    vel: Vec2::ZERO,
                };
                bullets.clear();
                asteroids.clear();
                gameover = false;
                for _ in 0..10 {
                    asteroids.push(Asteroid {
                        pos: screen_center
                            + vec2(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)).normalize()
                                * screen_width().min(screen_height())
                                / 2.,
                        vel: vec2(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)),
                        rot: 0.,
                        rot_speed: rand::gen_range(-2., 2.),
                        size: screen_width().min(screen_height()) / 10.,
                        sides: rand::gen_range(3u8, 8u8),
                        collided: false,
                    });
                }
            }
            next_frame().await;
            continue;
        }

        if is_key_pressed(KeyCode::Escape) {
            state = match state {
                GameState::Playing => GameState::Paused,
                GameState::Paused => GameState::Playing,
            };
        }

        match state {
            GameState::Playing => {
                let now = get_time();
                let mut acc = -ship.vel / 100.0;

                if is_key_down(KeyCode::Left) {
                    ship.rot -= 5.0;
                }
                if is_key_down(KeyCode::Right) {
                    ship.rot += 5.0;
                }
                if is_key_down(KeyCode::Up) {
                    let angle = ship.rot.to_radians();
                    acc = vec2(angle.sin(), -angle.cos()) * 2.0;
                }

                let tlist = touches();
                if !tlist.is_empty() {
                    let t0 = &tlist[0];
                    if joystick_touch_id.is_none() {
                        joystick_touch_id = Some(t0.id);
                        joystick_origin = t0.position;
                    }
                    if Some(t0.id) == joystick_touch_id {
                        let delta = t0.position - joystick_origin;
                        if delta.length() > 10.0 {
                            let dir = delta.normalize();
                            ship.rot = dir.y.atan2(dir.x).to_degrees();
                            acc = dir * 2.0;
                        }
                    }
                } else {
                    joystick_touch_id = None;
                }

                if (is_key_down(KeyCode::Space) || touches().len() >= 2) && (now - last_shot > 0.5)
                {
                    let angle = ship.rot.to_radians();
                    let dir = vec2(angle.sin(), -angle.cos());
                    bullets.push(Bullet {
                        pos: ship.pos + dir * SHIP_HEIGHT / 2.,
                        vel: dir * 7.0,
                        shot_at: now,
                        collided: false,
                    });
                    last_shot = now;
                }

                ship.vel += acc;
                if ship.vel.length() > 5.0 {
                    ship.vel = ship.vel.normalize() * 5.0;
                }

                ship.pos += ship.vel;
                ship.pos = wrap_around(&ship.pos);

                for b in bullets.iter_mut() {
                    b.pos += b.vel;
                }
                for a in asteroids.iter_mut() {
                    a.pos += a.vel;
                    a.pos = wrap_around(&a.pos);
                    a.rot += a.rot_speed;
                }

                bullets.retain(|b| b.shot_at + 1.5 > now && !b.collided);
                let mut new_asts = Vec::new();

                for a in asteroids.iter_mut() {
                    if (a.pos - ship.pos).length() < a.size + SHIP_HEIGHT / 3.0 {
                        gameover = true;
                        break;
                    }
                    for b in bullets.iter_mut() {
                        if (a.pos - b.pos).length() < a.size {
                            a.collided = true;
                            b.collided = true;
                            if a.sides > 3 {
                                new_asts.push(Asteroid {
                                    pos: a.pos,
                                    vel: vec2(b.vel.y, -b.vel.x).normalize()
                                        * rand::gen_range(1., 3.),
                                    rot: rand::gen_range(0., 360.),
                                    rot_speed: rand::gen_range(-2., 2.),
                                    size: a.size * 0.8,
                                    sides: a.sides - 1,
                                    collided: false,
                                });
                                new_asts.push(Asteroid {
                                    pos: a.pos,
                                    vel: vec2(-b.vel.y, b.vel.x).normalize()
                                        * rand::gen_range(1., 3.),
                                    rot: rand::gen_range(0., 360.),
                                    rot_speed: rand::gen_range(-2., 2.),
                                    size: a.size * 0.8,
                                    sides: a.sides - 1,
                                    collided: false,
                                });
                            }
                            break;
                        }
                    }
                }

                asteroids.retain(|a| !a.collided);
                asteroids.append(&mut new_asts);

                if asteroids.is_empty() {
                    gameover = true;
                }

                clear_background(LIGHTGRAY);

                for b in bullets.iter() {
                    draw_circle(b.pos.x, b.pos.y, 2.0, BLACK);
                }
                for a in asteroids.iter() {
                    draw_poly_lines(a.pos.x, a.pos.y, a.sides, a.size, a.rot, 2.0, BLACK);
                }

                // elongated ship triangle
                let angle = ship.rot.to_radians();
                let dir_forward = vec2(angle.sin(), -angle.cos());
                let dir_left = vec2(-angle.cos(), -angle.sin());
                let dir_right = vec2(angle.cos(), angle.sin());

                let nose = ship.pos + dir_forward * (SHIP_HEIGHT * 0.8);
                let back_offset = dir_forward * -(SHIP_HEIGHT * 0.3);
                let half_base = SHIP_BASE * 0.3;

                let v2 = ship.pos + back_offset + dir_left * half_base;
                let v3 = ship.pos + back_offset + dir_right * half_base;

                draw_triangle_lines(nose, v2, v3, 2.0, BLACK);
            }

            GameState::Paused => {
                clear_background(LIGHTGRAY);
                let msg = "PAUSED (press ESC to resume)";
                let ts = measure_text(msg, None, 30, 1.0);
                draw_text(
                    msg,
                    screen_width() / 2. - ts.width / 2.,
                    screen_height() / 2. - ts.height / 2.,
                    30.0,
                    DARKGRAY,
                );
            }
        }

        next_frame().await;
    }
}
