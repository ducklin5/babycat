use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

fn resample_babycat_sinc(c: &mut Criterion) {
    let mut group = c.benchmark_group("resample_babycat_sinc");
    // Configure Criterion.rs to detect smaller differences and increase sample size to improve
    // precision and counteract the resulting noise.
    group
        .significance_level(0.1)
        .sample_size(15)
        .measurement_time(Duration::from_secs(90));

    let audio = babycat::Waveform::from_file(
        "./audio-for-tests/blippy-trance/track.wav",
        Default::default(),
    )
    .unwrap();

    group.bench_function("resample_babycat_sinc_11025", |b| {
        b.iter(|| audio.resample_by_mode(11025, babycat::RESAMPLE_MODE_BABYCAT_SINC))
    });

    group.bench_function("resample_babycat_sinc_22050", |b| {
        b.iter(|| audio.resample_by_mode(22050, babycat::RESAMPLE_MODE_BABYCAT_SINC))
    });

    group.bench_function("resample_babycat_sinc_44099", |b| {
        b.iter(|| audio.resample_by_mode(44099, babycat::RESAMPLE_MODE_BABYCAT_SINC))
    });

    group.bench_function("resample_babycat_sinc_48000", |b| {
        b.iter(|| audio.resample_by_mode(48000, babycat::RESAMPLE_MODE_BABYCAT_SINC))
    });

    group.bench_function("resample_babycat_sinc_96000", |b| {
        b.iter(|| audio.resample_by_mode(96000, babycat::RESAMPLE_MODE_BABYCAT_SINC))
    });
}

criterion_group!(benches, resample_babycat_sinc);
criterion_main!(benches);