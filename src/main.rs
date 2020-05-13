extern crate ncurses;
extern crate rand;

use ncurses::*;
use rand::prelude::*;
use std::convert::TryFrom;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const WIDTH: u32 = 80;
const HEIGHT: u32 = 24;

enum MoveDir {
    Left,
    Right,
    Up,
    Down,
}

fn main() {
    initscr();

    let (input_tx, input_rx) = mpsc::channel();
    thread::spawn(move || loop {
        if let Ok(u) = u8::try_from(getch()) {
            input_tx.send(char::from(u));
        }
    });

    let mut snake_move_dir = MoveDir::Right;
    let mut snake = (0_u32, 0_u32);
    let mut snake_parts: Vec<(u32, u32)> = Vec::new();
    let mut food = new_food();

    loop {
        for input in input_rx.try_iter() {
            snake_move_dir = match input {
                'w' => MoveDir::Up,
                's' => MoveDir::Down,
                'a' => MoveDir::Left,
                'd' => MoveDir::Right,
                'q' => std::process::exit(0),
                _ => snake_move_dir,
            };
        }

        // MOVE PARTS
        for i in (0 .. snake_parts.len()).into_iter().rev() {
            let next_part = i
                .checked_sub(1)
                .and_then(|next_i| snake_parts.get(next_i))
                .unwrap_or(&snake)
                .clone();
            *snake_parts.get_mut(i).unwrap() = next_part;
        }

        // MOVE SNAKE
        match snake_move_dir {
            MoveDir::Up => snake.1 = snake.1.checked_sub(1).unwrap_or(snake.1),
            MoveDir::Down => {
                snake.1 = snake.1.checked_add(1).unwrap_or(snake.1)
            }
            MoveDir::Left => {
                snake.0 = snake.0.checked_sub(1).unwrap_or(snake.0)
            }
            MoveDir::Right => {
                snake.0 = snake.0.checked_add(1).unwrap_or(snake.0)
            }
        }

        // COLLECT FOOD
        if snake == food {
            food = new_food();
            snake_parts.insert(0, snake.clone());
        }

        // DRAW
        for y in 0 .. HEIGHT {
            for x in 0 .. WIDTH {
                if (x, y) == snake {
                    mvaddstr(y as i32, x as i32, "X");
                } else if snake_parts.iter().any(|part| part == &(x, y)) {
                    mvaddstr(y as i32, x as i32, "x");
                } else if food == (x, y) {
                    mvaddstr(y as i32, x as i32, "F");
                } else {
                    mvaddstr(y as i32, x as i32, "-");
                }
            }
        }

        refresh();

        thread::sleep(Duration::from_millis(50));
    }

    endwin();
}

fn new_food() -> (u32, u32) {
    let mut rng = rand::thread_rng();

    (rng.gen_range(0, WIDTH + 1), rng.gen_range(0, HEIGHT + 1))
}
