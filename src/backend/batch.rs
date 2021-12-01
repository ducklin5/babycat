//! Functions that use multithreading to manipulate multiple audio files in parallel.
//!
//! This submodule is only available if the Cargo feature
//! `enable-multithreading` is enabled. Functions that read audio from
//! the filesystem also need the Cargo feature `enable-filesystem`
//! to be enabled. Both of these feature are disabled in Babycat's
//! WebAssembly frontend.
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::backend::Waveform;
use crate::backend::WaveformArgs;
use crate::backend::WaveformNamedResult;

/// The default number of threads to use for multithreaded operations.
/// By default, we will initialize as many threads as *logical*
/// CPU cores on your machine.
pub const DEFAULT_NUM_WORKERS: u32 = 0;

/// Configures multithreading in Babycat.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BatchArgs {
    /// The maximum number of threads to initialize when doing multithreaded work.
    ///
    /// Babycat uses Rayon for multithreading, which
    /// [by default](https://github.com/rayon-rs/rayon/blob/master/FAQ.md)
    /// will initialize as many threads as *logical* CPU cores on your machine.
    pub num_workers: usize,
}

impl Default for BatchArgs {
    fn default() -> Self {
        BatchArgs {
            num_workers: DEFAULT_NUM_WORKERS as usize,
        }
    }
}

/// Decodes a list of audio files in parallel.
///
/// # Arguments
/// - `filenames`: A filename of an encoded audio file on the local filesystem.
/// - `waveform_args`: Instructions on how to demux/decode each audio file.
/// - `batch_args`: Instructions on how to divide the work across multiple threads.
///
/// # Feature flags
/// This function is only available if both of the `enable-filesystem`
/// and `enable-multithreading` features are enabled. These features
/// are enabled by default in Babycat's Rust, Python, and C frontends.
/// These features are disabled in Babycat's WebAssembly frontend.
///
/// # Examples
/// **(Attempt to) decode three files:**
///
/// In this example, we process three filenames and demonstrate how to handle errors.
/// The first two files are successfully processed, and we catch a
/// [`Error::FileNotFound`][crate::Error::FileNotFound] error when processing the third file.
/// ```
/// use babycat::{Error, WaveformArgs, WaveformNamedResult};
/// use babycat::batch::{BatchArgs, waveforms_from_files};
///
/// let filenames = &[
///     "audio-for-tests/andreas-theme/track.mp3",
///     "audio-for-tests/blippy-trance/track.mp3",
///     "does-not-exist",
/// ];
/// let decode_args = Default::default();
/// let batch_args = Default::default();
/// let batch = waveforms_from_files(
///     filenames,
///     decode_args,
///     batch_args
/// );
///
/// fn display_result(wnr: &WaveformNamedResult) -> String {
///     match &wnr.result {
///         Ok(waveform) => format!("\nSuccess: {}:\n{:?}", wnr.name, waveform),
///         Err(err) => format!("\nFailure: {}:\n{}", wnr.name, err),
///     }
/// }
/// assert_eq!(
///     display_result(&batch[0]),
///      "
/// Success: audio-for-tests/andreas-theme/track.mp3:
/// Waveform { frame_rate_hz: 44100, num_channels: 2, num_frames: 9586944}",
/// );
/// assert_eq!(
///     display_result(&batch[1]),
///      "
/// Success: audio-for-tests/blippy-trance/track.mp3:
/// Waveform { frame_rate_hz: 44100, num_channels: 2, num_frames: 5293440}",
/// );
/// assert_eq!(
///     display_result(&batch[2]),
///      "
/// Failure: does-not-exist:
/// Cannot find the given filename does-not-exist.",
/// );
/// ```
#[allow(dead_code)] // Silence dead code warning because we do not use this function in the C frontend.
pub fn waveforms_from_files(
    filenames: &[&str],
    waveform_args: WaveformArgs,
    batch_args: BatchArgs,
) -> Vec<WaveformNamedResult> {
    let thread_pool: rayon::ThreadPool = rayon::ThreadPoolBuilder::new()
        .num_threads(batch_args.num_workers)
        .build()
        .unwrap();

    let waveforms: Vec<WaveformNamedResult> = thread_pool.install(|| {
        filenames
            .par_iter()
            .map(|filename| WaveformNamedResult {
                name: (*filename).to_string(),
                result: Waveform::from_file(filename, waveform_args),
            })
            .collect()
    });
    waveforms
}