use macroquad::prelude::*;
use mq_test::si;
use si::{Alien, Ship};

// rows, columns
const ALIEN_GRID: (u16, u16) = (5, 11);
// left, right, top, bottom
const MARGIN: (f32, f32, f32, f32) = (0., 0., 20., 20.);
const ALIEN_SIZE: (u16, u16) = (33, 23);
const SHIP_SIZE: (f32, f32) = (33., 23.);

#[macroquad::main("Space Invaders")]
async fn main() {
    let mut ship = Ship {
        x_pos: (screen_width() / 2.).floor() as u16,
    };

    let alien_1_tex = load_texture("resources/alien_1_0.png").await.unwrap();

    let mut aliens = Vec::new();

    for r in 0_u16..ALIEN_GRID.0 {
        for c in 0_u16..ALIEN_GRID.1 {
            aliens.push(Alien {
                pos: U16Vec2::new(
                    c * (ALIEN_SIZE.0 + 10),
                    r * (ALIEN_SIZE.1 + 10),
                ),
                tex: &alien_1_tex,
                zapped: false,
            });
        }
    }

    let mut last_t = 0_f64;

    loop {
        clear_background(BLACK);
        let frame_t = get_time();

        draw_rectangle(
            ship.x_pos as f32,
            screen_height() - MARGIN.3 - SHIP_SIZE.1,
            SHIP_SIZE.0,
            SHIP_SIZE.1,
            ORANGE,
        );

        for alien in aliens.iter_mut() {
            draw_texture(
                alien.tex,
                alien.pos.x as f32,
                alien.pos.y as f32 + MARGIN.2 + ALIEN_SIZE.1 as f32,
                WHITE,
            );
            if alien.pos.x < screen_width() as u16 && frame_t > last_t + 1. {
                alien.pos.x += 10;
            }

            if alien.pos.x >= screen_width() as u16 {
                alien.pos.x = 0;
            }
        }

        if is_key_down(KeyCode::Right)
            && ship.x_pos <= (screen_width() - SHIP_SIZE.0 - MARGIN.1) as u16
        {
            ship.x_pos += 10;
        }

        if is_key_down(KeyCode::Left) && ship.x_pos >= (10. + MARGIN.0) as u16
        {
            ship.x_pos -= 10;
        }

        if frame_t > last_t + 1. {
            last_t = frame_t;
        }

        next_frame().await
    }
}
