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

#[test]
fn test_apply_field_modifier() {
    let mut l = LongWord {
        sign: false,
        bytes: [63, 17, 3, 5, 4],
    };
    l.apply_field_modifier_load(l.parse_subfield(0, 5));
    assert_eq!(
        LongWord {
            sign: false,
            bytes: [63, 17, 3, 5, 4],
        },
        l
    );
    let mut l = LongWord {
        sign: false,
        bytes: [63, 17, 3, 5, 4],
    };
    l.apply_field_modifier_load(l.parse_subfield(0, 3));
    assert_eq!(
        LongWord {
            sign: false,
            bytes: [0, 0, 63, 17, 3],
        },
        l
    );
}

#[test]
fn test_parse_subfield() {
    let input = LongWord::new(true, [1, 2, 3, 4, 5]);
    let expect = SubField {
        sign: true,
        bytes: vec![1, 2, 3, 4, 5],
    };
    assert_eq!(input.parse_subfield(0, 5), expect);
    let expect = SubField {
        sign: true,
        bytes: vec![1, 2, 3],
    };
    assert_eq!(input.parse_subfield(0, 3), expect);
    let expect = SubField {
        sign: true,
        bytes: vec![3, 4, 5],
    };
    assert_eq!(input.parse_subfield(3, 5), expect);
}
