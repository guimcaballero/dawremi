use dawremi_core::prelude::*;
use std::io::stdin;

macro_rules! loopers {
    ( $($num:pat =>  $mod:ident => $struct:ident,)* ) => {
        $(
            mod $mod;
            pub use $mod::$struct;
        )*

        pub fn select_looper() -> Box<dyn Looper> {
            println!("Select a looper:");

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

loopers!(
    _ => test_looper => Test,
);
