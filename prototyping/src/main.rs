use phoenix::{
    components::{
        color::RGBA,
        geometry::Triangle,
        shaders::{read_src_from_file, ShaderSource},
        Component,
    },
    managers::{entity::Entity, scene::Scene},
    renderer::opengl::OpenGL,
    window::{GlfwConfig, Resolution},
};
use std::{path::Path, rc::Rc};

fn main() {
    println!("Hello, world!");
    let config = GlfwConfig::create().unwrap();
    let window = Rc::new(
        config
            .create_window(
                "OpenGL",
                Resolution {
                    width: 800,
                    height: 600,
                },
            )
            .unwrap(),
    );

    window.set_current().unwrap();

    let render = Box::<OpenGL>::default();
    let mut scene = Scene::new(window, render);

    let vertices: [f32; 9] = [
        -0.2, -0.2, 0.0, // left
        0.0, 0.0, 0.0, // right
        0.5, 0.0, 0.0, // top
    ];

    let second_vertices: [f32; 9] = [
        0.4, 0.4, 0.0, // left
        0.9, 0.9, 0.0, // right
        0.0, 0.9, 0.0, // top
    ];

    scene.set_background_color(RGBA::from_hex(0xff_a5_90_FF));
    let triangle = Triangle::new(vertices);
    let green_triangle = Entity::new(vec![Component::Geometry(Box::new(triangle)), Component::Color(RGBA::from_hex(0x00_FF_00_FF))]);
    let blue_triangle = Entity::new(vec![Component::Geometry(Box::new(Triangle::new(second_vertices))), Component::Color(RGBA::from_hex(0x00_00_FF_FF))]);

    scene.add_entity(green_triangle);
    scene.add_entity(blue_triangle);
    scene.start().unwrap();
}
