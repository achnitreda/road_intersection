use crate::traffic_light::TrafficSystem;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Point;

pub const MIN_SPAWN_DISTANCE: i32 = 40;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn random() -> Self {
        match rand::thread_rng().gen_range(0..4) {
            0 => Self::North,
            1 => Self::South,
            2 => Self::East,
            _ => Self::West,
        }
    }

    pub fn start_position(&self) -> Point {
        match self {
            Self::North => Point::new(390, 0),
            Self::South => Point::new(410, 800),
            Self::East => Point::new(800, 390),
            Self::West => Point::new(0, 410),
        }
    }

    pub fn velocity(&self) -> (i32, i32) {
        match self {
            Self::North => (0, 2),
            Self::South => (0, -2),
            Self::East => (-2, 0),
            Self::West => (2, 0),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Route {
    Left,
    Right,
    Straight,
}

impl Route {
    pub fn random() -> Self {
        match rand::thread_rng().gen_range(0..3) {
            0 => Self::Left,
            1 => Self::Right,
            _ => Self::Straight,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Self::Left => Color::RGB(255, 165, 0),   // orange
            Self::Right => Color::RGB(0, 255, 0),    // green
            Self::Straight => Color::RGB(0, 0, 255), // blue
        }
    }
}
#[derive(Clone)]
pub struct Vehicle {
    pub pos: Point,
    pub dir: Direction,
    pub route: Route,
    pub color: Color,
    pub velocity: (i32, i32),
    pub turned: bool,
}

pub fn spawn_vehicle(dir: Direction) -> Vehicle {
    let pos = dir.start_position();
    let route = Route::random();
    let velocity = dir.velocity();
    let color = route.color();

    Vehicle {
        pos,
        dir,
        route,
        color,
        velocity,
        turned: false,
    }
}

impl Vehicle {
    pub fn update(&mut self, lights: &TrafficSystem, others: &[Vehicle]) {
        if self.is_blocked_by_front_vehicle(others) {
            return; // blocked by vehicle ahead
        }

        if self.should_stop_for_light(lights) {
            return; // blocked by red light
        }

        if !self.turned && self.is_at_intersection() {
            self.turn();
            self.turned = true;
        }

        self.pos = self.pos.offset(self.velocity.0, self.velocity.1);
    }

    fn is_blocked_by_front_vehicle(&self, others: &[Vehicle]) -> bool {
        for other in others {
            if std::ptr::eq(self, other) || self.dir != other.dir {
                continue;
            }

            match self.dir {
                Direction::North => {
                    if other.pos.y < self.pos.y
                        && (self.pos.y - other.pos.y) < MIN_SPAWN_DISTANCE
                        && (self.pos.x - other.pos.x).abs() < 20
                    {
                        return true;
                    }
                }
                Direction::South => {
                    if other.pos.y > self.pos.y
                        && (other.pos.y - self.pos.y) < MIN_SPAWN_DISTANCE
                        && (self.pos.x - other.pos.x).abs() < 20
                    {
                        return true;
                    }
                }
                Direction::East => {
                    if other.pos.x > self.pos.x
                        && (other.pos.x - self.pos.x) < MIN_SPAWN_DISTANCE
                        && (self.pos.y - other.pos.y).abs() < 20
                    {
                        return true;
                    }
                }
                Direction::West => {
                    if other.pos.x < self.pos.x
                        && (self.pos.x - other.pos.x) < MIN_SPAWN_DISTANCE
                        && (self.pos.y - other.pos.y).abs() < 20
                    {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn should_stop_for_light(&self, lights: &TrafficSystem) -> bool {
        match self.dir {
            Direction::North if self.pos.y >= 310 && self.pos.y <= 330 => {
                !lights.is_green(self.dir)
            }
            Direction::South if self.pos.y <= 470 && self.pos.y >= 450 => {
                !lights.is_green(self.dir)
            }
            Direction::West if self.pos.x >= 310 && self.pos.x <= 330 => !lights.is_green(self.dir),
            Direction::East if self.pos.x <= 470 && self.pos.x >= 450 => !lights.is_green(self.dir),
            _ => false,
        }
    }

    fn is_at_intersection(&self) -> bool {
        (380..420).contains(&self.pos.x) && (380..420).contains(&self.pos.y)
    }

    fn turn(&mut self) {
        match (self.dir, self.route) {
            (Direction::North, Route::Left) => {
                self.dir = Direction::West;
                self.velocity = (2, 0);
            }
            (Direction::North, Route::Right) => {
                self.dir = Direction::East;
                self.velocity = (-2, 0);
            }
            (Direction::South, Route::Left) => {
                self.dir = Direction::East;
                self.velocity = (-2, 0);
            }
            (Direction::South, Route::Right) => {
                self.dir = Direction::West;
                self.velocity = (2, 0);
            }
            (Direction::East, Route::Left) => {
                self.dir = Direction::North;
                self.velocity = (0, 2);
            }
            (Direction::East, Route::Right) => {
                self.dir = Direction::South;
                self.velocity = (0, -2);
            }
            (Direction::West, Route::Left) => {
                self.dir = Direction::South;
                self.velocity = (0, -2);
            }
            (Direction::West, Route::Right) => {
                self.dir = Direction::North;
                self.velocity = (0, 2);
            }
            (_, Route::Straight) => {}
        }
    }
}
