use rusttype::{Font, Scale};

fn main() {
    const FONT_BYTES: &[u8] = include_bytes!("./fonts/FiraCode-Retina.ttf");
    let font: Font<'static> = Font::from_bytes(FONT_BYTES).unwrap();
    let em_space_char = '\u{2003}';
    let scale =  Scale::uniform(60.0);
    let scaled = font.glyph(em_space_char).scaled(scale);
    let h_metrics = scaled.h_metrics();
    let v_metrics = font.v_metrics(scale);
    dbg!(h_metrics, v_metrics);
}
