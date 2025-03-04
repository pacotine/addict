use crate::direction::Direction;

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
        let rect1 = self.rect();
        let rect2 = other.rect();

        let left = rect1.x.max(rect2.x);
        let right = (rect1.x + rect1.w).min(rect2.x + rect2.w);
        let top = rect1.y.max(rect2.y);
        let bottom = (rect1.y + rect1.h).min(rect2.y + rect2.h);
        let overlap_x = right - left;
        let overlap_y = bottom - top;

        if overlap_x > 0.0 && overlap_y > 0.0 {
            let max_speed = self.speed.max(other.speed);
            if overlap_x < overlap_y {
                if self.direction_x == other.direction_x {
                    if self.speed == max_speed { self.direction_x.switch(); }
                    if other.speed == max_speed { other.direction_x.switch(); }
                } else {
                    self.direction_x.switch();
                    other.direction_x.switch();
                }

                let dx = rect1.x - rect2.x;
                let separation = overlap_x / 2.0;
                if dx < 0.0 {
                    self.x -= separation;
                    other.x += separation;
                } else {
                    self.x += separation;
                    other.x -= separation;
                }
            } else {
                if self.direction_y == other.direction_y {
                    if self.speed == max_speed { self.direction_y.switch(); }
                    if other.speed == max_speed { other.direction_y.switch(); }
                } else {
                    self.direction_y.switch();
                    other.direction_y.switch();
                }

                let dy = rect1.y - rect2.y;
                let separation = overlap_y / 2.0;
                if dy < 0.0 {
                    self.y -= separation;
                    other.y += separation;
                } else {
                    self.y += separation;
                    other.y -= separation;
                }
            }
        }


        if self.is_killer { self.kill(other); }
        else if other.is_killer { other.kill(self); }
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