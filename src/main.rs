use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;
use rand::Rng;

#[derive(PartialEq, Clone)]
enum TrafficPhase {
    Up,  
    Down,      
    Right,  
    Left,  
}

#[derive(Debug)]
struct Vehicle {
    car: Rect,
    direction: String,
    random_route: String,
    color: Color,
    speed: i32,
}

impl Vehicle {
    fn new(car: Rect, direction: String, random_route: String, color: Color) -> Vehicle {
        Vehicle {
            car,
            direction,
            random_route,
            color,
            speed: 2,
        }
    }

    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(self.color);
        canvas.fill_rect(self.car).unwrap();
    }

    fn update(&mut self) {
        match self.random_route.as_str() {
            "GoStraight" => {
                if self.direction.as_str() == "up" {
                    self.car.y -= self.speed;
                }else if self.direction.as_str() == "down" {
                    self.car.y += self.speed;
                }else if self.direction.as_str() == "left" {
                    self.car.x -= self.speed;
                } else if self.direction.as_str() == "right" {
                    self.car.x += self.speed;
                } else {
                    todo!();
                }
            },
            "TurnRight" => {
                if self.direction.as_str() == "up" {
                    if self.car.y <= 415 {
                    self.car.x += self.speed;
                    }else {
                        self.car.y -= self.speed;
                    }
                }else if self.direction.as_str() == "down" {
                    if self.car.y >= 340 {
                        self.car.x -= self.speed;
                    }else {
                        self.car.y += self.speed;
                    }
                }else if self.direction.as_str() == "left" {
                    if self.car.x >= 515 {
                        self.car.x -= self.speed;
                    }else {
                        self.car.y -= self.speed;
                    }
                } else if self.direction.as_str() == "right" {
                    if self.car.x <= 435 {
                        self.car.x += self.speed;
                    }else {
                        self.car.y += self.speed;
                    }
                }  else {
                    todo!();
                }
            },
            "TurnLeft" => {
                if self.direction.as_str() == "up" {
                    if self.car.y <= 340 {
                        self.car.x -= self.speed;
                    }else {
                        self.car.y -= self.speed;
                    }
                }else if self.direction.as_str() == "down" {
                    if self.car.y >= 410 {
                        self.car.x += self.speed;
                    }else {
                        self.car.y += self.speed;
                    }
                } else if self.direction.as_str() == "left" {
                    if self.car.x >= 440 {
                        self.car.x -= self.speed;
                    }else {
                        self.car.y += self.speed;
                    }
                } else if self.direction.as_str() == "right" {
                    if self.car.x <= 510 {
                        self.car.x += self.speed;
                    }else {
                        self.car.y -= self.speed;
                    }
                } else {
                    todo!();
                }
            },
            _ => {}
        }
    }

    fn is_off_screen(&self) -> bool {
        self.car.x < -75 || self.car.x > 1075 || self.car.y < -75 || self.car.y > 875
    }

    fn is_approaching_intersection(&self) -> bool {
        match self.direction.as_str() {
            "up" => self.car.y <= 500 && self.car.y >= 450,
            "down" => self.car.y >= 250 && self.car.y <= 300,
            "left" => self.car.x <= 600 && self.car.x >= 550,
            "right" => self.car.x >= 350 && self.car.x <= 400,
            _ => false
        }
    }

    fn is_in_intersection(&self) -> bool {
        self.car.x >= 425 && self.car.x <= 575 && self.car.y >= 325 && self.car.y <= 475
    }

}

struct AdvancedTrafficSystem {
    phase: TrafficPhase,
    timer: u32,
    phase_duration: u32,
}

impl AdvancedTrafficSystem {
    fn new() -> Self {
        AdvancedTrafficSystem {
            phase: TrafficPhase::Up,
            timer: 0,
            phase_duration: 240,    
        }
    }

