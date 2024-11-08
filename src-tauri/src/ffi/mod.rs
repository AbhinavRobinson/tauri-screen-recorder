use core_foundation::{
    base::{CFType, TCFType},
    dictionary::CFDictionary,
    string::CFString,
};
use core_graphics2::{
    display::{CGDisplayBounds, CGDisplayScreenSize, CGMainDisplayID},
    display_stream::CGDisplayStream,
    image::CGImagePixelFormatInfo,
};

fn test_core_graphics2() {
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
    // TODO:
    let properties: CFDictionary<CFString, CFType> = CFDictionary::from_CFType_pairs(&[]);

    let resp = CGDisplayStream::new(
        display,
        output_width,
        output_height,
        pixel_format,
        &properties,
        |status, image, _time, _o| {
            println!("Status: {:?}", status);
            println!("Image: {:?}", image);
            // println!("Time: {:?}", time);
            _o.unwrap();
        },
    );

    if resp.is_err() {
        eprintln!("Error: {:?}", resp.err());
    } else {
        let stream = resp.expect("Stream is None");
        let x = stream.run_loop_source().unwrap().show();
        println!("X: {:?}", x);
        let err = stream.start();

        println!("Error: {:?}", err);

        println!("Stream: {:?}", stream);

        let loop_source = stream.run_loop_source().unwrap();

        loop_source;
        // println!("Loop Source: {:}", loop_source);

        // loop {
        //     std::thread::sleep(std::time::Duration::from_secs(1));
        // }
    }
}
