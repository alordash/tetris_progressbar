use macroquad::prelude::*;
use macroquad::rand::*;

const PIXEL_LIGHT: Color = GREEN;
const PIXEL_DARK: Color = BLACK;

fn shuffle_vec<T>(vec: &mut Vec<T>) {
    for idx in (0..vec.len()).rev() {
        let rand_idx = gen_range(0, idx);
        vec.swap(idx, rand_idx);
    }
}

fn gen_shuffled_vec(size: i32) -> Vec<i32> {
    let mut vec: Vec<i32> = (0..size).collect();
    shuffle_vec(&mut vec);
    vec
}

struct Pendings {
    seed: u64,
    orders: Vec<Vec<i32>>,
}

impl Pendings {
    pub fn new(seed: u64, orders_count: i32, order_size: i32) -> Pendings {
        srand(seed);
        let gen_shuffled_vec = |_| gen_shuffled_vec(order_size);
        let orders: Vec<Vec<i32>> = (0..orders_count).map(gen_shuffled_vec).collect();
        Pendings { seed, orders }
    }

    pub fn orders(&self) -> &Vec<Vec<i32>> {
        &self.orders
    }

    pub fn update_seed(&mut self, new_seed: u64) {
        self.seed = new_seed;
        srand(self.seed);
        let gen_shuffled_vec = |_| gen_shuffled_vec(self.orders[0].len() as i32);
        self.orders = (0..self.orders.len()).map(gen_shuffled_vec).collect();
    }
}

fn update_image(img: &mut Image, step: usize, pendings: &Pendings) {
    let width = img.width();
    let height = img.height();

    let f_step = step as f32;
    let f_width = width as f32;
    let f_height = height as f32;

    for x in 0..width as u32 {
        for y in 0..height as u32 {
            img.set_pixel(x, y, PIXEL_LIGHT);
        }
    }

    let col = (f_width - (f_step / f_height).ceil()).max(0.0) as usize;
    for x in (col..width).rev() {
        for y in 0..height {
            let offset = step as i32
                - (width - x - 1) as i32 * height as i32
                - pendings.orders()[x][y] as i32;
            if offset > 0 {
                img.set_pixel(x as u32, y as u32, PIXEL_DARK);
                if (x + offset as usize) < width {
                    img.set_pixel(x as u32 + offset as u32, y as u32, PIXEL_LIGHT);
                }
            }
        }
    }
}

const HELP: &'static str =
    "[Space]: step, [LeftCtrl]: scroll, Hold [LeftShift]: reverse, [R]: randomize order, [H]: hide help";

#[macroquad::main("Tetris progressbar")]
async fn main() {
    let width: u16 = 58;
    let height: u16 = 30;

    let mut pendings = Pendings::new(
        gen_range(u64::MIN, u64::MAX),
        width as i32,
        height as i32,
    );

    let mut img = Image::gen_image_color(width, height, PIXEL_LIGHT);
    let texture = Texture2D::from_image(&img);
    texture.set_filter(FilterMode::Nearest);

    let mut step = 0i32;
    let max_step = (width * height + width - 1) as usize;

    let mut show_help = true;

    loop {
        if is_key_pressed(KeyCode::R) {
            pendings.update_seed(gen_range(u64::MIN, u64::MAX));
        }
        if is_key_pressed(KeyCode::H) {
            show_help = !show_help;
        }

        if is_key_pressed(KeyCode::Space) || is_key_down(KeyCode::LeftControl) {
            if is_key_down(KeyCode::LeftShift) {
                step = (step - 1).max(0);
            } else {
                step = (step + 1).min(max_step as i32);
            }
        }

        update_image(&mut img, step as usize, &pendings);
        texture.update(&img);
        draw_texture_ex(
            texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                // dest_size: Some(vec2(WIDTH as f32, HEIGHT as f32)),
                ..Default::default()
            },
        );
        draw_text(
            format!("step: {}", step).as_str(),
            20.0,
            20.0,
            16.0,
            DARKGRAY,
        );
        if show_help {
            draw_text(HELP, 20.0, 40.0, 16.0, DARKGRAY);
        }

        next_frame().await;
    }
}
