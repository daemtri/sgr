#[macro_export]
macro_rules! match_and_run {
    ( $event:expr, $name:ident $( , $arg:ident )* ) => {
        match &$event {
            Event::MouseClick(o) => o.$name($($arg),*),
            Event::MouseMove(o) => o.$name($($arg),*),
            Event::MouseOver(o) => o.$name($($arg),*),
        }
    };
}
