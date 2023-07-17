use minifb::{Key, ScaleMode, Window, WindowOptions};

const BLACK: u32 = 0x00_00_00;
const WHITE: u32 = 0xff_ff_ff;

fn main() {
    let mut game = GameOfLife::new(250,240);
    game.start()
}

struct GameOfLife {
    output: Vec<u32>,
    state: Vec<u32>,
    window:Window,
    w: usize,
    h: usize
}

impl GameOfLife {

    pub fn new(w: usize, h: usize) -> Self {
        let mut window = Window::new(
            "Game of Life - ESC to exit",
            w,
            h,
            WindowOptions {
                resize: false,
                scale_mode: ScaleMode::UpperLeft,
                scale: minifb::Scale::X4,
                ..WindowOptions::default()
            }
        ).expect("Unable to create window");
    
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
   
        Self {w, h,
            output: vec![0; w * h],
            state: vec![0; w * h],
            window }
    }

    fn start(&mut self) {
        //self.pentomino();
        //self.inf_growth();
        self.glider_gun();
        self.run()
    }

    fn pentomino(&mut self) {
        self.set(120, 120, "  ## ");
        self.set(120, 121, " ##  ");
        self.set(120, 122, "  #  ");
    }

    fn glider_gun(&mut self) {
        self.set(60, 45, "........................#............");
        self.set(60, 46, "......................#.#............");
        self.set(60, 47, "............##......##............##.");
        self.set(60, 48, "...........#...#....##............##.");
        self.set(60, 49, "##........#.....#...##...............");
        self.set(60, 50, "##........#...#.##....#.#............");
        self.set(60, 51, "..........#.....#.......#............");
        self.set(60, 52, "...........#...#.....................");
        self.set(60, 53, "............##.......................");
    }

    fn inf_growth(&mut self) {
        self.set(20, 200, "########.#####...###......#######.#####");
    }

    fn set(&mut self, x: usize, y: usize, s: &str){
        let mut p = 0;
        for c in s.chars() {
            self.state[y*self.w + x + p] =  if c == '#' {1} else {0};
            p+=1;
        }
    }

    fn cell(&self, x:usize,  y:usize) -> u32 {
        self.output[y * self.w + x]
    }

    fn run(&mut self) {
        //game loop
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            // tick
            std::thread::sleep(std::time::Duration::from_millis(50));

            let screen = self.update();
            self.window.update_with_buffer(&screen, self.w, self.h).unwrap();
        
        }
    }

    fn update(&mut self) -> Vec<u32> {

        let mut screen: Vec<u32> = vec![0; self.w * self.h];
        // Store output state
        for i in 0..self.w*self.h {
            self.output[i] = self.state[i];
        }
        for x in 1..self.w-1 {
            for y in 1..self.h-1
            {
                // The secret of artificial life =================================================
                let neighbours =   self.cell(x - 1, y - 1) + self.cell(x - 0, y - 1) + self.cell(x + 1, y - 1) +
                                        self.cell(x - 1, y + 0) +                0               + self.cell(x + 1, y + 0) +
                                        self.cell(x - 1, y +    1) + self.cell(x + 0, y + 1) + self.cell(x + 1, y + 1);

                if self.cell(x, y) == 1 {
                    self.state[y*self.w + x] = if neighbours == 2 || neighbours == 3 {1} else {0};
                } else {
                    self.state[y*self.w + x] = if neighbours == 3 {1} else {0};
                }
                // ===============================================================================


                screen[y*self.w + x] = if self.cell(x, y) == 1 {WHITE} else {BLACK};
                //Draw(x, y, if cell(x, y) == 1 {WHITE} else {BLACK});
            }
        }
        screen
    }

}