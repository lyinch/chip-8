use super::*;

#[test]
fn test_0nnn() {}

#[test]
fn test_00e0() {
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0x00;
    ch8.memory[0x201] = 0xE0;
    ch8.fb[0] = 1;
    ch8.fb[64] = 1;
    ch8.step();
    for pixel in ch8.fb {
        assert!(pixel == 0);
    }
    assert!(ch8.pc == 0x202);
}

#[test]
fn test_00ee() {
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0x00;
    ch8.memory[0x201] = 0xEE;
    ch8.sp = 1;
    ch8.stack[0] = 0x400;
    ch8.step();

    assert!(ch8.pc == 0x400);
    assert!(ch8.sp == 0);
}

#[test]
fn test_1nnn() {
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0x1F;
    ch8.memory[0x201] = 0x0B;
    ch8.step();

    assert!(ch8.pc == 0xF0B);
}

#[test]
fn test_2nnn() {
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0x2F;
    ch8.memory[0x201] = 0x0B;
    ch8.step();

    assert!(ch8.pc == 0xF0B);
    assert!(ch8.sp == 1);
    assert!(ch8.stack[0] == 0x202);
}

#[test]
fn test_3xnn() {
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0x30;
    ch8.memory[0x201] = 0xAF;
    ch8.step();

    assert!(ch8.pc == 0x202);

    ch8.pc = 0x200;
    ch8.v[0] = 0xAF;
    ch8.step();

    assert!(ch8.pc == 0x204);
}

#[test]
fn test_4xnn() {
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0x40;
    ch8.memory[0x201] = 0xAF;
    ch8.step();

    assert!(ch8.pc == 0x204);

    ch8.pc = 0x200;
    ch8.v[0] = 0xAF;
    ch8.step();

    assert!(ch8.pc == 0x202);
}

#[test]
fn test_5xy0() {
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0x50;
    ch8.memory[0x201] = 0x10;
    ch8.step();

    assert!(ch8.pc == 0x204);

    ch8.pc = 0x200;
    ch8.v[0] = 0x1;
    ch8.step();

    assert!(ch8.pc == 0x202);
}

#[test]
fn test_6xnn() {
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0x60;
    ch8.memory[0x201] = 0xF2;
    ch8.step();

    assert!(ch8.pc == 0x202);
    assert!(ch8.v[0] == 0xF2);
}

#[test]
fn test_7xnn() {
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0x70;
    ch8.memory[0x201] = 0x42;
    ch8.v[0] = 0x1D;
    ch8.step();

    assert!(ch8.pc == 0x202);
    assert!(ch8.v[0] == 0x5F);
}

#[test]
fn test_7xnn_regression() {
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0x70;
    ch8.memory[0x201] = 0xFF;
    ch8.v[0] = 0x1;
    ch8.step();

    assert!(ch8.pc == 0x202);
    assert!(ch8.v[0] == 0x00);
}

#[test]
fn test_8xy0() {
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0x81;
    ch8.memory[0x201] = 0x20;
    ch8.v[2] = 10;
    ch8.step();

    assert!(ch8.v[1] == 10);
    assert!(ch8.pc == 0x202);
}

#[test]
fn test_8xy1() {
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0x81;
    ch8.memory[0x201] = 0x21;
    ch8.v[1] = 0b011;
    ch8.v[2] = 0b101;
    ch8.step();

    assert!(ch8.v[1] == 0b111);
    assert!(ch8.pc == 0x202);
}

#[test]
fn test_8xy2() {
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0x81;
    ch8.memory[0x201] = 0x22;
    ch8.v[1] = 0b011;
    ch8.v[2] = 0b101;
    ch8.step();

    assert!(ch8.v[1] == 0b001);
    assert!(ch8.pc == 0x202);
}

#[test]
fn test_8xy3() {
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0x81;
    ch8.memory[0x201] = 0x23;
    ch8.v[1] = 0b011;
    ch8.v[2] = 0b101;
    ch8.step();

    assert!(ch8.v[1] == 0b110);
    assert!(ch8.pc == 0x202);
}

