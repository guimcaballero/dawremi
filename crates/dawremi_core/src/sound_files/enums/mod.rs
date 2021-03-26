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

        impl From<$name> for crate::sound_files::Sound {
            fn from(f: $name) -> Self {
                let path: &str = f.into();
                Self {
                    path: path.to_string(),
                    begin: 0,
                    end: None,
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

use super::Sound;

pub struct Metronome;
impl From<Metronome> for &'static str {
    fn from(_: Metronome) -> &'static str {
        "assets/internal/metronome.wav"
    }
}
impl From<Metronome> for Sound {
    fn from(_: Metronome) -> Self {
        let path: &str = Metronome.into();
        Self {
            path: path.to_string(),
            begin: 0,
            end: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;

    #[test]
    fn check_metronome_file() {
        let path: Sound = Metronome.into();

        let dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let parent = dir.parent().unwrap().parent().unwrap();
        let p = parent.join(path.path);

        assert!(p.as_path().exists());
    }
}
