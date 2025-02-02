// TODO: Generate this in the codegen crate

/// Counts the number of tokens in a macro invocation.
macro_rules! tt_count {
    () => { 0 };
    ($head:tt $($tail:tt)*) => { 1 + tt_count!($($tail)*) };
}

macro_rules! def_states {
    ($($state:ident),*) => {
        $( #[macro_export]
        macro_rules! $state {
            ($arg:expr) => {
                $crate::const_format::formatc!("{}:{}", stringify!($state), $arg)
            };
        })*

        pub(crate) fn states() -> [&'static str; tt_count!($($state)*)] {
            [$(stringify!($state)),*]
        }
    };
}

def_states!(hover, focus, active);
