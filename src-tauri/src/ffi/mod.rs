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
    display_stream::CGDisplayStream,
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
    let fps = 1f64;
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
            let h;
            let w;
            unsafe {
                h = IOSurfaceGetHeight(surface.as_concrete_TypeRef());
                println!("Height: {:?}", h);
                w = IOSurfaceGetWidth(surface.as_concrete_TypeRef());
                println!("Width: {:?}", w);
            }

            println!("Update: {:?}\n", update.drop_count());

            let pb = CVPixelBuffer::from_io_surface(&surface, None).unwrap();

            let bytes_per_row = pb.get_bytes_per_row();
            println!("Bytes Per Row: {:?}", bytes_per_row);

            pb.lock_base_address(kCVPixelBufferLock_ReadOnly);

            let base_address;

            unsafe {
                base_address = pb.get_base_address();
                println!("Base Address: {:?}", base_address);
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

            pb.unlock_base_address(kCVPixelBufferLock_ReadOnly);

            let mut img = image::RgbaImage::new(w as u32, h as u32);

            for y in 0..h {
                for x in 0..w {
                    let index = (y * bytes_per_row + x * 4) as usize;
                    let r = data[index];
                    let g = data[index + 1];
                    let b = data[index + 2];
                    let a = data[index + 3];
                    img.put_pixel(x as u32, y as u32, image::Rgba([b, g, r, a]));
                }
            }

            img.save("test.png").unwrap();

            todo!("Handle update");
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
