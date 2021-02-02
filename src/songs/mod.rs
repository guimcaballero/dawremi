use dawremi_core::song::Song;
use std::io::stdin;

macro_rules! songs {
    ( $($num:pat =>  $mod:ident => $struct:ident,)* ) => {
        $(
            mod $mod;
            pub use $mod::$struct;
        )*

        pub fn select_song() -> Box<dyn Song> {
            println!("Select a song:");

            $(
                println!("[{}]: {}", stringify!($num), $struct::default().name());
            )*

                let mut s = String::new();
            stdin()
                .read_line(&mut s)
                .expect("Did not enter a correct string");

            match s.trim().parse().unwrap_or(u32::MAX) {
                $(
                    $num => box $struct::default(),
                )*
            }
        }
    };
}

songs!(
    1 => twinkle_twinkle => TwinkleTwinkle,
    2 => audio_effects => AudioEffectsDemo,
    _ => test_song => Test,
);
