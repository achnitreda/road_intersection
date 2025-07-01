use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;
use rand::Rng;

struct Vehicle {
    car : Rect,
    direction : String,
    random_route : String,
    color : Color,
    speed: i32,
}

impl Vehicle {
    fn new(car: Rect, direction : String,random_route: String, color: Color) -> Vehicle {
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
                }else {
                    todo!();
                }
            },
            "TurnRight" => {
                if self.direction.as_str() == "up" {
                    if self.car.y-15 <= 400 {
                    self.car.x += self.speed;
                    }else {
                        self.car.y -= self.speed;
                    }
                }else if self.direction.as_str() == "down" {
                    if self.car.y-15 >= 325 {
                        self.car.x -= self.speed;
                    }else {
                        self.car.y += self.speed;
                    }
                }else {
                    todo!();
                }
            },
            "TurnLeft" => {
                if self.direction.as_str() == "up" {
                    if self.car.y-15 <= 325 {
                        self.car.x -= self.speed;
                    }else {
                        self.car.y -= self.speed;
                    }
                }else if self.direction.as_str() == "down" {
                    if self.car.y+15 >= 425 {
                        self.car.x += self.speed;
                    }else {
                        self.car.y += self.speed;
                    }
                }else {
                    todo!();
                }
            },
            _ => {}
        }
    }

    fn is_off_screen(&self) -> bool {
        self.car.y < -75  || self.car.x > 1000 || self.car.x < -75 || self.car.y > 800
    }
}

fn spawn_car(x : i32, y:i32, direction : &str) -> Vehicle {
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

    let car_rect = Rect::new(x,y, 50, 50);
    Vehicle::new(car_rect, direction.to_owned(),random_route, color)
}


fn can_spawn_vehicle(vehicles: &Vec<Vehicle>, spawn_x: i32, spawn_y: i32) -> bool {
    let safe_distance = 100;
    for vehicle in vehicles {
        match vehicle.direction.as_str() {
            "up" => {
                if (vehicle.car.y - spawn_y).abs() < safe_distance {
                    return false;
                }
            },
            "down" => {
                if (vehicle.car.y + spawn_y).abs() < safe_distance {
                    return false;
                }
            },
            _ => {}
        }
    }
    true
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
 
    let window = video_subsystem.window("Road Intersection", 1000, 800)
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();

    // TODO: remeber in readme I should check if it's working windows and other os
    let font = ttf_context.load_font("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", 24).unwrap();

    let dir = font.render("↑ | ↓ | → | ← : Spawn a vehicle from the specified direction")
        .blended(Color::WHITE)
        .unwrap();
    let random = font.render("r : Spawn a vehicle from the random direction")
        .blended(Color::WHITE)
        .unwrap();
    let exit = font.render("Esc: Exit simulation")
        .blended(Color::WHITE)
        .unwrap();

    let texture_creator = canvas.texture_creator();
    let texture_dir = texture_creator.create_texture_from_surface(&dir).unwrap();
    let texture_r = texture_creator.create_texture_from_surface(&random).unwrap();
    let texture_exit = texture_creator.create_texture_from_surface(&exit).unwrap();

    let mut vehicles: Vec<Vehicle> = Vec::new();

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
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown {keycode: Some(Keycode::Up), .. } => {
                    let spawn_x = 515;
                    let spawn_y = 700;
                    if can_spawn_vehicle(&vehicles, spawn_x, spawn_y) {
                        let new_car = spawn_car(spawn_x, spawn_y,"up");
                        vehicles.push(new_car);
                    }
                },
                Event::KeyDown {keycode: Some(Keycode::Down), .. } => {
                    let spawn_x = 440;
                    let spawn_y = 0;
                    if can_spawn_vehicle(&vehicles, spawn_x, spawn_y) {
                        let new_car = spawn_car(spawn_x, spawn_y,"down");
                        vehicles.push(new_car);
                    }
                },
                _ => {}
            }
        }
        
        canvas.set_draw_color(Color::RED);
        canvas.draw_rect(light_ne).unwrap();  
        canvas.draw_rect(light_nw).unwrap();
        canvas.set_draw_color(Color::GREEN);
        canvas.draw_rect(light_se).unwrap();  
        canvas.draw_rect(light_sw).unwrap(); 

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

        // test
        // canvas.set_draw_color(Color::YELLOW);
        // canvas.draw_line((0, 200), (1000, 200)).unwrap();
        // canvas.draw_line((0, 320), (1000, 320)).unwrap();  

        canvas.copy(&texture_dir, None, Some(Rect::new(10, 10, 390, 30))).unwrap();
        canvas.copy(&texture_r, None, Some(Rect::new(10, 40, 260, 30))).unwrap();
        canvas.copy(&texture_exit, None, Some(Rect::new(10, 70, 120, 30))).unwrap();

        for vehicle in &mut vehicles {
            vehicle.update();
        }

        vehicles.retain(|vehicle| !vehicle.is_off_screen());

        for vehicle in &vehicles {
            vehicle.draw(&mut canvas);
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
