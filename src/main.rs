mod car;
mod traffic_light;
mod map;
use map::Map;
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
    let map = Map;
    loop {
        clear_background(DARKGREEN);
        draw_texture(&map_img,0.0,0.0,WHITE);
        map.draw();
       
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
            light.draw();
        }

 


        next_frame().await
    }
}