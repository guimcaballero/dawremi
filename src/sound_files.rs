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
