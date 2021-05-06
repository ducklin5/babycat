mod fixtures;

mod test_float_waveform_from_encoded_stream {
    use crate::fixtures::*;
    use babycat::FloatWaveform;
    use babycat::Waveform;

    #[test]
    fn test_circus_of_freaks_default_1() {
        let bytes = std::fs::read(COF_FILENAME).unwrap();
        let waveform = FloatWaveform::from_encoded_bytes(&bytes, Default::default()).unwrap();
        assert_eq!(waveform.num_channels(), COF_NUM_CHANNELS);
        assert_eq!(waveform.num_frames(), COF_NUM_FRAMES);
        assert_eq!(waveform.frame_rate_hz(), COF_FRAME_RATE_HZ)
    }
}