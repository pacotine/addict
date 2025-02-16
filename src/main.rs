mod world;
mod objects;

use macroquad::audio::load_sound;
use macroquad::input::KeyCode::Space;
use macroquad::miniquad::date::now;
use macroquad::prelude::*;
use objects::Entity;
use objects::Weapon;
use world::Direction;

const INIT_SPEED: f32 = 200.0;
const FONT_SIZE: f32 = 14.0;

fn conf() -> Conf {
    Conf{
        window_title: "Addict".to_string(),
        window_width: 1920,
        window_height: 1920,
        high_dpi: false,
        fullscreen: false,
        ..Default::default()
    }
}

fn set_catchable(weapon: &mut Weapon, entity1: &mut Entity, entity2: &mut Entity) {
    draw_texture_ex(&weapon.texture, weapon.x, weapon.y, WHITE, DrawTextureParams {
        dest_size: Some(Vec2::new(weapon.size, weapon.size)),
        ..DrawTextureParams::default()
    });
    if entity1.catch_weapon(weapon) {
        weapon.on_catch(entity1);
    }
    if entity2.catch_weapon(weapon) {
        weapon.on_catch(entity2);
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

    //load assets
    let sound1 = match load_sound("assets/siuuu.wav").await {
        Ok(sound) => sound,
        Err(e) => panic!("Can't open audio: {e:?}")
    }; //load sound 1

    let sound2 = match load_sound("assets/messi.wav").await {
        Ok(sound) => sound,
        Err(e) => panic!("Can't open audio: {e:?}")
    }; //load sound 2

    let image1 = match Image::from_file_with_format(
        include_bytes!("../assets/ronaldo.png"),
        Some(ImageFormat::Png),
    ) {
        Ok(image) => image,
        Err(e) => panic!("Can't open image: {e:?}")
    }; //load image 1

    let image2 = match Image::from_file_with_format(
        include_bytes!("../assets/messi.png"),
        Some(ImageFormat::Png),
    ) {
        Ok(image) => image,
        Err(e) => panic!("Can't open image: {e:?}")
    }; //load image 2

    let sword = match Image::from_file_with_format(
        include_bytes!("../assets/sword.png"),
        Some(ImageFormat::Png),
    ) {
        Ok(image) => image,
        Err(e) => panic!("Can't open image: {e:?}")
    };

    let rabbit = match Image::from_file_with_format(
        include_bytes!("../assets/rabbit.png"),
        Some(ImageFormat::Png),
    ) {
        Ok(image) => image,
        Err(e) => panic!("Can't open image: {e:?}")
    };

    let texture1 = Texture2D::from_image(&image1); //create texture for image 1
    let texture2 = Texture2D::from_image(&image2); //create texture for image 2
    let texture_sword = Texture2D::from_image(&sword); //create texture for sword
    let texture_rabbit = Texture2D::from_image(&rabbit); //create texture for rabbit

    //init. edges
    let edges = Rect{
        x: 0.1*screen_width(), //x offset: 10% of screen width
        y: 0.1*screen_height(), //y offset: 10% of screen height
        w: 0.8*screen_width(), //top/bottom size: 80% of screen width
        h: 0.8*screen_height(), //left/right size: 80% of screen height
    };
    let entity_size = 0.1*edges.w; //10% of edge width

    let start_x1 = rand::gen_range(edges.x, edges.x+edges.w-entity_size);
    let start_y1 = rand::gen_range(edges.y, edges.y+edges.h-entity_size);
    let start_x2 = rand::gen_range(edges.x, edges.x+edges.w-entity_size);
    let start_y2 = rand::gen_range(edges.y, edges.y+edges.h-entity_size);
    /*
    let start_x1 = edges.x+100.0;
    let start_y1 = edges.y+100.0;
    let start_x2 = edges.x+100.0+entity_size*2.0;
    let start_y2 = edges.y+100.0;
     */
    let mut entity1 = Entity {
        name: String::from("Ronaldo"),
        size: entity_size,
        speed: INIT_SPEED,
        direction_x: Direction::random(),
        direction_y: Direction::random(),
        x: start_x1,
        y: start_y1,
        texture: texture1,
        sound: sound1,
        score: 10,
        is_killer: false,
    };
    let mut entity2 = Entity {
        name: String::from("Messi"),
        size: entity_size,
        speed: INIT_SPEED,
        direction_x: Direction::random(),
        direction_y: Direction::random(),
        x: start_x2,
        y: start_y2,
        texture: texture2,
        sound: sound2,
        score: 10,
        is_killer: false,
    };
    let mut sword = Weapon {
        size: entity_size/2.0,
        x: 0.0,
        y: 0.0,
        texture: texture_sword,
        is_placed: false,
        action: Box::new(|e| e.is_killer = true)
    };
    let mut speed = Weapon {
        size: entity_size/2.0,
        x: 0.0,
        y: 0.0,
        texture: texture_rabbit,
        is_placed: false,
        action: Box::new(|e| e.speed += 100.0)
    };

    let mut play = false;

    loop {
        if entity1.score == 0 || entity2.score == 0 {
            let winner = if entity1.score == 0 { &entity2 } else { &entity1 };
            show_winner(winner);
        } else {
            if is_key_released(Space) { play = !play; } //resume/pause
            clear_background(BLACK);

            let edges = Rect {
                x: 0.1 * screen_width(),
                y: 0.1 * screen_height(),
                w: 0.8 * screen_width(),
                h: 0.8 * screen_height(),
            }; //update edges
            let entity_size = 0.1 * edges.w; //update size of an entity

            entity1.size = entity_size;
            entity2.size = entity_size;
            sword.size = entity_size / 2.0;
            speed.size = entity_size / 2.0;

            draw_rectangle_lines(edges.x, edges.y, edges.w, edges.h, 1.0, WHITE); //draw edges

            //draw entities
            entity1.draw(Some(Rect::new(image1.width as f32 / 2.0 - 50.0, 0.0, 300.0, 300.0)));
            entity2.draw(Some(Rect::new(0.0, 0.0, 300.0, 300.0)));

            //set weapons catchable if generated
            if sword.is_placed { set_catchable(&mut sword, &mut entity1, &mut entity2); }
            if speed.is_placed { set_catchable(&mut speed, &mut entity1, &mut entity2); }

            if play {
                //generate weapons
                if !sword.is_placed && !(entity1.is_killer || entity2.is_killer) && rand::gen_range(1, 200) == 50 { //1/200
                    sword.is_placed = true;
                    sword.x = rand::gen_range(edges.x + sword.size, edges.x + edges.w - sword.size);
                    sword.y = rand::gen_range(edges.y + sword.size, edges.y + edges.h - sword.size);
                }
                if !speed.is_placed && rand::gen_range(1, ((entity1.speed + entity2.speed)/2.0) as i32) == 50 {
                    speed.is_placed = true;
                    speed.x = rand::gen_range(edges.x + speed.size, edges.x + edges.w - speed.size);
                    speed.y = rand::gen_range(edges.y + speed.size, edges.y + edges.h - speed.size);
                }

                //move entities
                entity1.move_frame();
                entity2.move_frame();

                //move the sword if caught
                if entity1.is_killer { entity1.attach_weapon(&sword); }
                else if entity2.is_killer { entity2.attach_weapon(&sword); }

                //check collisions
                if entity1.collides_with(&entity2) && entity1.speed <= entity2.speed {
                    entity1.switch_direction(&mut entity2);
                } else if entity2.collides_with(&entity1) && entity2.speed <= entity1.speed {
                    entity2.switch_direction(&mut entity1);
                }

                //check if entities are in the box + bounce
                let temp_e1_x = entity1.x;
                let temp_e1_y = entity1.y;
                entity1.x = clamp(entity1.x, edges.x, edges.x + edges.w - entity1.size);
                entity1.y = clamp(entity1.y, edges.y, edges.y + edges.h - entity1.size);
                entity1.check_bounce(temp_e1_x, temp_e1_y);

                let temp_e2_x = entity2.x;
                let temp_e2_y = entity2.y;
                entity2.x = clamp(entity2.x, edges.x, edges.x + edges.w - entity2.size);
                entity2.y = clamp(entity2.y, edges.y, edges.y + edges.h - entity2.size);
                entity2.check_bounce(temp_e2_x, temp_e2_y);

                //text
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
            } //end resume
        }
        next_frame().await
    }
}