#[test]
fn test_8xy4() {
    // test addition without and addition with carry
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0x81;
    ch8.memory[0x201] = 0x24;
    ch8.v[1] = 0x11;
    ch8.v[2] = 0x01;
    ch8.step();

    assert!(ch8.v[1] == 0x12);
    assert!(ch8.v[0xF] == 0);
    assert!(ch8.pc == 0x202);

    ch8.pc = 0x200;
    ch8.v[1] = 0xFF;
    ch8.v[2] = 0x01;
    ch8.step();

    assert!(ch8.v[1] == 0x0);
    assert!(ch8.v[0xF] == 1);
    assert!(ch8.pc == 0x202);
}

#[test]
fn test_8xy5() {
    // test subtraction without and with borrow
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0x80;
    ch8.memory[0x201] = 0x15;
    ch8.v[0] = 0x11;
    ch8.v[1] = 0x01;
    ch8.step();

    assert!(ch8.v[0] == 0x10);
    assert!(ch8.v[0xF] == 1);
    assert!(ch8.pc == 0x202);

    ch8.pc = 0x200;
    ch8.v[0] = 0x00;
    ch8.v[1] = 0x01;
    ch8.step();

    assert!(ch8.v[0] == 0xFF);
    assert!(ch8.v[0xF] == 0);
    assert!(ch8.pc == 0x202);
}

#[test]
fn test_8xy6() {
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0x81;
    ch8.memory[0x201] = 0x26;
    ch8.v[1] = 0b1011;
    ch8.step();

    assert!(ch8.v[1] == 0b101);
    assert!(ch8.v[0xF] == 1);
    assert!(ch8.pc == 0x202);
}

#[test]
fn test_8xy7() {
    // test subtraction without and with borrow
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0x80;
    ch8.memory[0x201] = 0x17;
    ch8.v[0] = 0x01;
    ch8.v[1] = 0x11;
    ch8.step();

    assert!(ch8.v[0] == 0x10);
    assert!(ch8.v[0xF] == 1);
    assert!(ch8.pc == 0x202);

    ch8.pc = 0x200;
    ch8.v[0] = 0x01;
    ch8.v[1] = 0x00;
    ch8.step();

    assert!(ch8.v[0] == 0xFF);
    assert!(ch8.v[0xF] == 0);
    assert!(ch8.pc == 0x202);
}

#[test]
fn test_8xye() {
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0x81;
    ch8.memory[0x201] = 0x2e;
    ch8.v[1] = 0b10010100;
    ch8.step();

    assert!(ch8.v[1] == 0b0101000);
    assert!(ch8.v[0xF] == 1);
    assert!(ch8.pc == 0x202);
}

#[test]
fn test_9xy0() {
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0x90;
    ch8.memory[0x201] = 0x10;
    ch8.step();

    assert!(ch8.pc == 0x202);

    ch8.pc = 0x200;
    ch8.v[0] = 0xAF;
    ch8.step();

    assert!(ch8.pc == 0x204);
}

#[test]
fn test_annn() {
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0xA1;
    ch8.memory[0x201] = 0x23;
    ch8.step();

    assert!(ch8.pc == 0x202);
    assert!(ch8.i == 0x123);
}

#[test]
fn test_bnnn() {
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0xB1;
    ch8.memory[0x201] = 0x23;
    ch8.v[0] = 0x2;
    ch8.step();

    assert!(ch8.pc == 0x125);
    assert!(ch8.i == 0);
}

#[test]
fn test_cxnn() {
    assert!(false);
}

#[test]
fn test_dxyn() {
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0xD0;
    ch8.memory[0x201] = 0x12;

    // drwaing in two lines on an empty frame buffer
    ch8.v[0] = 0x0;
    ch8.v[1] = 0x0;
    ch8.i = 0;
    ch8.memory[0] = 0b10000001;
    ch8.memory[1] = 0b11000001;
    ch8.step();

    assert!(ch8.pc == 0x202);
    assert!(ch8.i == 0);
    assert!(ch8.v[0xF] == 0);

    assert!(ch8.fb[0] == 1);
    assert!(ch8.fb[2] == 0);
    assert!(ch8.fb[7] == 1);
    assert!(ch8.fb[64] == 1);
    assert!(ch8.fb[65] == 1);
    assert!(ch8.fb[66] == 0);
    assert!(ch8.fb[71] == 1);
    println!("second test");
    // draw one line, overdraw pixels and check VF
    ch8.pc = 0x200;
    ch8.memory[0] = 0b11000001;
    ch8.step();

    assert!(ch8.fb[0] == 0);
    assert!(ch8.fb[1] == 1);
    assert!(ch8.fb[7] == 0);
    assert!(ch8.v[0xF] == 1);

    // wrapping around on the x direction
    ch8.pc = 0x200;
    ch8.fb[1] = 0;
    ch8.memory[0] = 0b11000000;
    ch8.v[0] = 63;
    ch8.v[1] = 0x0;

    ch8.step();

    assert!(ch8.fb[63] == 1);
    assert!(ch8.fb[0] == 1);
    assert!(ch8.v[0xF] == 0);
}

