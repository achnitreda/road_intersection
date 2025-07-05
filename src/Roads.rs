use crate::traffic::*;

#[derive(PartialEq, Clone, Debug)]

pub struct Roads {
    pub Up: Vec<Vehicle>,
    pub Down: Vec<Vehicle>,
    pub Left: Vec<Vehicle>,
    pub Right: Vec<Vehicle>,
}

impl Roads {
    pub fn new() -> Self {
        Roads {
            Up: Vec::new(),
            Down: Vec::new(),
            Left: Vec::new(),
            Right: Vec::new(),
        }
    }

    /// Push a vehicle into the appropriate direction queue
    pub fn push(&mut self, v: Vehicle) {
        match v.direction.as_str() {
            "up" => self.Up.push(v),
            "down" => self.Down.push(v),
            "left" => self.Left.push(v),
            "right" => self.Right.push(v),
            _ => eprintln!("Unknown direction: {}", v.direction),
        }
    }
    pub fn pop(&mut self, v: &Vehicle) -> Option<Vehicle> {
        let dir = v.direction.as_str();
        let vec = match dir {
            "up" => &mut self.Up,
            "down" => &mut self.Down,
            "left" => &mut self.Left,
            "right" => &mut self.Right,
            _ => {
                eprintln!("Unknown direction: {}", dir);
                return None;
            }
        };

        // Find the index of the vehicle
        if let Some(pos) = vec.iter().position(|x| x == v) {
            Some(vec.remove(pos))
        } else {
            None
        }
    }

    pub fn full(&self) -> (String, i32) {
        // Collect (direction name, length)
        let counts = [
            ("up", self.Up.len()),
            ("down", self.Down.len()),
            ("left", self.Left.len()),
            ("right", self.Right.len()),
        ];

        // Find the tuple with the maximum length
        let (dir, len) = counts.iter().max_by_key(|(_, len)| *len).unwrap(); // Safe because there are always 4 entries

        (dir.to_string(), *len as i32)
    }
}
