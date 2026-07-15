mod car;
mod traffic_light;
use traffic_light::TrafficLight;
use macroquad::prelude::*;
use car::{Depart, Direction, Car};

fn window_conf() -> Conf {
    Conf {
        window_title: "Road Intersection".to_owned(),
        window_width: 700,
        window_height: 700,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {

    let depart_up_dir = [Direction::East, Direction::North, Direction::West];
    let depart_down_dir = [Direction::West, Direction::South, Direction::East];
    let depart_left_dir = [Direction::South, Direction::East, Direction::North];
    let depart_right_dir = [Direction::North, Direction::West, Direction::South];
    let random_car = [Depart::Up, Depart::Down, Depart::Right, Depart::Left];
    let mut cars = Vec::new();
    let lights = vec![
        TrafficLight {
            x: 285.0,
            y: 275.0,
            is_on: false,
        },
        TrafficLight {
            x: 405.0,
            y: 275.0,
            is_on: false,
        },
        TrafficLight {
            x: 285.0,
            y: 405.0,
            is_on: true,
        },
        TrafficLight {
            x: 405.0,
            y: 405.0,
            is_on: false,
        },
        ];
        
        let map_img = load_texture("assets/map.png").await.unwrap();
        let car_1 = load_texture("assets/car1.png").await.unwrap();
        let car_2 = load_texture("assets/car2.png").await.unwrap();
        let car_3 = load_texture("assets/car3.png").await.unwrap();
        let car_colors = [car_1, car_2, car_3];

    loop {
        clear_background(DARKGREEN);

        draw_texture(&map_img,0.0,0.0,WHITE);
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

         if is_key_pressed(KeyCode::Down) {
            let r = macroquad::rand::gen_range(0, 3);
            cars.push(Car {
                x: 305.0,
                y: -40.0,
                depart: Depart::Up,
                direction: depart_down_dir[r],
                color: car_colors[r].clone(),
            });
        }

        if is_key_pressed(KeyCode::Up) {
            let r = macroquad::rand::gen_range(0, 3);
            cars.push(Car {
                x: 355.0,
                y: 700.0,
                depart: Depart::Down,
                direction: depart_up_dir[r],
                color: car_colors[r].clone(),
            });
        }

        if is_key_pressed(KeyCode::Right) {
            let r = macroquad::rand::gen_range(0, 3);
            cars.push(Car {
                x: -40.0,
                y: 355.0,
                depart: Depart::Left,
                direction: depart_right_dir[r],
                color: car_colors[r].clone(),
            });
        }

        if is_key_pressed(KeyCode::Left) {
            let r = macroquad::rand::gen_range(0, 3);
            cars.push(Car {
                x: 700.0,
                y: 305.0,
                depart: Depart::Right,
                direction: depart_left_dir[r],
                color: car_colors[r].clone(),
            });
        }

        if is_key_pressed(KeyCode::R) {
            let r = macroquad::rand::gen_range(0, 3);
            let rand_car = macroquad::rand::gen_range(0, 4);

            println!("{rand_car}");

            let car = match random_car[rand_car] {
                Depart::Up => Car {
                    x: 355.0,
                    y: 700.0,
                    depart: Depart::Down,
                    direction: depart_up_dir[r],
                    color: car_colors[r].clone(),
                },
            
                Depart::Down => Car {
                    x: 305.0,
                    y: -40.0,
                    depart: Depart::Up,
                    direction: depart_down_dir[r],
                    color: car_colors[r].clone(),
                },
            
                Depart::Left => Car {
                    x: -40.0,
                    y: 355.0,
                    depart: Depart::Left,
                    direction: depart_right_dir[r],
                    color: car_colors[r].clone(),
                },
            
                Depart::Right => Car {
                    x: 700.0,
                    y: 305.0,
                    depart: Depart::Right,
                    direction: depart_left_dir[r],
                    color: car_colors[r].clone(),
                },
            };
        
            cars.push(car);
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        for car in &mut cars {
            car.draw();
            car.update();
        }
        cars.retain(|car| {
            car.x > -40.0
            && car.x < screen_width()
            && car.y > -40.0
            && car.y < screen_height()
        });

        for light in &lights {
            draw_rectangle(light.x, light.y , 10.0 , 20.0, BLACK);
            if light.is_on {
                draw_circle(light.x+5.0, light.y+5.0 , 3.0, GREEN);
                draw_circle(light.x+5.0, light.y+15.0 , 3.0, DARKGRAY);
            } else {
                draw_circle(light.x+5.0, light.y+5.0 , 3.0, DARKGRAY);
                draw_circle(light.x+5.0, light.y+15.0 , 3.0, RED);
            }
        }

 


        next_frame().await
    }
}