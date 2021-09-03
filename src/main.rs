use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Point;

use sdl2::keyboard::Keycode;

use std::fs;

#[cfg(test)]
mod tests;

const SPRITE_DATA: [[u8; 5]; 16] = [
    [0xF0, 0x90, 0x90, 0x90, 0xF0],
    [0x20, 0x60, 0x20, 0x20, 0x70],
    [0xF0, 0x10, 0xF0, 0x80, 0xF0],
    [0xF0, 0x10, 0xF0, 0x10, 0xF0],
    [0x90, 0x90, 0xF0, 0x10, 0x10],
    [0xF0, 0x80, 0xF0, 0x10, 0xF0],
    [0xF0, 0x80, 0xF0, 0x90, 0xF0],
    [0xF0, 0x10, 0x20, 0x40, 0x40],
    [0xF0, 0x90, 0xF0, 0x90, 0xF0],
    [0xF0, 0x90, 0xF0, 0x10, 0xF0],
    [0xF0, 0x90, 0xF0, 0x90, 0x90],
    [0xE0, 0x90, 0xE0, 0x90, 0xE0],
    [0xF0, 0x80, 0x80, 0x80, 0xF0],
    [0xE0, 0x90, 0x90, 0x90, 0xE0],
    [0xF0, 0x80, 0xF0, 0x80, 0xF0],
    [0xF0, 0x80, 0xF0, 0x80, 0x80],
];

pub struct Chip8 {
    memory: [u8; 4096], // 4096 bytes of 8-bit. First 512 addresses are reserved
    stack: [u16; 16],   // 16 16-bit entries
    v: [u8; 16],        // 16 8-bit registers
    pc: u16,            // program counter
    i: u16,             // index register
    sp: u8,             // stack pointer
    fb: [u8; 64 * 32],  // frame buffer. Use one byte per pixel to make life easier
    key_pressed: [bool; 16],
}

