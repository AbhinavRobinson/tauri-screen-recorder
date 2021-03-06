extern crate repng;
extern crate scrap;

use scrap::{Capturer, Display};
use std::fs::File;
use std::io::ErrorKind::WouldBlock;
use std::thread;
use std::time::Duration;

use chrono::Utc;

pub fn capture() {
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;

    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());

    loop {
        // Wait until there's a frame.

        let buffer = match capturer.frame() {
            Ok(buffer) => buffer,
            Err(error) => {
                if error.kind() == WouldBlock {
                    // Keep spinning.
                    thread::sleep(one_frame);
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
        };

        println!("Captured! Saving...");

        // Flip the ARGB image into a BGRA image.

        let mut bitflipped = Vec::with_capacity(w * h * 4);
        let stride = buffer.len() / h;

        for y in 0..h {
            for x in 0..w {
                let i = stride * y + 4 * x;
                bitflipped.extend_from_slice(&[buffer[i + 2], buffer[i + 1], buffer[i], 255]);
            }
        }

        // Save the image.

        let dt = Utc::now();
        let current_time = dt.naive_local();

        repng::encode(
            File::create(format!("{}.png", current_time)).unwrap(),
            w as u32,
            h as u32,
            &bitflipped,
        )
        .unwrap();

        println!("Image saved to `{}.png`.", current_time);
        break;
    }
}
