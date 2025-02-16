use crate::world::Direction;
use macroquad::audio::{play_sound, PlaySoundParams, Sound};
use macroquad::color::{Color, WHITE};
use macroquad::math::{Rect, Vec2};
use macroquad::prelude::{draw_texture_ex, get_frame_time, DrawTextureParams, Texture2D};

pub struct Entity {
    pub name: String,
    pub size: f32,
    pub speed: f32,
    pub direction_x: Direction,
    pub direction_y: Direction,
    pub x: f32,
    pub y: f32,
    pub texture: Texture2D,
    pub sound: Sound,
    pub score: i8,
    pub is_killer: bool,
}

impl Entity {
    pub fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    pub fn catch_weapon(&self, weapon: &Weapon) -> bool {
        self.rect().overlaps(&weapon.rect())
    }

    pub fn switch_direction(&mut self, other: &mut Self) {
        let other_rect = other.rect();
        let left_edge = Rect{
            x: self.rect().left(),
            y: self.rect().top(),
            w: 1.0,
            h: self.size,
        };
        let right_edge = Rect{
            x: self.rect().right(),
            y: self.rect().top(),
            w: 1.0,
            h: self.size,
        };
        let top_edge = Rect{
            x: self.rect().left(),
            y: self.rect().top(),
            w: self.size,
            h: 1.0,
        };
        let bottom_edge = Rect{
            x: self.rect().left(),
            y: self.rect().bottom(),
            w: self.size,
            h: 1.0,
        };

        if other_rect.overlaps(&top_edge) {
            //draw_rectangle_lines(top_edge.x, top_edge.y, top_edge.w, top_edge.h, 1.0, RED);
            other.direction_y.switch();
            if other.direction_y == self.direction_y { self.direction_y.switch(); }
        }
        else if other_rect.overlaps(&bottom_edge) {
            //draw_rectangle_lines(bottom_edge.x, bottom_edge.y, bottom_edge.w, bottom_edge.h, 1.0, RED);
            other.direction_y.switch();
            if other.direction_y == self.direction_y { self.direction_y.switch(); } //well...no?
        }
        if other_rect.overlaps(&left_edge) {
            //draw_rectangle_lines(left_edge.x, left_edge.y, left_edge.w, left_edge.h, 1.0, GREEN);
            other.direction_x.switch();
            if other.direction_x == self.direction_x { self.direction_x.switch(); }
        }
        else if other_rect.overlaps(&right_edge) {
            //draw_rectangle_lines(right_edge.x, right_edge.y, right_edge.w, right_edge.h, 1.0, GREEN);
            other.direction_x.switch();
            if other.direction_x == self.direction_x { self.direction_x.switch(); }
        }

        if self.is_killer { self.kill(other); }
        else if other.is_killer { other.kill(self); }

        if self.rect().overlaps(&other.rect()) {
            self.move_frame();
            other.move_frame();
        }
    }

    pub fn move_frame(&mut self) {
        let delta = get_frame_time();
        self.x += delta * self.speed * self.direction_x.value();
        self.y += delta * self.speed * self.direction_y.value();
    }

    pub fn attach_weapon(&self, weapon: &Weapon) {
        let is_ltr = self.direction_x.value() > 0.0;
        draw_texture_ex(&weapon.texture,
                        if is_ltr
                        { self.rect().right() }
                        else { self.rect().left()-weapon.size },
                        self.rect().top()+weapon.size/2.0,
                        WHITE, DrawTextureParams {
                dest_size: Some(Vec2::new(weapon.size, weapon.size)), flip_x: !is_ltr,
                ..DrawTextureParams::default()
            });
    }

    pub fn check_bounce(&mut self, prev_x: f32, prev_y: f32) {
        if self.is_direction_x(Direction::Pos) && self.x < prev_x {
            self.direction_x = Direction::Neg;
        } else if self.is_direction_x(Direction::Neg) && self.x > prev_x {
            self.direction_x = Direction::Pos;
        }

        if self.is_direction_y(Direction::Pos) && self.y < prev_y {
            self.direction_y = Direction::Neg;
        } else if self.is_direction_y(Direction::Neg) && self.y > prev_y {
            self.direction_y = Direction::Pos;
        }
    }

    pub fn draw(&self, source: Option<Rect>) {
        let gradient = (self.score as f32) / 10.0;
            draw_texture_ex(&self.texture, self.x, self.y, Color::new(1.0, gradient, gradient, 1.0),
    DrawTextureParams {
                dest_size: Some(Vec2::new(self.size, self.size)),
                source, //= which part of the image?
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
        });
    }

    fn rect(&self) -> Rect {
        Rect {
            x: self.x,
            y: self.y,
            w: self.size,
            h: self.size,
        }
    }

    fn kill(&mut self, other: &mut Entity) {
        play_sound(&self.sound, PlaySoundParams{
            looped: false,
            volume: 1.0,
        });
        other.score -= 1;
        self.is_killer = false;
    }

    fn is_direction_x(&self, direction: Direction) -> bool {
        self.direction_x.value() == direction.value()
    }

    fn is_direction_y(&self, direction: Direction) -> bool {
        self.direction_y.value() == direction.value()
    }
}

pub struct Weapon {
    pub size: f32,
    pub x: f32,
    pub y: f32,
    pub texture: Texture2D,
    pub is_placed: bool,
    pub action: Box<dyn Fn(&mut Entity) -> ()>,
}

impl Weapon {
    pub fn on_catch(&mut self, with: &mut Entity) {
        (self.action)(with);
        self.is_placed = false;
    }

    fn rect(&self) -> Rect {
        Rect {
            x: self.x,
            y: self.y,
            w: self.size,
            h: self.size,
        }
    }
}