use macroquad::prelude::*;

#[derive(Debug)]
struct Grid {
    cols: usize,
    rows: usize,
    cells: Vec<Vec<bool>>,
}

impl Grid {
    fn new(w: usize, h: usize) -> Self {
        Grid {
            cols: w,
            rows: h,
            cells: vec![vec![false; w]; h],
        }
    }
    fn random_fill(&mut self, p_start: f32, p_end: f32){
        let mut p = p_start;
        let r = (p_end / p_start).powf(1.0 / self.rows as f32);
        for y in 0..self.rows - 1 {
            for x in 0..self.cols {
                let r_p: f32 = rand::gen_range(0.0, 1.0);
                if r_p < p {
                    self.set(x, y, true);
                }
            }
            p = p * r;
        }
    }
    fn get(&self, x: usize, y: usize) -> bool {
        if x >= self.cols || y >= self.rows {
            return false;
        }
        self.cells[y][x]
    }
    fn set(&mut self, x: usize, y: usize, val: bool) {
        if x < self.cols && y < self.rows {
            self.cells[y][x] = val;
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
    fn draw(&mut self, img: &mut Image){
        for y in 0..self.rows {
            for x in 0..self.cols {
                let color;
                if self.get(x, y)==true{
                    color = GOLD;
                }
                else {
                    color = BLACK;
                }
                img.set_pixel(x as u32, y as u32, color);
            }
        }
    }
}

#[macroquad::main("Rusty Sandfall")]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as u64);
    let mut grid = Grid::new(384, 384);
    grid.random_fill(0.9, 0.01);
    let mut image = Image::gen_image_color(grid.cols as u16, grid.rows as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);
    grid.draw(&mut image);
    texture.update(&image);

    let mut last_update_time = 0.0;
    let update_interval = 0.025;
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
            grid.draw(&mut image);
            texture.update(&image);
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
