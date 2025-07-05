use sdl2::pixels::Color;
use sdl2::rect::Rect;

#[derive(Debug, PartialEq, Clone)]
pub struct Vehicle {
    pub car: Rect,
    pub direction: String,
    pub random_route: String,
    pub color: Color,
    pub speed: i32,
}

impl Vehicle {
    pub fn new(car: Rect, direction: String, random_route: String, color: Color) -> Vehicle {
        Vehicle {
            car,
            direction,
            random_route,
            color,
            speed: 1,
        }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(self.color);
        canvas.fill_rect(self.car).unwrap();
    }

    pub fn update(&mut self) {
        match self.random_route.as_str() {
            "GoStraight" => {
                if self.direction.as_str() == "up" {
                    self.car.y -= self.speed;
                } else if self.direction.as_str() == "down" {
                    self.car.y += self.speed;
                } else if self.direction.as_str() == "left" {
                    self.car.x -= self.speed;
                } else if self.direction.as_str() == "right" {
                    self.car.x += self.speed;
                } else {
                    todo!();
                }
            }
            "TurnRight" => {
                if self.direction.as_str() == "up" {
                    if self.car.y <= 415 {
                        self.car.x += self.speed;
                    } else {
                        self.car.y -= self.speed;
                    }
                } else if self.direction.as_str() == "down" {
                    if self.car.y >= 340 {
                        self.car.x -= self.speed;
                    } else {
                        self.car.y += self.speed;
                    }
                } else if self.direction.as_str() == "left" {
                    if self.car.x >= 515 {
                        self.car.x -= self.speed;
                    } else {
                        self.car.y -= self.speed;
                    }
                } else if self.direction.as_str() == "right" {
                    if self.car.x <= 435 {
                        self.car.x += self.speed;
                    } else {
                        self.car.y += self.speed;
                    }
                } else {
                    todo!();
                }
            }
            "TurnLeft" => {
                if self.direction.as_str() == "up" {
                    if self.car.y <= 340 {
                        self.car.x -= self.speed;
                    } else {
                        self.car.y -= self.speed;
                    }
                } else if self.direction.as_str() == "down" {
                    if self.car.y >= 410 {
                        self.car.x += self.speed;
                    } else {
                        self.car.y += self.speed;
                    }
                } else if self.direction.as_str() == "left" {
                    if self.car.x >= 440 {
                        self.car.x -= self.speed;
                    } else {
                        self.car.y += self.speed;
                    }
                } else if self.direction.as_str() == "right" {
                    if self.car.x <= 510 {
                        self.car.x += self.speed;
                    } else {
                        self.car.y -= self.speed;
                    }
                } else {
                    todo!();
                }
            }
            _ => {}
        }
    }

    pub fn is_off_screen(&self) -> bool {
        self.car.x < -75 || self.car.x > 1075 || self.car.y < -75 || self.car.y > 875
    }

    pub fn is_approaching_intersection(&self) -> bool {
        match self.direction.as_str() {
            "up" => self.car.y <= 525 && self.car.y >= 475,
            "down" => self.car.y >= 275 && self.car.y + 50 <= 325,
            "right" => self.car.x + 50 <= 425 && self.car.x >= 375,
            "left" => self.car.x >= 575 && self.car.x <= 625,
            _ => false,
        }
    }

    pub fn is_in_intersection(&self) -> bool {
        // Car rectangle edges
        let car_left = self.car.x;
        let car_right = self.car.x + 50 as i32;
        let car_top = self.car.y;
        let car_bottom = self.car.y + 50 as i32;

        // Intersection rectangle edges
        let inter_left = 425;
        let inter_right = 575;
        let inter_top = 325;
        let inter_bottom = 475;

        // Check if the car rectangle intersects the intersection rectangle
        car_left < inter_right
            && car_right > inter_left
            && car_top < inter_bottom
            && car_bottom > inter_top
    }
}
