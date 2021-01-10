use dasp::{signal, Sample, Signal};
use hound::WavWriter;

pub fn open_file(path: &str, sample_rate: u32) -> Vec<f64> {
    let reader = hound::WavReader::open(path).unwrap();

    let spec = reader.spec();

    // Check if the file has the same sample rate as the song
    // If it doesn't we resample the file
    // If it does, we just return the file
    if spec.sample_rate != sample_rate {
        // If we have to resample the file, first:
        // Check if we already have it processed
        // If we do, load that
        // If we don't, resample the file and save it

        let processed_filename = format!(
            "assets/processed/{}-{}",
            sample_rate,
            path.replace("/", "_")
        );

        // TODO Check at what time the file was processed to see if we need to update it

        if let Ok(processed_file) = hound::WavReader::open(&processed_filename) {
            let processed_spec = reader.spec();
            processed_file
                .into_samples::<i16>()
                // NOTE Eventually this will be removed when we implement stereo
                .step_by(processed_spec.channels.into())
                .filter_map(Result::ok)
                .map(i16::to_sample::<f64>)
                .collect::<Vec<f64>>()
        } else {
            let orig = reader
                .into_samples::<i16>()
                // NOTE Eventually this will be removed when we implement stereo
                .step_by(spec.channels.into())
                .filter_map(Result::ok)
                .map(i16::to_sample::<f64>);

            // Convert the signal's sample rate using `Sinc` interpolation.
            use dasp::{interpolate::sinc::Sinc, ring_buffer};
            let signal = signal::from_interleaved_samples_iter(orig);
            let ring_buffer = ring_buffer::Fixed::from([[0.0f64]; 100]);
            let sinc = Sinc::new(ring_buffer);
            let new_signal =
                signal.from_hz_to_hz(sinc, spec.sample_rate as f64, sample_rate as f64);

            let vec = new_signal
                .until_exhausted()
                .map(|frame| frame[0])
                .collect::<Vec<f64>>();

            save_file(vec.clone(), &processed_filename, sample_rate);

            vec
        }
    } else {
        reader
            .into_samples::<i16>()
            // NOTE Eventually this will be removed when we implement stereo
            .step_by(spec.channels.into())
            .filter_map(Result::ok)
            .map(i16::to_sample::<f64>)
            .collect::<Vec<f64>>()
    }
}

pub fn save_file(audio: Vec<f64>, path: &str, sample_rate: u32) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = WavWriter::create(path, spec).unwrap();

    for i in audio {
        let val = i as f32;
        let value: i16 = cpal::Sample::from::<f32>(&val);

        writer.write_sample(value).unwrap();
    }
}

pub enum Reverb {
    BlockInside,
    BottleHall,
    CementBlock2,
    CementBlocks1,
    ChateauDeLogne,
    ConicLongEchoHall,
    DeepSpace,
    DerlonSanctuary,
    DirectCabinet1,
    DirectCabinet2,
    DirectCabinet3,
    DirectCabinet4,
    FiveColumns,
    FiveColumnsLong,
    FrenchSalon,
    GoingHome,
    Greek7EchoHall,
    HighlyDampedLargeRoom,
    InTheSileRevised,
    InTheSilo,
    LargeBottleHall,
    LargeLongEchoHall,
    LargeWideEchoHall,
    MasonicLodge,
    Musikvereinsaal,
    NarrowBumpySpace,
    NiceDrumRoom,
    OnAStar,
    ParkingGarage,
    Rays,
    RightGlassTriangle,
    RubyRoom,
    ScalaMilanOperaHall,
    SmallDrumRoom,
    SmallPrehistoricCave,
    StNicolaesChurch,
    TrigRoom,
    VocalDuo,
}

