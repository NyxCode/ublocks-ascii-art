use bit_vec::BitVec;

pub trait Raster {
    fn new(width: usize, height: usize) -> Self;

    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn set_pixel(&mut self, x: usize, y: usize);
    fn render(&self) -> String;
}

pub struct SimpleRaster { width: usize, height: usize, storage: BitVec }

pub struct RotatedRaster<R: Raster>(R);

pub type RotatedRaster90<R> = RotatedRaster<R>;
pub type RotatedRaster180<R> = RotatedRaster90<RotatedRaster90<R>>;
pub type RotatedRaster270<R> = RotatedRaster90<RotatedRaster180<R>>;

impl Raster for SimpleRaster {
    fn new(width: usize, height: usize) -> Self {
        // make sure width and height is a multiple of 2
        let width = width + (width % 2);
        let height = height + (height % 2);
        let storage = BitVec::from_elem(width * height, false);
        SimpleRaster { width, height, storage }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn set_pixel(&mut self, x: usize, y: usize) {
        self.storage.set(y * self.width + x, true);
    }

    fn render(&self) -> String {
        fn get_unicode_block_element(quadrants: (bool, bool, bool, bool)) -> char {
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

                let character = get_unicode_block_element(subpixel);
                output.push(character);
            }
            output.push('\n')
        }

        output
    }
}

impl<R: Raster> Raster for RotatedRaster<R> {
    fn new(width: usize, height: usize) -> Self {
        RotatedRaster(R::new(height, width))
    }

    fn width(&self) -> usize {
        self.0.height()
    }

    fn height(&self) -> usize {
        self.0.width()
    }

    fn set_pixel(&mut self, x: usize, y: usize) {
        self.0.set_pixel(y, self.0.height() - 1 - x)
    }

    fn render(&self) -> String {
        self.0.render()
    }
}