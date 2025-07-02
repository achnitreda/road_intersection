// main.rs
mod render;
mod traffic;
mod traffic_light;
use traffic_light::TrafficSystem;

use render::{draw_roads, draw_traffic_lights, draw_vehicles};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use traffic::{Direction, Vehicle, spawn_vehicle};

fn can_spawn_vehicle(dir: Direction, vehicles: &[Vehicle]) -> bool {
    let spawn_point = dir.start_position();

    // Find the last vehicle in the same direction
    for v in vehicles.iter().rev() {
        if v.dir == dir {
            let dist = match dir {
                Direction::North | Direction::South => (v.pos.y - spawn_point.y).abs(),
                Direction::East | Direction::West => (v.pos.x - spawn_point.x).abs(),
            };

            return dist >= traffic::MIN_SPAWN_DISTANCE;
        }
    }

    // No vehicle in same direction, allow spawn
    true
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Traffic Simulation", 800, 800)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut vehicles: Vec<Vehicle> = Vec::new();
    let mut traffic_lights = TrafficSystem::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    let dir = match key {
                        Keycode::Up => Some(Direction::South),
                        Keycode::Down => Some(Direction::North),
                        Keycode::Left => Some(Direction::East),
                        Keycode::Right => Some(Direction::West),
                        Keycode::R => Some(Direction::random()),
                        _ => None,
                    };

                    if let Some(d) = dir {
                        if can_spawn_vehicle(d, &vehicles) {
                            vehicles.push(spawn_vehicle(d));
                        }
                    }
                }
                _ => {}
            }
        }
        traffic_lights.update();

        for i in 0..vehicles.len() {
            let (left, right) = vehicles.split_at_mut(i);
            let (vehicle, rest) = right.split_first_mut().unwrap();
            let others: Vec<_> = left.iter().chain(rest.iter()).cloned().collect();
            vehicle.update(&traffic_lights, &others);
        }

        canvas.set_draw_color(sdl2::pixels::Color::RGB(200, 200, 200));
        canvas.clear();

        draw_roads(&mut canvas)?;
        draw_vehicles(&mut canvas, &vehicles)?;
        draw_roads(&mut canvas)?;
        draw_traffic_lights(&mut canvas, &traffic_lights)?;
        draw_vehicles(&mut canvas, &vehicles)?;
        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
