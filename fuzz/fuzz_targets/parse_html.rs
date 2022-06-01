#![no_main]
use libfuzzer_sys::fuzz_target;

use scraper::Html;

fuzz_target!(|data: &[u8]| {
    match std::str::from_utf8(data) {
        Ok(s) => {
            Html::parse_document(s);
        },
        _ => {},
    }
});