    fn update(&mut self, vehicles: &Vec<Vehicle>) {
        self.timer += 1;
        
        let should_extend = self.should_extend_phase(vehicles);
        
        if self.timer >= self.phase_duration && !should_extend {
            self.next_phase();
            self.timer = 0;
        }
    }

    fn should_extend_phase(&self, vehicles: &Vec<Vehicle>) -> bool {
        let current_direction = match self.phase {
            TrafficPhase::Up => "up",
            TrafficPhase::Down => "down",
            TrafficPhase::Left => "left",
            TrafficPhase::Right => "right",
        };
        
        vehicles.iter().any(|v| 
            v.direction == current_direction && v.is_in_intersection()
        )
    }

    fn next_phase(&mut self) {
        self.phase = match self.phase {
            TrafficPhase::Up => TrafficPhase::Down,
            TrafficPhase::Down => TrafficPhase::Left,
            TrafficPhase::Left => TrafficPhase::Right,
            TrafficPhase::Right => TrafficPhase::Up,
        };
    }

    fn can_vehicle_proceed(&self, vehicle: &Vehicle) -> bool {
        if vehicle.is_in_intersection() {
            return true;
        }

        if !vehicle.is_approaching_intersection() {
            return true;
        }

        let allowed_direction = match self.phase {
            TrafficPhase::Up => "up",
            TrafficPhase::Down => "down",
            TrafficPhase::Left => "left", 
            TrafficPhase::Right => "right",
        };
        vehicle.direction == allowed_direction
    }

    fn get_light_colors(&self) -> (Color, Color, Color, Color) {
        match self.phase {
            TrafficPhase::Down => {
                (Color::GREEN, Color::RED, Color::RED, Color::RED)
            },
            TrafficPhase::Up => {
                (Color::RED, Color::GREEN, Color::RED, Color::RED)
            },
            TrafficPhase::Left => {
                (Color::RED, Color::RED, Color::GREEN, Color::RED)
            },
            TrafficPhase::Right => {
                (Color::RED, Color::RED, Color::RED, Color::GREEN)
            },
        }
    }
}

fn spawn_car(x: i32, y: i32, direction: &str) -> Vehicle {
    let routes = ["TurnLeft", "TurnRight", "GoStraight"];
    let mut rng = rand::rng();
    let index = rng.random_range(0..3);
    let random_route = routes[index].to_owned();

    let color = match random_route.as_str() {
        "TurnLeft" => Color::YELLOW,
        "TurnRight" => Color::BLUE, 
        "GoStraight" => Color::GREY,
        _ => Color::WHITE,
    };

    let car_rect = Rect::new(x, y, 50, 50);
    Vehicle::new(car_rect, direction.to_owned(), random_route, color)
}

