use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Depart {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
pub struct Car {
    pub color: Texture2D,
    pub x: f32,
    pub y: f32,
    pub depart: Depart,
    pub direction: Direction,
}


use macroquad::prelude::*;

impl Car {
    pub fn draw(&self) {
        let rotation = match self.depart {
            Depart::Down => 0.0_f32,
            Depart::Up => 180.0_f32,
            Depart::Right => 270.0_f32,
            Depart::Left => 90.0_f32,
        }
        .to_radians();

        draw_texture_ex(
            &self.color,
            self.x,
            self.y,
            WHITE,
            DrawTextureParams {
                rotation,
                ..Default::default()
            },
        );
    }

    pub fn update(&mut self) {
        match self.depart {
            Depart::Up => self.y += 1.0,
            Depart::Down => self.y -= 1.0,
            Depart::Left => self.x += 1.0,
            Depart::Right => self.x -= 1.0,
        }

        match self.direction {
            Direction::North => {
                if self.x == 355.0 {
                    self.depart = Depart::Down;
                }
            }
            Direction::South => {
                if self.x == 305.0 {
                    self.depart = Depart::Up;
                }
            }
            Direction::East => {
                if self.y == 305.0 {
                    self.depart = Depart::Right;
                }
            }
            Direction::West => {
                if self.y == 355.0 {
                    self.depart = Depart::Left;
                }
            }
        }
    }
}