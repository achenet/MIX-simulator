use super::*;

#[test]
fn basic_add() {
    assert_eq!(1, 1);
}

#[test]
fn creation() {
    let got = MIX::new();
    let expect = MIX {
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
    };
    assert_eq!(got, expect);
}

#[test]
fn load_A() {
    let mut m = MIX::new();
    m.Memory[2000] = LongWord {
        sign: true,
        bytes: [63, 17, 3, 5, 4],
    };
    m.load_A(2000, 0, 5);
    let expect = LongWord {
        sign: true,
        bytes: [63, 17, 3, 5, 4],
    };
    assert_eq!(m.A, expect);
}

#[test]
fn calculate_field_modifier_test() {
    assert_eq!((0, 5), calculate_field_modifier(5));
    assert_eq!((1, 5), calculate_field_modifier(13));
    assert_eq!((2, 5), calculate_field_modifier(21));
}