fn can_spawn_vehicle(vehicles: &Vec<Vehicle>, spawn_x: i32, spawn_y: i32, direction: &str) -> bool {
    let safe_distance = 100;

    for vehicle in vehicles {
        let distance = match direction {
            "up" => {
                if vehicle.direction == "up" && (vehicle.car.x - spawn_x).abs() < 30 {
                    (spawn_y - vehicle.car.y).abs()
                } else {
                    safe_distance + 1  // Not same lane
                }
            },
            "down" => {
                if vehicle.direction == "down" && (vehicle.car.x - spawn_x).abs() < 30 {
                    (vehicle.car.y - spawn_y).abs()
                } else {
                    safe_distance + 1
                }
            },
            "left" => {
                if vehicle.direction == "left" && (vehicle.car.y - spawn_y).abs() < 30 {
                    (spawn_x - vehicle.car.x).abs()
                } else {
                    safe_distance + 1
                }
            },
            "right" => {
                if vehicle.direction == "right" && (vehicle.car.y - spawn_y).abs() < 30 {
                    (vehicle.car.x - spawn_x).abs()
                } else {
                    safe_distance + 1
                }
            },
            _ => safe_distance + 1
        };
        
        if distance < safe_distance {
            return false;
        }
    }
    true
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    // let ttf_context = sdl2::ttf::init().unwrap();
 
    let window = video_subsystem.window("Road Intersection", 1000, 800)
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();

    // TODO: check it later if it's working in all os
    // Font setup (you may need to adjust path for your OS)
    // let font = ttf_context.load_font("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", 20)
    //     .or_else(|_| ttf_context.load_font("C:\\Windows\\Fonts\\arial.ttf", 20))
    //     .or_else(|_| ttf_context.load_font("/System/Library/Fonts/Arial.ttf", 20))
    //     .unwrap();

    // let dir = font.render("↑ | ↓ | → | ← : Spawn vehicle from specified direction")
    //     .blended(Color::WHITE)
    //     .unwrap();
    // let random = font.render("R : Spawn vehicle from random direction")
    //     .blended(Color::WHITE)
    //     .unwrap();
    // let exit = font.render("ESC : Exit simulation")
    //     .blended(Color::WHITE)
    //     .unwrap();

    // let texture_creator = canvas.texture_creator();
    // let texture_dir = texture_creator.create_texture_from_surface(&dir).unwrap();
    // let texture_r = texture_creator.create_texture_from_surface(&random).unwrap();
    // let texture_exit = texture_creator.create_texture_from_surface(&exit).unwrap();

    let mut vehicles: Vec<Vehicle> = Vec::new();
    let mut traffic_system = AdvancedTrafficSystem::new();

    // Traffic light positions
    let light_ne = Rect::new(375, 275, 50, 50);
    let light_nw = Rect::new(575, 275, 50, 50);
    let light_se = Rect::new(375, 475, 50, 50);
    let light_sw = Rect::new(575, 475, 50, 50);
 
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear(); 
    canvas.present();
    
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        
        // DEBUG: is_in_intersection XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
        // canvas.set_draw_color(Color::YELLOW);
        // canvas.draw_line((0, 500), (1000, 500)).unwrap();
        
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown {keycode: Some(Keycode::Up), .. } => {
                    let spawn_x = 515;
                    let spawn_y = 700;
                    if can_spawn_vehicle(&vehicles, spawn_x, spawn_y,"up") {
                        let new_car = spawn_car(spawn_x, spawn_y, "up");
                        vehicles.push(new_car);
                    }
                },
                Event::KeyDown {keycode: Some(Keycode::Down), .. } => {
                    let spawn_x = 440;
                    let spawn_y = 0;
                    if can_spawn_vehicle(&vehicles, spawn_x, spawn_y,"down") {
                        let new_car = spawn_car(spawn_x, spawn_y, "down");
                        vehicles.push(new_car);
                    }
                },
                Event::KeyDown {keycode: Some(Keycode::Left), .. } => {
                    let spawn_x = 950;
                    let spawn_y = 335;
                    if can_spawn_vehicle(&vehicles, spawn_x, spawn_y,"left") {
                        let new_car = spawn_car(spawn_x, spawn_y, "left");
                        vehicles.push(new_car);
                    }
                },
                Event::KeyDown {keycode: Some(Keycode::Right), .. } => {
                    let spawn_x = 10;
                    let spawn_y = 415;
                    if can_spawn_vehicle(&vehicles, spawn_x, spawn_y,"right") {
                        let new_car = spawn_car(spawn_x, spawn_y, "right");
                        vehicles.push(new_car);
                    }
                },
                Event::KeyDown {keycode: Some(Keycode::R), .. } => {
                    let directions = ["up", "down", "left", "right"];
                    let mut rng = rand::rng();
                    let direction = directions[rng.random_range(0..4)];
                    
                     let (spawn_x, spawn_y) = match direction {
                        "up" => (515, 750),
                        "down" => (440, 0),
                        "left" => (950, 335),
                        "right" => (10, 415),
                        _ => (515, 750)
                    };
                    
                    if can_spawn_vehicle(&vehicles, spawn_x, spawn_y,direction) {
                        let new_car = spawn_car(spawn_x, spawn_y, direction);
                        vehicles.push(new_car);
                    }
                },
                _ => {}
            }
        }
        
        // Update traffic light system
        traffic_system.update(&vehicles);
        
        let (up_color, down_color, left_color, right_color) = traffic_system.get_light_colors();
        
        // Draw traffic lights
        canvas.set_draw_color(up_color);
        canvas.fill_rect(light_ne).unwrap();
        canvas.set_draw_color(down_color);
        canvas.fill_rect(light_sw).unwrap();
        canvas.set_draw_color(right_color);
        canvas.fill_rect(light_se).unwrap();
        canvas.set_draw_color(left_color);
        canvas.fill_rect(light_nw).unwrap();

        // Draw roads
        canvas.set_draw_color(Color::WHITE);
        // North-South road (vertical)
        canvas.draw_line((500, 0), (500, 325)).unwrap();
        canvas.draw_line((500, 475), (500, 800)).unwrap();
        canvas.draw_line((575, 0), (575, 325)).unwrap();
        canvas.draw_line((575, 475), (575, 800)).unwrap();
        canvas.draw_line((425, 0), (425, 325)).unwrap();
        canvas.draw_line((425, 475), (425, 800)).unwrap();
        
        // East-West road (horizontal) 
        canvas.draw_line((0, 400), (425, 400)).unwrap();
        canvas.draw_line((575, 400), (1000, 400)).unwrap();
        canvas.draw_line((0, 325), (425, 325)).unwrap();
        canvas.draw_line((575, 325), (1000, 325)).unwrap();
        canvas.draw_line((0, 475), (425, 475)).unwrap();
        canvas.draw_line((575, 475), (1000, 475)).unwrap();

        // canvas.copy(&texture_dir, None, Some(Rect::new(10, 10, 340, 25))).unwrap();
        // canvas.copy(&texture_r, None, Some(Rect::new(10, 35, 240, 25))).unwrap();
        // canvas.copy(&texture_exit, None, Some(Rect::new(10, 60, 120, 25))).unwrap();

        // Update vehicles with traffic light awareness (using indices to avoid borrowing conflicts)
        for i in 0..vehicles.len() {
            let can_proceed = traffic_system.can_vehicle_proceed(&vehicles[i]);
            let has_vehicle_ahead = {
                let current_vehicle = &vehicles[i];
                vehicles.iter().enumerate().any(|(j, other)| {
                    if i == j {
                        return false; // Skip self
                    }

                    let safe_distance = 60;
                    match current_vehicle.direction.as_str() {
                        "up" => {
                            (current_vehicle.car.x - other.car.x).abs() < 30 && 
                            other.car.y < current_vehicle.car.y && 
                            (current_vehicle.car.y - other.car.y) < safe_distance
                        },
                        "down" => {
                            (current_vehicle.car.x - other.car.x).abs() < 30 && 
                            other.car.y > current_vehicle.car.y && 
                            (other.car.y - current_vehicle.car.y) < safe_distance
                        },
                        "left" => {
                            (current_vehicle.car.y - other.car.y).abs() < 30 && 
                            other.car.x < current_vehicle.car.x && 
                            (current_vehicle.car.x - other.car.x) < safe_distance
                        },
                        "right" => {
                            (current_vehicle.car.y - other.car.y).abs() < 30 && 
                            other.car.x > current_vehicle.car.x && 
                            (other.car.x - current_vehicle.car.x) < safe_distance
                        },
                        _ => false
                    }
                })
            };

            // Only update if vehicle can proceed and no vehicle ahead
            if can_proceed && !has_vehicle_ahead {
                vehicles[i].update();
            }
        }

        vehicles.retain(|vehicle| !vehicle.is_off_screen());

        for vehicle in &vehicles {
            vehicle.draw(&mut canvas);
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}