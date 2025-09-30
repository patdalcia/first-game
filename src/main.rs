use macroquad::prelude::*;

const SHIP_HEIGHT: f32 = 25.;
const SHIP_BASE: f32 = 22.;
const FIRE_RATE: f64 = 0.25; // seconds between shots when holding

struct Ship {
    pos: Vec2,
    rot: f32,
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
    StartMenu,
    Playing,
    Paused,
    GameOver,
    Win,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ControlMode {
    Keyboard,
    Touch,
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

fn new_game() -> (Ship, Vec<Bullet>, Vec<Asteroid>, f64) {
    let ship = Ship {
        pos: vec2(screen_width() / 2.0, screen_height() / 2.0),
        rot: 0.0,
        vel: Vec2::ZERO,
    };
    let bullets = Vec::new();
    let mut asteroids = Vec::new();
    let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
    for _ in 0..10 {
        asteroids.push(Asteroid {
            pos: center
                + vec2(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)).normalize()
                    * screen_width().min(screen_height())
                    / 2.0,
            vel: vec2(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)),
            rot: 0.0,
            rot_speed: rand::gen_range(-2., 2.),
            size: screen_width().min(screen_height()) / 10.0,
            sides: rand::gen_range(3u8, 8u8),
            collided: false,
        });
    }
    let last_shot = get_time();
    (ship, bullets, asteroids, last_shot)
}

