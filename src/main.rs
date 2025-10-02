use macroquad::prelude::*;
use miniquad::window;

const SHIP_HEIGHT: f32 = 25.;
const SHIP_BASE: f32 = 22.;
const FIRE_RATE: f64 = 0.25;
const SCORE_MULTIPLIER: u8 = 5;

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
    color: Color,
}

struct ColorPalette {
    background: Color,
    ship: Color,
    asteroid_colors: Vec<Color>,
}

enum GameState {
    StartMenu,
    Playing,
    Paused,
    GameOver,
    Win,
    InfoScreen,
    Quit,
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

fn speed_factor(level_multiplier: f64) -> f32 {
    1.0 + (level_multiplier as f32) * 0.25
}

fn sample_palettes() -> Vec<ColorPalette> {
    vec![
        ColorPalette {
            background: color_u8!(10, 10, 30, 255),
            ship: color_u8!(0, 255, 255, 255),
            asteroid_colors: vec![
                color_u8!(255, 100, 255, 255), // bright magenta
                color_u8!(100, 255, 100, 255), // neon green
                color_u8!(255, 255, 100, 255), // soft yellow
                color_u8!(100, 255, 255, 255), // cyan
                color_u8!(255, 150, 50, 255),  // orange
                color_u8!(255, 50, 150, 255),  // pink
            ],
        },
        ColorPalette {
            background: color_u8!(40, 10, 10, 255),
            ship: color_u8!(255, 200, 150, 255),
            asteroid_colors: vec![
                color_u8!(255, 120, 40, 255),
                color_u8!(255, 60, 180, 255),
                color_u8!(255, 220, 60, 255),
                color_u8!(60, 255, 100, 255),
                color_u8!(220, 60, 255, 255),
                color_u8!(255, 80, 80, 255),
            ],
        },
        ColorPalette {
            background: color_u8!(10, 20, 40, 255),
            ship: color_u8!(220, 240, 255, 255),
            asteroid_colors: vec![
                color_u8!(200, 220, 255, 255),
                color_u8!(100, 255, 255, 255),
                color_u8!(255, 100, 255, 255),
                color_u8!(255, 255, 100, 255),
                color_u8!(120, 200, 255, 255),
                color_u8!(255, 120, 200, 255),
            ],
        },
        ColorPalette {
            background: color_u8!(5, 5, 25, 255),
            ship: color_u8!(180, 200, 255, 255),
            asteroid_colors: vec![
                color_u8!(180, 130, 230, 255),
                color_u8!(255, 60, 210, 255),
                color_u8!(220, 220, 80, 255),
                color_u8!(80, 255, 220, 255),
                color_u8!(210, 80, 255, 255),
                color_u8!(255, 180, 60, 255),
            ],
        },
        ColorPalette {
            background: color_u8!(30, 30, 10, 255),
            ship: color_u8!(255, 255, 200, 255),
            asteroid_colors: vec![
                color_u8!(255, 240, 120, 255),
                color_u8!(255, 60, 150, 255),
                color_u8!(60, 255, 180, 255),
                color_u8!(140, 0, 255, 255),
                color_u8!(255, 90, 30, 255),
                color_u8!(50, 200, 120, 255),
            ],
        },
        ColorPalette {
            background: color_u8!(15, 45, 15, 255),
            ship: color_u8!(200, 255, 200, 255),
            asteroid_colors: vec![
                color_u8!(180, 255, 180, 255),
                color_u8!(140, 255, 140, 255),
                color_u8!(220, 255, 180, 255),
                color_u8!(210, 200, 255, 255),
                color_u8!(170, 230, 200, 255),
                color_u8!(190, 240, 210, 255),
            ],
        },
        ColorPalette {
            background: color_u8!(50, 10, 50, 255),
            ship: color_u8!(255, 200, 255, 255),
            asteroid_colors: vec![
                color_u8!(255, 130, 255, 255),
                color_u8!(255, 60, 255, 255),
                color_u8!(255, 220, 240, 255),
                color_u8!(180, 80, 255, 255),
                color_u8!(255, 100, 200, 255),
                color_u8!(255, 150, 240, 255),
            ],
        },
        ColorPalette {
            background: color_u8!(20, 50, 50, 255),
            ship: color_u8!(200, 255, 255, 255),
            asteroid_colors: vec![
                color_u8!(160, 255, 255, 255),
                color_u8!(100, 255, 230, 255),
                color_u8!(255, 100, 255, 255),
                color_u8!(170, 230, 230, 255),
                color_u8!(150, 210, 210, 255),
                color_u8!(255, 50, 255, 255),
            ],
        },
        ColorPalette {
            background: color_u8!(25, 15, 45, 255),
            ship: color_u8!(240, 200, 255, 255),
            asteroid_colors: vec![
                color_u8!(255, 120, 200, 255),
                color_u8!(255, 60, 180, 255),
                color_u8!(255, 0, 255, 255),
                color_u8!(255, 110, 190, 255),
                color_u8!(255, 130, 210, 255),
                color_u8!(255, 150, 230, 255),
            ],
        },
        ColorPalette {
            background: color_u8!(40, 20, 5, 255),
            ship: color_u8!(255, 240, 200, 255),
            asteroid_colors: vec![
                color_u8!(255, 220, 100, 255),
                color_u8!(255, 0, 140, 255),
                color_u8!(255, 240, 140, 255),
                color_u8!(255, 210, 90, 255),
                color_u8!(255, 230, 110, 255),
                color_u8!(255, 245, 130, 255),
            ],
        },
        // Add more palettes as you like
    ]
}

fn pick_palette_for_level(level: f64, palettes: &[ColorPalette]) -> &ColorPalette {
    if (level - 1.0).abs() < std::f64::EPSILON {
        // first level: use the first palette always
        &palettes[0]
    } else {
        let idx = rand::gen_range(0, palettes.len() as i32) as usize;
        &palettes[idx]
    }
}

fn random_asteroid_color(palette: &ColorPalette) -> Color {
    let i = rand::gen_range(0, palette.asteroid_colors.len() as i32) as usize;
    palette.asteroid_colors[i]
}

fn new_game(
    level_multiplier: f64,
    palette: &ColorPalette,
) -> (Ship, Vec<Bullet>, Vec<Asteroid>, f64, u64) {
    let ship = Ship {
        pos: vec2(screen_width() / 2.0, screen_height() / 2.0),
        rot: 0.0,
        vel: Vec2::ZERO,
    };
    let bullets = Vec::new();
    let mut asteroids = Vec::new();
    let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let factor = speed_factor(level_multiplier);
    for _ in 0..(10. + (level_multiplier * 2.)) as u32 {
        let base_vel = vec2(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.));
        let norm = if base_vel.length() == 0. {
            vec2(1.0, 0.0)
        } else {
            base_vel.normalize()
        };
        asteroids.push(Asteroid {
            pos: center
                + vec2(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)).normalize()
                    * screen_width().min(screen_height())
                    / 2.0,
            vel: norm * factor,
            rot: 0.0,
            rot_speed: rand::gen_range(-2., 2.),
            size: screen_width().min(screen_height()) / 10.0,
            sides: rand::gen_range(3u8, 8u8),
            collided: false,
            color: random_asteroid_color(palette),
        });
    }
    let last_shot = get_time();
    let player_score = 0;
    (ship, bullets, asteroids, last_shot, player_score)
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
    for _ in 0..3 {
        next_frame().await;
    }

    let palettes = sample_palettes();
    let mut level_multiplier = 1.0;
    let mut current_palette = pick_palette_for_level(level_multiplier, &palettes);
    let (mut ship, mut bullets, mut asteroids, mut last_shot, mut player_score) =
        new_game(level_multiplier, current_palette);
    let mut game_state = GameState::StartMenu;
    let mut control_mode = ControlMode::Keyboard;

    loop {
        match game_state {
            GameState::StartMenu => {
                clear_background(LIGHTGRAY);
                let base = screen_width().min(screen_height());
                let fs_title = base * 0.05;
                let fs_prompt = base * 0.04;

                let welcome = "Asteroids - Lovingly cloned by patdalcia <3";
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
                    game_state = GameState::InfoScreen;
                }
                if !touches().is_empty() {
                    control_mode = ControlMode::Touch;
                    game_state = GameState::InfoScreen;
                }

                next_frame().await;
            }

            GameState::Playing => {
                let now = get_time();
                let mut acc = -ship.vel / 100.0;

                if control_mode == ControlMode::Keyboard {
                    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                        ship.rot -= 5.0;
                    }
                    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                        ship.rot += 5.0;
                    }
                    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
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

                if control_mode == ControlMode::Touch {
                    let scr_w = screen_width();
                    let scr_h = screen_height();
                    let btn_size = scr_w * 0.2;
                    let rotation_btn_w = scr_w / 4.;

                    let left_btn = Rect::new(0.0, scr_h - btn_size, rotation_btn_w, btn_size);
                    let right_btn =
                        Rect::new(rotation_btn_w, scr_h - btn_size, rotation_btn_w, btn_size);
                    let thrust_btn = Rect::new(scr_w / 2., scr_h - btn_size, scr_w / 2., btn_size);
                    let pause_btn = Rect::new(scr_w / 8., scr_h - btn_size, scr_w / 6., btn_size);

                    // Auto fire for touch
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

                    for touch in touches().iter() {
                        let p = touch.position;
                        if left_btn.contains(p) {
                            ship.rot -= 3.0;
                        } else if right_btn.contains(p) {
                            ship.rot += 3.0;
                        } else if thrust_btn.contains(p) {
                            let ang = ship.rot.to_radians();
                            acc = vec2(ang.sin(), -ang.cos()) * 0.25;
                        } else if pause_btn.contains(p) {
                            game_state = GameState::Paused;
                        }

                        // Uncomment to enable tap to shoot
                        // else if now - last_shot > FIRE_RATE {
                        //     let ang = ship.rot.to_radians();
                        //     let dir = vec2(ang.sin(), -ang.cos());
                        //     bullets.push(Bullet {
                        //         pos: ship.pos + dir * (SHIP_HEIGHT / 2.0),
                        //         vel: dir * 7.0,
                        //         shot_at: now,
                        //         collided: false,
                        //     });
                        //     last_shot = now;
                        // }
                    }
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

                    let max_speed = 5.0 + (level_multiplier as f32) * 0.5;
                    if a.vel.length() > max_speed {
                        a.vel = a.vel.normalize() * max_speed;
                    }
                }

                bullets.retain(|b| b.shot_at + 1.5 > now && !b.collided);

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
                            let side_count = a.sides;
                            player_score += side_count as u64 * SCORE_MULTIPLIER as u64;
                            if side_count > 3 {
                                new_asts.push(Asteroid {
                                    pos: a.pos,
                                    vel: vec2(b.vel.y, -b.vel.x).normalize()
                                        * speed_factor(level_multiplier),
                                    rot: rand::gen_range(0., 360.),
                                    rot_speed: rand::gen_range(-2., 2.),
                                    size: a.size * 0.8,
                                    sides: a.sides - 1,
                                    collided: false,
                                    color: random_asteroid_color(current_palette),
                                });
                                new_asts.push(Asteroid {
                                    pos: a.pos,
                                    vel: vec2(-b.vel.y, b.vel.x).normalize()
                                        * speed_factor(level_multiplier),
                                    rot: rand::gen_range(0., 360.),
                                    rot_speed: rand::gen_range(-2., 2.),
                                    size: a.size * 0.8,
                                    sides: a.sides - 1,
                                    collided: false,
                                    color: random_asteroid_color(current_palette),
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
                let ang = ship.rot.to_radians();
                let dir_f = vec2(ang.sin(), -ang.cos());
                let dir_l = vec2(-ang.cos(), -ang.sin());
                let dir_r = vec2(ang.cos(), ang.sin());

                let nose = ship.pos + dir_f * (SHIP_HEIGHT * 0.8);
                let back = ship.pos + dir_f * -(SHIP_HEIGHT * 0.3);
                let half_base = SHIP_BASE * 0.3;
                let v2 = back + dir_l * half_base;
                let v3 = back + dir_r * half_base;

                // Forcing Base Color pallette for first level.
                if level_multiplier == 1. {
                    clear_background(LIGHTGRAY);
                    for b in bullets.iter() {
                        draw_circle(b.pos.x, b.pos.y, 2.0, BLACK);
                    }
                    for a in asteroids.iter() {
                        draw_poly_lines(a.pos.x, a.pos.y, a.sides, a.size, a.rot, 2.0, BLACK);
                    }
                    draw_triangle_lines(nose, v2, v3, 2.0, BLACK);
                }
                // Random color pallette per level
                else {
                    clear_background(current_palette.background);
                    for b in bullets.iter() {
                        draw_circle(b.pos.x, b.pos.y, 2.0, current_palette.ship);
                    }
                    for a in asteroids.iter() {
                        draw_poly_lines(a.pos.x, a.pos.y, a.sides, a.size, a.rot, 2.0, a.color);
                    }
                    draw_triangle_lines(nose, v2, v3, 2.0, current_palette.ship);
                }

                let base = screen_width().min(screen_height());
                let score_and_level_fs = base * 0.04;
                let alpha = 0.75;

                // Drawing Score
                let msg = player_score.to_string();
                let ts = measure_text(msg.as_str(), None, score_and_level_fs as u16, 1.0);
                draw_text(
                    msg.as_str(),
                    screen_width() / 4.0,
                    ts.height * 2.0,
                    score_and_level_fs,
                    DARKGRAY.with_alpha(alpha),
                );

                // Drawing Level
                let msg = format!("Level: {level_multiplier}");
                let ts = measure_text(msg.as_str(), None, score_and_level_fs as u16, 1.0);
                draw_text(
                    msg.as_str(),
                    screen_width() * 0.75,
                    ts.height * 2.0,
                    score_and_level_fs,
                    DARKGRAY.with_alpha(alpha),
                );

                // Drawing touch controls
                if control_mode == ControlMode::Touch {
                    let scr_w = screen_width();
                    let scr_h = screen_height();
                    let btn_size = scr_w * 0.2;
                    let rotation_btn_w = scr_w / 4.;

                    let left_btn = Rect::new(0.0, scr_h - btn_size, rotation_btn_w, btn_size);
                    let right_btn =
                        Rect::new(rotation_btn_w, scr_h - btn_size, rotation_btn_w, btn_size);
                    let thrust_btn = Rect::new(scr_w / 2.0, scr_h - btn_size, scr_w / 2., btn_size);
                    let pause_btn = Rect::new(scr_w / 8., scr_h - btn_size, scr_w / 6., btn_size);

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
                    draw_rectangle(
                        pause_btn.x,
                        pause_btn.y,
                        pause_btn.w,
                        pause_btn.h,
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
                    draw_text(
                        "PAUSE",
                        pause_btn.x + pause_btn.w / 2.0 - small / 2.0,
                        pause_btn.y + pause_btn.h / 2.0 + small / 2.0,
                        small,
                        WHITE,
                    );

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
                let fs = base * 0.05;
                let fs2 = base * 0.04;
                let msg = "PAUSED";
                let msg2 = "Press [enter] to Resume";
                let ts = measure_text(msg, None, fs as u16, 1.0);
                let ts2 = measure_text(msg2, None, fs2 as u16, 1.0);
                draw_text(
                    msg,
                    screen_width() / 2.0 - ts.width / 2.0,
                    screen_height() / 2.0 - ts.height - 20.0,
                    fs,
                    DARKGRAY,
                );
                draw_text(
                    msg2,
                    screen_width() / 2.0 - ts2.width / 2.0,
                    screen_height() / 2.0 + 20.0,
                    fs2,
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

                let fs = base * 0.05;
                let fs2 = base * 0.04;
                let msg = format!("GAME OVER -> FINAL SCORE: {player_score}");
                let msg2 = if control_mode == ControlMode::Touch {
                    "Tap to Restart"
                } else {
                    "Press [enter] to Restart"
                };
                let ts = measure_text(msg.as_str(), None, fs as u16, 1.0);
                let ts2 = measure_text(msg2, None, fs2 as u16, 1.0);
                draw_text(
                    msg.as_str(),
                    screen_width() / 2.0 - ts.width / 2.0,
                    screen_height() / 2.0 - ts.height - 20.0,
                    fs,
                    DARKGRAY,
                );
                draw_text(
                    msg2,
                    screen_width() / 2.0 - ts2.width / 2.0,
                    screen_height() / 2.0 + 20.0,
                    fs2,
                    DARKGRAY,
                );
                if is_key_pressed(KeyCode::Enter) || !touches().is_empty() {
                    level_multiplier = 1.0;
                    current_palette = pick_palette_for_level(level_multiplier, &palettes);
                    let (ns, nb, na, nls, ps) = new_game(level_multiplier, current_palette);
                    ship = ns;
                    bullets = nb;
                    asteroids = na;
                    last_shot = nls;
                    game_state = GameState::StartMenu;
                    player_score = ps;
                }
                next_frame().await;
            }

            GameState::Win => {
                clear_background(current_palette.background);
                let base = screen_width().min(screen_height());
                let fs = base * 0.05;
                let msg = if control_mode == ControlMode::Touch {
                    "You Win! Tap to Move To Next Level"
                } else {
                    "You Win! Press Enter to Move To Next Level"
                };
                let ts = measure_text(msg, None, fs as u16, 1.0);
                draw_text(
                    msg,
                    screen_width() / 2.0 - ts.width / 2.0,
                    screen_height() / 2.0 - ts.height - 20.0,
                    fs,
                    current_palette.ship,
                );
                if is_key_pressed(KeyCode::Enter) {
                    level_multiplier += 1.0;
                    current_palette = pick_palette_for_level(level_multiplier, &palettes);
                    let (ns, nb, na, nls, _ps) = new_game(level_multiplier, current_palette);
                    ship = ns;
                    bullets = nb;
                    asteroids = na;
                    last_shot = nls;
                    game_state = GameState::Playing;
                }
                for touch in touches() {
                    if touch.phase == TouchPhase::Started {
                        level_multiplier += 1.0;
                        current_palette = pick_palette_for_level(level_multiplier, &palettes);
                        let (ns, nb, na, nls, _ps) = new_game(level_multiplier, current_palette);
                        ship = ns;
                        bullets = nb;
                        asteroids = na;
                        last_shot = nls;
                        game_state = GameState::Playing;
                        break;
                    }
                }
                next_frame().await;
            }

            GameState::InfoScreen => {
                let curr_color = if level_multiplier == 1. {
                    (LIGHTGRAY, DARKGRAY)
                } else {
                    (current_palette.background, current_palette.ship)
                };
                clear_background(curr_color.0);
                let base = screen_width().min(screen_height());
                let fs = base * 0.05;
                let fs2 = base * 0.04;
                let msg = if control_mode == ControlMode::Touch {
                    "Move with [buttons] ship will autofire."
                } else {
                    "Move with [wasd] or [arrows] fire with [space]."
                };
                let msg2 = if control_mode == ControlMode::Touch {
                    "[Tap Screen To Start Game]"
                } else {
                    "[Press Enter To Start Game]"
                };
                let ts = measure_text(msg, None, fs as u16, 1.0);
                let ts2 = measure_text(msg2, None, fs2 as u16, 1.0);

                draw_text(
                    msg,
                    screen_width() / 2.0 - ts.width / 2.0,
                    screen_height() / 2.0 - ts.height - 20.0,
                    fs,
                    curr_color.1,
                );
                draw_text(
                    msg2,
                    screen_width() / 2.0 - ts2.width / 2.0,
                    screen_height() / 2.0 + 20.0,
                    fs2,
                    curr_color.1,
                );
                for touch in touches() {
                    if touch.phase == TouchPhase::Started {
                        current_palette = pick_palette_for_level(level_multiplier, &palettes);
                        let (ns, nb, na, nls, ps) = new_game(level_multiplier, current_palette);
                        ship = ns;
                        bullets = nb;
                        asteroids = na;
                        last_shot = nls;
                        control_mode = ControlMode::Touch;
                        game_state = GameState::Playing;
                        player_score = ps;
                        break;
                    }
                }
                if is_key_pressed(KeyCode::Enter) {
                    current_palette = pick_palette_for_level(level_multiplier, &palettes);
                    let (ns, nb, na, nls, ps) = new_game(level_multiplier, current_palette);
                    ship = ns;
                    bullets = nb;
                    asteroids = na;
                    last_shot = nls;
                    control_mode = ControlMode::Keyboard;
                    game_state = GameState::Playing;
                    player_score = ps;
                }
                next_frame().await;
            }

            GameState::Quit => {
                window::order_quit();
                break;
            }
        }
    }
}
