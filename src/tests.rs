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