fn conf() -> Conf {
    Conf {
        window_title: "Asteroids".to_string(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    // settle a few frames
    for _ in 0..3 {
        next_frame().await;
    }

    let (mut ship, mut bullets, mut asteroids, mut last_shot) = new_game();
    let mut game_state = GameState::StartMenu;
    let mut control_mode = ControlMode::Keyboard;

    loop {
        match game_state {
            GameState::StartMenu => {
                clear_background(LIGHTGRAY);
                let base = screen_width().min(screen_height());
                let fs_title = base * 0.07;
                let fs_prompt = base * 0.04;

                let welcome = "Asteroids";
                let prompt = "Press [Enter] or tap screen to start with touch";

                let ts_w = measure_text(welcome, None, fs_title as u16, 1.0);
                let ts_p = measure_text(prompt, None, fs_prompt as u16, 1.0);

                draw_text(
                    welcome,
                    screen_width() / 2.0 - ts_w.width / 2.0,
                    screen_height() / 2.0 - ts_w.height - 20.0,
                    fs_title,
                    DARKGRAY,
                );
                draw_text(
                    prompt,
                    screen_width() / 2.0 - ts_p.width / 2.0,
                    screen_height() / 2.0 + 20.0,
                    fs_prompt,
                    DARKGRAY,
                );

                if is_key_pressed(KeyCode::Enter) {
                    control_mode = ControlMode::Keyboard;
                    game_state = GameState::Playing;
                }
                if !touches().is_empty() {
                    control_mode = ControlMode::Touch;
                    game_state = GameState::Playing;
                }

                next_frame().await;
            }

            GameState::Playing => {
                let now = get_time();
                let mut acc = -ship.vel / 100.0;

                // Keyboard mode
                if control_mode == ControlMode::Keyboard {
                    if is_key_down(KeyCode::Left) {
                        ship.rot -= 5.0;
                    }
                    if is_key_down(KeyCode::Right) {
                        ship.rot += 5.0;
                    }
                    if is_key_down(KeyCode::Up) {
                        let ang = ship.rot.to_radians();
                        acc = vec2(ang.sin(), -ang.cos()) * 2.0;
                    }
                    if is_key_down(KeyCode::Space) && (now - last_shot > FIRE_RATE) {
                        let ang = ship.rot.to_radians();
                        let dir = vec2(ang.sin(), -ang.cos());
                        bullets.push(Bullet {
                            pos: ship.pos + dir * (SHIP_HEIGHT / 2.0),
                            vel: dir * 7.0,
                            shot_at: now,
                            collided: false,
                        });
                        last_shot = now;
                    }
                    if is_key_down(KeyCode::Escape) {
                        game_state = GameState::Paused;
                    }
                }

                // Touch mode (multitouch)
                if control_mode == ControlMode::Touch {
                    let scr_w = screen_width();
                    let scr_h = screen_height();
                    let btn_size = scr_w * 0.2;

                    let left_btn = Rect::new(0.0, scr_h - btn_size, btn_size, btn_size);
                    let right_btn =
                        Rect::new(scr_w - btn_size, scr_h - btn_size, btn_size, btn_size);
                    let thrust_btn = Rect::new(
                        (scr_w - btn_size) / 2.0,
                        scr_h - btn_size,
                        btn_size,
                        btn_size,
                    );

                    // We will track whether any touch is doing fire, thrust, or rotate
                    // Because touches() is a Vec<Touch>, multiple touches can exist
                    for touch in touches().iter() {
                        let p = touch.position;
                        let phase = touch.phase;

                        // If touch in left zone → rotate left
                        if left_btn.contains(p) {
                            ship.rot -= 3.0;
                        }
                        // If in right zone → rotate right
                        else if right_btn.contains(p) {
                            ship.rot += 3.0;
                        }
                        // If in thrust zone → thrust
                        else if thrust_btn.contains(p) {
                            let ang = ship.rot.to_radians();
                            acc = vec2(ang.sin(), -ang.cos()) * 0.5;
                        }
                        // Else (touch not on control buttons) → fire
                        else {
                            // continuous fire if holding or starting
                            if now - last_shot > FIRE_RATE {
                                let ang = ship.rot.to_radians();
                                let dir = vec2(ang.sin(), -ang.cos());
                                bullets.push(Bullet {
                                    pos: ship.pos + dir * (SHIP_HEIGHT / 2.0),
                                    vel: dir * 7.0,
                                    shot_at: now,
                                    collided: false,
                                });
                                last_shot = now;
                            }
                        }
                    }
                }

                // Physics / movement
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

                // Collision / Win / GameOver
                let mut new_asts = Vec::new();
                let mut collided_ship = false;
                for a in asteroids.iter_mut() {
                    if (a.pos - ship.pos).length() < a.size + SHIP_HEIGHT / 3.0 {
                        collided_ship = true;
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

                if collided_ship {
                    game_state = GameState::GameOver;
                } else {
                    asteroids.retain(|a| !a.collided);
                    asteroids.extend(new_asts);
                    if asteroids.is_empty() {
                        game_state = GameState::Win;
                    }
                }

                // Drawing
                clear_background(LIGHTGRAY);
                for b in bullets.iter() {
                    draw_circle(b.pos.x, b.pos.y, 2.0, BLACK);
                }
                for a in asteroids.iter() {
                    draw_poly_lines(a.pos.x, a.pos.y, a.sides, a.size, a.rot, 2.0, BLACK);
                }

                let ang = ship.rot.to_radians();
                let dir_f = vec2(ang.sin(), -ang.cos());
                let dir_l = vec2(-ang.cos(), -ang.sin());
                let dir_r = vec2(ang.cos(), ang.sin());

                let nose = ship.pos + dir_f * (SHIP_HEIGHT * 0.8);
                let back = ship.pos + dir_f * -(SHIP_HEIGHT * 0.3);
                let half_base = SHIP_BASE * 0.3;
                let v2 = back + dir_l * half_base;
                let v3 = back + dir_r * half_base;
                draw_triangle_lines(nose, v2, v3, 2.0, BLACK);

                if control_mode == ControlMode::Touch {
                    let scr_w = screen_width();
                    let scr_h = screen_height();
                    let btn_size = scr_w * 0.2;

                    let left_btn =
                        Rect::new(0.0, scr_h - btn_size, (scr_w / 2.) - btn_size, btn_size);
                    let right_btn = Rect::new(
                        scr_w - ((scr_w / 2.) - btn_size),
                        scr_h - btn_size,
                        (scr_w / 2.) - btn_size,
                        btn_size,
                    );
                    let thrust_btn = Rect::new(
                        (scr_w - btn_size) / 2.0,
                        scr_h - btn_size,
                        btn_size,
                        btn_size,
                    );

                    let alpha = 0.1;
                    draw_rectangle(
                        left_btn.x,
                        left_btn.y,
                        left_btn.w,
                        left_btn.h,
                        Color::new(0.0, 0.0, 0.0, alpha),
                    );
                    draw_rectangle(
                        right_btn.x,
                        right_btn.y,
                        right_btn.w,
                        right_btn.h,
                        Color::new(0.0, 0.0, 0.0, alpha),
                    );
                    draw_rectangle(
                        thrust_btn.x,
                        thrust_btn.y,
                        thrust_btn.w,
                        thrust_btn.h,
                        Color::new(0.0, 0.0, 0.0, alpha),
                    );

                    draw_rectangle_lines(
                        left_btn.x, left_btn.y, left_btn.w, left_btn.h, 1.0, WHITE,
                    );
                    draw_rectangle_lines(
                        right_btn.x,
                        right_btn.y,
                        right_btn.w,
                        right_btn.h,
                        1.0,
                        WHITE,
                    );
                    draw_rectangle_lines(
                        thrust_btn.x,
                        thrust_btn.y,
                        thrust_btn.w,
                        thrust_btn.h,
                        1.0,
                        WHITE,
                    );

                    let small = btn_size * 0.3;
                    draw_text(
                        "<",
                        left_btn.x + left_btn.w / 2.0 - small / 2.0,
                        left_btn.y + left_btn.h / 2.0 + small / 2.0,
                        small,
                        WHITE,
                    );
                    draw_text(
                        ">",
                        right_btn.x + right_btn.w / 2.0 - small / 2.0,
                        right_btn.y + right_btn.h / 2.0 + small / 2.0,
                        small,
                        WHITE,
                    );
                    draw_text(
                        "^",
                        thrust_btn.x + thrust_btn.w / 2.0 - small / 2.0,
                        thrust_btn.y + thrust_btn.h / 2.0 + small / 2.0,
                        small,
                        WHITE,
                    );

                    // Fire label
                    let fire_label = "Tap anywhere to FIRE";
                    let fs = screen_width().max(screen_height()) * 0.025;
                    let ts = measure_text(fire_label, None, fs as u16, 1.0);
                    draw_text(
                        fire_label,
                        scr_w / 2.0 - ts.width / 2.0,
                        scr_h - btn_size - 12.0,
                        fs,
                        WHITE,
                    );
                }

                next_frame().await;
            }

            GameState::Paused => {
                clear_background(LIGHTGRAY);
                let base = screen_width().min(screen_height());
                let fs = base * 0.06;
                let msg = "PAUSED — Press Enter to Resume";
                let ts = measure_text(msg, None, fs as u16, 1.0);
                draw_text(
                    msg,
                    screen_width() / 2.0 - ts.width / 2.0,
                    screen_height() / 2.0 - ts.height / 2.0,
                    fs,
                    DARKGRAY,
                );
                if is_key_pressed(KeyCode::Enter) {
                    game_state = GameState::Playing;
                }
                next_frame().await;
            }

            GameState::GameOver => {
                clear_background(LIGHTGRAY);
                let base = screen_width().min(screen_height());
                let fs = base * 0.06;
                let msg = if control_mode == ControlMode::Touch {
                    "Game Over! Tap to Restart"
                } else {
                    "Game Over! Press Enter to Restart"
                };
                let ts = measure_text(msg, None, fs as u16, 1.0);
                draw_text(
                    msg,
                    screen_width() / 2.0 - ts.width / 2.0,
                    screen_height() / 2.0 - ts.height / 2.0,
                    fs,
                    DARKGRAY,
                );
                if is_key_pressed(KeyCode::Enter) || !touches().is_empty() {
                    let (ns, nb, na, nls) = new_game();
                    ship = ns;
                    bullets = nb;
                    asteroids = na;
                    last_shot = nls;
                    game_state = GameState::StartMenu;
                }
                next_frame().await;
            }

            GameState::Win => {
                clear_background(LIGHTGRAY);
                let base = screen_width().min(screen_height());
                let fs = base * 0.06;
                let msg = if control_mode == ControlMode::Touch {
                    "You Win! Tap to Play Again"
                } else {
                    "You Win! Press Enter to Play Again"
                };
                let ts = measure_text(msg, None, fs as u16, 1.0);
                draw_text(
                    msg,
                    screen_width() / 2.0 - ts.width / 2.0,
                    screen_height() / 2.0 - ts.height / 2.0,
                    fs,
                    DARKGRAY,
                );
                if is_key_pressed(KeyCode::Enter) || !touches().is_empty() {
                    let (ns, nb, na, nls) = new_game();
                    ship = ns;
                    bullets = nb;
                    asteroids = na;
                    last_shot = nls;
                    game_state = GameState::StartMenu;
                }
                next_frame().await;
            }
        }
    }
}
