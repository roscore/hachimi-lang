use hachimi_lang::{execute, parse, Limits};

#[test]
fn hello_example_prints_ascii_greeting() {
    let source =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/hello.hcm"))
            .expect("hello example exists");
    let output = execute(&parse(&source).unwrap(), b"", Limits::default())
        .unwrap()
        .output;
    assert_eq!(output, b"Hi!\n");
}
