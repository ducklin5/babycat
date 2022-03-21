#[cfg(feature = "enable-filesystem")]
use std::convert::AsRef;
#[cfg(feature = "enable-filesystem")]
use std::path::Path;

use std::io::Read;
use std::marker::Send;
use std::marker::Sync;

use crate::backend::constants::*;
use crate::backend::errors::Error;
use crate::backend::signal::Signal;
use crate::backend::Source;

/// Methods common to all audio decoders.
pub trait Decoder: Signal {
    fn begin(&mut self) -> Result<Box<dyn Source + '_>, Error>;
}

impl Decoder for Box<dyn Decoder> {
    #[inline(always)]
    fn begin(&mut self) -> Result<Box<dyn Source + '_>, Error> {
        (&mut **self).begin()
    }
}

impl Signal for Box<dyn Decoder> {
    #[inline(always)]
    fn frame_rate_hz(&self) -> u32 {
        (&**self).frame_rate_hz()
    }

    #[inline(always)]
    fn num_channels(&self) -> u16 {
        (&**self).num_channels()
    }

    #[inline(always)]
    fn num_frames_estimate(&self) -> Option<usize> {
        (&**self).num_frames_estimate()
    }
}

pub fn from_encoded_stream_with_hint<R: 'static + Read + Send + Sync>(
    decoding_backend: u32,
    encoded_stream: R,
    file_extension: &str,
    mime_type: &str,
) -> Result<Box<dyn Decoder>, Error> {
    match decoding_backend {
        DEFAULT_DECODING_BACKEND | DECODING_BACKEND_SYMPHONIA => {
            crate::backend::symphonia::SymphoniaDecoder::from_encoded_stream_with_hint(
                encoded_stream,
                file_extension,
                mime_type,
            )
        }
        _ => Err(Error::FeatureNotCompiled("decoding-backend-1")),
    }
}

pub fn from_encoded_stream<R: 'static + Read + Send + Sync>(
    decoding_backend: u32,
    encoded_stream: R,
) -> Result<Box<dyn Decoder>, Error> {
    from_encoded_stream_with_hint(
        decoding_backend,
        encoded_stream,
        DEFAULT_FILE_EXTENSION,
        DEFAULT_MIME_TYPE,
    )
}

pub fn from_encoded_bytes_with_hint(
    decoding_backend: u32,
    encoded_bytes: &[u8],
    file_extension: &str,
    mime_type: &str,
) -> Result<Box<dyn Decoder>, Error> {
    let owned = encoded_bytes.to_owned();
    let encoded_stream = std::io::Cursor::new(owned);
    from_encoded_stream_with_hint(decoding_backend, encoded_stream, file_extension, mime_type)
}

pub fn from_encoded_bytes(
    decoding_backend: u32,
    encoded_bytes: &[u8],
) -> Result<Box<dyn Decoder>, Error> {
    from_encoded_bytes_with_hint(
        decoding_backend,
        encoded_bytes,
        DEFAULT_FILE_EXTENSION,
        DEFAULT_MIME_TYPE,
    )
}

#[cfg(feature = "enable-filesystem")]
pub fn from_file<F: Clone + AsRef<Path>>(
    decoding_backend: u32,
    filename: F,
) -> Result<Box<dyn Decoder>, Error> {
    match decoding_backend {
        DEFAULT_DECODING_BACKEND => {
            #[cfg(feature = "enable-ffmpeg")]
            {
                crate::backend::ffmpeg::FFmpegDecoder::from_file(filename)
            }
            #[cfg(not(feature = "enable-ffmpeg"))]
            {
                crate::backend::symphonia::SymphoniaDecoder::from_file(filename)
            }
        }
        DECODING_BACKEND_SYMPHONIA => {
            crate::backend::symphonia::SymphoniaDecoder::from_file(filename)
        }
        DECODING_BACKEND_FFMPEG => {
            #[cfg(feature = "enable-ffmpeg")]
            {
                crate::backend::ffmpeg::FFmpegDecoder::from_file(filename)
            }
            #[cfg(not(feature = "enable-ffmpeg"))]
            {
                Err(Error::FeatureNotCompiled("decoding-backend-2"))
            }
        }
        _ => Err(Error::FeatureNotCompiled("decoding-backend-3")),
    }
}