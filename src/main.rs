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
    // Initialize a random begin state.
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

    // Each step assign one buffer to be the target buffer for drawing
    // the new state to. The previous step's target buffer will then be
    // used to read the current state from.
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
    // Set up the window for drawing.
    let mut window = Window::new(
        "Conway's Game of Life",
        WIDTH, HEIGHT,
        WindowOptions {
            scale: Scale::X4,  // Scale the screen up to make things more visible.
            ..WindowOptions::default()
        }
    ).unwrap();

    // Limit framerate to 60 fps.
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // Initialize the state and drawing buffer.
    let mut state = State::random();
    let mut buffer: Vec<u32> = vec![0x999999; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Q) {
        // Switch the state buffers.
        let (state, new_state) = state.step();

        let mut idx = WIDTH + 1;

        for _ in 1..(HEIGHT - 1) {
            for _ in 1..(WIDTH - 1) {
                // Collect the states in a Moore neighbourhood.
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
                let n: u8 = neighbourhood.iter().sum();

                // Determine this cell's new state.
                if !c {
                    if n == 3 { new_state[idx] = true; }
                    else { new_state[idx] = false; }
                } else if n < 2 || n > 3 {
                    new_state[idx] = false;
                } else {
                    new_state[idx] = true;
                }

                // Determine this cell's color.
                if new_state[idx] {
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
