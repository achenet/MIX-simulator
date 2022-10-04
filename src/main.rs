#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused)]

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
enum OverflowToggle {
    On,
    Off,
}

#[derive(Debug, PartialEq)]
enum ComparisonIndicator {
    Less,
    Equal,
    Greater,
}

//a byte is implemented as a u8

#[derive(Clone, Copy, Debug, PartialEq)]
struct LongWord {
    sign: bool,
    bytes: [u8; 5],
}

impl LongWord {
    fn new(sign: bool, bytes: [u8; 5]) -> Self {
        LongWord {
            sign: sign,
            bytes: bytes,
        }
    }

    fn value(&self) -> u32 {
        let mut o: u32 = 0;
        for i in 0..5 {
            o += self.bytes[i] as u32 * 64_u32.pow(i as u32);
        }
        o
    }

    fn parse_subfield(&self, mut l: u8, r: u8) -> SubField {
        let sign = if l > 0 { true } else { self.sign };
        let mut out = SubField {
            sign: if l > 0 { true } else { self.sign },
            bytes: vec![],
        };
        l = if l == 0 { 0 } else { l - 1 };
        for i in (0..(r - l)) {
            out.bytes.push(self.bytes[(i + l) as usize])
        }

        out
    }

    fn apply_field_modifier_load(&mut self, s: SubField) {
        self.sign = s.sign;
        self.bytes = [0; 5];
        // TODO
        for i in (0..s.bytes.len()) {
            self.bytes[5 - s.bytes.len() + i] = s.bytes[i]
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct SubField {
    sign: bool,
    bytes: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ShortWord {
    sign: bool,
    bytes: [u8; 2],
}
impl ShortWord {}

#[derive(Debug, PartialEq)]
struct MIX {
    // Registers
    A: LongWord,
    X: LongWord,
    I: [ShortWord; 6], // Will be be i0..i5, not i1..i6
    J: ShortWord,

    Memory: [LongWord; 4000],

    OverflowToggle: OverflowToggle,

    ComparisonIndicator: ComparisonIndicator,
    // Input/Output devices
    IO: Vec<LongWord>,
}

impl MIX {
    fn new() -> Self {
        MIX {
            A: LongWord {
                sign: true,
                bytes: [0; 5],
            },
            X: LongWord {
                sign: true,
                bytes: [0; 5],
            },
            I: [ShortWord {
                sign: true,
                bytes: [0; 2],
            }; 6],
            J: ShortWord {
                sign: true,
                bytes: [0; 2],
            },
            Memory: [LongWord {
                sign: true,
                bytes: [0; 5],
            }; 4000],
            OverflowToggle: OverflowToggle::Off,
            ComparisonIndicator: ComparisonIndicator::Equal,
            IO: vec![],
        }
    }

    fn run_program(&mut self, program: String) {}

    // Every op code
    // TODO take field modifier into account
    fn load_A(&mut self, address: u16, index: u8, field_modifier: u8) {
        let i = index as usize;
        let m = address + self.I[i].bytes[0] as u16 + self.I[i].bytes[1] as u16;
        let mut word = self.Memory[m as usize];
        let (l, r) = calculate_field_modifier(field_modifier);
        word.apply_field_modifier_load(word.parse_subfield(l, r));
        self.A = word;
    }

    fn load_X(&mut self, address: u16, index: u8, field_modifier: u8) {
        let i = index as usize;
        let m = address + self.I[i].bytes[0] as u16 + self.I[i].bytes[1] as u16;
        let mut word = self.Memory[m as usize];
        let (l, r) = calculate_field_modifier(field_modifier);
        word.apply_field_modifier_load(word.parse_subfield(l, r));
        self.X = word;
    }

    // TODO handle field modifier for this case
    fn load_I(&mut self, register_number: usize, address: u16, index: u8, field_modifier: u8) {
        let i = index as usize;
        let m: usize = (address + self.I[i].bytes[0] as u16 + self.I[i].bytes[1] as u16).into();
        // Word at Memory[m] SHOULD
        // have first 3 bytes set to 0
        // if Memory[m][0],Memory[m][1],Memory[m][2]
        // are not 0, this is undefined
        if self.Memory[m].bytes[0] != 0
            || self.Memory[m].bytes[1] != 0
            || self.Memory[m].bytes[2] != 0
        {
            panic!("undefined!")
        }
        self.I[register_number].sign = self.Memory[m as usize].sign;
        self.I[register_number].bytes[1] = self.Memory[m as usize].bytes[4];
        self.I[register_number].bytes[0] = self.Memory[m as usize].bytes[3];
    }

    fn load_A_negative(&mut self, address: u16, index: u8, field_modifier: u8) {
        self.load_A(address, index, field_modifier);
        self.A.sign = !self.A.sign;
    }
    fn load_X_negative(&mut self, address: u16, index: u8, field_modifier: u8) {
        self.load_X(address, index, field_modifier);
        self.X.sign = !self.X.sign;
    }
    fn load_I_negative(
        &mut self,
        register_number: usize,
        address: u16,
        index: u8,
        field_modifier: u8,
    ) {
        self.load_I(register_number, address, index, field_modifier);
        self.I[register_number].sign = !self.I[register_number].sign;
    }

    // Replace the field of CONTENTS(M) specified by field_modifier
    // with a portion of the contents of register A
    fn store_A(&mut self, address: u16, index: u8, field_modifier: u8) {}

    fn store_X(&mut self, address: u16, index: u8, field_modifier: u8) {}

    fn store_I(&mut self, register_number: usize, address: u16, index: u8, field_modifier: u8) {}

    fn store_J(&mut self, address: u16, index: u8, field_modifier: u8) {}

    fn store_Z(&mut self, address: u16, index: u8, field_modifier: u8) {}

    // Add contents of Memory cell to rA
    // set overflow toggle to on if the sum is too large
    // to store in register A
    fn add(&mut self, address: u16, index: u8, field_modifier: u8) {}

    fn sub(&mut self, address: u16, index: u8, field_modifier: u8) {}

    fn mul(&mut self, address: u16, index: u8, field_modifier: u8) {}

    fn div(&mut self, address: u16, index: u8, field_modifier: u8) {}

    fn no_op(&mut self) {
        // Do nothing
    }

    fn exec(&mut self, op: Operation) {
        match op.code {
            00 => self.no_op(),
            01 => self.add(op.address, op.index, op.field_modifier),
            02 => self.sub(op.address, op.index, op.field_modifier),
            03 => self.mul(op.address, op.index, op.field_modifier),
            04 => self.div(op.address, op.index, op.field_modifier),
            _ => todo!(),
        }
    }
}

fn main() {
    println!("Hello, MIX!");
    let mut mix = MIX::new();
    println!("{:?}", mix.A);
}

struct Operation {
    address: u16,
    index: u8,
    field_modifier: u8,
    code: u8,
}

impl Operation {
    fn parse_op_code(l: LongWord) -> Self {
        // bytes 1 and 2 are the address, 3 is ,
        // 4 is the modification/field specification,
        // 5 is the op code
        Operation {
            address: (64 * l.bytes[0] + l.bytes[1]) as u16,
            index: l.bytes[3],
            field_modifier: l.bytes[4],
            code: l.bytes[5],
        }
    }
}

fn long_to_short_word(l: LongWord) -> ShortWord {
    ShortWord {
        sign: l.sign,
        bytes: [l.bytes[3], l.bytes[4]],
    }
}

fn calculate_field_modifier(field_modifier: u8) -> (u8, u8) {
    (field_modifier / 8, field_modifier % 8)
}
