use macroquad::prelude::*;
pub struct Map;


impl Map {
    pub fn draw(&self) {
        draw_rectangle(0.0, 300.0, 700.0, 100.0, DARKGRAY);
        draw_rectangle(300.0, 0.0, 100.0, 700.0, DARKGRAY);
        draw_line(300.0, 0.0, 300.0, 700.0, 2.0, LIGHTGRAY);
        draw_line(350.0, 0.0, 350.0, 700.0, 2.0, ORANGE);
        draw_line(400.0, 0.0, 400.0, 700.0, 2.0, LIGHTGRAY);
        draw_line(0.0, 300.0, 700.0, 300.0, 2.0, LIGHTGRAY);
        draw_line(0.0, 350.0, 700.0, 350.0, 2.0, ORANGE);
        draw_line(0.0, 400.0, 700.0, 400.0, 2.0, LIGHTGRAY);
        draw_rectangle(300.0, 300.0, 100.0, 100.0, DARKGRAY);
        draw_rectangle(299.0, 301.0, 99.0, 101.0, DARKGRAY);
        draw_rectangle(301.0, 299.0, 101.0, 99.0, DARKGRAY);

    }
}