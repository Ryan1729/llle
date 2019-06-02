use super::*;
use platform_types::pos;
use proptest::prelude::*;
use proptest::{prop_compose, proptest};

macro_rules! r {
    ($s:expr) => {
        Rope::from_str(&$s)
    };
}

prop_compose! {
    fn arb_rope()(s in any::<String>()) -> Rope {
        r!(s)
    }
}

prop_compose! {
    fn arb_absolute_char_offset(max_len: usize)(offset in 0..max_len) -> AbsoluteCharOffset {
        AbsoluteCharOffset(offset)
    }
}

prop_compose! {
    fn arb_rope_and_offset()
        (s in ".*")
        (offset in 0..=r!(&s).len_chars(), s in Just(s)) -> (Rope, AbsoluteCharOffset) {
        (r!(s), AbsoluteCharOffset(offset))
    }
}

// Lifetimes!
fn arb_rope_and_pos() -> impl Strategy<Value = (Rope, Position)> {
    ".*".prop_flat_map(|s: String| {
        (0..r!(s).len_lines(), Just(s)).prop_flat_map(|(line_index, s)| {
            let line_len = r!(s)
                .lines()
                .nth(line_index)
                //The index comes from `len_lines()` so it should always produce a `Some`!
                .unwrap()
                .len_chars();

            ((0..=line_len), Just(s)).prop_map(move |(offset, s)| {
                (
                    r!(s),
                    Position {
                        line: line_index,
                        offset: CharOffset(offset),
                    },
                )
            })
        })
    })
}

#[test]
fn pos_to_char_offset_works_on_middle_of_single_line() {
    let rope = r!("0A");

    assert_eq!(
        pos_to_char_offset(&rope, &pos! {l 0 o 1}),
        Some(AbsoluteCharOffset(1))
    )
}

#[test]
fn char_offset_to_pos_works_on_middle_of_single_line() {
    let rope = r!("0A");

    assert_eq!(
        char_offset_to_pos(&rope, &AbsoluteCharOffset(1)),
        Some(pos! {l 0 o 1})
    )
}

fn pos_to_to_char_offset_to_pos(rope: &Rope, p: Position) {
    if let Some(o) = pos_to_char_offset(&rope, &p) {
        assert_eq!(char_offset_to_pos(&rope, &dbg!(o)), Some(p))
    }
}

#[test]
fn offset_in_middle_of_single_line_works() {
    let rope = r!("0A");
    let p = pos! {l 0 o 1};
    pos_to_to_char_offset_to_pos(&rope, p);
}

#[test]
fn char_offset_to_pos_works_on_final_offset() {
    let rope = r!("A");

    assert_eq!(
        char_offset_to_pos(&rope, &AbsoluteCharOffset(1)),
        Some(pos! {l 0 o 1})
    )
}

#[test]
fn final_offset_works() {
    let rope = r!("A");
    let p = pos! {l 0 o 1};
    pos_to_to_char_offset_to_pos(&rope, p);
}

proptest! {
    #[test]
    fn char_offset_to_pos_to_char_offset((rope, offset) in arb_rope_and_offset()) {
        if let Some(p) = char_offset_to_pos(&rope, &offset) {
            assert_eq!(pos_to_char_offset(&rope, &p), Some(offset))
        }
    }

    #[test]
    fn pos_to_to_char_offset_to_pos_works((rope, pos) in arb_rope_and_pos()) {
        if let Some(o) = pos_to_char_offset(&rope, &pos) {
            assert_eq!(char_offset_to_pos(&rope, &o), Some(pos))
        }
    }
}
