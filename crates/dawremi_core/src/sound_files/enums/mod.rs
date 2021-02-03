macro_rules! enum_to_str {
    (prefix: $prefix:expr, $(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident => $val:expr,)*
    }) => {
        $(#[$meta])*
            $vis enum $name {
                $($(#[$vmeta])* $vname,)*
            }

        impl $name {
            #[allow(dead_code)]
            pub fn enumerate() -> Vec<$name> {
                vec![
                    $($name::$vname,)*
                ]
            }
        }

        impl From<$name> for &'static str {
            fn from(f: $name) -> &'static str {
                match f {
                    $($name::$vname => concat!($prefix, $val),)*
                }
            }
        }
    }
}

#[cfg(test)]
macro_rules! test_enum {
    ($name:ident, $func:ident, $len:expr) => {
        #[test]
        fn $func() {
            use std::path::Path;

            let vec = $name::enumerate();
            assert_eq!($len, vec.len());

            for item in vec {
                let path: &str = item.into();

                let dir = Path::new(env!("CARGO_MANIFEST_DIR"));
                let parent = dir.parent().unwrap().parent().unwrap();
                let p = parent.join(path);

                assert!(p.as_path().exists());
            }
        }
    };
}

mod reverb;
pub use reverb::*;
pub mod princess_girlfriend;

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
    fn check_metronome_file() {
        let path: &str = Metronome.into();

        let dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let parent = dir.parent().unwrap().parent().unwrap();
        let p = parent.join(path);

        assert!(p.as_path().exists());
    }
}
