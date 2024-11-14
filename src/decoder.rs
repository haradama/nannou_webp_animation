use crate::frame::WebpFrame;
use crate::utils::create_image_from_raw;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::Duration;

use std::mem;
use std::slice;

use libc::c_int;

// Include the generated bindings
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
mod bindings {
    #![allow(improper_ctypes)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use bindings::*;

// Import constants from bindings
use bindings::{
    WebPFormatFeature_WEBP_FF_CANVAS_HEIGHT, WebPFormatFeature_WEBP_FF_CANVAS_WIDTH,
    WebPFormatFeature_WEBP_FF_FRAME_COUNT, WebPMuxAnimBlend_WEBP_MUX_BLEND,
    WebPMuxAnimBlend_WEBP_MUX_NO_BLEND, WebPMuxAnimDispose_WEBP_MUX_DISPOSE_BACKGROUND,
    WebPMuxAnimDispose_WEBP_MUX_DISPOSE_NONE,
};

extern "C" {
    pub fn WebPFree(ptr: *mut std::ffi::c_void);
}

pub struct WebpDecoder {
    frames: Vec<WebpFrame>,
}

impl WebpDecoder {
    pub fn decode<P: AsRef<Path>>(path: P) -> Result<Vec<WebpFrame>, String> {
        // Read the WebP file
        let mut file = File::open(&path).map_err(|e| e.to_string())?;
        let mut webp_data = Vec::new();
        file.read_to_end(&mut webp_data)
            .map_err(|e| e.to_string())?;

        unsafe {
            // Initialize WebPData
            let data = WebPData {
                bytes: webp_data.as_ptr(),
                size: webp_data.len(),
            };

            // Create Demuxer
            let demux = WebPDemux(&data);
            if demux.is_null() {
                return Err("Failed to create WebPDemuxer".to_string());
            }

            // Get the canvas width and height
            let canvas_width = WebPDemuxGetI(demux, WebPFormatFeature_WEBP_FF_CANVAS_WIDTH);
            let canvas_height = WebPDemuxGetI(demux, WebPFormatFeature_WEBP_FF_CANVAS_HEIGHT);

            // Get the number of frames
            let frame_count = WebPDemuxGetI(demux, WebPFormatFeature_WEBP_FF_FRAME_COUNT);
            println!("Number of frames: {}", frame_count);

            if frame_count == 0 {
                WebPDemuxDelete(demux);
                return Err("No frames found in the animation".to_string());
            }

            // Initialize canvas with transparent pixels
            let mut canvas = vec![0u8; (canvas_width * canvas_height * 4) as usize];

            let mut frames = Vec::new();
            let mut iter: WebPIterator = mem::MaybeUninit::zeroed().assume_init();

            if WebPDemuxGetFrame(demux, 1, &mut iter) != 0 {
                loop {
                    let frame_data = slice::from_raw_parts(iter.fragment.bytes, iter.fragment.size);

                    let mut frame_width = 0;
                    let mut frame_height = 0;

                    // Decode RGBA for the frame
                    let rgba_ptr = WebPDecodeRGBA(
                        frame_data.as_ptr(),
                        frame_data.len(),
                        &mut frame_width,
                        &mut frame_height,
                    );
                    if rgba_ptr.is_null() {
                        WebPDemuxReleaseIterator(&mut iter);
                        WebPDemuxDelete(demux);
                        return Err("Failed to decode frame".to_string());
                    }

                    // Create a slice from the raw pointer
                    let frame_slice =
                        slice::from_raw_parts(rgba_ptr, (frame_width * frame_height * 4) as usize);

                    // Free the original memory allocated by WebPDecodeRGBA
                    WebPFree(rgba_ptr as *mut std::ffi::c_void);

                    // Convert frame dimensions to u32
                    let frame_width_u32 = frame_width as u32;
                    let frame_height_u32 = frame_height as u32;

                    // Handle Disposal Method
                    if iter.dispose_method == WebPMuxAnimDispose_WEBP_MUX_DISPOSE_BACKGROUND as u32
                    {
                        // Clear the area occupied by the previous frame
                        clear_canvas_area(
                            &mut canvas,
                            canvas_width,
                            iter.x_offset as u32,
                            iter.y_offset as u32,
                            iter.width as u32,
                            iter.height as u32,
                        );
                    }

                    // Determine blend method
                    let blend = iter.blend_method == WebPMuxAnimBlend_WEBP_MUX_BLEND as u32;

                    // Blend the frame onto the canvas
                    blend_frame_onto_canvas(
                        &mut canvas,
                        canvas_width,
                        canvas_height,
                        frame_slice,
                        frame_width_u32,
                        frame_height_u32,
                        iter.x_offset as u32,
                        iter.y_offset as u32,
                        blend,
                    );

                    // Create DynamicImage from canvas data
                    let image = create_image_from_raw(canvas_width, canvas_height, canvas.clone())
                        .ok_or("Failed to create image from canvas data")?;

                    frames.push(WebpFrame {
                        image,
                        duration: Duration::from_millis(iter.duration as u64),
                    });

                    // Advance to the next frame
                    if WebPDemuxNextFrame(&mut iter) == 0 {
                        break;
                    }
                }
                // Release the iterator when done
                WebPDemuxReleaseIterator(&mut iter);
            } else {
                // Failed to get first frame
                WebPDemuxDelete(demux);
                return Err("Failed to get frames from WebP animation".to_string());
            }

            // Delete the demuxer
            WebPDemuxDelete(demux);

            Ok(frames)
        }
    }
}

// Define WebPDemux function if not available in bindings
unsafe fn WebPDemux(data: &WebPData) -> *mut WebPDemuxer {
    WebPDemuxInternal(
        data,
        0,
        std::ptr::null_mut(),
        WEBP_DEMUX_ABI_VERSION as c_int,
    )
}

// Helper function to clear a region of the canvas
fn clear_canvas_area(
    canvas: &mut [u8],
    canvas_width: u32,
    x_offset: u32,
    y_offset: u32,
    width: u32,
    height: u32,
) {
    for y in y_offset..(y_offset + height) {
        for x in x_offset..(x_offset + width) {
            let idx = ((y * canvas_width + x) * 4) as usize;
            if idx + 4 <= canvas.len() {
                canvas[idx..idx + 4].copy_from_slice(&[0, 0, 0, 0]);
            }
        }
    }
}

// Helper function to blend a frame onto the canvas
fn blend_frame_onto_canvas(
    canvas: &mut [u8],
    canvas_width: u32,
    canvas_height: u32,
    frame_data: &[u8],
    frame_width: u32,
    frame_height: u32,
    x_offset: u32,
    y_offset: u32,
    blend: bool,
) {
    for y in 0..frame_height {
        for x in 0..frame_width {
            let canvas_x = x + x_offset;
            let canvas_y = y + y_offset;

            if canvas_x >= canvas_width || canvas_y >= canvas_height {
                continue;
            }

            let canvas_idx = ((canvas_y * canvas_width + canvas_x) * 4) as usize;
            let frame_idx = ((y * frame_width + x) * 4) as usize;

            if canvas_idx + 4 > canvas.len() || frame_idx + 4 > frame_data.len() {
                continue;
            }

            if blend {
                // Perform alpha blending
                let src_pixel = &frame_data[frame_idx..frame_idx + 4];
                let dst_pixel = &mut canvas[canvas_idx..canvas_idx + 4];

                let src_alpha = src_pixel[3] as f32 / 255.0;
                let inv_src_alpha = 1.0 - src_alpha;

                for i in 0..3 {
                    dst_pixel[i] = (src_pixel[i] as f32 * src_alpha
                        + dst_pixel[i] as f32 * inv_src_alpha)
                        .round() as u8;
                }
                dst_pixel[3] = 255; // Set alpha to opaque
            } else {
                // Replace pixels without blending
                canvas[canvas_idx..canvas_idx + 4]
                    .copy_from_slice(&frame_data[frame_idx..frame_idx + 4]);
            }
        }
    }
}