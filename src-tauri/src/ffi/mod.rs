use std::sync::mpsc;

use core_foundation::{
    base::{CFType, TCFType},
    boolean::CFBoolean,
    dictionary::CFDictionary,
    number::CFNumber,
    runloop::{CFRunLoop, CFRunLoopMode},
    string::CFString,
};
use core_graphics2::{
    display::{CGDisplayBounds, CGDisplayScreenSize, CGMainDisplayID},
    display_stream::{CGDisplayStream, CGDisplayStreamGetRunLoopSource},
    image::CGImagePixelFormatInfo,
};
use dispatch2::{Queue, QueueAttribute};
use io_surface::{IOSurface, IOSurfaceGetHeight, IOSurfaceGetWidth};

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
    let fps: u32 = 1;
    let properties: CFDictionary<CFString, CFType> = CFDictionary::from_CFType_pairs(&[
        (
            CFString::from_static_string("kCGDisplayStreamShowCursor"),
            CFBoolean::true_value().as_CFType(),
        ),
        (
            CFString::from_static_string("kCGDisplayStreamMinimumFrameTime"),
            CFNumber::from(fps as i32).as_CFType(),
        ),
    ]);

    let queue = Queue::new("label", QueueAttribute::Serial);

    let resp = CGDisplayStream::new_with_dispatch_queue(
        display,
        output_width,
        output_height,
        pixel_format,
        &properties,
        &queue,
        |status, timestamp, iosurface, update| {
            println!("Status: {:?}", status);
            println!("Timestamp: {:?}", timestamp);
            // let mut data = [];
            let surface = iosurface.unwrap();
            let update = update.unwrap();

            unsafe {
                let h = IOSurfaceGetHeight(surface.as_concrete_TypeRef());
                println!("Height: {:?}", h);
                let w = IOSurfaceGetWidth(surface.as_concrete_TypeRef());
                println!("Width: {:?}", w);
            }

            println!("Update: {:?}\n", update.drop_count());

            // _o.unwrap();
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

        CFRunLoop::run_current();

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
