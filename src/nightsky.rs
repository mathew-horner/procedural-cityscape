use image::{ImageBuffer, Rgb};
use rand::{rngs::ThreadRng, Rng};

use crate::math::{Dimensions2, Vector2};

/// Parameters for `nightsky::render`.
pub struct RenderOpts<'a> {
    /// The image to render to.
    pub image: &'a mut ImageBuffer<Rgb<u8>, Vec<u8>>,

    /// Random number generator to use.
    pub rng: &'a mut ThreadRng,

    /// The number of cells in each dimension (x, y).
    pub cell_count: Vector2<u32>,

    /// The dimensions of each cell.
    pub cell_size: Dimensions2<u32>,

    /// The probability that a star will be rendered in each cell.
    pub star_presence_prob: f64,

    /// The probability that a star will be a "big" star.
    pub star_enlargement_prob: f64,

    /// The color to render stars with.
    pub star_color: Rgb<u8>,
}

/// Render a randomized night sky to the given image.
///
/// This function randomly generates stars by partitioning the screen space into cells and
/// choosing whether and where to include a star in each.
pub fn render(opts: RenderOpts<'_>) {
    for row in 0..opts.cell_count.y() {
        for col in 0..opts.cell_count.x() {
            if opts.rng.gen::<f64>() > opts.star_presence_prob {
                continue;
            }

            // Regular stars have a size of 1. If we should make a big star, then the outcome of
            // the conditional will be 1, and we add 1 to make a star size of 2.
            let is_big = opts.rng.gen::<f64>() <= opts.star_enlargement_prob;
            let star_size = u32::from(is_big) + 1;

            let offset = random_star_offset(&opts.cell_size, opts.rng);
            let x = (col * opts.cell_size.width()) + offset.x();
            let y = (row * opts.cell_size.height()) + offset.y();

            for i in 0..star_size {
                for j in 0..star_size {
                    opts.image.put_pixel(x + i, y + j, opts.star_color);
                }
            }
        }
    }
}

/// Generate a random offset to place the star inside its cell.
///
/// Grid-locking the stars would result in a boring effect, this offset generation provides some
/// stagger to the location of each star, even if the algorithm is based on grid cells.
fn random_star_offset(cell_size: &Dimensions2<u32>, rng: &mut ThreadRng) -> Vector2<u32> {
    let x = rng.gen_range(0..cell_size.width());
    let y = rng.gen_range(0..cell_size.height());
    Vector2::new(x, y)
}
