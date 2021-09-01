use super::*;

#[test]
fn test_0nnn() {
    assert!(false);
}

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
    assert!(ch8.stack[0] == 0x200);
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
    assert!(false);
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
    assert!(false);
}

#[test]
fn test_8xye() {
    assert!(false);
}

#[test]
fn test_9xy0() {
    assert!(false);
}

#[test]
fn test_annn() {
    assert!(false);
}

#[test]
fn test_bnnn() {
    assert!(false);
}

#[test]
fn test_cxnn() {
    assert!(false);
}

#[test]
fn test_dxyn() {
    assert!(false);
}

#[test]
fn test_ex9e() {
    assert!(false);
}

#[test]
fn test_exa1() {
    assert!(false);
}

#[test]
fn test_fx07() {
    assert!(false);
}

#[test]
fn test_fx0a() {
    assert!(false);
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
    assert!(false);
}

#[test]
fn test_fx29() {
    assert!(false);
}

#[test]
fn test_fx33() {
    assert!(false);
}

#[test]
fn test_fx55() {
    assert!(false);
}

#[test]
fn test_fx65() {
    assert!(false);
}
