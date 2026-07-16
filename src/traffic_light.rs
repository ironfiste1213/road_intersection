use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    North = 0, // Green for Depart::Up
    South = 1, // Green for Depart::Down
    East = 2,  // Green for Depart::Right
    West = 3,  // Green for Depart::Left
}

pub struct LightController {
    pub phase: Phase,
    pub phase_timer: f64,
    pub wait_times: [f64; 4],
    pub clearing: bool,
}

impl LightController {
    pub fn new() -> Self {
        Self {
            phase: Phase::North,
            phase_timer: 0.0,
            wait_times: [0.0; 4],
            clearing: false,
        }
    }

    pub fn update(&mut self, dt: f64, cars_north: usize, cars_south: usize, cars_east: usize, cars_west: usize, intersection_clear: bool) {
        self.phase_timer += dt;

        let car_counts = [cars_north, cars_south, cars_east, cars_west];

        // Update wait times
        for i in 0..4 {
            if i != self.phase as usize && car_counts[i] > 0 {
                self.wait_times[i] += dt;
            } else if i == self.phase as usize || car_counts[i] == 0 {
                // Reset wait time if the light is green or there are no cars
                self.wait_times[i] = 0.0;
            }
        }

        if self.clearing {
            if intersection_clear {
                self.clearing = false;
                self.phase_timer = 0.0;
            }
            return; // All lights stay red while clearing
        }

        // Minimum green time of 2.0 seconds to prevent flickering
        if self.phase_timer >= 2.0 {
            let current_cars = car_counts[self.phase as usize];
            
            // Current phase score. Give it a small bonus to prevent switching if everything is empty
            let mut best_score = current_cars as f64 * 2.0 + 1.0; 
            let mut best_phase = self.phase;

            for i in 0..4 {
                if i == self.phase as usize { continue; }
                
                // Score = (number of cars * 2) + (seconds waiting * 1)
                // This means a road with many cars gets priority, but a single car waiting 
                // a long time will eventually override it (starvation prevention).
                let score = car_counts[i] as f64 * 2.0 + self.wait_times[i];
                
                if score > best_score {
                    best_score = score;
                    best_phase = match i {
                        0 => Phase::North,
                        1 => Phase::South,
                        2 => Phase::East,
                        _ => Phase::West,
                    };
                }
            }

            if best_phase != self.phase {
                self.phase = best_phase;
                self.clearing = true; // Enter all-red clearance phase
                self.wait_times[self.phase as usize] = 0.0;
            }
        }
    }

    pub fn is_green(&self, phase: Phase) -> bool {
        !self.clearing && self.phase == phase
    }
}

pub struct TrafficLight {
    pub x: f32,
    pub y: f32,
    pub is_green: bool,
}

impl TrafficLight {
    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, 10.0, 20.0, BLACK);
        if self.is_green {
            draw_circle(self.x + 5.0, self.y + 5.0, 3.5, GREEN);
            draw_circle(self.x + 5.0, self.y + 15.0, 3.5, DARKGRAY);
        } else {
            draw_circle(self.x + 5.0, self.y + 5.0, 3.5, DARKGRAY);
            draw_circle(self.x + 5.0, self.y + 15.0, 3.5, RED);
        }
    }
}