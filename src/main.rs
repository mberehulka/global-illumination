use std::time::Instant;
use winit::{event_loop::{EventLoop, ControlFlow}, event::{Event, WindowEvent, KeyboardInput, VirtualKeyCode, ElementState}};
use math::Transform;

mod engine;
mod object;
mod camera;
mod dir_light;
mod color;

fn main() {
    let event_loop = EventLoop::new();
    let mut engine = engine::Engine::new(&event_loop);

    engine.objects.push(object::Object::load("assets/cube.gltf", Transform::from_translation(3., 0., 0.)));
    engine.objects.push(object::Object::load("assets/cube.gltf", Transform::from_translation(0., -3., 0.)));
    engine.objects.push(object::Object::load("assets/cube.gltf", Transform::from_translation(0., 0., 3.)));
    engine.objects.push(object::Object::load("assets/cube.gltf", Transform::from_scale(5., 0.01, 5.).with_translation(0., 1.5, 0.)));
    
    event_loop.run(move |e, _, control_flow| {
        match e {
            Event::MainEventsCleared => engine.window.request_redraw(),
            Event::RedrawRequested(_) => {
                let start = Instant::now();
                engine.update();
                println!("{}ms", (Instant::now() - start).as_millis());
            },
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode, state, .. }, .. } =>
                    if let Some(code) = virtual_keycode {
                        if let ElementState::Released = state { return }
                        match code {
                            VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
                            VirtualKeyCode::W => engine.camera.distance -= 0.1,
                            VirtualKeyCode::S => engine.camera.distance += 0.1,
                            VirtualKeyCode::A => engine.camera.rotation.y += 0.1,
                            VirtualKeyCode::D => engine.camera.rotation.y -= 0.1,
                            VirtualKeyCode::Q => engine.camera.rotation.x -= 0.1,
                            VirtualKeyCode::E => engine.camera.rotation.x += 0.1,
                            VirtualKeyCode::R => engine.camera.translation.y += 0.1,
                            VirtualKeyCode::F => engine.camera.translation.y -= 0.1,
                            _ => {}
                        }
                    }
                _ => {}
            }
            _ => {}
        }
    })
}