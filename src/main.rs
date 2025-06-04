use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

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

    let surface = font.render("Esc: Exit simulation")
        .blended(Color::WHITE)
        .unwrap();

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();


    let car_rect = sdl2::rect::Rect::new(200, 200, 50, 50);
    let light_ne = sdl2::rect::Rect::new(375, 275, 50, 50);
    let light_nw = sdl2::rect::Rect::new(575, 275, 50, 50);
    let light_se = sdl2::rect::Rect::new(375, 475, 50, 50);
    let light_sw = sdl2::rect::Rect::new(575, 475, 50, 50);
 
    canvas.set_draw_color(Color::RGB(0, 55, 55));
    
    canvas.clear(); 
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 55, 55));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        
        canvas.set_draw_color(Color::WHITE);
        // North-South road (vertical)
        canvas.draw_line((500, 0), (500, 800)).unwrap();
        canvas.draw_line((575, 0), (575, 800)).unwrap();
        canvas.draw_line((425, 0), (425, 800)).unwrap();
        // East-West road (horizontal) 
        canvas.draw_line((0, 400), (1000, 400)).unwrap();
        canvas.draw_line((0, 325), (1000, 325)).unwrap();
        canvas.draw_line((0, 475), (1000, 475)).unwrap();

        canvas.set_draw_color(Color::RED);
        canvas.draw_rect(light_ne).unwrap();  
        canvas.draw_rect(light_nw).unwrap();
        canvas.set_draw_color(Color::GREEN);
        canvas.draw_rect(light_se).unwrap();  
        canvas.draw_rect(light_sw).unwrap();  

        canvas.set_draw_color(Color::WHITE);
        canvas.fill_rect(car_rect).unwrap();  

        canvas.copy(&texture, None, Some(sdl2::rect::Rect::new(10, 10, 120, 30))).unwrap();
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}