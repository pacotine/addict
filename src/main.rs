mod world;
mod objects;
mod direction;

use macroquad::input::KeyCode::Space;
use macroquad::miniquad::date::now;
use macroquad::prelude::*;
use objects::Entity;
use world::World;

const FONT_SIZE: f32 = 14.0;

fn conf() -> Conf {
    Conf {
        window_title: "Addict".to_string(),
        window_width: 1920,
        window_height: 1920,
        high_dpi: false,
        fullscreen: false,
        ..Default::default()
    }
}

fn show_winner(winner: &Entity) {
    let winner_text = format!("Winner: {}", winner.name);
    let dim_winner = measure_text(winner_text.as_str(), None, 20, 1.0);
    let winner_texture = &winner.texture;
    let winner_texture_size = winner_texture.size()/2.0;
    let winner_texture_x = screen_width()/2.0-winner_texture_size.x/2.0;
    let winner_texture_y = screen_height()/2.0-winner_texture_size.y/2.0-dim_winner.height*2.0;
    draw_texture_ex(&winner_texture,
                    winner_texture_x,
                    winner_texture_y,
                    WHITE, DrawTextureParams {
                        dest_size: Some(winner_texture_size),
                        ..DrawTextureParams::default()
                    }
    );
    draw_text(winner_text.as_str(),
              winner_texture_x + winner_texture_size.x + dim_winner.width/2.0,
              winner_texture_y + (winner_texture_size.y)/2.0,
              FONT_SIZE+0.1*screen_height()/2.0, WHITE
    );
}

#[macroquad::main(conf)]
async fn main() {
    //random seed
    rand::srand(now() as u64);

    let mut world = World::new().await;

    let mut play = false;

    loop {
        if world.entity1.score == 0 || world.entity2.score == 0 {
            let winner = if world.entity1.score == 0 { &world.entity2 } else { &world.entity1 };
            show_winner(winner);
        } else {
            if is_key_released(Space) { play = !play; } //resume/pause
            clear_background(BLACK);

            world.draw();

            if play {
                world.update();

                world.update_text();
            } //end resume
        }
        next_frame().await
    }
}