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
    /// True once the car has entered the intersection and no longer needs to check the light.
    pub crossed: bool,
}

impl Car {
    pub const SAF_DISTANCE: f32 = 10.0;

    // NOTE: the Depart enum encodes the *source side* of the road, not the screen movement.
    // Original movement rules (preserved exactly):
    //   Up    => y += 1.0   (spawned at top, moving down on screen)
    //   Down  => y -= 1.0   (spawned at bottom, moving up on screen)
    //   Left  => x += 1.0   (spawned at left, moving right on screen)
    //   Right => x -= 1.0   (spawned at right, moving left on screen)

    pub fn draw(&self) {
        let rotation = match self.depart {
            Depart::Down  => 0.0_f32,
            Depart::Up    => 180.0_f32,
            Depart::Right => 270.0_f32,
            Depart::Left  => 90.0_f32,
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

    /// Update position. Receives current light state so cars stop at red lights.
    /// `is_green` indicates if the light for this car's direction is currently green.
    /// `blocked` indicates if there is another car directly in front of this one.
    pub fn update(&mut self, is_green: bool, blocked: bool) {
        if blocked {
            return; // hold position to avoid crashing into the car ahead
        }

        // --- Red-light stop check (only before the car has entered the intersection) ---
        if !self.crossed {
            // Intersection box spans x: 300–400, y: 300–400.
            // Stop lines sit just outside that box, one car-length away.
            let must_stop = match self.depart {
                // Spawned at bottom (y=700), moving UP on screen (y decreases each frame)
                Depart::Down => !is_green && self.y <= 410.0,
                // Spawned at top (y=-40), moving DOWN on screen (y increases each frame)
                Depart::Up   => !is_green && self.y >= 260.0,
                // Spawned at left (x=-40), moving RIGHT on screen (x increases each frame)
                Depart::Left  => !is_green && self.x >= 260.0,
                // Spawned at right (x=700), moving LEFT on screen (x decreases each frame)
                Depart::Right => !is_green && self.x <= 410.0,
            };

            if must_stop {
                return; // hold position until light turns green
            }
        }

        // --- Move (original directions preserved) ---
        match self.depart {
            Depart::Up    => self.y += 1.0,
            Depart::Down  => self.y -= 1.0,
            Depart::Left  => self.x += 1.0,
            Depart::Right => self.x -= 1.0,
        }

        // --- Mark as crossed once past the stop line ---
        // If a car has passed the stop line, it is committed to crossing and should not stop for a red light anymore.
        if !self.crossed {
            self.crossed = match self.depart {
                Depart::Down  => self.y < 410.0,
                Depart::Up    => self.y > 260.0,
                Depart::Left  => self.x > 260.0,
                Depart::Right => self.x < 410.0,
            };
        }

        // --- Turn at the intersection centre based on destination ---
        match self.direction {
            Direction::North => {
                if self.x == 355.0 { self.depart = Depart::Down; }
            }
            Direction::South => {
                if self.x == 305.0 { self.depart = Depart::Up; }
            }
            Direction::East => {
                if self.y == 305.0 { self.depart = Depart::Right; }
            }
            Direction::West => {
                if self.y == 355.0 { self.depart = Depart::Left; }
            }
        }
    }

    // --- Spawn helpers (positions/depart identical to original) ---

    pub fn spawn_up(color: Texture2D, direction: Direction) -> Self {
        Self { x: 355.0, y: 700.0, depart: Depart::Down, direction, color, crossed: false }
    }

    pub fn spawn_down(color: Texture2D, direction: Direction) -> Self {
        Self { x: 305.0, y: -40.0, depart: Depart::Up, direction, color, crossed: false }
    }

    pub fn spawn_left(color: Texture2D, direction: Direction) -> Self {
        Self { x: 700.0, y: 305.0, depart: Depart::Right, direction, color, crossed: false }
    }

    pub fn spawn_right(color: Texture2D, direction: Direction) -> Self {
        Self { x: -40.0, y: 355.0, depart: Depart::Left, direction, color, crossed: false }
    }
}

pub fn checkSafty(car: &Car, cars: &Vec<Car>) -> bool {
    match car.depart {
        Depart::Down => cars.iter().rev().any(|other| {
            other.depart == Depart::Down && (700.0 - (other.y + 40.0)) < Car::SAF_DISTANCE
        }),
        Depart::Up => cars.iter().rev().any(|other| {
            other.depart == Depart::Up && other.y <= Car::SAF_DISTANCE
        }),
        Depart::Right => cars.iter().rev().any(|other| {
            other.depart == Depart::Right && (700.0 - (other.x + 40.0)) < Car::SAF_DISTANCE
        }),
        Depart::Left => cars.iter().rev().any(|other| {
            other.depart == Depart::Left && other.x < Car::SAF_DISTANCE
        }),
    }
}