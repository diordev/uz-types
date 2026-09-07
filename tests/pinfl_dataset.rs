use std::{env, fs};

use uz_types::Pinfl;

const EXPECTED_ROWS: usize = 1_000;

#[test]
#[ignore = "maxfiy audit: PINFL_SAMPLE_FILE=<bir-ustunli.csv> bilan ishga tushiring"]
fn sample_file_passes_strict_validation() {
    let path = env::var_os("PINFL_SAMPLE_FILE").unwrap_or_else(|| {
        panic!("PINFL_SAMPLE_FILE o'rnatilmagan; maxfiy bir ustunli CSV fayl yo'lini ko'rsating")
    });
    let contents = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("PINFL sample faylini o'qib bo'lmadi: {error}"));
    let contents = contents.strip_prefix('\u{feff}').unwrap_or(&contents);

    let mut checked_rows = 0;
    let mut first_non_empty_seen = false;

    for (index, line) in contents.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let value = strip_optional_quotes(line);

        if !first_non_empty_seen {
            first_non_empty_seen = true;
            if is_header(value) {
                continue;
            }
        }

        if let Err(error) = Pinfl::parse_strict(value) {
            panic!(
                "PINFL sample auditi {}-qatorda muvaffaqiyatsiz tugadi: {error:?}",
                index + 1
            );
        }
        checked_rows += 1;
    }

    assert_eq!(
        checked_rows, EXPECTED_ROWS,
        "PINFL sample auditi aynan {EXPECTED_ROWS} ta ma'lumot qatorini kutadi"
    );
}

fn strip_optional_quotes(value: &str) -> &str {
    value
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .unwrap_or(value)
}

fn is_header(value: &str) -> bool {
    !value.bytes().all(|byte| byte.is_ascii_digit())
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphabetic() || matches!(byte, b'_' | b'-' | b' '))
}
