use rusttype::{point, PositionedGlyph, Scale};

pub use raster::*;

mod raster;

pub type SomeErr = Box<dyn std::error::Error>;

#[cfg(feature = "font-badscript")]
const BADSCRIPT_FONT: &[u8] = include_bytes!("../fonts/BadScript-Regular.ttf");
#[cfg(feature = "font-roboto")]
const ROBOTO_FONT: &[u8] = include_bytes!("../fonts/Roboto-Regular.ttf");
#[cfg(feature = "font-roboto-mono")]
const ROBOTO_MONO_FONT: &[u8] = include_bytes!("../fonts/RobotoMono-Regular.ttf");

pub enum Font<'f> {
    Custom(rusttype::Font<'f>),
    #[cfg(feature = "font-badscript")]
    BadScript,
    #[cfg(feature = "font-roboto")]
    Roboto,
    #[cfg(feature = "font-roboto-mono")]
    RobotoMono,
}

impl<'f> Font<'f> {
    fn get_font(self) -> rusttype::Font<'f> {
        match self {
            Font::Custom(font) => font,
            #[cfg(feature = "font-badscript")]
            Font::BadScript => Font::parse_font(BADSCRIPT_FONT),
            #[cfg(feature = "font-roboto")]
            Font::Roboto => Font::parse_font(ROBOTO_FONT),
            #[cfg(feature = "font-roboto-mono")]
            Font::RobotoMono => Font::parse_font(ROBOTO_MONO_FONT),
        }
    }

    fn parse_font(data: &[u8]) -> rusttype::Font {
        rusttype::FontCollection::from_bytes(data).unwrap().into_font().unwrap()
    }
}

pub fn render_text<R: Raster>(text: &str,
                              font: Font,
                              font_scale: (f32, f32)) -> Result<String, SomeErr> {
    const OPACITY_THRESHOLD: f32 = 0.4;

    let font = font.get_font();
    let font_scale = Scale { x: font_scale.0, y: font_scale.1 };

    let glyphs = get_glyphs(&font, font_scale, &text);

    let width = glyphs
        .last()
        .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
        .unwrap_or(0.0)
        .ceil() as usize;

    let mut buffer = R::new(width, font_scale.y.ceil() as usize);

    glyphs
        .iter()
        .flat_map(|glyph| Some((glyph, glyph.pixel_bounding_box()?)))
        .for_each(|(glyph, bounding_box)| {
            glyph.draw(|x, y, v| {
                if v > OPACITY_THRESHOLD {
                    let x = (x as i32 + bounding_box.min.x) as usize;
                    let y = (y as i32 + bounding_box.min.y) as usize;
                    if (0..width).contains(&x) && (0..(font_scale.y as usize)).contains(&y) {
                        buffer.set_pixel(x as usize, y as usize);
                    }
                }
            })
        });

    let rendered = buffer.render();
    let trimmed = trim_blank_lines(&rendered);
    Ok(trimmed.to_owned())
}

fn get_glyphs<'f>(font: &rusttype::Font<'f>, scale: Scale, text: &str) -> Vec<PositionedGlyph<'f>> {
    let v_metrics = font.v_metrics(scale);
    let offset = point(0.0, v_metrics.ascent);
    font.layout(text, scale, offset)
        .collect()
}

fn trim_blank_lines(string: &str) -> &str {
    let mut start = None;
    let mut end = 0;

    let mut is_current_line_blank = true;
    let mut current_line_start_index = 0;
    let mut character_index = 0;

    for character in string.chars().chain(Some('\n')) {
        match character {
            '\n' => {
                if !is_current_line_blank {
                    let _ = start.get_or_insert(current_line_start_index);
                    end = character_index;
                }

                current_line_start_index = character_index + character.len_utf8();
                is_current_line_blank = true;
            }
            _ if character != ' ' => is_current_line_blank = false,
            _ => {}
        }
        character_index += character.len_utf8();
    }

    &string[start.unwrap_or(0)..end]
}
