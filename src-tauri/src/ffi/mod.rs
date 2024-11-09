use std::{
    any::Any,
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
    display::{CGDisplayBounds, CGDisplayCreateImage, CGMainDisplayID},
    display_stream::{CGDisplayStream, CGDisplayStreamFrameStatus},
};
use core_video::{
    buffer::TCVBuffer,
    image_buffer::TCVImageBuffer,
    pixel_buffer::{kCVPixelBufferLock_ReadOnly, CVPixelBuffer},
};
use dispatch2::{Queue, QueueAttribute};
use image::RgbImage;
use io_surface::{IOSurfaceGetHeight, IOSurfaceGetWidth};

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
            let update = update.unwrap();
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
            println!("sending")
            // pb.unlock_base_address(kCVPixelBufferLock_ReadOnly);
        },
    );

    if resp.is_err() {
        eprintln!("Error: {:?}", resp.err());
    } else {
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
        let img_array = Arc::new(Mutex::new(Vec::new()));

        loop {
            let received = pixel_recv.try_recv();

            if received.is_ok() {
                count += 1;

                let (data, bytes_per_row, h, w, _timestamp) = received.unwrap();

                let img_array = img_array.clone();

                let t = std::thread::spawn(move || {
                    println!("Received in");

                    let x = &mut img_array.lock().as_mut().unwrap().push(data);

                    println!("Received: {:?}", img_array.lock().as_ref().unwrap().len());
                });

                t.join().unwrap();
                // t.join().unwrap();
            }
        }

        // t1.join().unwrap();

        // queue.suspend();

        // stream.stop();

        // if b.try_recv().is_ok() {
        //     println!("Received");
        //     stream.start();
        // }

        // let source = stream.run_loop_source().unwrap();

        // // let value = stream.start();

        // // println!("Value: {:?}", value);

        // let current_loop = CFRunLoop::get_current();
        // let mode = CFString::from_static_string("kCFRunLoopDefaultMode").as_concrete_TypeRef();

        // let x = current_loop.contains_source(&source, mode);
        // println!("Contains Source: {:?}", x);

        // CFRunLoop::run_current();

        // current_loop.add_source(&source, mode);
        // println!("Value: {:?}", value);

        // CFRunLoop::run_current();

        // let current_loop = CFRunLoop::get_current();

        // ?
        // let mode = CFString::from_static_string("");
        // println!("Current Loop: {:?}", current_loop);

        // let loop_source = stream.run_loop_source().unwrap();

        // loop_source.;
        // println!("Loop Source: {:}", loop_source);

        // loop {
        //     std::thread::sleep(std::time::Duration::from_secs(1));
        // }
    }
}
