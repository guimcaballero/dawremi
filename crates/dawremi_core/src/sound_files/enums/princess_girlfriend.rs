// Pack from: https://old.reddit.com/r/Drumkits/comments/chgj5y/drumkit_with_a_few_sophieflume_inspired_sounds/

enum_to_str! {
    prefix: "assets/internal/princess_girlfriend/808s/",
    pub enum PG808s {
         FM808Eb => "fm_808_Eb.wav",
         GeddurEb => "geddur_Eb.wav",
         Monster808F => "monster_808_(F).wav",
         OneOfTheFattestF => "one_of_the_fattest_(G).wav",
         Reese808D => "reese_808_(d).wav",
         Slammed808G => "slammed_808_G.wav",
         ThiccumEb => "thiccum_Eb.wav",
    }
}

enum_to_str! {
    prefix: "assets/internal/princess_girlfriend/atmos/",
    pub enum Atmos {
         Bubbly => "bubbly.wav",
         GlassPhaseyThingys => "glass_phasey_thingys.wav",
         PlanetBells => "planet_bells.wav",
         Siren => "siren_atmos.wav",
         SmallDemons => "small_demons.wav",
         SpaceyNoise => "spacey_noise.wav",
    }
}

enum_to_str! {
    prefix: "assets/internal/princess_girlfriend/claps/",
    pub enum Claps {
         BurntClap => "burnt_clap.wav",
         ClassyAssClap => "classy_assclap.wav",
         CorkClap => "cork_clap.wav",
         DeepFriedClap => "deep_fried_clap.wav",
         EnsembleClap => "ensemble_clap.wav",
         LightClap => "light_clap.wav",
         MetalSchmacc => "metal_schmacc.wav",
         ToastyClap => "toasty_clap.wav",
    }
}

enum_to_str! {
    prefix: "assets/internal/princess_girlfriend/fx/",
    pub enum Fx {
         ClassicSpookyRiser => "classic_spooky_riser.wav",
         ClickClack => "click_clack.wav",
         Gurgle => "gurgle.wav",
         PitchBender => "pitch_bender.wav",
         SpaceFx => "space_FX.wav",
         StutterThingy => "stutter_thingy.wav",
         Wobble => "wobble.wav",
         Wonky8Bar150 => "wonky_8_bar_150.wav",
         Wop => "wop.wav",
    }
}

enum_to_str! {
    prefix: "assets/internal/princess_girlfriend/hats/",
    pub enum Hats {
         ChipperHat => "chipper_hat.wav",
         HardHat => "hard_hat.wav",
         HeavyTrapHat => "heavy_trap_hat.wav",
         LoFiHat => "lo_fi_hat.wav",
         LoFiOpenHat => "lo_fi_open_hat.wav",
         SquishyOpenHat => "squishy_open_hat.wav",
         VirginHat => "virgin_hat.wav",
    }
}

enum_to_str! {
    prefix: "assets/internal/princess_girlfriend/kicks/",
    pub enum Kicks {
         DnBKick => "DnB_Kick.wav",
         BonkersKick => "bonkers_kick.wav",
         BuzzedKick => "buzzed_kick.wav",
         ClickyKick => "clicky_kick.wav",
         CrunchyKick => "crunchy_kick.wav",
         RoomyKick => "roomy_kick.wav",
         SlapHappyKick => "slap_happy_kick.wav",
         Stomp2 => "stomp_2.wav",
         ThumpyKick => "thumpy_kick.wav",
         WaterKick => "water_kick.wav",
    }
}
enum_to_str! {
    prefix: "assets/internal/princess_girlfriend/perc_stuff/",
    pub enum PercStuff {
         BigFartPerc => "big_fart_perc.wav",
         CrackedPerc => "cracked_perc.wav",
         CrowChant => "crow_chant.wav",
         OkraChant => "okra_chant.wav",
         Ping => "ping.wav",
         TennisRim => "tennis_rim.wav",
         Waa => "waa.wav",
         Wah => "wah.wav",
         WaterDrop => "water_drop.wav",
         Whee => "whee.wav",
         Wie => "wie.wav",
         Wop => "wop.wav",
    }
}
enum_to_str! {
    prefix: "assets/internal/princess_girlfriend/seinfeld/",
    pub enum Seinfeld {
         Seinfeld => "seinfeld.wav",
    }
}

enum_to_str! {
    prefix: "assets/internal/princess_girlfriend/snares/",
    pub enum Snares {
         BanginSnare => "bangin_snare.wav",
         DeathStarSnare => "death_star_snare.wav",
         FlamSnare => "flam_snare.wav",
         GlassSnareAgain => "glass_snare_again.wav",
         GlossySnare => "glossy_snare.wav",
         HeavyTrapSnare => "heavy_trap_snare.wav",
         JugSnare => "jug_snare.wav",
         Klonk => "klonk.wav",
         KonkySnare => "konky_snare.wav",
         MonsterSnare => "monster_snare.wav",
         NiceClang => "nice_clang.wav",
         NiceRim => "nice_rim_.wav",
         Schnareclapp3 => "schnareclapp_3.wav",
         Slamacow => "slamacow.wav",
         SneezeSnare => "sneeze_snare.wav",
         SophieSnare => "sophie_snare.wav",
         SowRemixSnare => "sow_remix_snare.wav",
         SpinSnare => "spin_snare.wav",
         SweetSnare => "sweet_snare.wav",
         TrapSmacc => "trap_smacc.wav",
         VroomSnare => "vroom_snare.wav",
         WackSnare => "wack_snare.wav",
         YonkSnare => "yonk_snare.wav",
    }
}

enum_to_str! {
    prefix: "assets/internal/princess_girlfriend/synth_shots/",
    pub enum SynthShots {
         BeeC => "bee_(C).wav",
         CreepyBellG => "creepy_bell_(g).wav",
         NiceBellC => "nice_bell_(C).wav",
         WaaaeeyF => "waaaeey_(F).wav",
    }
}

#[cfg(test)]
mod test {
    use super::*;

    test_enum!(PG808s, check_808s, 7);
    test_enum!(Atmos, check_atmos, 6);
    test_enum!(Claps, check_claps, 8);
    test_enum!(Fx, check_fx, 9);
    test_enum!(Hats, check_hats, 7);
    test_enum!(Kicks, check_kicks, 10);
    test_enum!(PercStuff, check_perc, 12);
    test_enum!(Seinfeld, check_seinfeld, 1);
    test_enum!(Snares, check_snares, 23);
    test_enum!(SynthShots, check_synth_shots, 4);
}
