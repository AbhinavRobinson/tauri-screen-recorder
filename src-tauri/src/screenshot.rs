use screenshots::Screen;
use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn capture() {
    let start = SystemTime::now();
    for screen in Screen::all().unwrap() {
        println!("capturer {screen:?}");
        let image = screen.capture().unwrap();
        let buffer = image.buffer();
        fs::write(
            format!(
                "screenshot-{}-{}.png",
                screen.display_info.id,
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("time error")
                    .as_secs()
            ),
            buffer,
        )
        .unwrap();
    }
    println!("Done: {:?}", start.elapsed().ok());
}
