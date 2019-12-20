use rusttype::{Font, Scale};
use glutin::{Api, GlProfile, GlRequest};

fn main() {
    let events = glutin::event_loop::EventLoop::new();

    let glutin_context = glutin::ContextBuilder::new()
    .with_gl_profile(GlProfile::Core)
    .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
    .with_srgb(true)
    .with_depth_buffer(24)
    .build_windowed(
        glutin::window::WindowBuilder::new()
            .with_inner_size((1024, 576).into()),
        &events,
    ).unwrap();
    
    let hidpi_factor = glutin_context.window().hidpi_factor() as f32;
    dbg!(hidpi_factor);
    const FONT_BYTES: &[u8] = include_bytes!("./fonts/FiraCode-Retina.ttf");
    let font: Font<'static> = Font::from_bytes(FONT_BYTES).unwrap();
    let em_space_char = '\u{2003}';
    let scale =  Scale::uniform((60.0 * hidpi_factor).round());
    let scaled = font.glyph(em_space_char).scaled(scale);
    let h_metrics = scaled.h_metrics();
    let v_metrics = font.v_metrics(scale);
    dbg!(h_metrics, v_metrics);
}
