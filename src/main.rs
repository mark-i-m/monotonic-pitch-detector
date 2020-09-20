//! A monotonic pitch finder.

const SAMPLE_RATE: usize = 44100;

//const FREQ: usize = 1661;
const FILE_DURATION: usize = 20; // seconds

const MIN_DETECTABLE_FREQ: usize = 40; // Hz

/// The number of minimum cycles in a buffer. We want more than one to make cycle detection more
/// relaiable.
const FUDGE_FACTOR: usize = 10;

/// Number of samples needed to relaiably detect the minimum detectable freq.
const CHUNK_SIZE: usize = FUDGE_FACTOR * SAMPLE_RATE / MIN_DETECTABLE_FREQ;

const NOTE_EPSILON: f64 = 1.0; // Hz

const FILENAME: &str = "sine.wav";

#[derive(Debug)]
enum Note {
    A,
    ASharp,
    B,
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    Unknown,
}

macro_rules! notes {
    ($test:expr, $($freq:literal => $note:ident),+ $(,)?) => {{
        if false { Note::Unknown } else

        $(
            if f64_eq_ish($test, $freq) {
                Note::$note
            } else
        )+

        {
            Note::Unknown
        }
    }}
}

fn generate_sound() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(FILENAME, spec).unwrap();
    const N_SAMPLES: usize = SAMPLE_RATE * FILE_DURATION;
    for i in 0..N_SAMPLES {
        let t = i as f32 / (SAMPLE_RATE as f32);

        const FREQS: &[f32] = &[
            130.81, 138.59, 146.83, 155.56, 164.81, 174.61, 185.00, 196.00, 207.65, 220.00, 233.08,
            246.94, 261.63, 277.18, 293.66, 2349.32, 2489.02, 2637.02, 2793.83, 2959.96, 3135.96,
            3322.44, 3520.00, 3729.31, 3951.07, 4186.01, 4434.92, 4698.63, 4978.03, 5274.04,
            5587.65, 5919.91, 6271.93, 6644.88,
        ];

        let step = FREQS.len() * i / N_SAMPLES;
        let f = FREQS[step];

        let sample = (t * f * 2.0 * std::f32::consts::PI).sin();
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }
}

fn compute_monotonic_freq(buffer: &[i16]) -> f64 {
    let mut prev_dp = 0;
    let mut is_increasing = false;

    let mut maxes = vec![];

    for i in 1..buffer.len() {
        // Take a suffix of the cloned_buf and align with the beginning of buffer (we're shifting
        // backwards technically).
        let shifted = &buffer[i..];
        let dot_prod: i64 = buffer
            .iter()
            .zip(shifted.iter())
            .map(|(a, b)| (*a as i64) * (*b as i64))
            .sum();

        // Did we find a local max?
        if is_increasing && dot_prod < prev_dp {
            maxes.push(i - 1);
        }

        is_increasing = dot_prod > prev_dp;
        prev_dp = dot_prod;
    }

    // Compute the average difference between elements of `maxes`.
    let sum: usize = maxes
        .iter()
        .zip(maxes.iter().skip(1))
        .map(|(a, b)| b - a)
        .skip(1)
        .sum();
    let avg_period = sum as f64 / ((maxes.len() - 2) as f64);

    (SAMPLE_RATE as f64) / avg_period
}

fn main() {
    generate_sound();

    let mut reader = hound::WavReader::open(FILENAME).unwrap();
    let buffer = reader
        .samples::<i16>()
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    for i in 0..(buffer.len() / CHUNK_SIZE) {
        let freq = compute_monotonic_freq(&buffer[(i * CHUNK_SIZE)..((i + 1) * CHUNK_SIZE)]);
        let note = hz_to_note(freq);
        println!("Estimated freq: {:0.0} Hz, {:?}", freq, note);
    }
}

fn hz_to_note(freq: f64) -> Note {
    fn f64_eq_ish(a: f64, b: f64) -> bool {
        (a - b).abs() < NOTE_EPSILON
    }

    notes! {
        freq,
    16.35 => C,
    17.32 => CSharp,
    18.35 => D,
    19.45 => DSharp,
    20.60 => E,
    21.83 => F,
    23.12 => FSharp,
    24.50 => G,
    25.96 => GSharp,
    27.50 => A,
    29.14 => ASharp,
    30.87 => B,
    32.70 => C,
    34.65 => CSharp,
    36.71 => D,
    38.89 => DSharp,
    41.20 => E,
    43.65 => F,
    46.25 => FSharp,
    49.00 => G,
    51.91 => GSharp,
    55.00 => A,
    58.27 => ASharp,
    61.74 => B,
    65.41 => C,
    69.30 => CSharp,
    73.42 => D,
    77.78 => DSharp,
    82.41 => E,
    87.31 => F,
    92.50 => FSharp,
    98.00 => G,
    103.83 => GSharp,
    110.00 => A,
    116.54 => ASharp,
    123.47 => B,
    130.81 => C,
    138.59 => CSharp,
    146.83 => D,
    155.56 => DSharp,
    164.81 => E,
    174.61 => F,
    185.00 => FSharp,
    196.00 => G,
    207.65 => GSharp,
    220.00 => A,
    233.08 => ASharp,
    246.94 => B,
    261.63 => C,
    277.18 => CSharp,
    293.66 => D,
    311.13 => DSharp,
    329.63 => E,
    349.23 => F,
    369.99 => FSharp,
    392.00 => G,
    415.30 => GSharp,
    440.00 => A,
    466.16 => ASharp,
    493.88 => B,
    523.25 => C,
    554.37 => CSharp,
    587.33 => D,
    622.25 => DSharp,
    659.25 => E,
    698.46 => F,
    739.99 => FSharp,
    783.99 => G,
    830.61 => GSharp,
    880.00 => A,
    932.33 => ASharp,
    987.77 => B,
    1046.50 => C,
    1108.73 => CSharp,
    1174.66 => D,
    1244.51 => DSharp,
    1318.51 => E,
    1396.91 => F,
    1479.98 => FSharp,
    1567.98 => G,
    1661.22 => GSharp,
    1760.00 => A,
    1864.66 => ASharp,
    1975.53 => B,
    2093.00 => C,
    2217.46 => CSharp,
    2349.32 => D,
    2489.02 => DSharp,
    2637.02 => E,
    2793.83 => F,
    2959.96 => FSharp,
    3135.96 => G,
    3322.44 => GSharp,
    3520.00 => A,
    3729.31 => ASharp,
    3951.07 => B,
    4186.01 => C,
    4434.92 => CSharp,
    4698.63 => D,
    4978.03 => DSharp,
    5274.04 => E,
    5587.65 => F,
    5919.91 => FSharp,
    6271.93 => G,
    6644.88 => GSharp,
    7040.00 => A,
    7458.62 => ASharp,
    7902.13 => B,
    }
}
