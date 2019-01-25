use basin;

fn main() {
    let mut events_loop = winit::EventsLoop::new();

    let window = winit::WindowBuilder::new().with_title("Basin Window");
    let mut basin = basin::BasinWindow::new(window, &events_loop).unwrap();

    events_loop.run_forever(|event| match event {
        winit::Event::WindowEvent {
            event: winit::WindowEvent::CloseRequested,
            ..
        } => winit::ControlFlow::Break,
        winit::Event::WindowEvent {
            event: winit::WindowEvent::Refresh,
            ..
        } => {
            let dimensions: (u32, u32) = basin.get_inner_size().unwrap().to_physical(1.).into();
            let mut pixels = Vec::new();

            for y in 0..dimensions.1 {
                for x in 0..dimensions.0 {
                    let color = if ((x as f32 / 20.) as usize + (y as f32 / 20.) as usize) % 2 == 0
                    {
                        0xFF1E1E1E
                    } else {
                        0xFF3C3C3C
                    };

                    pixels.push(color);
                }
            }

            basin.draw_argb32(&pixels);
            winit::ControlFlow::Continue
        }
        _ => winit::ControlFlow::Continue,
    });
}
