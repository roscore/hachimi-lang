use hachimi_lang::{execute, parse, Limits};
use std::io::Write;

const HELP: &str = "hachimi 0.1.0 — 하찌미랭 / Hachimi Lang

Usage:
  hachimi run <file> [--input TEXT] [--max-steps N] [--max-cells N]
  hachimi eval <source> [--input TEXT] [--max-steps N] [--max-cells N]
  hachimi check <file>
  hachimi --help
  hachimi --version

Default limits: 1,000,000 steps; 30,000 cells.
Exit codes: 0 success, 2 usage, 3 file/I/O, 4 parse, 5 runtime.
";

type CliError = (u8, String);

fn usage(message: impl Into<String>) -> CliError {
    (
        2,
        format!("{}\nUse hachimi --help for usage.", message.into()),
    )
}

fn parse_options(args: &[String]) -> Result<(Vec<u8>, Limits), CliError> {
    let mut input = Vec::new();
    let mut limits = Limits::default();
    let mut seen = [false; 3];
    let mut index = 0;
    while index < args.len() {
        let (slot, option) = match args[index].as_str() {
            "--input" => (0, "--input"),
            "--max-steps" => (1, "--max-steps"),
            "--max-cells" => (2, "--max-cells"),
            other => return Err(usage(format!("unknown argument: {other}"))),
        };
        if seen[slot] {
            return Err(usage(format!("duplicate option: {option}")));
        }
        seen[slot] = true;
        index += 1;
        let value = args
            .get(index)
            .ok_or_else(|| usage(format!("missing value for {option}")))?;
        match slot {
            0 => input = value.as_bytes().to_vec(),
            1 => {
                limits.max_steps = value
                    .parse()
                    .map_err(|_| usage("--max-steps requires a nonnegative integer"))?
            }
            2 => {
                limits.max_cells = value
                    .parse()
                    .map_err(|_| usage("--max-cells requires a positive integer"))?;
                if limits.max_cells == 0 {
                    return Err(usage("--max-cells requires a positive integer"));
                }
            }
            _ => unreachable!(),
        }
        index += 1;
    }
    Ok((input, limits))
}

fn run_cli(args: &[String]) -> Result<(), CliError> {
    match args {
        [flag] if flag == "--help" || flag == "-h" => {
            print!("{HELP}");
            return Ok(());
        }
        [flag] if flag == "--version" || flag == "-V" => {
            println!("hachimi {}", env!("CARGO_PKG_VERSION"));
            return Ok(());
        }
        _ => {}
    }

    let (command, target, options) = match args {
        [command, target, options @ ..] => (command.as_str(), target, options),
        _ => return Err(usage("expected a command and source or file")),
    };
    match command {
        "check" => {
            if !options.is_empty() {
                return Err(usage("check accepts only a file path"));
            }
            let source = std::fs::read_to_string(target)
                .map_err(|error| (3, format!("{target}: {error}")))?;
            parse(&source).map_err(|error| (4, error.to_string()))?;
            Ok(())
        }
        "run" | "eval" => {
            let (input, limits) = parse_options(options)?;
            let source = if command == "run" {
                std::fs::read_to_string(target)
                    .map_err(|error| (3, format!("{target}: {error}")))?
            } else {
                target.clone()
            };
            let program = parse(&source).map_err(|error| (4, error.to_string()))?;
            let result =
                execute(&program, &input, limits).map_err(|error| (5, error.to_string()))?;
            std::io::stdout()
                .write_all(&result.output)
                .map_err(|error| (3, error.to_string()))
        }
        _ => Err(usage(format!("unknown command: {command}"))),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err((code, message)) = run_cli(&args) {
        eprintln!("hachimi: {message}");
        std::process::exit(i32::from(code));
    }
}
