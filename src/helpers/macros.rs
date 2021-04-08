#[macro_export]
macro_rules! note_list {
    ( $( $x:tt ),*  $(,)?) => {
        vec![
            $( note_list!(@unpack $x),)*
        ]
    };
    (@unpack _) => { vec![] };
    (@unpack [$($x:tt),*] $(,)?) => { vec![ $($x,)* ] };
    (@unpack $x:tt) => { vec![$x] };
}

#[macro_export]
macro_rules! note_option {
    ( $( $x:tt ),*  $(,)?) => {
        vec![
            $( note_option!(@unpack $x),)*
        ]
    };
    (@unpack _) => { None };
    (@unpack $x:tt) => { Some($x) };
}

#[macro_export]
macro_rules! vec_into {
    ( $( $x:expr ),* $(,)?) => {
        {
            let mut temp_vec: Vec<TrackGenerator> = Vec::new();
            $(
                temp_vec.push($x.into());
            )*
                temp_vec
        }
    };
}
