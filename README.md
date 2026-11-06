# 하찌미랭 (Hachimi Lang)

한국어 토큰으로 쓰는 작은 오픈 소스 난해한 언어입니다. Rust 2021로 만든 바이트 테이프 인터프리터이며, Brainfuck 방식의 여덟 동작을 사용합니다. 게임 자료, 가사, 로고를 포함하지 않습니다.

Hachimi Lang is a small open-source esoteric language with Korean tokens. Its Rust 2021 interpreter runs a bounded byte-tape VM inspired by Brainfuck.

## 빠른 시작 / Quickstart

Rust 툴체인이 필요합니다. 저장소에서 실행하세요:

```sh
cargo run -- run examples/hello.hcm
# Hi!

cargo run -- eval "허니 테이오" --input "A"
# A

cargo run -- check examples/hello.hcm
cargo test --all-targets
```

`cargo build --release` 후 실행 파일은 `target/release/hachimi` (Windows: `hachimi.exe`)입니다. `hachimi --help`와 `hachimi --version`을 사용할 수 있습니다.

After `cargo build --release`, use the `hachimi` executable in `target/release` (with `.exe` on Windows).

## 언어 / Language

| 토큰 / Token | 동작 / Operation |
| --- | --- |
| `하찌미` | 현재 셀 +1 / increment current cell |
| `아시가` | 현재 셀 -1 / decrement current cell |
| `하야쿠` | 오른쪽으로 이동 / move right |
| `나루` | 왼쪽으로 이동 / move left |
| `테이오` | 현재 바이트 출력 / output current byte |
| `허니` | 입력 바이트 하나 읽기 / read one input byte |
| `맘보` | 셀이 0이면 대응하는 끝 다음으로 이동 / loop start |
| `띵띵똥똥` | 셀이 0이 아니면 대응하는 시작 다음으로 이동 / loop end |

셀은 `u8`이며 0과 255 사이에서 순환합니다. 테이프는 0으로 채워진 셀 하나로 시작하고 필요할 때 오른쪽으로 늘어납니다. 맨 왼쪽에서 `나루`를 실행하면 오류가 납니다. 입력이 끝나면 `허니`는 0을 저장합니다. `--input`의 텍스트는 UTF-8 바이트로 읽습니다. `테이오`는 **원시 바이트**를 출력하므로, 문자로 보이려면 출력 바이트들이 유효한 UTF-8을 이루어야 합니다.

Cells wrap as `u8`. The tape starts with one zero cell and grows right. Moving left from cell 0 is an error. Input uses UTF-8 bytes from `--input`; EOF stores zero. Output writes raw bytes, which can together encode UTF-8 text.

나머지 문자는 무시합니다. 토큰은 공백 없이 이어 쓸 수 있고 일반 문장 사이에서도 인식됩니다. 따라서 주석처럼 쓰는 문장 안에 토큰을 적으면 그 토큰도 실행됩니다. 구문 오류는 유니코드 문자 기준의 1부터 시작하는 `줄:열` 위치를 표시합니다.

All other characters are ignored. Tokens are recognized even inside surrounding text; avoid token spellings in comments. Parse errors use 1-based Unicode character line and column positions.

## 예제 / Examples

```sh
hachimi eval "하찌미 테이오"                 # byte 1
hachimi eval "허니 테이오" --input "Z"      # Z
hachimi run examples/hello.hcm              # Hi!
```

[`examples/hello.hcm`](examples/hello.hcm)은 `Hi!`와 줄바꿈을 출력합니다. / The bundled example prints `Hi!` and a newline.

## 안전 제한 / Safety limits

기본값은 실행 토큰 **1,000,000회**, 테이프 셀 **30,000개**입니다. 필요할 때 `--max-steps N` 및 `--max-cells N`으로 바꿀 수 있습니다. 첫 토큰 실행 전에 단계 한도를 확인하며, 셀 한도에는 시작 셀도 포함됩니다. `--max-cells`는 1 이상이어야 합니다. 루프가 끝나지 않아도 단계 한도에서 중지합니다.

Defaults are 1,000,000 executed instructions and 30,000 cells, including the initial cell. Override them with `--max-steps N` and `--max-cells N` on `run` or `eval`. A step limit of zero only permits an empty program; a cell limit must be positive.

```text
hachimi run <file> [--input TEXT] [--max-steps N] [--max-cells N]
hachimi eval <source> [--input TEXT] [--max-steps N] [--max-cells N]
hachimi check <file>
```

`check`는 UTF-8 파일을 읽고 토큰 및 루프 짝만 검사합니다. 실행은 하지 않습니다. 종료 코드: `0` 성공, `2` 인수 오류, `3` 파일 또는 출력 I/O 오류, `4` 구문 오류, `5` 실행 오류.

`check` parses a UTF-8 file without executing it. Exit codes: 0 success, 2 usage, 3 file/output I/O, 4 parse, 5 runtime.

## 라이브러리 / Library API

```rust
use hachimi_lang::{execute, parse, Limits};

let program = parse("허니 테이오")?;
let result = execute(&program, b"A", Limits::default())?;
assert_eq!(result.output, b"A");
# Ok::<(), Box<dyn std::error::Error>>(())
```

`parse`는 위치와 루프 점프가 담긴 `Program`을 반환합니다. `execute`는 출력, 최종 테이프, 포인터, 실행 단계 수를 반환합니다. 오류는 종류와 위치를 갖는 구조체입니다. 런타임 의존성은 없습니다.

`parse` returns a program with source positions and loop jumps. `execute` returns output, final tape, pointer, and step count. Errors carry structured kinds and positions. There are no runtime dependencies.

## 기여 / Contributing

기능 제안과 버그 보고를 환영합니다. 행동 규칙은 [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md), 개발 절차는 [CONTRIBUTING.md](CONTRIBUTING.md), 보안 제보는 [SECURITY.md](SECURITY.md)를 참고하세요. 새 동작은 실패하는 테스트를 먼저 추가하고 [TDD_LOG.md](TDD_LOG.md)에 RED/GREEN 명령을 기록해 주세요.

Contributions are welcome. See the code of conduct, contributing guide, and security policy. Write a failing test before each behavior change and record RED/GREEN commands in the TDD log.

## 상태와 고지 / Status and disclaimer

현재 **0.1.0 실험 버전**입니다. 언어와 API는 이후 버전에서 바뀔 수 있습니다. 이 프로젝트는 팬이 만든 비공식 창작물이며 어떤 게임, 제작사, 권리자와도 제휴하거나 승인을 받지 않았습니다. 저작권이 있는 게임 자산, 가사, 로고를 포함하지 않습니다.

Version **0.1.0** is experimental; language and API details may change. This is an unofficial fan project with no affiliation or endorsement from any game, publisher, or rights holder. It contains no copyrighted game assets, lyrics, or logos.

MIT 라이선스 / MIT licensed. See [LICENSE](LICENSE).
