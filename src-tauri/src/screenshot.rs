use std::{
    sync::Arc,
    time::{Instant, SystemTime, UNIX_EPOCH},
};
use xcap::Monitor;

pub fn capture() {
    let monitors = Monitor::all().unwrap();

    let mut threads = Vec::new();

    for monitor in monitors {
        let m_id = monitor.id();
        let start_base = Instant::now();

        let m_ref = Arc::new(monitor);

        for i in 1..20 {
            let m_ref = m_ref.clone();
            // do the rest of the work in a new thread
            let t = std::thread::spawn(move || {
                let capture_start_time = Instant::now();

                let image = m_ref.capture_image().unwrap();
                println!("Capture duration: {} {:?}", i, capture_start_time.elapsed());
                println!("Capture: {} {:?}", i, start_base.elapsed());

                let time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("time error")
                    .as_millis();

                let start = Instant::now();
                let var_name = format!("../screenshot-{}-{}.png", m_id, time);

                image.save(&var_name).unwrap();
                println!("Done saved {}: {:?}", var_name, start.elapsed());
            });
            threads.push(t);
        }
    }
    threads.into_iter().for_each(|t| t.join().unwrap());

    // println!("Times: {:?}", times);
}
