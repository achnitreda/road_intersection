use crate::traffic::{Direction, Vehicle};
use crate::traffic_light::{LightState, TrafficSystem};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn draw_roads(canvas: &mut Canvas<Window>) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(50, 50, 50));
    canvas.fill_rect(Rect::new(360, 0, 80, 800))?; // vertical road
    canvas.fill_rect(Rect::new(0, 360, 800, 80))?; // horizontal road
    Ok(())
}

pub fn draw_vehicles(canvas: &mut Canvas<Window>, vehicles: &[Vehicle]) -> Result<(), String> {
    for v in vehicles {
        canvas.set_draw_color(v.color);
        let rect = Rect::from_center(v.pos, 20, 10);
        canvas.fill_rect(rect)?;
    }
    Ok(())
}

pub fn draw_traffic_lights(
    canvas: &mut Canvas<Window>,
    lights: &TrafficSystem,
) -> Result<(), String> {
    for light in &lights.lights {
        let (x, y) = match light.direction {
            Direction::North => (350, 350), // top-left corner
            Direction::South => (430, 450), // bottom-right corner
            Direction::West => (350, 450),  // bottom-left corner
            Direction::East => (450, 350),  // top-right corner
        };
        canvas.set_draw_color(match light.state {
            LightState::Green => Color::RGB(0, 255, 0),
            LightState::Red => Color::RGB(255, 0, 0),
        });
        canvas.fill_rect(Rect::new(x, y, 10, 10))?;
    }
    Ok(())
}
