#[macro_export]
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

        impl From<$name> for Sound {
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
                let p = dir.join(path);

                assert!(p.as_path().exists());
            }
        }
    };
}

mod reverb;
pub use reverb::*;

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

#[macro_export]
macro_rules! include_sound_enums {
    () => {
        include!(concat!(env!("OUT_DIR"), "/sound_enums.rs"));
    };
}

// This next section is extremely hacky and ugly
// It could use a rework

use convert_case::{Case, Casing};
use std::env;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

struct Folder {
    path: String,
    enum_name: String,
    files: Vec<(String, String)>,
}

fn get_folders(paths: Vec<&str>) -> Vec<Folder> {
    let mut result = vec![];

    // yikes

    for path in paths {
        // Loop over all folders
        for parent in WalkDir::new(path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_dir())
        {
            let mut files = vec![];

            // For each child in a folder, get (enum variant name, file path), and add it to files
            for child in parent
                .path()
                .read_dir()
                .expect("read_dir call failed")
                .filter_map(|e| e.ok())
                .filter(|e| !e.file_type().unwrap().is_dir())
            {
                files.push({
                    let file_name =
                        format!("{}", child.path().file_name().unwrap().to_str().unwrap());
                    let mut file_stem = format!(
                        "{}",
                        child
                            .path()
                            .file_stem()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .chars()
                            // Remove bad characters
                            .map(|c| {
                                match c {
                                    '(' | ')' => '-',
                                    _ => c,
                                }
                            })
                            .collect::<String>()
                            // Change to pascal so we have a good variant name
                            .to_case(Case::Pascal)
                    );
                    // If it begins with a number, add and E in front
                    if let Some('0'..='9') = file_stem.chars().next() {
                        file_stem = format!("E{}", file_stem);
                    }
                    (file_stem, file_name)
                });
            }

            let parent_path = format!("{}", parent.path().display());
            let mut parent_name = format!(
                "{}",
                parent
                    .path()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_case(Case::Pascal)
            );
            // If it begins with a number, add and E in front
            if let Some('0'..='9') = parent_name.chars().next() {
                parent_name = format!("E{}", parent_name);
            }

            if !files.is_empty() {
                result.push(Folder {
                    path: parent_path,
                    enum_name: parent_name,
                    files,
                });
            }
        }
    }

    result
}

pub fn generate_sound_enums(paths: Vec<&str>) {
    println!("cargo:rerun-if-changed=build.rs");
    for path in &paths {
        println!("cargo:rerun-if-changed={}", path);
    }

    // Get a list of folders
    let folders = get_folders(paths);

    let mut string = "
mod sound_enums {
    use dawremi::prelude::*;
"
    .to_string();

    // For each folder, add a call to enum_to_str!
    for folder in folders {
        string.push_str(&format!(
            "
enum_to_str! {}
    prefix: \"{}\",
    pub enum {} {}
",
            "{", folder.path, folder.enum_name, "{"
        ));
        for child in folder.files {
            string.push_str(&format!("        {} => \"{}\",\n", child.0, child.1));
        }

        string.push_str("    }\n}\n");
    }
    string.push_str("}");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("sound_enums.rs");
    fs::write(&dest_path, string).unwrap();
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;

    #[test]
    fn check_metronome_file() {
        let path: Sound = Metronome.into();

        let dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let p = dir.join(path.path);

        assert!(p.as_path().exists());
    }
}
