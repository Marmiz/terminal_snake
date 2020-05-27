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
}

struct Food {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone)]
struct Coords {
    x: usize,
    y: usize,
}

struct Game {
    food: Food,
    score: u64,
    scene_w: usize,
    scene_h: usize,
    head_x: usize,
    head_y: usize,
    direction: Directions,
    rustbox: RustBox,
    snake: Vec<Coords>,
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
                    _ => {}
                },
                Err(e) => panic!("An error occurred: {}", e.to_string()),
                _ => {}
            }
            self.update_scene();
            self.paint_scene();
        }
    }

    fn update_scene(&mut self) {
        // update snake coordinates
        match self.direction {
            Directions::Left => self.move_snake_left(),
            Directions::Right => self.move_snake_right(),
            Directions::Up => self.move_snake_up(),
            Directions::Down => self.move_snake_down(),
        }

        // check collisions:
        let head = self.snake[0];

        // food collision
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
        self.draw_debug();
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
                "S",
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
            "F",
        )
    }

    fn draw_menu(&self) {
        self.rustbox.print(
            0,
            0,
            rustbox::RB_NORMAL,
            Color::White,
            Color::Black,
            &format!("Score: {} | Press 'q' to exit", self.score),
        )
    }

    fn draw_debug(&self) {
        self.rustbox.print(
            1,
            self.scene_h - 1,
            rustbox::RB_NORMAL,
            Color::White,
            Color::Black,
            &print_debug_info(
                self.head_x,
                self.head_y,
                self.scene_w,
                self.scene_h,
                &self.direction,
            ),
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
        if head.y == 0 {
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
        }
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

    let mut debug_snake = Vec::new();
    for z in 1..5 {
        debug_snake.push(Coords { x: z, y: 5 })
    }

    let mut game = Game {
        scene_w: w,
        scene_h: h,
        head_x: ((w / 2) - 1),
        head_y: ((h / 2) - 1),
        direction: Directions::Left,
        rustbox: rustbox,
        food: food,
        score: 0,
        snake: debug_snake, // snake: vec![initial_snake],
    };

    game.new();
}

fn generate_random_food(coord: (usize, usize)) -> Food {
    let mut rng = rand::thread_rng();
    let (x, y) = coord;
    let food = Food {
        x: rng.gen_range(0, x),
        y: rng.gen_range(0, y),
    };
    food
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
