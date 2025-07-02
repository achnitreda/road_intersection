use crate::traffic::Direction;

#[derive(Clone, Copy, PartialEq)]
pub enum LightState {
    Red,
    Green,
}

pub struct TrafficLight {
    pub direction: Direction,
    pub state: LightState,
}

pub struct TrafficSystem {
    pub lights: Vec<TrafficLight>,
    pub timer: u32,
    pub duration: u32, // seconds to stay green
    pub current_green: usize,
}

impl TrafficSystem {
    pub fn new() -> Self {
        let lights = vec![
            TrafficLight {
                direction: Direction::North,
                state: LightState::Red,
            },
            TrafficLight {
                direction: Direction::South,
                state: LightState::Red,
            },
            TrafficLight {
                direction: Direction::East,
                state: LightState::Red,
            },
            TrafficLight {
                direction: Direction::West,
                state: LightState::Green,
            },
        ];
        Self {
            lights,
            timer: 0,
            duration: 120,    // 2 seconds at 60 fps
            current_green: 3, // index of green light (e.g., West)
        }
    }

    pub fn update(&mut self) {
        self.timer += 1;
        if self.timer >= self.duration {
            // Rotate green light
            self.lights[self.current_green].state = LightState::Red;
            self.current_green = (self.current_green + 1) % self.lights.len();
            self.lights[self.current_green].state = LightState::Green;
            self.timer = 0;
        }
    }

    pub fn is_green(&self, dir: Direction) -> bool {
        self.lights
            .iter()
            .find(|light| light.direction == dir)
            .map(|light| light.state == LightState::Green)
            .unwrap_or(false)
    }
}
