use super::Sound;

enum_to_str! {
    prefix: "assets/internal/reverbs/",
    pub enum Reverb {
            BlockInside => "block_inside.wav",
            BottleHall => "bottle_hall.wav",
            CementBlock2 => "cement_block2.wav",
            CementBlocks1 => "cement_blocks1.wav",
            ChateauDeLogne => "chateau_de_logne.wav",
            ConicLongEchoHall => "conic_long_echo_hall.wav",
            DeepSpace => "deep_space.wav",
            DerlonSanctuary => "derlon_sanctuary.wav",
            DirectCabinet1 => "direct_cabinet1.wav",
            DirectCabinet2 => "direct_cabinet2.wav",
            DirectCabinet3 => "direct_cabinet3.wav",
            DirectCabinet4 => "direct_cabinet4.wav",
            FiveColumns => "five_columns.wav",
            FiveColumnsLong => "five_columns_long.wav",
            FrenchSalon => "french_salon.wav",
            GoingHome => "going_home.wav",
            Greek7EchoHall => "greek_7_echo_hall.wav",
            HighlyDampedLargeRoom => "highly_damped_large_room.wav",
            InTheSileRevised => "in_the_sile_revised.wav",
            InTheSilo => "in_the_silo.wav",
            LargeBottleHall => "large_bottle_hall.wav",
            LargeLongEchoHall => "large_long_echo_hall.wav",
            LargeWideEchoHall => "large_wide_echo_hall.wav",
            MasonicLodge => "masonic_lodge.wav",
            Musikvereinsaal => "musikvereinsaal.wav",
            NarrowBumpySpace => "narrow_bumpy_space.wav",
            NiceDrumRoom => "nice_drum_room.wav",
            OnAStar => "on_a_star.wav",
            ParkingGarage => "parking_garage.wav",
            Rays => "rays.wav",
            RightGlassTriangle => "right_glass_triangle.wav",
            RubyRoom => "ruby_room.wav",
            ScalaMilanOperaHall => "scala_milan_opera_hall.wav",
            SmallDrumRoom => "small_drum_room.wav",
            SmallPrehistoricCave => "small_prehistoric_cave.wav",
            StNicolaesChurch => "st_nicolaes_church.wav",
            TrigRoom => "trig_room.wav",
            VocalDuo => "vocal_duo.wav",
    }
}

#[cfg(test)]
mod test {
    use super::*;

    test_enum!(Reverb, check_reverb, 38);
}
