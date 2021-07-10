//! This module verifies that all of our resamplers return resampled
//! waveforms of the same dimensions.
mod fixtures;

mod test_float_waveform_resample_method {
    use crate::fixtures::*;

    use babycat::DecodeArgs;
    use babycat::FloatWaveform;
    use babycat::Waveform;
    use babycat::RESAMPLE_MODE_BABYCAT_LANCZOS;
    use babycat::RESAMPLE_MODE_BABYCAT_SINC;
    use babycat::RESAMPLE_MODE_LIBSAMPLERATE;

    const RESAMPLE_MODES: &[u32] = &[
        RESAMPLE_MODE_BABYCAT_LANCZOS,
        RESAMPLE_MODE_BABYCAT_SINC,
        RESAMPLE_MODE_LIBSAMPLERATE,
    ];

    /// Decodes a waveform and resamples it using every known resampler.
    /// Then it checks that all resampled waveforms are of the same size.
    fn decode_and_assert(
        test_name: &str,
        filename: &str,
        decode_args: DecodeArgs,
        frame_rate_hz: u32,
        expected_num_channels: u32,
        expected_num_frames: u64,
        expected_frame_rate_hz: u32,
    ) {
        let waveform = FloatWaveform::from_file(filename, decode_args).unwrap();
        for &resample_mode in RESAMPLE_MODES {
            match waveform.resample_by_mode(frame_rate_hz, resample_mode) {
                Ok(resampled) => {
                    //
                    // Test the number of channels is correct.
                    let actual_num_channels = resampled.num_channels();
                    assert_eq!(
                        actual_num_channels, expected_num_channels,
                        "Test {} expected {} channels. Got {}.",
                        test_name, expected_num_channels, actual_num_channels
                    );
                    //
                    // Test the number of frames is correct.
                    let actual_num_frames = resampled.num_frames();
                    assert_eq!(
                        actual_num_frames, expected_num_frames,
                        "Test {} expected {} frames. Got {}.",
                        test_name, expected_num_frames, actual_num_frames
                    );
                    //
                    // Test that the output frame rate is correct.
                    let actual_frame_rate_hz = resampled.frame_rate_hz();
                    assert_eq!(
                        actual_frame_rate_hz, expected_frame_rate_hz,
                        "Test {} expected frame rate as {} hz. Got {} hz.",
                        test_name, expected_frame_rate_hz, actual_frame_rate_hz
                    );
                }
                Err(err) => panic!(
                    "Test {} failed with error. Error message: {}",
                    test_name, err
                ),
            }
        }
    }

    #[test]
    fn test_circus_of_freaks_44099() {
        decode_and_assert(
            "test_circus_of_freaks_44099",
            COF_FILENAME,
            Default::default(),
            44099,
            2,
            2492872,
            44099,
        );
    }

    #[test]
    fn test_circus_of_freaks_22050() {
        decode_and_assert(
            "test_circus_of_freaks_22050",
            COF_FILENAME,
            Default::default(),
            22050,
            2,
            COF_NUM_FRAMES / 2,
            22050,
        );
    }

    #[test]
    fn test_circus_of_freaks_11025() {
        decode_and_assert(
            "test_circus_of_freaks_11025",
            COF_FILENAME,
            Default::default(),
            11025,
            2,
            COF_NUM_FRAMES / 4,
            11025,
        );
    }
}
