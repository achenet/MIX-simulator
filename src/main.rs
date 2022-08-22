#![allow(dead_code)]
#![allow(non_snake_case)]

#[derive(Debug)]
enum OverflowToggle {
    On,
    Off,
}

#[derive(Debug)]
enum ComparisonIndicator {
    Less,
    Equal,
    Greater,
}

//a byte is implemented as a u8

#[derive(Clone, Copy, Debug)]
struct LongWord {
    sign: bool,
    value: [u8; 5],
}

impl LongWord {
    fn value(&self) -> u32 {
        let mut o: u32 = 0;
        for i in 0..5 {
            o += self.value[i] as u32 * 64_u32.pow(i as u32);
        }
        o
    }
}

#[derive(Clone, Copy, Debug)]
struct ShortWord {
    sign: bool,
    value: [u8; 2],
}
impl ShortWord {}

#[derive(Debug)]
struct MIX {
    // Registers
    A: LongWord,
    X: LongWord,
    I: [ShortWord; 6], // Will be be i0..i5, not i1..i6
    J: ShortWord,
    // Memory
    Memory: [LongWord; 4000],
    // Oveflow toggle
    OverflowToggle: OverflowToggle,
    // Comparison indicator
    ComparisonIndicator: ComparisonIndicator,
    // Input/Output devices
    IO: Vec<LongWord>,
}

impl MIX {
    // New
    fn new() -> Self {
        MIX {
            A: LongWord {
                sign: true,
                value: [0; 5],
            },
            X: LongWord {
                sign: true,
                value: [0; 5],
            },
            I: [ShortWord {
                sign: true,
                value: [0; 2],
            }; 6],
            J: ShortWord {
                sign: true,
                value: [0; 2],
            },
            Memory: [LongWord {
                sign: true,
                value: [0; 5],
            }; 4000],
            OverflowToggle: OverflowToggle::Off,
            ComparisonIndicator: ComparisonIndicator::Equal,
            IO: vec![],
        }
    }

    // Every op code
    fn load_A(&mut self, address: u16, index: u8, field_modifier: u8) {
        let i = index as usize;
        let m = address + self.I[i].value[0] as u16 + self.I[i].value[1] as u16;
        self.A = self.Memory[m as usize];
    }
}

struct Operation {
    Address: u16,
    Index: u8,
    FieldModifier: u8,
    OpCode: u8,
}

fn main() {
    println!("Hello, MIX!");
    //    let mut mix = MIX::new();
    //    println!("{:?}", mix);
}

fn parse_op_code() {
    //  bytes 1 and 2 are the address, 3 is , 4 is the modificatin/field specificatoin, 5 is the op
    //  code.
}
