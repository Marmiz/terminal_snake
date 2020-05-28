extern crate rand;
extern crate rustbox;

use rand::Rng;
use std::default::Default;

use rustbox::Key;
use rustbox::{Color, OutputMode, RustBox};
use std::time::Duration;

#[derive(Debug, PartialEq)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
    Stop,
}

struct Food {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, PartialEq)]
struct Coords {
    x: usize,
    y: usize,
}

struct Game {
    food: Food,
    score: u64,
    scene_w: usize,
    scene_h: usize,
    direction: Directions,
    rustbox: RustBox,
    snake: Vec<Coords>,
    game_over: bool,
}

impl Game {
    fn new(&mut self) {
        // basic Game Loop:
        // - process_input -
        // - update -
        // - render -
        loop {
            match self
                .rustbox
                // .poll_event(false)
                // 20 fps
                .peek_event(Duration::new(0, 1_000_000_000u32 / 20), false)
            {
                Ok(rustbox::Event::KeyEvent(key)) => match key {
                    Key::Char('q') => {
                        break;
                    }
                    Key::Char('w') => {
                        self.set_direction(Directions::Up);
                    }
                    Key::Char('a') => {
                        self.set_direction(Directions::Left);
                    }
                    Key::Char('s') => {
                        self.set_direction(Directions::Down);
                    }
                    Key::Char('d') => {
                        self.set_direction(Directions::Right);
                    }
                    Key::Char('m') => {
                        self.set_direction(Directions::Stop);
                    }
                    Key::Char('n') => {
                        if self.game_over == true {
                            self.start_new_game();
                        }
                    }
                    _ => {}
                },
                Err(e) => panic!("An error occurred: {}", e.to_string()),
                _ => {}
            }
            if self.game_over == false {
                self.update_scene();
                self.paint_scene();
            } else {
                self.draw_end_screen();
            }
        }
    }

    fn update_scene(&mut self) {
        // update snake coordinates
        match self.direction {
            Directions::Left => self.move_snake_left(),
            Directions::Right => self.move_snake_right(),
            Directions::Up => self.move_snake_up(),
            Directions::Down => self.move_snake_down(),
            _ => {}
        }

        // check collisions:
        let head = self.snake[0];

        // check if eating itself:
        // tail is is all but Snake[0] (the head)
        let mut tail = self.snake.clone();
        tail.remove(0);
        // if head has same Coords that one of the tail means we are colliding
        if let Some(_x) = tail.iter().find(|&&x| x == head) {
            self.set_direction(Directions::Stop);
            self.game_over();
            // return;
        }

        // check food collision
        if head.x == self.food.x && head.y == self.food.y {
            self.score += 1;
            // pass the "old" food coordinates, those will be the new node
            let old_food = Coords {
                x: self.food.x,
                y: self.food.y,
            };
            self.add_snake_node(old_food);
            self.food = generate_random_food((self.scene_w, self.scene_h));
        }
    }

    fn paint_scene(&self) {
        self.rustbox.clear();
        self.draw_snake();
        self.draw_food();
        self.draw_menu();
        // self.draw_debug();
        self.rustbox.present();
    }

    fn draw_snake(&self) {
        for cell in self.snake.iter() {
            self.rustbox.print(
                cell.x,
                cell.y,
                rustbox::RB_BOLD,
                Color::Byte(0xa2),
                Color::Byte(0xa2),
                "s",
            )
        }
    }

    fn draw_food(&self) {
        self.rustbox.print(
            self.food.x,
            self.food.y,
            rustbox::RB_NORMAL,
            Color::White,
            Color::Cyan,
            "f",
        )
    }

    fn draw_menu(&self) {
        self.rustbox.print(
            0,
            0,
            rustbox::RB_NORMAL,
            Color::White,
            Color::Black,
            &format!("Score: {} | Press 'q' to exit | 'wasd' to move", self.score),
        )
    }

    // Dev only
    #[allow(dead_code)]
    fn draw_debug(&self) {
        let head = self.snake[0];
        self.rustbox.print(
            1,
            self.scene_h - 1,
            rustbox::RB_NORMAL,
            Color::White,
            Color::Black,
            &print_debug_info(head.x, head.y, self.scene_w, self.scene_h, &self.direction),
        )
    }

    fn add_snake_node(&mut self, coords: Coords) {
        self.snake.push(coords);
    }