#[test]
fn test_ex9e() {
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0xe0;
    ch8.memory[0x201] = 0x9e;
    ch8.v[0] = 0x2;
    ch8.step();

    assert!(ch8.pc == 0x202);

    ch8.pc = 0x200;
    ch8.key_pressed[2] = true;

    ch8.step();
    assert!(ch8.pc == 0x204);
}

#[test]
fn test_exa1() {
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0xe0;
    ch8.memory[0x201] = 0xa1;
    ch8.v[0] = 0x2;
    ch8.step();

    assert!(ch8.pc == 0x204);

    ch8.pc = 0x200;
    ch8.key_pressed[2] = true;

    ch8.step();
    assert!(ch8.pc == 0x202);
}

#[test]
fn test_fx07() {
    assert!(false);
}

#[test]
fn test_fx0a() {
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0xf0;
    ch8.memory[0x201] = 0x0a;
    ch8.step();

    assert!(ch8.pc == 0x200);
    assert!(ch8.v[0] == 0x0);

    ch8.key_pressed[2] = true;
    ch8.step();

    assert!(ch8.pc == 0x202);
    assert!(ch8.v[0] == 0x2);
}

#[test]
fn test_fx15() {
    assert!(false);
}

#[test]
fn test_fx18() {
    assert!(false);
}

#[test]
fn test_fx1e() {
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0xF1;
    ch8.memory[0x201] = 0x1E;
    ch8.v[1] = 0xF0;
    ch8.i = 0x1;
    ch8.step();

    assert!(ch8.pc == 0x202);
    assert!(ch8.i == 0xF1);
}

#[test]
fn test_fx29() {
    // Tests correctly setting ch8.i as well as copying the sprite data
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0xF0;
    ch8.memory[0x201] = 0x29;
    ch8.v[0] = 4;

    ch8.step();

    assert!(ch8.memory[ch8.i as usize] == 0x90);
    assert!(ch8.memory[ch8.i as usize + 1] == 0x90);
    assert!(ch8.memory[ch8.i as usize + 2] == 0xF0);
    assert!(ch8.memory[ch8.i as usize + 3] == 0x10);
    assert!(ch8.memory[ch8.i as usize + 4] == 0x10);
    assert!(ch8.pc == 0x202);
}

#[test]
fn test_fx33() {
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0xF0;
    ch8.memory[0x201] = 0x33;
    ch8.v[0] = 234;
    ch8.i = 0x400;

    ch8.step();

    assert!(ch8.memory[0x400] == 2);
    assert!(ch8.memory[0x401] == 3);
    assert!(ch8.memory[0x402] == 4);
    assert!(ch8.pc == 0x202);
}

#[test]
fn test_fx55() {
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0xF7;
    ch8.memory[0x201] = 0x55;
    ch8.v[0] = 0xF0;
    ch8.v[3] = 0x11;
    ch8.v[7] = 0x05;
    ch8.i = 0xAB;
    ch8.step();

    assert!(ch8.pc == 0x202);
    assert!(ch8.memory[0xAB] == 0xF0);
    assert!(ch8.memory[0xAE] == 0x11);
    assert!(ch8.memory[0xB2] == 0x05);
}

#[test]
fn test_fx65() {
    let mut ch8 = Chip8::new();
    ch8.memory[0x200] = 0xF7;
    ch8.memory[0x201] = 0x65;
    ch8.memory[0xAB] = 0xF0;
    ch8.memory[0xAE] = 0x11;
    ch8.memory[0xB2] = 0x05;

    ch8.i = 0xAB;
    ch8.step();

    assert!(ch8.pc == 0x202);
    assert!(ch8.v[0] == 0xF0);
    assert!(ch8.v[3] == 0x11);
    assert!(ch8.v[6] == 0);
    assert!(ch8.v[7] == 0x05);
}
