use macroquad::prelude::*;

const GOLD_BYTES: [u8; 4] = [255, 215, 0, 255];
const BLACK_BYTES: [u8; 4] = [0, 0, 0, 255];
const PALETTE: [[u8; 4]; 2] = [BLACK_BYTES, GOLD_BYTES];

#[derive(Debug)]
struct Grid {
    cols: usize,
    rows: usize,
    cells: Vec<bool>,
    img: Image,
}

impl Grid {
    fn new(w: usize, h: usize) -> Self {
        Grid {
            cols: w,
            rows: h,
            cells: vec![false; w * h],
            img: Image::gen_image_color(w as u16, h as u16, BLACK),
        }
    }
    fn random_fill(&mut self, p_start: f32, p_end: f32){
        let mut p = p_start;
        let r = (p_end / p_start).powf(1.0 / self.rows as f32);
        for y in 0..self.rows - 1 {
            for x in 0..self.cols {
                self.set(x, y, rand::gen_range(0.0, 1.0) < p);
            }
            p = p * r;
        }
    }
    fn get(&self, x: usize, y: usize) -> bool {
        (x < self.cols) & (y < self.rows) && self.cells[y * self.cols + x]
    }
    fn set(&mut self, x: usize, y: usize, val: bool) {
        if (x < self.cols) & (y < self.rows) {
            let idx = y * self.cols + x;
            let old_val = self.cells[idx];
            if old_val != val {
                self.cells[idx] = val;
                self.img.bytes[(idx << 2)..(idx << 2) + 4].copy_from_slice(&PALETTE[val as usize]);
            }
        }
    }
    fn step(&mut self) {
        for y in (0..self.rows - 1).rev() {
            for x in 0..self.cols {
                if self.get(x, y) == true && self.get(x, y + 1) == false {
                    if rand::gen_range(0.0, 1.0) > 0.08{
                        self.set(x, y, false);
                        self.set(x, y + 1, true);
                    }
                    else if self.get(x, y + 2) == false{
                        self.set(x, y, false);
                        self.set(x, y + 2, true);
                    }
                }
            }
        }
    }
}

#[macroquad::main("Rusty Sandfall")]
async fn main() {
   // rand::srand(macroquad::miniquad::date::now() as u64);
    let mut grid = Grid::new(512, 512);
    grid.random_fill(0.9, 0.01);
    let texture = Texture2D::from_image(&grid.img);
    texture.set_filter(FilterMode::Nearest);

    let mut last_update_time = 0.0;
    let update_interval = 0.02;
    let mut is_pressed: bool = false;

    loop {
        clear_background(BLACK);

        let current_time = get_time();
        let time_passed = current_time - last_update_time > update_interval;

        if is_mouse_button_pressed(MouseButton::Left){
            is_pressed = true;
        }

        if is_pressed && time_passed{
            grid.step();
            texture.update(&grid.img);
            last_update_time = current_time;
        }

        draw_texture_ex(
            &texture, 
            0.0, 
            0.0, 
            WHITE, 
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            }
        );
        next_frame().await
    }
}
