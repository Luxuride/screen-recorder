use crate::screen_record::record;
use gstreamer::prelude::*;
use gstreamer::{Buffer, ClockTime, ElementFactory, EventType, Format, Fraction, glib, PadProbeReturn, State};
use gstreamer_app::{AppLeakyType, AppSrc};
use gstreamer_base::gst;
use scap::{capturer::Options, frame::Frame};
use std::error::Error;
use std::process::exit;
use std::sync::Arc;
use std::time::Instant;

pub mod portal;
mod screen_record;

fn main() -> Result<(), Box<dyn Error>> {
    // record()?;
    // return Ok(());
    gstreamer::init()?;
    // Check if the platform is supported
    let supported = scap::is_supported();
    if !supported {
        println!("❌ Platform not supported");
        return Ok(());
    } else {
        println!("✅ Platform supported");
    }

    // Check if we have permission to capture screen
    // If we don't, request it.
    if !scap::has_permission() {
        println!("❌ Permission not granted. Requesting permission...");
        if !scap::request_permission() {
            println!("❌ Permission denied");
            return Ok(());
        }
    }
    println!("✅ Permission granted");

    // Create Options
    let options = Options {
        fps: 60,
        show_cursor: true,
        show_highlight: true,
        excluded_targets: None,
        output_type: scap::frame::FrameType::BGR0,
        output_resolution: scap::capturer::Resolution::Captured,
        ..Default::default()
    };

    let video_info = &gst::Caps::builder("video/x-raw")
        .field("format", "BGRx")
        .field("width", 1920)
        .field("height", 1080)
        .field("framerate", Fraction::new(60, 1))
        .build();

    let source: AppSrc = AppSrc::builder()
        .caps(video_info)
        .leaky_type(AppLeakyType::Upstream)
        .format(Format::Time)
        .do_timestamp(true)
        .is_live(true)
        .build();

    let file_sink = ElementFactory::make("filesink")
        .property("location", "output.mkv")
        .build()?;

    let probe = gstreamer::PadProbeType::from_bits(1 << 6).unwrap();
    let pad = file_sink.static_pad("sink").unwrap();
    pad.add_probe(probe, |_, event| {
        if event.event().unwrap().type_() == EventType::Eos {
            exit(0);
        }
        PadProbeReturn::Pass
    });
    let elements = [
        source.upcast_ref(),
        &ElementFactory::make("queue").build()?,
        &ElementFactory::make("videorate").build()?,
        &ElementFactory::make("videoconvert").build()?,
        &ElementFactory::make("timeoverlay").build()?,
        &ElementFactory::make("vaapih264enc")
            //.property_from_str("speed-preset", "2")
            .property("bitrate", 2000_u32)
            .build()?,
        &ElementFactory::make("matroskamux").build()?,
        &file_sink,
        // &ElementFactory::make("autovideosink").build()?,
    ];
    let mut pipeline = gstreamer::Pipeline::builder().message_forward(true).build();
    pipeline.add_many(&elements)?;
    gstreamer::Element::link_many(&elements)?;
    // Create Recorder

    let mut capturer = scap::capturer::Capturer::new(options);
    capturer.start_capture();
    let (sender, receiver) = std::sync::mpsc::sync_channel::<()>(1);
    let mut time = Instant::now();
    println!("{:?}", pipeline.set_state(State::Playing).unwrap());
    let source = Arc::new(source);
    let source_clone = source.clone();
    ctrlc::set_handler(move || {
        source_clone.end_of_stream().unwrap();
        sender.send(()).unwrap();
    })
    .unwrap();
    while let Ok(frame) = capturer.get_next_frame() {
        if receiver.try_recv().is_ok() {
            break;
        }
        //println!("{:?}fps", 1000 / ((Instant::now() - time).as_millis() + 1));
        let frame = match frame {
            Frame::YUVFrame(_) => {
                panic!()
            }
            Frame::RGB(frame) => frame.data,
            Frame::RGBx(frame) => frame.data,
            Frame::XBGR(frame) => frame.data,
            Frame::BGRx(frame) => frame.data,
            Frame::BGR0(frame) => frame.data,
            Frame::BGRA(frame) => frame.data,
        };
        let buffer = Buffer::from_mut_slice(frame);
        if source.push_buffer(buffer).is_err() {
            break;
        }

        time = Instant::now();
    }
    capturer.stop_capture();
    loop {}
}
