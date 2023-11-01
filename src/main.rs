use std::time::Instant;
use winit::{event_loop::{EventLoop, ControlFlow}, event::{Event, WindowEvent, KeyboardInput, VirtualKeyCode, ElementState}};
use winit::platform::run_return::EventLoopExtRunReturn;
use math::Transform;

mod engine;
mod object;
mod camera;
mod dir_light;
mod texture;
mod render;
mod text;
mod gimap;

fn main() {
    let grass = Box::leak(Box::new(
        texture::Texture::load("assets/grass.jpg")
    ));
    let terraccota = Box::leak(Box::new(
        texture::Texture::load("assets/terracotta.jpg")
    ));
    
    let objects = Box::leak(Box::new([
        object::Object::load("assets/cube.gltf", terraccota, Transform::from_translation(3., 0., 0.)),
        object::Object::load("assets/cube.gltf", terraccota, Transform::from_translation(0., -3., 0.)),
        object::Object::load("assets/cube.gltf", terraccota, Transform::from_translation(0., 0., 3.)),
        object::Object::load("assets/cube.gltf", grass, Transform::from_scale(5., 0.01, 5.).with_translation(0., 1.5, 0.))
    ]));
    
    let gi_log = text::Log::default();
    let _gi_log = gi_log.clone();
    let render_log = text::Log::default();
    let logs = vec![render_log.clone(), gi_log];

    let mut event_loop = EventLoop::new();
    let mut engine = engine::Engine::new(&event_loop, objects, logs);
    
    std::thread::spawn(|| {
        let gi_log = _gi_log;
        loop {
            let start = Instant::now();
            for object in objects.iter() {
                object.gimap.update(objects, engine.dir_light)
            }
            gi_log.set(format!("GI {}ms", (Instant::now() - start).as_millis()));
        }
    });
    
    event_loop.run_return(move |e, _, control_flow| {
        match e {
            Event::MainEventsCleared => engine.window.request_redraw(),
            Event::RedrawRequested(_) => {
                let start = Instant::now();
                engine.update();
                render_log.set(format!("Main {}ms", (Instant::now() - start).as_millis()));
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
    });
}