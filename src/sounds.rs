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
            Reverb::BlockInside => "assets/reverbs/block_inside.wav",
            Reverb::BottleHall => "assets/reverbs/bottle_hall.wav",
            Reverb::CementBlock2 => "assets/reverbs/cement_block2.wav",
            Reverb::CementBlocks1 => "assets/reverbs/cement_blocks1.wav",
            Reverb::ChateauDeLogne => "assets/reverbs/chateau_de_logne.wav",
            Reverb::ConicLongEchoHall => "assets/reverbs/conic_long_echo_hall.wav",
            Reverb::DeepSpace => "assets/reverbs/deep_space.wav",
            Reverb::DerlonSanctuary => "assets/reverbs/derlon_sanctuary.wav",
            Reverb::DirectCabinet1 => "assets/reverbs/direct_cabinet1.wav",
            Reverb::DirectCabinet2 => "assets/reverbs/direct_cabinet2.wav",
            Reverb::DirectCabinet3 => "assets/reverbs/direct_cabinet3.wav",
            Reverb::DirectCabinet4 => "assets/reverbs/direct_cabinet4.wav",
            Reverb::FiveColumns => "assets/reverbs/five_columns.wav",
            Reverb::FiveColumnsLong => "assets/reverbs/five_columns_long.wav",
            Reverb::FrenchSalon => "assets/reverbs/french_salon.wav",
            Reverb::GoingHome => "assets/reverbs/going_home.wav",
            Reverb::Greek7EchoHall => "assets/reverbs/greek_7_echo_hall.wav",
            Reverb::HighlyDampedLargeRoom => "assets/reverbs/highly_damped_large_room.wav",
            Reverb::InTheSileRevised => "assets/reverbs/in_the_sile_revised.wav",
            Reverb::InTheSilo => "assets/reverbs/in_the_silo.wav",
            Reverb::LargeBottleHall => "assets/reverbs/large_bottle_hall.wav",
            Reverb::LargeLongEchoHall => "assets/reverbs/large_long_echo_hall.wav",
            Reverb::LargeWideEchoHall => "assets/reverbs/large_wide_echo_hall.wav",
            Reverb::MasonicLodge => "assets/reverbs/masonic_lodge.wav",
            Reverb::Musikvereinsaal => "assets/reverbs/musikvereinsaal.wav",
            Reverb::NarrowBumpySpace => "assets/reverbs/narrow_bumpy_space.wav",
            Reverb::NiceDrumRoom => "assets/reverbs/nice_drum_room.wav",
            Reverb::OnAStar => "assets/reverbs/on_a_star.wav",
            Reverb::ParkingGarage => "assets/reverbs/parking_garage.wav",
            Reverb::Rays => "assets/reverbs/rays.wav",
            Reverb::RightGlassTriangle => "assets/reverbs/right_glass_triangle.wav",
            Reverb::RubyRoom => "assets/reverbs/ruby_room.wav",
            Reverb::ScalaMilanOperaHall => "assets/reverbs/scala_milan_opera_hall.wav",
            Reverb::SmallDrumRoom => "assets/reverbs/small_drum_room.wav",
            Reverb::SmallPrehistoricCave => "assets/reverbs/small_prehistoric_cave.wav",
            Reverb::StNicolaesChurch => "assets/reverbs/st_nicolaes_church.wav",
            Reverb::TrigRoom => "assets/reverbs/trig_room.wav",
            Reverb::VocalDuo => "assets/reverbs/vocal_duo.wav",
        }
    }
}

pub struct Metronome;
impl From<Metronome> for &'static str {
    fn from(_: Metronome) -> &'static str {
        "assets/metronome.wav"
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
