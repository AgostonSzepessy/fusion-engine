extern crate gl;
extern crate glutin;
extern crate libc;

mod graphics;

use graphics::model::{Model, ModelBuilder};
use graphics::texture::Texture;

fn main() {
    let window = glutin::Window::new().unwrap();

    unsafe { window.make_current() };

    unsafe {
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        gl::ClearColor(0.0, 1.0, 0.0, 1.0);
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
    }

    let bear_texture = Texture::new("bear-texture.dds").unwrap();
    let model = ModelBuilder::new("bear.obj").set_texture(bear_texture);

    for event in window.wait_events() {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) };
        window.swap_buffers();

        match event {
            glutin::Event::Closed => break,
            _ => ()
        }
    }
}
