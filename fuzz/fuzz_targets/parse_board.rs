#![no_main]
use libfuzzer_sys::fuzz_target;
use pabi::chess::position;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        if let Ok(position) = position::Position::try_from(s) {
            // TODO: Check printing the position back to FEN.
            let _ = position.is_legal();
        }
    }
});
