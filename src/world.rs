use crate::direction::Direction;
use crate::objects::{Entity, Weapon};

use macroquad::audio::load_sound;
use macroquad::color::WHITE;
use macroquad::math::{clamp, Rect, Vec2};
use macroquad::prelude::{draw_rectangle_lines, draw_texture_ex, load_image, rand, screen_height, screen_width, DrawTextureParams, Texture2D};

const INIT_SPEED: f32 = 200.0;
const SPEED_BOOST: f32 = 100.0;

pub struct World {
    pub entity1: Entity,
    pub entity2: Entity,
    sword: Weapon, //need to fix this and make a Vec<Weapon>
    rabbit: Weapon,
}

impl World {
    pub async fn new() -> World {
        let entity1 = Self::generate_entity("Ronaldo","assets/siuuu.wav", "assets/ronaldo.png").await;
        let entity2 = Self::generate_entity("Messi", "assets/messi.wav", "assets/messi.png").await;
        let mut world = Self {
            entity1,
            entity2,
            sword: Self::generate_weapon("assets/sword.png", Box::new(|e| e.is_killer = true)).await,
            rabbit: Self::generate_weapon("assets/rabbit.png", Box::new(|e| e.speed += SPEED_BOOST)).await,
        };
        world.init();

        world
    }

    pub fn edges(&self) -> Rect {
        Rect{
            x: 0.1*screen_width(), //x offset: 10% of screen width
            y: 0.1*screen_height(), //y offset: 10% of screen height
            w: 0.8*screen_width(), //top/bottom size: 80% of screen width
            h: 0.8*screen_height(), //left/right size: 80% of screen height
        }
    }

    pub fn draw(&mut self) {
        let edges = self.edges();
        let entity_size = self.entity_size();

        self.entity1.size = entity_size;
        self.entity2.size = entity_size;
        self.sword.size = entity_size / 2.0;
        self.rabbit.size = entity_size / 2.0;

        draw_rectangle_lines(edges.x, edges.y, edges.w, edges.h, 1.0, WHITE);
        self.entity1.draw(Some(Rect::new(self.entity1.texture.get_texture_data().width as f32 / 2.0 - 50.0, 0.0, 300.0, 300.0)));
        self.entity2.draw(Some(Rect::new(0.0, 0.0, 300.0, 300.0)));
    }

    pub fn update(&mut self) {
        self.set_weapons_catchable();
        self.spawn_weapons();

        self.update_entities();
    }

    pub fn entity_size(&self) -> f32 {
        0.1*self.edges().w
    }

    fn init(&mut self) {
        (self.entity1.x, self.entity1.y) = self.get_entity_start();
        (self.entity2.x, self.entity2.y) = self.get_entity_start();

        self.update();
    }

    fn update_entities(&mut self) {
        let edges = self.edges();

        //move entities
        self.entity1.move_frame();
        self.entity2.move_frame();

        //move the sword if caught
        if self.entity1.is_killer { self.entity1.attach_weapon(&self.sword); }
        else if self.entity2.is_killer { self.entity2.attach_weapon(&self.sword); }

        //check collisions
        if self.entity1.collides_with(&self.entity2) && self.entity1.speed <= self.entity2.speed {
            self.entity1.switch_direction(&mut self.entity2);
        } else if self.entity2.collides_with(&self.entity1) && self.entity2.speed <= self.entity1.speed {
            self.entity2.switch_direction(&mut self.entity1);
        }

        let temp_e1_x = self.entity1.x;
        let temp_e1_y = self.entity1.y;
        self.entity1.x = clamp(self.entity1.x, edges.x, edges.x + edges.w - self.entity1.size);
        self.entity1.y = clamp(self.entity1.y, edges.y, edges.y + edges.h - self.entity1.size);
        self.entity1.check_bounce(temp_e1_x, temp_e1_y);

        let temp_e2_x = self.entity2.x;
        let temp_e2_y = self.entity2.y;
        self.entity2.x = clamp(self.entity2.x, edges.x, edges.x + edges.w - self.entity2.size);
        self.entity2.y = clamp(self.entity2.y, edges.y, edges.y + edges.h - self.entity2.size);
        self.entity2.check_bounce(temp_e2_x, temp_e2_y);
    }

    fn set_weapons_catchable(&mut self) {
        if self.sword.is_placed { set_catchable(&mut self.sword, &mut self.entity1, &mut self.entity2); }
        if self.rabbit.is_placed { set_catchable(&mut self.rabbit, &mut self.entity1, &mut self.entity2); }
    }

    fn spawn_weapons(&mut self) {
        let edges = self.edges();

        if !(self.entity1.is_killer || self.entity2.is_killer) {
            roll_spawn(&mut self.sword, 1.0/200.0, edges);
        }
        let avg_speed = (self.entity1.speed + self.entity2.speed) / 2.0;
        roll_spawn(&mut self.rabbit, 1.0/avg_speed, edges);
    }

    fn get_entity_start(&self) -> (f32, f32) {
        let edges = self.edges();
        let entity_size = self.entity_size();
        let start_x = rand::gen_range(edges.x, edges.x+edges.w-entity_size);
        let start_y = rand::gen_range(edges.y, edges.y+edges.h-entity_size);

        (start_x, start_y)
    }

    async fn generate_entity(name: &str, sound_asset_path: &str, image_asset_path: &str) -> Entity {
        let sound = match load_sound(sound_asset_path).await {
            Ok(sound) => sound,
            Err(e) => panic!("Can't open audio: {e:?}")
        };
        let image = match load_image(image_asset_path).await {
            Ok(image) => image,
            Err(e) => panic!("Can't open image: {e:?}")
        };

        Entity {
            name: String::from(name),
            size: 0.0,
            speed: INIT_SPEED,
            direction_x: Direction::random(),
            direction_y: Direction::random(),
            x: 0.0,
            y: 0.0,
            texture: Texture2D::from_image(&image),
            sound,
            score: 10,
            is_killer: false,
        }
    }

    async fn generate_weapon(image_asset_path: &str, action: Box<dyn Fn(&mut Entity) -> ()>) -> Weapon {
        let asset = match load_image(image_asset_path).await {
            Ok(image) => image,
            Err(e) => panic!("Can't open image: {e:?}")
        };

        Weapon {
            size: 0.0,
            x: 0.0,
            y: 0.0,
            texture: Texture2D::from_image(&asset),
            is_placed: false,
            action,
        }
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

fn roll_spawn(weapon: &mut Weapon, probability: f32, edges: Rect) {
    if !weapon.is_placed && rand::gen_range(0.0, 1.0) < probability {
        weapon.is_placed = true;
        weapon.x = rand::gen_range(edges.x + weapon.size, edges.x + edges.w - weapon.size);
        weapon.y = rand::gen_range(edges.y + weapon.size, edges.y + edges.h - weapon.size);
    }
}