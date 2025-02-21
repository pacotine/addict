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

fn update_text(edges: Rect, entity1: &Entity, entity2: &Entity) {
    let score_entity = format!("{} (speed {}km/h): {}", entity1.name, entity1.speed, entity1.score);
    let font_size = FONT_SIZE * screen_width() / 960.0;
    let dim_score_e1 = measure_text(score_entity.as_str(), None, font_size as u16, 1.0);
    draw_text(
        score_entity.as_str(),
        edges.x + edges.w / 2.0 - dim_score_e1.width / 2.0,
        edges.y / 2.0 - dim_score_e1.height,
        font_size,
        WHITE
    );
    let score_entity = format!("{} (speed {}km/s): {}", entity2.name, entity2.speed, entity2.score);
    let dim_score_e2 = measure_text(score_entity.as_str(), None, font_size as u16, 1.0);
    draw_text(
        score_entity.as_str(),
        edges.x + edges.w / 2.0 - dim_score_e2.width / 2.0,
        edges.y / 2.0 - dim_score_e2.height + dim_score_e1.height * 2.0,
        font_size,
        WHITE
    );
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

                update_text(world.edges(), &world.entity1, &world.entity2);
            } //end resume
        }
        next_frame().await
    }
}