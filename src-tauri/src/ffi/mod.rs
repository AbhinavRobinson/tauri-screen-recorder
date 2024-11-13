use std::{
    collections::VecDeque,
    sync::{mpsc, Arc, Mutex},
    time::Instant,
};

use core_foundation::{
    base::{CFType, TCFType},
    boolean::CFBoolean,
    dictionary::CFDictionary,
    number::CFNumber,
    runloop::CFRunLoop,
    string::CFString,
};
use core_graphics2::{
    display::{CGDisplayBounds, CGMainDisplayID},
    display_stream::{CGDisplayStream, CGDisplayStreamFrameStatus},
};
use core_video::pixel_buffer::{kCVPixelBufferLock_ReadOnly, CVPixelBuffer};
use dispatch2::{Queue, QueueAttribute};
use io_surface::{IOSurfaceGetHeight, IOSurfaceGetWidth};

struct ImgDataStore {
    bytes_per_row: usize,
    height: usize,
    width: usize,
}

impl ImgDataStore {
    fn new(bytes_per_row: usize, height: usize, width: usize) -> Self {
        Self {
            bytes_per_row,
            height,
            width,
        }
    }
}

pub fn ffi_loop() {
    println!("Hello, world!");

    let display;

    let display_size;
    unsafe {
        display = CGMainDisplayID();
        println!("Display: {:?}", display);
        display_size = CGDisplayBounds(display).size;
    }

    let output_width = display_size.width as usize;
    let output_height = display_size.height as usize;

    println!("Width: {:?}", output_width);
    println!("Height: {:?}", output_height);
    let pixel_format = 1111970369;
    let fps = 1.0 / 60.0;
    let properties: CFDictionary<CFString, CFType> = CFDictionary::from_CFType_pairs(&[
        (
            CFString::from_static_string("kCGDisplayStreamShowCursor"),
            CFBoolean::true_value().as_CFType(),
        ),
        (
            CFString::from_static_string("kCGDisplayStreamMinimumFrameTime"),
            CFNumber::from(fps).as_CFType(),
        ),
        (
            CFString::from_static_string("kCGDisplayStreamPreserveAspectRatio"),
            CFBoolean::true_value().as_CFType(),
        ),
        (
            CFString::from_static_string("kCGDisplayStreamQueueDepth"),
            CFNumber::from(60).as_CFType(),
        ),
    ]);

    let queue = Queue::new("label", QueueAttribute::Serial);

    let (pixel_send, pixel_recv) = mpsc::channel();

    let pixel_send = Arc::new(pixel_send);

    let ins = Instant::now();

    let resp = CGDisplayStream::new_with_dispatch_queue(
        display,
        output_width * 2,
        output_height * 2,
        pixel_format,
        &properties,
        &queue,
        move |status, timestamp, iosurface, update| {
            // let mut data = [];
            let surface = iosurface.expect("Surface is None");
            if status != CGDisplayStreamFrameStatus::FrameComplete {
                eprintln!("Error: {:?}", status);
                return;
            }
            let h;
            let w;
            unsafe {
                h = IOSurfaceGetHeight(surface.as_concrete_TypeRef());
                w = IOSurfaceGetWidth(surface.as_concrete_TypeRef());
            }

            let pb = CVPixelBuffer::from_io_surface(&surface, None).unwrap();

            let bytes_per_row = pb.get_bytes_per_row();

            pb.lock_base_address(kCVPixelBufferLock_ReadOnly);

            let base_address;

            unsafe {
                base_address = pb.get_base_address();
            }
            let total_bytes = h * bytes_per_row;

            let mut data = Vec::with_capacity(total_bytes as usize);

            unsafe {
                data.set_len(total_bytes as usize);
                std::ptr::copy_nonoverlapping(
                    base_address as *const u8,
                    data.as_mut_ptr(),
                    total_bytes as usize,
                );
            }

            if ins.elapsed().as_secs() > 5 {
                return;
            }
            pb.unlock_base_address(kCVPixelBufferLock_ReadOnly);

            pixel_send
                .send((data, bytes_per_row, h, w, timestamp))
                .unwrap();

            // pb.unlock_base_address(kCVPixelBufferLock_ReadOnly);
        },
    );

    if resp.is_err() {
        eprintln!("Error: {:?}", resp.err());
        panic!("Failed to create stream");
    }
    let stream: CGDisplayStream = resp.expect("Stream is None");
    // let x = stream.run_loop_source();

    stream.start();

    // std::thread::sleep(std::time::Duration::from_secs(5));
    // stream.stop();

    let t1 = std::thread::spawn(|| {
        CFRunLoop::run_current();
    });

    // let duration = std::time::Duration::from_secs(5);

    let mut count = 0;
    let img_array = Arc::new(Mutex::new(VecDeque::new()));

    let img_data = Arc::new(Mutex::new(ImgDataStore::new(0, 0, 0)));

    let mut img_val_set = false;

    let img_array1 = img_array.clone();

    std::thread::spawn(move || {
        loop {
            let received = pixel_recv.try_recv();

            if received.is_ok() {
                count += 1;

                let (data, bytes_per_row, h, w, _timestamp) = received.unwrap();

                let mut send_data = Vec::new();

                for y in 0..h {
                    for x in 0..w {
                        let index = (y * bytes_per_row + x * 4) as usize;
                        let r = data[index];
                        let g = data[index + 1];
                        let b = data[index + 2];

                        let y = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;
                        let u = ((b as f32 - y as f32) * 0.493) as u8;
                        let v = ((r as f32 - y as f32) * 0.877) as u8;

                        send_data.push(y);
                        send_data.push(u);
                        send_data.push(v);
                    }
                }

                if !img_val_set {
                    let mut lock = img_data.lock();
                    let img_data_store = lock.as_mut().unwrap();

                    img_data_store.bytes_per_row = bytes_per_row;
                    img_data_store.height = h;
                    img_data_store.width = w;
                    img_val_set = true;
                }

                let img_array = img_array.clone();

                let t = std::thread::spawn(move || {
                    let mut lock = img_array.lock();

                    let img_array = lock.as_mut().unwrap();

                    img_array.push_back(send_data);
                });

                t.join().unwrap();
                // t.join().unwrap();
            }
        }
    });

    std::thread::spawn(move || {
        loop {
            // sleep
            std::thread::sleep(std::time::Duration::from_secs(1));
            let img_array = img_array1.clone();

            // save to video
        }
    });
    loop {}
}
