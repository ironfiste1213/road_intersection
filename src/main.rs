mod car;
mod traffic_light;
mod map;

use map::Map;
use traffic_light::{LightController, Phase, TrafficLight};
use macroquad::prelude::*;
use car::{Depart, Direction, Car, checkSafty};

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
    let depart_up_dir    = [Direction::East,  Direction::North, Direction::West];
    let depart_down_dir  = [Direction::West,  Direction::South, Direction::East];
    let depart_left_dir  = [Direction::South, Direction::East,  Direction::North];
    let depart_right_dir = [Direction::North, Direction::West,  Direction::South];
    let random_car = [Depart::Up, Depart::Down, Depart::Right, Depart::Left];

    let mut cars: Vec<Car> = Vec::new();

    let mut controller = LightController::new();

    let map_img = load_texture("assets/map.png").await.unwrap();
    let car_1   = load_texture("assets/car1.png").await.unwrap();
    let car_2   = load_texture("assets/car2.png").await.unwrap();
    let car_3   = load_texture("assets/car3.png").await.unwrap();
    let car_colors = [car_1, car_2, car_3];

    let map = Map;

    loop {
        let dt = get_frame_time() as f64;

        // --- Count waiting cars ---
        let mut cars_north = 0;
        let mut cars_south = 0;
        let mut cars_east = 0;
        let mut cars_west = 0;

        for car in &cars {
            if !car.crossed {
                match car.depart {
                    Depart::Up => cars_north += 1,
                    Depart::Down => cars_south += 1,
                    Depart::Right => cars_east += 1,
                    Depart::Left => cars_west += 1,
                }
            }
        }

        // --- Check if intersection is clear ---
        let mut intersection_clear = true;
        for car in &cars {
            let passed_stop_line = match car.depart {
                Depart::Up => car.y > 260.0,
                Depart::Down => car.y < 410.0,
                Depart::Left => car.x > 260.0,
                Depart::Right => car.x < 410.0,
            };
            let fully_exited = match car.depart {
                Depart::Up => car.y > 450.0,
                Depart::Down => car.y < 250.0,
                Depart::Left => car.x > 450.0,
                Depart::Right => car.x < 250.0,
            };
            if passed_stop_line && !fully_exited {
                intersection_clear = false;
                break;
            }
        }

        // --- Update light controller ---
        controller.update(dt, cars_north, cars_south, cars_east, cars_west, intersection_clear);

        let n_green = controller.is_green(Phase::North);
        let s_green = controller.is_green(Phase::South);
        let e_green = controller.is_green(Phase::East);
        let w_green = controller.is_green(Phase::West);

        // --- Build this frame's light visuals ---
        // Lights are on the right-hand side of each arriving road.
        let lights = vec![
            TrafficLight { x: 285.0, y: 275.0, is_green: n_green }, // NW – North road (Depart::Up, going down)
            TrafficLight { x: 405.0, y: 275.0, is_green: e_green }, // NE – East road (Depart::Right, going left)
            TrafficLight { x: 285.0, y: 405.0, is_green: w_green }, // SW – West road (Depart::Left, going right)
            TrafficLight { x: 405.0, y: 405.0, is_green: s_green }, // SE – South road (Depart::Down, going up)
        ];

        clear_background(DARKGREEN);
        draw_texture(&map_img, 0.0, 0.0, WHITE);
        map.draw();

        // --- Input: spawn cars ---
        if is_key_pressed(KeyCode::Down) {
            let r = macroquad::rand::gen_range(0, 3);
            let car = Car::spawn_down(car_colors[r].clone(), depart_down_dir[r]);
            if !checkSafty(&car, &cars) { cars.push(car); }
        }

        if is_key_pressed(KeyCode::Up) {
            let r = macroquad::rand::gen_range(0, 3);
            let car = Car::spawn_up(car_colors[r].clone(), depart_up_dir[r]);
            if !checkSafty(&car, &cars) { cars.push(car); }
        }

        if is_key_pressed(KeyCode::Right) {
            let r = macroquad::rand::gen_range(0, 3);
            let car = Car::spawn_right(car_colors[r].clone(), depart_right_dir[r]);
            if !checkSafty(&car, &cars) { cars.push(car); }
        }

        if is_key_pressed(KeyCode::Left) {
            let r = macroquad::rand::gen_range(0, 3);
            let car = Car::spawn_left(car_colors[r].clone(), depart_left_dir[r]);
            if !checkSafty(&car, &cars) { cars.push(car); }
        }

        if is_key_pressed(KeyCode::R) {
            let r = macroquad::rand::gen_range(0, 3);
            let rand_car = macroquad::rand::gen_range(0, 4);
            let car = match random_car[rand_car] {
                Depart::Up    => Car::spawn_up   (car_colors[r].clone(), depart_up_dir[r]),
                Depart::Down  => Car::spawn_down (car_colors[r].clone(), depart_down_dir[r]),
                Depart::Left  => Car::spawn_right(car_colors[r].clone(), depart_right_dir[r]),
                Depart::Right => Car::spawn_left (car_colors[r].clone(), depart_left_dir[r]),
            };
            if !checkSafty(&car, &cars) { cars.push(car); }
        }

        if is_key_pressed(KeyCode::Escape) { break; }

        // --- Calculate blocking (safe distance) ---
        let mut blocked_cars = vec![false; cars.len()];
        for i in 0..cars.len() {
            let car_i = &cars[i];
            let (vx, vy) = match car_i.depart {
                Depart::Up => (0.0, 1.0),
                Depart::Down => (0.0, -1.0),
                Depart::Left => (1.0, 0.0),
                Depart::Right => (-1.0, 0.0),
            };

            for j in 0..cars.len() {
                if i == j { continue; }
                let car_j = &cars[j];
                
                let dx = car_j.x - car_i.x;
                let dy = car_j.y - car_i.y;
                
                let dot = dx * vx + dy * vy;
                let cross = dx * vy - dy * vx;
                
                // If car_j is in front of car_i (dot > 0), close enough (dot < 55.0), 
                // and in the same lane (|cross| < 20.0)
                if dot > 0.0 && dot < 55.0 && cross.abs() < 20.0 {
                    blocked_cars[i] = true;
                    break;
                }
            }
        }

        // --- Update and draw cars ---
        for (i, car) in cars.iter_mut().enumerate() {
            let is_green = match car.depart {
                Depart::Up => n_green,
                Depart::Down => s_green,
                Depart::Right => e_green,
                Depart::Left => w_green,
            };
            car.update(is_green, blocked_cars[i]);
            car.draw();
        }

        // Remove cars that have left the screen
        cars.retain(|car| {
            car.x > -40.0
                && car.x < screen_width()
                && car.y > -40.0
                && car.y < screen_height()
        });

        // --- Draw traffic lights ---
        for light in &lights {
            light.draw();
        }

        // --- HUD: show current phase and wait times ---
        let mut phase_label = match controller.phase {
            Phase::North => "NORTH GREEN",
            Phase::South => "SOUTH GREEN",
            Phase::East => "EAST GREEN",
            Phase::West => "WEST GREEN",
        }.to_string();
        
        if controller.clearing {
            phase_label = "ALL RED (clearing)".to_string();
        }
        
        let hud = format!("{} (active for {:.1}s)", phase_label, controller.phase_timer);
        draw_text(&hud, 10.0, 20.0, 18.0, WHITE);
        
        let wait_stats = format!("Wait stats - N: {:.1}s, S: {:.1}s, E: {:.1}s, W: {:.1}s", 
            controller.wait_times[0], controller.wait_times[1], controller.wait_times[2], controller.wait_times[3]);
        draw_text(&wait_stats, 10.0, 40.0, 16.0, WHITE);

        next_frame().await
    }
}