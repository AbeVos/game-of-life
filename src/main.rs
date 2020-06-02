extern crate minifb;
extern crate rand;

use minifb::{Key, Scale, Window, WindowOptions};

const WIDTH: usize = 160;
const HEIGHT: usize = 120;

struct State {
    buffer_1: Vec<bool>,
    buffer_2: Vec<bool>,
    first_buffer: bool,
}

impl State {
    fn random() -> Self {
        let mut buffer_1: Vec<bool> = vec![false; WIDTH * HEIGHT];
        let buffer_2: Vec<bool> = vec![false; WIDTH * HEIGHT];
        let first_buffer = true;

        for c in buffer_1.iter_mut() {
            *c = rand::random::<bool>();
        }

        Self {
            buffer_1,
            buffer_2,
            first_buffer,
        }
    }

    fn step(&mut self) -> (&Vec<bool>, &mut Vec<bool>) {
        let (state, new_state) = if self.first_buffer {
            (&self.buffer_1, &mut self.buffer_2)
        } else {
            (&self.buffer_2, &mut self.buffer_1)
        };

        self.first_buffer = !self.first_buffer;

        return (state, new_state);
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0x999999; WIDTH * HEIGHT];

    let mut state = State::random();

    let mut window = Window::new(
        "Test", WIDTH, HEIGHT,
        WindowOptions {
            scale: Scale::X4,
            ..WindowOptions::default()
        }
    ).unwrap();

    window.limit_update_rate(
        Some(std::time::Duration::from_micros(2 * 16600))
    );

    while window.is_open() && !window.is_key_down(Key::Q) {
        // Switch the state buffers.
        let (state, new_state) = state.step();

        let mut idx = WIDTH + 1;

        for _ in 1..(HEIGHT - 1) {
            for _ in 1..(WIDTH - 1) {
                let c = state[idx];
                let neighbourhood = vec![
                    state[idx - WIDTH - 1] as u8,
                    state[idx - WIDTH] as u8,
                    state[idx - WIDTH + 1] as u8,
                    state[idx - 1] as u8,
                    state[idx + 1] as u8,
                    state[idx + WIDTH - 1] as u8,
                    state[idx + WIDTH] as u8,
                    state[idx + WIDTH + 1] as u8,
                ];
                let n = neighbourhood.iter().sum();

                if !c {
                    if n == 3 { new_state[idx] = true; }
                    else { new_state[idx] = false; }
                } else if n < 2 || n > 3 {
                    new_state[idx] = false;
                } else {
                    new_state[idx] = true;
                }

                if new_state[idx] {
                    buffer[idx] = match n {
                        0 => 0x0000ff,
                        1 => 0x00ff00,
                        2 => 0xff0000,
                        3 => 0xff00ff,
                        _ => 0xffffff,
                    };
                    buffer[idx] = 0xffffff;
                } else {
                    buffer[idx] = 0x000000;
                }

                idx += 1;
            }

            idx += 2;
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
