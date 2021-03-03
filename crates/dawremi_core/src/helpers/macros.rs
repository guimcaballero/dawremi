#[macro_export]
macro_rules! sequence {
    // This a test to be able to do something with vocaloids down the line
    (@lyrics $self:ident, $len_id:ident : $len:expr, $sign_id:ident : $sign:expr, $( $([ $($lyrics:tt)* ],)? ( $($x:tt)* ),)*) => {
        sequence!($self,
                  // TODO Add note above
                  $len_id: $len, note: Note, $sign_id: $sign,
                  $($($x)*)*
        )
    };

    // With a signal
    ($self:ident, len: $len:expr, $(note: $note:ident,)? signal: $sign:expr, $($x:tt)*) => {
        {
            let mut vec: Vec<Frame> = Vec::new();
            $(
                vec.append(
                    &mut sequence!(@map $self sign: $sign, $x)
                        .take_samples($self.beats($len))
                );
            )*
                vec
        }
    };
    // With a function that takes a note
    ($self:ident, len: $len:expr, note: $note:ident, fun: $fun:expr, $($x:tt)*) => {
        {
            let mut vec: Vec<Frame> = Vec::new();
                $(
                    vec.append(
                        &mut sequence!(@map $self fun: $fun, len: $len, note: $note, $x)
                    );
                )*
            vec
        }
    };

    // Helpers

    (@unwrap_note, $note:ident, ([$($x:tt)*] * $len:expr)) => { [$($note::$x,)*] };
    (@unwrap_note, $note:ident, ($x:tt * $len:expr)) => { [$note::$x] };
    (@unwrap_note, $note:ident, [$($x:tt)*]) => { [$($note::$x,)*] };
    (@unwrap_note, $note:ident, $x:tt) => { [$note::$x] };
    (@unwrap_len ($x:tt * $len:expr)) => { $len };
    (@unwrap_len $x:tt) => { 1. };

    (@map $self:ident fun: $fun:expr, len: $len:expr, note: $note:ident, _) =>  { silence().take_samples($self.beats(1.))};
    (@map $self:ident fun: $fun:expr, len: $len:expr, note: $note:ident, __) => { silence().take_samples($self.beats(1.))};
    (@map $self:ident fun: $fun:expr, len: $len:expr, note: $note:ident, $x:tt) => {
        join_tracks(
            sequence!(@unwrap_note, $note, $x)
                    .iter()
                    .map(|note| {
                        $fun(*note)
                            .take_samples($self.beats($len * sequence!(@unwrap_len $x)))
                    })
                    .collect()
        )
    };
    (@map $self:ident sign: $sign:expr, _) => { silence() };
    (@map $self:ident sign: $sign:expr, __) => { silence() };
    (@map $self:ident sign: $sign:expr, $_x:tt) => { $sign };
}

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
macro_rules! pattern {
    // With a function that takes a note
    ($self:ident, note: $note:ident, repetitions: $rep:expr, $( beat: $beat:expr, $(note: $subnote:ident,)? fun: $fun:expr, pat: ( $($x:tt)* ), )* ) => {
        {
            join_tracks(
                vec![
                    $(
                        {
                            let mut vec: Vec<Frame> = Vec::new();
                            $(
                                vec.append(
                                    &mut sequence!(@map $self
                                                   fun: $fun,
                                                   len: $beat,
                                                   note: $note,
                                                   $x)
                                );
                            )*
                            vec
                        },
                    )*
                ]
            )
                .repeat($rep)
         }
    };

    (@note_ident $a:ident $b:ident) => { $b };
    (@note_ident $a:ident) => { $a };
}
