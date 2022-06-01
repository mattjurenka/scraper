#![no_main]
use libfuzzer_sys::fuzz_target;

use scraper::{Html, Selector};

fuzz_target!(|data: &[u8]| {
    match std::str::from_utf8(data) {
        Ok(s) => {
            let html = Html::parse_fragment(r#"
                <ul>
                    <li>Foo</li>
                    <li>Bar</li>
                    <li>Baz</li>
                </ul>
            "#);
            match Selector::parse(s) {
                Ok(selector) => {
                    for element in html.select(&selector) {
                        element.value().name();
                    }
                },
                _ => {},
            }
        },
        _ => {},
    }
});