impl Chip8 {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut memory = [0; 4096];
        for (i, sprite) in SPRITE_DATA.iter().enumerate() {
            memory[i * 5..i * 5 + 5].copy_from_slice(sprite);
        }
        Self {
            memory,
            stack: [0; 16],
            v: [0; 16],
            pc: 0x200, // first 512 addresses are reserved
            i: 0,
            sp: 0,
            fb: [0; 64 * 32],
            key_pressed: [false; 16],
        }
    }

    pub fn load_rom(&mut self) {
        let data: Vec<u8> = fs::read("test_opcode.ch8").unwrap();
        for (index, byte) in data.into_iter().enumerate() {
            self.memory[index + 0x200] = byte;
        }
    }

    pub fn run(&mut self) {
        loop {
            self.step();
        }
    }

    pub fn step(&mut self) {
        let instruction = ((self.memory[self.pc as usize] as u16) << 8)
            | self.memory[(self.pc + 1) as usize] as u16;
        self.decode(instruction);
    }

    fn decode(&mut self, instruction: u16) {
        let opcode = instruction >> 12; // opcode is first 4 bits

        // note that these values aren't always valid
        let vx = ((instruction & 0x0F00) >> 8) as usize;
        let vy = ((instruction & 0x00F0) >> 4) as usize;
        let n = (instruction & 0xF) as u8;
        let nn = (instruction & 0xFF) as u8;
        let nnn = (instruction & 0xFFF) as u16;
        println!("{:x}", instruction);
        match opcode {
            0 => match instruction {
                0x00e0 => {
                    // Clears the screen
                    for pixel in &mut self.fb {
                        *pixel = 0;
                    }
                    self.pc += 2;
                }

                0x0ee => {
                    // Returns from a subroutine
                    self.pc = self.stack[(self.sp - 1) as usize];
                    self.sp -= 1;
                }
                _ => {
                    println!("Unknown instruction {:x}", instruction);
                }
            },
            1 => {
                // Jumps to address NNN
                self.pc = nnn;
            }
            2 => {
                // Calls subroutine at nnn
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            }
            3 => {
                // Skips the next instruction if VX == NN
                if self.v[vx] == nn {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            4 => {
                // Skips the next instruction if VX != NN
                if self.v[vx] != nn {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            5 => {
                // Skips the next instruction if VX == VY
                if self.v[vx] == self.v[vy] {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            6 => {
                // 6XNN: Set VX to NN
                self.v[vx] = nn;
                self.pc += 2;
            }
            7 => {
                // 7XNN: Add NN to VX without changing carry flag
                self.v[vx] += nn;
                self.pc += 2;
            }
            8 => {
                match instruction & 0xF {
                    0 => {
                        //8XY0: Assign VX to VY
                        self.v[vx] = self.v[vy];
                        self.pc += 2;
                    }
                    1 => {
                        // 8XY1: Sets VX to VX | VY
                        self.v[vx] |= self.v[vy];
                        self.pc += 2;
                    }
                    2 => {
                        // 8XY1: Sets VX to VX & VY
                        self.v[vx] &= self.v[vy];
                        self.pc += 2;
                    }
                    3 => {
                        // 8XY1: Sets VX to VX ^ VY
                        self.v[vx] ^= self.v[vy];
                        self.pc += 2;
                    }
                    4 => {
                        // Adds VY to VX. VF is set to 1 when there's a carry, 0 otherwise
                        let res: u16 = (self.v[vx] as u16) + (self.v[vy] as u16);
                        self.v[0xF] = ((res & 0x100) >> 8) as u8;
                        self.v[vx] = res as u8;
                        self.pc += 2;
                    }
                    5 => {
                        // VY is subtracted from VX. VF is set to 0 when there's a borrow 1 otherwise
                        self.v[0xF] = if self.v[vy] > self.v[vx] { 0 } else { 1 };
                        self.v[vx] = self.v[vx].wrapping_sub(self.v[vy]);
                        self.pc += 2;
                    }
                    6 => {
                        // Stores the least significant bit of VX in VF. Then shifts VX to the
                        // right by 1
                        self.v[0xF] = self.v[vx] & 0b1;
                        self.v[vx] >>= 1;
                        self.pc += 2;
                    }
                    7 => {
                        // VX = VY-VX. VF is set to 0 when there's a borrow 1 otherwise
                        self.v[0xF] = if self.v[vx] > self.v[vy] { 0 } else { 1 };
                        self.v[vx] = self.v[vy].wrapping_sub(self.v[vx]);
                        self.pc += 2;
                    }
                    0xE => {
                        // Stores the most significant bit of VX in VF. Then shifts VX to the
                        // left by 1
                        self.v[0xF] = (self.v[vx] & 0x80) >> 7;
                        self.v[vx] <<= 1;
                        self.pc += 2;
                    }
                    _ => {
                        println!("Unknown instruction {:x}", instruction);
                    }
                }
            }
            9 => {
                // Skips the next instruction if VX does not equal VY
                if self.v[vx] != self.v[vy] {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            0xA => {
                // Sets I to the address NNN
                self.i = nnn;
                self.pc += 2;
            }
            0xB => {
                // Jumps to the address NNN plus V0
                self.pc = nnn + (self.v[0] as u16);
            }
            0xC => {
                println!("Unknown instruction {:x}", instruction);
            }
            0xD => {
                let mut flipped: bool = false;

                for y in 0..n {
                    let sprite = self.memory[(self.i + (y as u16)) as usize];
                    for x in 0..8 {
                        // address individual bits
                        let pixel_value = (sprite & (0x80 >> x)) >> (7 - x);
                        let old_value =
                            self.fb[((vx + (x as usize)) % 64 + ((y as usize) + vy) * 64) as usize];
                        if old_value == 1 && pixel_value == 1 {
                            flipped = true;
                        }
                        self.fb[(((self.v[vx] + x) as usize) % 64
                            + ((y + self.v[vy]) as usize) * 64)
                            as usize] ^= pixel_value;
                    }
                }
                self.v[0xF] = flipped.into();
                self.pc += 2;
            }
            0xE => match instruction & 0xFF {
                0x9E => {
                    // Skips the next instruction if the key stored in VX is pressed
                    if self.key_pressed[self.v[vx] as usize] {
                        self.pc += 2;
                    }
                    self.pc += 2;
                }
                0xA1 => {
                    // Skips the next instruction if the key stored in VX is not pressed
                    if !self.key_pressed[self.v[vx] as usize] {
                        self.pc += 2;
                    }
                    self.pc += 2;
                }
                _ => {
                    println!("Unknown instruction {:x}", instruction);
                }
            },
            0xF => {
                match instruction & 0xFF {
                    0x7 => {
                        println!("Unknown instruction {:x}", instruction);
                    }
                    0x0A => {
                        // Blocks until a key is pressed. Stores the key in VX
                        for k in 0..0xF {
                            if self.key_pressed[k] {
                                self.v[vx] = k as u8;
                                self.pc += 2;
                            }
                        }
                    }
                    0x15 => {
                        println!("Unknown instruction {:x}", instruction);
                    }
                    0x18 => {
                        println!("Unknown instruction {:x}", instruction);
                    }
                    0x1E => {
                        // Adds VX to I
                        self.i += self.v[vx] as u16;
                        self.pc += 2;
                    }
                    0x29 => {
                        self.i = (self.v[vx] as u16) * 5;
                        self.pc += 2;
                    }
                    0x33 => {
                        self.memory[self.i as usize] = (self.v[vx] / 100) as u8;
                        self.memory[self.i as usize + 1] = ((self.v[vx] / 10) % 10) as u8;
                        self.memory[self.i as usize + 2] = (self.v[vx] % 10) as u8;
                        self.pc += 2;
                    }
                    0x55 => {
                        // Stores V0 to VX (inclusive) in memory starting at I
                        for i in 0..=vx {
                            self.memory[(self.i + (i as u16)) as usize] = self.v[i as usize];
                        }
                        self.pc += 2;
                    }
                    0x65 => {
                        for i in 0..=vx {
                            self.v[i as usize] = self.memory[(self.i + (i as u16)) as usize];
                        }
                        self.pc += 2;
                    }
                    _ => {
                        println!("Unknown instruction {:x}", instruction);
                    }
                }
            }
            _ => {
                println!("Unknown instruction {:x}", instruction);
            }
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Test", 640, 320).build().unwrap(); // Window is 10x larger than frame buffer

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.clear();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.set_scale(10.0, 10.0).unwrap(); // window is 10x larger than fb, so scale pixels up by 10
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut ch8 = Chip8::new();
    ch8.load_rom();
    'l: loop {
        for event in event_pump.poll_iter() {
            macro_rules! register_key{(
                $(
                    ($a:pat, $b:expr)
                    ),* $(,)?)
                => {
                    match event {
                        $(
                        Event::KeyDown { keycode: Some($a), .. } => { ch8.key_pressed[$b] = true; },
                        Event::KeyUp   { keycode: Some($a), .. } => { ch8.key_pressed[$b] = false; },
                        )*
                        _ => {}
                    }
                }
            }

            register_key![
                (Keycode::Num0, 0),
                (Keycode::Num1, 1),
                (Keycode::Num2, 2),
                (Keycode::Num3, 3),
                (Keycode::Num4, 4),
                (Keycode::Num5, 5),
                (Keycode::Num6, 6),
                (Keycode::Num7, 7),
                (Keycode::Num8, 8),
                (Keycode::Num9, 9),
                (Keycode::A, 10),
                (Keycode::B, 11),
                (Keycode::C, 12),
                (Keycode::D, 13),
                (Keycode::E, 14),
                (Keycode::F, 15),
            ];

            match event {
                Event::Quit { .. } => {
                    break 'l;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'l;
                }
                //register_key![(Keycode::B,2)];
                Event::KeyDown {
                    keycode: Some(Keycode::N),
                    ..
                } => {
                    ch8.step();

                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    canvas.clear();
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                    for (index, pixel) in ch8.fb.iter().enumerate() {
                        let x = (index % 64) as i32;
                        let y = (index / 64) as i32;
                        if *pixel == 1 {
                            canvas.draw_point(Point::new(x, y)).unwrap();
                        }
                    }
                    canvas.present();
                }
                _ => {}
            }
        }
    }
}
