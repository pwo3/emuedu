const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

const FONT_START_ADDRESS: usize = 0x50;

struct Chip8 {
    registers: [u8; 16],
    register_i: u16,
    program_counter: u16,
    stack: [u16; 16],
    stack_pointer: u8,
    delay_timer: u8,
    sound_timer: u8,
    memory: [u8; 4096],
    screen: [[u8; 64]; 32],
    keyboard: [bool; 16],
}

impl Chip8 {
    fn new() -> Chip8 {
        let mut chip8: Chip8 = Chip8 {
            registers: [0; 16],
            register_i: 0,
            program_counter: 0x200,
            stack: [0; 16],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            memory: [0; 4096],
            screen: [[0; 64]; 32],
            keyboard: [false; 16],
        };

        chip8.memory[FONT_START_ADDRESS..FONT_START_ADDRESS + FONTSET.len()]
            .copy_from_slice(&FONTSET);

        chip8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn font_set_is_loaded_correctly() {
        let chip8: Chip8 = Chip8::new();

        assert_eq!(chip8.memory[FONT_START_ADDRESS], 0xF0);
        assert_eq!(chip8.memory[FONT_START_ADDRESS + FONTSET.len() - 1], 0x80);
    }

    #[test]
    fn program_counter_is_initialized_correctly() {
        let chip8: Chip8 = Chip8::new();

        assert_eq!(chip8.program_counter, 0x200);
    }
}
