use std::time::{Instant, SystemTime, UNIX_EPOCH};
use xcap::Monitor;

pub fn capture() {
    let start = Instant::now();
    let monitors = Monitor::all().unwrap();

    for monitor in monitors {
        let image = monitor.capture_image().unwrap();
        image
            .save(format!(
                "../screenshot-{}-{}.png",
                monitor.id(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("time error")
                    .as_secs()
            ))
            .unwrap();
    }
    println!("Done: {:?}", start.elapsed());
}
