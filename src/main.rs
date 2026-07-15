use macroquad::prelude::*;


fn window_conf() -> Conf {
    Conf {
        window_title: "Road Intersection".to_owned(),
        window_width: 700,
        window_height: 700,
        window_resizable: false,
        fullscreen: false,
        sample_count: 1,
        high_dpi: true,
        ..Default::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Depart {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Car {
    color: Color,
    x: f32,
    y: f32,
    direction: Direction,
}

#[macroquad::main(window_conf)]
async fn main() {

    let mut cars = Vec::new();

    loop {
        clear_background(DARKGRAY);

        draw_line(300.0, 0.0, 300.0, 700.0, 2.0, LIGHTGRAY);
        draw_line(350.0, 0.0, 350.0, 700.0, 2.0, LIGHTGRAY);
        draw_line(400.0, 0.0, 400.0, 700.0, 2.0, LIGHTGRAY);
        draw_line(0.0, 300.0, 700.0, 300.0, 2.0, LIGHTGRAY);
        draw_line(0.0, 350.0, 700.0, 350.0, 2.0, LIGHTGRAY);
        draw_line(0.0, 400.0, 700.0, 400.0, 2.0, LIGHTGRAY);

         if is_key_pressed(KeyCode::Up) {
            cars.push(Car {
                x: 305.0,
                y: -40.0,
                direction: Direction::Up,
                color: RED,
            });
        }

        if is_key_pressed(KeyCode::Down) {
            cars.push(Car {
                x: 355.0,
                y: 700.0,
                direction: Direction::Down,
                color: BLUE,
            });
        }

        if is_key_pressed(KeyCode::Left) {
            cars.push(Car {
                x: -40.0,
                y: 355.0,
                direction: Direction::Left,
                color: GREEN,
            });
        }

        if is_key_pressed(KeyCode::Right) {
            cars.push(Car {
                x: 700.0,
                y: 305.0,
                direction: Direction::Right,
                color: YELLOW,
            });
        }

        for car in &mut cars {
            
            draw_rectangle(car.x, car.y, 40.0, 40.0, car.color);
            match car.direction {
                Direction::Up => {
                    car.y += 1.0;
                }
                Direction::Down => {
                    car.y -= 1.0;
                }
                Direction::Left => {
                    car.x += 1.0;
                }
                Direction::Right => {
                    car.x -= 1.0;
                }
            }

            
        }
        cars.retain(|car| {
            car.x > -40.0
            && car.x < screen_width()
            && car.y > -40.0
            && car.y < screen_height()
        });

        println!("{:?}",cars);
 


        next_frame().await
    }
}