use std::time::Instant;

use bit_vec::BitVec;
use rusttype::{Font, point, PositionedGlyph, Scale};

type SomeErr = Box<dyn std::error::Error>;


struct PixelBuffer {
    width: usize,
    height: usize,
    storage: BitVec,
}

impl PixelBuffer {
    fn new(width: usize, height: usize) -> Self {
        // make sure width and height is a multiple of 2
        let width = width + (width % 2);
        let height = height + (height % 2);
        let storage = BitVec::from_elem(width * height, false);
        PixelBuffer { width, height, storage }
    }

    #[inline(always)]
    fn set_pixel(&mut self, x: usize, y: usize) {
        assert!((0..self.width).contains(&x) & &(0..self.height).contains(&y));
        self.storage.set(y * self.width + x, true);
    }

    fn render(&self) -> String {
        fn get_unicode_block_element(quadrants: &(bool, bool, bool, bool)) -> char {
            match quadrants {
                (false, false, false, false) => ' ',  // 0 0 0 0   0
                (false, false, false, true) => '▗',  // 0 0 0 1   1
                (false, false, true, false) => '▖',  // 0 0 1 0   2
                (false, false, true, true) => '▄',  // 0 0 1 1   3
                (false, true, false, false) => '▝',  // 0 1 0 0   4
                (false, true, false, true) => '▐',  // 0 1 0 1   5
                (false, true, true, false) => '▞',  // 0 1 1 0   6
                (false, true, true, true) => '▟',  // 0 1 1 1   7
                (true, false, false, false) => '▘',  // 1 0 0 0   8
                (true, false, false, true) => '▚',  // 1 0 0 1   9
                (true, false, true, false) => '▌',  // 1 0 1 0   A
                (true, false, true, true) => '▙',  // 1 0 1 1   B
                (true, true, false, false) => '▀',  // 1 1 0 0   C
                (true, true, false, true) => '▜',  // 1 1 0 1   D
                (true, true, true, false) => '▛',  // 1 1 1 0   E
                (true, true, true, true) => '█',  // 1 1 1 1   F
            }
        }


        let mut output = String::new();
        // width and height are always a multiple of 2!
        let width_in_blocks = self.width / 2;
        let height_in_blocks = self.height / 2;

        for block_y in 0..height_in_blocks {
            for block_x in 0..width_in_blocks {
                let subpixel = (
                    self.storage[block_y * 2 * self.width + block_x * 2],
                    self.storage[block_y * 2 * self.width + block_x * 2 + 1],
                    self.storage[(block_y * 2 + 1) * self.width + block_x * 2],
                    self.storage[(block_y * 2 + 1) * self.width + block_x * 2 + 1],
                );

                let character = get_unicode_block_element(&subpixel);
                output.push(character);
            }
            output.push('\n')
        }

        output
    }
}

fn main() -> Result<(), SomeErr> {
    let font_data = std::fs::read("Arvo-Regular.ttf")?;
    let font_collection = rusttype::FontCollection::from_bytes(font_data)?;
    let font = font_collection.into_font()?;

    let font_scale = rusttype::Scale { x: 20.0, y: 10.0 };

    let start = Instant::now();
    let result = render_text("Heading".to_string(), &font, font_scale, 0.4)?;
    println!("{}", result);
    println!("{:?}", Instant::now().duration_since(start));
    Ok(())
}

fn render_text(text: String,
               font: &Font,
               font_scale: Scale,
               opacity_threshold: f32) -> Result<String, SomeErr> {
    let v_metrics = font.v_metrics(font_scale);
    let offset = point(0.0, v_metrics.ascent);
    let glyphs: Vec<PositionedGlyph<'_>> = font
        .layout(&text, font_scale, offset)
        .collect();

    let width = glyphs
        .last()
        .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
        .unwrap_or(0.0)
        .ceil() as usize;

    let mut buffer = PixelBuffer::new(width, font_scale.y.ceil() as usize);

    glyphs
        .iter()
        .flat_map(|glyph| {
            let bounding_box = glyph.pixel_bounding_box()?;
            Some((glyph, bounding_box))
        })
        .for_each(|(glyph, bounding_box)| {
            glyph.draw(|x, y, v| {
                if v >= opacity_threshold {
                    let x = (x as i32 + bounding_box.min.x) as usize;
                    let y = (y as i32 + bounding_box.min.y) as usize;
                    buffer.set_pixel(x as usize, y as usize);
                }
            })
        });


    Ok(buffer.render())
}