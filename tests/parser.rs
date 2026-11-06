use hachimi_lang::{parse, Op, ParseErrorKind, Position};

#[test]
fn recognizes_every_token_and_ignores_other_text() {
    let program = parse("noise 하찌미! 아시가 하야쿠 나루 테이오 허니 맘보 띵띵똥똥 end")
        .expect("valid source");
    let ops: Vec<_> = program.instructions.iter().map(|i| i.op).collect();
    assert_eq!(
        ops,
        [
            Op::Increment,
            Op::Decrement,
            Op::Right,
            Op::Left,
            Op::Output,
            Op::Input,
            Op::LoopStart,
            Op::LoopEnd,
        ]
    );
    assert_eq!(program.instructions[6].jump, Some(7));
    assert_eq!(program.instructions[7].jump, Some(6));
}

#[test]
fn tracks_unicode_character_columns_and_lines() {
    let program = parse("🙂한글\n  하찌미\r\n메모 테이오").unwrap();
    assert_eq!(
        program.instructions[0].position,
        Position { line: 2, column: 3 }
    );
    assert_eq!(
        program.instructions[1].position,
        Position { line: 3, column: 4 }
    );
}

#[test]
fn reports_unmatched_loop_start_at_its_position() {
    let error = parse("메모\n 맘보 하찌미").unwrap_err();
    assert_eq!(error.kind, ParseErrorKind::UnmatchedLoopStart);
    assert_eq!(error.position, Position { line: 2, column: 2 });
}

#[test]
fn reports_unmatched_loop_end_at_its_position() {
    let error = parse("🙂 띵띵똥똥").unwrap_err();
    assert_eq!(error.kind, ParseErrorKind::UnmatchedLoopEnd);
    assert_eq!(error.position, Position { line: 1, column: 3 });
}

#[test]
fn pairs_nested_loops() {
    let program = parse("맘보 맘보 띵띵똥똥 띵띵똥똥").unwrap();
    assert_eq!(program.instructions[0].jump, Some(3));
    assert_eq!(program.instructions[1].jump, Some(2));
    assert_eq!(program.instructions[2].jump, Some(1));
    assert_eq!(program.instructions[3].jump, Some(0));
}