impl From<Reverb> for &'static str {
    fn from(reverb: Reverb) -> &'static str {
        match reverb {
            Reverb::BlockInside => "assets/internal/reverbs/block_inside.wav",
            Reverb::BottleHall => "assets/internal/reverbs/bottle_hall.wav",
            Reverb::CementBlock2 => "assets/internal/reverbs/cement_block2.wav",
            Reverb::CementBlocks1 => "assets/internal/reverbs/cement_blocks1.wav",
            Reverb::ChateauDeLogne => "assets/internal/reverbs/chateau_de_logne.wav",
            Reverb::ConicLongEchoHall => "assets/internal/reverbs/conic_long_echo_hall.wav",
            Reverb::DeepSpace => "assets/internal/reverbs/deep_space.wav",
            Reverb::DerlonSanctuary => "assets/internal/reverbs/derlon_sanctuary.wav",
            Reverb::DirectCabinet1 => "assets/internal/reverbs/direct_cabinet1.wav",
            Reverb::DirectCabinet2 => "assets/internal/reverbs/direct_cabinet2.wav",
            Reverb::DirectCabinet3 => "assets/internal/reverbs/direct_cabinet3.wav",
            Reverb::DirectCabinet4 => "assets/internal/reverbs/direct_cabinet4.wav",
            Reverb::FiveColumns => "assets/internal/reverbs/five_columns.wav",
            Reverb::FiveColumnsLong => "assets/internal/reverbs/five_columns_long.wav",
            Reverb::FrenchSalon => "assets/internal/reverbs/french_salon.wav",
            Reverb::GoingHome => "assets/internal/reverbs/going_home.wav",
            Reverb::Greek7EchoHall => "assets/internal/reverbs/greek_7_echo_hall.wav",
            Reverb::HighlyDampedLargeRoom => "assets/internal/reverbs/highly_damped_large_room.wav",
            Reverb::InTheSileRevised => "assets/internal/reverbs/in_the_sile_revised.wav",
            Reverb::InTheSilo => "assets/internal/reverbs/in_the_silo.wav",
            Reverb::LargeBottleHall => "assets/internal/reverbs/large_bottle_hall.wav",
            Reverb::LargeLongEchoHall => "assets/internal/reverbs/large_long_echo_hall.wav",
            Reverb::LargeWideEchoHall => "assets/internal/reverbs/large_wide_echo_hall.wav",
            Reverb::MasonicLodge => "assets/internal/reverbs/masonic_lodge.wav",
            Reverb::Musikvereinsaal => "assets/internal/reverbs/musikvereinsaal.wav",
            Reverb::NarrowBumpySpace => "assets/internal/reverbs/narrow_bumpy_space.wav",
            Reverb::NiceDrumRoom => "assets/internal/reverbs/nice_drum_room.wav",
            Reverb::OnAStar => "assets/internal/reverbs/on_a_star.wav",
            Reverb::ParkingGarage => "assets/internal/reverbs/parking_garage.wav",
            Reverb::Rays => "assets/internal/reverbs/rays.wav",
            Reverb::RightGlassTriangle => "assets/internal/reverbs/right_glass_triangle.wav",
            Reverb::RubyRoom => "assets/internal/reverbs/ruby_room.wav",
            Reverb::ScalaMilanOperaHall => "assets/internal/reverbs/scala_milan_opera_hall.wav",
            Reverb::SmallDrumRoom => "assets/internal/reverbs/small_drum_room.wav",
            Reverb::SmallPrehistoricCave => "assets/internal/reverbs/small_prehistoric_cave.wav",
            Reverb::StNicolaesChurch => "assets/internal/reverbs/st_nicolaes_church.wav",
            Reverb::TrigRoom => "assets/internal/reverbs/trig_room.wav",
            Reverb::VocalDuo => "assets/internal/reverbs/vocal_duo.wav",
        }
    }
}

pub struct Metronome;
impl From<Metronome> for &'static str {
    fn from(_: Metronome) -> &'static str {
        "assets/internal/metronome.wav"
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;

    #[test]
    fn check_reverb_files() {
        use Reverb::*;
        let vec = vec![
            BlockInside,
            BottleHall,
            CementBlock2,
            CementBlocks1,
            ChateauDeLogne,
            ConicLongEchoHall,
            DeepSpace,
            DerlonSanctuary,
            DirectCabinet1,
            DirectCabinet2,
            DirectCabinet3,
            DirectCabinet4,
            FiveColumns,
            FiveColumnsLong,
            FrenchSalon,
            GoingHome,
            Greek7EchoHall,
            HighlyDampedLargeRoom,
            InTheSileRevised,
            InTheSilo,
            LargeBottleHall,
            LargeLongEchoHall,
            LargeWideEchoHall,
            MasonicLodge,
            Musikvereinsaal,
            NarrowBumpySpace,
            NiceDrumRoom,
            OnAStar,
            ParkingGarage,
            Rays,
            RightGlassTriangle,
            RubyRoom,
            ScalaMilanOperaHall,
            SmallDrumRoom,
            SmallPrehistoricCave,
            StNicolaesChurch,
            TrigRoom,
            VocalDuo,
        ];

        for item in vec {
            let path: &str = item.into();
            assert!(Path::new(path).exists());
        }
    }

    #[test]
    fn check_metronome_file() {
        let path: &str = Metronome.into();
        assert!(Path::new(path).exists());
    }
}
