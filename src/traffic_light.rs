use macroquad::prelude::*;

pub struct TrafficLight {
    pub x: f32,
    pub y: f32,
    pub is_on: bool,
}

impl TrafficLight {
    pub fn draw(&self) {
         draw_rectangle(self.x, self.y , 10.0 , 20.0, BLACK);
            if self.is_on {
                draw_circle(self.x+5.0, self.y+5.0 , 3.0, GREEN);
                draw_circle(self.x+5.0, self.y+15.0 , 3.0, DARKGRAY);
            } else {
                draw_circle(self.x+5.0, self.y+5.0 , 3.0, DARKGRAY);
                draw_circle(self.x+5.0, self.y+15.0 , 3.0, RED);
            }
    }
}