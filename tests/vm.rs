use hachimi_lang::{execute, parse, Limits, RuntimeErrorKind};

fn run(source: &str, input: &[u8]) -> hachimi_lang::Execution {
    execute(&parse(source).unwrap(), input, Limits::default()).unwrap()
}

#[test]
fn increment_and_decrement_wrap_u8_cells() {
    assert_eq!(run("아시가 테이오", b"").output, [255]);
    assert_eq!(run("아시가 하찌미 테이오", b"").output, [0]);
    assert_eq!(
        run(&format!("{} 테이오", "하찌미 ".repeat(256)), b"").output,
        [0]
    );
}

#[test]
fn moves_right_and_left_and_grows_tape() {
    let result = run("하찌미 하야쿠 하찌미 하찌미 나루 테이오 하야쿠 테이오", b"");
    assert_eq!(result.output, [1, 2]);
    assert_eq!(result.tape, [1, 2]);
    assert_eq!(result.pointer, 1);
}

#[test]
fn output_emits_raw_bytes_that_can_form_utf8() {
    let source = format!(
        "{} 테이오 하야쿠 {} 테이오",
        "하찌미 ".repeat(0xc3),
        "하찌미 ".repeat(0xa9)
    );
    assert_eq!(run(&source, b"").output, "é".as_bytes());
}

#[test]
fn input_reads_bytes_and_uses_zero_at_eof() {
    let result = run("허니 테이오 허니 테이오 허니 테이오", &[65, 66]);
    assert_eq!(result.output, [65, 66, 0]);
}

#[test]
fn loops_repeat_until_cell_is_zero_and_skip_when_zero() {
    let result = run("하찌미 하찌미 맘보 테이오 아시가 띵띵똥똥", b"");
    assert_eq!(result.output, [2, 1]);
    assert_eq!(run("맘보 하찌미 띵띵똥똥 테이오", b"").output, [0]);
}

#[test]
fn pointer_underflow_has_source_position() {
    let error = execute(&parse("메모\n나루").unwrap(), b"", Limits::default()).unwrap_err();
    assert_eq!(error.kind, RuntimeErrorKind::PointerUnderflow);
    assert_eq!(error.position.unwrap().line, 2);
}

#[test]
fn step_limit_stops_infinite_loop() {
    let program = parse("하찌미 맘보 띵띵똥똥").unwrap();
    let error = execute(
        &program,
        b"",
        Limits {
            max_steps: 5,
            max_cells: 10,
        },
    )
    .unwrap_err();
    assert_eq!(error.kind, RuntimeErrorKind::StepLimitExceeded);
    assert!(error.position.is_some());
}

#[test]
fn cell_limit_stops_growth() {
    let program = parse("하야쿠 하야쿠").unwrap();
    let error = execute(
        &program,
        b"",
        Limits {
            max_steps: 10,
            max_cells: 2,
        },
    )
    .unwrap_err();
    assert_eq!(error.kind, RuntimeErrorKind::CellLimitExceeded);
    assert_eq!(error.position.unwrap().column, 5);
}

#[test]
fn zero_cell_limit_is_rejected() {
    let error = execute(
        &parse("").unwrap(),
        b"",
        Limits {
            max_steps: 0,
            max_cells: 0,
        },
    )
    .unwrap_err();
    assert_eq!(error.kind, RuntimeErrorKind::InvalidCellLimit);
    assert_eq!(error.position, None);
}
