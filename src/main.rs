extern crate ncurses;
extern crate rand;

use ncurses::*;
use rand::prelude::*;
use std::convert::TryFrom;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const WIDTH: i32 = 40;
const HEIGHT: i32 = 10;

enum MoveDir {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    initscr();

    let (input_tx, input_rx) = mpsc::channel();
    thread::spawn(move || loop {
        if let Ok(u) = u8::try_from(getch()) {
            input_tx.send(char::from(u)).unwrap();
        }
    });

    let mut running = true;
    let mut snake: (i32, i32) = (3, 0);
    let mut parts: Vec<(i32, i32)> = vec![(2, 0), (1, 0), (0, 0)];
    let mut food: (i32, i32) = new_food();
    let mut move_dir = MoveDir::Right;

    while running {
        // INPUT
        for input in input_rx.try_iter() {
            move_dir = match input {
                'w' => MoveDir::Up,
                's' => MoveDir::Down,
                'a' => MoveDir::Left,
                'd' => MoveDir::Right,
                'q' => {
                    running = false;
                    move_dir
                }
                _ => move_dir,
            };
        }

        // MOVE PARTS
        for i in (0 .. parts.len()).rev() {
            let next_pos = i
                .checked_sub(1)
                .and_then(|next_i| parts.get(next_i))
                .unwrap_or(&snake)
                .clone();
            *parts.get_mut(i).unwrap() = next_pos;
        }

        // MOVE SNAKE
        match move_dir {
            MoveDir::Up => snake.1 = (snake.1 - 1).rem_euclid(HEIGHT),
            MoveDir::Down => snake.1 = (snake.1 + 1).rem_euclid(HEIGHT),
            MoveDir::Left => snake.0 = (snake.0 - 1).rem_euclid(WIDTH),
            MoveDir::Right => snake.0 = (snake.0 + 1).rem_euclid(WIDTH),
        }

        // GAME OVER
        if parts.iter().any(|part| part == &snake) {
            running = false;
        }

        // COLLECT FOOD
        if snake == food {
            parts.insert(0, snake.clone());
            food = new_food();
        }

        // DRAW
        for y in 0 .. HEIGHT {
            for x in 0 .. WIDTH {
                let pos = (x, y);
                if snake == pos {
                    mvaddstr(y, x, "X");
                } else if parts.iter().any(|part| part == &pos) {
                    mvaddstr(y, x, "x");
                } else if food == pos {
                    mvaddstr(y, x, "#");
                } else {
                    mvaddstr(y, x, "-");
                }
            }
        }

        refresh();

        thread::sleep(Duration::from_millis(200));
    }

    endwin();

    println!("FINAL SCORE: {}", parts.len());
}

fn new_food() -> (i32, i32) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0, WIDTH), rng.gen_range(0, HEIGHT))
}