    // Moving the Snake
    // the idea is to inly update the first element of the Vector
    // the other will take place of the one ahead.
    fn move_snake_left(&mut self) {
        let mut head = self.snake[0];
        if head.x == 0 {
            head = Coords {
                x: self.scene_w - 1,
                y: head.y,
            }
        } else {
            head = Coords {
                x: head.x - 1,
                y: head.y,
            }
        }
        let mut temp = Vec::new();
        temp.push(head);
        // remove tail
        self.snake.pop();
        temp.append(&mut self.snake);
        self.snake = temp;
    }

    fn move_snake_right(&mut self) {
        let mut head = self.snake[0];
        if head.x == self.scene_w - 1 {
            head = Coords { x: 0, y: head.y }
        } else {
            head = Coords {
                x: head.x + 1,
                y: head.y,
            }
        }
        let mut temp = Vec::new();
        temp.push(head);
        // remove tail
        self.snake.pop();
        temp.append(&mut self.snake);
        self.snake = temp;
    }

    fn move_snake_up(&mut self) {
        let mut head = self.snake[0];
        // row [0] is used by score and other system messages
        if head.y == 1 {
            head = Coords {
                x: head.x,
                y: self.scene_h - 1,
            }
        } else {
            head = Coords {
                x: head.x,
                y: head.y - 1,
            }
        }

        let mut temp = Vec::new();
        temp.push(head);
        self.snake.pop();
        temp.append(&mut self.snake);
        self.snake = temp;
    }

    fn move_snake_down(&mut self) {
        let mut head = self.snake[0];
        if head.y == self.scene_h - 1 {
            head = Coords { x: head.x, y: 0 }
        } else {
            head = Coords {
                x: head.x,
                y: head.y + 1,
            }
        }

        let mut temp = Vec::new();
        temp.push(head);
        self.snake.pop();
        temp.append(&mut self.snake);
        self.snake = temp;
    }

    fn set_direction(&mut self, dir: Directions) {
        // update direction unless we are going the opposite (cannot perform a 180 turn in place)
        match dir {
            Directions::Up => {
                if self.direction != Directions::Down {
                    self.direction = Directions::Up
                }
            }
            Directions::Left => {
                if self.direction != Directions::Right {
                    self.direction = Directions::Left
                }
            }
            Directions::Right => {
                if self.direction != Directions::Left {
                    self.direction = Directions::Right
                }
            }
            Directions::Down => {
                if self.direction != Directions::Up {
                    self.direction = Directions::Down
                }
            }
            Directions::Stop => self.direction = Directions::Stop,
        }
    }

    fn draw_end_screen(&self) {
        let s = format!(
            "Game Over | Final Score: {} | 'n' to start a new game | 'q' to quit",
            self.score
        );

        self.rustbox.print(
            (self.scene_w / 2) - (s.len() / 2),
            self.scene_h / 2,
            rustbox::RB_NORMAL,
            Color::White,
            Color::Black,
            &s,
        );

        self.rustbox.present();
    }

    fn game_over(&mut self) {
        self.game_over = true;
    }

    fn start_new_game(&mut self) {
        self.game_over = false;
        self.score = 0;
        self.set_direction(Directions::Left);

        let initial_snake = Coords {
            x: (self.scene_w / 2) - 1,
            y: (self.scene_h / 2) - 1,
        };

        self.snake = vec![initial_snake];
    }
}

fn main() {
    let mut rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("Error occurred: {}", e),
    };

    rustbox.set_output_mode(OutputMode::EightBit);

    let w = rustbox.width();
    let h = rustbox.height();

    let food = generate_random_food((w, h));

    let initial_snake = Coords {
        x: (w / 2) - 1,
        y: (h / 2) - 1,
    };

    let mut game = Game {
        scene_w: w,
        scene_h: h,
        direction: Directions::Left,
        rustbox,
        food,
        score: 0,
        snake: vec![initial_snake],
        game_over: false,
    };

    game.new();
}

fn generate_random_food(coord: (usize, usize)) -> Food {
    let mut rng = rand::thread_rng();
    let (x, y) = coord;
    Food {
        x: rng.gen_range(0, x),
        // from 1 - row 0 is used for game related messaes
        y: rng.gen_range(1, y),
    }
}

// Use for Dev purposes
// create a 35 long snake
#[allow(dead_code)]
fn generate_debug_snake() -> Vec<Coords> {
    let mut debug_snake = Vec::new();
    for z in 1..35 {
        debug_snake.push(Coords { x: z, y: 5 })
    }
    debug_snake
}

#[allow(dead_code)]
fn print_debug_info(
    w: usize,
    h: usize,
    scene_w: usize,
    scene_h: usize,
    dir: &Directions,
) -> String {
    format!(
        "| Debug Info: | Width: {} | Heigth: {} | Scene: {}, {}, | Direction: {:?}",
        w, h, scene_w, scene_h, dir
    )
}
