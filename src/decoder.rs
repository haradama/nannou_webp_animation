use crate::frame::WebpFrame;
use crate::utils::create_image_from_raw;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::Duration;

use std::slice;
use libc::c_int;

//---------------------------------------------------------------------
// Bindgen‑generated FFI layer
//---------------------------------------------------------------------

#[allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    improper_ctypes
)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
use bindings::*;

//---------------------------------------------------------------------
// Public decoder
//---------------------------------------------------------------------

/// High‑level helper that converts an animated WebP file to a vector of
/// [`WebpFrame`] (RGBA bitmaps + per‑frame duration).
///
/// The heavy lifting (parsing, disposal, blending, etc.) is delegated to
/// libwebp’s *animated* decoder, so the implementation here is thin and mostly
/// concerned with:
///
/// 1.  Reading the file into memory.
/// 2.  Calling the C API.
/// 3.  Copying the returned raw RGBA canvas into a `Vec<u8>`.
/// 4.  Wrapping the bytes in `nannou::image::DynamicImage`.
///
/// All memory management stays on the Rust side except for the lifetime of the
/// decoder object itself, which is freed immediately after use.
pub struct WebpDecoder;

impl WebpDecoder {
    /// Decode `path` and return the sequence of frames.
    ///
    /// # Errors
    /// * I/O failures (file not found, no read permission, …)
    /// * Corrupted / unsupported WebP payloads
    pub fn decode<P: AsRef<Path>>(path: P) -> Result<Vec<WebpFrame>, String> {
        //-----------------------------------------------------------------
        // 1. Read file into a Vec<u8>
        //-----------------------------------------------------------------
        let mut data = Vec::new();
        File::open(path)
            .and_then(|mut f| f.read_to_end(&mut data))
            .map_err(|e| e.to_string())?;

        // libwebp keeps *pointers* into this buffer, so it must stay alive for
        // the decoder’s lifetime.
        let webp_data = WebPData {
            bytes: data.as_ptr(),
            size: data.len(),
        };

        //-----------------------------------------------------------------
        // 2. Build decoder options
        //-----------------------------------------------------------------
        let mut dec_opts: WebPAnimDecoderOptions = unsafe { std::mem::zeroed() };
        let ok = unsafe {
            WebPAnimDecoderOptionsInitInternal(
                &mut dec_opts,
                WEBP_DEMUX_ABI_VERSION as c_int,
            )
        };
        if ok == 0 {
            return Err("WebPAnimDecoderOptionsInitInternal failed".into());
        }
        dec_opts.color_mode = WEBP_CSP_MODE_MODE_RGBA; // RGBA output
        dec_opts.use_threads = 1;

        //-----------------------------------------------------------------
        // 3. Create decoder
        //-----------------------------------------------------------------
        let dec = unsafe {
            WebPAnimDecoderNewInternal(
                &webp_data,
                &dec_opts,
                WEBP_DEMUX_ABI_VERSION as c_int,
            )
        };
        if dec.is_null() {
            return Err("WebPAnimDecoderNewInternal failed".into());
        }

        //-----------------------------------------------------------------
        // 4. Fetch global animation info (canvas size, frame count, …)
        //-----------------------------------------------------------------
        let mut info: WebPAnimInfo = unsafe { std::mem::zeroed() };
        let ok = unsafe { WebPAnimDecoderGetInfo(dec, &mut info) };
        if ok == 0 {
            unsafe { WebPAnimDecoderDelete(dec) };
            return Err("WebPAnimDecoderGetInfo failed".into());
        }
        let (w, h) = (info.canvas_width, info.canvas_height);

        //-----------------------------------------------------------------
        // 5. Decode all frames
        //-----------------------------------------------------------------
        let mut rgba_ptr: *mut u8 = std::ptr::null_mut();
        let mut timestamp_ms: c_int = 0;

        let mut raws: Vec<Vec<u8>> = Vec::with_capacity(info.frame_count as usize);
        let mut timestamps: Vec<i32> = Vec::with_capacity(info.frame_count as usize);

        while unsafe { WebPAnimDecoderHasMoreFrames(dec) } != 0 {
            let ok =
                unsafe { WebPAnimDecoderGetNext(dec, &mut rgba_ptr, &mut timestamp_ms) };
            if ok == 0 {
                unsafe { WebPAnimDecoderDelete(dec) };
                return Err("WebPAnimDecoderGetNext failed".into());
            }

            // Copy the RGBA canvas into Rust‑owned memory
            let slice = unsafe {
                slice::from_raw_parts(rgba_ptr, (w * h * 4) as usize)
            };
            raws.push(slice.to_vec());
            timestamps.push(timestamp_ms);
        }

        unsafe { WebPAnimDecoderDelete(dec) };

        if raws.is_empty() {
            return Err("No frames decoded".into());
        }

        //-----------------------------------------------------------------
        // 6. Convert raw bytes → DynamicImage → WebpFrame
        //-----------------------------------------------------------------
        let mut frames = Vec::with_capacity(raws.len());
        for i in 0..raws.len() {
            // libwebp gives us “display_timestamp”; frame duration is the
            // difference to the next frame.  Fallback for the final frame:
            // reuse previous duration or default to 100 ms.
            let dur_ms = if i + 1 < timestamps.len() {
                (timestamps[i + 1] - timestamps[i]) as u64
            } else if i > 0 {
                (timestamps[i] - timestamps[i - 1]) as u64
            } else {
                100
            };

            let img = create_image_from_raw(w, h, raws[i].clone())
                .ok_or("Failed to create image from RGBA buffer")?;

            frames.push(WebpFrame {
                image: img,
                duration: Duration::from_millis(dur_ms),
            });
        }

        Ok(frames)
    }
}
