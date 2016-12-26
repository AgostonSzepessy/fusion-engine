extern crate gl;
extern crate glutin;
extern crate libc;

mod graphics;

use graphics::Model;

fn main() {
    let window = glutin::Window::new().unwrap();

    unsafe { window.make_current() };

    unsafe {
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        gl::ClearColor(0.0, 1.0, 0.0, 1.0);
        gl::GlEnable(gl::GL_DEPTH_TEST);
        gl::GlDepthFunc(gl::GL_LESS);
    }

    let model = Model::new("bear.obj");

    for event in window.wait_events() {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::GL_DEPTH_BUFFER_BIT) };
        window.swap_buffers();

        match event {
            glutin::Event::Closed => break,
            _ => ()
        }
    }
}
