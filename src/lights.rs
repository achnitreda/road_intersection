use crate::{Roads::Roads, traffic::*};
use sdl2::{hint::set, pixels::Color};

#[derive(PartialEq, Clone, Debug)]
pub enum TrafficPhase {
    Up,
    Down,
    Right,
    Left,
}
#[derive(PartialEq, Clone, Debug)]

pub struct AdvancedTrafficSystem {
    phase: TrafficPhase,
    timer: u32,
    phase_duration: u32,
    is_timeover: bool,
}

impl AdvancedTrafficSystem {
    pub fn new() -> Self {
        AdvancedTrafficSystem {
            phase: TrafficPhase::Up,
            timer: 0,
            phase_duration: 240,
            is_timeover: false,
        }
    }

    pub fn update(&mut self, vehicles: &Vec<Vehicle>, roads: &Roads) {
        self.timer += 1;

        let should_extend = self.should_extend_phase(vehicles);
        if self.timer >= self.phase_duration {
            self.is_timeover = true;
            // self.next_phase();
        }

        if self.timer >= self.phase_duration && !should_extend {
            self.next_phase(roads);
            self.timer = 0;
            self.is_timeover = false;
        }
    }

    pub fn should_extend_phase(&self, vehicles: &Vec<Vehicle>) -> bool {
        let current_direction = match self.phase {
            TrafficPhase::Up => "up",
            TrafficPhase::Down => "down",
            TrafficPhase::Left => "left",
            TrafficPhase::Right => "right",
        };

        vehicles
            .iter()
            .any(|v| v.direction == current_direction && v.is_in_intersection())
    }

    pub fn next_phase(&mut self, roads: &Roads) {
        let max = roads.full(); // (String, i32)

        if max.1 >= 4 && self.timer.saturating_sub(self.phase_duration) < 30 {
            self.phase = match max.0.as_str() {
                "up" => TrafficPhase::Up,
                "down" => TrafficPhase::Down,
                "left" => TrafficPhase::Left,
                "right" => TrafficPhase::Right,
                _ => self.phase.clone(), // fallback if somehow invalid
            };
            return; // Done updating phase
        }

        // Rotate to the next phase
        self.phase = match self.phase {
            TrafficPhase::Up => TrafficPhase::Down,
            TrafficPhase::Down => TrafficPhase::Left,
            TrafficPhase::Left => TrafficPhase::Right,
            TrafficPhase::Right => TrafficPhase::Up,
        };
    }

    pub fn can_vehicle_proceed(&self, vehicle: &Vehicle, traffic: &AdvancedTrafficSystem) -> bool {
        if !vehicle.is_approaching_intersection() {
            return true;
        }

        if vehicle.is_approaching_intersection() && traffic.timer > traffic.phase_duration {
            return false;
        }

        let allowed_direction = match self.phase {
            TrafficPhase::Up => "up",
            TrafficPhase::Down => "down",
            TrafficPhase::Left => "left",
            TrafficPhase::Right => "right",
        };
        vehicle.direction == allowed_direction
    }

    pub fn get_light_colors(&self) -> (Color, Color, Color, Color) {
        if self.is_timeover {
            return (Color::RED, Color::RED, Color::RED, Color::RED);
        }
        match self.phase {
            TrafficPhase::Down => (Color::GREEN, Color::RED, Color::RED, Color::RED),
            TrafficPhase::Up => (Color::RED, Color::GREEN, Color::RED, Color::RED),
            TrafficPhase::Left => (Color::RED, Color::RED, Color::GREEN, Color::RED),
            TrafficPhase::Right => (Color::RED, Color::RED, Color::RED, Color::GREEN),
        }
    }
}
